#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Application {
    /// Unique identifier for the object.
    pub id: stripe_shared::ApplicationId,
    /// The name of the application.
    pub name: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct ApplicationBuilder {
    id: Option<stripe_shared::ApplicationId>,
    name: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Application {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Application>,
        builder: ApplicationBuilder,
    }

    impl Visitor for Place<Application> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ApplicationBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ApplicationBuilder {
        type Out = Application;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "name" => Ok(Deserialize::begin(&mut self.name)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { id: Deserialize::default(), name: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let id = self.id.take()?;
            let name = self.name.take()?;

            Some(Self::Out { id, name })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for Application {
        type Builder = ApplicationBuilder;
    }
};
impl stripe_types::Object for Application {
    type Id = stripe_shared::ApplicationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(ApplicationId, "ca_");
