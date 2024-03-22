#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceTypeCard {
    pub address_line1_check: Option<String>,
    pub address_zip_check: Option<String>,
    pub brand: Option<String>,
    pub country: Option<String>,
    pub cvc_check: Option<String>,
    pub description: Option<String>,
    pub dynamic_last4: Option<String>,
    pub exp_month: Option<i64>,
    pub exp_year: Option<i64>,
    pub fingerprint: Option<String>,
    pub funding: Option<String>,
    pub iin: Option<String>,
    pub issuer: Option<String>,
    pub last4: Option<String>,
    pub name: Option<String>,
    pub three_d_secure: Option<String>,
    pub tokenization_method: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SourceTypeCardBuilder {
    address_line1_check: Option<Option<String>>,
    address_zip_check: Option<Option<String>>,
    brand: Option<Option<String>>,
    country: Option<Option<String>>,
    cvc_check: Option<Option<String>>,
    description: Option<Option<String>>,
    dynamic_last4: Option<Option<String>>,
    exp_month: Option<Option<i64>>,
    exp_year: Option<Option<i64>>,
    fingerprint: Option<Option<String>>,
    funding: Option<Option<String>>,
    iin: Option<Option<String>>,
    issuer: Option<Option<String>>,
    last4: Option<Option<String>>,
    name: Option<Option<String>>,
    three_d_secure: Option<Option<String>>,
    tokenization_method: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTypeCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeCard>,
        builder: SourceTypeCardBuilder,
    }

    impl Visitor for Place<SourceTypeCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceTypeCardBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceTypeCardBuilder {
        type Out = SourceTypeCard;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "address_line1_check" => Ok(Deserialize::begin(&mut self.address_line1_check)),
                "address_zip_check" => Ok(Deserialize::begin(&mut self.address_zip_check)),
                "brand" => Ok(Deserialize::begin(&mut self.brand)),
                "country" => Ok(Deserialize::begin(&mut self.country)),
                "cvc_check" => Ok(Deserialize::begin(&mut self.cvc_check)),
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "dynamic_last4" => Ok(Deserialize::begin(&mut self.dynamic_last4)),
                "exp_month" => Ok(Deserialize::begin(&mut self.exp_month)),
                "exp_year" => Ok(Deserialize::begin(&mut self.exp_year)),
                "fingerprint" => Ok(Deserialize::begin(&mut self.fingerprint)),
                "funding" => Ok(Deserialize::begin(&mut self.funding)),
                "iin" => Ok(Deserialize::begin(&mut self.iin)),
                "issuer" => Ok(Deserialize::begin(&mut self.issuer)),
                "last4" => Ok(Deserialize::begin(&mut self.last4)),
                "name" => Ok(Deserialize::begin(&mut self.name)),
                "three_d_secure" => Ok(Deserialize::begin(&mut self.three_d_secure)),
                "tokenization_method" => Ok(Deserialize::begin(&mut self.tokenization_method)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                address_line1_check: Deserialize::default(),
                address_zip_check: Deserialize::default(),
                brand: Deserialize::default(),
                country: Deserialize::default(),
                cvc_check: Deserialize::default(),
                description: Deserialize::default(),
                dynamic_last4: Deserialize::default(),
                exp_month: Deserialize::default(),
                exp_year: Deserialize::default(),
                fingerprint: Deserialize::default(),
                funding: Deserialize::default(),
                iin: Deserialize::default(),
                issuer: Deserialize::default(),
                last4: Deserialize::default(),
                name: Deserialize::default(),
                three_d_secure: Deserialize::default(),
                tokenization_method: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let address_line1_check = self.address_line1_check.take()?;
            let address_zip_check = self.address_zip_check.take()?;
            let brand = self.brand.take()?;
            let country = self.country.take()?;
            let cvc_check = self.cvc_check.take()?;
            let description = self.description.take()?;
            let dynamic_last4 = self.dynamic_last4.take()?;
            let exp_month = self.exp_month.take()?;
            let exp_year = self.exp_year.take()?;
            let fingerprint = self.fingerprint.take()?;
            let funding = self.funding.take()?;
            let iin = self.iin.take()?;
            let issuer = self.issuer.take()?;
            let last4 = self.last4.take()?;
            let name = self.name.take()?;
            let three_d_secure = self.three_d_secure.take()?;
            let tokenization_method = self.tokenization_method.take()?;

            Some(Self::Out {
                address_line1_check,
                address_zip_check,
                brand,
                country,
                cvc_check,
                description,
                dynamic_last4,
                exp_month,
                exp_year,
                fingerprint,
                funding,
                iin,
                issuer,
                last4,
                name,
                three_d_secure,
                tokenization_method,
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

    impl ObjectDeser for SourceTypeCard {
        type Builder = SourceTypeCardBuilder;
    }
};
