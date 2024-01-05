#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodUsBankAccountStatusDetails {
    pub blocked: Option<stripe_shared::PaymentMethodUsBankAccountBlocked>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodUsBankAccountStatusDetailsBuilder {
    blocked: Option<Option<stripe_shared::PaymentMethodUsBankAccountBlocked>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodUsBankAccountStatusDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodUsBankAccountStatusDetails>,
        builder: PaymentMethodUsBankAccountStatusDetailsBuilder,
    }

    impl Visitor for Place<PaymentMethodUsBankAccountStatusDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodUsBankAccountStatusDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodUsBankAccountStatusDetailsBuilder {
        type Out = PaymentMethodUsBankAccountStatusDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "blocked" => Ok(Deserialize::begin(&mut self.blocked)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { blocked: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let blocked = self.blocked.take()?;

            Some(Self::Out { blocked })
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

    impl ObjectDeser for PaymentMethodUsBankAccountStatusDetails {
        type Builder = PaymentMethodUsBankAccountStatusDetailsBuilder;
    }
};
