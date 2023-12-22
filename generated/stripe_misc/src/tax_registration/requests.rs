#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListTaxRegistration<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// The status of the Tax Registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListTaxRegistrationStatus>,
}
impl<'a> ListTaxRegistration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The status of the Tax Registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTaxRegistrationStatus {
    Active,
    All,
    Expired,
    Scheduled,
}
impl ListTaxRegistrationStatus {
    pub fn as_str(self) -> &'static str {
        use ListTaxRegistrationStatus::*;
        match self {
            Active => "active",
            All => "all",
            Expired => "expired",
            Scheduled => "scheduled",
        }
    }
}

impl std::str::FromStr for ListTaxRegistrationStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTaxRegistrationStatus::*;
        match s {
            "active" => Ok(Active),
            "all" => Ok(All),
            "expired" => Ok(Expired),
            "scheduled" => Ok(Scheduled),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for ListTaxRegistrationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for ListTaxRegistrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTaxRegistrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTaxRegistrationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> ListTaxRegistration<'a> {
    /// Returns a list of Tax `Registration` objects.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_misc::TaxRegistration>> {
        client.get_query("/tax/registrations", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_misc::TaxRegistration>> {
        stripe::ListPaginator::from_list_params("/tax/registrations", self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistration<'a> {
    /// Time at which the Tax Registration becomes active.
    ///
    /// It can be either `now` to indicate the current time, or a future timestamp measured in seconds since the Unix epoch.
    pub active_from: CreateTaxRegistrationActiveFrom,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: &'a str,
    /// Specific options for a registration in the specified `country`.
    pub country_options: CreateTaxRegistrationCountryOptions<'a>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// If set, the Tax Registration stops being active at this time.
    ///
    /// If not set, the Tax Registration will be active indefinitely.
    /// Timestamp measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<stripe_types::Timestamp>,
}
impl<'a> CreateTaxRegistration<'a> {
    pub fn new(
        active_from: CreateTaxRegistrationActiveFrom,
        country: &'a str,
        country_options: CreateTaxRegistrationCountryOptions<'a>,
    ) -> Self {
        Self { active_from, country, country_options, expand: None, expires_at: None }
    }
}
/// Time at which the Tax Registration becomes active.
///
/// It can be either `now` to indicate the current time, or a future timestamp measured in seconds since the Unix epoch.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum CreateTaxRegistrationActiveFrom {
    Now,
    Timestamp(stripe_types::Timestamp),
}
/// Specific options for a registration in the specified `country`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptions<'a> {
    /// Options for the registration in AE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ae: Option<CreateTaxRegistrationCountryOptionsAe>,
    /// Options for the registration in AT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<CreateTaxRegistrationCountryOptionsAt>,
    /// Options for the registration in AU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au: Option<CreateTaxRegistrationCountryOptionsAu>,
    /// Options for the registration in BE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub be: Option<CreateTaxRegistrationCountryOptionsBe>,
    /// Options for the registration in BG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg: Option<CreateTaxRegistrationCountryOptionsBg>,
    /// Options for the registration in CA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca: Option<CreateTaxRegistrationCountryOptionsCa<'a>>,
    /// Options for the registration in CH.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch: Option<CreateTaxRegistrationCountryOptionsCh>,
    /// Options for the registration in CL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl: Option<CreateTaxRegistrationCountryOptionsCl>,
    /// Options for the registration in CO.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub co: Option<CreateTaxRegistrationCountryOptionsCo>,
    /// Options for the registration in CY.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cy: Option<CreateTaxRegistrationCountryOptionsCy>,
    /// Options for the registration in CZ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cz: Option<CreateTaxRegistrationCountryOptionsCz>,
    /// Options for the registration in DE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub de: Option<CreateTaxRegistrationCountryOptionsDe>,
    /// Options for the registration in DK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dk: Option<CreateTaxRegistrationCountryOptionsDk>,
    /// Options for the registration in EE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ee: Option<CreateTaxRegistrationCountryOptionsEe>,
    /// Options for the registration in ES.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub es: Option<CreateTaxRegistrationCountryOptionsEs>,
    /// Options for the registration in FI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fi: Option<CreateTaxRegistrationCountryOptionsFi>,
    /// Options for the registration in FR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fr: Option<CreateTaxRegistrationCountryOptionsFr>,
    /// Options for the registration in GB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gb: Option<CreateTaxRegistrationCountryOptionsGb>,
    /// Options for the registration in GR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gr: Option<CreateTaxRegistrationCountryOptionsGr>,
    /// Options for the registration in HR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hr: Option<CreateTaxRegistrationCountryOptionsHr>,
    /// Options for the registration in HU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hu: Option<CreateTaxRegistrationCountryOptionsHu>,
    /// Options for the registration in ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<CreateTaxRegistrationCountryOptionsId>,
    /// Options for the registration in IE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ie: Option<CreateTaxRegistrationCountryOptionsIe>,
    /// Options for the registration in IS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is: Option<CreateTaxRegistrationCountryOptionsIs>,
    /// Options for the registration in IT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub it: Option<CreateTaxRegistrationCountryOptionsIt>,
    /// Options for the registration in JP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jp: Option<CreateTaxRegistrationCountryOptionsJp>,
    /// Options for the registration in KR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kr: Option<CreateTaxRegistrationCountryOptionsKr>,
    /// Options for the registration in LT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<CreateTaxRegistrationCountryOptionsLt>,
    /// Options for the registration in LU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lu: Option<CreateTaxRegistrationCountryOptionsLu>,
    /// Options for the registration in LV.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lv: Option<CreateTaxRegistrationCountryOptionsLv>,
    /// Options for the registration in MT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mt: Option<CreateTaxRegistrationCountryOptionsMt>,
    /// Options for the registration in MX.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mx: Option<CreateTaxRegistrationCountryOptionsMx>,
    /// Options for the registration in MY.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my: Option<CreateTaxRegistrationCountryOptionsMy>,
    /// Options for the registration in NL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nl: Option<CreateTaxRegistrationCountryOptionsNl>,
    /// Options for the registration in NO.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no: Option<CreateTaxRegistrationCountryOptionsNo>,
    /// Options for the registration in NZ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nz: Option<CreateTaxRegistrationCountryOptionsNz>,
    /// Options for the registration in PL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pl: Option<CreateTaxRegistrationCountryOptionsPl>,
    /// Options for the registration in PT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pt: Option<CreateTaxRegistrationCountryOptionsPt>,
    /// Options for the registration in RO.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ro: Option<CreateTaxRegistrationCountryOptionsRo>,
    /// Options for the registration in SA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sa: Option<CreateTaxRegistrationCountryOptionsSa>,
    /// Options for the registration in SE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se: Option<CreateTaxRegistrationCountryOptionsSe>,
    /// Options for the registration in SG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sg: Option<CreateTaxRegistrationCountryOptionsSg>,
    /// Options for the registration in SI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub si: Option<CreateTaxRegistrationCountryOptionsSi>,
    /// Options for the registration in SK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sk: Option<CreateTaxRegistrationCountryOptionsSk>,
    /// Options for the registration in TH.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub th: Option<CreateTaxRegistrationCountryOptionsTh>,
    /// Options for the registration in TR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tr: Option<CreateTaxRegistrationCountryOptionsTr>,
    /// Options for the registration in US.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us: Option<CreateTaxRegistrationCountryOptionsUs<'a>>,
    /// Options for the registration in VN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vn: Option<CreateTaxRegistrationCountryOptionsVn>,
    /// Options for the registration in ZA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub za: Option<CreateTaxRegistrationCountryOptionsZa>,
}
impl<'a> CreateTaxRegistrationCountryOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Options for the registration in AE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAe {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsAeType,
}
impl CreateTaxRegistrationCountryOptionsAe {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsAeType) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsAeType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsAeType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsAeType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsAeType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsAeType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsAeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsAeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsAeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in AT.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAt {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsAtStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsAtType,
}
impl CreateTaxRegistrationCountryOptionsAt {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsAtType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAtStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsAtStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsAtType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsAtType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsAtType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsAtType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsAtType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsAtType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsAtType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAtType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsAtType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in AU.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAu {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsAuType,
}
impl CreateTaxRegistrationCountryOptionsAu {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsAuType) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsAuType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsAuType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsAuType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsAuType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsAuType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsAuType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsAuType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAuType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsAuType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in BE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBe {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsBeStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsBeType,
}
impl CreateTaxRegistrationCountryOptionsBe {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsBeType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBeStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsBeStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsBeType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsBeType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBeType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsBeType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsBeType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsBeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsBeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsBeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in BG.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBg {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsBgStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsBgType,
}
impl CreateTaxRegistrationCountryOptionsBg {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsBgType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBgStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsBgStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsBgType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsBgType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBgType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsBgType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsBgType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsBgType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsBgType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBgType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsBgType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in CA.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCa<'a> {
    /// Options for the provincial tax registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_standard: Option<CreateTaxRegistrationCountryOptionsCaProvinceStandard<'a>>,
    /// Type of registration to be created in Canada.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsCaType,
}
impl<'a> CreateTaxRegistrationCountryOptionsCa<'a> {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsCaType) -> Self {
        Self { province_standard: None, type_ }
    }
}
/// Options for the provincial tax registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCaProvinceStandard<'a> {
    /// Two-letter CA province code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    pub province: &'a str,
}
impl<'a> CreateTaxRegistrationCountryOptionsCaProvinceStandard<'a> {
    pub fn new(province: &'a str) -> Self {
        Self { province }
    }
}
/// Type of registration to be created in Canada.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsCaType {
    ProvinceStandard,
    Simplified,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsCaType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsCaType::*;
        match self {
            ProvinceStandard => "province_standard",
            Simplified => "simplified",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsCaType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsCaType::*;
        match s {
            "province_standard" => Ok(ProvinceStandard),
            "simplified" => Ok(Simplified),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsCaType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsCaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsCaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in CH.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCh {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsChType,
}
impl CreateTaxRegistrationCountryOptionsCh {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsChType) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsChType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsChType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsChType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsChType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsChType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsChType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsChType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsChType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsChType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in CL.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCl {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsClType,
}
impl CreateTaxRegistrationCountryOptionsCl {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsClType) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsClType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsClType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsClType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsClType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsClType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsClType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsClType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsClType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsClType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in CO.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCo {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsCoType,
}
impl CreateTaxRegistrationCountryOptionsCo {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsCoType) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsCoType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsCoType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsCoType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsCoType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsCoType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsCoType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsCoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsCoType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in CY.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCy {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsCyStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsCyType,
}
impl CreateTaxRegistrationCountryOptionsCy {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsCyType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCyStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsCyStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsCyType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsCyType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsCyType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsCyType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsCyType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsCyType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsCyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsCyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in CZ.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCz {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsCzStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsCzType,
}
impl CreateTaxRegistrationCountryOptionsCz {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsCzType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCzStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsCzStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsCzType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsCzType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsCzType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsCzType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsCzType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsCzType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsCzType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCzType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsCzType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in DE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsDe {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsDeStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsDeType,
}
impl CreateTaxRegistrationCountryOptionsDe {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsDeType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsDeStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsDeStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsDeType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsDeType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsDeType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsDeType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsDeType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsDeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsDeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsDeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsDeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in DK.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsDk {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsDkStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsDkType,
}
impl CreateTaxRegistrationCountryOptionsDk {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsDkType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsDkStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsDkStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsDkType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsDkType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsDkType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsDkType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsDkType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsDkType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsDkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsDkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsDkType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in EE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsEe {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsEeStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsEeType,
}
impl CreateTaxRegistrationCountryOptionsEe {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsEeType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsEeStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsEeStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsEeType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsEeType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsEeType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsEeType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsEeType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsEeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsEeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsEeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsEeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in ES.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsEs {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsEsStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsEsType,
}
impl CreateTaxRegistrationCountryOptionsEs {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsEsType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsEsStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsEsStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsEsType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsEsType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsEsType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsEsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsEsType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsEsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsEsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsEsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsEsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in FI.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsFi {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsFiStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsFiType,
}
impl CreateTaxRegistrationCountryOptionsFi {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsFiType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsFiStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsFiStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsFiType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsFiType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsFiType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsFiType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsFiType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsFiType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsFiType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsFiType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsFiType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in FR.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsFr {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsFrStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsFrType,
}
impl CreateTaxRegistrationCountryOptionsFr {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsFrType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsFrStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsFrStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsFrType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsFrType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsFrType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsFrType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsFrType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsFrType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsFrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsFrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsFrType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in GB.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsGb {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsGbType,
}
impl CreateTaxRegistrationCountryOptionsGb {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsGbType) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsGbType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsGbType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsGbType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsGbType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsGbType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsGbType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsGbType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsGbType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsGbType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in GR.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsGr {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsGrStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsGrType,
}
impl CreateTaxRegistrationCountryOptionsGr {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsGrType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsGrStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsGrStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsGrType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsGrType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsGrType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsGrType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsGrType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsGrType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsGrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsGrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsGrType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in HR.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsHr {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsHrStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsHrType,
}
impl CreateTaxRegistrationCountryOptionsHr {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsHrType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsHrStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsHrStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsHrType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsHrType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsHrType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsHrType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsHrType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsHrType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsHrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsHrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsHrType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in HU.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsHu {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsHuStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsHuType,
}
impl CreateTaxRegistrationCountryOptionsHu {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsHuType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsHuStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsHuStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsHuType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsHuType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsHuType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsHuType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsHuType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsHuType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsHuType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsHuType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsHuType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in ID.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsId {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsIdType,
}
impl CreateTaxRegistrationCountryOptionsId {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsIdType) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsIdType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsIdType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsIdType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsIdType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsIdType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsIdType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsIdType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in IE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsIe {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsIeStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsIeType,
}
impl CreateTaxRegistrationCountryOptionsIe {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsIeType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsIeStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsIeStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsIeType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsIeType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsIeType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsIeType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsIeType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsIeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsIeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsIeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsIeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in IS.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsIs {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsIsType,
}
impl CreateTaxRegistrationCountryOptionsIs {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsIsType) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsIsType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsIsType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsIsType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsIsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsIsType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsIsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsIsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsIsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsIsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in IT.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsIt {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsItStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsItType,
}
impl CreateTaxRegistrationCountryOptionsIt {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsItType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsItStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsItStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsItType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsItType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsItType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsItType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsItType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsItType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsItType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsItType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsItType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in JP.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsJp {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsJpType,
}
impl CreateTaxRegistrationCountryOptionsJp {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsJpType) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsJpType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsJpType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsJpType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsJpType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsJpType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsJpType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsJpType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsJpType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsJpType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in KR.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsKr {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsKrType,
}
impl CreateTaxRegistrationCountryOptionsKr {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsKrType) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsKrType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsKrType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsKrType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsKrType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsKrType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsKrType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsKrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsKrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsKrType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in LT.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsLt {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsLtStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsLtType,
}
impl CreateTaxRegistrationCountryOptionsLt {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsLtType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsLtStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsLtStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsLtType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsLtType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsLtType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsLtType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsLtType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsLtType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsLtType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsLtType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsLtType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in LU.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsLu {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsLuStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsLuType,
}
impl CreateTaxRegistrationCountryOptionsLu {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsLuType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsLuStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsLuStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsLuType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsLuType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsLuType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsLuType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsLuType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsLuType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsLuType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsLuType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsLuType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in LV.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsLv {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsLvStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsLvType,
}
impl CreateTaxRegistrationCountryOptionsLv {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsLvType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsLvStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsLvStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsLvType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsLvType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsLvType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsLvType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsLvType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsLvType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsLvType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsLvType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsLvType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in MT.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsMt {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsMtStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsMtType,
}
impl CreateTaxRegistrationCountryOptionsMt {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsMtType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsMtStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsMtStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsMtType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsMtType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsMtType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsMtType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsMtType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsMtType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsMtType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMtType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsMtType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in MX.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsMx {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsMxType,
}
impl CreateTaxRegistrationCountryOptionsMx {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsMxType) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsMxType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsMxType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsMxType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsMxType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsMxType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsMxType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsMxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsMxType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in MY.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsMy {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsMyType,
}
impl CreateTaxRegistrationCountryOptionsMy {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsMyType) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsMyType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsMyType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsMyType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsMyType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsMyType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsMyType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsMyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsMyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in NL.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsNl {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsNlStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsNlType,
}
impl CreateTaxRegistrationCountryOptionsNl {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsNlType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsNlStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsNlStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsNlType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsNlType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsNlType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsNlType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsNlType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsNlType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsNlType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsNlType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsNlType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in NO.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsNo {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsNoType,
}
impl CreateTaxRegistrationCountryOptionsNo {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsNoType) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsNoType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsNoType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsNoType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsNoType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsNoType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsNoType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsNoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsNoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsNoType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in NZ.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsNz {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsNzType,
}
impl CreateTaxRegistrationCountryOptionsNz {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsNzType) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsNzType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsNzType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsNzType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsNzType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsNzType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsNzType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsNzType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsNzType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsNzType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in PL.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsPl {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsPlStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsPlType,
}
impl CreateTaxRegistrationCountryOptionsPl {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsPlType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsPlStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsPlStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsPlType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsPlType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsPlType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsPlType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsPlType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsPlType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsPlType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsPlType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsPlType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in PT.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsPt {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsPtStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsPtType,
}
impl CreateTaxRegistrationCountryOptionsPt {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsPtType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsPtStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsPtStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsPtType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsPtType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsPtType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsPtType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsPtType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsPtType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsPtType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsPtType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsPtType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in RO.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsRo {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsRoStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsRoType,
}
impl CreateTaxRegistrationCountryOptionsRo {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsRoType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsRoStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsRoStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsRoType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsRoType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsRoType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsRoType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsRoType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsRoType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsRoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsRoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsRoType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in SA.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsSa {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsSaType,
}
impl CreateTaxRegistrationCountryOptionsSa {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsSaType) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsSaType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsSaType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSaType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsSaType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsSaType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsSaType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsSaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsSaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in SE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsSe {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsSeStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsSeType,
}
impl CreateTaxRegistrationCountryOptionsSe {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsSeType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsSeStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsSeStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsSeType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsSeType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSeType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsSeType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsSeType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsSeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsSeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsSeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in SG.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsSg {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsSgType,
}
impl CreateTaxRegistrationCountryOptionsSg {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsSgType) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsSgType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsSgType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSgType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsSgType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsSgType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsSgType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsSgType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSgType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsSgType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in SI.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsSi {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsSiStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsSiType,
}
impl CreateTaxRegistrationCountryOptionsSi {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsSiType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsSiStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsSiStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsSiType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsSiType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSiType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsSiType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsSiType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsSiType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsSiType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSiType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsSiType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in SK.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsSk {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsSkStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsSkType,
}
impl CreateTaxRegistrationCountryOptionsSk {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsSkType) -> Self {
        Self { standard: None, type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsSkStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsSkStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsSkType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsSkType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSkType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsSkType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsSkType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsSkType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsSkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsSkType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in TH.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsTh {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsThType,
}
impl CreateTaxRegistrationCountryOptionsTh {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsThType) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsThType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsThType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsThType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsThType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsThType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsThType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsThType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsThType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsThType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in TR.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsTr {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsTrType,
}
impl CreateTaxRegistrationCountryOptionsTr {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsTrType) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsTrType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsTrType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsTrType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsTrType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsTrType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsTrType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsTrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsTrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsTrType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in US.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUs<'a> {
    /// Options for the local amusement tax registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_amusement_tax: Option<CreateTaxRegistrationCountryOptionsUsLocalAmusementTax<'a>>,
    /// Options for the local lease tax registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_lease_tax: Option<CreateTaxRegistrationCountryOptionsUsLocalLeaseTax<'a>>,
    /// Two-letter US state code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    pub state: &'a str,
    /// Type of registration to be created in the US.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsUsType,
}
impl<'a> CreateTaxRegistrationCountryOptionsUs<'a> {
    pub fn new(state: &'a str, type_: CreateTaxRegistrationCountryOptionsUsType) -> Self {
        Self { local_amusement_tax: None, local_lease_tax: None, state, type_ }
    }
}
/// Options for the local amusement tax registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUsLocalAmusementTax<'a> {
    /// A [FIPS code](https://www.census.gov/library/reference/code-lists/ansi.html) representing the local jurisdiction.
    ///
    /// Supported FIPS codes are: `14000` (Chicago), `06613` (Bloomington), `21696` (East Dundee), `24582` (Evanston), and `68081` (Schiller Park).
    pub jurisdiction: &'a str,
}
impl<'a> CreateTaxRegistrationCountryOptionsUsLocalAmusementTax<'a> {
    pub fn new(jurisdiction: &'a str) -> Self {
        Self { jurisdiction }
    }
}
/// Options for the local lease tax registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUsLocalLeaseTax<'a> {
    /// A [FIPS code](https://www.census.gov/library/reference/code-lists/ansi.html) representing the local jurisdiction.
    ///
    /// Supported FIPS codes are: `14000` (Chicago).
    pub jurisdiction: &'a str,
}
impl<'a> CreateTaxRegistrationCountryOptionsUsLocalLeaseTax<'a> {
    pub fn new(jurisdiction: &'a str) -> Self {
        Self { jurisdiction }
    }
}
/// Type of registration to be created in the US.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsUsType {
    LocalAmusementTax,
    LocalLeaseTax,
    StateCommunicationsTax,
    StateSalesTax,
}
impl CreateTaxRegistrationCountryOptionsUsType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsUsType::*;
        match self {
            LocalAmusementTax => "local_amusement_tax",
            LocalLeaseTax => "local_lease_tax",
            StateCommunicationsTax => "state_communications_tax",
            StateSalesTax => "state_sales_tax",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsUsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsUsType::*;
        match s {
            "local_amusement_tax" => Ok(LocalAmusementTax),
            "local_lease_tax" => Ok(LocalLeaseTax),
            "state_communications_tax" => Ok(StateCommunicationsTax),
            "state_sales_tax" => Ok(StateSalesTax),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsUsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsUsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsUsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in VN.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsVn {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsVnType,
}
impl CreateTaxRegistrationCountryOptionsVn {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsVnType) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsVnType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsVnType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsVnType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsVnType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsVnType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsVnType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsVnType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsVnType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsVnType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in ZA.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsZa {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsZaType,
}
impl CreateTaxRegistrationCountryOptionsZa {
    pub fn new(type_: CreateTaxRegistrationCountryOptionsZaType) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsZaType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsZaType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsZaType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsZaType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsZaType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreateTaxRegistrationCountryOptionsZaType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsZaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsZaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsZaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> CreateTaxRegistration<'a> {
    /// Creates a new Tax `Registration` object.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_misc::TaxRegistration> {
        client.send_form("/tax/registrations", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTaxRegistration<'a> {
    /// Time at which the registration becomes active.
    ///
    /// It can be either `now` to indicate the current time, or a timestamp measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_from: Option<UpdateTaxRegistrationActiveFrom>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// If set, the registration stops being active at this time.
    ///
    /// If not set, the registration will be active indefinitely.
    /// It can be either `now` to indicate the current time, or a timestamp measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<UpdateTaxRegistrationExpiresAt>,
}
impl<'a> UpdateTaxRegistration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Time at which the registration becomes active.
///
/// It can be either `now` to indicate the current time, or a timestamp measured in seconds since the Unix epoch.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum UpdateTaxRegistrationActiveFrom {
    Now,
    Timestamp(stripe_types::Timestamp),
}
/// If set, the registration stops being active at this time.
///
/// If not set, the registration will be active indefinitely.
/// It can be either `now` to indicate the current time, or a timestamp measured in seconds since the Unix epoch.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum UpdateTaxRegistrationExpiresAt {
    Now,
    Timestamp(stripe_types::Timestamp),
}
impl<'a> UpdateTaxRegistration<'a> {
    /// Updates an existing Tax `Registration` object.
    ///
    /// A registration cannot be deleted after it has been created.
    ///
    /// If you wish to end a registration you may do so by setting `expires_at`.
    pub fn send(
        &self,
        client: &stripe::Client,
        id: &stripe_misc::tax_registration::TaxRegistrationId,
    ) -> stripe::Response<stripe_misc::TaxRegistration> {
        client.send_form(&format!("/tax/registrations/{id}"), self, http_types::Method::Post)
    }
}
