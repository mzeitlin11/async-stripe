#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceTypeCardPresent {
    pub application_cryptogram: Option<String>,
    pub application_preferred_name: Option<String>,
    pub authorization_code: Option<String>,
    pub authorization_response_code: Option<String>,
    pub brand: Option<String>,
    pub country: Option<String>,
    pub cvm_type: Option<String>,
    pub data_type: Option<String>,
    pub dedicated_file_name: Option<String>,
    pub description: Option<String>,
    pub emv_auth_data: Option<String>,
    pub evidence_customer_signature: Option<String>,
    pub evidence_transaction_certificate: Option<String>,
    pub exp_month: Option<i64>,
    pub exp_year: Option<i64>,
    pub fingerprint: Option<String>,
    pub funding: Option<String>,
    pub iin: Option<String>,
    pub issuer: Option<String>,
    pub last4: Option<String>,
    pub pos_device_id: Option<String>,
    pub pos_entry_mode: Option<String>,
    pub read_method: Option<String>,
    pub reader: Option<String>,
    pub terminal_verification_results: Option<String>,
    pub transaction_status_information: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct SourceTypeCardPresentBuilder {
    application_cryptogram: Option<Option<String>>,
    application_preferred_name: Option<Option<String>>,
    authorization_code: Option<Option<String>>,
    authorization_response_code: Option<Option<String>>,
    brand: Option<Option<String>>,
    country: Option<Option<String>>,
    cvm_type: Option<Option<String>>,
    data_type: Option<Option<String>>,
    dedicated_file_name: Option<Option<String>>,
    description: Option<Option<String>>,
    emv_auth_data: Option<Option<String>>,
    evidence_customer_signature: Option<Option<String>>,
    evidence_transaction_certificate: Option<Option<String>>,
    exp_month: Option<Option<i64>>,
    exp_year: Option<Option<i64>>,
    fingerprint: Option<Option<String>>,
    funding: Option<Option<String>>,
    iin: Option<Option<String>>,
    issuer: Option<Option<String>>,
    last4: Option<Option<String>>,
    pos_device_id: Option<Option<String>>,
    pos_entry_mode: Option<Option<String>>,
    read_method: Option<Option<String>>,
    reader: Option<Option<String>>,
    terminal_verification_results: Option<Option<String>>,
    transaction_status_information: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTypeCardPresent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeCardPresent>,
        builder: SourceTypeCardPresentBuilder,
    }

    impl Visitor for Place<SourceTypeCardPresent> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceTypeCardPresentBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceTypeCardPresentBuilder {
        type Out = SourceTypeCardPresent;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "application_cryptogram" => Ok(Deserialize::begin(&mut self.application_cryptogram)),
                "application_preferred_name" => Ok(Deserialize::begin(&mut self.application_preferred_name)),
                "authorization_code" => Ok(Deserialize::begin(&mut self.authorization_code)),
                "authorization_response_code" => Ok(Deserialize::begin(&mut self.authorization_response_code)),
                "brand" => Ok(Deserialize::begin(&mut self.brand)),
                "country" => Ok(Deserialize::begin(&mut self.country)),
                "cvm_type" => Ok(Deserialize::begin(&mut self.cvm_type)),
                "data_type" => Ok(Deserialize::begin(&mut self.data_type)),
                "dedicated_file_name" => Ok(Deserialize::begin(&mut self.dedicated_file_name)),
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "emv_auth_data" => Ok(Deserialize::begin(&mut self.emv_auth_data)),
                "evidence_customer_signature" => Ok(Deserialize::begin(&mut self.evidence_customer_signature)),
                "evidence_transaction_certificate" => Ok(Deserialize::begin(&mut self.evidence_transaction_certificate)),
                "exp_month" => Ok(Deserialize::begin(&mut self.exp_month)),
                "exp_year" => Ok(Deserialize::begin(&mut self.exp_year)),
                "fingerprint" => Ok(Deserialize::begin(&mut self.fingerprint)),
                "funding" => Ok(Deserialize::begin(&mut self.funding)),
                "iin" => Ok(Deserialize::begin(&mut self.iin)),
                "issuer" => Ok(Deserialize::begin(&mut self.issuer)),
                "last4" => Ok(Deserialize::begin(&mut self.last4)),
                "pos_device_id" => Ok(Deserialize::begin(&mut self.pos_device_id)),
                "pos_entry_mode" => Ok(Deserialize::begin(&mut self.pos_entry_mode)),
                "read_method" => Ok(Deserialize::begin(&mut self.read_method)),
                "reader" => Ok(Deserialize::begin(&mut self.reader)),
                "terminal_verification_results" => Ok(Deserialize::begin(&mut self.terminal_verification_results)),
                "transaction_status_information" => Ok(Deserialize::begin(&mut self.transaction_status_information)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                application_cryptogram: Deserialize::default(),
                application_preferred_name: Deserialize::default(),
                authorization_code: Deserialize::default(),
                authorization_response_code: Deserialize::default(),
                brand: Deserialize::default(),
                country: Deserialize::default(),
                cvm_type: Deserialize::default(),
                data_type: Deserialize::default(),
                dedicated_file_name: Deserialize::default(),
                description: Deserialize::default(),
                emv_auth_data: Deserialize::default(),
                evidence_customer_signature: Deserialize::default(),
                evidence_transaction_certificate: Deserialize::default(),
                exp_month: Deserialize::default(),
                exp_year: Deserialize::default(),
                fingerprint: Deserialize::default(),
                funding: Deserialize::default(),
                iin: Deserialize::default(),
                issuer: Deserialize::default(),
                last4: Deserialize::default(),
                pos_device_id: Deserialize::default(),
                pos_entry_mode: Deserialize::default(),
                read_method: Deserialize::default(),
                reader: Deserialize::default(),
                terminal_verification_results: Deserialize::default(),
                transaction_status_information: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let application_cryptogram = self.application_cryptogram.take()?;
            let application_preferred_name = self.application_preferred_name.take()?;
            let authorization_code = self.authorization_code.take()?;
            let authorization_response_code = self.authorization_response_code.take()?;
            let brand = self.brand.take()?;
            let country = self.country.take()?;
            let cvm_type = self.cvm_type.take()?;
            let data_type = self.data_type.take()?;
            let dedicated_file_name = self.dedicated_file_name.take()?;
            let description = self.description.take()?;
            let emv_auth_data = self.emv_auth_data.take()?;
            let evidence_customer_signature = self.evidence_customer_signature.take()?;
            let evidence_transaction_certificate = self.evidence_transaction_certificate.take()?;
            let exp_month = self.exp_month.take()?;
            let exp_year = self.exp_year.take()?;
            let fingerprint = self.fingerprint.take()?;
            let funding = self.funding.take()?;
            let iin = self.iin.take()?;
            let issuer = self.issuer.take()?;
            let last4 = self.last4.take()?;
            let pos_device_id = self.pos_device_id.take()?;
            let pos_entry_mode = self.pos_entry_mode.take()?;
            let read_method = self.read_method.take()?;
            let reader = self.reader.take()?;
            let terminal_verification_results = self.terminal_verification_results.take()?;
            let transaction_status_information = self.transaction_status_information.take()?;

            Some(Self::Out {
                application_cryptogram,
                application_preferred_name,
                authorization_code,
                authorization_response_code,
                brand,
                country,
                cvm_type,
                data_type,
                dedicated_file_name,
                description,
                emv_auth_data,
                evidence_customer_signature,
                evidence_transaction_certificate,
                exp_month,
                exp_year,
                fingerprint,
                funding,
                iin,
                issuer,
                last4,
                pos_device_id,
                pos_entry_mode,
                read_method,
                reader,
                terminal_verification_results,
                transaction_status_information,
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

    impl ObjectDeser for SourceTypeCardPresent {
        type Builder = SourceTypeCardPresentBuilder;
    }
};
