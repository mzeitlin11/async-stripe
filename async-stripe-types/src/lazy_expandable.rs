use std::fmt::{self, Display, Formatter};

use miniserde::json::Value;
use serde::{Serialize, Serializer};

use crate::miniserde_helpers::FromValueOpt;
use crate::{FromCursor, Object};

/// An ID or retained expanded JSON, independent of the resource's model type.
///
/// Parsing reads the JSON eagerly and validates the ID through `I`. Other
/// resource fields are checked only when the existing model decoder is used.
/// This also supports expandable resources without an `object` discriminator.
#[derive(Clone)]
pub struct LazyExpandable<I> {
    value: State<I>,
}

#[derive(Clone)]
enum State<I> {
    Id(I),
    Object { id: I, value: Value },
}

/// A failure to parse or decode a lazy expandable reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LazyExpandableError {
    /// The value was neither a string ID nor an expanded JSON object.
    InvalidValue,
    /// The expanded object lacked an ID permitted by the selected ID type.
    MissingId,
    /// A supplied ID could not be converted to the selected ID type.
    InvalidId,
    /// The expanded payload did not satisfy the selected model's decoder.
    InvalidObject,
}

impl Display for LazyExpandableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidValue => "expected a string ID or expanded object",
            Self::MissingId => "expanded object has no permitted ID",
            Self::InvalidId => "invalid expandable ID",
            Self::InvalidObject => "expanded payload does not satisfy the selected model",
        })
    }
}

impl std::error::Error for LazyExpandableError {}

impl<I> LazyExpandable<I> {
    /// Construct an unexpanded reference from its typed ID.
    pub fn from_id(id: I) -> Self {
        Self { value: State::Id(id) }
    }

    /// Borrow the ID without decoding the resource model.
    pub fn id(&self) -> &I {
        match &self.value {
            State::Id(id) | State::Object { id, .. } => id,
        }
    }

    /// Take the ID without decoding the resource model.
    pub fn into_id(self) -> I {
        match self.value {
            State::Id(id) | State::Object { id, .. } => id,
        }
    }

    /// Whether the response contained an expanded object.
    pub fn is_object(&self) -> bool {
        matches!(self.value, State::Object { .. })
    }

    /// Borrow the expanded JSON, including unknown fields.
    pub fn as_object(&self) -> Option<&Value> {
        match &self.value {
            State::Id(_) => None,
            State::Object { value, .. } => Some(value),
        }
    }

    /// Take the expanded JSON, including unknown fields.
    pub fn into_object(self) -> Option<Value> {
        match self.value {
            State::Id(_) => None,
            State::Object { value, .. } => Some(value),
        }
    }

    /// Decode an expanded model while retaining this reference.
    ///
    /// Each call clones and decodes the retained JSON. The ID need not be
    /// cloneable. An ID-only reference returns `None` without decoding.
    /// The selected model must use the same ID type as this relationship.
    ///
    /// # Errors
    /// Returns an error if expanded data does not satisfy the model's existing
    /// decoder. That decoder also determines discriminator acceptance.
    pub fn to_expanded_as<T: Object<Id = I> + FromValueOpt>(
        &self,
    ) -> Result<Option<T>, LazyExpandableError> {
        self.as_object()
            .map(|value| T::from_value(value.clone()).ok_or(LazyExpandableError::InvalidObject))
            .transpose()
    }

    /// Decode an expanded model by consuming its retained JSON without cloning.
    ///
    /// An ID-only reference returns `None`. The selected model must use the
    /// same ID type as this relationship, including for concrete object unions.
    ///
    /// # Errors
    /// Returns an error if the expanded payload fails the model's existing decoder.
    pub fn into_expanded_as<T: Object<Id = I> + FromValueOpt>(
        self,
    ) -> Result<Option<T>, LazyExpandableError> {
        self.into_object()
            .map(|value| T::from_value(value).ok_or(LazyExpandableError::InvalidObject))
            .transpose()
    }
}

impl<I: FromCursor> LazyExpandable<I> {
    /// Read a string ID or retain an expanded JSON object.
    ///
    /// Optional ID types allow missing/null IDs within an expanded object.
    /// Other resource fields and discriminators are left to the model decoder.
    ///
    /// # Errors
    /// Returns an error for an invalid JSON shape, a missing required ID, or
    /// an ID rejected by `I`.
    pub fn try_from_value(value: Value) -> Result<Self, LazyExpandableError> {
        match value {
            Value::String(id) => {
                I::from_cursor(&id).map(Self::from_id).ok_or(LazyExpandableError::InvalidId)
            }
            Value::Object(object) => {
                let id = match object.get("id") {
                    Some(Value::String(id)) => {
                        I::from_cursor(id).ok_or(LazyExpandableError::InvalidId)?
                    }
                    None | Some(Value::Null) => {
                        I::from_missing_id().ok_or(LazyExpandableError::MissingId)?
                    }
                    _ => return Err(LazyExpandableError::InvalidId),
                };
                Ok(Self { value: State::Object { id, value: Value::Object(object) } })
            }
            _ => Err(LazyExpandableError::InvalidValue),
        }
    }
}

impl<I: Default> Default for LazyExpandable<I> {
    fn default() -> Self {
        Self::from_id(I::default())
    }
}

impl<I: fmt::Debug> fmt::Debug for LazyExpandable<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out = f.debug_struct("LazyExpandable");
        #[cfg(not(feature = "redact-generated-debug"))]
        out.field("id", self.id()).field("object", &self.as_object());
        out.finish_non_exhaustive()
    }
}

impl<I: Serialize> Serialize for LazyExpandable<I> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match &self.value {
            State::Id(id) => id.serialize(serializer),
            State::Object { value, .. } => crate::with_serde_json::serialize(value, serializer),
        }
    }
}

impl<I: FromCursor> FromValueOpt for LazyExpandable<I> {
    fn from_value(value: Value) -> Option<Self> {
        Self::try_from_value(value).ok()
    }
}

#[cfg(feature = "deserialize")]
impl<'de, I: FromCursor> serde::Deserialize<'de> for LazyExpandable<I> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from_value(crate::with_serde_json::deserialize(deserializer)?)
            .map_err(serde::de::Error::custom)
    }
}

mod miniserde_impl {
    use miniserde::Deserialize;
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;

    use super::LazyExpandable;
    use crate::FromCursor;
    use crate::miniserde_helpers::ObjectBuilderInner;

    impl<I: FromCursor> Deserialize for LazyExpandable<I> {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            crate::Place::new(out)
        }
    }

    impl<I: FromCursor> Visitor for crate::Place<LazyExpandable<I>> {
        fn string(&mut self, value: &str) -> miniserde::Result<()> {
            self.out =
                Some(LazyExpandable::from_id(I::from_cursor(value).ok_or(miniserde::Error)?));
            Ok(())
        }

        fn map(&mut self) -> miniserde::Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, inner: ObjectBuilderInner::default() }))
        }
    }

    struct Builder<'a, I> {
        out: &'a mut Option<LazyExpandable<I>>,
        inner: ObjectBuilderInner,
    }

    impl<I: FromCursor> Map for Builder<'_, I> {
        fn key(&mut self, key: &str) -> miniserde::Result<&mut dyn Visitor> {
            self.inner.key_inner(key)
        }

        fn finish(&mut self) -> miniserde::Result<()> {
            *self.out = Some(
                LazyExpandable::try_from_value(Value::Object(self.inner.take_object()))
                    .map_err(|_| miniserde::Error)?,
            );
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests;
