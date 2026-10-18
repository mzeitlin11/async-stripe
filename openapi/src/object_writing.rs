use std::fmt::Write;

use crate::STRIPE_TYPES;
use crate::item_catalog::{ItemId, ItemMetadata, ItemProperties, RustItem};
use crate::item_origin::TypeRoot;
use crate::render::{Lifetime, RenderContext};
use crate::request_model::RequestModel;
use crate::rust_object::EnumOfObjects::ObjectUnion;
use crate::rust_object::{ObjectIdentity, ObjectUsage, RustObject};
use crate::rust_type::DeclarationRoot;
use crate::templates::ObjectWriter;
use crate::templates::object_trait::{write_object_trait, write_object_trait_for_enum};
use crate::templates::utils::write_doc_comment;

impl RenderContext<'_> {
    pub fn write_item(&self, item: ItemId, out: &mut String) {
        self.assert_emits(item);
        let record = self.item(item);
        match &record.item {
            RustItem::Object(object) => self.write_object_with_properties(
                &object.definition,
                &record.metadata,
                object.usage,
                self.item_properties(item),
                object.enum_of_objects.as_ref(),
                out,
            ),
            RustItem::IdNewtype => {
                let _ = writeln!(out, "{STRIPE_TYPES}::def_id!({});", record.metadata.ident);
            }
        }
        for &child in &record.children {
            self.write_item(child, out);
        }
    }

    fn write_object_with_properties(
        &self,
        obj: &RustObject,
        metadata: &ItemMetadata,
        usage: ObjectUsage,
        properties: ItemProperties,
        enum_of_objects: Option<&crate::rust_object::EnumOfObjects>,
        out: &mut String,
    ) {
        if let Some(doc) = &metadata.doc {
            let comment = write_doc_comment(doc, 0);
            let _ = write!(out, "{comment}");
        }

        // If the object contains any references, we'll need to print with a lifetime
        let lifetime = if properties.has_lifetime { Some(Lifetime) } else { None };
        let ident = &metadata.ident;

        let mut writer = ObjectWriter::new(*self, ident, usage);
        writer.lifetime(lifetime).derive_copy(properties.is_copy).derive_eq(properties.is_eq);

        match obj {
            RustObject::Struct(struct_) => {
                writer.write_struct_definition(out, struct_);
                if usage.used_as_request_param {
                    writer.write_struct_constructor(out, struct_);
                }
            }
            RustObject::FieldlessEnum(variants) => {
                writer.provide_unknown_variant(obj.provide_unknown_variant());
                writer.use_underscore_unknown(obj.use_underscore_unknown());
                writer.write_fieldless_enum_variants(out, variants);
            }
            RustObject::Enum(variants) => {
                if let Some(objects) = enum_of_objects {
                    writer.write_enum_of_objects(out, objects);
                } else {
                    writer.write_arbitrary_enum_variants(out, variants);
                }
            }
        }
    }
}

pub fn gen_obj(item: ItemId, context: RenderContext<'_>) -> String {
    let mut out = String::with_capacity(128);

    context.write_item(item, &mut out);
    let record = context.item(item);
    let RustItem::Object(object) = &record.item else {
        panic!("component root is not an object: {item:?}")
    };
    let TypeRoot::Declaration(DeclarationRoot::Component(component)) = &record.origin.type_root
    else {
        panic!("component root has a non-component origin: {:?}", record.origin)
    };

    let ident = &record.metadata.ident;
    let obj = &object.definition;
    match (obj, &object.identity) {
        (_, None) => {}
        (RustObject::Struct(_), Some(ObjectIdentity::IdType(id_typ))) => {
            write_object_trait(&mut out, ident, context.display_type(id_typ));

            // Only the component that owns an ID has an ID item. For example,
            // `DeletedAccount` refers to the existing `AccountId` item instead of
            // rendering a duplicate declaration.
            if let Some(id_item) = context
                .root(&TypeRoot::Declaration(DeclarationRoot::ObjectId(*component)))
                .and_then(|root| root.declaration)
            {
                context.write_item(id_item, &mut out);
            }
        }
        (RustObject::Enum(_), Some(ObjectIdentity::ObjectUnion)) => {
            let Some(object_names) = &object.enum_of_objects else {
                panic!("Object {ident} is an enum that is not a union of stripe objects");
            };
            let ObjectUnion(objects) = object_names else {
                panic!("Object {ident} is an enum that is not a union of stripe objects");
            };
            write_object_trait_for_enum(&mut out, ident, objects, context)
        }
        (kind, identity) => {
            panic!("invalid object identity combination for {ident}: {kind:?} with {identity:?}")
        }
    }
    out
}

pub fn gen_requests(requests: &[RequestModel], context: RenderContext<'_>) -> String {
    let mut out = String::with_capacity(128);

    for request in requests {
        if let Some(item) = request.params.as_ref().and_then(|params| params.declaration) {
            context.write_item(item, &mut out);
        }

        let req_body = request.generate(context);
        let _ = write!(out, "{req_body}");

        if let Some(item) = request.returned.declaration {
            context.write_item(item, &mut out);
        }
    }
    out
}
