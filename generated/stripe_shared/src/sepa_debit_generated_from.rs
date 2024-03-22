#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SepaDebitGeneratedFrom {
    /// The ID of the Charge that generated this PaymentMethod, if any.
    pub charge: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    /// The ID of the SetupAttempt that generated this PaymentMethod, if any.
    pub setup_attempt: Option<stripe_types::Expandable<stripe_shared::SetupAttempt>>,
}
#[cfg(feature = "min-ser")]
pub struct SepaDebitGeneratedFromBuilder {
    charge: Option<Option<stripe_types::Expandable<stripe_shared::Charge>>>,
    setup_attempt: Option<Option<stripe_types::Expandable<stripe_shared::SetupAttempt>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SepaDebitGeneratedFrom {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SepaDebitGeneratedFrom>,
        builder: SepaDebitGeneratedFromBuilder,
    }

    impl Visitor for Place<SepaDebitGeneratedFrom> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SepaDebitGeneratedFromBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SepaDebitGeneratedFromBuilder {
        type Out = SepaDebitGeneratedFrom;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "charge" => Ok(Deserialize::begin(&mut self.charge)),
                "setup_attempt" => Ok(Deserialize::begin(&mut self.setup_attempt)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { charge: Deserialize::default(), setup_attempt: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let charge = self.charge.take()?;
            let setup_attempt = self.setup_attempt.take()?;

            Some(Self::Out { charge, setup_attempt })
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

    impl ObjectDeser for SepaDebitGeneratedFrom {
        type Builder = SepaDebitGeneratedFromBuilder;
    }
};
