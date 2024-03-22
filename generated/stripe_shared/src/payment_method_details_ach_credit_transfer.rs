#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetailsAchCreditTransfer {
    /// Account number to transfer funds to.
    pub account_number: Option<String>,
    /// Name of the bank associated with the routing number.
    pub bank_name: Option<String>,
    /// Routing transit number for the bank account to transfer funds to.
    pub routing_number: Option<String>,
    /// SWIFT code of the bank associated with the routing number.
    pub swift_code: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDetailsAchCreditTransferBuilder {
    account_number: Option<Option<String>>,
    bank_name: Option<Option<String>>,
    routing_number: Option<Option<String>>,
    swift_code: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsAchCreditTransfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsAchCreditTransfer>,
        builder: PaymentMethodDetailsAchCreditTransferBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsAchCreditTransfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDetailsAchCreditTransferBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsAchCreditTransferBuilder {
        type Out = PaymentMethodDetailsAchCreditTransfer;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "account_number" => Ok(Deserialize::begin(&mut self.account_number)),
                "bank_name" => Ok(Deserialize::begin(&mut self.bank_name)),
                "routing_number" => Ok(Deserialize::begin(&mut self.routing_number)),
                "swift_code" => Ok(Deserialize::begin(&mut self.swift_code)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { account_number: Deserialize::default(), bank_name: Deserialize::default(), routing_number: Deserialize::default(), swift_code: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let account_number = self.account_number.take()?;
            let bank_name = self.bank_name.take()?;
            let routing_number = self.routing_number.take()?;
            let swift_code = self.swift_code.take()?;

            Some(Self::Out { account_number, bank_name, routing_number, swift_code })
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

    impl ObjectDeser for PaymentMethodDetailsAchCreditTransfer {
        type Builder = PaymentMethodDetailsAchCreditTransferBuilder;
    }
};
