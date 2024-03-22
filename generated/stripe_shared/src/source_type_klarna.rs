#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceTypeKlarna {
    pub background_image_url: Option<String>,
    pub client_token: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub locale: Option<String>,
    pub logo_url: Option<String>,
    pub page_title: Option<String>,
    pub pay_later_asset_urls_descriptive: Option<String>,
    pub pay_later_asset_urls_standard: Option<String>,
    pub pay_later_name: Option<String>,
    pub pay_later_redirect_url: Option<String>,
    pub pay_now_asset_urls_descriptive: Option<String>,
    pub pay_now_asset_urls_standard: Option<String>,
    pub pay_now_name: Option<String>,
    pub pay_now_redirect_url: Option<String>,
    pub pay_over_time_asset_urls_descriptive: Option<String>,
    pub pay_over_time_asset_urls_standard: Option<String>,
    pub pay_over_time_name: Option<String>,
    pub pay_over_time_redirect_url: Option<String>,
    pub payment_method_categories: Option<String>,
    pub purchase_country: Option<String>,
    pub purchase_type: Option<String>,
    pub redirect_url: Option<String>,
    pub shipping_delay: Option<i64>,
    pub shipping_first_name: Option<String>,
    pub shipping_last_name: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SourceTypeKlarnaBuilder {
    background_image_url: Option<Option<String>>,
    client_token: Option<Option<String>>,
    first_name: Option<Option<String>>,
    last_name: Option<Option<String>>,
    locale: Option<Option<String>>,
    logo_url: Option<Option<String>>,
    page_title: Option<Option<String>>,
    pay_later_asset_urls_descriptive: Option<Option<String>>,
    pay_later_asset_urls_standard: Option<Option<String>>,
    pay_later_name: Option<Option<String>>,
    pay_later_redirect_url: Option<Option<String>>,
    pay_now_asset_urls_descriptive: Option<Option<String>>,
    pay_now_asset_urls_standard: Option<Option<String>>,
    pay_now_name: Option<Option<String>>,
    pay_now_redirect_url: Option<Option<String>>,
    pay_over_time_asset_urls_descriptive: Option<Option<String>>,
    pay_over_time_asset_urls_standard: Option<Option<String>>,
    pay_over_time_name: Option<Option<String>>,
    pay_over_time_redirect_url: Option<Option<String>>,
    payment_method_categories: Option<Option<String>>,
    purchase_country: Option<Option<String>>,
    purchase_type: Option<Option<String>>,
    redirect_url: Option<Option<String>>,
    shipping_delay: Option<Option<i64>>,
    shipping_first_name: Option<Option<String>>,
    shipping_last_name: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTypeKlarna {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeKlarna>,
        builder: SourceTypeKlarnaBuilder,
    }

    impl Visitor for Place<SourceTypeKlarna> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceTypeKlarnaBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceTypeKlarnaBuilder {
        type Out = SourceTypeKlarna;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "background_image_url" => Ok(Deserialize::begin(&mut self.background_image_url)),
                "client_token" => Ok(Deserialize::begin(&mut self.client_token)),
                "first_name" => Ok(Deserialize::begin(&mut self.first_name)),
                "last_name" => Ok(Deserialize::begin(&mut self.last_name)),
                "locale" => Ok(Deserialize::begin(&mut self.locale)),
                "logo_url" => Ok(Deserialize::begin(&mut self.logo_url)),
                "page_title" => Ok(Deserialize::begin(&mut self.page_title)),
                "pay_later_asset_urls_descriptive" => Ok(Deserialize::begin(&mut self.pay_later_asset_urls_descriptive)),
                "pay_later_asset_urls_standard" => Ok(Deserialize::begin(&mut self.pay_later_asset_urls_standard)),
                "pay_later_name" => Ok(Deserialize::begin(&mut self.pay_later_name)),
                "pay_later_redirect_url" => Ok(Deserialize::begin(&mut self.pay_later_redirect_url)),
                "pay_now_asset_urls_descriptive" => Ok(Deserialize::begin(&mut self.pay_now_asset_urls_descriptive)),
                "pay_now_asset_urls_standard" => Ok(Deserialize::begin(&mut self.pay_now_asset_urls_standard)),
                "pay_now_name" => Ok(Deserialize::begin(&mut self.pay_now_name)),
                "pay_now_redirect_url" => Ok(Deserialize::begin(&mut self.pay_now_redirect_url)),
                "pay_over_time_asset_urls_descriptive" => Ok(Deserialize::begin(&mut self.pay_over_time_asset_urls_descriptive)),
                "pay_over_time_asset_urls_standard" => Ok(Deserialize::begin(&mut self.pay_over_time_asset_urls_standard)),
                "pay_over_time_name" => Ok(Deserialize::begin(&mut self.pay_over_time_name)),
                "pay_over_time_redirect_url" => Ok(Deserialize::begin(&mut self.pay_over_time_redirect_url)),
                "payment_method_categories" => Ok(Deserialize::begin(&mut self.payment_method_categories)),
                "purchase_country" => Ok(Deserialize::begin(&mut self.purchase_country)),
                "purchase_type" => Ok(Deserialize::begin(&mut self.purchase_type)),
                "redirect_url" => Ok(Deserialize::begin(&mut self.redirect_url)),
                "shipping_delay" => Ok(Deserialize::begin(&mut self.shipping_delay)),
                "shipping_first_name" => Ok(Deserialize::begin(&mut self.shipping_first_name)),
                "shipping_last_name" => Ok(Deserialize::begin(&mut self.shipping_last_name)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                background_image_url: Deserialize::default(),
                client_token: Deserialize::default(),
                first_name: Deserialize::default(),
                last_name: Deserialize::default(),
                locale: Deserialize::default(),
                logo_url: Deserialize::default(),
                page_title: Deserialize::default(),
                pay_later_asset_urls_descriptive: Deserialize::default(),
                pay_later_asset_urls_standard: Deserialize::default(),
                pay_later_name: Deserialize::default(),
                pay_later_redirect_url: Deserialize::default(),
                pay_now_asset_urls_descriptive: Deserialize::default(),
                pay_now_asset_urls_standard: Deserialize::default(),
                pay_now_name: Deserialize::default(),
                pay_now_redirect_url: Deserialize::default(),
                pay_over_time_asset_urls_descriptive: Deserialize::default(),
                pay_over_time_asset_urls_standard: Deserialize::default(),
                pay_over_time_name: Deserialize::default(),
                pay_over_time_redirect_url: Deserialize::default(),
                payment_method_categories: Deserialize::default(),
                purchase_country: Deserialize::default(),
                purchase_type: Deserialize::default(),
                redirect_url: Deserialize::default(),
                shipping_delay: Deserialize::default(),
                shipping_first_name: Deserialize::default(),
                shipping_last_name: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let background_image_url = self.background_image_url.take()?;
            let client_token = self.client_token.take()?;
            let first_name = self.first_name.take()?;
            let last_name = self.last_name.take()?;
            let locale = self.locale.take()?;
            let logo_url = self.logo_url.take()?;
            let page_title = self.page_title.take()?;
            let pay_later_asset_urls_descriptive = self.pay_later_asset_urls_descriptive.take()?;
            let pay_later_asset_urls_standard = self.pay_later_asset_urls_standard.take()?;
            let pay_later_name = self.pay_later_name.take()?;
            let pay_later_redirect_url = self.pay_later_redirect_url.take()?;
            let pay_now_asset_urls_descriptive = self.pay_now_asset_urls_descriptive.take()?;
            let pay_now_asset_urls_standard = self.pay_now_asset_urls_standard.take()?;
            let pay_now_name = self.pay_now_name.take()?;
            let pay_now_redirect_url = self.pay_now_redirect_url.take()?;
            let pay_over_time_asset_urls_descriptive = self.pay_over_time_asset_urls_descriptive.take()?;
            let pay_over_time_asset_urls_standard = self.pay_over_time_asset_urls_standard.take()?;
            let pay_over_time_name = self.pay_over_time_name.take()?;
            let pay_over_time_redirect_url = self.pay_over_time_redirect_url.take()?;
            let payment_method_categories = self.payment_method_categories.take()?;
            let purchase_country = self.purchase_country.take()?;
            let purchase_type = self.purchase_type.take()?;
            let redirect_url = self.redirect_url.take()?;
            let shipping_delay = self.shipping_delay.take()?;
            let shipping_first_name = self.shipping_first_name.take()?;
            let shipping_last_name = self.shipping_last_name.take()?;

            Some(Self::Out {
                background_image_url,
                client_token,
                first_name,
                last_name,
                locale,
                logo_url,
                page_title,
                pay_later_asset_urls_descriptive,
                pay_later_asset_urls_standard,
                pay_later_name,
                pay_later_redirect_url,
                pay_now_asset_urls_descriptive,
                pay_now_asset_urls_standard,
                pay_now_name,
                pay_now_redirect_url,
                pay_over_time_asset_urls_descriptive,
                pay_over_time_asset_urls_standard,
                pay_over_time_name,
                pay_over_time_redirect_url,
                payment_method_categories,
                purchase_country,
                purchase_type,
                redirect_url,
                shipping_delay,
                shipping_first_name,
                shipping_last_name,
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

    impl ObjectDeser for SourceTypeKlarna {
        type Builder = SourceTypeKlarnaBuilder;
    }
};
