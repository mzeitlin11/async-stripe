#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalReaderReader<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The new label of the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateTerminalReaderReader<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdateTerminalReaderReader<'a> {
    /// Updates a `Reader` object by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.
    pub fn send(&self, client: &stripe::Client, reader: &str) -> stripe::Response<UpdateReturned> {
        client.send_form(&format!("/terminal/readers/{reader}"), self, http_types::Method::Post)
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum UpdateReturned {
    #[serde(rename = "terminal.reader")]
    TerminalReaderDeletedReader(stripe_terminal::TerminalReaderDeletedReader),
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTerminalReaderReader<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTerminalReaderReader<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTerminalReaderReader<'a> {
    /// Retrieves a `Reader` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        reader: &str,
    ) -> stripe::Response<RetrieveReturned> {
        client.get_query(&format!("/terminal/readers/{reader}"), self)
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum RetrieveReturned {
    #[serde(rename = "terminal.reader")]
    TerminalReaderDeletedReader(stripe_terminal::TerminalReaderDeletedReader),
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTerminalReaderReader<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Custom label given to the reader for easier identification.
    ///
    /// If no label is specified, the registration code will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<&'a str>,
    /// The location to assign the reader to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// A code generated by the reader used for registering to an account.
    pub registration_code: &'a str,
}
impl<'a> CreateTerminalReaderReader<'a> {
    pub fn new(registration_code: &'a str) -> Self {
        Self {
            expand: Default::default(),
            label: Default::default(),
            location: Default::default(),
            metadata: Default::default(),
            registration_code,
        }
    }
}
impl<'a> CreateTerminalReaderReader<'a> {
    /// Creates a new `Reader` object.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_terminal::TerminalReaderReader> {
        client.send_form("/terminal/readers", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListTerminalReaderReader<'a> {
    /// Filters readers by device type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<ListTerminalReaderReaderDeviceType>,
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
    /// A location ID to filter the response list to only readers at the specific location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<&'a str>,
    /// Filters readers by serial number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// A status filter to filter readers to only offline or online readers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListTerminalReaderReaderStatus>,
}
impl<'a> ListTerminalReaderReader<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Filters readers by device type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTerminalReaderReaderDeviceType {
    BbposChipper2x,
    BbposWisepad3,
    BbposWiseposE,
    SimulatedWiseposE,
    StripeM2,
    VerifoneP400,
}
impl ListTerminalReaderReaderDeviceType {
    pub fn as_str(self) -> &'static str {
        use ListTerminalReaderReaderDeviceType::*;
        match self {
            BbposChipper2x => "bbpos_chipper2x",
            BbposWisepad3 => "bbpos_wisepad3",
            BbposWiseposE => "bbpos_wisepos_e",
            SimulatedWiseposE => "simulated_wisepos_e",
            StripeM2 => "stripe_m2",
            VerifoneP400 => "verifone_P400",
        }
    }
}

impl std::str::FromStr for ListTerminalReaderReaderDeviceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTerminalReaderReaderDeviceType::*;
        match s {
            "bbpos_chipper2x" => Ok(BbposChipper2x),
            "bbpos_wisepad3" => Ok(BbposWisepad3),
            "bbpos_wisepos_e" => Ok(BbposWiseposE),
            "simulated_wisepos_e" => Ok(SimulatedWiseposE),
            "stripe_m2" => Ok(StripeM2),
            "verifone_P400" => Ok(VerifoneP400),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for ListTerminalReaderReaderDeviceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for ListTerminalReaderReaderDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTerminalReaderReaderDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTerminalReaderReaderDeviceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// A status filter to filter readers to only offline or online readers.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTerminalReaderReaderStatus {
    Offline,
    Online,
}
impl ListTerminalReaderReaderStatus {
    pub fn as_str(self) -> &'static str {
        use ListTerminalReaderReaderStatus::*;
        match self {
            Offline => "offline",
            Online => "online",
        }
    }
}

