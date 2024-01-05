#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetailsSepaDebit {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,
    /// Branch code of bank associated with the bank account.
    pub branch_code: Option<String>,
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four characters of the IBAN.
    pub last4: Option<String>,
    /// Find the ID of the mandate used for this payment under the [payment_method_details.sepa_debit.mandate](https://stripe.com/docs/api/charges/object#charge_object-payment_method_details-sepa_debit-mandate) property on the Charge.
    /// Use this mandate ID to [retrieve the Mandate](https://stripe.com/docs/api/mandates/retrieve).
    pub mandate: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDetailsSepaDebitBuilder {
    bank_code: Option<Option<String>>,
    branch_code: Option<Option<String>>,
    country: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsSepaDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsSepaDebit>,
        builder: PaymentMethodDetailsSepaDebitBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsSepaDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDetailsSepaDebitBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsSepaDebitBuilder {
        type Out = PaymentMethodDetailsSepaDebit;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "bank_code" => Ok(Deserialize::begin(&mut self.bank_code)),
                "branch_code" => Ok(Deserialize::begin(&mut self.branch_code)),
                "country" => Ok(Deserialize::begin(&mut self.country)),
                "fingerprint" => Ok(Deserialize::begin(&mut self.fingerprint)),
                "last4" => Ok(Deserialize::begin(&mut self.last4)),
                "mandate" => Ok(Deserialize::begin(&mut self.mandate)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                bank_code: Deserialize::default(),
                branch_code: Deserialize::default(),
                country: Deserialize::default(),
                fingerprint: Deserialize::default(),
                last4: Deserialize::default(),
                mandate: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let bank_code = self.bank_code.take()?;
            let branch_code = self.branch_code.take()?;
            let country = self.country.take()?;
            let fingerprint = self.fingerprint.take()?;
            let last4 = self.last4.take()?;
            let mandate = self.mandate.take()?;

            Some(Self::Out { bank_code, branch_code, country, fingerprint, last4, mandate })
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

    impl ObjectDeser for PaymentMethodDetailsSepaDebit {
        type Builder = PaymentMethodDetailsSepaDebitBuilder;
    }
};
