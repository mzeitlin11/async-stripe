#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingTransactionPurchaseDetails {
    /// Information about the flight that was purchased with this transaction.
    pub flight: Option<stripe_shared::IssuingTransactionFlightData>,
    /// Information about fuel that was purchased with this transaction.
    pub fuel: Option<stripe_shared::IssuingTransactionFuelData>,
    /// Information about lodging that was purchased with this transaction.
    pub lodging: Option<stripe_shared::IssuingTransactionLodgingData>,
    /// The line items in the purchase.
    pub receipt: Option<Vec<stripe_shared::IssuingTransactionReceiptData>>,
    /// A merchant-specific order number.
    pub reference: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingTransactionPurchaseDetailsBuilder {
    flight: Option<Option<stripe_shared::IssuingTransactionFlightData>>,
    fuel: Option<Option<stripe_shared::IssuingTransactionFuelData>>,
    lodging: Option<Option<stripe_shared::IssuingTransactionLodgingData>>,
    receipt: Option<Option<Vec<stripe_shared::IssuingTransactionReceiptData>>>,
    reference: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingTransactionPurchaseDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionPurchaseDetails>,
        builder: IssuingTransactionPurchaseDetailsBuilder,
    }

    impl Visitor for Place<IssuingTransactionPurchaseDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingTransactionPurchaseDetailsBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingTransactionPurchaseDetailsBuilder {
        type Out = IssuingTransactionPurchaseDetails;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "flight" => Ok(Deserialize::begin(&mut self.flight)),
                "fuel" => Ok(Deserialize::begin(&mut self.fuel)),
                "lodging" => Ok(Deserialize::begin(&mut self.lodging)),
                "receipt" => Ok(Deserialize::begin(&mut self.receipt)),
                "reference" => Ok(Deserialize::begin(&mut self.reference)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { flight: Deserialize::default(), fuel: Deserialize::default(), lodging: Deserialize::default(), receipt: Deserialize::default(), reference: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let flight = self.flight.take()?;
            let fuel = self.fuel.take()?;
            let lodging = self.lodging.take()?;
            let receipt = self.receipt.take()?;
            let reference = self.reference.take()?;

            Some(Self::Out { flight, fuel, lodging, receipt, reference })
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

    impl ObjectDeser for IssuingTransactionPurchaseDetails {
        type Builder = IssuingTransactionPurchaseDetailsBuilder;
    }
};
