use std::collections::HashMap;

use indexmap::IndexSet;

use crate::component_catalog::ComponentId;
use crate::components::Components;
use crate::operations::OperationId;
use crate::rust_object::RustObject;
use crate::rust_type::{Container, DeclarationRoot, RustType};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DependencySource {
    Component(ComponentId),
    Operation(OperationId),
    WebhookEvent(String),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum DependencyUsage {
    ModelField,
    RequestParam,
    RequestReturn,
    WebhookPayload,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ReferenceForm {
    Direct,
    InContainer,
    Expandable,
    UnionMember,
    /// A reference to a component's generated ID newtype rather than its
    /// model declaration.
    IdOnly,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EdgeTreatment {
    Keep,
    Opaque,
}

/// Policy used when projecting immutable dependency facts into a graph.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EdgePolicy {
    pub expandable: EdgeTreatment,
    pub id_only: EdgeTreatment,
}

impl EdgePolicy {
    /// The dependency semantics used by current generated source: expandable
    /// model references affect placement, while ID types do not pull in their
    /// corresponding object definitions.
    pub const fn current_layout() -> Self {
        Self { expandable: EdgeTreatment::Keep, id_only: EdgeTreatment::Opaque }
    }

    /// Include every recorded relationship. Intended for diagnostics and
    /// counterfactual graph projections, not current source generation.
    pub const fn all_dependencies() -> Self {
        Self { expandable: EdgeTreatment::Keep, id_only: EdgeTreatment::Keep }
    }

    pub const fn without_expandables() -> Self {
        Self { expandable: EdgeTreatment::Opaque, id_only: EdgeTreatment::Opaque }
    }

    pub const fn keeps_expandables(self) -> bool {
        matches!(self.expandable, EdgeTreatment::Keep)
    }

    pub const fn keeps_id_only(self) -> bool {
        matches!(self.id_only, EdgeTreatment::Keep)
    }

    fn includes(self, dependency: &Dependency) -> bool {
        match dependency.form {
            ReferenceForm::Expandable => self.keeps_expandables(),
            ReferenceForm::IdOnly => self.keeps_id_only(),
            ReferenceForm::Direct | ReferenceForm::InContainer | ReferenceForm::UnionMember => true,
        }
    }
}

impl Default for EdgePolicy {
    fn default() -> Self {
        Self::current_layout()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct DependencyOrigin {
    pub source: DependencySource,
    pub path: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Dependency {
    pub target: ComponentId,
    pub usage: DependencyUsage,
    pub form: ReferenceForm,
    pub origin: DependencyOrigin,
}

#[derive(Debug, Clone, Default)]
pub struct DependencyIndex {
    facts: Vec<Dependency>,
    by_component: HashMap<ComponentId, Vec<usize>>,
    webhooks: Vec<usize>,
}

impl DependencyIndex {
    pub fn build(components: &Components) -> Self {
        let mut index = Self::default();
        for (&component_id, component) in &components.components {
            Collector::new(
                &mut index,
                Some(component_id),
                DependencySource::Component(component_id),
                DependencyUsage::ModelField,
            )
            .collect_object(component.rust_obj(), ReferenceForm::Direct);

            for request in components.requests_for_component(component_id) {
                Collector::new(
                    &mut index,
                    Some(component_id),
                    DependencySource::Operation(request.operation_id),
                    DependencyUsage::RequestReturn,
                )
                .with_root("return")
                .collect_type(&request.returned, ReferenceForm::Direct);
                if let Some(params) = &request.params {
                    Collector::new(
                        &mut index,
                        Some(component_id),
                        DependencySource::Operation(request.operation_id),
                        DependencyUsage::RequestParam,
                    )
                    .with_root("params")
                    .collect_type(&params.typ, ReferenceForm::Direct);
                }
                for path_param in &request.path_params {
                    Collector::new(
                        &mut index,
                        Some(component_id),
                        DependencySource::Operation(request.operation_id),
                        DependencyUsage::RequestParam,
                    )
                    .with_root("path_params")
                    .with_root(&path_param.name)
                    .collect_type(&path_param.rust_type, ReferenceForm::Direct);
                }
            }
        }

        for webhook in &components.webhook_objs {
            Collector::new(
                &mut index,
                None,
                DependencySource::WebhookEvent(webhook.event_type.clone()),
                DependencyUsage::WebhookPayload,
            )
            .collect_type(&webhook.typ, ReferenceForm::Direct);
        }
        index
    }

    pub fn facts(&self) -> &[Dependency] {
        &self.facts
    }

    pub fn component_targets(
        &self,
        component: ComponentId,
        policy: EdgePolicy,
    ) -> IndexSet<ComponentId> {
        self.targets(self.by_component.get(&component).map_or(&[], Vec::as_slice), policy, |_| true)
    }

    pub fn component_model_targets(
        &self,
        component: ComponentId,
        policy: EdgePolicy,
    ) -> IndexSet<ComponentId> {
        self.targets(self.by_component.get(&component).map_or(&[], Vec::as_slice), policy, |fact| {
            fact.usage == DependencyUsage::ModelField
        })
    }

    pub fn webhook_targets(&self) -> IndexSet<ComponentId> {
        self.targets(&self.webhooks, EdgePolicy::current_layout(), |_| true)
    }

    fn targets(
        &self,
        indices: &[usize],
        policy: EdgePolicy,
        include: impl Fn(&Dependency) -> bool,
    ) -> IndexSet<ComponentId> {
        indices
            .iter()
            .map(|index| &self.facts[*index])
            .filter(|fact| include(fact))
            .filter(|fact| policy.includes(fact))
            .map(|fact| fact.target)
            .collect()
    }

    fn push(&mut self, owner: Option<ComponentId>, dependency: Dependency) {
        let index = self.facts.len();
        self.facts.push(dependency);
        if let Some(component) = owner {
            self.by_component.entry(component).or_default().push(index);
        } else {
            self.webhooks.push(index);
        }
    }
}

struct Collector<'a> {
    index: &'a mut DependencyIndex,
    owner: Option<ComponentId>,
    source: DependencySource,
    usage: DependencyUsage,
    path: Vec<String>,
}

impl<'a> Collector<'a> {
    fn new(
        index: &'a mut DependencyIndex,
        owner: Option<ComponentId>,
        source: DependencySource,
        usage: DependencyUsage,
    ) -> Self {
        Self { index, owner, source, usage, path: Vec::new() }
    }

    fn with_root(mut self, root: &str) -> Self {
        self.path.push(root.to_string());
        self
    }

    fn collect_type(&mut self, typ: &RustType, form: ReferenceForm) {
        match typ {
            RustType::Object(object, _) => self.collect_object(object, form),
            RustType::Path(DeclarationRoot::Component(target)) => {
                self.index.push(
                    self.owner,
                    Dependency {
                        target: *target,
                        usage: self.usage,
                        form,
                        origin: DependencyOrigin {
                            source: self.source.clone(),
                            path: self.path.clone(),
                        },
                    },
                );
            }
            RustType::Path(DeclarationRoot::ObjectId(target)) => {
                self.index.push(
                    self.owner,
                    Dependency {
                        target: *target,
                        usage: self.usage,
                        form: ReferenceForm::IdOnly,
                        origin: DependencyOrigin {
                            source: self.source.clone(),
                            path: self.path.clone(),
                        },
                    },
                );
            }
            RustType::Path(DeclarationRoot::Shared(_) | DeclarationRoot::Deduplicated { .. })
            | RustType::Primitive(_)
            | RustType::External(_)
            | RustType::Item(_) => {}
            RustType::Container(container) => {
                let nested_form = match container {
                    Container::Expandable(_) => ReferenceForm::Expandable,
                    _ if form == ReferenceForm::Direct => ReferenceForm::InContainer,
                    _ => form,
                };
                self.collect_type(container.value_typ(), nested_form);
            }
            RustType::Reference(inner) => self.collect_type(inner, form),
        }
    }

    fn collect_object(&mut self, object: &RustObject, form: ReferenceForm) {
        match object {
            RustObject::Struct(struct_) => {
                for field in &struct_.fields {
                    self.path.push(field.field_name.clone());
                    self.collect_type(&field.rust_type, form);
                    self.path.pop();
                }
            }
            RustObject::Enum(variants) => {
                for variant in variants {
                    let Some(typ) = &variant.rust_type else {
                        continue;
                    };
                    self.path.push(variant.variant.to_string());
                    let variant_form = if form == ReferenceForm::Expandable {
                        form
                    } else {
                        ReferenceForm::UnionMember
                    };
                    self.collect_type(typ, variant_form);
                    self.path.pop();
                }
            }
            RustObject::FieldlessEnum(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Collector, DependencyIndex, DependencySource, DependencyUsage, EdgePolicy, ReferenceForm,
    };
    use crate::component_catalog::ComponentCatalog;
    use crate::rust_object::{RustObject, Struct, StructField};
    use crate::rust_type::RustType;
    use crate::types::ComponentName;

    #[test]
    fn records_expandable_form_and_field_path() {
        let catalog = ComponentCatalog::from_names([
            ComponentName::new("charge".to_string()),
            ComponentName::new("invoice".to_string()),
        ])
        .unwrap();
        let charge = catalog.id(&ComponentName::new("charge".to_string())).unwrap();
        let invoice = catalog.id(&ComponentName::new("invoice".to_string())).unwrap();
        let object = RustObject::Struct(Struct::new(vec![StructField::new(
            "invoice",
            RustType::expandable(RustType::component(invoice)),
        )]));
        let mut index = DependencyIndex::default();

        Collector::new(
            &mut index,
            Some(charge),
            DependencySource::Component(charge),
            DependencyUsage::ModelField,
        )
        .collect_object(&object, ReferenceForm::Direct);

        assert_eq!(index.facts().len(), 1);
        assert_eq!(index.facts()[0].target, invoice);
        assert_eq!(index.facts()[0].form, ReferenceForm::Expandable);
        assert_eq!(index.facts()[0].origin.path, ["invoice"]);
        assert!(index.component_targets(charge, EdgePolicy::without_expandables()).is_empty());
        assert_eq!(
            index.component_targets(charge, EdgePolicy::current_layout()).as_slice(),
            &[invoice]
        );
    }

    #[test]
    fn records_id_references_without_using_them_for_current_layout() {
        let catalog = ComponentCatalog::from_names([
            ComponentName::new("charge".to_string()),
            ComponentName::new("customer".to_string()),
        ])
        .unwrap();
        let charge = catalog.id_str("charge").unwrap();
        let customer = catalog.id_str("customer").unwrap();
        let object = RustObject::Struct(Struct::new(vec![StructField::new(
            "customer",
            RustType::object_id(customer),
        )]));
        let mut index = DependencyIndex::default();

        Collector::new(
            &mut index,
            Some(charge),
            DependencySource::Component(charge),
            DependencyUsage::ModelField,
        )
        .collect_object(&object, ReferenceForm::Direct);

        assert_eq!(index.facts().len(), 1);
        assert_eq!(index.facts()[0].form, ReferenceForm::IdOnly);
        assert_eq!(index.facts()[0].origin.path, ["customer"]);
        assert!(index.component_targets(charge, EdgePolicy::current_layout()).is_empty());
        assert_eq!(
            index.component_targets(charge, EdgePolicy::all_dependencies()).as_slice(),
            &[customer]
        );
    }
}
