#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetailsAuBecsDebit {
    /// Bank-State-Branch number of the bank account.
    pub bsb_number: Option<String>,
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// ID of the mandate used to make this payment.
    pub mandate: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDetailsAuBecsDebitBuilder {
    bsb_number: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
    mandate: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsAuBecsDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsAuBecsDebit>,
        builder: PaymentMethodDetailsAuBecsDebitBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsAuBecsDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDetailsAuBecsDebitBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsAuBecsDebitBuilder {
        type Out = PaymentMethodDetailsAuBecsDebit;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "bsb_number" => Ok(Deserialize::begin(&mut self.bsb_number)),
                "fingerprint" => Ok(Deserialize::begin(&mut self.fingerprint)),
                "last4" => Ok(Deserialize::begin(&mut self.last4)),
                "mandate" => Ok(Deserialize::begin(&mut self.mandate)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { bsb_number: Deserialize::default(), fingerprint: Deserialize::default(), last4: Deserialize::default(), mandate: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let bsb_number = self.bsb_number.take()?;
            let fingerprint = self.fingerprint.take()?;
            let last4 = self.last4.take()?;
            let mandate = self.mandate.take()?;

            Some(Self::Out { bsb_number, fingerprint, last4, mandate })
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

    impl ObjectDeser for PaymentMethodDetailsAuBecsDebit {
        type Builder = PaymentMethodDetailsAuBecsDebitBuilder;
    }
};
