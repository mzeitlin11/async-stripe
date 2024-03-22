/// Issue a credit note to adjust an invoice's amount after the invoice is finalized.
///
/// Related guide: [Credit notes](https://stripe.com/docs/billing/invoices/credit-notes)
///
/// For more details see <<https://stripe.com/docs/api/credit_notes/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CreditNote {
    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note, including tax.
    pub amount: i64,
    /// This is the sum of all the shipping amounts.
    pub amount_shipping: i64,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// ID of the customer.
    pub customer: stripe_types::Expandable<stripe_shared::Customer>,
    /// Customer balance transaction related to this credit note.
    pub customer_balance_transaction: Option<stripe_types::Expandable<stripe_shared::CustomerBalanceTransaction>>,
    /// The integer amount in cents (or local equivalent) representing the total amount of discount that was credited.
    pub discount_amount: i64,
    /// The aggregate amounts calculated per discount for all line items.
    pub discount_amounts: Vec<stripe_shared::DiscountsResourceDiscountAmount>,
    /// The date when this credit note is in effect.
    /// Same as `created` unless overwritten.
    /// When defined, this value replaces the system-generated 'Date of issue' printed on the credit note PDF.
    pub effective_at: Option<stripe_types::Timestamp>,
    /// Unique identifier for the object.
    pub id: stripe_shared::CreditNoteId,
    /// ID of the invoice.
    pub invoice: stripe_types::Expandable<stripe_shared::Invoice>,
    /// Line items that make up the credit note
    pub lines: stripe_types::List<stripe_shared::CreditNoteLineItem>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Customer-facing text that appears on the credit note PDF.
    pub memo: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// A unique number that identifies this particular credit note and appears on the PDF of the credit note and its associated invoice.
    pub number: String,
    /// Amount that was credited outside of Stripe.
    pub out_of_band_amount: Option<i64>,
    /// The link to download the PDF of the credit note.
    pub pdf: String,
    /// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
    pub reason: Option<stripe_shared::CreditNoteReason>,
    /// Refund related to this credit note.
    pub refund: Option<stripe_types::Expandable<stripe_shared::Refund>>,
    /// The details of the cost of shipping, including the ShippingRate applied to the invoice.
    pub shipping_cost: Option<stripe_shared::InvoicesShippingCost>,
    /// Status of this credit note, one of `issued` or `void`.
    /// Learn more about [voiding credit notes](https://stripe.com/docs/billing/invoices/credit-notes#voiding).
    pub status: CreditNoteStatus,
    /// The integer amount in cents (or local equivalent) representing the amount of the credit note, excluding exclusive tax and invoice level discounts.
    pub subtotal: i64,
    /// The integer amount in cents (or local equivalent) representing the amount of the credit note, excluding all tax and invoice level discounts.
    pub subtotal_excluding_tax: Option<i64>,
    /// The aggregate amounts calculated per tax rate for all line items.
    pub tax_amounts: Vec<stripe_shared::CreditNoteTaxAmount>,
    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note, including tax and all discount.
    pub total: i64,
    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note, excluding tax, but including discounts.
    pub total_excluding_tax: Option<i64>,
    /// Type of this credit note, one of `pre_payment` or `post_payment`.
    /// A `pre_payment` credit note means it was issued when the invoice was open.
    /// A `post_payment` credit note means it was issued when the invoice was paid.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: CreditNoteType,
    /// The time that the credit note was voided.
    pub voided_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
pub struct CreditNoteBuilder {
    amount: Option<i64>,
    amount_shipping: Option<i64>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    customer_balance_transaction: Option<Option<stripe_types::Expandable<stripe_shared::CustomerBalanceTransaction>>>,
    discount_amount: Option<i64>,
    discount_amounts: Option<Vec<stripe_shared::DiscountsResourceDiscountAmount>>,
    effective_at: Option<Option<stripe_types::Timestamp>>,
    id: Option<stripe_shared::CreditNoteId>,
    invoice: Option<stripe_types::Expandable<stripe_shared::Invoice>>,
    lines: Option<stripe_types::List<stripe_shared::CreditNoteLineItem>>,
    livemode: Option<bool>,
    memo: Option<Option<String>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    number: Option<String>,
    out_of_band_amount: Option<Option<i64>>,
    pdf: Option<String>,
    reason: Option<Option<stripe_shared::CreditNoteReason>>,
    refund: Option<Option<stripe_types::Expandable<stripe_shared::Refund>>>,
    shipping_cost: Option<Option<stripe_shared::InvoicesShippingCost>>,
    status: Option<CreditNoteStatus>,
    subtotal: Option<i64>,
    subtotal_excluding_tax: Option<Option<i64>>,
    tax_amounts: Option<Vec<stripe_shared::CreditNoteTaxAmount>>,
    total: Option<i64>,
    total_excluding_tax: Option<Option<i64>>,
    type_: Option<CreditNoteType>,
    voided_at: Option<Option<stripe_types::Timestamp>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CreditNote {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CreditNote>,
        builder: CreditNoteBuilder,
    }

