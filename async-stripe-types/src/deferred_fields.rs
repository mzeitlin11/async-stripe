use std::fmt::{self, Formatter};

use miniserde::json::{Object, Value};
use serde::{Serialize, Serializer};

use crate::RecordDecodeError;
use crate::miniserde_helpers::FromValueOpt;

/// Retained JSON fields alongside the typed common fields of a Stripe model.
///
/// The map has no required discriminator or identity. Unknown fields remain
/// available, while concrete field models are constructed only on demand.
#[derive(Clone)]
pub struct DeferredFields {
    value: Value,
}

impl DeferredFields {
    /// Retain an already parsed JSON object without decoding its fields.
    pub fn from_object(object: Object) -> Self {
        Self { value: Value::Object(object) }
    }

    /// Retain a JSON object without decoding its fields.
    ///
    /// # Errors
    /// Returns an error if the supplied value is not an object.
    pub fn try_from_value(value: Value) -> Result<Self, RecordDecodeError> {
        if !matches!(value, Value::Object(_)) {
            return Err(RecordDecodeError::NotObject);
        }
        Ok(Self { value })
    }

    /// Borrow all retained fields, including unknown fields.
    pub fn as_value(&self) -> &Value {
        &self.value
    }

    /// Recover all retained fields.
    pub fn into_value(self) -> Value {
        self.value
    }

    /// Decode an optional field without consuming or changing the retained map.
    ///
    /// Missing and null fields return `None`. Each call clones and decodes only
    /// the selected JSON subtree; no result is cached.
    ///
    /// # Errors
    /// Returns an error if a present, non-null field does not satisfy `T`.
    pub fn field_as<T: FromValueOpt>(
        &self,
        field: &'static str,
    ) -> Result<Option<T>, RecordDecodeError> {
        let Value::Object(object) = &self.value else { return Err(RecordDecodeError::NotObject) };
        match object.get(field) {
            None | Some(Value::Null) => Ok(None),
            Some(value) => T::from_value(value.clone())
                .map(Some)
                .ok_or(RecordDecodeError::InvalidField { field }),
        }
    }

    /// Serialize these entries into a generated model's existing object map.
    #[doc(hidden)]
    pub fn serialize_entries<M: serde::ser::SerializeMap>(
        &self,
        map: &mut M,
    ) -> Result<(), M::Error> {
        let Value::Object(object) = &self.value else {
            return Err(serde::ser::Error::custom("retained fields must be a JSON object"));
        };
        for (key, value) in object {
            map.serialize_entry(key, &crate::with_serde_json::as_serialize(value))?;
        }
        Ok(())
    }
}

impl Default for DeferredFields {
    fn default() -> Self {
        Self::from_object(Object::new())
    }
}

impl fmt::Debug for DeferredFields {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out = f.debug_struct("DeferredFields");
        #[cfg(not(feature = "redact-generated-debug"))]
        out.field("value", &self.value);
        out.finish_non_exhaustive()
    }
}

impl Serialize for DeferredFields {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        crate::with_serde_json::serialize(&self.value, serializer)
    }
}

impl FromValueOpt for DeferredFields {
    fn from_value(value: Value) -> Option<Self> {
        Self::try_from_value(value).ok()
    }
}

#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for DeferredFields {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from_value(crate::with_serde_json::deserialize(deserializer)?)
            .map_err(serde::de::Error::custom)
    }
}

impl miniserde::Deserialize for DeferredFields {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<DeferredFields> {
    fn map(&mut self) -> miniserde::Result<Box<dyn miniserde::de::Map + '_>> {
        Ok(crate::miniserde_helpers::object_value_map(&mut self.out))
    }
}

#[cfg(test)]
mod tests;
