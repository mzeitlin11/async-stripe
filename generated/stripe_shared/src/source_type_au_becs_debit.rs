#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceTypeAuBecsDebit {
    pub bsb_number: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SourceTypeAuBecsDebitBuilder {
    bsb_number: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTypeAuBecsDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeAuBecsDebit>,
        builder: SourceTypeAuBecsDebitBuilder,
    }

    impl Visitor for Place<SourceTypeAuBecsDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceTypeAuBecsDebitBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceTypeAuBecsDebitBuilder {
        type Out = SourceTypeAuBecsDebit;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "bsb_number" => Ok(Deserialize::begin(&mut self.bsb_number)),
                "fingerprint" => Ok(Deserialize::begin(&mut self.fingerprint)),
                "last4" => Ok(Deserialize::begin(&mut self.last4)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { bsb_number: Deserialize::default(), fingerprint: Deserialize::default(), last4: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let bsb_number = self.bsb_number.take()?;
            let fingerprint = self.fingerprint.take()?;
            let last4 = self.last4.take()?;

            Some(Self::Out { bsb_number, fingerprint, last4 })
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

    impl ObjectDeser for SourceTypeAuBecsDebit {
        type Builder = SourceTypeAuBecsDebitBuilder;
    }
};
