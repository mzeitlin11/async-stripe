#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SubscriptionSchedulesResourceDefaultSettings {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account during this phase of the schedule.
    pub application_fee_percent: Option<f64>,
    pub automatic_tax: Option<stripe_shared::SubscriptionSchedulesResourceDefaultSettingsAutomaticTax>,
    /// Possible values are `phase_start` or `automatic`.
    /// If `phase_start` then billing cycle anchor of the subscription is set to the start of the phase when entering the phase.
    /// If `automatic` then the billing cycle anchor is automatically modified as needed when entering the phase.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    pub billing_cycle_anchor: SubscriptionSchedulesResourceDefaultSettingsBillingCycleAnchor,
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    pub billing_thresholds: Option<stripe_shared::SubscriptionBillingThresholds>,
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    pub collection_method: Option<SubscriptionSchedulesResourceDefaultSettingsCollectionMethod>,
    /// ID of the default payment method for the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    pub default_payment_method: Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>,
    /// Subscription description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    pub description: Option<String>,
    /// The subscription schedule's default invoice settings.
    pub invoice_settings: Option<stripe_shared::InvoiceSettingSubscriptionScheduleSetting>,
    /// The account (if any) the charge was made on behalf of for charges associated with the schedule's subscription.
    /// See the Connect documentation for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// The account (if any) the associated subscription's payments will be attributed to for tax reporting, and where funds from each payment will be transferred to for each of the subscription's invoices.
    pub transfer_data: Option<stripe_shared::SubscriptionTransferData>,
}
#[cfg(feature = "min-ser")]
pub struct SubscriptionSchedulesResourceDefaultSettingsBuilder {
    application_fee_percent: Option<Option<f64>>,
    automatic_tax: Option<Option<stripe_shared::SubscriptionSchedulesResourceDefaultSettingsAutomaticTax>>,
    billing_cycle_anchor: Option<SubscriptionSchedulesResourceDefaultSettingsBillingCycleAnchor>,
    billing_thresholds: Option<Option<stripe_shared::SubscriptionBillingThresholds>>,
    collection_method: Option<Option<SubscriptionSchedulesResourceDefaultSettingsCollectionMethod>>,
    default_payment_method: Option<Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>>,
    description: Option<Option<String>>,
    invoice_settings: Option<Option<stripe_shared::InvoiceSettingSubscriptionScheduleSetting>>,
    on_behalf_of: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    transfer_data: Option<Option<stripe_shared::SubscriptionTransferData>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SubscriptionSchedulesResourceDefaultSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionSchedulesResourceDefaultSettings>,
        builder: SubscriptionSchedulesResourceDefaultSettingsBuilder,
    }

    impl Visitor for Place<SubscriptionSchedulesResourceDefaultSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SubscriptionSchedulesResourceDefaultSettingsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SubscriptionSchedulesResourceDefaultSettingsBuilder {
        type Out = SubscriptionSchedulesResourceDefaultSettings;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "application_fee_percent" => Ok(Deserialize::begin(&mut self.application_fee_percent)),
                "automatic_tax" => Ok(Deserialize::begin(&mut self.automatic_tax)),
                "billing_cycle_anchor" => Ok(Deserialize::begin(&mut self.billing_cycle_anchor)),
                "billing_thresholds" => Ok(Deserialize::begin(&mut self.billing_thresholds)),
                "collection_method" => Ok(Deserialize::begin(&mut self.collection_method)),
                "default_payment_method" => Ok(Deserialize::begin(&mut self.default_payment_method)),
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "invoice_settings" => Ok(Deserialize::begin(&mut self.invoice_settings)),
                "on_behalf_of" => Ok(Deserialize::begin(&mut self.on_behalf_of)),
                "transfer_data" => Ok(Deserialize::begin(&mut self.transfer_data)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                application_fee_percent: Deserialize::default(),
                automatic_tax: Deserialize::default(),
                billing_cycle_anchor: Deserialize::default(),
                billing_thresholds: Deserialize::default(),
                collection_method: Deserialize::default(),
                default_payment_method: Deserialize::default(),
                description: Deserialize::default(),
                invoice_settings: Deserialize::default(),
                on_behalf_of: Deserialize::default(),
                transfer_data: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let application_fee_percent = self.application_fee_percent.take()?;
            let automatic_tax = self.automatic_tax.take()?;
            let billing_cycle_anchor = self.billing_cycle_anchor.take()?;
            let billing_thresholds = self.billing_thresholds.take()?;
            let collection_method = self.collection_method.take()?;
            let default_payment_method = self.default_payment_method.take()?;
            let description = self.description.take()?;
            let invoice_settings = self.invoice_settings.take()?;
            let on_behalf_of = self.on_behalf_of.take()?;
            let transfer_data = self.transfer_data.take()?;

            Some(Self::Out {
                application_fee_percent,
                automatic_tax,
                billing_cycle_anchor,
                billing_thresholds,
                collection_method,
                default_payment_method,
                description,
                invoice_settings,
                on_behalf_of,
                transfer_data,
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

    impl ObjectDeser for SubscriptionSchedulesResourceDefaultSettings {
        type Builder = SubscriptionSchedulesResourceDefaultSettingsBuilder;
    }
};
/// Possible values are `phase_start` or `automatic`.
/// If `phase_start` then billing cycle anchor of the subscription is set to the start of the phase when entering the phase.
/// If `automatic` then the billing cycle anchor is automatically modified as needed when entering the phase.
/// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionSchedulesResourceDefaultSettingsBillingCycleAnchor {
    Automatic,
    PhaseStart,
}
impl SubscriptionSchedulesResourceDefaultSettingsBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        use SubscriptionSchedulesResourceDefaultSettingsBillingCycleAnchor::*;
        match self {
            Automatic => "automatic",
            PhaseStart => "phase_start",
        }
    }
}

