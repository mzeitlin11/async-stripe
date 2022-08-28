use std::collections::{BTreeMap, BTreeSet, HashMap};

use heck::{CamelCase, SnakeCase};
use openapiv3::{
    AdditionalProperties, IntegerFormat, ReferenceOr, Schema, SchemaKind, Type,
    VariantOrUnknownOrEmpty,
};

use crate::util::print_doc_from_schema;
use crate::{
    file_generator::FileGenerator,
    metadata::Metadata,
    types::{
        InferredEnum, InferredObject, InferredParams, InferredStruct, InferredUnion, MethodTypes,
        TypeError,
    },
    url_finder::UrlFinder,
    util::{infer_integer_type, print_doc_comment, write_out_field},
};

#[tracing::instrument(skip_all, fields(name = %state.name))]
pub fn gen_struct(
    out: &mut String,
    state: &mut FileGenerator,
    meta: &Metadata,

    shared_objects: &mut BTreeSet<FileGenerator>,
    url_finder: &UrlFinder,
) {
    let name = state.name.clone();
    let object = &name;

    let id_type = meta.schema_to_id_type(object);
    let struct_name = meta.schema_to_rust_type(object);
    let schema = meta.spec.get_schema_unchecked(object);
    let schema_title = schema.title().unwrap();
    let deleted_schema = meta.spec.get_schema(&format!("deleted_{}", object));

    let fields = schema.properties().unwrap().get_fields();
    log::trace!("struct {} {{...}}", struct_name);

    // Generate the struct type
    out.push_str("/// The resource representing a Stripe \"");
    out.push_str(schema_title);
    out.push_str("\".\n");
    if let Some(doc_url) = url_finder.url_for_object(object) {
        out.push_str("///\n");
        out.push_str("/// For more details see <");
        out.push_str(&doc_url);
        out.push_str(">\n");
    }
    out.push_str("#[derive(Clone, Debug, Default, Deserialize, Serialize)]\n");
    out.push_str("pub struct ");
    out.push_str(&struct_name);
    out.push_str(" {\n");
    if let Some((id_type, _)) = &id_type {
        state.use_ids.insert(id_type.clone());
        if let Some(doc) = schema.get_id_schema().and_then(|id| id.description.as_ref()) {
            print_doc_comment(out, doc, 1);
        }
        if id_type == "InvoiceId" {
            out.push_str("    #[serde(default = \"InvoiceId::none\")]");
        }
        out.push_str("    pub id: ");
        out.push_str(id_type);
        out.push_str(",\n");
    }
    let mut did_emit_deleted = false;
    for (key, field) in fields {
        if key == "id" {
            continue;
        }
        if key == "object" {
            continue;
        }
        if !did_emit_deleted
            && deleted_schema.is_some()
            && key.as_str().cmp("deleted") == std::cmp::Ordering::Greater
        {
            out.push('\n');
            out.push_str("    // Always true for a deleted object\n");
            out.push_str("    #[serde(default)]\n");
            out.push_str("    pub deleted: bool,\n");
            did_emit_deleted = true;
        }
        let required = schema
            .required
            .as_ref()
            .map(|arr| arr.iter().map(|x| x.as_str()).any(|x| x == key))
            .unwrap_or(false);
        let force_optional =
            if let Some(properties) = deleted_schema.and_then(|schema| schema.properties()) {
                properties.get_field(key).is_none()
            } else {
                false
            };
        out.push('\n');
        out.push_str(&gen_field(
            state,
            meta,
            object,
            key,
            field,
            required && !force_optional,
            false,
            shared_objects,
        ));
    }
    out.push_str("}\n");
}

#[tracing::instrument(skip_all)]
pub fn gen_prelude(state: &FileGenerator, meta: &Metadata) -> String {
    let name = state.name.clone();
    let object = &name;

    let mut prelude = String::new();
    prelude.push_str("// ======================================\n");
    prelude.push_str("// This file was automatically generated.\n");
    prelude.push_str("// ======================================\n\n");
    if !state.use_config.is_empty() {
        prelude.push_str("use crate::client::{");
        for (n, type_) in state.use_config.iter().enumerate() {
            if n > 0 {
                prelude.push_str(", ");
            }
            prelude.push_str(type_);
        }
        prelude.push_str("};\n");
    }
    if !state.use_ids.is_empty() {
        prelude.push_str("use crate::ids::{");
        for (n, type_) in state.use_ids.iter().enumerate() {
            if n > 0 {
                prelude.push_str(", ");
            }
            prelude.push_str(type_);
        }
        prelude.push_str("};\n");
    }
    if !state.use_params.is_empty() {
        prelude.push_str("use crate::params::{");
        for (n, type_) in state.use_params.iter().enumerate() {
            if n > 0 {
                prelude.push_str(", ");
            }
            prelude.push_str(type_);
        }
        prelude.push_str("};\n");
    }
    if !state.use_resources.is_empty() {
        prelude.push_str("use crate::resources::{");
        for (n, type_) in state
            .use_resources
            .iter()
            .filter(|&x| {
                state.generated_schemas.keys().all(|sch| x != &meta.schema_to_rust_type(sch))
                    && !state.inferred_parameters.contains_key(x)
                    && !state.inferred_structs.contains_key(x)
                    && !state.inferred_unions.contains_key(x)
                    && !state.inferred_enums.contains_key(x)
                    && x != &meta.schema_to_rust_type(object)
            })
            .enumerate()
        {
            if n > 0 {
                prelude.push_str(", ");
            }
            prelude.push_str(type_);
        }
        prelude.push_str("};\n");
    }
    prelude.push_str("use serde::{Deserialize, Serialize};\n");
    prelude.push('\n');
    prelude
}

#[tracing::instrument(skip_all)]
pub fn gen_generated_schemas(
    out: &mut String,
    state: &mut FileGenerator,
    meta: &Metadata,
    shared_objects: &mut BTreeSet<FileGenerator>,
) {
    while let Some(schema_name) =
        state.generated_schemas.iter().find_map(|(k, &v)| if !v { Some(k) } else { None }).cloned()
    {
        let struct_name = meta.schema_to_rust_type(&schema_name);
        out.push('\n');
        out.push_str("#[derive(Clone, Debug, Default, Deserialize, Serialize)]\n");
        out.push_str("pub struct ");
        out.push_str(&struct_name);
        out.push_str(" {\n");
        for (key, field) in
            meta.spec.get_schema_unchecked(&schema_name).properties().unwrap().get_fields()
        {
            let required = meta
                .spec
                .get_schema_unchecked(&schema_name)
                .required
                .as_ref()
                .map(|arr| arr.iter().map(|x| x.as_str()).any(|x| x == key))
                .unwrap_or(false);
            out.push('\n');
            out.push_str(&gen_field(
                state,
                meta,
                &schema_name,
                key,
                field,
                required,
                false,
                shared_objects,
            ));
        }
        out.push_str("}\n");

        // Set the schema to generated
        *state.generated_schemas.entry(schema_name).or_default() = true;
    }
}

