#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetailsGiropay {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,
    /// Owner's verified full name. Values are verified or provided by Giropay directly
    /// (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    /// Giropay rarely provides this information so the attribute is usually empty.
    pub verified_name: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDetailsGiropayBuilder {
    bank_code: Option<Option<String>>,
    bank_name: Option<Option<String>>,
    bic: Option<Option<String>>,
    verified_name: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsGiropay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsGiropay>,
        builder: PaymentMethodDetailsGiropayBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsGiropay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDetailsGiropayBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsGiropayBuilder {
        type Out = PaymentMethodDetailsGiropay;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "bank_code" => Ok(Deserialize::begin(&mut self.bank_code)),
                "bank_name" => Ok(Deserialize::begin(&mut self.bank_name)),
                "bic" => Ok(Deserialize::begin(&mut self.bic)),
                "verified_name" => Ok(Deserialize::begin(&mut self.verified_name)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { bank_code: Deserialize::default(), bank_name: Deserialize::default(), bic: Deserialize::default(), verified_name: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let bank_code = self.bank_code.take()?;
            let bank_name = self.bank_name.take()?;
            let bic = self.bic.take()?;
            let verified_name = self.verified_name.take()?;

            Some(Self::Out { bank_code, bank_name, bic, verified_name })
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

    impl ObjectDeser for PaymentMethodDetailsGiropay {
        type Builder = PaymentMethodDetailsGiropayBuilder;
    }
};
