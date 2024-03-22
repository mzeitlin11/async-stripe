/// PaymentMethodConfigurations control which payment methods are displayed to your customers when you don't explicitly specify payment method types.
/// You can have multiple configurations with different sets of payment methods for different scenarios.
///
/// There are two types of PaymentMethodConfigurations.
/// Which is used depends on the [charge type](https://stripe.com/docs/connect/charges):.
///
/// **Direct** configurations apply to payments created on your account, including Connect destination charges, Connect separate charges and transfers, and payments not involving Connect.
///
/// **Child** configurations apply to payments created on your connected accounts using direct charges, and charges with the on_behalf_of parameter.
///
/// Child configurations have a `parent` that sets default values and controls which settings connected accounts may override.
/// You can specify a parent ID at payment time, and Stripe will automatically resolve the connected account’s associated child configuration.
/// Parent configurations are [managed in the dashboard](https://dashboard.stripe.com/settings/payment_methods/connected_accounts) and are not available in this API.
///
/// Related guides:
/// - [Payment Method Configurations API](https://stripe.com/docs/connect/payment-method-configurations).
/// - [Multiple configurations on dynamic payment methods](https://stripe.com/docs/payments/multiple-payment-method-configs).
/// - [Multiple configurations for your Connect accounts](https://stripe.com/docs/connect/multiple-payment-method-configurations).
///
/// For more details see <<https://stripe.com/docs/api/payment_method_configurations/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodConfiguration {
    pub acss_debit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// Whether the configuration can be used for new payments.
    pub active: bool,
    pub affirm: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub afterpay_clearpay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub alipay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub apple_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// For child configs, the Connect application associated with the configuration.
    pub application: Option<String>,
    pub au_becs_debit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub bacs_debit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub bancontact: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub blik: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub boleto: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub card: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub cartes_bancaires: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub cashapp: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub eps: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub fpx: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub giropay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub google_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub grabpay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// Unique identifier for the object.
    pub id: stripe_payment::PaymentMethodConfigurationId,
    pub id_bank_transfer: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub ideal: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// The default configuration is used whenever a payment method configuration is not specified.
    pub is_default: bool,
    pub jcb: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub klarna: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub konbini: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub link: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    pub multibanco: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// The configuration's name.
    pub name: String,
    pub netbanking: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub oxxo: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub p24: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// For child configs, the configuration's parent configuration.
    pub parent: Option<String>,
    pub pay_by_bank: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub paynow: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub paypal: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub promptpay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub sepa_debit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub sofort: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub upi: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub us_bank_account: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub wechat_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodConfigurationBuilder {
    acss_debit: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    active: Option<bool>,
    affirm: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    afterpay_clearpay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    alipay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    apple_pay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    application: Option<Option<String>>,
    au_becs_debit: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    bacs_debit: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    bancontact: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    blik: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    boleto: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    card: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    cartes_bancaires: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    cashapp: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    eps: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    fpx: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    giropay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    google_pay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    grabpay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    id: Option<stripe_payment::PaymentMethodConfigurationId>,
    id_bank_transfer: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    ideal: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    is_default: Option<bool>,
    jcb: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    klarna: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    konbini: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    link: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    livemode: Option<bool>,
    multibanco: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    name: Option<String>,
    netbanking: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    oxxo: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    p24: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    parent: Option<Option<String>>,
    pay_by_bank: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    paynow: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    paypal: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    promptpay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    sepa_debit: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    sofort: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    upi: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    us_bank_account: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    wechat_pay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodConfiguration {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodConfiguration>,
        builder: PaymentMethodConfigurationBuilder,
    }

    impl Visitor for Place<PaymentMethodConfiguration> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodConfigurationBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodConfigurationBuilder {
        type Out = PaymentMethodConfiguration;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "acss_debit" => Ok(Deserialize::begin(&mut self.acss_debit)),
                "active" => Ok(Deserialize::begin(&mut self.active)),
                "affirm" => Ok(Deserialize::begin(&mut self.affirm)),
                "afterpay_clearpay" => Ok(Deserialize::begin(&mut self.afterpay_clearpay)),
                "alipay" => Ok(Deserialize::begin(&mut self.alipay)),
                "apple_pay" => Ok(Deserialize::begin(&mut self.apple_pay)),
                "application" => Ok(Deserialize::begin(&mut self.application)),
                "au_becs_debit" => Ok(Deserialize::begin(&mut self.au_becs_debit)),
                "bacs_debit" => Ok(Deserialize::begin(&mut self.bacs_debit)),
                "bancontact" => Ok(Deserialize::begin(&mut self.bancontact)),
                "blik" => Ok(Deserialize::begin(&mut self.blik)),
                "boleto" => Ok(Deserialize::begin(&mut self.boleto)),
                "card" => Ok(Deserialize::begin(&mut self.card)),
                "cartes_bancaires" => Ok(Deserialize::begin(&mut self.cartes_bancaires)),
                "cashapp" => Ok(Deserialize::begin(&mut self.cashapp)),
                "eps" => Ok(Deserialize::begin(&mut self.eps)),
                "fpx" => Ok(Deserialize::begin(&mut self.fpx)),
                "giropay" => Ok(Deserialize::begin(&mut self.giropay)),
                "google_pay" => Ok(Deserialize::begin(&mut self.google_pay)),
                "grabpay" => Ok(Deserialize::begin(&mut self.grabpay)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "id_bank_transfer" => Ok(Deserialize::begin(&mut self.id_bank_transfer)),
                "ideal" => Ok(Deserialize::begin(&mut self.ideal)),
                "is_default" => Ok(Deserialize::begin(&mut self.is_default)),
                "jcb" => Ok(Deserialize::begin(&mut self.jcb)),
                "klarna" => Ok(Deserialize::begin(&mut self.klarna)),
                "konbini" => Ok(Deserialize::begin(&mut self.konbini)),
                "link" => Ok(Deserialize::begin(&mut self.link)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "multibanco" => Ok(Deserialize::begin(&mut self.multibanco)),
                "name" => Ok(Deserialize::begin(&mut self.name)),
                "netbanking" => Ok(Deserialize::begin(&mut self.netbanking)),
                "oxxo" => Ok(Deserialize::begin(&mut self.oxxo)),
                "p24" => Ok(Deserialize::begin(&mut self.p24)),
                "parent" => Ok(Deserialize::begin(&mut self.parent)),
                "pay_by_bank" => Ok(Deserialize::begin(&mut self.pay_by_bank)),
                "paynow" => Ok(Deserialize::begin(&mut self.paynow)),
                "paypal" => Ok(Deserialize::begin(&mut self.paypal)),
                "promptpay" => Ok(Deserialize::begin(&mut self.promptpay)),
                "sepa_debit" => Ok(Deserialize::begin(&mut self.sepa_debit)),
                "sofort" => Ok(Deserialize::begin(&mut self.sofort)),
                "upi" => Ok(Deserialize::begin(&mut self.upi)),
                "us_bank_account" => Ok(Deserialize::begin(&mut self.us_bank_account)),
                "wechat_pay" => Ok(Deserialize::begin(&mut self.wechat_pay)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                acss_debit: Deserialize::default(),
                active: Deserialize::default(),
                affirm: Deserialize::default(),
                afterpay_clearpay: Deserialize::default(),
                alipay: Deserialize::default(),
                apple_pay: Deserialize::default(),
                application: Deserialize::default(),
                au_becs_debit: Deserialize::default(),
                bacs_debit: Deserialize::default(),
                bancontact: Deserialize::default(),
                blik: Deserialize::default(),
                boleto: Deserialize::default(),
                card: Deserialize::default(),
                cartes_bancaires: Deserialize::default(),
                cashapp: Deserialize::default(),
                eps: Deserialize::default(),
                fpx: Deserialize::default(),
                giropay: Deserialize::default(),
                google_pay: Deserialize::default(),
                grabpay: Deserialize::default(),
                id: Deserialize::default(),
                id_bank_transfer: Deserialize::default(),
                ideal: Deserialize::default(),
                is_default: Deserialize::default(),
                jcb: Deserialize::default(),
                klarna: Deserialize::default(),
                konbini: Deserialize::default(),
                link: Deserialize::default(),
                livemode: Deserialize::default(),
                multibanco: Deserialize::default(),
                name: Deserialize::default(),
                netbanking: Deserialize::default(),
                oxxo: Deserialize::default(),
                p24: Deserialize::default(),
                parent: Deserialize::default(),
                pay_by_bank: Deserialize::default(),
                paynow: Deserialize::default(),
                paypal: Deserialize::default(),
                promptpay: Deserialize::default(),
                sepa_debit: Deserialize::default(),
                sofort: Deserialize::default(),
                upi: Deserialize::default(),
                us_bank_account: Deserialize::default(),
                wechat_pay: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let acss_debit = self.acss_debit.take()?;
            let active = self.active.take()?;
            let affirm = self.affirm.take()?;
            let afterpay_clearpay = self.afterpay_clearpay.take()?;
            let alipay = self.alipay.take()?;
            let apple_pay = self.apple_pay.take()?;
            let application = self.application.take()?;
            let au_becs_debit = self.au_becs_debit.take()?;
            let bacs_debit = self.bacs_debit.take()?;
            let bancontact = self.bancontact.take()?;
            let blik = self.blik.take()?;
            let boleto = self.boleto.take()?;
            let card = self.card.take()?;
            let cartes_bancaires = self.cartes_bancaires.take()?;
            let cashapp = self.cashapp.take()?;
            let eps = self.eps.take()?;
            let fpx = self.fpx.take()?;
            let giropay = self.giropay.take()?;
            let google_pay = self.google_pay.take()?;
            let grabpay = self.grabpay.take()?;
            let id = self.id.take()?;
            let id_bank_transfer = self.id_bank_transfer.take()?;
            let ideal = self.ideal.take()?;
            let is_default = self.is_default.take()?;
            let jcb = self.jcb.take()?;
            let klarna = self.klarna.take()?;
            let konbini = self.konbini.take()?;
            let link = self.link.take()?;
            let livemode = self.livemode.take()?;
            let multibanco = self.multibanco.take()?;
            let name = self.name.take()?;
            let netbanking = self.netbanking.take()?;
            let oxxo = self.oxxo.take()?;
            let p24 = self.p24.take()?;
            let parent = self.parent.take()?;
            let pay_by_bank = self.pay_by_bank.take()?;
            let paynow = self.paynow.take()?;
            let paypal = self.paypal.take()?;
            let promptpay = self.promptpay.take()?;
            let sepa_debit = self.sepa_debit.take()?;
            let sofort = self.sofort.take()?;
            let upi = self.upi.take()?;
            let us_bank_account = self.us_bank_account.take()?;
            let wechat_pay = self.wechat_pay.take()?;

            Some(Self::Out {
                acss_debit,
                active,
                affirm,
                afterpay_clearpay,
                alipay,
                apple_pay,
                application,
                au_becs_debit,
                bacs_debit,
                bancontact,
                blik,
                boleto,
                card,
                cartes_bancaires,
                cashapp,
                eps,
                fpx,
                giropay,
                google_pay,
                grabpay,
                id,
                id_bank_transfer,
                ideal,
                is_default,
                jcb,
                klarna,
                konbini,
                link,
                livemode,
                multibanco,
                name,
                netbanking,
                oxxo,
                p24,
                parent,
                pay_by_bank,
                paynow,
                paypal,
                promptpay,
                sepa_debit,
                sofort,
                upi,
                us_bank_account,
                wechat_pay,
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

    impl ObjectDeser for PaymentMethodConfiguration {
        type Builder = PaymentMethodConfigurationBuilder;
    }
};
impl stripe_types::Object for PaymentMethodConfiguration {
    type Id = stripe_payment::PaymentMethodConfigurationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(PaymentMethodConfigurationId);
