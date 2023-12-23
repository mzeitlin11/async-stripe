#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfiguration<'a> {
    /// An object containing device type specific settings for BBPOS WisePOS E readers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbpos_wisepos_e: Option<CreateTerminalConfigurationBbposWiseposE<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Configurations for collecting transactions offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<CreateTerminalConfigurationOffline>,
    /// Tipping configurations for readers supporting on-reader tips
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<CreateTerminalConfigurationTipping<'a>>,
    /// An object containing device type specific settings for Verifone P400 readers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifone_p400: Option<CreateTerminalConfigurationVerifoneP400<'a>>,
}
impl<'a> CreateTerminalConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An object containing device type specific settings for BBPOS WisePOS E readers
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationBbposWiseposE<'a> {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<&'a str>,
}
impl<'a> CreateTerminalConfigurationBbposWiseposE<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configurations for collecting transactions offline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTerminalConfigurationOffline {
    /// Determines whether to allow transactions to be collected while reader is offline.
    /// Defaults to false.
    pub enabled: bool,
}
impl CreateTerminalConfigurationOffline {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Tipping configurations for readers supporting on-reader tips
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationTipping<'a> {
    /// Tipping configuration for AUD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<CreateTerminalConfigurationTippingAud<'a>>,
    /// Tipping configuration for CAD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cad: Option<CreateTerminalConfigurationTippingCad<'a>>,
    /// Tipping configuration for CHF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf: Option<CreateTerminalConfigurationTippingChf<'a>>,
    /// Tipping configuration for CZK
    #[serde(skip_serializing_if = "Option::is_none")]
    pub czk: Option<CreateTerminalConfigurationTippingCzk<'a>>,
    /// Tipping configuration for DKK
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkk: Option<CreateTerminalConfigurationTippingDkk<'a>>,
    /// Tipping configuration for EUR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur: Option<CreateTerminalConfigurationTippingEur<'a>>,
    /// Tipping configuration for GBP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp: Option<CreateTerminalConfigurationTippingGbp<'a>>,
    /// Tipping configuration for HKD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkd: Option<CreateTerminalConfigurationTippingHkd<'a>>,
    /// Tipping configuration for MYR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myr: Option<CreateTerminalConfigurationTippingMyr<'a>>,
    /// Tipping configuration for NOK
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nok: Option<CreateTerminalConfigurationTippingNok<'a>>,
    /// Tipping configuration for NZD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nzd: Option<CreateTerminalConfigurationTippingNzd<'a>>,
    /// Tipping configuration for SEK
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sek: Option<CreateTerminalConfigurationTippingSek<'a>>,
    /// Tipping configuration for SGD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgd: Option<CreateTerminalConfigurationTippingSgd<'a>>,
    /// Tipping configuration for USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<CreateTerminalConfigurationTippingUsd<'a>>,
}
impl<'a> CreateTerminalConfigurationTipping<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for AUD
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationTippingAud<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> CreateTerminalConfigurationTippingAud<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for CAD
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationTippingCad<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> CreateTerminalConfigurationTippingCad<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for CHF
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationTippingChf<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> CreateTerminalConfigurationTippingChf<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for CZK
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationTippingCzk<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> CreateTerminalConfigurationTippingCzk<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for DKK
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationTippingDkk<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> CreateTerminalConfigurationTippingDkk<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for EUR
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationTippingEur<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> CreateTerminalConfigurationTippingEur<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for GBP
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationTippingGbp<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> CreateTerminalConfigurationTippingGbp<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for HKD
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationTippingHkd<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> CreateTerminalConfigurationTippingHkd<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for MYR
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationTippingMyr<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> CreateTerminalConfigurationTippingMyr<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for NOK
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationTippingNok<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> CreateTerminalConfigurationTippingNok<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for NZD
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationTippingNzd<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> CreateTerminalConfigurationTippingNzd<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for SEK
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationTippingSek<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> CreateTerminalConfigurationTippingSek<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for SGD
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationTippingSgd<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> CreateTerminalConfigurationTippingSgd<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for USD
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationTippingUsd<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> CreateTerminalConfigurationTippingUsd<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An object containing device type specific settings for Verifone P400 readers
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTerminalConfigurationVerifoneP400<'a> {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<&'a str>,
}
impl<'a> CreateTerminalConfigurationVerifoneP400<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> CreateTerminalConfiguration<'a> {
    /// Creates a new `Configuration` object.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_terminal::TerminalConfiguration> {
        client.send_form("/terminal/configurations", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListTerminalConfiguration<'a> {
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// if present, only return the account default or non-default configurations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_account_default: Option<bool>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListTerminalConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListTerminalConfiguration<'a> {
    /// Returns a list of `Configuration` objects.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_terminal::TerminalConfiguration>> {
        client.get_query("/terminal/configurations", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_terminal::TerminalConfiguration>> {
        stripe::ListPaginator::from_list_params("/terminal/configurations", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTerminalConfiguration<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTerminalConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTerminalConfiguration<'a> {
    /// Retrieves a `Configuration` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        configuration: &stripe_terminal::TerminalConfigurationId,
    ) -> stripe::Response<RetrieveReturned> {
        client.get_query(&format!("/terminal/configurations/{configuration}"), self)
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum RetrieveReturned {
    #[serde(rename = "terminal.configuration")]
    DeletedTerminalConfiguration(stripe_terminal::DeletedTerminalConfiguration),
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfiguration<'a> {
    /// An object containing device type specific settings for BBPOS WisePOS E readers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbpos_wisepos_e: Option<UpdateTerminalConfigurationBbposWiseposE<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Configurations for collecting transactions offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<UpdateTerminalConfigurationOffline>,
    /// Tipping configurations for readers supporting on-reader tips
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<UpdateTerminalConfigurationTipping<'a>>,
    /// An object containing device type specific settings for Verifone P400 readers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifone_p400: Option<UpdateTerminalConfigurationVerifoneP400<'a>>,
}
impl<'a> UpdateTerminalConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An object containing device type specific settings for BBPOS WisePOS E readers
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationBbposWiseposE<'a> {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<&'a str>,
}
impl<'a> UpdateTerminalConfigurationBbposWiseposE<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configurations for collecting transactions offline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTerminalConfigurationOffline {
    /// Determines whether to allow transactions to be collected while reader is offline.
    /// Defaults to false.
    pub enabled: bool,
}
impl UpdateTerminalConfigurationOffline {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Tipping configurations for readers supporting on-reader tips
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationTipping<'a> {
    /// Tipping configuration for AUD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<UpdateTerminalConfigurationTippingAud<'a>>,
    /// Tipping configuration for CAD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cad: Option<UpdateTerminalConfigurationTippingCad<'a>>,
    /// Tipping configuration for CHF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf: Option<UpdateTerminalConfigurationTippingChf<'a>>,
    /// Tipping configuration for CZK
    #[serde(skip_serializing_if = "Option::is_none")]
    pub czk: Option<UpdateTerminalConfigurationTippingCzk<'a>>,
    /// Tipping configuration for DKK
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkk: Option<UpdateTerminalConfigurationTippingDkk<'a>>,
    /// Tipping configuration for EUR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur: Option<UpdateTerminalConfigurationTippingEur<'a>>,
    /// Tipping configuration for GBP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp: Option<UpdateTerminalConfigurationTippingGbp<'a>>,
    /// Tipping configuration for HKD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkd: Option<UpdateTerminalConfigurationTippingHkd<'a>>,
    /// Tipping configuration for MYR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myr: Option<UpdateTerminalConfigurationTippingMyr<'a>>,
    /// Tipping configuration for NOK
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nok: Option<UpdateTerminalConfigurationTippingNok<'a>>,
    /// Tipping configuration for NZD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nzd: Option<UpdateTerminalConfigurationTippingNzd<'a>>,
    /// Tipping configuration for SEK
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sek: Option<UpdateTerminalConfigurationTippingSek<'a>>,
    /// Tipping configuration for SGD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgd: Option<UpdateTerminalConfigurationTippingSgd<'a>>,
    /// Tipping configuration for USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<UpdateTerminalConfigurationTippingUsd<'a>>,
}
impl<'a> UpdateTerminalConfigurationTipping<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for AUD
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationTippingAud<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> UpdateTerminalConfigurationTippingAud<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for CAD
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationTippingCad<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> UpdateTerminalConfigurationTippingCad<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for CHF
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationTippingChf<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> UpdateTerminalConfigurationTippingChf<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for CZK
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationTippingCzk<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> UpdateTerminalConfigurationTippingCzk<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for DKK
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationTippingDkk<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> UpdateTerminalConfigurationTippingDkk<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for EUR
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationTippingEur<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> UpdateTerminalConfigurationTippingEur<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for GBP
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationTippingGbp<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> UpdateTerminalConfigurationTippingGbp<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for HKD
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationTippingHkd<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> UpdateTerminalConfigurationTippingHkd<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for MYR
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationTippingMyr<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> UpdateTerminalConfigurationTippingMyr<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for NOK
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationTippingNok<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> UpdateTerminalConfigurationTippingNok<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for NZD
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationTippingNzd<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> UpdateTerminalConfigurationTippingNzd<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for SEK
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationTippingSek<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> UpdateTerminalConfigurationTippingSek<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for SGD
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationTippingSgd<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> UpdateTerminalConfigurationTippingSgd<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for USD
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationTippingUsd<'a> {
    /// Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<&'a [i64]>,
    /// Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<&'a [i64]>,
    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl<'a> UpdateTerminalConfigurationTippingUsd<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An object containing device type specific settings for Verifone P400 readers
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalConfigurationVerifoneP400<'a> {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<&'a str>,
}
impl<'a> UpdateTerminalConfigurationVerifoneP400<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdateTerminalConfiguration<'a> {
    /// Updates a new `Configuration` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        configuration: &stripe_terminal::TerminalConfigurationId,
    ) -> stripe::Response<UpdateReturned> {
        client.send_form(
            &format!("/terminal/configurations/{configuration}"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum UpdateReturned {
    #[serde(rename = "terminal.configuration")]
    DeletedTerminalConfiguration(stripe_terminal::DeletedTerminalConfiguration),
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeleteTerminalConfiguration {}
impl DeleteTerminalConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
impl DeleteTerminalConfiguration {
    /// Deletes a `Configuration` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        configuration: &stripe_terminal::TerminalConfigurationId,
    ) -> stripe::Response<stripe_terminal::DeletedTerminalConfiguration> {
        client.send_form(
            &format!("/terminal/configurations/{configuration}"),
            self,
            http_types::Method::Delete,
        )
    }
}
