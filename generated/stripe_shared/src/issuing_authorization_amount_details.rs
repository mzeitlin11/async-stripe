#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingAuthorizationAmountDetails {
    /// The fee charged by the ATM for the cash withdrawal.
    pub atm_fee: Option<i64>,
    /// The amount of cash requested by the cardholder.
    pub cashback_amount: Option<i64>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingAuthorizationAmountDetailsBuilder {
    atm_fee: Option<Option<i64>>,
    cashback_amount: Option<Option<i64>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingAuthorizationAmountDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationAmountDetails>,
        builder: IssuingAuthorizationAmountDetailsBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationAmountDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingAuthorizationAmountDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingAuthorizationAmountDetailsBuilder {
        type Out = IssuingAuthorizationAmountDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "atm_fee" => Ok(Deserialize::begin(&mut self.atm_fee)),
                "cashback_amount" => Ok(Deserialize::begin(&mut self.cashback_amount)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { atm_fee: Deserialize::default(), cashback_amount: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let atm_fee = self.atm_fee.take()?;
            let cashback_amount = self.cashback_amount.take()?;

            Some(Self::Out { atm_fee, cashback_amount })
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

    impl ObjectDeser for IssuingAuthorizationAmountDetails {
        type Builder = IssuingAuthorizationAmountDetailsBuilder;
    }
};