#[tracing::instrument(skip_all)]
pub fn gen_multitype_params(
    parent_struct_rust_type: &str,
    param_name: &str,
    param: &Parameter,
    initializers: &mut Vec<(String, String, bool)>,
    required: bool,
    state: &mut FileGenerator,
    meta: &Metadata,
    out: &mut String,
) {
    let member_schema = param.schema();
    match gen_member_variable_string(member_schema) {
        Ok(type_) => {
            initializers.push((param_name.into(), type_.clone(), required));
            write_out_field(out, param_name, &type_, required);
        }
        Err(TypeError::NoType) => {
            //Weird case, found with anyOf so only case we are handling
            //at the current moment
            if member_schema.any_of().is_some() {
                let mut union_addition = param_name.to_owned();
                union_addition.push_str("_union");
                let mut new_type_name = parent_struct_rust_type.to_owned();
                new_type_name.push_str(&meta.schema_to_rust_type(&union_addition));

                let inferred_object = InferredObject {
                    rust_type: new_type_name.clone(),
                    schema: member_schema.clone(),
                };
                state.generated_objects.insert(new_type_name.clone(), inferred_object);
                initializers.push((param_name.into(), new_type_name.clone(), required));
                write_out_field(out, param_name, &new_type_name, required);
            } else {
                panic!("Strange case, haven't handled this yet: {:#?}", member_schema);
            }
        }
        Err(TypeError::IsObject) => {
            let new_type_name =
                member_schema.title().map(|s| s.to_camel_case()).unwrap_or_else(|| {
                    format!("{}{}Info", parent_struct_rust_type, param_name.to_camel_case())
                });

            let inferred_object =
                InferredObject { rust_type: new_type_name.clone(), schema: member_schema.clone() };
            state.generated_objects.insert(new_type_name.clone(), inferred_object);
            initializers.push((param_name.into(), new_type_name.clone(), required));
            write_out_field(out, param_name, &new_type_name, required);
        }
        _ => {
            panic!("Don't recognize this: {:#?}", member_schema);
        }
    }
}

