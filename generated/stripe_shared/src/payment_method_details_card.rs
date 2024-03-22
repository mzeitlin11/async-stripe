#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetailsCard {
    /// The authorized amount.
    pub amount_authorized: Option<i64>,
    /// Card brand.
    /// Can be `amex`, `diners`, `discover`, `eftpos_au`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub brand: Option<String>,
    /// When using manual capture, a future timestamp at which the charge will be automatically refunded if uncaptured.
    pub capture_before: Option<stripe_types::Timestamp>,
    /// Check results by Card networks on Card address and CVC at time of payment.
    pub checks: Option<stripe_shared::PaymentMethodDetailsCardChecks>,
    /// Two-letter ISO code representing the country of the card.
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,
    /// A high-level description of the type of cards issued in this range.
    /// (For internal use only and not typically available in standard API requests.).
    pub description: Option<String>,
    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,
    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,
    pub extended_authorization: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorization>,
    /// Uniquely identifies this particular card number.
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.
    ///
    /// *As of May 1, 2021, card fingerprint in India for Connect changed to allow two fingerprints for the same card---one for India and one for the rest of the world.*.
    pub fingerprint: Option<String>,
    /// Card funding type. Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: Option<String>,
    /// Issuer identification number of the card.
    /// (For internal use only and not typically available in standard API requests.).
    pub iin: Option<String>,
    pub incremental_authorization: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorization>,
    /// Installment details for this payment (Mexico only).
    ///
    /// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
    pub installments: Option<stripe_shared::PaymentMethodDetailsCardInstallments>,
    /// The name of the card's issuing bank.
    /// (For internal use only and not typically available in standard API requests.).
    pub issuer: Option<String>,
    /// The last four digits of the card.
    pub last4: Option<String>,
    /// ID of the mandate used to make this payment or created by it.
    pub mandate: Option<String>,
    /// True if this payment was marked as MOTO and out of scope for SCA.
    pub moto: Option<bool>,
    pub multicapture: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture>,
    /// Identifies which network this charge was processed on.
    /// Can be `amex`, `cartes_bancaires`, `diners`, `discover`, `eftpos_au`, `interac`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub network: Option<String>,
    /// If this card has network token credentials, this contains the details of the network token credentials.
    pub network_token: Option<stripe_shared::PaymentMethodDetailsCardNetworkToken>,
    pub overcapture: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercapture>,
    /// Populated if this transaction used 3D Secure authentication.
    pub three_d_secure: Option<stripe_shared::ThreeDSecureDetailsCharge>,
    /// If this Card is part of a card wallet, this contains the details of the card wallet.
    pub wallet: Option<stripe_shared::PaymentMethodDetailsCardWallet>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDetailsCardBuilder {
    amount_authorized: Option<Option<i64>>,
    brand: Option<Option<String>>,
    capture_before: Option<Option<stripe_types::Timestamp>>,
    checks: Option<Option<stripe_shared::PaymentMethodDetailsCardChecks>>,
    country: Option<Option<String>>,
    description: Option<Option<String>>,
    exp_month: Option<i64>,
    exp_year: Option<i64>,
    extended_authorization: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorization>>,
    fingerprint: Option<Option<String>>,
    funding: Option<Option<String>>,
    iin: Option<Option<String>>,
    incremental_authorization: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorization>>,
    installments: Option<Option<stripe_shared::PaymentMethodDetailsCardInstallments>>,
    issuer: Option<Option<String>>,
    last4: Option<Option<String>>,
    mandate: Option<Option<String>>,
    moto: Option<Option<bool>>,
    multicapture: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture>>,
    network: Option<Option<String>>,
    network_token: Option<Option<stripe_shared::PaymentMethodDetailsCardNetworkToken>>,
    overcapture: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercapture>>,
    three_d_secure: Option<Option<stripe_shared::ThreeDSecureDetailsCharge>>,
    wallet: Option<Option<stripe_shared::PaymentMethodDetailsCardWallet>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsCard>,
        builder: PaymentMethodDetailsCardBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDetailsCardBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsCardBuilder {
        type Out = PaymentMethodDetailsCard;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "amount_authorized" => Ok(Deserialize::begin(&mut self.amount_authorized)),
                "brand" => Ok(Deserialize::begin(&mut self.brand)),
                "capture_before" => Ok(Deserialize::begin(&mut self.capture_before)),
                "checks" => Ok(Deserialize::begin(&mut self.checks)),
                "country" => Ok(Deserialize::begin(&mut self.country)),
                "description" => Ok(Deserialize::begin(&mut self.description)),
                "exp_month" => Ok(Deserialize::begin(&mut self.exp_month)),
                "exp_year" => Ok(Deserialize::begin(&mut self.exp_year)),
                "extended_authorization" => Ok(Deserialize::begin(&mut self.extended_authorization)),
                "fingerprint" => Ok(Deserialize::begin(&mut self.fingerprint)),
                "funding" => Ok(Deserialize::begin(&mut self.funding)),
                "iin" => Ok(Deserialize::begin(&mut self.iin)),
                "incremental_authorization" => Ok(Deserialize::begin(&mut self.incremental_authorization)),
                "installments" => Ok(Deserialize::begin(&mut self.installments)),
                "issuer" => Ok(Deserialize::begin(&mut self.issuer)),
                "last4" => Ok(Deserialize::begin(&mut self.last4)),
                "mandate" => Ok(Deserialize::begin(&mut self.mandate)),
                "moto" => Ok(Deserialize::begin(&mut self.moto)),
                "multicapture" => Ok(Deserialize::begin(&mut self.multicapture)),
                "network" => Ok(Deserialize::begin(&mut self.network)),
                "network_token" => Ok(Deserialize::begin(&mut self.network_token)),
                "overcapture" => Ok(Deserialize::begin(&mut self.overcapture)),
                "three_d_secure" => Ok(Deserialize::begin(&mut self.three_d_secure)),
                "wallet" => Ok(Deserialize::begin(&mut self.wallet)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                amount_authorized: Deserialize::default(),
                brand: Deserialize::default(),
                capture_before: Deserialize::default(),
                checks: Deserialize::default(),
                country: Deserialize::default(),
                description: Deserialize::default(),
                exp_month: Deserialize::default(),
                exp_year: Deserialize::default(),
                extended_authorization: Deserialize::default(),
                fingerprint: Deserialize::default(),
                funding: Deserialize::default(),
                iin: Deserialize::default(),
                incremental_authorization: Deserialize::default(),
                installments: Deserialize::default(),
                issuer: Deserialize::default(),
                last4: Deserialize::default(),
                mandate: Deserialize::default(),
                moto: Deserialize::default(),
                multicapture: Deserialize::default(),
                network: Deserialize::default(),
                network_token: Deserialize::default(),
                overcapture: Deserialize::default(),
                three_d_secure: Deserialize::default(),
                wallet: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let amount_authorized = self.amount_authorized.take()?;
            let brand = self.brand.take()?;
            let capture_before = self.capture_before.take()?;
            let checks = self.checks.take()?;
            let country = self.country.take()?;
            let description = self.description.take()?;
            let exp_month = self.exp_month.take()?;
            let exp_year = self.exp_year.take()?;
            let extended_authorization = self.extended_authorization.take()?;
            let fingerprint = self.fingerprint.take()?;
            let funding = self.funding.take()?;
            let iin = self.iin.take()?;
            let incremental_authorization = self.incremental_authorization.take()?;
            let installments = self.installments.take()?;
            let issuer = self.issuer.take()?;
            let last4 = self.last4.take()?;
            let mandate = self.mandate.take()?;
            let moto = self.moto.take()?;
            let multicapture = self.multicapture.take()?;
            let network = self.network.take()?;
            let network_token = self.network_token.take()?;
            let overcapture = self.overcapture.take()?;
            let three_d_secure = self.three_d_secure.take()?;
            let wallet = self.wallet.take()?;

            Some(Self::Out {
                amount_authorized,
                brand,
                capture_before,
                checks,
                country,
                description,
                exp_month,
                exp_year,
                extended_authorization,
                fingerprint,
                funding,
                iin,
                incremental_authorization,
                installments,
                issuer,
                last4,
                mandate,
                moto,
                multicapture,
                network,
                network_token,
                overcapture,
                three_d_secure,
                wallet,
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

    impl ObjectDeser for PaymentMethodDetailsCard {
        type Builder = PaymentMethodDetailsCardBuilder;
    }
};