    impl Visitor for Place<CreditNote> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CreditNoteBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CreditNoteBuilder {
        type Out = CreditNote;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount" => Ok(Deserialize::begin(&mut self.amount)),
                "amount_shipping" => Ok(Deserialize::begin(&mut self.amount_shipping)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "currency" => Ok(Deserialize::begin(&mut self.currency)),
                "customer" => Ok(Deserialize::begin(&mut self.customer)),
                "customer_balance_transaction" => Ok(Deserialize::begin(&mut self.customer_balance_transaction)),
                "discount_amount" => Ok(Deserialize::begin(&mut self.discount_amount)),
                "discount_amounts" => Ok(Deserialize::begin(&mut self.discount_amounts)),
                "effective_at" => Ok(Deserialize::begin(&mut self.effective_at)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "invoice" => Ok(Deserialize::begin(&mut self.invoice)),
                "lines" => Ok(Deserialize::begin(&mut self.lines)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "memo" => Ok(Deserialize::begin(&mut self.memo)),
                "metadata" => Ok(Deserialize::begin(&mut self.metadata)),
                "number" => Ok(Deserialize::begin(&mut self.number)),
                "out_of_band_amount" => Ok(Deserialize::begin(&mut self.out_of_band_amount)),
                "pdf" => Ok(Deserialize::begin(&mut self.pdf)),
                "reason" => Ok(Deserialize::begin(&mut self.reason)),
                "refund" => Ok(Deserialize::begin(&mut self.refund)),
                "shipping_cost" => Ok(Deserialize::begin(&mut self.shipping_cost)),
                "status" => Ok(Deserialize::begin(&mut self.status)),
                "subtotal" => Ok(Deserialize::begin(&mut self.subtotal)),
                "subtotal_excluding_tax" => Ok(Deserialize::begin(&mut self.subtotal_excluding_tax)),
                "tax_amounts" => Ok(Deserialize::begin(&mut self.tax_amounts)),
                "total" => Ok(Deserialize::begin(&mut self.total)),
                "total_excluding_tax" => Ok(Deserialize::begin(&mut self.total_excluding_tax)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),
                "voided_at" => Ok(Deserialize::begin(&mut self.voided_at)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                amount_shipping: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                customer: Deserialize::default(),
                customer_balance_transaction: Deserialize::default(),
                discount_amount: Deserialize::default(),
                discount_amounts: Deserialize::default(),
                effective_at: Deserialize::default(),
                id: Deserialize::default(),
                invoice: Deserialize::default(),
                lines: Deserialize::default(),
                livemode: Deserialize::default(),
                memo: Deserialize::default(),
                metadata: Deserialize::default(),
                number: Deserialize::default(),
                out_of_band_amount: Deserialize::default(),
                pdf: Deserialize::default(),
                reason: Deserialize::default(),
                refund: Deserialize::default(),
                shipping_cost: Deserialize::default(),
                status: Deserialize::default(),
                subtotal: Deserialize::default(),
                subtotal_excluding_tax: Deserialize::default(),
                tax_amounts: Deserialize::default(),
                total: Deserialize::default(),
                total_excluding_tax: Deserialize::default(),
                type_: Deserialize::default(),
                voided_at: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount = self.amount.take()?;
            let amount_shipping = self.amount_shipping.take()?;
            let created = self.created.take()?;
            let currency = self.currency.take()?;
            let customer = self.customer.take()?;
            let customer_balance_transaction = self.customer_balance_transaction.take()?;
            let discount_amount = self.discount_amount.take()?;
            let discount_amounts = self.discount_amounts.take()?;
            let effective_at = self.effective_at.take()?;
            let id = self.id.take()?;
            let invoice = self.invoice.take()?;
            let lines = self.lines.take()?;
            let livemode = self.livemode.take()?;
            let memo = self.memo.take()?;
            let metadata = self.metadata.take()?;
            let number = self.number.take()?;
            let out_of_band_amount = self.out_of_band_amount.take()?;
            let pdf = self.pdf.take()?;
            let reason = self.reason.take()?;
            let refund = self.refund.take()?;
            let shipping_cost = self.shipping_cost.take()?;
            let status = self.status.take()?;
            let subtotal = self.subtotal.take()?;
            let subtotal_excluding_tax = self.subtotal_excluding_tax.take()?;
            let tax_amounts = self.tax_amounts.take()?;
            let total = self.total.take()?;
            let total_excluding_tax = self.total_excluding_tax.take()?;
            let type_ = self.type_.take()?;
            let voided_at = self.voided_at.take()?;

            Some(Self::Out {
                amount,
                amount_shipping,
                created,
                currency,
                customer,
                customer_balance_transaction,
                discount_amount,
                discount_amounts,
                effective_at,
                id,
                invoice,
                lines,
                livemode,
                memo,
                metadata,
                number,
                out_of_band_amount,
                pdf,
                reason,
                refund,
                shipping_cost,
                status,
                subtotal,
                subtotal_excluding_tax,
                tax_amounts,
                total,
                total_excluding_tax,
                type_,
                voided_at,
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

    impl ObjectDeser for CreditNote {
        type Builder = CreditNoteBuilder;
    }
};
/// Status of this credit note, one of `issued` or `void`.
/// Learn more about [voiding credit notes](https://stripe.com/docs/billing/invoices/credit-notes#voiding).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreditNoteStatus {
    Issued,
    Void,
}
impl CreditNoteStatus {
    pub fn as_str(self) -> &'static str {
        use CreditNoteStatus::*;
        match self {
            Issued => "issued",
            Void => "void",
        }
    }
}

impl std::str::FromStr for CreditNoteStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreditNoteStatus::*;
        match s {
            "issued" => Ok(Issued),
            "void" => Ok(Void),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreditNoteStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreditNoteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreditNoteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreditNoteStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreditNoteStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreditNoteStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CreditNoteStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CreditNoteStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreditNoteStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Type of this credit note, one of `pre_payment` or `post_payment`.
/// A `pre_payment` credit note means it was issued when the invoice was open.
/// A `post_payment` credit note means it was issued when the invoice was paid.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreditNoteType {
    PostPayment,
    PrePayment,
}
impl CreditNoteType {
    pub fn as_str(self) -> &'static str {
        use CreditNoteType::*;
        match self {
            PostPayment => "post_payment",
            PrePayment => "pre_payment",
        }
    }
}

impl std::str::FromStr for CreditNoteType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreditNoteType::*;
        match s {
            "post_payment" => Ok(PostPayment),
            "pre_payment" => Ok(PrePayment),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreditNoteType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreditNoteType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreditNoteType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreditNoteType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreditNoteType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreditNoteType"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CreditNoteType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CreditNoteType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreditNoteType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for CreditNote {
    type Id = stripe_shared::CreditNoteId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(CreditNoteId, "cn_");
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreditNoteReason {
    Duplicate,
    Fraudulent,
    OrderChange,
    ProductUnsatisfactory,
}
impl CreditNoteReason {
    pub fn as_str(self) -> &'static str {
        use CreditNoteReason::*;
        match self {
            Duplicate => "duplicate",
            Fraudulent => "fraudulent",
            OrderChange => "order_change",
            ProductUnsatisfactory => "product_unsatisfactory",
        }
    }
}

impl std::str::FromStr for CreditNoteReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreditNoteReason::*;
        match s {
            "duplicate" => Ok(Duplicate),
            "fraudulent" => Ok(Fraudulent),
            "order_change" => Ok(OrderChange),
            "product_unsatisfactory" => Ok(ProductUnsatisfactory),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for CreditNoteReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for CreditNoteReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreditNoteReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreditNoteReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreditNoteReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreditNoteReason"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CreditNoteReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CreditNoteReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreditNoteReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
