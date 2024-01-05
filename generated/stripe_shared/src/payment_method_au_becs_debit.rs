#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodAuBecsDebit {
    /// Six-digit number identifying bank and branch associated with this bank account.
    pub bsb_number: Option<String>,
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodAuBecsDebitBuilder {
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

    impl Deserialize for PaymentMethodAuBecsDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodAuBecsDebit>,
        builder: PaymentMethodAuBecsDebitBuilder,
    }

    impl Visitor for Place<PaymentMethodAuBecsDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodAuBecsDebitBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodAuBecsDebitBuilder {
        type Out = PaymentMethodAuBecsDebit;
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

    impl ObjectDeser for PaymentMethodAuBecsDebit {
        type Builder = PaymentMethodAuBecsDebitBuilder;
    }
};
