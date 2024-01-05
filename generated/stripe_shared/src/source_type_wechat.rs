#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceTypeWechat {
    pub prepay_id: Option<String>,
    pub qr_code_url: Option<String>,
    pub statement_descriptor: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SourceTypeWechatBuilder {
    prepay_id: Option<Option<String>>,
    qr_code_url: Option<Option<String>>,
    statement_descriptor: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTypeWechat {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeWechat>,
        builder: SourceTypeWechatBuilder,
    }

    impl Visitor for Place<SourceTypeWechat> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceTypeWechatBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceTypeWechatBuilder {
        type Out = SourceTypeWechat;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "prepay_id" => Ok(Deserialize::begin(&mut self.prepay_id)),
                "qr_code_url" => Ok(Deserialize::begin(&mut self.qr_code_url)),
                "statement_descriptor" => Ok(Deserialize::begin(&mut self.statement_descriptor)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { prepay_id: Deserialize::default(), qr_code_url: Deserialize::default(), statement_descriptor: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let prepay_id = self.prepay_id.take()?;
            let qr_code_url = self.qr_code_url.take()?;
            let statement_descriptor = self.statement_descriptor.take()?;

            Some(Self::Out { prepay_id, qr_code_url, statement_descriptor })
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

    impl ObjectDeser for SourceTypeWechat {
        type Builder = SourceTypeWechatBuilder;
    }
};
