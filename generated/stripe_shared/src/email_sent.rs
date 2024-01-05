#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct EmailSent {
    /// The timestamp when the email was sent.
    pub email_sent_at: stripe_types::Timestamp,
    /// The recipient's email address.
    pub email_sent_to: String,
}
#[cfg(feature = "min-ser")]
pub struct EmailSentBuilder {
    email_sent_at: Option<stripe_types::Timestamp>,
    email_sent_to: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for EmailSent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<EmailSent>,
        builder: EmailSentBuilder,
    }

    impl Visitor for Place<EmailSent> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: EmailSentBuilder::deser_default() }))
        }
    }

    impl MapBuilder for EmailSentBuilder {
        type Out = EmailSent;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "email_sent_at" => Ok(Deserialize::begin(&mut self.email_sent_at)),
                "email_sent_to" => Ok(Deserialize::begin(&mut self.email_sent_to)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { email_sent_at: Deserialize::default(), email_sent_to: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let email_sent_at = self.email_sent_at.take()?;
            let email_sent_to = self.email_sent_to.take()?;

            Some(Self::Out { email_sent_at, email_sent_to })
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

    impl ObjectDeser for EmailSent {
        type Builder = EmailSentBuilder;
    }
};
