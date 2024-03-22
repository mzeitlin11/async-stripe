#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct NotificationEventData {
    /// Object containing the API resource relevant to the event.
    /// For example, an `invoice.created` event will have a full [invoice object](https://stripe.com/docs/api#invoice_object) as the value of the object key.
    pub object: stripe_types::Value,
    /// Object containing the names of the updated attributes and their values prior to the event (only included in events of type `*.updated`).
    /// If an array attribute has any updated elements, this object contains the entire array.
    /// In Stripe API versions 2017-04-06 or earlier, an updated array attribute in this object includes only the updated array elements.
    pub previous_attributes: Option<stripe_types::Value>,
}
#[cfg(feature = "min-ser")]
pub struct NotificationEventDataBuilder {
    object: Option<stripe_types::Value>,
    previous_attributes: Option<Option<stripe_types::Value>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for NotificationEventData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<NotificationEventData>,
        builder: NotificationEventDataBuilder,
    }

    impl Visitor for Place<NotificationEventData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: NotificationEventDataBuilder::deser_default() }))
        }
    }

    impl MapBuilder for NotificationEventDataBuilder {
        type Out = NotificationEventData;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "object" => Ok(Deserialize::begin(&mut self.object)),
                "previous_attributes" => Ok(Deserialize::begin(&mut self.previous_attributes)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { object: Deserialize::default(), previous_attributes: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let object = self.object.take()?;
            let previous_attributes = self.previous_attributes.take()?;

            Some(Self::Out { object, previous_attributes })
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

    impl ObjectDeser for NotificationEventData {
        type Builder = NotificationEventDataBuilder;
    }
};
