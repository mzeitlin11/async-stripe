#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceTypeMultibanco {
    pub entity: Option<String>,
    pub reference: Option<String>,
    pub refund_account_holder_address_city: Option<String>,
    pub refund_account_holder_address_country: Option<String>,
    pub refund_account_holder_address_line1: Option<String>,
    pub refund_account_holder_address_line2: Option<String>,
    pub refund_account_holder_address_postal_code: Option<String>,
    pub refund_account_holder_address_state: Option<String>,
    pub refund_account_holder_name: Option<String>,
    pub refund_iban: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SourceTypeMultibancoBuilder {
    entity: Option<Option<String>>,
    reference: Option<Option<String>>,
    refund_account_holder_address_city: Option<Option<String>>,
    refund_account_holder_address_country: Option<Option<String>>,
    refund_account_holder_address_line1: Option<Option<String>>,
    refund_account_holder_address_line2: Option<Option<String>>,
    refund_account_holder_address_postal_code: Option<Option<String>>,
    refund_account_holder_address_state: Option<Option<String>>,
    refund_account_holder_name: Option<Option<String>>,
    refund_iban: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTypeMultibanco {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeMultibanco>,
        builder: SourceTypeMultibancoBuilder,
    }

    impl Visitor for Place<SourceTypeMultibanco> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceTypeMultibancoBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceTypeMultibancoBuilder {
        type Out = SourceTypeMultibanco;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "entity" => Ok(Deserialize::begin(&mut self.entity)),
                "reference" => Ok(Deserialize::begin(&mut self.reference)),
                "refund_account_holder_address_city" => Ok(Deserialize::begin(&mut self.refund_account_holder_address_city)),
                "refund_account_holder_address_country" => Ok(Deserialize::begin(&mut self.refund_account_holder_address_country)),
                "refund_account_holder_address_line1" => Ok(Deserialize::begin(&mut self.refund_account_holder_address_line1)),
                "refund_account_holder_address_line2" => Ok(Deserialize::begin(&mut self.refund_account_holder_address_line2)),
                "refund_account_holder_address_postal_code" => Ok(Deserialize::begin(&mut self.refund_account_holder_address_postal_code)),
                "refund_account_holder_address_state" => Ok(Deserialize::begin(&mut self.refund_account_holder_address_state)),
                "refund_account_holder_name" => Ok(Deserialize::begin(&mut self.refund_account_holder_name)),
                "refund_iban" => Ok(Deserialize::begin(&mut self.refund_iban)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                entity: Deserialize::default(),
                reference: Deserialize::default(),
                refund_account_holder_address_city: Deserialize::default(),
                refund_account_holder_address_country: Deserialize::default(),
                refund_account_holder_address_line1: Deserialize::default(),
                refund_account_holder_address_line2: Deserialize::default(),
                refund_account_holder_address_postal_code: Deserialize::default(),
                refund_account_holder_address_state: Deserialize::default(),
                refund_account_holder_name: Deserialize::default(),
                refund_iban: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let entity = self.entity.take()?;
            let reference = self.reference.take()?;
            let refund_account_holder_address_city = self.refund_account_holder_address_city.take()?;
            let refund_account_holder_address_country = self.refund_account_holder_address_country.take()?;
            let refund_account_holder_address_line1 = self.refund_account_holder_address_line1.take()?;
            let refund_account_holder_address_line2 = self.refund_account_holder_address_line2.take()?;
            let refund_account_holder_address_postal_code = self.refund_account_holder_address_postal_code.take()?;
            let refund_account_holder_address_state = self.refund_account_holder_address_state.take()?;
            let refund_account_holder_name = self.refund_account_holder_name.take()?;
            let refund_iban = self.refund_iban.take()?;

            Some(Self::Out {
                entity,
                reference,
                refund_account_holder_address_city,
                refund_account_holder_address_country,
                refund_account_holder_address_line1,
                refund_account_holder_address_line2,
                refund_account_holder_address_postal_code,
                refund_account_holder_address_state,
                refund_account_holder_name,
                refund_iban,
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

    impl ObjectDeser for SourceTypeMultibanco {
        type Builder = SourceTypeMultibancoBuilder;
    }
};
