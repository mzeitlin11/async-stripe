#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AccountCapabilities {
    /// The status of the Canadian pre-authorized debits payments capability of the account, or whether the account can directly process Canadian pre-authorized debits charges.
    pub acss_debit_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Affirm capability of the account, or whether the account can directly process Affirm charges.
    pub affirm_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Afterpay Clearpay capability of the account, or whether the account can directly process Afterpay Clearpay charges.
    pub afterpay_clearpay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the BECS Direct Debit (AU) payments capability of the account, or whether the account can directly process BECS Direct Debit (AU) charges.
    pub au_becs_debit_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Bacs Direct Debits payments capability of the account, or whether the account can directly process Bacs Direct Debits charges.
    pub bacs_debit_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Bancontact payments capability of the account, or whether the account can directly process Bancontact charges.
    pub bancontact_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the customer_balance payments capability of the account, or whether the account can directly process customer_balance charges.
    pub bank_transfer_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the blik payments capability of the account, or whether the account can directly process blik charges.
    pub blik_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the boleto payments capability of the account, or whether the account can directly process boleto charges.
    pub boleto_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the card issuing capability of the account, or whether you can use Issuing to distribute funds on cards.
    pub card_issuing: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the card payments capability of the account, or whether the account can directly process credit and debit card charges.
    pub card_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Cartes Bancaires payments capability of the account, or whether the account can directly process Cartes Bancaires card charges in EUR currency.
    pub cartes_bancaires_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Cash App Pay capability of the account, or whether the account can directly process Cash App Pay payments.
    pub cashapp_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the EPS payments capability of the account, or whether the account can directly process EPS charges.
    pub eps_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the FPX payments capability of the account, or whether the account can directly process FPX charges.
    pub fpx_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the giropay payments capability of the account, or whether the account can directly process giropay charges.
    pub giropay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the GrabPay payments capability of the account, or whether the account can directly process GrabPay charges.
    pub grabpay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the iDEAL payments capability of the account, or whether the account can directly process iDEAL charges.
    pub ideal_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the india_international_payments capability of the account, or whether the account can process international charges (non INR) in India.
    pub india_international_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the JCB payments capability of the account, or whether the account (Japan only) can directly process JCB credit card charges in JPY currency.
    pub jcb_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Klarna payments capability of the account, or whether the account can directly process Klarna charges.
    pub klarna_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the konbini payments capability of the account, or whether the account can directly process konbini charges.
    pub konbini_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the legacy payments capability of the account.
    pub legacy_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the link_payments capability of the account, or whether the account can directly process Link charges.
    pub link_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the OXXO payments capability of the account, or whether the account can directly process OXXO charges.
    pub oxxo_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the P24 payments capability of the account, or whether the account can directly process P24 charges.
    pub p24_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the paynow payments capability of the account, or whether the account can directly process paynow charges.
    pub paynow_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the promptpay payments capability of the account, or whether the account can directly process promptpay charges.
    pub promptpay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the RevolutPay capability of the account, or whether the account can directly process RevolutPay payments.
    pub revolut_pay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the SEPA Direct Debits payments capability of the account, or whether the account can directly process SEPA Direct Debits charges.
    pub sepa_debit_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Sofort payments capability of the account, or whether the account can directly process Sofort charges.
    pub sofort_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the tax reporting 1099-K (US) capability of the account.
    pub tax_reporting_us_1099_k: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the tax reporting 1099-MISC (US) capability of the account.
    pub tax_reporting_us_1099_misc: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the transfers capability of the account, or whether your platform can transfer funds to the account.
    pub transfers: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the banking capability, or whether the account can have bank accounts.
    pub treasury: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the US bank account ACH payments capability of the account, or whether the account can directly process US bank account charges.
    pub us_bank_account_ach_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Zip capability of the account, or whether the account can directly process Zip charges.
    pub zip_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
}
#[cfg(feature = "min-ser")]
pub struct AccountCapabilitiesBuilder {
    acss_debit_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    affirm_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    afterpay_clearpay_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    au_becs_debit_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    bacs_debit_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    bancontact_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    bank_transfer_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    blik_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    boleto_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    card_issuing: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    card_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    cartes_bancaires_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    cashapp_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    eps_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    fpx_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    giropay_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    grabpay_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    ideal_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    india_international_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    jcb_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    klarna_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    konbini_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    legacy_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    link_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    oxxo_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    p24_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    paynow_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    promptpay_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    revolut_pay_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    sepa_debit_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    sofort_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    tax_reporting_us_1099_k: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    tax_reporting_us_1099_misc: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    transfers: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    treasury: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    us_bank_account_ach_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    zip_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountCapabilities {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountCapabilities>,
        builder: AccountCapabilitiesBuilder,
    }

