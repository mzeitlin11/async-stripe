#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentLinksResourceConsentCollection {
    /// If set to `auto`, enables the collection of customer consent for promotional communications.
    pub promotions: Option<PaymentLinksResourceConsentCollectionPromotions>,
    /// If set to `required`, it requires cutomers to accept the terms of service before being able to pay.
    /// If set to `none`, customers won't be shown a checkbox to accept the terms of service.
    pub terms_of_service: Option<PaymentLinksResourceConsentCollectionTermsOfService>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentLinksResourceConsentCollectionBuilder {
    promotions: Option<Option<PaymentLinksResourceConsentCollectionPromotions>>,
    terms_of_service: Option<Option<PaymentLinksResourceConsentCollectionTermsOfService>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentLinksResourceConsentCollection {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceConsentCollection>,
        builder: PaymentLinksResourceConsentCollectionBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceConsentCollection> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentLinksResourceConsentCollectionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentLinksResourceConsentCollectionBuilder {
        type Out = PaymentLinksResourceConsentCollection;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "promotions" => Ok(Deserialize::begin(&mut self.promotions)),
                "terms_of_service" => Ok(Deserialize::begin(&mut self.terms_of_service)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { promotions: Deserialize::default(), terms_of_service: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let promotions = self.promotions.take()?;
            let terms_of_service = self.terms_of_service.take()?;

            Some(Self::Out { promotions, terms_of_service })
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

    impl ObjectDeser for PaymentLinksResourceConsentCollection {
        type Builder = PaymentLinksResourceConsentCollectionBuilder;
    }
};
/// If set to `auto`, enables the collection of customer consent for promotional communications.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinksResourceConsentCollectionPromotions {
    Auto,
    None,
}
impl PaymentLinksResourceConsentCollectionPromotions {
    pub fn as_str(self) -> &'static str {
        use PaymentLinksResourceConsentCollectionPromotions::*;
        match self {
            Auto => "auto",
            None => "none",
        }
    }
}

impl std::str::FromStr for PaymentLinksResourceConsentCollectionPromotions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourceConsentCollectionPromotions::*;
        match s {
            "auto" => Ok(Auto),
            "none" => Ok(None),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentLinksResourceConsentCollectionPromotions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentLinksResourceConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinksResourceConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinksResourceConsentCollectionPromotions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinksResourceConsentCollectionPromotions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentLinksResourceConsentCollectionPromotions"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentLinksResourceConsentCollectionPromotions {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentLinksResourceConsentCollectionPromotions> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentLinksResourceConsentCollectionPromotions::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// If set to `required`, it requires cutomers to accept the terms of service before being able to pay.
/// If set to `none`, customers won't be shown a checkbox to accept the terms of service.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinksResourceConsentCollectionTermsOfService {
    None,
    Required,
}
impl PaymentLinksResourceConsentCollectionTermsOfService {
    pub fn as_str(self) -> &'static str {
        use PaymentLinksResourceConsentCollectionTermsOfService::*;
        match self {
            None => "none",
            Required => "required",
        }
    }
}

impl std::str::FromStr for PaymentLinksResourceConsentCollectionTermsOfService {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourceConsentCollectionTermsOfService::*;
        match s {
            "none" => Ok(None),
            "required" => Ok(Required),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PaymentLinksResourceConsentCollectionTermsOfService {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PaymentLinksResourceConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinksResourceConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinksResourceConsentCollectionTermsOfService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinksResourceConsentCollectionTermsOfService {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentLinksResourceConsentCollectionTermsOfService"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentLinksResourceConsentCollectionTermsOfService {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentLinksResourceConsentCollectionTermsOfService> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentLinksResourceConsentCollectionTermsOfService::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
