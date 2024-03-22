#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingCardholderCardIssuing {
    /// Information about cardholder acceptance of [Authorized User Terms](https://stripe.com/docs/issuing/cards).
    pub user_terms_acceptance: Option<stripe_shared::IssuingCardholderUserTermsAcceptance>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingCardholderCardIssuingBuilder {
    user_terms_acceptance: Option<Option<stripe_shared::IssuingCardholderUserTermsAcceptance>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingCardholderCardIssuing {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardholderCardIssuing>,
        builder: IssuingCardholderCardIssuingBuilder,
    }

    impl Visitor for Place<IssuingCardholderCardIssuing> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingCardholderCardIssuingBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingCardholderCardIssuingBuilder {
        type Out = IssuingCardholderCardIssuing;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "user_terms_acceptance" => Ok(Deserialize::begin(&mut self.user_terms_acceptance)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { user_terms_acceptance: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let user_terms_acceptance = self.user_terms_acceptance.take()?;

            Some(Self::Out { user_terms_acceptance })
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

    impl ObjectDeser for IssuingCardholderCardIssuing {
        type Builder = IssuingCardholderCardIssuingBuilder;
    }
};
