#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentLinksResourceCompletionBehaviorRedirect {
    /// The URL the customer will be redirected to after the purchase is complete.
    pub url: String,
}
#[cfg(feature = "min-ser")]
pub struct PaymentLinksResourceCompletionBehaviorRedirectBuilder {
    url: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentLinksResourceCompletionBehaviorRedirect {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceCompletionBehaviorRedirect>,
        builder: PaymentLinksResourceCompletionBehaviorRedirectBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceCompletionBehaviorRedirect> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentLinksResourceCompletionBehaviorRedirectBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentLinksResourceCompletionBehaviorRedirectBuilder {
        type Out = PaymentLinksResourceCompletionBehaviorRedirect;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "url" => Ok(Deserialize::begin(&mut self.url)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { url: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let url = self.url.take()?;

            Some(Self::Out { url })
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

    impl ObjectDeser for PaymentLinksResourceCompletionBehaviorRedirect {
        type Builder = PaymentLinksResourceCompletionBehaviorRedirectBuilder;
    }
};
