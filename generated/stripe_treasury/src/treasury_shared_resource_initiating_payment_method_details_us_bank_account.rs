#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount {
    /// Bank name.
    pub bank_name: Option<String>,
    /// The last four digits of the bank account number.
    pub last4: Option<String>,
    /// The routing number for the bank account.
    pub routing_number: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccountBuilder {
    bank_name: Option<Option<String>>,
    last4: Option<Option<String>>,
    routing_number: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount>,
        builder: TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccountBuilder,
    }

    impl Visitor for Place<TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccountBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccountBuilder {
        type Out = TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "bank_name" => Ok(Deserialize::begin(&mut self.bank_name)),
                "last4" => Ok(Deserialize::begin(&mut self.last4)),
                "routing_number" => Ok(Deserialize::begin(&mut self.routing_number)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { bank_name: Deserialize::default(), last4: Deserialize::default(), routing_number: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let bank_name = self.bank_name.take()?;
            let last4 = self.last4.take()?;
            let routing_number = self.routing_number.take()?;

            Some(Self::Out { bank_name, last4, routing_number })
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

    impl ObjectDeser for TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount {
        type Builder = TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccountBuilder;
    }
};
