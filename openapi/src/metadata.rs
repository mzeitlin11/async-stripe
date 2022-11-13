use std::collections::{BTreeMap, BTreeSet};
use std::fs::write;
use std::path::Path;

use heck::{CamelCase, SnakeCase};
use openapiv3::{ReferenceOr, SchemaKind};

use crate::mappings::{FIELD_MAPPINGS, OBJECT_MAPPINGS};
use crate::spec::{as_object_properties, Spec};
use crate::types::{
    EnumVariantName, FeatureGroups, IdType, RustObjectTypeName, RustType, SchemaName,
};
use crate::{file_generator::FileGenerator, mappings, types::CopyOrClone};

/// Global metadata for the entire codegen process.
#[derive(Debug)]
pub struct Metadata<'a> {
    pub spec: &'a Spec,
    /// A map of `objects` to their rust id type
    pub id_mappings: BTreeMap<String, (IdType, CopyOrClone)>,

    /// The set of schemas which should implement `Object`.
    /// These have both an `id` property and an `object` property.
    pub objects: BTreeSet<SchemaName>,
    /// A one to many map of schema to depending types.
    pub dependents: BTreeMap<&'a str, BTreeSet<&'a str>>,
    /// A one to many map of _objects_ to requests which should be
    /// implemented for that object.
    ///
    /// This is typically determined by the first segment in the path.
    pub requests: BTreeMap<SchemaName, BTreeSet<&'a str>>,
}

impl<'a> Metadata<'a> {
    pub fn from_spec(spec: &'a Spec) -> Self {
        let id_renames = mappings::id_renames();

        let mut objects = BTreeSet::new();
        let mut dependents: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
        let mut id_mappings = BTreeMap::new();

        for (key, ref_or_schema) in spec.component_schemas() {
            let schema_name = key.as_str();
            let properties = match ref_or_schema.as_item().and_then(as_object_properties) {
                Some(props) => props,
                None => continue,
            };
            if properties.contains_key("object") {
                objects.insert(schema_name);
                if properties.contains_key("id") {
                    let id_type = IdType::new(
                        id_renames
                            .get(&schema_name)
                            .unwrap_or(&schema_name)
                            .replace('.', "_")
                            .to_camel_case()
                            + "Id",
                    );

                    id_mappings.insert(
                        schema_name.replace('.', "_").to_owned(),
                        (id_type, CopyOrClone::Clone),
                    );
                }
            }
            for schema_or_ref in properties.values() {
                match schema_or_ref {
                    ReferenceOr::Reference { reference } => {
                        let dep = reference.trim_start_matches("#/components/schemas/");
                        dependents.entry(dep).or_default().insert(schema_name);
                    }
                    ReferenceOr::Item(schema) => {
                        if let SchemaKind::AnyOf { any_of } = &schema.schema_kind {
                            for ty in any_of {
                                if let ReferenceOr::Reference { reference } = ty {
                                    let dep = reference.trim_start_matches("#/components/schemas/");
                                    dependents.entry(dep).or_default().insert(schema_name);
                                }
                            }
                        }
                    }
                }
            }
        }

        Self { spec, requests: metadata_requests(spec, &objects), objects, dependents, id_mappings }
    }

    /// generate placeholder types with stubs for potentially missing features
    pub fn write_placeholders<T>(&self, out_path: T)
    where
        T: AsRef<Path>,
    {
        let mut out = String::new();
        out.push_str("use crate::ids::*;\n");
        out.push_str("use crate::params::Object;\n");
        out.push_str("use serde::{Deserialize, Serialize};\n");

        let feature_groups = feature_groups();

        for (schema, feature) in feature_groups {
            out.push('\n');
            let id_info = self.schema_to_id_type(&schema);
            let id_field = id_info.as_ref().map(|m| m.0.as_ref()).unwrap_or("()");
            let c_c = id_info.as_ref().map(|m| m.1).unwrap_or(CopyOrClone::Copy);
            let struct_type = schema_to_rust_type(&schema);
            out.push_str(&format!("#[cfg(not(feature = \"{}\"))]\n", feature.as_str()));
            out.push_str("#[derive(Clone, Debug, Default, Deserialize, Serialize)]\n");
            out.push_str(&format!("pub struct {} {{\n", struct_type));
            out.push_str(&format!("\tpub id: {},\n", id_field));
            out.push_str("}\n\n");
            out.push_str(&format!("#[cfg(not(feature = \"{}\"))]\n", feature.as_str()));
            out.push_str(&format!("impl Object for {} {{\n", struct_type));
            out.push_str(&format!("\ttype Id = {};\n", id_field));
            out.push_str(&format!(
                "\tfn id(&self) -> Self::Id {{ self.id{} }}\n",
                match c_c {
                    CopyOrClone::Clone => ".clone()",
                    CopyOrClone::Copy => "",
                }
            ));
            out.push_str(&format!("\tfn object(&self) -> &'static str {{ \"{}\" }}\n", schema));
            out.push_str("}\n");
        }

        write(&out_path.as_ref().join("placeholders.rs"), out.as_bytes()).unwrap();
    }

    #[tracing::instrument(skip_all)]
    pub fn get_files(&self) -> Vec<FileGenerator> {
        self.objects
            .iter()
            .filter(|o| !o.starts_with("deleted_"))
            .map(|o| FileGenerator::new(o.clone()))
            .collect()
    }

    pub fn schema_to_id_type(&self, schema: &SchemaName) -> Option<(IdType, CopyOrClone)> {
        let schema = schema.replace('.', "_");
        self.id_mappings.get(schema.as_str()).map(ToOwned::to_owned)
    }
}

