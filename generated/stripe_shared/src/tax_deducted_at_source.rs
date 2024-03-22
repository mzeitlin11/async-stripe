#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxDeductedAtSource {
    /// Unique identifier for the object.
    pub id: stripe_shared::TaxDeductedAtSourceId,
    /// The end of the invoicing period.
    /// This TDS applies to Stripe fees collected during this invoicing period.
    pub period_end: stripe_types::Timestamp,
    /// The start of the invoicing period.
    /// This TDS applies to Stripe fees collected during this invoicing period.
    pub period_start: stripe_types::Timestamp,
    /// The TAN that was supplied to Stripe when TDS was assessed
    pub tax_deduction_account_number: String,
}
#[cfg(feature = "min-ser")]
pub struct TaxDeductedAtSourceBuilder {
    id: Option<stripe_shared::TaxDeductedAtSourceId>,
    period_end: Option<stripe_types::Timestamp>,
    period_start: Option<stripe_types::Timestamp>,
    tax_deduction_account_number: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxDeductedAtSource {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxDeductedAtSource>,
        builder: TaxDeductedAtSourceBuilder,
    }

    impl Visitor for Place<TaxDeductedAtSource> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxDeductedAtSourceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxDeductedAtSourceBuilder {
        type Out = TaxDeductedAtSource;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "period_end" => Ok(Deserialize::begin(&mut self.period_end)),
                "period_start" => Ok(Deserialize::begin(&mut self.period_start)),
                "tax_deduction_account_number" => Ok(Deserialize::begin(&mut self.tax_deduction_account_number)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { id: Deserialize::default(), period_end: Deserialize::default(), period_start: Deserialize::default(), tax_deduction_account_number: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let id = self.id.take()?;
            let period_end = self.period_end.take()?;
            let period_start = self.period_start.take()?;
            let tax_deduction_account_number = self.tax_deduction_account_number.take()?;

            Some(Self::Out { id, period_end, period_start, tax_deduction_account_number })
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

    impl ObjectDeser for TaxDeductedAtSource {
        type Builder = TaxDeductedAtSourceBuilder;
    }
};
impl stripe_types::Object for TaxDeductedAtSource {
    type Id = stripe_shared::TaxDeductedAtSourceId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TaxDeductedAtSourceId, "itds_");