#[tracing::instrument(skip_all)]
pub fn gen_inferred_params(
    out: &mut String,
    state: &mut FileGenerator,
    meta: &Metadata,
    shared_objects: &mut BTreeSet<FileGenerator>,
) {
    let object = state.name.clone();
    let id_type = meta.schema_to_id_type(&object);
    let struct_name = meta.schema_to_rust_type(&object);

    for (_, params) in state.inferred_parameters.clone() {
        let params_schema = params.rust_type.to_snake_case();
        let parameters = match &params.parameters {
            Some(some) => some.as_slice(),
            None => &[],
        };

        // Derive Default when no param is required
        let can_derive_default = parameters.iter().all(|param| !param.is_required());

        out.push('\n');
        out.push_str(&format!("/// The parameters for `{}::{}`.\n", struct_name, params.method));

        if can_derive_default {
            out.push_str("#[derive(Clone, Debug, Serialize, Default)]\n");
        } else {
            out.push_str("#[derive(Clone, Debug, Serialize)]\n");
        }

        out.push_str("pub struct ");
        out.push_str(&params.rust_type);
        out.push_str("<'a> {\n");
        let mut initializers: Vec<(String, String, bool)> = Vec::new();
        for param in parameters {
            match param.location() {
                "query" | "form" => (),
                _ => continue,
            }

            let param_name = param.name();
            let param_rename = match param_name {
                "type" => "type_",
                other => other,
            };
            let print_doc = |out: &mut String| {
                out.push('\n');
                if let Some(doc) = param.description() {
                    print_doc_comment(out, doc, 1);
                }
                if param_rename != param_name {
                    out.push_str(&format!("    #[serde(rename = \"{}\")]\n", param_name));
                }
            };
            let required = param.is_required();
            match param_name {
                // TODO: Handle these unusual params
                "bank_account" | "usage" => continue,
                "destination" | "card" => {
                    print_doc(out);
                    gen_multitype_params(
                        &params.rust_type,
                        param_name,
                        param,
                        &mut initializers,
                        required,
                        state,
                        meta,
                        out,
                    );
                }
                "product" => {
                    print_doc(out);
                    initializers.push((
                        "product".into(),
                        "IdOrCreate<'a, CreateProduct<'a>>".into(),
                        required,
                    ));
                    state.use_params.insert("IdOrCreate");
                    state.use_resources.insert("CreateProduct".to_owned());
                    if required {
                        out.push_str("    pub product: IdOrCreate<'a, CreateProduct<'a>>,\n");
                    } else {
                        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                        out.push_str(
                            "    pub product: Option<IdOrCreate<'a, CreateProduct<'a>>>,\n",
                        );
                    }
                }
                "metadata" => {
                    print_doc(out);
                    initializers.push(("metadata".into(), "Metadata".into(), required));
                    state.use_params.insert("Metadata");
                    if required {
                        out.push_str("    pub metadata: Metadata,\n");
                    } else {
                        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                        out.push_str("    pub metadata: Option<Metadata>,\n");
                    }
                }
                "expand" => {
                    print_doc(out);
                    initializers.push(("expand".into(), "&'a [&'a str]".into(), false));
                    state.use_params.insert("Expand");
                    out.push_str("    #[serde(skip_serializing_if = \"Expand::is_empty\")]\n");
                    out.push_str("    pub expand: &'a [&'a str],\n");
                }
                "limit" => {
                    print_doc(out);
                    initializers.push(("limit".into(), "u64".into(), false));
                    if required {
                        out.push_str("    pub limit: u64,\n");
                    } else {
                        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                        out.push_str("    pub limit: Option<u64>,\n");
                    }
                }
                "ending_before" => {
                    print_doc(out);
                    let cursor_type =
                        id_type.as_ref().map(|(x, _)| x.as_str()).unwrap_or("&'a str");
                    initializers.push(("ending_before".into(), cursor_type.into(), false));
                    if required {
                        panic!("unexpected \"required\" `ending_before` parameter");
                    } else {
                        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                        out.push_str("    pub ending_before: Option<");
                        out.push_str(cursor_type);
                        out.push_str(">,\n");
                    }
                }
                "starting_after" => {
                    print_doc(out);
                    let cursor_type =
                        id_type.as_ref().map(|(x, _)| x.as_str()).unwrap_or("&'a str");
                    initializers.push(("starting_after".into(), cursor_type.into(), false));
                    if required {
                        panic!("unexpected \"required\" `starting_after` parameter");
                    } else {
                        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                        out.push_str("    pub starting_after: Option<");
                        out.push_str(cursor_type);
                        out.push_str(">,\n");
                    }
                }
                _ => {
                    if let Some((use_path, rust_type)) =
                        meta.field_to_rust_type(params_schema.as_str(), param_name)
                    {
                        print_doc(out);
                        initializers.push((param_rename.into(), rust_type.to_string(), required));
                        match use_path {
                            "" | "String" => {}
                            "Metadata" => {
                                state.use_params.insert("Metadata");
                            }
                            path if path.ends_with("Id") && path != "TaxId" => {
                                state.use_ids.insert(path.into());
                            }
                            path => {
                                state.use_resources.insert(path.into());
                            }
                        }
                        if rust_type.starts_with("Option<") {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                        }
                        out.push_str("    pub ");
                        out.push_str(param_rename);
                        out.push_str(": ");
                        out.push_str(rust_type);
                        out.push_str(",\n");
                    } else if meta.schema_to_id_type(param_name).is_some()
                        && param.schema_type() == Some("string")
                        && param_name != "tax_id"
                    {
                        let (id_type, _) = meta.schema_to_id_type(param_name).unwrap();
                        print_doc(out);
                        initializers.push((param_name.into(), id_type.clone(), required));
                        state.use_ids.insert(id_type.clone());
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_name);
                            out.push_str(": ");
                            out.push_str(&id_type);
                            out.push_str(",\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_name);
                            out.push_str(": Option<");
                            out.push_str(&id_type);
                            out.push_str(">,\n");
                        }
                    } else if param.schema_type() == Some("boolean") {
                        print_doc(out);
                        initializers.push((param_rename.into(), "bool".into(), false));
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": bool,\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Option<bool>,\n");
                        }
                    } else if param.schema_type() == Some("integer") {
                        let rust_type =
                            infer_integer_type(state, param_name, param.schema().format());

                        print_doc(out);
                        initializers.push((param_rename.into(), rust_type.clone(), required));
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": ");
                            out.push_str(&rust_type);
                            out.push_str(",\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Option<");
                            out.push_str(&rust_type);
                            out.push_str(">,\n");
                        }
                    } else if param.schema_type() == Some("number") {
                        print_doc(out);
                        initializers.push((param_rename.into(), "f64".into(), required));
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": f64,\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Option<f64>,\n");
                        }
                    } else if param
                        .schema()
                        .any_of()
                        .and_then(|schemas| schemas.first())
                        .and_then(|schema| schema.title())
                        == Some("range_query_specs")
                    {
                        print_doc(out);
                        initializers.push((
                            param_rename.into(),
                            "RangeQuery<Timestamp>".into(),
                            required,
                        ));
                        state.use_params.insert("RangeQuery");
                        state.use_params.insert("Timestamp");
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": RangeQuery<Timestamp>,\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Option<RangeQuery<Timestamp>>,\n");
                        }
                    } else if param.schema_type() == Some("string")
                        && param.schema().get_enum_strings().is_some()
                    {
                        let enum_schema = meta.schema_field(&object, param_rename);
                        let enum_name = meta.schema_to_rust_type(&enum_schema);
                        let enum_ = InferredEnum {
                            parent: params.rust_type.clone(),
                            field: param_rename.into(),
                            options: param.schema().get_enum_strings().unwrap().clone(),
                        };
                        let inserted = state.try_insert_enum(enum_name.clone(), enum_.clone());
                        let enum_name = if inserted.is_err() {
                            let enum_schema = format!("{}_filter", enum_schema);
                            let enum_name = meta.schema_to_rust_type(&enum_schema);
                            state.insert_enum(enum_name.clone(), enum_);
                            enum_name
                        } else {
                            enum_name
                        };

                        print_doc(out);
                        initializers.push((param_rename.into(), enum_name.clone(), required));
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": ");
                            out.push_str(&enum_name);
                            out.push_str(",\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Option<");
                            out.push_str(&enum_name);
                            out.push_str(">,\n");
                        }
                    } else if (param_name == "currency" || param_name.ends_with("_currency"))
                        && param.schema_type() == Some("string")
                    {
                        print_doc(out);
                        initializers.push((param_rename.into(), "Currency".into(), required));
                        state.use_resources.insert("Currency".into());
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Currency,\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Option<Currency>,\n");
                        }
                    } else if param.schema_type() == Some("string") {
                        print_doc(out);
                        initializers.push((param_rename.into(), "&'a str".into(), required));
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": &'a str,\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Option<&'a str>,\n");
                        }
                    } else if param.schema_type() == Some("object")
                        || param.schema_type() == Some("array")
                        || param.schema().path_ref().is_some()
                        || param.schema().any_of().is_some()
                    {
                        let rust_type = gen_field_rust_type(
                            state,
                            meta,
                            &params.rust_type.to_snake_case(),
                            param_rename,
                            param.schema(),
                            required,
                            false,
                            shared_objects,
                        );
                        initializers.push((param_rename.into(), rust_type.clone(), required));

                        print_doc(out);
                        if !required {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                        }
                        out.push_str("    pub ");
                        out.push_str(param_rename);
                        out.push_str(": ");
                        out.push_str(&rust_type);
                        out.push_str(",\n");
                    } else if required {
                        panic!(
                            "error: skipped required parameter: {} => {:?}",
                            param_name,
                            param.schema()
                        );
                    } else {
                        log::warn!("skipping optional parameter: {}", param_name);
                    }
                }
            }
        }
        out.push_str("}\n");
        out.push('\n');
        out.push_str("impl<'a> ");
        out.push_str(&params.rust_type);
        out.push_str("<'a> {\n");
        out.push_str("    pub fn new(");
        let mut required_count = 0;
        for (name, type_, required) in &initializers {
            if *required {
                if required_count > 0 {
                    out.push_str(", ");
                }
                out.push_str(name);
                out.push_str(": ");
                out.push_str(type_);
                required_count += 1;
            }
        }
        out.push_str(") -> Self {\n");
        out.push_str("        ");
        out.push_str(&params.rust_type);
        out.push_str(" {\n");
        for (name, _, required) in &initializers {
            out.push_str("            ");
            out.push_str(name);
            if *required {
                out.push_str(",\n");
            } else {
                out.push_str(": Default::default(),\n");
            }
        }
        out.push_str("        }\n");
        out.push_str("    }\n");
        out.push_str("}\n");

        // we implement paginate on lists that have an Id
        if let ("list", Some(_)) = (params.method.as_str(), &id_type) {
            state.use_params.insert("Paginable");

            out.push_str("impl Paginable for ");
            out.push_str(&params.rust_type);
            out.push_str("<'_> {\n");
            out.push_str("    type O = ");
            out.push_str(&struct_name);
            out.push_str(";\n");
            out.push_str(
                "    fn set_last(&mut self, item: Self::O) {
                self.starting_after = Some(item.id());
            }",
            );
            out.push_str("}");
        }
    }
}

