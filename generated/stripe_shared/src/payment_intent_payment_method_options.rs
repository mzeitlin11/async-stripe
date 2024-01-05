#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentPaymentMethodOptions {
    pub acss_debit: Option<stripe_shared::PaymentIntentPaymentMethodOptionsAcssDebit>,
    pub affirm: Option<stripe_shared::PaymentMethodOptionsAffirm>,
    pub afterpay_clearpay: Option<stripe_shared::PaymentMethodOptionsAfterpayClearpay>,
    pub alipay: Option<stripe_shared::PaymentMethodOptionsAlipay>,
    pub au_becs_debit: Option<stripe_shared::PaymentIntentPaymentMethodOptionsAuBecsDebit>,
    pub bacs_debit: Option<stripe_shared::PaymentMethodOptionsBacsDebit>,
    pub bancontact: Option<stripe_shared::PaymentMethodOptionsBancontact>,
    pub blik: Option<stripe_shared::PaymentIntentPaymentMethodOptionsBlik>,
    pub boleto: Option<stripe_shared::PaymentMethodOptionsBoleto>,
    pub card: Option<stripe_shared::PaymentIntentPaymentMethodOptionsCard>,
    pub card_present: Option<stripe_shared::PaymentMethodOptionsCardPresent>,
    pub cashapp: Option<stripe_shared::PaymentMethodOptionsCashapp>,
    pub customer_balance: Option<stripe_shared::PaymentMethodOptionsCustomerBalance>,
    pub eps: Option<stripe_shared::PaymentIntentPaymentMethodOptionsEps>,
    pub fpx: Option<stripe_shared::PaymentMethodOptionsFpx>,
    pub giropay: Option<stripe_shared::PaymentMethodOptionsGiropay>,
    pub grabpay: Option<stripe_shared::PaymentMethodOptionsGrabpay>,
    pub ideal: Option<stripe_shared::PaymentMethodOptionsIdeal>,
    pub interac_present: Option<stripe_shared::PaymentMethodOptionsInteracPresent>,
    pub klarna: Option<stripe_shared::PaymentMethodOptionsKlarna>,
    pub konbini: Option<stripe_shared::PaymentMethodOptionsKonbini>,
    pub link: Option<stripe_shared::PaymentIntentPaymentMethodOptionsLink>,
    pub oxxo: Option<stripe_shared::PaymentMethodOptionsOxxo>,
    pub p24: Option<stripe_shared::PaymentMethodOptionsP24>,
    pub paynow: Option<stripe_shared::PaymentMethodOptionsPaynow>,
    pub paypal: Option<stripe_shared::PaymentMethodOptionsPaypal>,
    pub pix: Option<stripe_shared::PaymentMethodOptionsPix>,
    pub promptpay: Option<stripe_shared::PaymentMethodOptionsPromptpay>,
    pub revolut_pay: Option<stripe_shared::PaymentMethodOptionsRevolutPay>,
    pub sepa_debit: Option<stripe_shared::PaymentIntentPaymentMethodOptionsSepaDebit>,
    pub sofort: Option<stripe_shared::PaymentMethodOptionsSofort>,
    pub us_bank_account: Option<stripe_shared::PaymentIntentPaymentMethodOptionsUsBankAccount>,
    pub wechat_pay: Option<stripe_shared::PaymentMethodOptionsWechatPay>,
    pub zip: Option<stripe_shared::PaymentMethodOptionsZip>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentIntentPaymentMethodOptionsBuilder {
    acss_debit: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsAcssDebit>>,
    affirm: Option<Option<stripe_shared::PaymentMethodOptionsAffirm>>,
    afterpay_clearpay: Option<Option<stripe_shared::PaymentMethodOptionsAfterpayClearpay>>,
    alipay: Option<Option<stripe_shared::PaymentMethodOptionsAlipay>>,
    au_becs_debit: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsAuBecsDebit>>,
    bacs_debit: Option<Option<stripe_shared::PaymentMethodOptionsBacsDebit>>,
    bancontact: Option<Option<stripe_shared::PaymentMethodOptionsBancontact>>,
    blik: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsBlik>>,
    boleto: Option<Option<stripe_shared::PaymentMethodOptionsBoleto>>,
    card: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsCard>>,
    card_present: Option<Option<stripe_shared::PaymentMethodOptionsCardPresent>>,
    cashapp: Option<Option<stripe_shared::PaymentMethodOptionsCashapp>>,
    customer_balance: Option<Option<stripe_shared::PaymentMethodOptionsCustomerBalance>>,
    eps: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsEps>>,
    fpx: Option<Option<stripe_shared::PaymentMethodOptionsFpx>>,
    giropay: Option<Option<stripe_shared::PaymentMethodOptionsGiropay>>,
    grabpay: Option<Option<stripe_shared::PaymentMethodOptionsGrabpay>>,
    ideal: Option<Option<stripe_shared::PaymentMethodOptionsIdeal>>,
    interac_present: Option<Option<stripe_shared::PaymentMethodOptionsInteracPresent>>,
    klarna: Option<Option<stripe_shared::PaymentMethodOptionsKlarna>>,
    konbini: Option<Option<stripe_shared::PaymentMethodOptionsKonbini>>,
    link: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsLink>>,
    oxxo: Option<Option<stripe_shared::PaymentMethodOptionsOxxo>>,
    p24: Option<Option<stripe_shared::PaymentMethodOptionsP24>>,
    paynow: Option<Option<stripe_shared::PaymentMethodOptionsPaynow>>,
    paypal: Option<Option<stripe_shared::PaymentMethodOptionsPaypal>>,
    pix: Option<Option<stripe_shared::PaymentMethodOptionsPix>>,
    promptpay: Option<Option<stripe_shared::PaymentMethodOptionsPromptpay>>,
    revolut_pay: Option<Option<stripe_shared::PaymentMethodOptionsRevolutPay>>,
    sepa_debit: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsSepaDebit>>,
    sofort: Option<Option<stripe_shared::PaymentMethodOptionsSofort>>,
    us_bank_account: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsUsBankAccount>>,
    wechat_pay: Option<Option<stripe_shared::PaymentMethodOptionsWechatPay>>,
    zip: Option<Option<stripe_shared::PaymentMethodOptionsZip>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentPaymentMethodOptions>,
        builder: PaymentIntentPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<PaymentIntentPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentIntentPaymentMethodOptionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentIntentPaymentMethodOptionsBuilder {
        type Out = PaymentIntentPaymentMethodOptions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "acss_debit" => Ok(Deserialize::begin(&mut self.acss_debit)),
                "affirm" => Ok(Deserialize::begin(&mut self.affirm)),
                "afterpay_clearpay" => Ok(Deserialize::begin(&mut self.afterpay_clearpay)),
                "alipay" => Ok(Deserialize::begin(&mut self.alipay)),
                "au_becs_debit" => Ok(Deserialize::begin(&mut self.au_becs_debit)),
                "bacs_debit" => Ok(Deserialize::begin(&mut self.bacs_debit)),
                "bancontact" => Ok(Deserialize::begin(&mut self.bancontact)),
                "blik" => Ok(Deserialize::begin(&mut self.blik)),
                "boleto" => Ok(Deserialize::begin(&mut self.boleto)),
                "card" => Ok(Deserialize::begin(&mut self.card)),
                "card_present" => Ok(Deserialize::begin(&mut self.card_present)),
                "cashapp" => Ok(Deserialize::begin(&mut self.cashapp)),
                "customer_balance" => Ok(Deserialize::begin(&mut self.customer_balance)),
                "eps" => Ok(Deserialize::begin(&mut self.eps)),
                "fpx" => Ok(Deserialize::begin(&mut self.fpx)),
                "giropay" => Ok(Deserialize::begin(&mut self.giropay)),
                "grabpay" => Ok(Deserialize::begin(&mut self.grabpay)),
                "ideal" => Ok(Deserialize::begin(&mut self.ideal)),
                "interac_present" => Ok(Deserialize::begin(&mut self.interac_present)),
                "klarna" => Ok(Deserialize::begin(&mut self.klarna)),
                "konbini" => Ok(Deserialize::begin(&mut self.konbini)),
                "link" => Ok(Deserialize::begin(&mut self.link)),
                "oxxo" => Ok(Deserialize::begin(&mut self.oxxo)),
                "p24" => Ok(Deserialize::begin(&mut self.p24)),
                "paynow" => Ok(Deserialize::begin(&mut self.paynow)),
                "paypal" => Ok(Deserialize::begin(&mut self.paypal)),
                "pix" => Ok(Deserialize::begin(&mut self.pix)),
                "promptpay" => Ok(Deserialize::begin(&mut self.promptpay)),
                "revolut_pay" => Ok(Deserialize::begin(&mut self.revolut_pay)),
                "sepa_debit" => Ok(Deserialize::begin(&mut self.sepa_debit)),
                "sofort" => Ok(Deserialize::begin(&mut self.sofort)),
                "us_bank_account" => Ok(Deserialize::begin(&mut self.us_bank_account)),
                "wechat_pay" => Ok(Deserialize::begin(&mut self.wechat_pay)),
                "zip" => Ok(Deserialize::begin(&mut self.zip)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                acss_debit: Deserialize::default(),
                affirm: Deserialize::default(),
                afterpay_clearpay: Deserialize::default(),
                alipay: Deserialize::default(),
                au_becs_debit: Deserialize::default(),
                bacs_debit: Deserialize::default(),
                bancontact: Deserialize::default(),
                blik: Deserialize::default(),
                boleto: Deserialize::default(),
                card: Deserialize::default(),
                card_present: Deserialize::default(),
                cashapp: Deserialize::default(),
                customer_balance: Deserialize::default(),
                eps: Deserialize::default(),
                fpx: Deserialize::default(),
                giropay: Deserialize::default(),
                grabpay: Deserialize::default(),
                ideal: Deserialize::default(),
                interac_present: Deserialize::default(),
                klarna: Deserialize::default(),
                konbini: Deserialize::default(),
                link: Deserialize::default(),
                oxxo: Deserialize::default(),
                p24: Deserialize::default(),
                paynow: Deserialize::default(),
                paypal: Deserialize::default(),
                pix: Deserialize::default(),
                promptpay: Deserialize::default(),
                revolut_pay: Deserialize::default(),
                sepa_debit: Deserialize::default(),
                sofort: Deserialize::default(),
                us_bank_account: Deserialize::default(),
                wechat_pay: Deserialize::default(),
                zip: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let acss_debit = self.acss_debit.take()?;
            let affirm = self.affirm.take()?;
            let afterpay_clearpay = self.afterpay_clearpay.take()?;
            let alipay = self.alipay.take()?;
            let au_becs_debit = self.au_becs_debit.take()?;
            let bacs_debit = self.bacs_debit.take()?;
            let bancontact = self.bancontact.take()?;
            let blik = self.blik.take()?;
            let boleto = self.boleto.take()?;
            let card = self.card.take()?;
            let card_present = self.card_present.take()?;
            let cashapp = self.cashapp.take()?;
            let customer_balance = self.customer_balance.take()?;
            let eps = self.eps.take()?;
            let fpx = self.fpx.take()?;
            let giropay = self.giropay.take()?;
            let grabpay = self.grabpay.take()?;
            let ideal = self.ideal.take()?;
            let interac_present = self.interac_present.take()?;
            let klarna = self.klarna.take()?;
            let konbini = self.konbini.take()?;
            let link = self.link.take()?;
            let oxxo = self.oxxo.take()?;
            let p24 = self.p24.take()?;
            let paynow = self.paynow.take()?;
            let paypal = self.paypal.take()?;
            let pix = self.pix.take()?;
            let promptpay = self.promptpay.take()?;
            let revolut_pay = self.revolut_pay.take()?;
            let sepa_debit = self.sepa_debit.take()?;
            let sofort = self.sofort.take()?;
            let us_bank_account = self.us_bank_account.take()?;
            let wechat_pay = self.wechat_pay.take()?;
            let zip = self.zip.take()?;

            Some(Self::Out {
                acss_debit,
                affirm,
                afterpay_clearpay,
                alipay,
                au_becs_debit,
                bacs_debit,
                bancontact,
                blik,
                boleto,
                card,
                card_present,
                cashapp,
                customer_balance,
                eps,
                fpx,
                giropay,
                grabpay,
                ideal,
                interac_present,
                klarna,
                konbini,
                link,
                oxxo,
                p24,
                paynow,
                paypal,
                pix,
                promptpay,
                revolut_pay,
                sepa_debit,
                sofort,
                us_bank_account,
                wechat_pay,
                zip,
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

    impl ObjectDeser for PaymentIntentPaymentMethodOptions {
        type Builder = PaymentIntentPaymentMethodOptionsBuilder;
    }
};
