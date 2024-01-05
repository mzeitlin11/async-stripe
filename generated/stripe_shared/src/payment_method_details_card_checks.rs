#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetailsCardChecks {
    /// If a address line1 was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_line1_check: Option<String>,
    /// If a address postal code was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_postal_code_check: Option<String>,
    /// If a CVC was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub cvc_check: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDetailsCardChecksBuilder {
    address_line1_check: Option<Option<String>>,
    address_postal_code_check: Option<Option<String>>,
    cvc_check: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsCardChecks {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsCardChecks>,
        builder: PaymentMethodDetailsCardChecksBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsCardChecks> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDetailsCardChecksBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsCardChecksBuilder {
        type Out = PaymentMethodDetailsCardChecks;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "address_line1_check" => Ok(Deserialize::begin(&mut self.address_line1_check)),
                "address_postal_code_check" => Ok(Deserialize::begin(&mut self.address_postal_code_check)),
                "cvc_check" => Ok(Deserialize::begin(&mut self.cvc_check)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { address_line1_check: Deserialize::default(), address_postal_code_check: Deserialize::default(), cvc_check: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let address_line1_check = self.address_line1_check.take()?;
            let address_postal_code_check = self.address_postal_code_check.take()?;
            let cvc_check = self.cvc_check.take()?;

            Some(Self::Out { address_line1_check, address_postal_code_check, cvc_check })
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

    impl ObjectDeser for PaymentMethodDetailsCardChecks {
        type Builder = PaymentMethodDetailsCardChecksBuilder;
    }
};
