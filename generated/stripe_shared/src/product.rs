/// Products describe the specific goods or services you offer to your customers.
/// For example, you might offer a Standard and Premium version of your goods or service; each version would be a separate Product.
/// They can be used in conjunction with [Prices](https://stripe.com/docs/api#prices) to configure pricing in Payment Links, Checkout, and Subscriptions.
///
/// Related guides: [Set up a subscription](https://stripe.com/docs/billing/subscriptions/set-up-subscription),.
/// [share a Payment Link](https://stripe.com/docs/payment-links),
/// [accept payments with Checkout](https://stripe.com/docs/payments/accept-a-payment#create-product-prices-upfront),.
/// and more about [Products and Prices](https://stripe.com/docs/products-prices/overview)
///
/// For more details see <<https://stripe.com/docs/api/products/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Product {
    /// Whether the product is currently available for purchase.
    pub active: bool,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The ID of the [Price](https://stripe.com/docs/api/prices) object that is the default price for this product.
    pub default_price: Option<stripe_types::Expandable<stripe_shared::Price>>,
    /// The product's description, meant to be displayable to the customer.
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    pub description: Option<String>,
    /// A list of up to 15 features for this product.
    /// These are displayed in [pricing tables](https://stripe.com/docs/payments/checkout/pricing-table).
    pub features: Vec<stripe_shared::ProductFeature>,
    /// Unique identifier for the object.
    pub id: stripe_shared::ProductId,
    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    pub images: Vec<String>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The product's name, meant to be displayable to the customer.
    pub name: String,
    /// The dimensions of this product for shipping purposes.
    pub package_dimensions: Option<stripe_shared::PackageDimensions>,
    /// Whether this product is shipped (i.e., physical goods).
    pub shippable: Option<bool>,
    /// Extra information about a product which will appear on your customer's credit card statement.
    /// In the case that multiple products are billed at once, the first statement descriptor will be used.
    pub statement_descriptor: Option<String>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    pub tax_code: Option<stripe_types::Expandable<stripe_shared::TaxCode>>,
    /// The type of the product.
    /// The product is either of type `good`, which is eligible for use with Orders and SKUs, or `service`, which is eligible for use with Subscriptions and Plans.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: stripe_shared::ProductType,
    /// A label that represents units of this product.
    /// When set, this will be included in customers' receipts, invoices, Checkout, and the customer portal.
    pub unit_label: Option<String>,
    /// Time at which the object was last updated. Measured in seconds since the Unix epoch.
    pub updated: stripe_types::Timestamp,
    /// A URL of a publicly-accessible webpage for this product.
    pub url: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct ProductBuilder {
    active: Option<bool>,
    created: Option<stripe_types::Timestamp>,
    default_price: Option<Option<stripe_types::Expandable<stripe_shared::Price>>>,
    description: Option<Option<String>>,
    features: Option<Vec<stripe_shared::ProductFeature>>,
    id: Option<stripe_shared::ProductId>,
    images: Option<Vec<String>>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    name: Option<String>,
    package_dimensions: Option<Option<stripe_shared::PackageDimensions>>,
    shippable: Option<Option<bool>>,
    statement_descriptor: Option<Option<String>>,
    tax_code: Option<Option<stripe_types::Expandable<stripe_shared::TaxCode>>>,
    type_: Option<stripe_shared::ProductType>,
    unit_label: Option<Option<String>>,
    updated: Option<stripe_types::Timestamp>,
    url: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Product {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Product>,
        builder: ProductBuilder,
    }

    impl Visitor for Place<Product> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ProductBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ProductBuilder {
        type Out = Product;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "active" => Ok(Deserialize::begin(&mut self.active)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "default_price" => Ok(Deserialize::begin(&mut self.default_price)),
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "features" => Ok(Deserialize::begin(&mut self.features)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "images" => Ok(Deserialize::begin(&mut self.images)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "name" => Ok(Deserialize::begin(&mut self.name)),
                "package_dimensions" => Ok(Deserialize::begin(&mut self.package_dimensions)),
                "shippable" => Ok(Deserialize::begin(&mut self.shippable)),
                "statement_descriptor" => Ok(Deserialize::begin(&mut self.statement_descriptor)),
                "tax_code" => Ok(Deserialize::begin(&mut self.tax_code)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),
                "unit_label" => Ok(Deserialize::begin(&mut self.unit_label)),
                "updated" => Ok(Deserialize::begin(&mut self.updated)),
                "url" => Ok(Deserialize::begin(&mut self.url)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                active: Deserialize::default(),
                created: Deserialize::default(),
                default_price: Deserialize::default(),
                description: Deserialize::default(),
                features: Deserialize::default(),
                id: Deserialize::default(),
                images: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                name: Deserialize::default(),
                package_dimensions: Deserialize::default(),
                shippable: Deserialize::default(),
                statement_descriptor: Deserialize::default(),
                tax_code: Deserialize::default(),
                type_: Deserialize::default(),
                unit_label: Deserialize::default(),
                updated: Deserialize::default(),
                url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let active = self.active.take()?;
            let created = self.created.take()?;
            let default_price = self.default_price.take()?;
            let description = self.description.take()?;
            let features = self.features.take()?;
            let id = self.id.take()?;
            let images = self.images.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let name = self.name.take()?;
            let package_dimensions = self.package_dimensions.take()?;
            let shippable = self.shippable.take()?;
            let statement_descriptor = self.statement_descriptor.take()?;
            let tax_code = self.tax_code.take()?;
            let type_ = self.type_.take()?;
            let unit_label = self.unit_label.take()?;
            let updated = self.updated.take()?;
            let url = self.url.take()?;

            Some(Self::Out {
                active,
                created,
                default_price,
                description,
                features,
                id,
                images,
                livemode,
                metadata,
                name,
                package_dimensions,
                shippable,
                statement_descriptor,
                tax_code,
                type_,
                unit_label,
                updated,
                url,
            })
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

    impl ObjectDeser for Product {
        type Builder = ProductBuilder;
    }
};
impl stripe_types::Object for Product {
    type Id = stripe_shared::ProductId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(ProductId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ProductType {
    Good,
    Service,
}
impl ProductType {
    pub fn as_str(self) -> &'static str {
        use ProductType::*;
        match self {
            Good => "good",
            Service => "service",
        }
    }
}

impl std::str::FromStr for ProductType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ProductType::*;
        match s {
            "good" => Ok(Good),
            "service" => Ok(Service),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for ProductType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for ProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ProductType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ProductType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ProductType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ProductType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<ProductType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ProductType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
