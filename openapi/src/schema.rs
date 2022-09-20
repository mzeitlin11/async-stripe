use indexmap::IndexMap;
use openapiv3::{
    ObjectType, Operation, Parameter, ParameterData, ParameterSchemaOrContent, QueryStyle,
    ReferenceOr, Response, Schema, SchemaKind, StatusCode, Type,
};

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

pub fn as_data_array_item(obj: &ObjectType) -> Option<&ReferenceOr<Box<Schema>>> {
    let schema = obj.properties.get("data")?.as_item()?;
    if let SchemaKind::Type(Type::Array(typ)) = &schema.schema_kind {
        typ.items.as_ref()
    } else {
        None
    }
}

pub fn as_object_properties(
    schema: &Schema,
) -> Option<&IndexMap<String, ReferenceOr<Box<Schema>>>> {
    as_object_type(schema).map(|o| &o.properties)
}

pub fn as_object_enum_name(schema: &Schema) -> Option<String> {
    as_object_properties(schema)
        .and_then(|s| s.get("object"))
        .and_then(|s| s.as_item())
        .and_then(|s| as_first_enum_value(s))
}

pub fn as_enum_strings(schema: &Schema) -> Option<Vec<String>> {
    match &schema.schema_kind {
        SchemaKind::Type(Type::String(typ)) => {
            let variants = typ.enumeration.clone().into_iter().flatten().collect::<Vec<_>>();
            if variants.is_empty() {
                None
            } else {
                Some(variants)
            }
        }
        _ => None,
    }
}

pub fn as_first_enum_value(schema: &Schema) -> Option<String> {
    as_enum_strings(schema)?.first().cloned()
}

pub fn get_ok_response(operation: &Operation) -> Option<&ReferenceOr<Response>> {
    operation.responses.responses.get(&StatusCode::Code(200))
}

pub fn get_ok_response_schema(operation: &Operation) -> Option<&ReferenceOr<Schema>> {
    let resp = get_ok_response(operation)?.as_item()?;
    resp.content.get("application/json")?.schema.as_ref()
}

pub fn as_any_of_first_item_title(schema: &Schema) -> Option<&str> {
    let any_of = match &schema.schema_kind {
        SchemaKind::AnyOf { any_of } => any_of,
        _ => return None,
    };
    any_of.first()?.as_item()?.schema_data.title.as_deref()
}

pub fn err_schema_expected(operation: &Operation) -> bool {
    operation
        .responses
        .default
        .as_ref()
        .map(|err_schema| match err_schema {
            ReferenceOr::Reference { reference } => reference == "#/components/schemas/error",
            ReferenceOr::Item(_) => true,
        })
        .unwrap_or_default()
}

pub fn non_path_ref_params(operation: &Operation) -> Vec<Parameter> {
    operation.parameters.iter().flat_map(|p| p.as_item()).cloned().collect()
}

pub fn get_request_form_parameters(operation: &Operation) -> Vec<Parameter> {
    let form_schema = operation
        .request_body
        .as_ref()
        .expect("No request body")
        .as_item()
        .expect("Expected item")
        .content
        .get("application/x-www-form-urlencoded")
        .expect("No form content found")
        .schema
        .as_ref()
        .expect("No request schema")
        .as_item()
        .expect("Expected item");
    let obj_type = as_object_type(form_schema).expect("Expected object type schema");
    let properties = &obj_type.properties;
    properties
        .iter()
        .map(|(key, value)| {
            let maybe_item = value.as_item();
            Parameter::Query {
                parameter_data: ParameterData {
                    name: key.clone(),
                    description: maybe_item.and_then(|i| i.schema_data.description.clone()),
                    required: obj_type.required.iter().any(|v| v.as_str() == key),
                    deprecated: None,
                    format: ParameterSchemaOrContent::Schema(value.clone().unbox()),
                    example: None,
                    examples: Default::default(),
                    explode: None,
                    extensions: Default::default(),
                },
                allow_reserved: false,
                style: QueryStyle::DeepObject,
                allow_empty_value: None,
            }
        })
        .collect()
}

pub fn find_param_by_name<'a>(operation: &'a Operation, name: &str) -> Option<&'a Parameter> {
    operation.parameters.iter().find_map(|p| {
        p.as_item().and_then(|p| if p.parameter_data_ref().name == name { Some(p) } else { None })
    })
}

pub fn get_id_param(params: &[ReferenceOr<Parameter>]) -> Option<&Parameter> {
    params.iter().find_map(|p| match p {
        ReferenceOr::Item(param @ Parameter::Path { .. }) => Some(param),
        _ => None,
    })
}
