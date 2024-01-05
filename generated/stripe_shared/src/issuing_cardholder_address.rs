#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingCardholderAddress {
    pub address: stripe_shared::Address,
}
#[cfg(feature = "min-ser")]
pub struct IssuingCardholderAddressBuilder {
    address: Option<stripe_shared::Address>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingCardholderAddress {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardholderAddress>,
        builder: IssuingCardholderAddressBuilder,
    }

    impl Visitor for Place<IssuingCardholderAddress> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingCardholderAddressBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingCardholderAddressBuilder {
        type Out = IssuingCardholderAddress;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "address" => Ok(Deserialize::begin(&mut self.address)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { address: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let address = self.address.take()?;

            Some(Self::Out { address })
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

    impl ObjectDeser for IssuingCardholderAddress {
        type Builder = IssuingCardholderAddressBuilder;
    }
};
