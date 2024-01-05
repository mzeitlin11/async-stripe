#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct MandatePaymentMethodDetails {
    pub acss_debit: Option<stripe_shared::MandateAcssDebit>,
    pub au_becs_debit: Option<stripe_shared::MandateAuBecsDebit>,
    pub bacs_debit: Option<stripe_shared::MandateBacsDebit>,
    pub card: Option<stripe_shared::CardMandatePaymentMethodDetails>,
    pub cashapp: Option<stripe_shared::MandateCashapp>,
    pub link: Option<stripe_shared::MandateLink>,
    pub paypal: Option<stripe_shared::MandatePaypal>,
    pub sepa_debit: Option<stripe_shared::MandateSepaDebit>,
    /// This mandate corresponds with a specific payment method type.
    /// The `payment_method_details` includes an additional hash with the same name and contains mandate information that's specific to that payment method.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: String,
    pub us_bank_account: Option<stripe_shared::MandateUsBankAccount>,
}
#[cfg(feature = "min-ser")]
pub struct MandatePaymentMethodDetailsBuilder {
    acss_debit: Option<Option<stripe_shared::MandateAcssDebit>>,
    au_becs_debit: Option<Option<stripe_shared::MandateAuBecsDebit>>,
    bacs_debit: Option<Option<stripe_shared::MandateBacsDebit>>,
    card: Option<Option<stripe_shared::CardMandatePaymentMethodDetails>>,
    cashapp: Option<Option<stripe_shared::MandateCashapp>>,
    link: Option<Option<stripe_shared::MandateLink>>,
    paypal: Option<Option<stripe_shared::MandatePaypal>>,
    sepa_debit: Option<Option<stripe_shared::MandateSepaDebit>>,
    type_: Option<String>,
    us_bank_account: Option<Option<stripe_shared::MandateUsBankAccount>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for MandatePaymentMethodDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<MandatePaymentMethodDetails>,
        builder: MandatePaymentMethodDetailsBuilder,
    }

    impl Visitor for Place<MandatePaymentMethodDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: MandatePaymentMethodDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for MandatePaymentMethodDetailsBuilder {
        type Out = MandatePaymentMethodDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "acss_debit" => Ok(Deserialize::begin(&mut self.acss_debit)),
                "au_becs_debit" => Ok(Deserialize::begin(&mut self.au_becs_debit)),
                "bacs_debit" => Ok(Deserialize::begin(&mut self.bacs_debit)),
                "card" => Ok(Deserialize::begin(&mut self.card)),
                "cashapp" => Ok(Deserialize::begin(&mut self.cashapp)),
                "link" => Ok(Deserialize::begin(&mut self.link)),
                "paypal" => Ok(Deserialize::begin(&mut self.paypal)),
                "sepa_debit" => Ok(Deserialize::begin(&mut self.sepa_debit)),
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
                card: Deserialize::default(),
                cashapp: Deserialize::default(),
                link: Deserialize::default(),
                paypal: Deserialize::default(),
                sepa_debit: Deserialize::default(),
                type_: Deserialize::default(),
                us_bank_account: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let acss_debit = self.acss_debit.take()?;
            let au_becs_debit = self.au_becs_debit.take()?;
            let bacs_debit = self.bacs_debit.take()?;
            let card = self.card.take()?;
            let cashapp = self.cashapp.take()?;
            let link = self.link.take()?;
            let paypal = self.paypal.take()?;
            let sepa_debit = self.sepa_debit.take()?;
            let type_ = self.type_.take()?;
            let us_bank_account = self.us_bank_account.take()?;

            Some(Self::Out { acss_debit, au_becs_debit, bacs_debit, card, cashapp, link, paypal, sepa_debit, type_, us_bank_account })
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

    impl ObjectDeser for MandatePaymentMethodDetails {
        type Builder = MandatePaymentMethodDetailsBuilder;
    }
};
