#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodBacsDebit {
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// Sort code of the bank account. (e.g., `10-20-30`)
    pub sort_code: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodBacsDebitBuilder {
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
    sort_code: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodBacsDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodBacsDebit>,
        builder: PaymentMethodBacsDebitBuilder,
    }

    impl Visitor for Place<PaymentMethodBacsDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodBacsDebitBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodBacsDebitBuilder {
        type Out = PaymentMethodBacsDebit;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "fingerprint" => Ok(Deserialize::begin(&mut self.fingerprint)),
                "last4" => Ok(Deserialize::begin(&mut self.last4)),
                "sort_code" => Ok(Deserialize::begin(&mut self.sort_code)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { fingerprint: Deserialize::default(), last4: Deserialize::default(), sort_code: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let fingerprint = self.fingerprint.take()?;
            let last4 = self.last4.take()?;
            let sort_code = self.sort_code.take()?;

            Some(Self::Out { fingerprint, last4, sort_code })
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

    impl ObjectDeser for PaymentMethodBacsDebit {
        type Builder = PaymentMethodBacsDebitBuilder;
    }
};
