#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingNetworkTokenVisa {
    /// A unique reference ID from Visa to represent the card account number.
    pub card_reference_id: String,
    /// The network-unique identifier for the token.
    pub token_reference_id: String,
    /// The ID of the entity requesting tokenization, specific to Visa.
    pub token_requestor_id: String,
    /// Degree of risk associated with the token between `01` and `99`, with higher number indicating higher risk.
    /// A `00` value indicates the token was not scored by Visa.
    pub token_risk_score: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingNetworkTokenVisaBuilder {
    card_reference_id: Option<String>,
    token_reference_id: Option<String>,
    token_requestor_id: Option<String>,
    token_risk_score: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingNetworkTokenVisa {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingNetworkTokenVisa>,
        builder: IssuingNetworkTokenVisaBuilder,
    }

    impl Visitor for Place<IssuingNetworkTokenVisa> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingNetworkTokenVisaBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingNetworkTokenVisaBuilder {
        type Out = IssuingNetworkTokenVisa;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "card_reference_id" => Ok(Deserialize::begin(&mut self.card_reference_id)),
                "token_reference_id" => Ok(Deserialize::begin(&mut self.token_reference_id)),
                "token_requestor_id" => Ok(Deserialize::begin(&mut self.token_requestor_id)),
                "token_risk_score" => Ok(Deserialize::begin(&mut self.token_risk_score)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { card_reference_id: Deserialize::default(), token_reference_id: Deserialize::default(), token_requestor_id: Deserialize::default(), token_risk_score: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let card_reference_id = self.card_reference_id.take()?;
            let token_reference_id = self.token_reference_id.take()?;
            let token_requestor_id = self.token_requestor_id.take()?;
            let token_risk_score = self.token_risk_score.take()?;

            Some(Self::Out { card_reference_id, token_reference_id, token_requestor_id, token_risk_score })
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

    impl ObjectDeser for IssuingNetworkTokenVisa {
        type Builder = IssuingNetworkTokenVisaBuilder;
    }
};