#[tracing::instrument(skip_all)]
pub fn gen_emitted_structs(
    out: &mut String,
    state: &mut FileGenerator,
    meta: &Metadata,
    shared_objects: &mut BTreeSet<FileGenerator>,
) {
    let mut emitted_structs = BTreeSet::new();
    while state
        .inferred_structs
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>()
        .difference(&emitted_structs)
        .any(|_| true)
    {
        for (struct_name, struct_) in state.inferred_structs.clone() {
            if emitted_structs.contains(&struct_name) {
                continue;
            } else {
                emitted_structs.insert(struct_name.clone());
                log::trace!("struct {} {{ ... }}", struct_name);
            }

            let fields = match struct_.schema.properties() {
                None => {
                    // TODO: Handle these objects
                    log::warn!("{} has no properties ({:#?})", struct_name, struct_.schema);
                    continue;
                }
                Some(f) => f,
            };

            out.push('\n');
            out.push_str("#[derive(Clone, Debug, Default, Deserialize, Serialize)]\n");
            out.push_str("pub struct ");
            out.push_str(&struct_name.to_camel_case());
            out.push_str(" {\n");

            let required_arr = struct_.schema.required.as_ref();
            for (key, field) in fields.get_fields() {
                let required = required_arr
                    .map(|arr| arr.iter().map(|x| x.as_str()).any(|x| x == key))
                    .unwrap_or(false);
                out.push('\n');
                out.push_str(&gen_field(
                    state,
                    meta,
                    &struct_name.to_snake_case(),
                    key,
                    field,
                    required,
                    false,
                    shared_objects,
                ));
            }
            out.push_str("}\n");
        }
    }
}

#[tracing::instrument(skip_all)]
pub fn gen_unions(out: &mut String, unions: &BTreeMap<String, InferredUnion>, meta: &Metadata) {
    for (union_name, union_) in unions {
        log::trace!("union {} {{ ... }}", union_name);

        out.push('\n');
        out.push_str("#[derive(Clone, Debug, Deserialize, Serialize)]\n");
        out.push_str("#[serde(untagged, rename_all = \"snake_case\")]\n");
        out.push_str("pub enum ");
        out.push_str(&union_name.to_camel_case());
        out.push_str(" {\n");
        for variant_schema in &union_.schema_variants {
            let schema = meta.spec.get_schema_unchecked(variant_schema);
            let object_name =
                schema.get_object_enum_name().unwrap_or_else(|| schema.title().unwrap());
            let variant_name = meta.schema_to_rust_type(object_name);
            let type_name = meta.schema_to_rust_type(variant_schema);
            if variant_name.to_snake_case() != object_name {
                out.push_str("    #[serde(rename = \"");
                out.push_str(object_name);
                out.push_str("\")]\n");
            }
            out.push_str("    ");
            out.push_str(&variant_name);
            out.push('(');
            out.push_str(&type_name);
            out.push_str("),\n");
        }
        out.push_str("}\n");

        if let Some(first) = union_
            .schema_variants
            .iter()
            .filter_map(|var| match var.trim() {
                "" => None,
                n => Some(n),
            })
            .map(|s| gen_variant_name(s, meta))
            .next()
        {
            out.push_str("impl std::default::Default for ");
            out.push_str(&union_name.to_camel_case());
            out.push_str(" {\n");
            out.push_str("    fn default() -> Self {\n");
            out.push_str(&format!("        Self::{}(Default::default())\n", first));
            out.push_str("    }\n");
            out.push_str("}\n");
        }
    }
}

#[tracing::instrument(skip_all)]
pub fn gen_variant_name(wire_name: &str, meta: &Metadata) -> String {
    match wire_name {
        "*" => "All".to_string(),
        n => {
            if n.chars().next().unwrap().is_digit(10) {
                format!("V{}", n.to_string().replace('-', "_").replace('.', "_"))
            } else {
                meta.schema_to_rust_type(wire_name)
            }
        }
    }
}

