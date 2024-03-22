#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SubscriptionsResourcePaymentMethodOptions {
    /// This sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to invoices created by the subscription.
    pub acss_debit: Option<stripe_shared::InvoicePaymentMethodOptionsAcssDebit>,
    /// This sub-hash contains details about the Bancontact payment method options to pass to invoices created by the subscription.
    pub bancontact: Option<stripe_shared::InvoicePaymentMethodOptionsBancontact>,
    /// This sub-hash contains details about the Card payment method options to pass to invoices created by the subscription.
    pub card: Option<stripe_shared::SubscriptionPaymentMethodOptionsCard>,
    /// This sub-hash contains details about the Bank transfer payment method options to pass to invoices created by the subscription.
    pub customer_balance: Option<stripe_shared::InvoicePaymentMethodOptionsCustomerBalance>,
    /// This sub-hash contains details about the Konbini payment method options to pass to invoices created by the subscription.
    pub konbini: Option<stripe_shared::InvoicePaymentMethodOptionsKonbini>,
    /// This sub-hash contains details about the ACH direct debit payment method options to pass to invoices created by the subscription.
    pub us_bank_account: Option<stripe_shared::InvoicePaymentMethodOptionsUsBankAccount>,
}
#[cfg(feature = "min-ser")]
pub struct SubscriptionsResourcePaymentMethodOptionsBuilder {
    acss_debit: Option<Option<stripe_shared::InvoicePaymentMethodOptionsAcssDebit>>,
    bancontact: Option<Option<stripe_shared::InvoicePaymentMethodOptionsBancontact>>,
    card: Option<Option<stripe_shared::SubscriptionPaymentMethodOptionsCard>>,
    customer_balance: Option<Option<stripe_shared::InvoicePaymentMethodOptionsCustomerBalance>>,
    konbini: Option<Option<stripe_shared::InvoicePaymentMethodOptionsKonbini>>,
    us_bank_account: Option<Option<stripe_shared::InvoicePaymentMethodOptionsUsBankAccount>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SubscriptionsResourcePaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsResourcePaymentMethodOptions>,
        builder: SubscriptionsResourcePaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<SubscriptionsResourcePaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SubscriptionsResourcePaymentMethodOptionsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SubscriptionsResourcePaymentMethodOptionsBuilder {
        type Out = SubscriptionsResourcePaymentMethodOptions;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "acss_debit" => Ok(Deserialize::begin(&mut self.acss_debit)),
                "bancontact" => Ok(Deserialize::begin(&mut self.bancontact)),
                "card" => Ok(Deserialize::begin(&mut self.card)),
                "customer_balance" => Ok(Deserialize::begin(&mut self.customer_balance)),
                "konbini" => Ok(Deserialize::begin(&mut self.konbini)),
                "us_bank_account" => Ok(Deserialize::begin(&mut self.us_bank_account)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                acss_debit: Deserialize::default(),
                bancontact: Deserialize::default(),
                card: Deserialize::default(),
                customer_balance: Deserialize::default(),
                konbini: Deserialize::default(),
                us_bank_account: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let acss_debit = self.acss_debit.take()?;
            let bancontact = self.bancontact.take()?;
            let card = self.card.take()?;
            let customer_balance = self.customer_balance.take()?;
            let konbini = self.konbini.take()?;
            let us_bank_account = self.us_bank_account.take()?;

            Some(Self::Out { acss_debit, bancontact, card, customer_balance, konbini, us_bank_account })
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

    impl ObjectDeser for SubscriptionsResourcePaymentMethodOptions {
        type Builder = SubscriptionsResourcePaymentMethodOptionsBuilder;
    }
};
