#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PlatformTaxFee {
    /// The Connected account that incurred this charge.
    pub account: String,
    /// Unique identifier for the object.
    pub id: stripe_shared::PlatformTaxFeeId,
    /// The payment object that caused this tax to be inflicted.
    pub source_transaction: String,
    /// The type of tax (VAT).
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: String,
}
#[cfg(feature = "min-ser")]
pub struct PlatformTaxFeeBuilder {
    account: Option<String>,
    id: Option<stripe_shared::PlatformTaxFeeId>,
    source_transaction: Option<String>,
    type_: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PlatformTaxFee {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PlatformTaxFee>,
        builder: PlatformTaxFeeBuilder,
    }

    impl Visitor for Place<PlatformTaxFee> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PlatformTaxFeeBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PlatformTaxFeeBuilder {
        type Out = PlatformTaxFee;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "account" => Ok(Deserialize::begin(&mut self.account)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "source_transaction" => Ok(Deserialize::begin(&mut self.source_transaction)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { account: Deserialize::default(), id: Deserialize::default(), source_transaction: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let account = self.account.take()?;
            let id = self.id.take()?;
            let source_transaction = self.source_transaction.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { account, id, source_transaction, type_ })
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

    impl ObjectDeser for PlatformTaxFee {
        type Builder = PlatformTaxFeeBuilder;
    }
};
impl stripe_types::Object for PlatformTaxFee {
    type Id = stripe_shared::PlatformTaxFeeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(PlatformTaxFeeId, "ptf_");
