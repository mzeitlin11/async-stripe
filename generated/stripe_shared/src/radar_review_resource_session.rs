#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct RadarReviewResourceSession {
    /// The browser used in this browser session (e.g., `Chrome`).
    pub browser: Option<String>,
    /// Information about the device used for the browser session (e.g., `Samsung SM-G930T`).
    pub device: Option<String>,
    /// The platform for the browser session (e.g., `Macintosh`).
    pub platform: Option<String>,
    /// The version for the browser session (e.g., `61.0.3163.100`).
    pub version: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct RadarReviewResourceSessionBuilder {
    browser: Option<Option<String>>,
    device: Option<Option<String>>,
    platform: Option<Option<String>>,
    version: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for RadarReviewResourceSession {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RadarReviewResourceSession>,
        builder: RadarReviewResourceSessionBuilder,
    }

    impl Visitor for Place<RadarReviewResourceSession> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: RadarReviewResourceSessionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for RadarReviewResourceSessionBuilder {
        type Out = RadarReviewResourceSession;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "browser" => Ok(Deserialize::begin(&mut self.browser)),
                "device" => Ok(Deserialize::begin(&mut self.device)),
                "platform" => Ok(Deserialize::begin(&mut self.platform)),
                "version" => Ok(Deserialize::begin(&mut self.version)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { browser: Deserialize::default(), device: Deserialize::default(), platform: Deserialize::default(), version: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let browser = self.browser.take()?;
            let device = self.device.take()?;
            let platform = self.platform.take()?;
            let version = self.version.take()?;

            Some(Self::Out { browser, device, platform, version })
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

    impl ObjectDeser for RadarReviewResourceSession {
        type Builder = RadarReviewResourceSessionBuilder;
    }
};
