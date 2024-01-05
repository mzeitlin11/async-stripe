#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingNetworkTokenMastercard {
    /// A unique reference ID from MasterCard to represent the card account number.
    pub card_reference_id: Option<String>,
    /// The network-unique identifier for the token.
    pub token_reference_id: String,
    /// The ID of the entity requesting tokenization, specific to MasterCard.
    pub token_requestor_id: String,
    /// The name of the entity requesting tokenization, if known.
    /// This is directly provided from MasterCard.
    pub token_requestor_name: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingNetworkTokenMastercardBuilder {
    card_reference_id: Option<Option<String>>,
    token_reference_id: Option<String>,
    token_requestor_id: Option<String>,
    token_requestor_name: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingNetworkTokenMastercard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingNetworkTokenMastercard>,
        builder: IssuingNetworkTokenMastercardBuilder,
    }

    impl Visitor for Place<IssuingNetworkTokenMastercard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingNetworkTokenMastercardBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingNetworkTokenMastercardBuilder {
        type Out = IssuingNetworkTokenMastercard;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "card_reference_id" => Ok(Deserialize::begin(&mut self.card_reference_id)),
                "token_reference_id" => Ok(Deserialize::begin(&mut self.token_reference_id)),
                "token_requestor_id" => Ok(Deserialize::begin(&mut self.token_requestor_id)),
                "token_requestor_name" => Ok(Deserialize::begin(&mut self.token_requestor_name)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { card_reference_id: Deserialize::default(), token_reference_id: Deserialize::default(), token_requestor_id: Deserialize::default(), token_requestor_name: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let card_reference_id = self.card_reference_id.take()?;
            let token_reference_id = self.token_reference_id.take()?;
            let token_requestor_id = self.token_requestor_id.take()?;
            let token_requestor_name = self.token_requestor_name.take()?;

            Some(Self::Out { card_reference_id, token_reference_id, token_requestor_id, token_requestor_name })
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

    impl ObjectDeser for IssuingNetworkTokenMastercard {
        type Builder = IssuingNetworkTokenMastercardBuilder;
    }
};
