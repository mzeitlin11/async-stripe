use std::fmt::{self, Display, Formatter};

use miniserde::json::Value;
use serde::{Serialize, Serializer};
use smol_str::SmolStr;

use crate::FromCursor;
use crate::miniserde_helpers::FromValueOpt;

/// The wire discriminator of a concrete Stripe object model.
///
/// A model implements this trait where it is defined. Opaque values can then
/// validate a requested model without depending on it until decoding is used.
/// The tag is not a unique schema identity: live and deleted models can share
/// one. Typed conversion follows the selected model decoder's acceptance rules.
pub trait ObjectType {
    /// The value of the model's JSON `object` property.
    const OBJECT_TYPE: &'static str;
}

/// A failure to read an opaque Stripe object or decode it as a concrete model.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ObjectDecodeError {
    /// The supplied JSON value was not an object or an expandable string ID.
    NotObject,
    /// The object did not have a string `object` discriminator.
    MissingObjectType,
    /// The object did not have a string `id` property.
    MissingId,
    /// The ID could not be converted to the selected ID type.
    InvalidId,
    /// The object discriminator did not match the requested model.
    UnexpectedObjectType {
        /// The discriminator required by the requested model.
        expected: &'static str,
        /// The discriminator present in the payload.
        actual: SmolStr,
    },
    /// The discriminator matched, but the payload did not satisfy the model.
    InvalidObject {
        /// The discriminator of the requested model.
        object_type: &'static str,
    },
}

impl Display for ObjectDecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotObject => f.write_str("expected a Stripe object or string ID"),
            Self::MissingObjectType => {
                f.write_str("Stripe object has no string object discriminator")
            }
            Self::MissingId => f.write_str("Stripe object has no string ID"),
            Self::InvalidId => f.write_str("Stripe object ID is invalid for the selected ID type"),
            Self::UnexpectedObjectType { expected, actual } => {
                write!(f, "expected Stripe object {expected}, found {actual}")
            }
            Self::InvalidObject { object_type } => {
                write!(f, "invalid payload for Stripe object {object_type}")
            }
        }
    }
}

impl std::error::Error for ObjectDecodeError {}

/// A Stripe object whose full payload is retained for optional typed decoding.
///
/// Parsing validates only the string `id` and `object` properties. Unknown
/// discriminators and fields remain available even when no model is enabled.
/// The JSON tree is parsed eagerly; construction of a concrete model is deferred.
#[derive(Clone)]
pub struct ObjectValue {
    id: SmolStr,
    object_type: SmolStr,
    value: Value,
}

impl ObjectValue {
    /// Read the object's identity while retaining its complete payload.
    ///
    /// # Errors
    /// Returns an error if the value is not an object with string `id` and
    /// `object` properties.
    pub fn try_from_value(value: Value) -> Result<Self, ObjectDecodeError> {
        let Value::Object(object) = &value else { return Err(ObjectDecodeError::NotObject) };
        let Some(Value::String(id)) = object.get("id") else {
            return Err(ObjectDecodeError::MissingId);
        };
        let Some(Value::String(object_type)) = object.get("object") else {
            return Err(ObjectDecodeError::MissingObjectType);
        };
        Ok(Self { id: SmolStr::from(id), object_type: SmolStr::from(object_type), value })
    }

    /// The object's ID, without constructing its model.
    pub fn id(&self) -> &SmolStr {
        &self.id
    }

    /// The JSON `object` discriminator, including unrecognized values.
    pub fn object_type(&self) -> &str {
        &self.object_type
    }

    /// Borrow the complete JSON payload.
    pub fn as_value(&self) -> &Value {
        &self.value
    }

    /// Recover the complete JSON payload.
    pub fn into_value(self) -> Value {
        self.value
    }

