#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IssuingAuthorizationNetworkData {
    /// Identifier assigned to the acquirer by the card network.
    /// Sometimes this value is not provided by the network; in this case, the value will be `null`.
    pub acquiring_institution_id: Option<String>,
    /// The System Trace Audit Number (STAN) is a 6-digit identifier assigned by the acquirer.
    /// Prefer `network_data.transaction_id` if present, unless you have special requirements.
    pub system_trace_audit_number: Option<String>,
    /// Unique identifier for the authorization assigned by the card network used to match subsequent messages, disputes, and transactions.
    pub transaction_id: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct IssuingAuthorizationNetworkDataBuilder {
    acquiring_institution_id: Option<Option<String>>,
    system_trace_audit_number: Option<Option<String>>,
    transaction_id: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingAuthorizationNetworkData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationNetworkData>,
        builder: IssuingAuthorizationNetworkDataBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationNetworkData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: IssuingAuthorizationNetworkDataBuilder::deser_default() }))
        }
    }

    impl MapBuilder for IssuingAuthorizationNetworkDataBuilder {
        type Out = IssuingAuthorizationNetworkData;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "acquiring_institution_id" => Ok(Deserialize::begin(&mut self.acquiring_institution_id)),
                "system_trace_audit_number" => Ok(Deserialize::begin(&mut self.system_trace_audit_number)),
                "transaction_id" => Ok(Deserialize::begin(&mut self.transaction_id)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self { acquiring_institution_id: Deserialize::default(), system_trace_audit_number: Deserialize::default(), transaction_id: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let acquiring_institution_id = self.acquiring_institution_id.take()?;
            let system_trace_audit_number = self.system_trace_audit_number.take()?;
            let transaction_id = self.transaction_id.take()?;

            Some(Self::Out { acquiring_institution_id, system_trace_audit_number, transaction_id })
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

    impl ObjectDeser for IssuingAuthorizationNetworkData {
        type Builder = IssuingAuthorizationNetworkDataBuilder;
    }
};
