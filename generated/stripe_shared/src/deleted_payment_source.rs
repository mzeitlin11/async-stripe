/// The resource representing a Stripe Polymorphic
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(not(feature = "min-ser"), serde(tag = "object"))]
pub enum DeletedPaymentSource {
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "bank_account"))]
    DeletedBankAccount(stripe_shared::DeletedBankAccount),
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "card"))]
    DeletedCard(stripe_shared::DeletedCard),
}

#[cfg(feature = "min-ser")]
#[derive(Default)]
pub struct DeletedPaymentSourceBuilder {
    inner: stripe_types::miniserde_helpers::ObjectBuilderInner,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::{from_str, to_string};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::MapBuilder;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<DeletedPaymentSource>,
        builder: DeletedPaymentSourceBuilder,
    }

    impl Deserialize for DeletedPaymentSource {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<DeletedPaymentSource> {
        fn map(&mut self) -> miniserde::Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
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

    impl stripe_types::MapBuilder for DeletedPaymentSourceBuilder {
        type Out = DeletedPaymentSource;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (obj_key, object) = self.inner.finish_inner()?;
            let obj_str = to_string(&object);
            Some(match obj_key.as_str() {
                "bank_account" => DeletedPaymentSource::DeletedBankAccount(from_str(&obj_str).ok()?),
                "card" => DeletedPaymentSource::DeletedCard(from_str(&obj_str).ok()?),

                _ => return None,
            })
        }
    }

    impl stripe_types::ObjectDeser for DeletedPaymentSource {
        type Builder = DeletedPaymentSourceBuilder;
    }
};

impl stripe_types::Object for DeletedPaymentSource {
    type Id = smol_str::SmolStr;
    fn id(&self) -> &Self::Id {
        match self {
            Self::DeletedBankAccount(v) => v.id.inner(),
            Self::DeletedCard(v) => v.id.inner(),
        }
    }
}
