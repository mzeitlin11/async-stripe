#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodCardWalletMasterpass {
    /// Owner's verified billing address.
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub billing_address: Option<stripe_shared::Address>,
    /// Owner's verified email.
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub email: Option<String>,
    /// Owner's verified full name.
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub name: Option<String>,
    /// Owner's verified shipping address.
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub shipping_address: Option<stripe_shared::Address>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodCardWalletMasterpassBuilder {
    billing_address: Option<Option<stripe_shared::Address>>,
    email: Option<Option<String>>,
    name: Option<Option<String>>,
    shipping_address: Option<Option<stripe_shared::Address>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodCardWalletMasterpass {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodCardWalletMasterpass>,
        builder: PaymentMethodCardWalletMasterpassBuilder,
    }

    impl Visitor for Place<PaymentMethodCardWalletMasterpass> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodCardWalletMasterpassBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodCardWalletMasterpassBuilder {
        type Out = PaymentMethodCardWalletMasterpass;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "billing_address" => Ok(Deserialize::begin(&mut self.billing_address)),
                "email" => Ok(Deserialize::begin(&mut self.email)),
                "name" => Ok(Deserialize::begin(&mut self.name)),
                "shipping_address" => Ok(Deserialize::begin(&mut self.shipping_address)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { billing_address: Deserialize::default(), email: Deserialize::default(), name: Deserialize::default(), shipping_address: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let billing_address = self.billing_address.take()?;
            let email = self.email.take()?;
            let name = self.name.take()?;
            let shipping_address = self.shipping_address.take()?;

            Some(Self::Out { billing_address, email, name, shipping_address })
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

    impl ObjectDeser for PaymentMethodCardWalletMasterpass {
        type Builder = PaymentMethodCardWalletMasterpassBuilder;
    }
};
