#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetails {
    pub ach_credit_transfer: Option<stripe_shared::PaymentMethodDetailsAchCreditTransfer>,
    pub ach_debit: Option<stripe_shared::PaymentMethodDetailsAchDebit>,
    pub acss_debit: Option<stripe_shared::PaymentMethodDetailsAcssDebit>,
    pub affirm: Option<stripe_shared::PaymentMethodDetailsAffirm>,
    pub afterpay_clearpay: Option<stripe_shared::PaymentMethodDetailsAfterpayClearpay>,
    pub alipay: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsAlipayDetails>,
    pub au_becs_debit: Option<stripe_shared::PaymentMethodDetailsAuBecsDebit>,
    pub bacs_debit: Option<stripe_shared::PaymentMethodDetailsBacsDebit>,
    pub bancontact: Option<stripe_shared::PaymentMethodDetailsBancontact>,
    pub blik: Option<stripe_shared::PaymentMethodDetailsBlik>,
    pub boleto: Option<stripe_shared::PaymentMethodDetailsBoleto>,
    pub card: Option<stripe_shared::PaymentMethodDetailsCard>,
    pub card_present: Option<stripe_shared::PaymentMethodDetailsCardPresent>,
    pub cashapp: Option<stripe_shared::PaymentMethodDetailsCashapp>,
    pub customer_balance: Option<stripe_shared::PaymentMethodDetailsCustomerBalance>,
    pub eps: Option<stripe_shared::PaymentMethodDetailsEps>,
    pub fpx: Option<stripe_shared::PaymentMethodDetailsFpx>,
    pub giropay: Option<stripe_shared::PaymentMethodDetailsGiropay>,
    pub grabpay: Option<stripe_shared::PaymentMethodDetailsGrabpay>,
    pub ideal: Option<stripe_shared::PaymentMethodDetailsIdeal>,
    pub interac_present: Option<stripe_shared::PaymentMethodDetailsInteracPresent>,
    pub klarna: Option<stripe_shared::PaymentMethodDetailsKlarna>,
    pub konbini: Option<stripe_shared::PaymentMethodDetailsKonbini>,
    pub link: Option<stripe_shared::PaymentMethodDetailsLink>,
    pub multibanco: Option<stripe_shared::PaymentMethodDetailsMultibanco>,
    pub oxxo: Option<stripe_shared::PaymentMethodDetailsOxxo>,
    pub p24: Option<stripe_shared::PaymentMethodDetailsP24>,
    pub paynow: Option<stripe_shared::PaymentMethodDetailsPaynow>,
    pub paypal: Option<stripe_shared::PaymentMethodDetailsPaypal>,
    pub pix: Option<stripe_shared::PaymentMethodDetailsPix>,
    pub promptpay: Option<stripe_shared::PaymentMethodDetailsPromptpay>,
    pub revolut_pay: Option<stripe_shared::PaymentMethodDetailsRevolutPay>,
    pub sepa_credit_transfer: Option<stripe_shared::PaymentMethodDetailsSepaCreditTransfer>,
    pub sepa_debit: Option<stripe_shared::PaymentMethodDetailsSepaDebit>,
    pub sofort: Option<stripe_shared::PaymentMethodDetailsSofort>,
    pub stripe_account: Option<stripe_shared::PaymentMethodDetailsStripeAccount>,
    /// The type of transaction-specific details of the payment method used in the payment, one of `ach_credit_transfer`, `ach_debit`, `acss_debit`, `alipay`, `au_becs_debit`, `bancontact`, `card`, `card_present`, `eps`, `giropay`, `ideal`, `klarna`, `multibanco`, `p24`, `sepa_debit`, `sofort`, `stripe_account`, or `wechat`.
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains information specific to the payment method.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: String,
    pub us_bank_account: Option<stripe_shared::PaymentMethodDetailsUsBankAccount>,
    pub wechat: Option<stripe_shared::PaymentMethodDetailsWechat>,
    pub wechat_pay: Option<stripe_shared::PaymentMethodDetailsWechatPay>,
    pub zip: Option<stripe_shared::PaymentMethodDetailsZip>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDetailsBuilder {
    ach_credit_transfer: Option<Option<stripe_shared::PaymentMethodDetailsAchCreditTransfer>>,
    ach_debit: Option<Option<stripe_shared::PaymentMethodDetailsAchDebit>>,
    acss_debit: Option<Option<stripe_shared::PaymentMethodDetailsAcssDebit>>,
    affirm: Option<Option<stripe_shared::PaymentMethodDetailsAffirm>>,
    afterpay_clearpay: Option<Option<stripe_shared::PaymentMethodDetailsAfterpayClearpay>>,
    alipay: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsAlipayDetails>>,
    au_becs_debit: Option<Option<stripe_shared::PaymentMethodDetailsAuBecsDebit>>,
    bacs_debit: Option<Option<stripe_shared::PaymentMethodDetailsBacsDebit>>,
    bancontact: Option<Option<stripe_shared::PaymentMethodDetailsBancontact>>,
    blik: Option<Option<stripe_shared::PaymentMethodDetailsBlik>>,
    boleto: Option<Option<stripe_shared::PaymentMethodDetailsBoleto>>,
    card: Option<Option<stripe_shared::PaymentMethodDetailsCard>>,
    card_present: Option<Option<stripe_shared::PaymentMethodDetailsCardPresent>>,
    cashapp: Option<Option<stripe_shared::PaymentMethodDetailsCashapp>>,
    customer_balance: Option<Option<stripe_shared::PaymentMethodDetailsCustomerBalance>>,
    eps: Option<Option<stripe_shared::PaymentMethodDetailsEps>>,
    fpx: Option<Option<stripe_shared::PaymentMethodDetailsFpx>>,
    giropay: Option<Option<stripe_shared::PaymentMethodDetailsGiropay>>,
    grabpay: Option<Option<stripe_shared::PaymentMethodDetailsGrabpay>>,
    ideal: Option<Option<stripe_shared::PaymentMethodDetailsIdeal>>,
    interac_present: Option<Option<stripe_shared::PaymentMethodDetailsInteracPresent>>,
    klarna: Option<Option<stripe_shared::PaymentMethodDetailsKlarna>>,
    konbini: Option<Option<stripe_shared::PaymentMethodDetailsKonbini>>,
    link: Option<Option<stripe_shared::PaymentMethodDetailsLink>>,
    multibanco: Option<Option<stripe_shared::PaymentMethodDetailsMultibanco>>,
    oxxo: Option<Option<stripe_shared::PaymentMethodDetailsOxxo>>,
    p24: Option<Option<stripe_shared::PaymentMethodDetailsP24>>,
    paynow: Option<Option<stripe_shared::PaymentMethodDetailsPaynow>>,
    paypal: Option<Option<stripe_shared::PaymentMethodDetailsPaypal>>,
    pix: Option<Option<stripe_shared::PaymentMethodDetailsPix>>,
    promptpay: Option<Option<stripe_shared::PaymentMethodDetailsPromptpay>>,
    revolut_pay: Option<Option<stripe_shared::PaymentMethodDetailsRevolutPay>>,
    sepa_credit_transfer: Option<Option<stripe_shared::PaymentMethodDetailsSepaCreditTransfer>>,
    sepa_debit: Option<Option<stripe_shared::PaymentMethodDetailsSepaDebit>>,
    sofort: Option<Option<stripe_shared::PaymentMethodDetailsSofort>>,
    stripe_account: Option<Option<stripe_shared::PaymentMethodDetailsStripeAccount>>,
    type_: Option<String>,
    us_bank_account: Option<Option<stripe_shared::PaymentMethodDetailsUsBankAccount>>,
    wechat: Option<Option<stripe_shared::PaymentMethodDetailsWechat>>,
    wechat_pay: Option<Option<stripe_shared::PaymentMethodDetailsWechatPay>>,
    zip: Option<Option<stripe_shared::PaymentMethodDetailsZip>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetails>,
        builder: PaymentMethodDetailsBuilder,
    }

    impl Visitor for Place<PaymentMethodDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsBuilder {
        type Out = PaymentMethodDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "ach_credit_transfer" => Ok(Deserialize::begin(&mut self.ach_credit_transfer)),
                "ach_debit" => Ok(Deserialize::begin(&mut self.ach_debit)),
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
                "multibanco" => Ok(Deserialize::begin(&mut self.multibanco)),
                "oxxo" => Ok(Deserialize::begin(&mut self.oxxo)),
                "p24" => Ok(Deserialize::begin(&mut self.p24)),
                "paynow" => Ok(Deserialize::begin(&mut self.paynow)),
                "paypal" => Ok(Deserialize::begin(&mut self.paypal)),
                "pix" => Ok(Deserialize::begin(&mut self.pix)),
                "promptpay" => Ok(Deserialize::begin(&mut self.promptpay)),
                "revolut_pay" => Ok(Deserialize::begin(&mut self.revolut_pay)),
                "sepa_credit_transfer" => Ok(Deserialize::begin(&mut self.sepa_credit_transfer)),
                "sepa_debit" => Ok(Deserialize::begin(&mut self.sepa_debit)),
                "sofort" => Ok(Deserialize::begin(&mut self.sofort)),
                "stripe_account" => Ok(Deserialize::begin(&mut self.stripe_account)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),
                "us_bank_account" => Ok(Deserialize::begin(&mut self.us_bank_account)),
                "wechat" => Ok(Deserialize::begin(&mut self.wechat)),
                "wechat_pay" => Ok(Deserialize::begin(&mut self.wechat_pay)),
                "zip" => Ok(Deserialize::begin(&mut self.zip)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                ach_credit_transfer: Deserialize::default(),
                ach_debit: Deserialize::default(),
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
                multibanco: Deserialize::default(),
                oxxo: Deserialize::default(),
                p24: Deserialize::default(),
                paynow: Deserialize::default(),
                paypal: Deserialize::default(),
                pix: Deserialize::default(),
                promptpay: Deserialize::default(),
                revolut_pay: Deserialize::default(),
                sepa_credit_transfer: Deserialize::default(),
                sepa_debit: Deserialize::default(),
                sofort: Deserialize::default(),
                stripe_account: Deserialize::default(),
                type_: Deserialize::default(),
                us_bank_account: Deserialize::default(),
                wechat: Deserialize::default(),
                wechat_pay: Deserialize::default(),
                zip: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let ach_credit_transfer = self.ach_credit_transfer.take()?;
            let ach_debit = self.ach_debit.take()?;
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
            let multibanco = self.multibanco.take()?;
            let oxxo = self.oxxo.take()?;
            let p24 = self.p24.take()?;
            let paynow = self.paynow.take()?;
            let paypal = self.paypal.take()?;
            let pix = self.pix.take()?;
            let promptpay = self.promptpay.take()?;
            let revolut_pay = self.revolut_pay.take()?;
            let sepa_credit_transfer = self.sepa_credit_transfer.take()?;
            let sepa_debit = self.sepa_debit.take()?;
            let sofort = self.sofort.take()?;
            let stripe_account = self.stripe_account.take()?;
            let type_ = self.type_.take()?;
            let us_bank_account = self.us_bank_account.take()?;
            let wechat = self.wechat.take()?;
            let wechat_pay = self.wechat_pay.take()?;
            let zip = self.zip.take()?;

            Some(Self::Out {
                ach_credit_transfer,
                ach_debit,
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
                multibanco,
                oxxo,
                p24,
                paynow,
                paypal,
                pix,
                promptpay,
                revolut_pay,
                sepa_credit_transfer,
                sepa_debit,
                sofort,
                stripe_account,
                type_,
                us_bank_account,
                wechat,
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

    impl ObjectDeser for PaymentMethodDetails {
        type Builder = PaymentMethodDetailsBuilder;
    }
};
