use indexmap::IndexMap;
use indexmap::map::Entry;
use tracing::debug;

use crate::component_catalog::ComponentId;
use crate::operations::RequestSpec;
use crate::rust_object::{ObjectKind, ObjectMetadata, ObjectUsage, RustObject};
use crate::rust_type::{DeclarationRoot, RustType};
use crate::stripe_object::StripeObject;
use crate::types::RustIdent;
use crate::visitor::{Visit, VisitMut};

#[derive(Debug, Default)]
struct CollectDuplicateObjects {
    objs: IndexMap<RustObject, Vec<(ObjectMetadata, ObjectUsage)>>,
}

impl Visit<'_> for CollectDuplicateObjects {
    fn visit_obj(&mut self, obj: &RustObject, meta: Option<&ObjectMetadata>, usage: ObjectUsage) {
        if let Some(meta) = meta {
            match self.objs.entry(obj.clone()) {
                Entry::Occupied(mut occ) => {
                    occ.get_mut().push((meta.clone(), usage));
                }
                Entry::Vacant(entry) => {
                    entry.insert(vec![(meta.clone(), usage)]);
                }
            }
        };
        obj.visit(self, usage);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct DeduppedObjectInfo {
    pub ident: RustIdent,
    pub usage: ObjectUsage,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct DeduppedObject {
    pub usage: ObjectUsage,
    pub object: RustObject,
}

#[derive(Debug)]
struct DeduplicateObjects {
    objs: IndexMap<RustObject, DeduppedObjectInfo>,
    component: ComponentId,
}

impl DeduplicateObjects {
    pub fn new(component: ComponentId) -> Self {
        Self { objs: Default::default(), component }
    }
}

impl VisitMut for DeduplicateObjects {
    fn visit_typ_mut(&mut self, typ: &mut RustType, usage: ObjectUsage)
    where
        Self: Sized,
    {
        if let Some((obj, _)) = typ.as_object_mut()
            && let Some(dedup_spec) = self.objs.get(obj)
        {
            *typ = RustType::Path(DeclarationRoot::Deduplicated {
                component: self.component,
                ident: dedup_spec.ident.clone(),
            })
        }
        typ.visit_mut(self, usage);
    }
}

fn implied_name_from_meta_doc(metadata: &ObjectMetadata) -> Option<&str> {
    let doc = metadata.doc.as_ref()?;
    implied_name_from_doc(doc)
}

fn implied_name_from_doc(doc: &str) -> Option<&str> {
    let mut words = doc.split_ascii_whitespace();
    if words.next() != Some("The") {
        return None;
    }
    let second_word = words.next();
    if words.next() != Some("of") {
        return None;
    }
    second_word
}

fn maybe_infer_by_field_name(meta: &[(ObjectMetadata, ObjectUsage)]) -> Option<DeduppedObjectInfo> {
    let (type_data, _) = meta.iter().find(|(_, usage)| usage.kind == ObjectKind::Type)?;
    let parent = type_data.parent.as_ref()?;
    let field_name = type_data.field_name.as_ref()?;

    if meta.iter().any(|(m, _)| m.field_name.as_ref() != Some(field_name)) {
        return None;
    }
    Some(DeduppedObjectInfo {
        ident: RustIdent::joined(parent, field_name),
        usage: ObjectUsage {
            kind: ObjectKind::Type,
            used_as_request_param: meta.iter().any(|(_, usage)| usage.used_as_request_param),
        },
    })
}

/// Try to infer an identifier given metadata about identical objects
fn infer_dedupped_ident(objs: &[(ObjectMetadata, ObjectUsage)]) -> Option<RustIdent> {
    let first = objs.first().unwrap();
    let first_meta = &first.0;
    if let Some(title) = &first_meta.title {
        // `param` is used very generally and will not be a helpful name to infer
        if title != "param" && objs.iter().all(|(m, _)| m.title.as_ref() == Some(title)) {
            return Some(RustIdent::create(title));
        }
    }

    if let Some(doc_name) = implied_name_from_meta_doc(first_meta)
        && let Some(parent) = &first_meta.parent
        && objs.iter().all(|(m, _)| {
            implied_name_from_meta_doc(m) == Some(doc_name) && m.parent.as_ref() == Some(parent)
        })
    {
        return Some(RustIdent::joined(parent, doc_name));
    }
    None
}

fn infer_dedupped_object_for(
    meta: &[(ObjectMetadata, ObjectUsage)],
    obj: &RustObject,
) -> Option<DeduppedObjectInfo> {
    if matches!(obj, RustObject::FieldlessEnum(_))
        && let Some(dedupped) = maybe_infer_by_field_name(meta)
    {
        return Some(dedupped);
    }
    let ident = infer_dedupped_ident(meta)?;
    let first = meta.first().unwrap();
    let first_usage = first.1;
    if meta.iter().all(|(_, usage)| *usage == first_usage) {
        return Some(DeduppedObjectInfo { ident, usage: first_usage });
    }
    None
}

#[tracing::instrument(level = "debug", skip(comp), fields(path = %comp.path()))]
pub fn deduplicate_types(
    comp: &mut StripeObject,
    requests: &mut [RequestSpec],
    component: ComponentId,
) -> IndexMap<RustIdent, DeduppedObject> {
    let mut objs = IndexMap::new();

    // We run deduplication passes until there are no further changes since one round
    // of deduplicating can enable another
    loop {
        let mut collector = CollectDuplicateObjects::default();
        comp.visit(&mut collector);
        for request in requests.iter() {
            request.visit(&mut collector);
        }

        let mut dedupper = DeduplicateObjects::new(component);
        for (obj, meta) in collector.objs {
            // Nothing to deduplicate
            if meta.len() < 2 {
                continue;
            }
            if let Some(inferred) = infer_dedupped_object_for(&meta, &obj) {
                let ident = &inferred.ident;
                // Don't add another deduplicated type with the same name as an existing one
                if dedupper.objs.values().all(|o| &o.ident != ident) && !objs.contains_key(ident) {
                    dedupper.objs.insert(obj, inferred);
                }
            }
        }
        // If we weren't able to deduplicate anything new, we're done
        if dedupper.objs.is_empty() {
            break;
        }

        comp.visit_mut(&mut dedupper);
        for request in requests.iter_mut() {
            request.visit_mut(&mut dedupper);
        }
        for (obj, info) in dedupper.objs {
            let ident = info.ident.clone();
            let usage = info.usage;
            if objs.insert(ident.clone(), DeduppedObject { usage, object: obj }).is_some() {
                panic!("Tried to add duplicate ident {ident}");
            }
        }
    }
    if !objs.is_empty() {
        debug!("Deduplicated {} types", objs.len());
    }
    objs
}
