use crate::rust_object::{ObjectUsage, StructField};
use crate::rust_type::{Container, ExternalType, RustType};

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum ParameterConversion {
    #[default]
    Direct,
    Into,
}

/// Decide how a constructor parameter exposes an existing stored type.
///
/// This is API planning, not formatting: the renderer only prints the chosen
/// conversion. The lifetime-sensitive cases preserve the generator's current
/// signatures exactly.
pub fn constructor_parameter_conversion(typ: &RustType, has_lifetime: bool) -> ParameterConversion {
    if !has_lifetime {
        return ParameterConversion::Into;
    }
    match typ {
        RustType::Reference(_)
        | RustType::Container(Container::Slice(_) | Container::Expandable(_)) => {
            ParameterConversion::Direct
        }
        _ => ParameterConversion::Into,
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SerdeWith {
    JsonValue,
    OptionalJsonValue,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FieldRenderPlan<'a> {
    pub rename: Option<&'a str>,
    pub skip_serializing_if_none: bool,
    pub serde_with: Option<SerdeWith>,
    pub allow_dead_code: bool,
}

impl<'a> FieldRenderPlan<'a> {
    pub fn new(field: &'a StructField, usage: ObjectUsage) -> Self {
        let is_option = field.rust_type.is_option();
        let serde_with = if matches!(
            field.rust_type.with_option_stripped(),
            RustType::External(ExternalType::Value)
        ) {
            Some(if is_option { SerdeWith::OptionalJsonValue } else { SerdeWith::JsonValue })
        } else {
            None
        };

        Self {
            rename: field.rename_name(),
            skip_serializing_if_none: !field.required && is_option && usage.used_as_request_param,
            serde_with,
            allow_dead_code: matches!(
                field.rust_type,
                RustType::External(ExternalType::AlwaysTrue)
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FieldRenderPlan, ParameterConversion, SerdeWith, constructor_parameter_conversion,
    };
    use crate::rust_object::{ObjectUsage, StructField};
    use crate::rust_type::{ExternalType, RustType};

    #[test]
    fn plans_constructor_conversion_before_rendering() {
        let owned = RustType::string();
        let borrowed = RustType::Primitive(crate::rust_type::PrimitiveType::Str).into_reference();
        let expandable = RustType::expandable(RustType::external(ExternalType::Value));

        assert_eq!(constructor_parameter_conversion(&owned, true), ParameterConversion::Into);
        assert_eq!(constructor_parameter_conversion(&borrowed, true), ParameterConversion::Direct);
        assert_eq!(
            constructor_parameter_conversion(&expandable, true),
            ParameterConversion::Direct
        );
        assert_eq!(constructor_parameter_conversion(&expandable, false), ParameterConversion::Into);
    }

    #[test]
    fn plans_field_serde_behavior_before_rendering() {
        let field = StructField::new("payload", RustType::option(RustType::json_value()));
        let plan = FieldRenderPlan::new(&field, ObjectUsage::request_param());

        assert!(plan.skip_serializing_if_none);
        assert_eq!(plan.serde_with, Some(SerdeWith::OptionalJsonValue));
        assert!(!plan.allow_dead_code);

        let discriminant = StructField::new("object", RustType::external(ExternalType::AlwaysTrue));
        let plan = FieldRenderPlan::new(&discriminant, ObjectUsage::type_def());
        assert!(plan.allow_dead_code);
    }
}
