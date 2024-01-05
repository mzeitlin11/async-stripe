#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceRedirectFlow {
    /// The failure reason for the redirect, either `user_abort` (the customer aborted or dropped out of the redirect flow), `declined` (the authentication failed or the transaction was declined), or `processing_error` (the redirect failed due to a technical error).
    /// Present only if the redirect status is `failed`.
    pub failure_reason: Option<String>,
    /// The URL you provide to redirect the customer to after they authenticated their payment.
    pub return_url: String,
    /// The status of the redirect, either `pending` (ready to be used by your customer to authenticate the transaction), `succeeded` (succesful authentication, cannot be reused) or `not_required` (redirect should not be used) or `failed` (failed authentication, cannot be reused).
    pub status: String,
    /// The URL provided to you to redirect a customer to as part of a `redirect` authentication flow.
    pub url: String,
}
#[cfg(feature = "min-ser")]
pub struct SourceRedirectFlowBuilder {
    failure_reason: Option<Option<String>>,
    return_url: Option<String>,
    status: Option<String>,
    url: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceRedirectFlow {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceRedirectFlow>,
        builder: SourceRedirectFlowBuilder,
    }

    impl Visitor for Place<SourceRedirectFlow> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceRedirectFlowBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceRedirectFlowBuilder {
        type Out = SourceRedirectFlow;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "failure_reason" => Ok(Deserialize::begin(&mut self.failure_reason)),
                "return_url" => Ok(Deserialize::begin(&mut self.return_url)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "url" => Ok(Deserialize::begin(&mut self.url)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { failure_reason: Deserialize::default(), return_url: Deserialize::default(), status: Deserialize::default(), url: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let failure_reason = self.failure_reason.take()?;
            let return_url = self.return_url.take()?;
            let status = self.status.take()?;
            let url = self.url.take()?;

            Some(Self::Out { failure_reason, return_url, status, url })
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

    impl ObjectDeser for SourceRedirectFlow {
        type Builder = SourceRedirectFlowBuilder;
    }
};
