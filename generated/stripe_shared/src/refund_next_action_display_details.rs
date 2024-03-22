#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct RefundNextActionDisplayDetails {
    pub email_sent: stripe_shared::EmailSent,
    /// The expiry timestamp.
    pub expires_at: stripe_types::Timestamp,
}
#[cfg(feature = "min-ser")]
pub struct RefundNextActionDisplayDetailsBuilder {
    email_sent: Option<stripe_shared::EmailSent>,
    expires_at: Option<stripe_types::Timestamp>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for RefundNextActionDisplayDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RefundNextActionDisplayDetails>,
        builder: RefundNextActionDisplayDetailsBuilder,
    }

    impl Visitor for Place<RefundNextActionDisplayDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: RefundNextActionDisplayDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for RefundNextActionDisplayDetailsBuilder {
        type Out = RefundNextActionDisplayDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "email_sent" => Ok(Deserialize::begin(&mut self.email_sent)),
                "expires_at" => Ok(Deserialize::begin(&mut self.expires_at)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { email_sent: Deserialize::default(), expires_at: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let email_sent = self.email_sent.take()?;
            let expires_at = self.expires_at.take()?;

            Some(Self::Out { email_sent, expires_at })
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

    impl ObjectDeser for RefundNextActionDisplayDetails {
        type Builder = RefundNextActionDisplayDetailsBuilder;
    }
};