pub fn field_to_rust_type(schema: &SchemaName, field: &str) -> Option<RustType> {
    let schema = schema.replace('.', "_");
    FIELD_MAPPINGS.get(&(schema.as_str(), field)).cloned()
}

pub fn schema_to_rust_object_name(schema: &SchemaName) -> RustObjectTypeName {
    let schema = schema.replace('.', "_");
    if let Some(rename) = OBJECT_MAPPINGS.get(schema.as_str()) {
        rename.to_camel_case()
    } else {
        schema.to_camel_case()
    }
}

pub fn schema_field(parent: &SchemaName, field: &str) -> SchemaName {
    let parent_type = schema_to_rust_object_name(parent);
    format!("{}_{}", parent_type, field).to_snake_case()
}

pub fn gen_variant_name(wire_name: &SchemaName) -> EnumVariantName {
    match wire_name.as_str() {
        "*" => EnumVariantName::All,
        name => {
            let object_name = if n.chars().next().unwrap().is_digit(10) {
                RustObjectTypeName::new(format!(
                    "V{}",
                    name.to_string().replace('-', "_").replace('.', "_")
                ))
            } else {
                schema_to_rust_object_name(wire_name)
            };
            EnumVariantName::ObjectName(object_name)
        }
    }
}

/// given a spec and a set of objects in that spec, metadatas a
/// map with the requests to implement for each of the types in the spec
pub fn metadata_requests<'a>(
    spec: &'a Spec,
    objects: &BTreeSet<&'a str>,
) -> BTreeMap<SchemaName, BTreeSet<&'a str>> {
    let mut requests = BTreeMap::<SchemaName, BTreeSet<_>>::new();
    for path in spec.paths() {
        let mut seg_iterator = path.trim_start_matches("/v1/").split('/');
        let object = match (seg_iterator.next(), seg_iterator.next(), seg_iterator.next()) {
            // handle special case for sessions
            (Some(x), Some("sessions"), _) => format!("{}.session", x),

            // special case for usage_records
            (_, _, Some("usage_records")) => "usage_records".to_string(),

            (Some(x), _, _) => x.to_string(),
            _ => {
                // this should never happen
                log::error!("path ignored: {path}");
                continue;
            }
        };

        // This isn't documented in the API reference so let's skip it
        if object == "account" {
            continue;
        }

        let seg_like = &object[0..object.len() - 1];
        if objects.contains(object.as_str()) {
            requests.entry(object).or_default().insert(path.as_str());
        } else if object.ends_with('s') && objects.contains(seg_like) {
            requests.entry(seg_like.to_string()).or_default().insert(path.as_str());
        }
    }
    requests
}

#[rustfmt::skip]
fn feature_groups() -> BTreeMap<SchemaName, FeatureGroups> {
   [
		// N.B. For now both `core` and `payment-methods` are always enabled.
		/*
		// Core Resources
		("balance", "core"),
		("balance_transaction", "core"),
		("charge", "core"),
		("customer", "core"),
		("dispute", "core"),
		("file", "core"),
		("file_link", "core"),
		("setup_intent", "core"),
		("payout", "core"),
		("platform_tax_fee", "core"),
		("product", "core"),
		("refund", "core"),
		("reserve_transaction", "core"),
		("token", "core"),
		// Payment Methods
                ("alipay_account", "payment-methods"),
		("bank_account", "payment-methods"),
		("payment_method", "payment-methods"),
		("source", "payment-methods"),
		*/

		// Checkout
		("checkout_session", FeatureGroups::Checkout),

		// Billing (aka. Subscriptions)
		("coupon", FeatureGroups::Billing),
		("discount", FeatureGroups::Billing),
		("invoice", FeatureGroups::Billing),
		("invoiceitem", FeatureGroups::Billing),
        ("line_item", FeatureGroups::Billing),
		("plan", FeatureGroups::Billing),
		("subscription", FeatureGroups::Billing),
		("subscription_item", FeatureGroups::Billing),
		("subscription_schedule", FeatureGroups::Billing),
 		("subscription_schedule_revision", FeatureGroups::Billing),
        ("tax_id", FeatureGroups::Billing),
		("tax_rate", FeatureGroups::Billing),

		// Connect
		("account", FeatureGroups::Connect),
		("application", FeatureGroups::Connect),
		("application_fee", FeatureGroups::Connect),
		("connect_collection_transfer", FeatureGroups::Connect),
		("fee_refund", FeatureGroups::Connect),
		("person", FeatureGroups::Connect),
		("topup", FeatureGroups::Connect),
		("transfer", FeatureGroups::Connect),
		("transfer_reversal", FeatureGroups::Connect),

		// Fraud
		("review", FeatureGroups::Fraud),

		// Issuing
		("issuing.authorization", FeatureGroups::Issuing),
		("issuing.card", FeatureGroups::Issuing),
		("issuing.cardholder", FeatureGroups::Issuing),
		("issuing.dispute", FeatureGroups::Issuing),
		("issuing.transaction", FeatureGroups::Issuing),

		// Orders
		("order", FeatureGroups::Orders),
		("order_item", FeatureGroups::Orders),
		("order_return", FeatureGroups::Orders),
		("sku", FeatureGroups::Orders),

		// Sigma
		("scheduled_query_run", FeatureGroups::Sigma),

		// Webhooks Endpoints
		("webhook_endpoint", FeatureGroups::WebhookEndpoints),
	]
	.iter().map(|(s, feature)| (SchemaName::new(s.to_string()), *feature))
	.collect() 
}