impl std::str::FromStr for SubscriptionSchedulesResourceDefaultSettingsBillingCycleAnchor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionSchedulesResourceDefaultSettingsBillingCycleAnchor::*;
        match s {
            "automatic" => Ok(Automatic),
            "phase_start" => Ok(PhaseStart),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for SubscriptionSchedulesResourceDefaultSettingsBillingCycleAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for SubscriptionSchedulesResourceDefaultSettingsBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionSchedulesResourceDefaultSettingsBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SubscriptionSchedulesResourceDefaultSettingsBillingCycleAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionSchedulesResourceDefaultSettingsBillingCycleAnchor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SubscriptionSchedulesResourceDefaultSettingsBillingCycleAnchor"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SubscriptionSchedulesResourceDefaultSettingsBillingCycleAnchor {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SubscriptionSchedulesResourceDefaultSettingsBillingCycleAnchor> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SubscriptionSchedulesResourceDefaultSettingsBillingCycleAnchor::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Either `charge_automatically`, or `send_invoice`.
/// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionSchedulesResourceDefaultSettingsCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}
impl SubscriptionSchedulesResourceDefaultSettingsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use SubscriptionSchedulesResourceDefaultSettingsCollectionMethod::*;
        match self {
            ChargeAutomatically => "charge_automatically",
            SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for SubscriptionSchedulesResourceDefaultSettingsCollectionMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionSchedulesResourceDefaultSettingsCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for SubscriptionSchedulesResourceDefaultSettingsCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for SubscriptionSchedulesResourceDefaultSettingsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionSchedulesResourceDefaultSettingsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SubscriptionSchedulesResourceDefaultSettingsCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionSchedulesResourceDefaultSettingsCollectionMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SubscriptionSchedulesResourceDefaultSettingsCollectionMethod"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SubscriptionSchedulesResourceDefaultSettingsCollectionMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SubscriptionSchedulesResourceDefaultSettingsCollectionMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SubscriptionSchedulesResourceDefaultSettingsCollectionMethod::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
