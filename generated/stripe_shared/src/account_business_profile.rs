#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountBusinessProfile {
    /// [The merchant category code for the account](https://stripe.com/docs/connect/setting-mcc).
    /// MCCs are used to classify businesses based on the goods or services they provide.
    pub mcc: Option<String>,
    pub monthly_estimated_revenue: Option<stripe_shared::AccountMonthlyEstimatedRevenue>,
    /// The customer-facing business name.
    pub name: Option<String>,
    /// Internal-only description of the product sold or service provided by the business.
    /// It's used by Stripe for risk and underwriting purposes.
    pub product_description: Option<String>,
    /// A publicly available mailing address for sending support issues to.
    pub support_address: Option<stripe_shared::Address>,
    /// A publicly available email address for sending support issues to.
    pub support_email: Option<String>,
    /// A publicly available phone number to call with support issues.
    pub support_phone: Option<String>,
    /// A publicly available website for handling support issues.
    pub support_url: Option<String>,
    /// The business's publicly available website.
    pub url: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct AccountBusinessProfileBuilder {
    mcc: Option<Option<String>>,
    monthly_estimated_revenue: Option<Option<stripe_shared::AccountMonthlyEstimatedRevenue>>,
    name: Option<Option<String>>,
    product_description: Option<Option<String>>,
    support_address: Option<Option<stripe_shared::Address>>,
    support_email: Option<Option<String>>,
    support_phone: Option<Option<String>>,
    support_url: Option<Option<String>>,
    url: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountBusinessProfile {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountBusinessProfile>,
        builder: AccountBusinessProfileBuilder,
    }

    impl Visitor for Place<AccountBusinessProfile> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: AccountBusinessProfileBuilder::deser_default() }))
        }
    }

    impl MapBuilder for AccountBusinessProfileBuilder {
        type Out = AccountBusinessProfile;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "mcc" => Ok(Deserialize::begin(&mut self.mcc)),
                "monthly_estimated_revenue" => Ok(Deserialize::begin(&mut self.monthly_estimated_revenue)),
                "name" => Ok(Deserialize::begin(&mut self.name)),
                "product_description" => Ok(Deserialize::begin(&mut self.product_description)),
                "support_address" => Ok(Deserialize::begin(&mut self.support_address)),
                "support_email" => Ok(Deserialize::begin(&mut self.support_email)),
                "support_phone" => Ok(Deserialize::begin(&mut self.support_phone)),
                "support_url" => Ok(Deserialize::begin(&mut self.support_url)),
                "url" => Ok(Deserialize::begin(&mut self.url)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                mcc: Deserialize::default(),
                monthly_estimated_revenue: Deserialize::default(),
                name: Deserialize::default(),
                product_description: Deserialize::default(),
                support_address: Deserialize::default(),
                support_email: Deserialize::default(),
                support_phone: Deserialize::default(),
                support_url: Deserialize::default(),
                url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let mcc = self.mcc.take()?;
            let monthly_estimated_revenue = self.monthly_estimated_revenue.take()?;
            let name = self.name.take()?;
            let product_description = self.product_description.take()?;
            let support_address = self.support_address.take()?;
            let support_email = self.support_email.take()?;
            let support_phone = self.support_phone.take()?;
            let support_url = self.support_url.take()?;
            let url = self.url.take()?;

            Some(Self::Out { mcc, monthly_estimated_revenue, name, product_description, support_address, support_email, support_phone, support_url, url })
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

    impl ObjectDeser for AccountBusinessProfile {
        type Builder = AccountBusinessProfileBuilder;
    }
};