#[tracing::instrument(skip_all)]
pub fn gen_enums(out: &mut String, enums: &BTreeMap<String, InferredEnum>, meta: &Metadata) {
    for (enum_name, enum_) in enums {
        log::trace!("enum {} {{ ... }}", enum_name);

        out.push('\n');
        out.push_str(&format!(
            "/// An enum representing the possible values of an `{}`'s `{}` field.\n",
            enum_.parent, enum_.field
        ));
        out.push_str("#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]\n");
        out.push_str("#[serde(rename_all = \"snake_case\")]\n");
        out.push_str("pub enum ");
        out.push_str(enum_name);
        out.push_str(" {\n");
        for wire_name in &enum_.options {
            if wire_name.trim().is_empty() {
                continue;
            }
            let variant_name = gen_variant_name(wire_name.as_str(), meta);
            if variant_name.trim().is_empty() {
                panic!("unhandled enum variant: {:?}", wire_name)
            }
            if &variant_name.to_snake_case() != wire_name {
                out.push_str("    #[serde(rename = \"");
                out.push_str(wire_name);
                out.push_str("\")]\n");
            }
            out.push_str("    ");
            out.push_str(&variant_name);
            out.push_str(",\n");
        }
        out.push_str("}\n");
        out.push('\n');
        out.push_str("impl ");
        out.push_str(enum_name);
        out.push_str(" {\n");
        out.push_str("    pub fn as_str(self) -> &'static str {\n");
        out.push_str("        match self {\n");
        for wire_name in &enum_.options {
            if wire_name.trim().is_empty() {
                continue;
            }
            let variant_name = gen_variant_name(wire_name.as_str(), meta);
            out.push_str("            ");
            out.push_str(enum_name);
            out.push_str("::");
            out.push_str(&variant_name);
            out.push_str(" => ");
            out.push_str(&format!("{:?}", wire_name));
            out.push_str(",\n");
        }
        out.push_str("        }\n");
        out.push_str("    }\n");
        out.push_str("}\n");
        out.push('\n');
        out.push_str("impl AsRef<str> for ");
        out.push_str(enum_name);
        out.push_str(" {\n");
        out.push_str("    fn as_ref(&self) -> &str {\n");
        out.push_str("        self.as_str()\n");
        out.push_str("    }\n");
        out.push_str("}\n");
        out.push('\n');
        out.push_str("impl std::fmt::Display for ");
        out.push_str(enum_name);
        out.push_str(" {\n");
        out.push_str("    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {\n");
        out.push_str("        self.as_str().fmt(f)\n");
        out.push_str("    }\n");
        out.push_str("}\n");

        if let Some(first) = enum_
            .options
            .iter()
            .filter_map(|var| match var.trim() {
                "" => None,
                n => Some(n),
            })
            .map(|s| gen_variant_name(s, meta))
            .next()
        {
            out.push_str("impl std::default::Default for ");
            out.push_str(enum_name);
            out.push_str(" {\n");
            out.push_str("    fn default() -> Self {\n");
            out.push_str(&format!("        Self::{}\n", first));
            out.push_str("    }\n");
            out.push_str("}\n");
        }
    }
}

#[tracing::instrument(skip_all)]
pub fn gen_member_variable_string(schema: &Schema) -> Result<String, TypeError> {
    if let Some(type_) = schema.get_type() {
        match type_ {
            "integer" => Ok("i32".into()),
            "string" => Ok("String".into()),
            "boolean" => Ok("bool".into()),
            "array" => Ok(format!(
                "Vec<{}>",
                gen_member_variable_string(schema.items.as_ref().unwrap()).unwrap()
            )),
            "object" => Err(TypeError::IsObject),
            _ => Err(TypeError::Unhandled),
        }
    } else {
        Err(TypeError::NoType)
    }
}

#[tracing::instrument(skip_all)]
pub fn gen_objects(out: &mut String, objects: &BTreeMap<String, InferredObject>) {
    let mut generated_objects = objects.clone();
    while !generated_objects.is_empty() {
        let key_str: String;
        let value_obj: InferredObject;
        {
            let (key, value) = generated_objects.iter().next().unwrap();
            log::trace!("object: {} -- {:#?}", key, value);
            key_str = key.clone();
            value_obj = value.clone();
        }

        //Okay, we need something more general for these common
        //cases.  Hopefully, they aren't too common.  Right now, we
        //don't have a clear way to handle 2nd level nested new types
        if key_str == "Metadata" {
            generated_objects.remove(&key_str);
            continue;
        }

        let schema = value_obj.schema.clone();
        out.push('\n');
        print_doc_from_schema(out, &schema, 0);

        if let Some(type_) = schema.get_type() {
            match type_ {
                "object" => {
                    out.push_str("#[derive(Clone, Debug, Default, Deserialize, Serialize)]\n");
                    out.push_str(&format!("pub struct {} {{\n", key_str));
                    let prop_map = match schema.properties() {
                        None => panic!("Object has no properties: {:#?}", schema),
                        Some(p) => p,
                    };
                    let required = schema.required.clone().unwrap_or_default();

                    for (member_name, member_schema) in prop_map.get_fields() {
                        let mut is_required = false;
                        if required.iter().any(|val| val == member_name) {
                            is_required = true;
                        }
                        print_doc_from_schema(out, member_schema, 1);
                        match gen_member_variable_string(member_schema) {
                            Ok(normal_var) => {
                                write_out_field(out, member_name, &normal_var, is_required)
                            }
                            Err(TypeError::IsObject) => {
                                let rust_type = member_name.to_camel_case();
                                write_out_field(out, member_name, &rust_type, is_required);
                                let new_params = InferredObject {
                                    rust_type: rust_type.clone(),
                                    schema: member_schema.clone(),
                                };
                                generated_objects.insert(rust_type, new_params);
                            }
                            _ => {
                                panic!("Unhandled case, inspect: {:#?}", member_schema);
                            }
                        }
                    }
                    out.push_str("}\n");
                }
                other => panic!("Expected an object here got: {}", other),
            }
        };

        if let Some(array) = schema.any_of() {
            out.push_str("#[derive(Clone, Debug, Deserialize, Serialize)]\n");
            out.push_str("#[serde(untagged, rename_all = \"snake_case\")]\n");
            out.push_str(&format!("pub enum {} {{\n", key_str));

            let mut variants = HashMap::new();

            for value in array {
                let type_name = gen_member_variable_string(value).unwrap_or_else(|_| {
                    let type_name = value.title().unwrap().to_camel_case();

                    generated_objects.insert(
                        type_name.clone(),
                        InferredObject { rust_type: type_name.clone(), schema: value.clone() },
                    );

                    type_name
                });

                // if the title is not provided, the variant
                // should have the same name as the type it contains
                // with an optional suffix if there are clashes
                let variant_name = value.title().map(|s| s.to_camel_case()).unwrap_or_else(|| {
                    let count = variants.entry(type_name.clone()).or_insert(0);
                    let suffix = if *count == 0 { "".to_string() } else { count.to_string() };
                    *count += 1;

                    format!("{}{}", type_name, suffix)
                });

                out.push_str(&format!("    pub {}({}),\n", variant_name, type_name));
            }
            out.push_str("}\n");
        }

        generated_objects.remove(&key_str);
    }
}

