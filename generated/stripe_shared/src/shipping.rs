#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Shipping {
    pub address: Option<stripe_shared::Address>,
    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    pub carrier: Option<String>,
    /// Recipient name.
    pub name: Option<String>,
    /// Recipient phone (including extension).
    pub phone: Option<String>,
    /// The tracking number for a physical product, obtained from the delivery service.
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    pub tracking_number: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct ShippingBuilder {
    address: Option<Option<stripe_shared::Address>>,
    carrier: Option<Option<String>>,
    name: Option<Option<String>>,
    phone: Option<Option<String>>,
    tracking_number: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Shipping {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Shipping>,
        builder: ShippingBuilder,
    }

    impl Visitor for Place<Shipping> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ShippingBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ShippingBuilder {
        type Out = Shipping;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "address" => Ok(Deserialize::begin(&mut self.address)),
                "carrier" => Ok(Deserialize::begin(&mut self.carrier)),
                "name" => Ok(Deserialize::begin(&mut self.name)),
                "phone" => Ok(Deserialize::begin(&mut self.phone)),
                "tracking_number" => Ok(Deserialize::begin(&mut self.tracking_number)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { address: Deserialize::default(), carrier: Deserialize::default(), name: Deserialize::default(), phone: Deserialize::default(), tracking_number: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let address = self.address.take()?;
            let carrier = self.carrier.take()?;
            let name = self.name.take()?;
            let phone = self.phone.take()?;
            let tracking_number = self.tracking_number.take()?;

            Some(Self::Out { address, carrier, name, phone, tracking_number })
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

    impl ObjectDeser for Shipping {
        type Builder = ShippingBuilder;
    }
};
