/// Encodes whether a FinancialAccount has access to a particular Feature, with a `status` enum and associated `status_details`.
/// Stripe or the platform can control Features via the requested field.
#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountFeatures {
    pub card_issuing: Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>,
    pub deposit_insurance: Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>,
    pub financial_addresses: Option<stripe_treasury::TreasuryFinancialAccountsResourceFinancialAddressesFeatures>,
    pub inbound_transfers: Option<stripe_treasury::TreasuryFinancialAccountsResourceInboundTransfers>,
    pub intra_stripe_flows: Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>,
    pub outbound_payments: Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundPayments>,
    pub outbound_transfers: Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundTransfers>,
}
#[cfg(feature = "min-ser")]
pub struct TreasuryFinancialAccountFeaturesBuilder {
    card_issuing: Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>>,
    deposit_insurance: Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>>,
    financial_addresses: Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceFinancialAddressesFeatures>>,
    inbound_transfers: Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceInboundTransfers>>,
    intra_stripe_flows: Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>>,
    outbound_payments: Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundPayments>>,
    outbound_transfers: Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundTransfers>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryFinancialAccountFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountFeatures>,
        builder: TreasuryFinancialAccountFeaturesBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountFeatures> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TreasuryFinancialAccountFeaturesBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TreasuryFinancialAccountFeaturesBuilder {
        type Out = TreasuryFinancialAccountFeatures;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "card_issuing" => Ok(Deserialize::begin(&mut self.card_issuing)),
                "deposit_insurance" => Ok(Deserialize::begin(&mut self.deposit_insurance)),
                "financial_addresses" => Ok(Deserialize::begin(&mut self.financial_addresses)),
                "inbound_transfers" => Ok(Deserialize::begin(&mut self.inbound_transfers)),
                "intra_stripe_flows" => Ok(Deserialize::begin(&mut self.intra_stripe_flows)),
                "outbound_payments" => Ok(Deserialize::begin(&mut self.outbound_payments)),
                "outbound_transfers" => Ok(Deserialize::begin(&mut self.outbound_transfers)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                card_issuing: Deserialize::default(),
                deposit_insurance: Deserialize::default(),
                financial_addresses: Deserialize::default(),
                inbound_transfers: Deserialize::default(),
                intra_stripe_flows: Deserialize::default(),
                outbound_payments: Deserialize::default(),
                outbound_transfers: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let card_issuing = self.card_issuing.take()?;
            let deposit_insurance = self.deposit_insurance.take()?;
            let financial_addresses = self.financial_addresses.take()?;
            let inbound_transfers = self.inbound_transfers.take()?;
            let intra_stripe_flows = self.intra_stripe_flows.take()?;
            let outbound_payments = self.outbound_payments.take()?;
            let outbound_transfers = self.outbound_transfers.take()?;

            Some(Self::Out { card_issuing, deposit_insurance, financial_addresses, inbound_transfers, intra_stripe_flows, outbound_payments, outbound_transfers })
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

    impl ObjectDeser for TreasuryFinancialAccountFeatures {
        type Builder = TreasuryFinancialAccountFeaturesBuilder;
    }
};
