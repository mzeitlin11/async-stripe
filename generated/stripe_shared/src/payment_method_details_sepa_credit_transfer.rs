#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetailsSepaCreditTransfer {
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,
    /// IBAN of the bank account to transfer funds to.
    pub iban: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDetailsSepaCreditTransferBuilder {
    bank_name: Option<Option<String>>,
    bic: Option<Option<String>>,
    iban: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsSepaCreditTransfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsSepaCreditTransfer>,
        builder: PaymentMethodDetailsSepaCreditTransferBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsSepaCreditTransfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDetailsSepaCreditTransferBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsSepaCreditTransferBuilder {
        type Out = PaymentMethodDetailsSepaCreditTransfer;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "bank_name" => Ok(Deserialize::begin(&mut self.bank_name)),
                "bic" => Ok(Deserialize::begin(&mut self.bic)),
                "iban" => Ok(Deserialize::begin(&mut self.iban)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { bank_name: Deserialize::default(), bic: Deserialize::default(), iban: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let bank_name = self.bank_name.take()?;
            let bic = self.bic.take()?;
            let iban = self.iban.take()?;

            Some(Self::Out { bank_name, bic, iban })
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

    impl ObjectDeser for PaymentMethodDetailsSepaCreditTransfer {
        type Builder = PaymentMethodDetailsSepaCreditTransferBuilder;
    }
};
