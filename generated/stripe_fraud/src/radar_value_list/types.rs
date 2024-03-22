/// Value lists allow you to group values together which can then be referenced in rules.
///
/// Related guide: [Default Stripe lists](https://stripe.com/docs/radar/lists#managing-list-items)
///
/// For more details see <<https://stripe.com/docs/api/radar/value_lists/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct RadarValueList {
    /// The name of the value list for use in rules.
    pub alias: String,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The name or email address of the user who created this value list.
    pub created_by: String,
    /// Unique identifier for the object.
    pub id: stripe_fraud::RadarValueListId,
    /// The type of items in the value list.
    /// One of `card_fingerprint`, `us_bank_account_fingerprint`, `sepa_debit_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`.
    pub item_type: stripe_fraud::RadarValueListItemType,
    /// List of items contained within this value list.
    pub list_items: stripe_types::List<stripe_fraud::RadarValueListItem>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The name of the value list.
    pub name: String,
}
#[cfg(feature = "min-ser")]
pub struct RadarValueListBuilder {
    alias: Option<String>,
    created: Option<stripe_types::Timestamp>,
    created_by: Option<String>,
    id: Option<stripe_fraud::RadarValueListId>,
    item_type: Option<stripe_fraud::RadarValueListItemType>,
    list_items: Option<stripe_types::List<stripe_fraud::RadarValueListItem>>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    name: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for RadarValueList {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RadarValueList>,
        builder: RadarValueListBuilder,
    }

    impl Visitor for Place<RadarValueList> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: RadarValueListBuilder::deser_default() }))
        }
    }

    impl MapBuilder for RadarValueListBuilder {
        type Out = RadarValueList;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "alias" => Ok(Deserialize::begin(&mut self.alias)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "created_by" => Ok(Deserialize::begin(&mut self.created_by)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "item_type" => Ok(Deserialize::begin(&mut self.item_type)),
                "list_items" => Ok(Deserialize::begin(&mut self.list_items)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "name" => Ok(Deserialize::begin(&mut self.name)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                alias: Deserialize::default(),
                created: Deserialize::default(),
                created_by: Deserialize::default(),
                id: Deserialize::default(),
                item_type: Deserialize::default(),
                list_items: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                name: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let alias = self.alias.take()?;
            let created = self.created.take()?;
            let created_by = self.created_by.take()?;
            let id = self.id.take()?;
            let item_type = self.item_type.take()?;
            let list_items = self.list_items.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let name = self.name.take()?;

            Some(Self::Out { alias, created, created_by, id, item_type, list_items, livemode, metadata, name })
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

    impl ObjectDeser for RadarValueList {
        type Builder = RadarValueListBuilder;
    }
};
impl stripe_types::Object for RadarValueList {
    type Id = stripe_fraud::RadarValueListId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(RadarValueListId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RadarValueListItemType {
    CardBin,
    CardFingerprint,
    CaseSensitiveString,
    Country,
    CustomerId,
    Email,
    IpAddress,
    SepaDebitFingerprint,
    String,
    UsBankAccountFingerprint,
}
impl RadarValueListItemType {
    pub fn as_str(self) -> &'static str {
        use RadarValueListItemType::*;
        match self {
            CardBin => "card_bin",
            CardFingerprint => "card_fingerprint",
            CaseSensitiveString => "case_sensitive_string",
            Country => "country",
            CustomerId => "customer_id",
            Email => "email",
            IpAddress => "ip_address",
            SepaDebitFingerprint => "sepa_debit_fingerprint",
            String => "string",
            UsBankAccountFingerprint => "us_bank_account_fingerprint",
        }
    }
}

impl std::str::FromStr for RadarValueListItemType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RadarValueListItemType::*;
        match s {
            "card_bin" => Ok(CardBin),
            "card_fingerprint" => Ok(CardFingerprint),
            "case_sensitive_string" => Ok(CaseSensitiveString),
            "country" => Ok(Country),
            "customer_id" => Ok(CustomerId),
            "email" => Ok(Email),
            "ip_address" => Ok(IpAddress),
            "sepa_debit_fingerprint" => Ok(SepaDebitFingerprint),
            "string" => Ok(String),
            "us_bank_account_fingerprint" => Ok(UsBankAccountFingerprint),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for RadarValueListItemType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for RadarValueListItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for RadarValueListItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for RadarValueListItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RadarValueListItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for RadarValueListItemType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for RadarValueListItemType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<RadarValueListItemType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(RadarValueListItemType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
