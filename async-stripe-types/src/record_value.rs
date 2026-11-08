use std::fmt::{self, Display, Formatter};

use miniserde::json::Value;
use serde::{Serialize, Serializer};
use smol_str::SmolStr;

use crate::miniserde_helpers::FromValueOpt;

/// A failure to read a Stripe record or decode one of its retained fields.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RecordDecodeError {
    /// The supplied JSON value was not an object.
    NotObject,
    /// The object did not have a string `type` discriminator.
    MissingType,
    /// A present, non-null field did not satisfy the requested model.
    InvalidField {
        /// The field whose retained payload could not be decoded.
        field: String,
    },
}

impl Display for RecordDecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotObject => f.write_str("expected a Stripe record object"),
            Self::MissingType => f.write_str("Stripe record has no string type discriminator"),
            Self::InvalidField { field } => {
                write!(f, "invalid payload for Stripe record field {field}")
            }
        }
    }
}

impl std::error::Error for RecordDecodeError {}

/// A Stripe record whose fields are retained for optional typed decoding.
///
/// Parsing validates only that the JSON is an object with a string `type`
/// discriminator. An `id` or `object` property is not required. Unknown types
/// and fields remain available without constructing their concrete models.
/// The JSON tree is parsed eagerly; construction of field models is deferred.
#[derive(Clone)]
pub struct RecordValue {
    type_: SmolStr,
    value: Value,
}

impl RecordValue {
    /// Read the record's type while retaining its complete payload.
    ///
    /// # Errors
    /// Returns an error if the value is not an object with a string `type`.
    pub fn try_from_value(value: Value) -> Result<Self, RecordDecodeError> {
        let Value::Object(object) = &value else { return Err(RecordDecodeError::NotObject) };
        let Some(Value::String(type_)) = object.get("type") else {
            return Err(RecordDecodeError::MissingType);
        };
        Ok(Self { type_: SmolStr::from(type_), value })
    }

    /// Borrow the record's `type`, including unrecognized values.
    pub fn type_(&self) -> &str {
        &self.type_
    }

    /// Borrow the complete JSON payload, including unknown fields.
    pub fn as_value(&self) -> &Value {
        &self.value
    }

    /// Recover the complete JSON payload.
    pub fn into_value(self) -> Value {
        self.value
    }

    /// Decode an optional field while retaining the original record.
    ///
    /// Missing and null fields return `None`. Any other value is decoded with
    /// the selected model's decoder, independently of the record's `type`.
    /// This clones only the selected field's JSON subtree. Repeated calls
    /// decode that field again; no result is cached.
    ///
    /// # Errors
    /// Returns an error if a present, non-null field cannot be decoded as `T`.
    pub fn field_as<T: FromValueOpt>(&self, field: &str) -> Result<Option<T>, RecordDecodeError> {
        decode_field(&self.value, field)
    }
}

pub(crate) fn decode_field<T: FromValueOpt>(
    value: &Value,
    field: &str,
) -> Result<Option<T>, RecordDecodeError> {
    let Value::Object(object) = value else { return Err(RecordDecodeError::NotObject) };
    match object.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => T::from_value(value.clone())
            .map(Some)
            .ok_or_else(|| RecordDecodeError::InvalidField { field: field.to_owned() }),
    }
}

impl fmt::Debug for RecordValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out = f.debug_struct("RecordValue");
        #[cfg(not(feature = "redact-generated-debug"))]
        out.field("type", &self.type_).field("value", &self.value);
        out.finish_non_exhaustive()
    }
}

impl Serialize for RecordValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        crate::with_serde_json::serialize(&self.value, serializer)
    }
}

impl FromValueOpt for RecordValue {
    fn from_value(value: Value) -> Option<Self> {
        Self::try_from_value(value).ok()
    }
}

#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for RecordValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from_value(crate::with_serde_json::deserialize(deserializer)?)
            .map_err(serde::de::Error::custom)
    }
}

impl miniserde::Deserialize for RecordValue {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<RecordValue> {
    fn map(&mut self) -> miniserde::Result<Box<dyn miniserde::de::Map + '_>> {
        Ok(crate::miniserde_helpers::object_value_map(&mut self.out))
    }
}

/// Define a named Stripe record backed by retained JSON.
///
/// Generated typed field accessors can call `self.field_as(field)` from their
/// decoder module. This macro uses that crate's `Place` and dependencies,
/// following the conventions of the generated object and ID macros.
#[doc(hidden)]
#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! def_record_value {
    ($struct_name:ident) => {
        /// A Stripe record with optional typed field decoding.
        #[derive(Clone, Debug)]
        pub struct $struct_name($crate::RecordValue);

        impl $struct_name {
            /// Borrow the record's type, including unrecognized values.
            pub fn type_(&self) -> &str {
                self.0.type_()
            }

            /// Borrow the complete JSON record, including unknown properties.
            pub fn as_value(&self) -> &miniserde::json::Value {
                self.0.as_value()
            }

            /// Recover the complete JSON record.
            pub fn into_value(self) -> miniserde::json::Value {
                self.0.into_value()
            }

            /// Decode an optional field while retaining the original record.
            ///
            /// Missing and null fields return `None`, independently of `type`.
            /// Each call clones and decodes only the selected JSON subtree.
            ///
            /// # Errors
            /// Returns an error if a present, non-null field cannot be decoded as `T`.
            pub fn field_as<T: $crate::miniserde_helpers::FromValueOpt>(
                &self,
                field: &str,
            ) -> Result<Option<T>, $crate::RecordDecodeError> {
                self.0.field_as(field)
            }
        }

        impl serde::Serialize for $struct_name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serde::Serialize::serialize(&self.0, serializer)
            }
        }

        #[cfg(feature = "deserialize")]
        impl<'de> serde::Deserialize<'de> for $struct_name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                <$crate::RecordValue as serde::Deserialize>::deserialize(deserializer).map(Self)
            }
        }

        impl $crate::miniserde_helpers::FromValueOpt for $struct_name {
            fn from_value(value: miniserde::json::Value) -> Option<Self> {
                <$crate::RecordValue as $crate::miniserde_helpers::FromValueOpt>::from_value(value)
                    .map(Self)
            }
        }

        impl miniserde::Deserialize for $struct_name {
            fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
                crate::Place::new(out)
            }
        }

        impl miniserde::de::Visitor for crate::Place<$struct_name> {
            fn map(&mut self) -> miniserde::Result<Box<dyn miniserde::de::Map + '_>> {
                Ok($crate::miniserde_helpers::object_value_map(&mut self.out))
            }
        }
    };
}

#[cfg(test)]
mod tests;
