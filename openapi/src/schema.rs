use indexmap::IndexMap;
use openapiv3::{ObjectType, ReferenceOr, Schema, SchemaKind, Type};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct ExpansionResources {
    #[serde(rename = "oneOf")]
    pub one_of: Vec<PathRef>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct PathRef {
    #[serde(rename = "$ref")]
    pub reference: String,
}

pub fn as_object_type(schema: &Schema) -> Option<&ObjectType> {
    match &schema.schema_kind {
        SchemaKind::Type(Type::Object(obj)) => Some(obj),
        _ => None,
    }
}

pub fn as_array_item_schema(schema: &Schema) -> Option<&Schema> {
    let arr = if let SchemaKind::Type(Type::Array(typ)) = &schema.schema_kind {
        typ
    } else {
        return None;
    };
    arr.items.and_then(|s| s.as_item().map(|s| s.as_ref()))
}

pub fn as_object_properties(
    schema: &Schema,
) -> Option<&IndexMap<String, ReferenceOr<Box<Schema>>>> {
    as_object_type(schema).map(|o| &o.properties)
}

pub fn as_object_enum_name(schema: &Schema) -> Option<&str> {
    as_object_properties(schema)
        .and_then(|s| s.get("object"))
        .and_then(|s| s.as_item())
        .and_then(|s| as_first_enum_value(s))
}

pub fn as_enum_strings(schema: &Schema) -> Option<Vec<String>> {
    match &schema.schema_kind {
        SchemaKind::Type(Type::String(typ)) => {
            let variants = typ.enumeration.into_iter().flatten().collect::<Vec<_>>();
            if variants.is_empty() {
                None
            } else {
                Some(variants)
            }
        }
        _ => None,
    }
}

pub fn as_first_enum_value(schema: &Schema) -> Option<&str> {
    as_enum_strings(schema).and_then(|s| s.first()).map(|s| s.as_str())
}

// use std::collections::BTreeMap;
//
// /// Stripe equivalent of https://spec.openapis.org/oas/v3.1.0#schema-object
// #[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Eq, PartialEq, Default)]
// pub struct Schema {
//     /// Short description of the purpose of the data
//     /// https://json-schema.org/understanding-json-schema/reference/generic.html#annotations
//     title: Option<String>,
//     /// The value of properties is an object, where each key is the name of a property and
//     /// each value is a schema used to validate that property.
//     properties: Option<Box<Properties>>,
//     /// List of specified properties which are required
//     /// https://swagger.io/docs/specification/data-models/data-types/#required
//     pub required: Option<Vec<String>>,
//     /// A true value adds "null" to the allowed type specified by the type keyword,
//     /// only if type is explicitly defined within the same Schema Object.
//     #[serde(default)]
//     pub nullable: bool,
//     /// For array-type data. This schema describes each item in the array
//     /// https://json-schema.org/understanding-json-schema/reference/array.html#array
//     pub items: Option<Box<Schema>>,
//     /// Longer description of the purpose of the data
//     /// https://json-schema.org/understanding-json-schema/reference/generic.html#annotations
//     pub description: Option<String>,
//     pub object: Option<String>,
//     #[serde(rename = "type")]
//     /// The data type of the schema
//     type_: Option<String>,
//     /// Specify possible values of the schema type
//     #[serde(rename = "enum")]
//     enum_: Option<EnumData>,
//     #[serde(rename = "$ref")]
//     /// A URI-reference that is resolved to another schema
//     /// https://json-schema.org/understanding-json-schema/structuring.html#ref
//     path_ref: Option<String>,
//     // https://swagger.io/docs/specification/data-models/oneof-anyof-allof-not/
//     #[serde(rename = "anyOf")]
//     /// Specifies this schema should be valid for at least one of the provided sub-schemas
//     any_of: Option<Vec<Schema>>,
//     #[serde(rename = "oneOf")]
//     /// Specifies this schema should be valid for no more than one of the provided sub-schemas
//     one_of: Option<Vec<Schema>>,
//     /// Specific formatting information relevant to this schema's data type
//     format: Option<String>,
//     /// False -> No additional properties allowed
//     /// Schema -> Schema used to validate additional properties not specified in `properties`
//     #[serde(rename = "additionalProperties")]
//     #[serde(default)]
//     additional_properties: AdditionalProperties,
//     /// Any expandable field within a resource contains a set of references to the
//     /// resources that it might be expanded to.
//     /// https://github.com/stripe/openapi#x-expansionresources
//     #[serde(rename = "x-expansionResources")]
//     pub expansion_resources: Option<ExpansionResources>,
// }
//
// #[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
// #[serde(untagged)]
// enum EnumData {
//     Strings(Vec<String>),
//     // Only hit for the property `deleted` in deleted structs
//     Bools(Vec<bool>),
// }
//
// impl EnumData {
//     fn as_strings(&self) -> Option<&Vec<String>> {
//         match self {
//             EnumData::Strings(vals) => Some(vals),
//             EnumData::Bools(_) => None,
//         }
//     }
// }
//
// impl Default for EnumData {
//     fn default() -> Self {
//         EnumData::Strings(vec![])
//     }
// }
//
// impl Schema {
//     pub fn get_id_schema(&self) -> Option<&Schema> {
//         self.properties.as_ref().and_then(|p| p.get_field("id"))
//     }
//
//     pub fn properties(&self) -> Option<&Properties> {
//         self.properties.as_deref()
//     }
//
//     pub fn get_object_enum_name(&self) -> Option<&str> {
//         self.properties
//             .as_ref()
//             .and_then(|p| p.get_field("object"))
//             .and_then(|schema| schema.get_first_enum_value())
//     }
//
//     pub fn get_enum_strings(&self) -> Option<&Vec<String>> {
//         self.enum_.as_ref().and_then(|enum_| enum_.as_strings())
//     }
//
//     pub fn get_first_enum_value(&self) -> Option<&str> {
//         self.get_enum_strings().and_then(|vals| vals.first().map(|v| v.as_str()))
//     }
//
//     pub fn additional_properties(&self) -> Option<&Schema> {
//         match &self.additional_properties {
//             AdditionalProperties::Boolean(false) => None,
//             AdditionalProperties::Boolean(true) => {
//                 // According to the OpenAPI spec, this is not a valid value
//                 panic!("Unexpected value in additionalProperties")
//             }
//             AdditionalProperties::Schema(schema) => Some(schema),
//         }
//     }
//
//     pub fn get_data_items_schema(&self) -> &Schema {
//         self.properties().unwrap().get_field("data").unwrap().items.as_ref().unwrap()
//     }
//
//     pub fn with_one_of(one_of: Vec<Schema>) -> Self {
//         Self { one_of: Some(one_of), ..Default::default() }
//     }
//
//     pub fn any_of(&self) -> Option<&Vec<Schema>> {
//         self.any_of.as_ref()
//     }
//
//     pub fn one_of(&self) -> Option<&Vec<Schema>> {
//         self.one_of.as_ref()
//     }
//
//     pub fn get_type(&self) -> Option<&str> {
//         self.type_.as_deref()
//     }
//
//     pub fn path_ref(&self) -> Option<&str> {
//         self.path_ref.as_deref()
//     }
//
//     pub fn title(&self) -> Option<&str> {
//         self.title.as_deref()
//     }
//
//     pub fn format(&self) -> Option<&str> {
//         self.format.as_deref()
//     }
//
//     pub fn to_request_parameters(&self) -> Vec<Parameter> {
//         let mut params = vec![];
//         for (key, value) in self.properties().unwrap().get_fields() {
//             params.push(Parameter {
//                 location: "form".to_string(),
//                 name: key.clone(),
//                 description: value.description.clone(),
//                 required: self
//                     .required
//                     .as_ref()
//                     .map(|arr| arr.iter().any(|v| v.as_str() == key))
//                     .unwrap_or(false),
//                 schema: value.clone(),
//                 style: "deepObject".to_string(),
//             });
//         }
//         params
//     }
//
//     pub fn equal_without_title_desc(&self, other: &Self) -> bool {
//         let s1 = Self { description: None, title: None, ..self.clone() };
//         let s2 = Self { description: None, title: None, ..other.clone() };
//         s1 == s2
//     }
// }
//
// /// https://json-schema.org/understanding-json-schema/reference/object.html#additional-properties
// /// False -> No additional properties allowed
// /// Schema -> Schema used to validate additional properties not specified in `properties`
// #[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
// #[serde(untagged)]
// pub enum AdditionalProperties {
//     Boolean(bool),
//     Schema(Box<Schema>),
// }
//
// impl Default for AdditionalProperties {
//     fn default() -> Self {
//         Self::Boolean(false)
//     }
// }
//
// #[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
// pub struct ExpansionResources {
//     #[serde(rename = "oneOf")]
//     pub one_of: Option<Vec<Schema>>,
// }
//
// /// https://spec.openapis.org/oas/v3.1.0#properties
// /// https://json-schema.org/understanding-json-schema/reference/object.html#properties
// /// The value of properties is an object, where each key is the name of a property and
// /// each value is a schema used to validate that property.
// #[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Eq, PartialEq, Default)]
// pub struct Properties {
//     #[serde(flatten)]
//     properties: BTreeMap<String, Schema>,
// }
//
// impl Properties {
//     pub fn get_fields(&self) -> &BTreeMap<String, Schema> {
//         &self.properties
//     }
//
//     pub fn get_field(&self, name: &str) -> Option<&Schema> {
//         self.properties.get(name)
//     }
// }
//
// #[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
// struct DeletedSchema {
//     description: Option<String>,
// }
//
// /// Describes a single operation parameter.
// #[derive(Debug, Clone, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
// pub struct Parameter {
//     /// The schema defining the type used for the parameter.
//     schema: Schema,
//     #[serde(rename = "in")]
//     /// The location of the parameter. Possible values are "query", "header", "path" or "cookie".
//     location: String,
//     /// The name of the parameter. Parameter names are case sensitive.
//     name: String,
//     /// Determines whether this parameter is mandatory.
//     required: bool,
//     /// Describes how the parameter value will be serialized depending on the type of
//     /// the parameter value.
//     style: String,
//     /// A brief description of the parameter.
//     description: Option<String>,
// }
//
// impl Parameter {
//     /// A brief description of the parameter.
//     pub fn description(&self) -> Option<&str> {
//         self.description.as_deref()
//     }
//
//     /// The location of the parameter. Possible values are "query", "header", "path" or "cookie".
//     pub fn location(&self) -> &str {
//         &self.location
//     }
//
//     pub fn name(&self) -> &str {
//         &self.name
//     }
//
//     pub fn style(&self) -> &str {
//         &self.style
//     }
//
//     pub fn is_required(&self) -> bool {
//         self.required
//     }
//
//     pub fn schema_type(&self) -> Option<&str> {
//         self.schema().get_type()
//     }
//
//     pub fn schema(&self) -> &Schema {
//         &self.schema
//     }
// }

// use openapiv3::SchemaKind;
//
// #[derive(Debug, Clone, PartialEq)]
// pub struct Schema(openapiv3::Schema);
//
// impl Schema {
//     pub fn new(schema: openapiv3::Schema) -> Self {
//         Self(schema)
//     }
//     pub fn description(&self) -> Option<&str> {
//         self.0.schema_data.description.as_deref()
//     }
//
//     pub fn nullable(&self) -> bool {
//         self.0.schema_data.nullable
//     }
//
//     pub fn kind(&self) -> &SchemaKind {
//         &self.0.schema_kind
//     }
// }
