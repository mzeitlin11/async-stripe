#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SetupAttemptPaymentMethodDetails {
    pub acss_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsAcssDebit>,
    pub au_becs_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsAuBecsDebit>,
    pub bacs_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsBacsDebit>,
    pub bancontact: Option<stripe_shared::SetupAttemptPaymentMethodDetailsBancontact>,
    pub boleto: Option<stripe_shared::SetupAttemptPaymentMethodDetailsBoleto>,
    pub card: Option<stripe_shared::SetupAttemptPaymentMethodDetailsCard>,
    pub card_present: Option<stripe_shared::SetupAttemptPaymentMethodDetailsCardPresent>,
    pub cashapp: Option<stripe_shared::SetupAttemptPaymentMethodDetailsCashapp>,
    pub ideal: Option<stripe_shared::SetupAttemptPaymentMethodDetailsIdeal>,
    pub klarna: Option<stripe_shared::SetupAttemptPaymentMethodDetailsKlarna>,
    pub link: Option<stripe_shared::SetupAttemptPaymentMethodDetailsLink>,
    pub paypal: Option<stripe_shared::SetupAttemptPaymentMethodDetailsPaypal>,
    pub sepa_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsSepaDebit>,
    pub sofort: Option<stripe_shared::SetupAttemptPaymentMethodDetailsSofort>,
    /// The type of the payment method used in the SetupIntent (e.g., `card`).
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains confirmation-specific information for the payment method.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: String,
    pub us_bank_account: Option<stripe_shared::SetupAttemptPaymentMethodDetailsUsBankAccount>,
}
#[cfg(feature = "min-ser")]
pub struct SetupAttemptPaymentMethodDetailsBuilder {
    acss_debit: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsAcssDebit>>,
    au_becs_debit: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsAuBecsDebit>>,
    bacs_debit: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsBacsDebit>>,
    bancontact: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsBancontact>>,
    boleto: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsBoleto>>,
    card: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsCard>>,
    card_present: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsCardPresent>>,
    cashapp: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsCashapp>>,
    ideal: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsIdeal>>,
    klarna: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsKlarna>>,
    link: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsLink>>,
    paypal: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsPaypal>>,
    sepa_debit: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsSepaDebit>>,
    sofort: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsSofort>>,
    type_: Option<String>,
    us_bank_account: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsUsBankAccount>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SetupAttemptPaymentMethodDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupAttemptPaymentMethodDetails>,
        builder: SetupAttemptPaymentMethodDetailsBuilder,
    }

    impl Visitor for Place<SetupAttemptPaymentMethodDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SetupAttemptPaymentMethodDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SetupAttemptPaymentMethodDetailsBuilder {
        type Out = SetupAttemptPaymentMethodDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "acss_debit" => Ok(Deserialize::begin(&mut self.acss_debit)),
                "au_becs_debit" => Ok(Deserialize::begin(&mut self.au_becs_debit)),
                "bacs_debit" => Ok(Deserialize::begin(&mut self.bacs_debit)),
                "bancontact" => Ok(Deserialize::begin(&mut self.bancontact)),
                "boleto" => Ok(Deserialize::begin(&mut self.boleto)),
                "card" => Ok(Deserialize::begin(&mut self.card)),
                "card_present" => Ok(Deserialize::begin(&mut self.card_present)),
                "cashapp" => Ok(Deserialize::begin(&mut self.cashapp)),
                "ideal" => Ok(Deserialize::begin(&mut self.ideal)),
                "klarna" => Ok(Deserialize::begin(&mut self.klarna)),
                "link" => Ok(Deserialize::begin(&mut self.link)),
                "paypal" => Ok(Deserialize::begin(&mut self.paypal)),
                "sepa_debit" => Ok(Deserialize::begin(&mut self.sepa_debit)),
                "sofort" => Ok(Deserialize::begin(&mut self.sofort)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),
                "us_bank_account" => Ok(Deserialize::begin(&mut self.us_bank_account)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                acss_debit: Deserialize::default(),
                au_becs_debit: Deserialize::default(),
                bacs_debit: Deserialize::default(),
                bancontact: Deserialize::default(),
                boleto: Deserialize::default(),
                card: Deserialize::default(),
                card_present: Deserialize::default(),
                cashapp: Deserialize::default(),
                ideal: Deserialize::default(),
                klarna: Deserialize::default(),
                link: Deserialize::default(),
                paypal: Deserialize::default(),
                sepa_debit: Deserialize::default(),
                sofort: Deserialize::default(),
                type_: Deserialize::default(),
                us_bank_account: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let acss_debit = self.acss_debit.take()?;
            let au_becs_debit = self.au_becs_debit.take()?;
            let bacs_debit = self.bacs_debit.take()?;
            let bancontact = self.bancontact.take()?;
            let boleto = self.boleto.take()?;
            let card = self.card.take()?;
            let card_present = self.card_present.take()?;
            let cashapp = self.cashapp.take()?;
            let ideal = self.ideal.take()?;
            let klarna = self.klarna.take()?;
            let link = self.link.take()?;
            let paypal = self.paypal.take()?;
            let sepa_debit = self.sepa_debit.take()?;
            let sofort = self.sofort.take()?;
            let type_ = self.type_.take()?;
            let us_bank_account = self.us_bank_account.take()?;

            Some(Self::Out { acss_debit, au_becs_debit, bacs_debit, bancontact, boleto, card, card_present, cashapp, ideal, klarna, link, paypal, sepa_debit, sofort, type_, us_bank_account })
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

    impl ObjectDeser for SetupAttemptPaymentMethodDetails {
        type Builder = SetupAttemptPaymentMethodDetailsBuilder;
    }
};