    impl Visitor for Place<AccountCapabilities> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: AccountCapabilitiesBuilder::deser_default() }))
        }
    }

    impl MapBuilder for AccountCapabilitiesBuilder {
        type Out = AccountCapabilities;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "acss_debit_payments" => Ok(Deserialize::begin(&mut self.acss_debit_payments)),
                "affirm_payments" => Ok(Deserialize::begin(&mut self.affirm_payments)),
                "afterpay_clearpay_payments" => Ok(Deserialize::begin(&mut self.afterpay_clearpay_payments)),
                "au_becs_debit_payments" => Ok(Deserialize::begin(&mut self.au_becs_debit_payments)),
                "bacs_debit_payments" => Ok(Deserialize::begin(&mut self.bacs_debit_payments)),
                "bancontact_payments" => Ok(Deserialize::begin(&mut self.bancontact_payments)),
                "bank_transfer_payments" => Ok(Deserialize::begin(&mut self.bank_transfer_payments)),
                "blik_payments" => Ok(Deserialize::begin(&mut self.blik_payments)),
                "boleto_payments" => Ok(Deserialize::begin(&mut self.boleto_payments)),
                "card_issuing" => Ok(Deserialize::begin(&mut self.card_issuing)),
                "card_payments" => Ok(Deserialize::begin(&mut self.card_payments)),
                "cartes_bancaires_payments" => Ok(Deserialize::begin(&mut self.cartes_bancaires_payments)),
                "cashapp_payments" => Ok(Deserialize::begin(&mut self.cashapp_payments)),
                "eps_payments" => Ok(Deserialize::begin(&mut self.eps_payments)),
                "fpx_payments" => Ok(Deserialize::begin(&mut self.fpx_payments)),
                "giropay_payments" => Ok(Deserialize::begin(&mut self.giropay_payments)),
                "grabpay_payments" => Ok(Deserialize::begin(&mut self.grabpay_payments)),
                "ideal_payments" => Ok(Deserialize::begin(&mut self.ideal_payments)),
                "india_international_payments" => Ok(Deserialize::begin(&mut self.india_international_payments)),
                "jcb_payments" => Ok(Deserialize::begin(&mut self.jcb_payments)),
                "klarna_payments" => Ok(Deserialize::begin(&mut self.klarna_payments)),
                "konbini_payments" => Ok(Deserialize::begin(&mut self.konbini_payments)),
                "legacy_payments" => Ok(Deserialize::begin(&mut self.legacy_payments)),
                "link_payments" => Ok(Deserialize::begin(&mut self.link_payments)),
                "oxxo_payments" => Ok(Deserialize::begin(&mut self.oxxo_payments)),
                "p24_payments" => Ok(Deserialize::begin(&mut self.p24_payments)),
                "paynow_payments" => Ok(Deserialize::begin(&mut self.paynow_payments)),
                "promptpay_payments" => Ok(Deserialize::begin(&mut self.promptpay_payments)),
                "revolut_pay_payments" => Ok(Deserialize::begin(&mut self.revolut_pay_payments)),
                "sepa_debit_payments" => Ok(Deserialize::begin(&mut self.sepa_debit_payments)),
                "sofort_payments" => Ok(Deserialize::begin(&mut self.sofort_payments)),
                "tax_reporting_us_1099_k" => Ok(Deserialize::begin(&mut self.tax_reporting_us_1099_k)),
                "tax_reporting_us_1099_misc" => Ok(Deserialize::begin(&mut self.tax_reporting_us_1099_misc)),
                "transfers" => Ok(Deserialize::begin(&mut self.transfers)),
                "treasury" => Ok(Deserialize::begin(&mut self.treasury)),
                "us_bank_account_ach_payments" => Ok(Deserialize::begin(&mut self.us_bank_account_ach_payments)),
                "zip_payments" => Ok(Deserialize::begin(&mut self.zip_payments)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                acss_debit_payments: Deserialize::default(),
                affirm_payments: Deserialize::default(),
                afterpay_clearpay_payments: Deserialize::default(),
                au_becs_debit_payments: Deserialize::default(),
                bacs_debit_payments: Deserialize::default(),
                bancontact_payments: Deserialize::default(),
                bank_transfer_payments: Deserialize::default(),
                blik_payments: Deserialize::default(),
                boleto_payments: Deserialize::default(),
                card_issuing: Deserialize::default(),
                card_payments: Deserialize::default(),
                cartes_bancaires_payments: Deserialize::default(),
                cashapp_payments: Deserialize::default(),
                eps_payments: Deserialize::default(),
                fpx_payments: Deserialize::default(),
                giropay_payments: Deserialize::default(),
                grabpay_payments: Deserialize::default(),
                ideal_payments: Deserialize::default(),
                india_international_payments: Deserialize::default(),
                jcb_payments: Deserialize::default(),
                klarna_payments: Deserialize::default(),
                konbini_payments: Deserialize::default(),
                legacy_payments: Deserialize::default(),
                link_payments: Deserialize::default(),
                oxxo_payments: Deserialize::default(),
                p24_payments: Deserialize::default(),
                paynow_payments: Deserialize::default(),
                promptpay_payments: Deserialize::default(),
                revolut_pay_payments: Deserialize::default(),
                sepa_debit_payments: Deserialize::default(),
                sofort_payments: Deserialize::default(),
                tax_reporting_us_1099_k: Deserialize::default(),
                tax_reporting_us_1099_misc: Deserialize::default(),
                transfers: Deserialize::default(),
                treasury: Deserialize::default(),
                us_bank_account_ach_payments: Deserialize::default(),
                zip_payments: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let acss_debit_payments = self.acss_debit_payments.take()?;
            let affirm_payments = self.affirm_payments.take()?;
            let afterpay_clearpay_payments = self.afterpay_clearpay_payments.take()?;
            let au_becs_debit_payments = self.au_becs_debit_payments.take()?;
            let bacs_debit_payments = self.bacs_debit_payments.take()?;
            let bancontact_payments = self.bancontact_payments.take()?;
            let bank_transfer_payments = self.bank_transfer_payments.take()?;
            let blik_payments = self.blik_payments.take()?;
            let boleto_payments = self.boleto_payments.take()?;
            let card_issuing = self.card_issuing.take()?;
            let card_payments = self.card_payments.take()?;
            let cartes_bancaires_payments = self.cartes_bancaires_payments.take()?;
            let cashapp_payments = self.cashapp_payments.take()?;
            let eps_payments = self.eps_payments.take()?;
            let fpx_payments = self.fpx_payments.take()?;
            let giropay_payments = self.giropay_payments.take()?;
            let grabpay_payments = self.grabpay_payments.take()?;
            let ideal_payments = self.ideal_payments.take()?;
            let india_international_payments = self.india_international_payments.take()?;
            let jcb_payments = self.jcb_payments.take()?;
            let klarna_payments = self.klarna_payments.take()?;
            let konbini_payments = self.konbini_payments.take()?;
            let legacy_payments = self.legacy_payments.take()?;
            let link_payments = self.link_payments.take()?;
            let oxxo_payments = self.oxxo_payments.take()?;
            let p24_payments = self.p24_payments.take()?;
            let paynow_payments = self.paynow_payments.take()?;
            let promptpay_payments = self.promptpay_payments.take()?;
            let revolut_pay_payments = self.revolut_pay_payments.take()?;
            let sepa_debit_payments = self.sepa_debit_payments.take()?;
            let sofort_payments = self.sofort_payments.take()?;
            let tax_reporting_us_1099_k = self.tax_reporting_us_1099_k.take()?;
            let tax_reporting_us_1099_misc = self.tax_reporting_us_1099_misc.take()?;
            let transfers = self.transfers.take()?;
            let treasury = self.treasury.take()?;
            let us_bank_account_ach_payments = self.us_bank_account_ach_payments.take()?;
            let zip_payments = self.zip_payments.take()?;

            Some(Self::Out {
                acss_debit_payments,
                affirm_payments,
                afterpay_clearpay_payments,
                au_becs_debit_payments,
                bacs_debit_payments,
                bancontact_payments,
                bank_transfer_payments,
                blik_payments,
                boleto_payments,
                card_issuing,
                card_payments,
                cartes_bancaires_payments,
                cashapp_payments,
                eps_payments,
                fpx_payments,
                giropay_payments,
                grabpay_payments,
                ideal_payments,
                india_international_payments,
                jcb_payments,
                klarna_payments,
                konbini_payments,
                legacy_payments,
                link_payments,
                oxxo_payments,
                p24_payments,
                paynow_payments,
                promptpay_payments,
                revolut_pay_payments,
                sepa_debit_payments,
                sofort_payments,
                tax_reporting_us_1099_k,
                tax_reporting_us_1099_misc,
                transfers,
                treasury,
                us_bank_account_ach_payments,
                zip_payments,
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

    impl ObjectDeser for AccountCapabilities {
        type Builder = AccountCapabilitiesBuilder;
    }
};
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesStatus {
    Active,
    Inactive,
    Pending,
}
impl AccountCapabilitiesStatus {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesStatus::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesStatus::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}
impl AsRef<str> for AccountCapabilitiesStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl std::fmt::Display for AccountCapabilitiesStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for AccountCapabilitiesStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AccountCapabilitiesStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<AccountCapabilitiesStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountCapabilitiesStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
