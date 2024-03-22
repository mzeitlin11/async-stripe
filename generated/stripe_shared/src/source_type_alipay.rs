#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceTypeAlipay {
    pub data_string: Option<String>,
    pub native_url: Option<String>,
    pub statement_descriptor: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SourceTypeAlipayBuilder {
    data_string: Option<Option<String>>,
    native_url: Option<Option<String>>,
    statement_descriptor: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTypeAlipay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeAlipay>,
        builder: SourceTypeAlipayBuilder,
    }

    impl Visitor for Place<SourceTypeAlipay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceTypeAlipayBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceTypeAlipayBuilder {
        type Out = SourceTypeAlipay;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "data_string" => Ok(Deserialize::begin(&mut self.data_string)),
                "native_url" => Ok(Deserialize::begin(&mut self.native_url)),
                "statement_descriptor" => Ok(Deserialize::begin(&mut self.statement_descriptor)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { data_string: Deserialize::default(), native_url: Deserialize::default(), statement_descriptor: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let data_string = self.data_string.take()?;
            let native_url = self.native_url.take()?;
            let statement_descriptor = self.statement_descriptor.take()?;

            Some(Self::Out { data_string, native_url, statement_descriptor })
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

    impl ObjectDeser for SourceTypeAlipay {
        type Builder = SourceTypeAlipayBuilder;
    }
};