#[tracing::instrument(skip_all)]
pub fn gen_field(
    state: &mut FileGenerator,
    meta: &Metadata,
    object: &str,
    field_name: &str,
    field: &Schema,
    required: bool,
    default: bool,
    shared_objects: &mut BTreeSet<FileGenerator>,
) -> String {
    let mut out = String::new();
    if let Some(doc) = &field.schema_data.description {
        print_doc_comment(&mut out, doc, 1);
    }
    let mut field_rename = field_name.to_snake_case();
    if field_rename == "type" {
        field_rename = "type_".into();
    }
    if field_rename != field_name {
        out.push_str("    #[serde(rename = \"");
        out.push_str(field_name);
        out.push_str("\")]\n");
    }
    let rust_type = gen_field_rust_type(
        state,
        meta,
        object,
        field_name,
        field,
        required,
        default,
        shared_objects,
    );
    if !required {
        if rust_type == "bool" || rust_type == "Metadata" || rust_type.starts_with("List<") {
            out.push_str("    #[serde(default)]\n");
        } else if rust_type.starts_with("Option<") {
            out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
        }
    }
    out.push_str("    pub ");
    out.push_str(&field_rename);
    out.push_str(": ");
    out.push_str(&rust_type);
    out.push_str(",\n");
    out
}

fn gen_schema_ref(
    state: &mut FileGenerator,
    meta: &Metadata,
    object: &str,
    path: &str,
    shared_objects: &mut BTreeSet<FileGenerator>,
) -> String {
    let schema_name = path.trim_start_matches("#/components/schemas/");
    let type_name = meta.schema_to_rust_type(schema_name);
    if schema_name != object {
        if meta.objects.contains(schema_name) {
            state.use_resources.insert(type_name.clone());
        } else if meta.dependents.get(schema_name).map(|x| x.len()).unwrap_or(0) > 1 {
            state.use_resources.insert(type_name.clone());
            shared_objects.insert(FileGenerator::new(schema_name.to_string()));
        } else if !state.generated_schemas.contains_key(schema_name) {
            state.generated_schemas.insert(schema_name.into(), false);
        }
    }
    type_name
}

fn gen_schema_or_ref_type(
    state: &mut FileGenerator,
    meta: &Metadata,
    object: &str,
    field_name: &str,
    field: &ReferenceOr<openapiv3::Schema>,
    required: bool,
    default: bool,
    shared_objects: &mut BTreeSet<FileGenerator>,
) -> String {
    match field {
        ReferenceOr::Reference { reference } => {
            gen_schema_ref(state, meta, object, reference, shared_objects)
        }
        ReferenceOr::Item(schema) => gen_field_type(
            state,
            meta,
            object,
            field_name,
            schema,
            required,
            default,
            shared_objects,
        ),
    }
}

#[tracing::instrument(skip_all)]
fn gen_field_type(
    state: &mut FileGenerator,
    meta: &Metadata,
    object: &str,
    field_name: &str,
    field: &openapiv3::Schema,
    required: bool,
    default: bool,
    shared_objects: &mut BTreeSet<FileGenerator>,
) -> String {
    match &field.schema_kind {
        SchemaKind::Type(Type::Boolean {}) => {
            if default {
                // N.B. return immediately; if we want to use `Default` for bool rather than `Option`
                return "bool".into();
            } else {
                "bool".into()
            }
        }
        SchemaKind::Type(Type::Number(_)) => "f64".into(),
        SchemaKind::Type(Type::Integer(format)) => {
            let is_unix_time_fmt = match &format.format {
                VariantOrUnknownOrEmpty::Unknown(val) => val == "unix-time",
                _ => false,
            };
            infer_integer_type(state, field_name, is_unix_time_fmt)
        }
        SchemaKind::Type(Type::String(typ)) => {
            let variants = typ.enumeration.iter().flatten().cloned().collect::<Vec<_>>();
            if !variants.is_empty() {
                let enum_schema = meta.schema_field(object, field_name);
                let enum_name = meta.schema_to_rust_type(&enum_schema);
                let parent_type = meta.schema_to_rust_type(object);
                let enum_ = InferredEnum {
                    parent: parent_type,
                    field: field_name.into(),
                    options: variants.clone(),
                };
                state.insert_enum(enum_name.clone(), enum_);
                enum_name
            } else {
                "String".into()
            }
        }
        SchemaKind::Type(Type::Array(typ)) => {
            let element = field.items.as_ref().unwrap();
            let element_type = gen_field_rust_type(
                state,
                meta,
                object,
                field_name,
                element,
                true,
                false,
                shared_objects,
            );
            format!("Vec<{}>", element_type)
        }
        SchemaKind::Type(Type::Object(typ)) => {
            if field.get_object_enum_name() == Some("list") {
                state.use_params.insert("List");
                let element = field.get_data_items_schema();
                let element_field_name = if field_name.ends_with('s') {
                    field_name[0..field_name.len() - 1].into()
                } else if field_name.ends_with("ies") {
                    format!("{}y", &field_name[0..field_name.len() - 3])
                } else {
                    field_name.into()
                };
                let element_type = gen_field_rust_type(
                    state,
                    meta,
                    object,
                    &element_field_name,
                    element,
                    true,
                    false,
                    shared_objects,
                );

                // N.B. return immediately; we use `Default` for list rather than `Option`
                return format!("List<{}>", element_type);
            }

            if let Some(AdditionalProperties::Schema(additional)) = &typ.additional_properties {
                return gen_schema_or_ref_type(
                    state,
                    meta,
                    object,
                    field_name,
                    additional,
                    required,
                    default,
                    shared_objects,
                );
            }

            let struct_schema = meta.schema_field(object, field_name);
            let struct_name = meta.schema_to_rust_type(&struct_schema);
            let struct_ = InferredStruct { field: field_name.into(), schema: field.clone() };
            state.insert_struct(struct_name.clone(), struct_);
            struct_name
        }
        SchemaKind::AllOf { .. } | SchemaKind::OneOf { .. } => {
            let any_of = match &field.schema_kind {
                SchemaKind::AllOf { all_of } => all_of,
                SchemaKind::OneOf { one_of } => one_of,
                _ => unreachable!(),
            };
            if any_of.len() == 1
                || (any_of.len() == 2
                    && any_of[1].get_first_enum_value().map(|v| v.is_empty()).unwrap_or_default())
            {
                gen_field_rust_type(
                    state,
                    meta,
                    object,
                    field_name,
                    &any_of[0],
                    true,
                    false,
                    shared_objects,
                )
            } else if let Some(resources) = &field.expansion_resources {
                let schema = Schema::with_one_of(
                    resources
                        .one_of
                        .as_ref()
                        .unwrap()
                        .iter()
                        .filter(|v| {
                            !v.path_ref().unwrap().starts_with("#/components/schemas/deleted_")
                        })
                        .cloned()
                        .collect(),
                );

                let ty_ = gen_field_rust_type(
                    state,
                    meta,
                    object,
                    field_name,
                    &schema,
                    true,
                    false,
                    shared_objects,
                );
                state.use_params.insert("Expandable");
                format!("Expandable<{}>", ty_)
            } else if any_of[0].title() == Some("range_query_specs") {
                state.use_params.insert("RangeQuery");
                state.use_params.insert("Timestamp");
                "RangeQuery<Timestamp>".into()
            } else {
                log::trace!("object: {}, field_name: {}", object, field_name);
                let mut union_addition = field_name.to_owned();
                union_addition.push_str("_union");
                let union_schema = meta.schema_field(object, &union_addition);
                let union_name = meta.schema_to_rust_type(&union_schema);
                log::trace!("union_schema: {}, union_name: {}", union_schema, union_name);
                let union_ = InferredUnion {
                    field: field_name.into(),
                    schema_variants: any_of
                        .iter()
                        .map(|x| {
                            let schema_name = x
                                .path_ref()
                                .unwrap_or_else(|| {
                                    panic!(
                                        "invalid union for `{}.{}`:  {:#?}",
                                        object, field_name, field
                                    )
                                })
                                .trim_start_matches("#/components/schemas/");
                            let type_name = meta.schema_to_rust_type(schema_name);
                            if meta.objects.contains(schema_name)
                                || meta.dependents.get(schema_name).map(|x| x.len()).unwrap_or(0)
                                    > 1
                            {
                                state.use_resources.insert(type_name);
                            } else if !state.generated_schemas.contains_key(schema_name) {
                                state.generated_schemas.insert(schema_name.into(), false);
                            }
                            schema_name.into()
                        })
                        .collect(),
                };
                state.inferred_unions.insert(union_name.clone(), union_);
                union_name
            }
        }
        _ => {
            panic!("unhandled field type for `{}.{}`: {:#?}\n", object, field_name, field)
        }
    }
}

