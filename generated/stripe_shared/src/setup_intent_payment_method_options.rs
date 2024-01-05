#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptions {
    pub acss_debit: Option<stripe_shared::SetupIntentPaymentMethodOptionsAcssDebit>,
    pub card: Option<stripe_shared::SetupIntentPaymentMethodOptionsCard>,
    pub link: Option<stripe_shared::SetupIntentPaymentMethodOptionsLink>,
    pub paypal: Option<stripe_shared::SetupIntentPaymentMethodOptionsPaypal>,
    pub sepa_debit: Option<stripe_shared::SetupIntentPaymentMethodOptionsSepaDebit>,
    pub us_bank_account: Option<stripe_shared::SetupIntentPaymentMethodOptionsUsBankAccount>,
}
#[cfg(feature = "min-ser")]
pub struct SetupIntentPaymentMethodOptionsBuilder {
    acss_debit: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsAcssDebit>>,
    card: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsCard>>,
    link: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsLink>>,
    paypal: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsPaypal>>,
    sepa_debit: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsSepaDebit>>,
    us_bank_account: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsUsBankAccount>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SetupIntentPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentPaymentMethodOptions>,
        builder: SetupIntentPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<SetupIntentPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SetupIntentPaymentMethodOptionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SetupIntentPaymentMethodOptionsBuilder {
        type Out = SetupIntentPaymentMethodOptions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "acss_debit" => Ok(Deserialize::begin(&mut self.acss_debit)),
                "card" => Ok(Deserialize::begin(&mut self.card)),
                "link" => Ok(Deserialize::begin(&mut self.link)),
                "paypal" => Ok(Deserialize::begin(&mut self.paypal)),
                "sepa_debit" => Ok(Deserialize::begin(&mut self.sepa_debit)),
                "us_bank_account" => Ok(Deserialize::begin(&mut self.us_bank_account)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                acss_debit: Deserialize::default(),
                card: Deserialize::default(),
                link: Deserialize::default(),
                paypal: Deserialize::default(),
                sepa_debit: Deserialize::default(),
                us_bank_account: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let acss_debit = self.acss_debit.take()?;
            let card = self.card.take()?;
            let link = self.link.take()?;
            let paypal = self.paypal.take()?;
            let sepa_debit = self.sepa_debit.take()?;
            let us_bank_account = self.us_bank_account.take()?;

            Some(Self::Out { acss_debit, card, link, paypal, sepa_debit, us_bank_account })
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

    impl ObjectDeser for SetupIntentPaymentMethodOptions {
        type Builder = SetupIntentPaymentMethodOptionsBuilder;
    }
};
