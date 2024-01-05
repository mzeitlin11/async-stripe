#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptionsLink {
    /// \[Deprecated\] This is a legacy parameter that no longer has any function.
    pub persistent_token: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SetupIntentPaymentMethodOptionsLinkBuilder {
    persistent_token: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SetupIntentPaymentMethodOptionsLink {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentPaymentMethodOptionsLink>,
        builder: SetupIntentPaymentMethodOptionsLinkBuilder,
    }

    impl Visitor for Place<SetupIntentPaymentMethodOptionsLink> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SetupIntentPaymentMethodOptionsLinkBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SetupIntentPaymentMethodOptionsLinkBuilder {
        type Out = SetupIntentPaymentMethodOptionsLink;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "persistent_token" => Ok(Deserialize::begin(&mut self.persistent_token)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { persistent_token: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let persistent_token = self.persistent_token.take()?;

            Some(Self::Out { persistent_token })
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

    impl ObjectDeser for SetupIntentPaymentMethodOptionsLink {
        type Builder = SetupIntentPaymentMethodOptionsLinkBuilder;
    }
};
