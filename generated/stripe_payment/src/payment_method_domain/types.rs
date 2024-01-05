/// A payment method domain represents a web domain that you have registered with Stripe.
/// Stripe Elements use registered payment method domains to control where certain payment methods are shown.
///
/// Related guides: [Payment method domains](https://stripe.com/docs/payments/payment-methods/pmd-registration).
///
/// For more details see <<https://stripe.com/docs/api/payment_method_domains/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDomain {
    pub apple_pay: stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The domain name that this payment method domain object represents.
    pub domain_name: String,
    /// Whether this payment method domain is enabled.
    /// If the domain is not enabled, payment methods that require a payment method domain will not appear in Elements.
    pub enabled: bool,
    pub google_pay: stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus,
    /// Unique identifier for the object.
    pub id: stripe_payment::PaymentMethodDomainId,
    pub link: stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    pub paypal: stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodDomainBuilder {
    apple_pay: Option<stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus>,
    created: Option<stripe_types::Timestamp>,
    domain_name: Option<String>,
    enabled: Option<bool>,
    google_pay: Option<stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus>,
    id: Option<stripe_payment::PaymentMethodDomainId>,
    link: Option<stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus>,
    livemode: Option<bool>,
    paypal: Option<stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDomain {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDomain>,
        builder: PaymentMethodDomainBuilder,
    }

    impl Visitor for Place<PaymentMethodDomain> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodDomainBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodDomainBuilder {
        type Out = PaymentMethodDomain;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "apple_pay" => Ok(Deserialize::begin(&mut self.apple_pay)),
                "created" => Ok(Deserialize::begin(&mut self.created)),
                "domain_name" => Ok(Deserialize::begin(&mut self.domain_name)),
                "enabled" => Ok(Deserialize::begin(&mut self.enabled)),
                "google_pay" => Ok(Deserialize::begin(&mut self.google_pay)),
                "id" => Ok(Deserialize::begin(&mut self.id)),
                "link" => Ok(Deserialize::begin(&mut self.link)),
                "livemode" => Ok(Deserialize::begin(&mut self.livemode)),
                "paypal" => Ok(Deserialize::begin(&mut self.paypal)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                apple_pay: Deserialize::default(),
                created: Deserialize::default(),
                domain_name: Deserialize::default(),
                enabled: Deserialize::default(),
                google_pay: Deserialize::default(),
                id: Deserialize::default(),
                link: Deserialize::default(),
                livemode: Deserialize::default(),
                paypal: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let apple_pay = self.apple_pay.take()?;
            let created = self.created.take()?;
            let domain_name = self.domain_name.take()?;
            let enabled = self.enabled.take()?;
            let google_pay = self.google_pay.take()?;
            let id = self.id.take()?;
            let link = self.link.take()?;
            let livemode = self.livemode.take()?;
            let paypal = self.paypal.take()?;

            Some(Self::Out { apple_pay, created, domain_name, enabled, google_pay, id, link, livemode, paypal })
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

    impl ObjectDeser for PaymentMethodDomain {
        type Builder = PaymentMethodDomainBuilder;
    }
};
impl stripe_types::Object for PaymentMethodDomain {
    type Id = stripe_payment::PaymentMethodDomainId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(PaymentMethodDomainId);
