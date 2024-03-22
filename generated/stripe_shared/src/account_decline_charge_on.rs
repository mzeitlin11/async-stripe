#[derive(Copy, Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountDeclineChargeOn {
    /// Whether Stripe automatically declines charges with an incorrect ZIP or postal code.
    /// This setting only applies when a ZIP or postal code is provided and they fail bank verification.
    pub avs_failure: bool,
    /// Whether Stripe automatically declines charges with an incorrect CVC.
    /// This setting only applies when a CVC is provided and it fails bank verification.
    pub cvc_failure: bool,
}
#[cfg(feature = "min-ser")]
pub struct AccountDeclineChargeOnBuilder {
    avs_failure: Option<bool>,
    cvc_failure: Option<bool>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountDeclineChargeOn {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountDeclineChargeOn>,
        builder: AccountDeclineChargeOnBuilder,
    }

    impl Visitor for Place<AccountDeclineChargeOn> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: AccountDeclineChargeOnBuilder::deser_default() }))
        }
    }

    impl MapBuilder for AccountDeclineChargeOnBuilder {
        type Out = AccountDeclineChargeOn;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "avs_failure" => Ok(Deserialize::begin(&mut self.avs_failure)),
                "cvc_failure" => Ok(Deserialize::begin(&mut self.cvc_failure)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { avs_failure: Deserialize::default(), cvc_failure: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let avs_failure = self.avs_failure.take()?;
            let cvc_failure = self.cvc_failure.take()?;

            Some(Self::Out { avs_failure, cvc_failure })
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

    impl ObjectDeser for AccountDeclineChargeOn {
        type Builder = AccountDeclineChargeOnBuilder;
    }
};
