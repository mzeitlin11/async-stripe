#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingCardWallets {
    pub apple_pay: stripe_shared::IssuingCardApplePay,
    pub google_pay: stripe_shared::IssuingCardGooglePay,
    /// Unique identifier for a card used with digital wallets
    pub primary_account_identifier: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingCardWalletsBuilder {
    apple_pay: Option<stripe_shared::IssuingCardApplePay>,
    google_pay: Option<stripe_shared::IssuingCardGooglePay>,
    primary_account_identifier: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingCardWallets {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardWallets>,
        builder: IssuingCardWalletsBuilder,
    }

    impl Visitor for Place<IssuingCardWallets> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingCardWalletsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingCardWalletsBuilder {
        type Out = IssuingCardWallets;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "apple_pay" => Ok(Deserialize::begin(&mut self.apple_pay)),
                "google_pay" => Ok(Deserialize::begin(&mut self.google_pay)),
                "primary_account_identifier" => Ok(Deserialize::begin(&mut self.primary_account_identifier)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { apple_pay: Deserialize::default(), google_pay: Deserialize::default(), primary_account_identifier: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let apple_pay = self.apple_pay.take()?;
            let google_pay = self.google_pay.take()?;
            let primary_account_identifier = self.primary_account_identifier.take()?;

            Some(Self::Out { apple_pay, google_pay, primary_account_identifier })
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

    impl ObjectDeser for IssuingCardWallets {
        type Builder = IssuingCardWalletsBuilder;
    }
};
