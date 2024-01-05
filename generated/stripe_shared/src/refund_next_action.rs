#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct RefundNextAction {
    /// Contains the refund details.
    pub display_details: Option<stripe_shared::RefundNextActionDisplayDetails>,
    /// Type of the next action to perform.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: String,
}
#[cfg(feature = "min-ser")]
pub struct RefundNextActionBuilder {
    display_details: Option<Option<stripe_shared::RefundNextActionDisplayDetails>>,
    type_: Option<String>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for RefundNextAction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RefundNextAction>,
        builder: RefundNextActionBuilder,
    }

    impl Visitor for Place<RefundNextAction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: RefundNextActionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for RefundNextActionBuilder {
        type Out = RefundNextAction;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "display_details" => Ok(Deserialize::begin(&mut self.display_details)),
                "type" => Ok(Deserialize::begin(&mut self.type_)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { display_details: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let display_details = self.display_details.take()?;
            let type_ = self.type_.take()?;

            Some(Self::Out { display_details, type_ })
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

    impl ObjectDeser for RefundNextAction {
        type Builder = RefundNextActionBuilder;
    }
};
