#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InvoicePaymentMethodOptionsCustomerBalanceBankTransfer {
    pub eu_bank_transfer: Option<stripe_shared::InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>,
    /// The bank transfer type that can be used for funding.
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct InvoicePaymentMethodOptionsCustomerBalanceBankTransferBuilder {
    eu_bank_transfer: Option<Option<stripe_shared::InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>>,
    type_: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoicePaymentMethodOptionsCustomerBalanceBankTransfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicePaymentMethodOptionsCustomerBalanceBankTransfer>,
        builder: InvoicePaymentMethodOptionsCustomerBalanceBankTransferBuilder,
    }

    impl Visitor for Place<InvoicePaymentMethodOptionsCustomerBalanceBankTransfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: InvoicePaymentMethodOptionsCustomerBalanceBankTransferBuilder::deser_default() }))
        }
    }

    impl MapBuilder for InvoicePaymentMethodOptionsCustomerBalanceBankTransferBuilder {
        type Out = InvoicePaymentMethodOptionsCustomerBalanceBankTransfer;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "eu_bank_transfer" => Ok(Deserialize::begin(&mut self.eu_bank_transfer)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { eu_bank_transfer: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let eu_bank_transfer = self.eu_bank_transfer.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { eu_bank_transfer, type_ })
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

    impl ObjectDeser for InvoicePaymentMethodOptionsCustomerBalanceBankTransfer {
        type Builder = InvoicePaymentMethodOptionsCustomerBalanceBankTransferBuilder;
    }
};
