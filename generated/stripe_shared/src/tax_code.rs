/// [Tax codes](https://stripe.com/docs/tax/tax-categories) classify goods and services for tax purposes.
///
/// For more details see <<https://stripe.com/docs/api/tax_codes/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxCode {
    /// A detailed description of which types of products the tax code represents.
    pub description: String,
    /// Unique identifier for the object.
    pub id: stripe_shared::TaxCodeId,
    /// A short name for the tax code.
    pub name: String,
}
#[cfg(feature = "min-ser")]
pub struct TaxCodeBuilder {
    description: Option<String>,
    id: Option<stripe_shared::TaxCodeId>,
    name: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxCode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxCode>,
        builder: TaxCodeBuilder,
    }

    impl Visitor for Place<TaxCode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxCodeBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxCodeBuilder {
        type Out = TaxCode;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "name" => Ok(Deserialize::begin(&mut self.name)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { description: Deserialize::default(), id: Deserialize::default(), name: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let description = self.description.take()?;
            let id = self.id.take()?;
            let name = self.name.take()?;

            Some(Self::Out { description, id, name })
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

    impl ObjectDeser for TaxCode {
        type Builder = TaxCodeBuilder;
    }
};
impl stripe_types::Object for TaxCode {
    type Id = stripe_shared::TaxCodeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TaxCodeId, "txcd_");
