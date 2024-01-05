#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceTypeAchCreditTransfer {
    pub account_number: Option<String>,
    pub bank_name: Option<String>,
    pub fingerprint: Option<String>,
    pub refund_account_holder_name: Option<String>,
    pub refund_account_holder_type: Option<String>,
    pub refund_routing_number: Option<String>,
    pub routing_number: Option<String>,
    pub swift_code: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SourceTypeAchCreditTransferBuilder {
    account_number: Option<Option<String>>,
    bank_name: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    refund_account_holder_name: Option<Option<String>>,
    refund_account_holder_type: Option<Option<String>>,
    refund_routing_number: Option<Option<String>>,
    routing_number: Option<Option<String>>,
    swift_code: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTypeAchCreditTransfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeAchCreditTransfer>,
        builder: SourceTypeAchCreditTransferBuilder,
    }

    impl Visitor for Place<SourceTypeAchCreditTransfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceTypeAchCreditTransferBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceTypeAchCreditTransferBuilder {
        type Out = SourceTypeAchCreditTransfer;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "account_number" => Ok(Deserialize::begin(&mut self.account_number)),
                "bank_name" => Ok(Deserialize::begin(&mut self.bank_name)),
                "fingerprint" => Ok(Deserialize::begin(&mut self.fingerprint)),
                "refund_account_holder_name" => Ok(Deserialize::begin(&mut self.refund_account_holder_name)),
                "refund_account_holder_type" => Ok(Deserialize::begin(&mut self.refund_account_holder_type)),
                "refund_routing_number" => Ok(Deserialize::begin(&mut self.refund_routing_number)),
                "routing_number" => Ok(Deserialize::begin(&mut self.routing_number)),
                "swift_code" => Ok(Deserialize::begin(&mut self.swift_code)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                account_number: Deserialize::default(),
                bank_name: Deserialize::default(),
                fingerprint: Deserialize::default(),
                refund_account_holder_name: Deserialize::default(),
                refund_account_holder_type: Deserialize::default(),
                refund_routing_number: Deserialize::default(),
                routing_number: Deserialize::default(),
                swift_code: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let account_number = self.account_number.take()?;
            let bank_name = self.bank_name.take()?;
            let fingerprint = self.fingerprint.take()?;
            let refund_account_holder_name = self.refund_account_holder_name.take()?;
            let refund_account_holder_type = self.refund_account_holder_type.take()?;
            let refund_routing_number = self.refund_routing_number.take()?;
            let routing_number = self.routing_number.take()?;
            let swift_code = self.swift_code.take()?;

            Some(Self::Out { account_number, bank_name, fingerprint, refund_account_holder_name, refund_account_holder_type, refund_routing_number, routing_number, swift_code })
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

    impl ObjectDeser for SourceTypeAchCreditTransfer {
        type Builder = SourceTypeAchCreditTransferBuilder;
    }
};