#[tracing::instrument(skip_all)]
pub fn gen_field_rust_type(
    state: &mut FileGenerator,
    meta: &Metadata,
    object: &str,
    field_name: &str,
    field: &ReferenceOr<Schema>,
    required: bool,
    default: bool,
    shared_objects: &mut BTreeSet<FileGenerator>,
) -> String {
    if let Some((use_path, rust_type)) = meta.field_to_rust_type(object, field_name) {
        match use_path {
            "" | "String" => (),
            "Metadata" => {
                state.use_params.insert("Metadata");
            }
            _ => {
                state.use_resources.insert(use_path.into());
            }
        }
        return rust_type.into();
    }
    if field_name == "metadata" {
        state.use_params.insert("Metadata");
        return "Metadata".into();
    } else if (field_name == "currency" || field_name.ends_with("_currency"))
        && matches!(field.schema_kind, SchemaKind::Type(Type::String(_)))
    {
        state.use_resources.insert("Currency".into());
        return if !required || field.schema_data.nullable {
            "Option<Currency>".into()
        } else {
            "Currency".into()
        };
    } else if field_name == "created" {
        state.use_params.insert("Timestamp");
        return if !required || field.schema_data.nullable {
            "Option<Timestamp>".into()
        } else {
            "Timestamp".into()
        };
    }

    let ty = gen_schema_or_ref_type(
        state,
        meta,
        object,
        field_name,
        field,
        required,
        default,
        shared_objects,
    );
    if ty == "bool" && default {
        // N.B. return immediately; if we want to use `Default` for bool rather than `Option`
        // Not sure why this is here, but we want to preserve it for now
        return "bool".into();
    }
    if ty.contains("List<") {
        // N.B. return immediately; we use `Default` for list rather than `Option`
        return ty;
    }

    let optional = !required || field.schema_data.nullable;
    let should_box = ty.as_str() == "ApiErrors";

    match (optional, should_box) {
        (true, true) => format!("Option<Box<{}>>", ty),
        (true, false) => format!("Option<{}>", ty),
        (false, true) => format!("Box<{}>", ty),
        (false, false) => ty,
    }
}

