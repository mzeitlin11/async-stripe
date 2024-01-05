use serde::{Deserialize, Serialize};

use crate::Object;

/// An id or object.
///
/// By default stripe will return an id for most fields, but if more detail is
/// necessary the `expand` parameter can be provided to ask for the id to be
/// loaded as an object instead.
///
/// For more details see <https://stripe.com/docs/api/expanding_objects>.
#[derive(Clone, Debug, Serialize, Deserialize)] // TODO: Implement deserialize by hand for better error messages
#[serde(untagged)]
pub enum Expandable<T: Object> {
    Id(T::Id),
    Object(Box<T>),
}

impl<T: Object> Default for Expandable<T>
where
    T::Id: Default,
{
    fn default() -> Self {
        Expandable::Id(Default::default())
    }
}

impl<T: Object> Expandable<T> {
    pub fn is_object(&self) -> bool {
        match self {
            Expandable::Id(_) => false,
            Expandable::Object(_) => true,
        }
    }

    pub fn as_object(&self) -> Option<&T> {
        match self {
            Expandable::Id(_) => None,
            Expandable::Object(obj) => Some(obj),
        }
    }

    pub fn id(&self) -> &T::Id {
        match self {
            Expandable::Id(id) => id,
            Expandable::Object(obj) => obj.id(),
        }
    }

    pub fn into_object(self) -> Option<T> {
        match self {
            Expandable::Id(_) => None,
            Expandable::Object(obj) => Some(*obj),
        }
    }
}

#[cfg(feature = "min-ser")]
mod miniserde {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize};

    use crate::{Expandable, FromCursor, Object};
    make_place!(Place);

    pub trait MapBuilder {
        type Out;

        /// Initial state for the builder. Note that this does _not_ match the `Default` trait, it
        /// matches `miniserde::Deserialize::default` -> specifically we need `Option<Option<>>` to
        /// default to `Some(None)`
        fn deser_default() -> Self;

        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor>;

        fn take_out(&mut self) -> Option<Self::Out>
        where
            Self::Out: Sized;
    }

    pub trait ObjectDeser
    where
        Self: Sized,
    {
        type Builder: MapBuilder<Out = Self>;
    }

    impl<T> Deserialize for Expandable<T>
    where
        T: Object + Deserialize + ObjectDeser,
    {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct ExpandableBuilder<'a, T: ObjectDeser + Object> {
        out: &'a mut Option<Expandable<T>>,
        builder: T::Builder,
    }

    impl<'a, T> Map for ExpandableBuilder<'a, T>
    where
        T: ObjectDeser + Object,
    {
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> miniserde::Result<()> {
            let finalized = self.builder.take_out().ok_or(miniserde::Error)?;
            *self.out = Some(Expandable::Object(Box::new(finalized)));
            Ok(())
        }
    }

    impl<T> Visitor for Place<Expandable<T>>
    where
        T: Object + ObjectDeser,
    {
        fn string(&mut self, s: &str) -> miniserde::Result<()> {
            let val = T::Id::from_cursor(s).ok_or(miniserde::Error)?;
            self.out = Some(Expandable::Id(val));
            Ok(())
        }

        fn map(&mut self) -> miniserde::Result<Box<dyn Map + '_>> {
            Ok(Box::new(ExpandableBuilder {
                out: &mut self.out,
                builder: T::Builder::deser_default(),
            }))
        }
    }
}

#[cfg(feature = "min-ser")]
pub use miniserde::{MapBuilder, ObjectDeser};