    /// Decode the retained payload as a concrete model after checking its tag.
    ///
    /// This consumes the JSON tree directly, without serializing and reparsing it.
    ///
    /// # Errors
    /// Returns an error if the model requires another discriminator or the
    /// payload does not satisfy that model's required fields.
    pub fn into_typed<T: ObjectType + FromValueOpt>(self) -> Result<T, ObjectDecodeError> {
        if self.object_type != T::OBJECT_TYPE {
            return Err(ObjectDecodeError::UnexpectedObjectType {
                expected: T::OBJECT_TYPE,
                actual: self.object_type,
            });
        }
        T::from_value(self.value)
            .ok_or(ObjectDecodeError::InvalidObject { object_type: T::OBJECT_TYPE })
    }
}

impl fmt::Debug for ObjectValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out = f.debug_struct("ObjectValue");
        #[cfg(not(feature = "redact-generated-debug"))]
        out.field("id", &self.id)
            .field("object_type", &self.object_type)
            .field("value", &self.value);
        out.finish_non_exhaustive()
    }
}

impl Serialize for ObjectValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        crate::with_serde_json::serialize(&self.value, serializer)
    }
}

impl crate::Object for ObjectValue {
    type Id = SmolStr;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}

impl FromValueOpt for ObjectValue {
    fn from_value(value: Value) -> Option<Self> {
        Self::try_from_value(value).ok()
    }
}

#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ObjectValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from_value(crate::with_serde_json::deserialize(deserializer)?)
            .map_err(serde::de::Error::custom)
    }
}

/// An ID or an expanded object, without a dependency on its concrete model.
///
/// `I` controls typed ID access. The target of typed decoding is validated by
/// its wire discriminator at runtime, allowing polymorphic relationships whose
/// models use different ID newtypes to share the same representation.
#[derive(Clone)]
pub struct ExpandableValue<I> {
    id: I,
    object: Option<ObjectValue>,
}

impl<I: fmt::Debug> fmt::Debug for ExpandableValue<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out = f.debug_struct("ExpandableValue");
        #[cfg(not(feature = "redact-generated-debug"))]
        out.field("id", &self.id).field("object", &self.object);
        out.finish_non_exhaustive()
    }
}

impl<I> ExpandableValue<I> {
    /// Construct an unexpanded reference from an ID.
    pub fn from_id(id: I) -> Self {
        Self { id, object: None }
    }

    /// Borrow the ID without decoding a model.
    pub fn id(&self) -> &I {
        &self.id
    }

    /// Take ownership of the ID without decoding a model.
    pub fn into_id(self) -> I {
        self.id
    }

    /// Whether Stripe supplied an expanded object rather than only an ID.
    pub fn is_object(&self) -> bool {
        self.object.is_some()
    }

    /// Borrow the retained object, if Stripe supplied one.
    pub fn as_object(&self) -> Option<&ObjectValue> {
        self.object.as_ref()
    }

    /// Take the retained object, if Stripe supplied one.
    pub fn into_object(self) -> Option<ObjectValue> {
        self.object
    }

    /// Decode an expanded payload, returning `None` when only an ID was supplied.
    ///
    /// # Errors
    /// Returns an error if an expanded payload has the wrong discriminator or
    /// does not satisfy the selected model.
    pub fn into_expanded_as<T: ObjectType + FromValueOpt>(
        self,
    ) -> Result<Option<T>, ObjectDecodeError> {
        self.object.map(ObjectValue::into_typed).transpose()
    }
}

impl<I: FromCursor> ExpandableValue<I> {
    /// Read an ID string or expanded object while retaining the typed ID.
    ///
    /// # Errors
    /// Returns an error if the value is not a valid expandable value or the
    /// ID cannot be converted to `I`.
    pub fn try_from_value(value: Value) -> Result<Self, ObjectDecodeError> {
        if let Value::String(id) = value {
            return I::from_cursor(&id).map(Self::from_id).ok_or(ObjectDecodeError::InvalidId);
        }
        let object = ObjectValue::try_from_value(value)?;
        let id = I::from_cursor(object.id()).ok_or(ObjectDecodeError::InvalidId)?;
        Ok(Self { id, object: Some(object) })
    }
}

