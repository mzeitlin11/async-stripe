// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::params::Object;

/// The resource representing a Stripe "cash_balance".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CashBalance {
    /// A hash of all cash balances available to this customer.
    ///
    /// You cannot delete a customer with any cash balances, even if the balance is 0.
    /// Amounts are represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<i64>,

    /// The ID of the customer whose cash balance this object represents.
    pub customer: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    pub settings: CustomerBalanceCustomerBalanceSettings,
}

impl Object for CashBalance {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "cash_balance"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerBalanceCustomerBalanceSettings {
    /// The configuration for how funds that land in the customer cash balance are reconciled.
    pub reconciliation_mode: CustomerBalanceCustomerBalanceSettingsReconciliationMode,
}

/// An enum representing the possible values of an `CustomerBalanceCustomerBalanceSettings`'s `reconciliation_mode` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    Automatic,
    Manual,
}

impl CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    pub fn as_str(self) -> &'static str {
        match self {
            CustomerBalanceCustomerBalanceSettingsReconciliationMode::Automatic => "automatic",
            CustomerBalanceCustomerBalanceSettingsReconciliationMode::Manual => "manual",
        }
    }
}

impl AsRef<str> for CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    fn default() -> Self {
        Self::Automatic
    }
}
