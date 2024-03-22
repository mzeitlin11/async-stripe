#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodAcssDebit {
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Institution number of the bank account.
    pub institution_number: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// Transit number of the bank account.
    pub transit_number: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodAcssDebitBuilder {
    bank_name: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    institution_number: Option<Option<String>>,
    last4: Option<Option<String>>,
    transit_number: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodAcssDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodAcssDebit>,
        builder: PaymentMethodAcssDebitBuilder,
    }

    impl Visitor for Place<PaymentMethodAcssDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodAcssDebitBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodAcssDebitBuilder {
        type Out = PaymentMethodAcssDebit;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "bank_name" => Ok(Deserialize::begin(&mut self.bank_name)),
                "fingerprint" => Ok(Deserialize::begin(&mut self.fingerprint)),
                "institution_number" => Ok(Deserialize::begin(&mut self.institution_number)),
                "last4" => Ok(Deserialize::begin(&mut self.last4)),
                "transit_number" => Ok(Deserialize::begin(&mut self.transit_number)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                bank_name: Deserialize::default(),
                fingerprint: Deserialize::default(),
                institution_number: Deserialize::default(),
                last4: Deserialize::default(),
                transit_number: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let bank_name = self.bank_name.take()?;
            let fingerprint = self.fingerprint.take()?;
            let institution_number = self.institution_number.take()?;
            let last4 = self.last4.take()?;
            let transit_number = self.transit_number.take()?;

            Some(Self::Out { bank_name, fingerprint, institution_number, last4, transit_number })
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

    impl ObjectDeser for PaymentMethodAcssDebit {
        type Builder = PaymentMethodAcssDebitBuilder;
    }
};