impl<I: Default> Default for ExpandableValue<I> {
    fn default() -> Self {
        Self::from_id(I::default())
    }
}

impl<I: Serialize> Serialize for ExpandableValue<I> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match &self.object {
            Some(object) => object.serialize(serializer),
            None => self.id.serialize(serializer),
        }
    }
}

impl<I: FromCursor> FromValueOpt for ExpandableValue<I> {
    fn from_value(value: Value) -> Option<Self> {
        Self::try_from_value(value).ok()
    }
}

#[cfg(feature = "deserialize")]
impl<'de, I: FromCursor> serde::Deserialize<'de> for ExpandableValue<I> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from_value(crate::with_serde_json::deserialize(deserializer)?)
            .map_err(serde::de::Error::custom)
    }
}

mod miniserde_impl {
    use miniserde::de::{Map, Visitor};
    use miniserde::{Deserialize, make_place};

    use super::{ExpandableValue, ObjectValue};
    use crate::FromCursor;
    use crate::miniserde_helpers::object_value_map;

    make_place!(Place);

    impl Deserialize for ObjectValue {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<ObjectValue> {
        fn map(&mut self) -> miniserde::Result<Box<dyn Map + '_>> {
            Ok(object_value_map(&mut self.out))
        }
    }

    impl<I: FromCursor> Deserialize for ExpandableValue<I> {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl<I: FromCursor> Visitor for Place<ExpandableValue<I>> {
        fn string(&mut self, value: &str) -> miniserde::Result<()> {
            let id = I::from_cursor(value).ok_or(miniserde::Error)?;
            self.out = Some(ExpandableValue::from_id(id));
            Ok(())
        }

        fn map(&mut self) -> miniserde::Result<Box<dyn Map + '_>> {
            Ok(object_value_map(&mut self.out))
        }
    }
}

/// Define a named polymorphic Stripe object backed by an opaque JSON payload.
///
/// The generated type retains its public identity without importing the
/// concrete models its object discriminator may select.
#[doc(hidden)]
#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! def_object_value {
    ($struct_name:ident) => {
        /// A polymorphic Stripe object with optional typed decoding.
        #[derive(Clone, Debug)]
        pub struct $struct_name($crate::ObjectValue);

        impl $struct_name {
            /// Borrow the object's ID without decoding a concrete model.
            pub fn id(&self) -> &<$crate::ObjectValue as $crate::Object>::Id {
                self.0.id()
            }

            /// Borrow the JSON object discriminator, including unknown values.
            pub fn object_type(&self) -> &str {
                self.0.object_type()
            }

            /// Borrow the full JSON object, including unknown properties.
            pub fn as_value(&self) -> &miniserde::json::Value {
                self.0.as_value()
            }

            /// Recover the full JSON object.
            pub fn into_value(self) -> miniserde::json::Value {
                self.0.into_value()
            }

            /// Decode the object after validating the selected model's tag.
            ///
            /// # Errors
            /// Returns an error for a different discriminator or malformed model.
            pub fn into_typed<T: $crate::ObjectType + $crate::miniserde_helpers::FromValueOpt>(
                self,
            ) -> Result<T, $crate::ObjectDecodeError> {
                self.0.into_typed()
            }
        }

        impl $crate::Object for $struct_name {
            type Id = <$crate::ObjectValue as $crate::Object>::Id;

            fn id(&self) -> &Self::Id {
                self.0.id()
            }

            fn into_id(self) -> Self::Id {
                $crate::Object::into_id(self.0)
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
                <$crate::ObjectValue as serde::Deserialize>::deserialize(deserializer).map(Self)
            }
        }

        impl $crate::miniserde_helpers::FromValueOpt for $struct_name {
            fn from_value(value: miniserde::json::Value) -> Option<Self> {
                <$crate::ObjectValue as $crate::miniserde_helpers::FromValueOpt>::from_value(value)
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
