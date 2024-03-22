#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CheckoutSessionPaymentMethodOptions {
    pub acss_debit: Option<stripe_checkout::CheckoutAcssDebitPaymentMethodOptions>,
    pub affirm: Option<stripe_checkout::CheckoutAffirmPaymentMethodOptions>,
    pub afterpay_clearpay: Option<stripe_checkout::CheckoutAfterpayClearpayPaymentMethodOptions>,
    pub alipay: Option<stripe_checkout::CheckoutAlipayPaymentMethodOptions>,
    pub au_becs_debit: Option<stripe_checkout::CheckoutAuBecsDebitPaymentMethodOptions>,
    pub bacs_debit: Option<stripe_checkout::CheckoutBacsDebitPaymentMethodOptions>,
    pub bancontact: Option<stripe_checkout::CheckoutBancontactPaymentMethodOptions>,
    pub boleto: Option<stripe_checkout::CheckoutBoletoPaymentMethodOptions>,
    pub card: Option<stripe_checkout::CheckoutCardPaymentMethodOptions>,
    pub cashapp: Option<stripe_checkout::CheckoutCashappPaymentMethodOptions>,
    pub customer_balance: Option<stripe_checkout::CheckoutCustomerBalancePaymentMethodOptions>,
    pub eps: Option<stripe_checkout::CheckoutEpsPaymentMethodOptions>,
    pub fpx: Option<stripe_checkout::CheckoutFpxPaymentMethodOptions>,
    pub giropay: Option<stripe_checkout::CheckoutGiropayPaymentMethodOptions>,
    pub grabpay: Option<stripe_checkout::CheckoutGrabPayPaymentMethodOptions>,
    pub ideal: Option<stripe_checkout::CheckoutIdealPaymentMethodOptions>,
    pub klarna: Option<stripe_checkout::CheckoutKlarnaPaymentMethodOptions>,
    pub konbini: Option<stripe_checkout::CheckoutKonbiniPaymentMethodOptions>,
    pub link: Option<stripe_checkout::CheckoutLinkPaymentMethodOptions>,
    pub oxxo: Option<stripe_checkout::CheckoutOxxoPaymentMethodOptions>,
    pub p24: Option<stripe_checkout::CheckoutP24PaymentMethodOptions>,
    pub paynow: Option<stripe_checkout::CheckoutPaynowPaymentMethodOptions>,
    pub paypal: Option<stripe_checkout::CheckoutPaypalPaymentMethodOptions>,
    pub pix: Option<stripe_checkout::CheckoutPixPaymentMethodOptions>,
    pub revolut_pay: Option<stripe_checkout::CheckoutRevolutPayPaymentMethodOptions>,
    pub sepa_debit: Option<stripe_checkout::CheckoutSepaDebitPaymentMethodOptions>,
    pub sofort: Option<stripe_checkout::CheckoutSofortPaymentMethodOptions>,
    pub us_bank_account: Option<stripe_checkout::CheckoutUsBankAccountPaymentMethodOptions>,
}
#[cfg(feature = "min-ser")]
pub struct CheckoutSessionPaymentMethodOptionsBuilder {
    acss_debit: Option<Option<stripe_checkout::CheckoutAcssDebitPaymentMethodOptions>>,
    affirm: Option<Option<stripe_checkout::CheckoutAffirmPaymentMethodOptions>>,
    afterpay_clearpay: Option<Option<stripe_checkout::CheckoutAfterpayClearpayPaymentMethodOptions>>,
    alipay: Option<Option<stripe_checkout::CheckoutAlipayPaymentMethodOptions>>,
    au_becs_debit: Option<Option<stripe_checkout::CheckoutAuBecsDebitPaymentMethodOptions>>,
    bacs_debit: Option<Option<stripe_checkout::CheckoutBacsDebitPaymentMethodOptions>>,
    bancontact: Option<Option<stripe_checkout::CheckoutBancontactPaymentMethodOptions>>,
    boleto: Option<Option<stripe_checkout::CheckoutBoletoPaymentMethodOptions>>,
    card: Option<Option<stripe_checkout::CheckoutCardPaymentMethodOptions>>,
    cashapp: Option<Option<stripe_checkout::CheckoutCashappPaymentMethodOptions>>,
    customer_balance: Option<Option<stripe_checkout::CheckoutCustomerBalancePaymentMethodOptions>>,
    eps: Option<Option<stripe_checkout::CheckoutEpsPaymentMethodOptions>>,
    fpx: Option<Option<stripe_checkout::CheckoutFpxPaymentMethodOptions>>,
    giropay: Option<Option<stripe_checkout::CheckoutGiropayPaymentMethodOptions>>,
    grabpay: Option<Option<stripe_checkout::CheckoutGrabPayPaymentMethodOptions>>,
    ideal: Option<Option<stripe_checkout::CheckoutIdealPaymentMethodOptions>>,
    klarna: Option<Option<stripe_checkout::CheckoutKlarnaPaymentMethodOptions>>,
    konbini: Option<Option<stripe_checkout::CheckoutKonbiniPaymentMethodOptions>>,
    link: Option<Option<stripe_checkout::CheckoutLinkPaymentMethodOptions>>,
    oxxo: Option<Option<stripe_checkout::CheckoutOxxoPaymentMethodOptions>>,
    p24: Option<Option<stripe_checkout::CheckoutP24PaymentMethodOptions>>,
    paynow: Option<Option<stripe_checkout::CheckoutPaynowPaymentMethodOptions>>,
    paypal: Option<Option<stripe_checkout::CheckoutPaypalPaymentMethodOptions>>,
    pix: Option<Option<stripe_checkout::CheckoutPixPaymentMethodOptions>>,
    revolut_pay: Option<Option<stripe_checkout::CheckoutRevolutPayPaymentMethodOptions>>,
    sepa_debit: Option<Option<stripe_checkout::CheckoutSepaDebitPaymentMethodOptions>>,
    sofort: Option<Option<stripe_checkout::CheckoutSofortPaymentMethodOptions>>,
    us_bank_account: Option<Option<stripe_checkout::CheckoutUsBankAccountPaymentMethodOptions>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CheckoutSessionPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutSessionPaymentMethodOptions>,
        builder: CheckoutSessionPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<CheckoutSessionPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CheckoutSessionPaymentMethodOptionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CheckoutSessionPaymentMethodOptionsBuilder {
        type Out = CheckoutSessionPaymentMethodOptions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "acss_debit" => Ok(Deserialize::begin(&mut self.acss_debit)),
                "affirm" => Ok(Deserialize::begin(&mut self.affirm)),
                "afterpay_clearpay" => Ok(Deserialize::begin(&mut self.afterpay_clearpay)),
                "alipay" => Ok(Deserialize::begin(&mut self.alipay)),
                "au_becs_debit" => Ok(Deserialize::begin(&mut self.au_becs_debit)),
                "bacs_debit" => Ok(Deserialize::begin(&mut self.bacs_debit)),
                "bancontact" => Ok(Deserialize::begin(&mut self.bancontact)),
                "boleto" => Ok(Deserialize::begin(&mut self.boleto)),
                "card" => Ok(Deserialize::begin(&mut self.card)),
                "cashapp" => Ok(Deserialize::begin(&mut self.cashapp)),
                "customer_balance" => Ok(Deserialize::begin(&mut self.customer_balance)),
                "eps" => Ok(Deserialize::begin(&mut self.eps)),
                "fpx" => Ok(Deserialize::begin(&mut self.fpx)),
                "giropay" => Ok(Deserialize::begin(&mut self.giropay)),
                "grabpay" => Ok(Deserialize::begin(&mut self.grabpay)),
                "ideal" => Ok(Deserialize::begin(&mut self.ideal)),
                "klarna" => Ok(Deserialize::begin(&mut self.klarna)),
                "konbini" => Ok(Deserialize::begin(&mut self.konbini)),
                "link" => Ok(Deserialize::begin(&mut self.link)),
                "oxxo" => Ok(Deserialize::begin(&mut self.oxxo)),
                "p24" => Ok(Deserialize::begin(&mut self.p24)),
                "paynow" => Ok(Deserialize::begin(&mut self.paynow)),
                "paypal" => Ok(Deserialize::begin(&mut self.paypal)),
                "pix" => Ok(Deserialize::begin(&mut self.pix)),
                "revolut_pay" => Ok(Deserialize::begin(&mut self.revolut_pay)),
                "sepa_debit" => Ok(Deserialize::begin(&mut self.sepa_debit)),
                "sofort" => Ok(Deserialize::begin(&mut self.sofort)),
                "us_bank_account" => Ok(Deserialize::begin(&mut self.us_bank_account)),

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
                boleto: Deserialize::default(),
                card: Deserialize::default(),
                cashapp: Deserialize::default(),
                customer_balance: Deserialize::default(),
                eps: Deserialize::default(),
                fpx: Deserialize::default(),
                giropay: Deserialize::default(),
                grabpay: Deserialize::default(),
                ideal: Deserialize::default(),
                klarna: Deserialize::default(),
                konbini: Deserialize::default(),
                link: Deserialize::default(),
                oxxo: Deserialize::default(),
                p24: Deserialize::default(),
                paynow: Deserialize::default(),
                paypal: Deserialize::default(),
                pix: Deserialize::default(),
                revolut_pay: Deserialize::default(),
                sepa_debit: Deserialize::default(),
                sofort: Deserialize::default(),
                us_bank_account: Deserialize::default(),
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
            let boleto = self.boleto.take()?;
            let card = self.card.take()?;
            let cashapp = self.cashapp.take()?;
            let customer_balance = self.customer_balance.take()?;
            let eps = self.eps.take()?;
            let fpx = self.fpx.take()?;
            let giropay = self.giropay.take()?;
            let grabpay = self.grabpay.take()?;
            let ideal = self.ideal.take()?;
            let klarna = self.klarna.take()?;
            let konbini = self.konbini.take()?;
            let link = self.link.take()?;
            let oxxo = self.oxxo.take()?;
            let p24 = self.p24.take()?;
            let paynow = self.paynow.take()?;
            let paypal = self.paypal.take()?;
            let pix = self.pix.take()?;
            let revolut_pay = self.revolut_pay.take()?;
            let sepa_debit = self.sepa_debit.take()?;
            let sofort = self.sofort.take()?;
            let us_bank_account = self.us_bank_account.take()?;

            Some(Self::Out {
                acss_debit,
                affirm,
                afterpay_clearpay,
                alipay,
                au_becs_debit,
                bacs_debit,
                bancontact,
                boleto,
                card,
                cashapp,
                customer_balance,
                eps,
                fpx,
                giropay,
                grabpay,
                ideal,
                klarna,
                konbini,
                link,
                oxxo,
                p24,
                paynow,
                paypal,
                pix,
                revolut_pay,
                sepa_debit,
                sofort,
                us_bank_account,
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

    impl ObjectDeser for CheckoutSessionPaymentMethodOptions {
        type Builder = CheckoutSessionPaymentMethodOptionsBuilder;
    }
};
