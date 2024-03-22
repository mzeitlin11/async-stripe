#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SubscriptionAutomaticTax {
    /// Whether Stripe automatically computes tax on this subscription.
    pub enabled: bool,
}
#[cfg(feature = "min-ser")]
pub struct SubscriptionAutomaticTaxBuilder {
    enabled: Option<bool>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SubscriptionAutomaticTax {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionAutomaticTax>,
        builder: SubscriptionAutomaticTaxBuilder,
    }

    impl Visitor for Place<SubscriptionAutomaticTax> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SubscriptionAutomaticTaxBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SubscriptionAutomaticTaxBuilder {
        type Out = SubscriptionAutomaticTax;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "enabled" => Ok(Deserialize::begin(&mut self.enabled)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { enabled: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let enabled = self.enabled.take()?;

            Some(Self::Out { enabled })
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

    impl ObjectDeser for SubscriptionAutomaticTax {
        type Builder = SubscriptionAutomaticTaxBuilder;
    }
};
