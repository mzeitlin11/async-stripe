#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentNextActionKonbiniStores {
    /// FamilyMart instruction details.
    pub familymart: Option<stripe_shared::PaymentIntentNextActionKonbiniFamilymart>,
    /// Lawson instruction details.
    pub lawson: Option<stripe_shared::PaymentIntentNextActionKonbiniLawson>,
    /// Ministop instruction details.
    pub ministop: Option<stripe_shared::PaymentIntentNextActionKonbiniMinistop>,
    /// Seicomart instruction details.
    pub seicomart: Option<stripe_shared::PaymentIntentNextActionKonbiniSeicomart>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentIntentNextActionKonbiniStoresBuilder {
    familymart: Option<Option<stripe_shared::PaymentIntentNextActionKonbiniFamilymart>>,
    lawson: Option<Option<stripe_shared::PaymentIntentNextActionKonbiniLawson>>,
    ministop: Option<Option<stripe_shared::PaymentIntentNextActionKonbiniMinistop>>,
    seicomart: Option<Option<stripe_shared::PaymentIntentNextActionKonbiniSeicomart>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentNextActionKonbiniStores {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionKonbiniStores>,
        builder: PaymentIntentNextActionKonbiniStoresBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionKonbiniStores> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentIntentNextActionKonbiniStoresBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionKonbiniStoresBuilder {
        type Out = PaymentIntentNextActionKonbiniStores;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "familymart" => Ok(Deserialize::begin(&mut self.familymart)),
                "lawson" => Ok(Deserialize::begin(&mut self.lawson)),
                "ministop" => Ok(Deserialize::begin(&mut self.ministop)),
                "seicomart" => Ok(Deserialize::begin(&mut self.seicomart)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { familymart: Deserialize::default(), lawson: Deserialize::default(), ministop: Deserialize::default(), seicomart: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let familymart = self.familymart.take()?;
            let lawson = self.lawson.take()?;
            let ministop = self.ministop.take()?;
            let seicomart = self.seicomart.take()?;

            Some(Self::Out { familymart, lawson, ministop, seicomart })
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

    impl ObjectDeser for PaymentIntentNextActionKonbiniStores {
        type Builder = PaymentIntentNextActionKonbiniStoresBuilder;
    }
};