impl std::str::FromStr for ListTerminalReaderReaderStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTerminalReaderReaderStatus::*;
        match s {
            "offline" => Ok(Offline),
            "online" => Ok(Online),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for ListTerminalReaderReaderStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for ListTerminalReaderReaderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTerminalReaderReaderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTerminalReaderReaderStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> ListTerminalReaderReader<'a> {
    /// Returns a list of `Reader` objects.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_terminal::TerminalReaderReader>> {
        client.get_query("/terminal/readers", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_terminal::TerminalReaderReader> {
        stripe::ListPaginator::from_params("/terminal/readers", self)
    }
}
impl<'a> stripe::PaginationParams for ListTerminalReaderReader<'a> {}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeleteTerminalReaderReader {}
impl DeleteTerminalReaderReader {
    pub fn new() -> Self {
        Self::default()
    }
}
impl DeleteTerminalReaderReader {
    /// Deletes a `Reader` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        reader: &str,
    ) -> stripe::Response<stripe_terminal::TerminalReaderDeletedReader> {
        client.send_form(&format!("/terminal/readers/{reader}"), self, http_types::Method::Delete)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ProcessPaymentIntentTerminalReaderReader<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// PaymentIntent ID.
    pub payment_intent: &'a str,
    /// Configuration overrides.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_config: Option<ProcessPaymentIntentTerminalReaderReaderProcessConfig>,
}
impl<'a> ProcessPaymentIntentTerminalReaderReader<'a> {
    pub fn new(payment_intent: &'a str) -> Self {
        Self { expand: Default::default(), payment_intent, process_config: Default::default() }
    }
}
/// Configuration overrides.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ProcessPaymentIntentTerminalReaderReaderProcessConfig {
    /// Override showing a tipping selection screen on this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_tipping: Option<bool>,
    /// Tipping configuration for this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<ProcessPaymentIntentTerminalReaderReaderProcessConfigTipping>,
}
impl ProcessPaymentIntentTerminalReaderReaderProcessConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Tipping configuration for this transaction.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ProcessPaymentIntentTerminalReaderReaderProcessConfigTipping {
    /// Amount used to calculate tip suggestions on tipping selection screen for this transaction.
    ///
    /// Must be a positive integer in the smallest currency unit (e.g., 100 cents to represent $1.00 or 100 to represent ¥100, a zero-decimal currency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_eligible: Option<i64>,
}
impl ProcessPaymentIntentTerminalReaderReaderProcessConfigTipping {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ProcessPaymentIntentTerminalReaderReader<'a> {
    /// Initiates a payment flow on a Reader.
    pub fn send(
        &self,
        client: &stripe::Client,
        reader: &str,
    ) -> stripe::Response<stripe_terminal::TerminalReaderReader> {
        client.send_form(
            &format!("/terminal/readers/{reader}/process_payment_intent"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ProcessSetupIntentTerminalReaderReader<'a> {
    /// Customer Consent Collected.
    pub customer_consent_collected: bool,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Configuration overrides.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_config: Option<&'a serde_json::Value>,
    /// SetupIntent ID.
    pub setup_intent: &'a str,
}
impl<'a> ProcessSetupIntentTerminalReaderReader<'a> {
    pub fn new(customer_consent_collected: bool, setup_intent: &'a str) -> Self {
        Self {
            customer_consent_collected,
            expand: Default::default(),
            process_config: Default::default(),
            setup_intent,
        }
    }
}
impl<'a> ProcessSetupIntentTerminalReaderReader<'a> {
    /// Initiates a setup intent flow on a Reader.
    pub fn send(
        &self,
        client: &stripe::Client,
        reader: &str,
    ) -> stripe::Response<stripe_terminal::TerminalReaderReader> {
        client.send_form(
            &format!("/terminal/readers/{reader}/process_setup_intent"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelActionTerminalReaderReader<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CancelActionTerminalReaderReader<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> CancelActionTerminalReaderReader<'a> {
    /// Cancels the current reader action.
    pub fn send(
        &self,
        client: &stripe::Client,
        reader: &str,
    ) -> stripe::Response<stripe_terminal::TerminalReaderReader> {
        client.send_form(
            &format!("/terminal/readers/{reader}/cancel_action"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SetReaderDisplayTerminalReaderReader<'a> {
    /// Cart.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart: Option<SetReaderDisplayTerminalReaderReaderCart<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Type.
    #[serde(rename = "type")]
    pub type_: SetReaderDisplayTerminalReaderReaderType,
}
impl<'a> SetReaderDisplayTerminalReaderReader<'a> {
    pub fn new(type_: SetReaderDisplayTerminalReaderReaderType) -> Self {
        Self { cart: Default::default(), expand: Default::default(), type_ }
    }
}
/// Cart.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SetReaderDisplayTerminalReaderReaderCart<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Array of line items that were purchased.
    pub line_items: &'a [SetReaderDisplayTerminalReaderReaderCartLineItems<'a>],
    /// The amount of tax in cents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<i64>,
    /// Total balance of cart due in cents.
    pub total: i64,
}
impl<'a> SetReaderDisplayTerminalReaderReaderCart<'a> {
    pub fn new(
        currency: stripe_types::Currency,
        line_items: &'a [SetReaderDisplayTerminalReaderReaderCartLineItems<'a>],
        total: i64,
    ) -> Self {
        Self { currency, line_items, tax: Default::default(), total }
    }
}
/// Array of line items that were purchased.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SetReaderDisplayTerminalReaderReaderCartLineItems<'a> {
    /// The price of the item in cents.
    pub amount: i64,
    /// The description or name of the item.
    pub description: &'a str,
    /// The quantity of the line item being purchased.
    pub quantity: u64,
}
impl<'a> SetReaderDisplayTerminalReaderReaderCartLineItems<'a> {
    pub fn new(amount: i64, description: &'a str, quantity: u64) -> Self {
        Self { amount, description, quantity }
    }
}
/// Type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetReaderDisplayTerminalReaderReaderType {
    Cart,
}
impl SetReaderDisplayTerminalReaderReaderType {
    pub fn as_str(self) -> &'static str {
        use SetReaderDisplayTerminalReaderReaderType::*;
        match self {
            Cart => "cart",
        }
    }
}

impl std::str::FromStr for SetReaderDisplayTerminalReaderReaderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetReaderDisplayTerminalReaderReaderType::*;
        match s {
            "cart" => Ok(Cart),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for SetReaderDisplayTerminalReaderReaderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for SetReaderDisplayTerminalReaderReaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetReaderDisplayTerminalReaderReaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SetReaderDisplayTerminalReaderReaderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> SetReaderDisplayTerminalReaderReader<'a> {
    /// Sets reader display to show cart details.
    pub fn send(
        &self,
        client: &stripe::Client,
        reader: &str,
    ) -> stripe::Response<stripe_terminal::TerminalReaderReader> {
        client.send_form(
            &format!("/terminal/readers/{reader}/set_reader_display"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RefundPaymentTerminalReaderReader<'a> {
    /// A positive integer in __cents__ representing how much of this charge to refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// ID of the Charge to refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// ID of the PaymentIntent to refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<&'a str>,
    /// Boolean indicating whether the application fee should be refunded when refunding this charge.
    ///
    /// If a full charge refund is given, the full application fee will be refunded.
    /// Otherwise, the application fee will be refunded in an amount proportional to the amount of the charge refunded.
    /// An application fee can be refunded only by the application that created the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_application_fee: Option<bool>,
    /// Boolean indicating whether the transfer should be reversed when refunding this charge.
    ///
    /// The transfer will be reversed proportionally to the amount being refunded (either the entire or partial amount).
    /// A transfer can be reversed only by the application that created the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_transfer: Option<bool>,
}
impl<'a> RefundPaymentTerminalReaderReader<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RefundPaymentTerminalReaderReader<'a> {
    /// Initiates a refund on a Reader.
    pub fn send(
        &self,
        client: &stripe::Client,
        reader: &str,
    ) -> stripe::Response<stripe_terminal::TerminalReaderReader> {
        client.send_form(
            &format!("/terminal/readers/{reader}/refund_payment"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PresentPaymentMethodTerminalReaderReader<'a> {
    /// Simulated on-reader tip amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tip: Option<i64>,
    /// Simulated data for the card_present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<PresentPaymentMethodTerminalReaderReaderCardPresent<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Simulated data for the interac_present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<PresentPaymentMethodTerminalReaderReaderInteracPresent<'a>>,
    /// Simulated payment type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<PresentPaymentMethodTerminalReaderReaderType>,
}
impl<'a> PresentPaymentMethodTerminalReaderReader<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Simulated data for the card_present payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PresentPaymentMethodTerminalReaderReaderCardPresent<'a> {
    /// The card number, as a string without any separators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<&'a str>,
}
impl<'a> PresentPaymentMethodTerminalReaderReaderCardPresent<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Simulated data for the interac_present payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PresentPaymentMethodTerminalReaderReaderInteracPresent<'a> {
    /// Card Number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<&'a str>,
}
impl<'a> PresentPaymentMethodTerminalReaderReaderInteracPresent<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Simulated payment type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PresentPaymentMethodTerminalReaderReaderType {
    CardPresent,
    InteracPresent,
}
impl PresentPaymentMethodTerminalReaderReaderType {
    pub fn as_str(self) -> &'static str {
        use PresentPaymentMethodTerminalReaderReaderType::*;
        match self {
            CardPresent => "card_present",
            InteracPresent => "interac_present",
        }
    }
}

impl std::str::FromStr for PresentPaymentMethodTerminalReaderReaderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PresentPaymentMethodTerminalReaderReaderType::*;
        match s {
            "card_present" => Ok(CardPresent),
            "interac_present" => Ok(InteracPresent),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for PresentPaymentMethodTerminalReaderReaderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for PresentPaymentMethodTerminalReaderReaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PresentPaymentMethodTerminalReaderReaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PresentPaymentMethodTerminalReaderReaderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> PresentPaymentMethodTerminalReaderReader<'a> {
    /// Presents a payment method on a simulated reader.
    ///
    /// Can be used to simulate accepting a payment, saving a card or refunding a transaction.
    pub fn send(
        &self,
        client: &stripe::Client,
        reader: &str,
    ) -> stripe::Response<stripe_terminal::TerminalReaderReader> {
        client.send_form(
            &format!("/test_helpers/terminal/readers/{reader}/present_payment_method"),
            self,
            http_types::Method::Post,
        )
    }
}