#[tracing::instrument(skip_all, fields(name = %state.name))]
pub fn gen_impl_requests(
    state: &mut FileGenerator,
    meta: &Metadata,
    object_id: Option<&str>,
) -> Option<String> {
    let name = state.name.clone();
    let object = &name;
    let requests = meta.requests.get(object)?;
    let rust_struct = meta.schema_to_rust_type(object);
    log::trace!("impl {} {{ ... }}", rust_struct);

    let mut methods = BTreeMap::new();

    for path in requests {
        // Unchecked is safe here to avoid dealing with an `Option` since these paths come
        // from the spec already
        let request = meta.spec.get_request_unchecked(*path);
        let segments = path.trim_start_matches("/v1/").split('/').collect::<Vec<_>>();

        if let Some(get_request) = request.get("get") {
            let ok_schema = get_request.get_200_resp();
            if ok_schema.is_none() || !get_request.err_schema_expected() {
                continue; // skip generating this unusual request (for now...)
            }
            let doc_comment = get_request.description();
            if ok_schema.unwrap().get_object_enum_name() == Some("list")
                && !methods.contains_key(&MethodTypes::List)
            {
                let params_name = if rust_struct.ends_with('y') {
                    format!("List{}ies", &rust_struct[0..rust_struct.len() - 1])
                } else {
                    format!("List{}s", rust_struct)
                };
                let params = InferredParams {
                    method: "list".into(),
                    rust_type: params_name.clone(),
                    parameters: Some(get_request.parameters().clone()),
                };
                state.inferred_parameters.insert(params_name.to_snake_case(), params);
                state.use_params.insert("List");

                let mut out = String::new();
                out.push('\n');
                print_doc_comment(&mut out, doc_comment, 1);
                out.push_str("    pub fn list(client: &Client, params: &");
                out.push_str(&params_name);
                out.push_str("<'_>) -> Response<List<");
                out.push_str(&rust_struct);
                out.push_str(">> {\n");
                out.push_str("        client.get_query(\"/");
                out.push_str(&segments.join("/"));
                out.push_str("\", &params)\n");
                out.push_str("    }");
                methods.insert(MethodTypes::List, out);
            } else if segments.len() == 2 && !methods.contains_key(&MethodTypes::Retrieve) {
                let id_param = match get_request.get_id_param() {
                    Some(p) => p,
                    None => continue,
                };
                let expand_param = get_request.parameters().iter().find(|p| p.name() == "expand");
                if let Some(id_type) = &object_id {
                    assert!(id_param.is_required());
                    assert_eq!(id_param.style(), "simple");

                    let mut out = String::new();
                    out.push('\n');
                    print_doc_comment(&mut out, doc_comment, 1);
                    out.push_str("    pub fn retrieve(client: &Client, id: &");
                    out.push_str(id_type);
                    if let Some(param) = expand_param {
                        state.use_params.insert("Expand");
                        assert_eq!(param.location(), "query");
                        out.push_str(", expand: &[&str]) -> Response<");
                        out.push_str(&rust_struct);
                        out.push_str("> {\n");
                        out.push_str("        client.get_query(");
                        out.push_str(&format!("&format!(\"/{}/{{}}\", id)", segments[0]));
                        out.push_str(", &Expand { expand })\n");
                    } else {
                        out.push_str(") -> Response<");
                        out.push_str(&rust_struct);
                        out.push_str("> {\n");
                        out.push_str("        client.get(/");
                        out.push_str(&format!("&format!(\"/{}/{{}}\", id)", segments[0]));
                        out.push_str(")\n");
                    }
                    out.push_str("    }");
                    methods.insert(MethodTypes::Retrieve, out);
                }
            }
        }

        if let Some(post_request) = request.get("post") {
            let ok_schema = post_request.get_200_resp();
            if ok_schema.is_none() || !post_request.err_schema_expected() {
                continue; // skip generating this unusual request (for now...)
            }
            // Safe because of above continue
            let ok_schema = ok_schema.unwrap();
            let return_type = if let Some(path) = ok_schema.path_ref() {
                assert!(!ok_schema.nullable);
                let schema = path.trim_start_matches("#/components/schemas/");
                meta.schema_to_rust_type(schema)
            } else {
                continue;
            };

            let doc_comment = post_request.description();
            let parameter_count = post_request.parameters().len();

            let create = doc_comment.contains("Create")
                || doc_comment.contains("create")
                || doc_comment.contains("Adds")
                || doc_comment.contains("adds");

            let update = doc_comment.contains("Update") || doc_comment.contains("update");

            if !methods.contains_key(&MethodTypes::Create) && parameter_count == 0 && create {
                // Just make sure I don't miss anything unexpected
                assert!(post_request.parameters().is_empty());

                // Construct `parameters` from the request body schema
                let create_schema = post_request.get_request_body_schema();
                let create_parameters = create_schema.to_request_parameters();

                let params_name = format!("Create{}", rust_struct);
                let params = InferredParams {
                    method: "create".into(),
                    rust_type: params_name.clone(),
                    parameters: Some(create_parameters),
                };
                state.inferred_parameters.insert(params_name.to_snake_case(), params);

                let mut out = String::new();
                out.push('\n');
                print_doc_comment(&mut out, doc_comment, 1);
                out.push_str("    pub fn create(client: &Client, params: ");
                out.push_str(&params_name);
                out.push_str("<'_>) -> Response<");
                out.push_str(&return_type);
                out.push_str("> {\n");
                out.push_str("        client.post_form(\"/");
                out.push_str(&segments.join("/"));
                out.push_str("\", &params)\n");
                out.push_str("    }");
                methods.insert(MethodTypes::Create, out);
            } else if !methods.contains_key(&MethodTypes::Update) && parameter_count == 1 && update
            {
                // Get the id parameter
                let id_param = match post_request.get_id_param() {
                    Some(p) => p,
                    None => continue,
                };

                // Construct `parameters` from the request body schema
                let update_schema = post_request.get_request_body_schema();
                let update_parameters = update_schema.to_request_parameters();
                let params_name = format!("Update{}", rust_struct);
                let params = InferredParams {
                    method: "update".into(),
                    rust_type: params_name.clone(),
                    parameters: Some(update_parameters),
                };
                state.inferred_parameters.insert(params_name.to_snake_case(), params);

                if let Some(id_type) = &object_id {
                    assert!(id_param.is_required());
                    assert_eq!(id_param.style(), "simple");

                    let mut out = String::new();
                    out.push('\n');
                    print_doc_comment(&mut out, doc_comment, 1);
                    out.push_str("    pub fn update(client: &Client, id: &");
                    out.push_str(id_type);
                    out.push_str(", params: ");
                    out.push_str(&params_name);
                    out.push_str("<'_>) -> Response<");
                    out.push_str(&return_type);
                    out.push_str("> {\n");
                    out.push_str("        client.post_form(");
                    out.push_str(&format!("&format!(\"/{}/{{}}\", id)", segments[0]));
                    out.push_str(", &params)\n");
                    out.push_str("    }");
                    methods.insert(MethodTypes::Update, out);
                }
            } else {
                log::warn!(
                    "unhandled {} for {rust_struct}: POST {path} (already have {:?})",
                    match (create, update) {
                        (true, _) => "CREATE",
                        (_, true) => "UPDATE",
                        _ => "UNKNOWN",
                    },
                    methods.keys()
                );
            }
        }

        if let Some(delete_request) = request.get("delete") {
            let ok_schema = delete_request.get_200_resp();
            if ok_schema.is_none() || !delete_request.err_schema_expected() {
                continue; // skip generating this unusual request (for now...)
            }

            let doc_comment = delete_request.description();
            if segments.len() == 2 && !methods.contains_key(&MethodTypes::Delete) {
                let id_param = match delete_request.get_id_param() {
                    Some(p) => p,
                    None => continue,
                };
                if let Some(id_type) = &object_id {
                    state.use_params.insert("Deleted");
                    assert!(id_param.is_required());
                    assert_eq!(id_param.style(), "simple");

                    let mut out = String::new();
                    out.push('\n');
                    print_doc_comment(&mut out, doc_comment, 1);
                    out.push_str("    pub fn delete(client: &Client, id: &");
                    out.push_str(id_type);
                    out.push_str(") -> Response<Deleted<");
                    out.push_str(id_type);
                    out.push_str(">> {\n");
                    out.push_str("        client.delete(");
                    out.push_str(&format!("&format!(\"/{}/{{}}\", id)", segments[0]));
                    out.push_str(")\n");
                    out.push_str("    }");
                    methods.insert(MethodTypes::Delete, out);
                }
            } else {
                log::warn!("unhandled DELETE for {rust_struct}: {path}");
            }
        }
    }

    if methods.is_empty() {
        None
    } else {
        // Add imports
        state.use_config.insert("Client");
        state.use_config.insert("Response");

        // Output the impl block
        Some(format!(
            "impl {} {{\n{}\n}}\n",
            rust_struct,
            methods.values().map(String::as_str).collect::<Vec<_>>().join("\n")
        ))
    }
}
