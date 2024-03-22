#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodLink {
    /// Account owner's email address.
    pub email: Option<String>,
    /// \[Deprecated\] This is a legacy parameter that no longer has any function.
    pub persistent_token: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodLinkBuilder {
    email: Option<Option<String>>,
    persistent_token: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodLink {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodLink>,
        builder: PaymentMethodLinkBuilder,
    }

    impl Visitor for Place<PaymentMethodLink> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodLinkBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodLinkBuilder {
        type Out = PaymentMethodLink;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "email" => Ok(Deserialize::begin(&mut self.email)),
                "persistent_token" => Ok(Deserialize::begin(&mut self.persistent_token)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { email: Deserialize::default(), persistent_token: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let email = self.email.take()?;
            let persistent_token = self.persistent_token.take()?;

            Some(Self::Out { email, persistent_token })
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

    impl ObjectDeser for PaymentMethodLink {
        type Builder = PaymentMethodLinkBuilder;
    }
};
