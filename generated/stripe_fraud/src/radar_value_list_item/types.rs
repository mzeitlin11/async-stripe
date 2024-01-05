/// Value list items allow you to add specific values to a given Radar value list, which can then be used in rules.
///
/// Related guide: [Managing list items](https://stripe.com/docs/radar/lists#managing-list-items)
///
/// For more details see <<https://stripe.com/docs/api/radar/value_list_items/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct RadarValueListItem {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The name or email address of the user who added this item to the value list.
    pub created_by: String,
    /// Unique identifier for the object.
    pub id: stripe_fraud::RadarValueListItemId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The value of the item.
    pub value: String,
    /// The identifier of the value list this item belongs to.
    pub value_list: String,
}
#[cfg(feature = "min-ser")]
pub struct RadarValueListItemBuilder {
    created: Option<stripe_types::Timestamp>,
    created_by: Option<String>,
    id: Option<stripe_fraud::RadarValueListItemId>,
    livemode: Option<bool>,
    value: Option<String>,
    value_list: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for RadarValueListItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RadarValueListItem>,
        builder: RadarValueListItemBuilder,
    }

    impl Visitor for Place<RadarValueListItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: RadarValueListItemBuilder::deser_default() }))
        }
    }

    impl MapBuilder for RadarValueListItemBuilder {
        type Out = RadarValueListItem;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "created_by" => Ok(Deserialize::begin(&mut self.created_by)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "value" => Ok(Deserialize::begin(&mut self.value)),
                "value_list" => Ok(Deserialize::begin(&mut self.value_list)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                created: Deserialize::default(),
                created_by: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                value: Deserialize::default(),
                value_list: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let created = self.created.take()?;
            let created_by = self.created_by.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let value = self.value.take()?;
            let value_list = self.value_list.take()?;

            Some(Self::Out { created, created_by, id, livemode, value, value_list })
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

    impl ObjectDeser for RadarValueListItem {
        type Builder = RadarValueListItemBuilder;
    }
};
impl stripe_types::Object for RadarValueListItem {
    type Id = stripe_fraud::RadarValueListItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(RadarValueListItemId);
