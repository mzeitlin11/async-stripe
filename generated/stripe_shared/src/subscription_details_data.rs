#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SubscriptionDetailsData {
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will reflect the metadata of the subscription at the time of invoice creation.
    /// *Note: This attribute is populated only for invoices created on or after June 29, 2023.*.
    pub metadata: Option<std::collections::HashMap<String, String>>,
}
#[cfg(feature = "min-ser")]
pub struct SubscriptionDetailsDataBuilder {
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SubscriptionDetailsData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionDetailsData>,
        builder: SubscriptionDetailsDataBuilder,
    }

    impl Visitor for Place<SubscriptionDetailsData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SubscriptionDetailsDataBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SubscriptionDetailsDataBuilder {
        type Out = SubscriptionDetailsData;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { metadata: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let metadata = self.metadata.take()?;

            Some(Self::Out { metadata })
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

    impl ObjectDeser for SubscriptionDetailsData {
        type Builder = SubscriptionDetailsDataBuilder;
    }
};
