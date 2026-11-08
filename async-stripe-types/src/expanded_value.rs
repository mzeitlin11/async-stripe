use std::fmt::{self, Formatter};

use miniserde::json::Value;
use serde::{Serialize, Serializer};

use crate::miniserde_helpers::FromValueOpt;
use crate::object_value::parse_object_id;
use crate::{FromCursor, ObjectDecodeError};

/// An ID or retained expanded resource whose schema has no `object` tag.
///
/// This is used for resources such as Radar rules. It validates the identity
/// through `I`, and defers the rest of the model to its existing value decoder.
/// Tagged Stripe resources use [`crate::ExpandableValue`], which also checks
/// the selected model's wire discriminator.
#[derive(Clone)]
pub struct ExpandedValue<I> {
    value: ExpandedState<I>,
}

#[derive(Clone, Debug)]
enum ExpandedState<I> {
    Id(I),
    Object { id: I, value: Value },
}

impl<I> ExpandedValue<I> {
    /// Construct an unexpanded reference from its typed ID.
    pub fn from_id(id: I) -> Self {
        Self { value: ExpandedState::Id(id) }
    }

    /// Borrow the typed identity without constructing the expanded model.
    pub fn id(&self) -> &I {
        match &self.value {
            ExpandedState::Id(id) | ExpandedState::Object { id, .. } => id,
        }
    }

    /// Recover the typed identity, discarding any retained expanded payload.
    pub fn into_id(self) -> I {
        match self.value {
            ExpandedState::Id(id) | ExpandedState::Object { id, .. } => id,
        }
    }

    /// Whether the reference contains an expanded object.
    pub fn is_object(&self) -> bool {
        matches!(&self.value, ExpandedState::Object { .. })
    }

    /// Borrow the retained expanded JSON object, if present.
    pub fn as_object(&self) -> Option<&Value> {
        match &self.value {
            ExpandedState::Id(_) => None,
            ExpandedState::Object { value, .. } => Some(value),
        }
    }

    /// Recover the retained expanded JSON object, if present.
    pub fn into_object(self) -> Option<Value> {
        match self.value {
            ExpandedState::Id(_) => None,
            ExpandedState::Object { value, .. } => Some(value),
        }
    }

    /// Decode an expanded payload, or return `None` for an unexpanded ID.
    ///
    /// This follows the selected model decoder's structural acceptance rules.
    /// There is no `object` discriminator check because the resource schema has
    /// no such tag. The retained JSON tree is consumed without reparsing.
    ///
    /// # Errors
    /// Returns an error if the expanded payload does not satisfy `T`.
    pub fn into_expanded_as<T: FromValueOpt>(self) -> Result<Option<T>, ObjectDecodeError> {
        match self.value {
            ExpandedState::Id(_) => Ok(None),
            ExpandedState::Object { value, .. } => {
                T::from_value(value).map(Some).ok_or(ObjectDecodeError::InvalidUntaggedObject)
            }
        }
    }
}

impl<I: FromCursor> ExpandedValue<I> {
    /// Read an ID string or expanded object using the selected ID policy.
    ///
    /// Missing or null IDs are allowed only when `I` permits them. Fields other
    /// than `id` are retained without model validation.
    ///
    /// # Errors
    /// Returns an error for a different JSON shape or an invalid identity.
    pub fn try_from_value(value: Value) -> Result<Self, ObjectDecodeError> {
        if let Value::String(id) = value {
            return I::from_cursor(&id).map(Self::from_id).ok_or(ObjectDecodeError::InvalidId);
        }
        let Value::Object(object) = &value else { return Err(ObjectDecodeError::NotObject) };
        let id = parse_object_id(object)?;
        Ok(Self { value: ExpandedState::Object { id, value } })
    }
}

impl<I: Default> Default for ExpandedValue<I> {
    fn default() -> Self {
        Self::from_id(I::default())
    }
}

impl<I: fmt::Debug> fmt::Debug for ExpandedValue<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out = f.debug_struct("ExpandedValue");
        #[cfg(not(feature = "redact-generated-debug"))]
        out.field("value", &self.value);
        out.finish_non_exhaustive()
    }
}

impl<I: Serialize> Serialize for ExpandedValue<I> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match &self.value {
            ExpandedState::Id(id) => id.serialize(serializer),
            ExpandedState::Object { value, .. } => {
                crate::with_serde_json::serialize(value, serializer)
            }
        }
    }
}

impl<I: FromCursor> FromValueOpt for ExpandedValue<I> {
    fn from_value(value: Value) -> Option<Self> {
        Self::try_from_value(value).ok()
    }
}

#[cfg(feature = "deserialize")]
impl<'de, I: FromCursor> serde::Deserialize<'de> for ExpandedValue<I> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from_value(crate::with_serde_json::deserialize(deserializer)?)
            .map_err(serde::de::Error::custom)
    }
}

impl<I: FromCursor> miniserde::Deserialize for ExpandedValue<I> {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl<I: FromCursor> miniserde::de::Visitor for crate::Place<ExpandedValue<I>> {
    fn string(&mut self, value: &str) -> miniserde::Result<()> {
        self.out = Some(ExpandedValue::from_id(I::from_cursor(value).ok_or(miniserde::Error)?));
        Ok(())
    }

    fn map(&mut self) -> miniserde::Result<Box<dyn miniserde::de::Map + '_>> {
        Ok(crate::miniserde_helpers::object_value_map(&mut self.out))
    }
}

#[cfg(test)]
mod tests;
