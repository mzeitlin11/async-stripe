#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodSepaDebit {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,
    /// Branch code of bank associated with the bank account.
    pub branch_code: Option<String>,
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Information about the object that generated this PaymentMethod.
    pub generated_from: Option<stripe_shared::SepaDebitGeneratedFrom>,
    /// Last four characters of the IBAN.
    pub last4: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodSepaDebitBuilder {
    bank_code: Option<Option<String>>,
    branch_code: Option<Option<String>>,
    country: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    generated_from: Option<Option<stripe_shared::SepaDebitGeneratedFrom>>,
    last4: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodSepaDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodSepaDebit>,
        builder: PaymentMethodSepaDebitBuilder,
    }

    impl Visitor for Place<PaymentMethodSepaDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodSepaDebitBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodSepaDebitBuilder {
        type Out = PaymentMethodSepaDebit;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "bank_code" => Ok(Deserialize::begin(&mut self.bank_code)),
                "branch_code" => Ok(Deserialize::begin(&mut self.branch_code)),
                "country" => Ok(Deserialize::begin(&mut self.country)),
                "fingerprint" => Ok(Deserialize::begin(&mut self.fingerprint)),
                "generated_from" => Ok(Deserialize::begin(&mut self.generated_from)),
                "last4" => Ok(Deserialize::begin(&mut self.last4)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                bank_code: Deserialize::default(),
                branch_code: Deserialize::default(),
                country: Deserialize::default(),
                fingerprint: Deserialize::default(),
                generated_from: Deserialize::default(),
                last4: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let bank_code = self.bank_code.take()?;
            let branch_code = self.branch_code.take()?;
            let country = self.country.take()?;
            let fingerprint = self.fingerprint.take()?;
            let generated_from = self.generated_from.take()?;
            let last4 = self.last4.take()?;

            Some(Self::Out { bank_code, branch_code, country, fingerprint, generated_from, last4 })
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

    impl ObjectDeser for PaymentMethodSepaDebit {
        type Builder = PaymentMethodSepaDebitBuilder;
    }
};
