/// For more details see <<https://stripe.com/docs/api/disputes/evidence_object>>.
#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DisputeEvidence {
    /// Any server or activity logs showing proof that the customer accessed or downloaded the purchased digital product.
    /// This information should include IP addresses, corresponding timestamps, and any detailed recorded activity.
    pub access_activity_log: Option<String>,
    /// The billing address provided by the customer.
    pub billing_address: Option<String>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Your subscription cancellation policy, as shown to the customer.
    pub cancellation_policy: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// An explanation of how and when the customer was shown your refund policy prior to purchase.
    pub cancellation_policy_disclosure: Option<String>,
    /// A justification for why the customer's subscription was not canceled.
    pub cancellation_rebuttal: Option<String>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any communication with the customer that you feel is relevant to your case.
    /// Examples include emails proving that the customer received the product or service, or demonstrating their use of or satisfaction with the product or service.
    pub customer_communication: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// The email address of the customer.
    pub customer_email_address: Option<String>,
    /// The name of the customer.
    pub customer_name: Option<String>,
    /// The IP address that the customer used when making the purchase.
    pub customer_purchase_ip: Option<String>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A relevant document or contract showing the customer's signature.
    pub customer_signature: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation for the prior charge that can uniquely identify the charge, such as a receipt, shipping label, work order, etc.
    /// This document should be paired with a similar document from the disputed payment that proves the two payments are separate.
    pub duplicate_charge_documentation: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// An explanation of the difference between the disputed charge versus the prior charge that appears to be a duplicate.
    pub duplicate_charge_explanation: Option<String>,
    /// The Stripe ID for the prior charge which appears to be a duplicate of the disputed charge.
    pub duplicate_charge_id: Option<String>,
    /// A description of the product or service that was sold.
    pub product_description: Option<String>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any receipt or message sent to the customer notifying them of the charge.
    pub receipt: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Your refund policy, as shown to the customer.
    pub refund_policy: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// Documentation demonstrating that the customer was shown your refund policy prior to purchase.
    pub refund_policy_disclosure: Option<String>,
    /// A justification for why the customer is not entitled to a refund.
    pub refund_refusal_explanation: Option<String>,
    /// The date on which the customer received or began receiving the purchased service, in a clear human-readable format.
    pub service_date: Option<String>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation showing proof that a service was provided to the customer.
    /// This could include a copy of a signed contract, work order, or other form of written agreement.
    pub service_documentation: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// The address to which a physical product was shipped.
    /// You should try to include as complete address information as possible.
    pub shipping_address: Option<String>,
    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    /// If multiple carriers were used for this purchase, please separate them with commas.
    pub shipping_carrier: Option<String>,
    /// The date on which a physical product began its route to the shipping address, in a clear human-readable format.
    pub shipping_date: Option<String>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation showing proof that a product was shipped to the customer at the same address the customer provided to you.
    /// This could include a copy of the shipment receipt, shipping label, etc.
    /// It should show the customer's full shipping address, if possible.
    pub shipping_documentation: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// The tracking number for a physical product, obtained from the delivery service.
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    pub shipping_tracking_number: Option<String>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any additional evidence or statements.
    pub uncategorized_file: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// Any additional evidence or statements.
    pub uncategorized_text: Option<String>,
}
#[cfg(feature = "min-ser")]
pub struct DisputeEvidenceBuilder {
    access_activity_log: Option<Option<String>>,
    billing_address: Option<Option<String>>,
    cancellation_policy: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    cancellation_policy_disclosure: Option<Option<String>>,
    cancellation_rebuttal: Option<Option<String>>,
    customer_communication: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    customer_email_address: Option<Option<String>>,
    customer_name: Option<Option<String>>,
    customer_purchase_ip: Option<Option<String>>,
    customer_signature: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    duplicate_charge_documentation: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    duplicate_charge_explanation: Option<Option<String>>,
    duplicate_charge_id: Option<Option<String>>,
    product_description: Option<Option<String>>,
    receipt: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    refund_policy: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    refund_policy_disclosure: Option<Option<String>>,
    refund_refusal_explanation: Option<Option<String>>,
    service_date: Option<Option<String>>,
    service_documentation: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    shipping_address: Option<Option<String>>,
    shipping_carrier: Option<Option<String>>,
    shipping_date: Option<Option<String>>,
    shipping_documentation: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    shipping_tracking_number: Option<Option<String>>,
    uncategorized_file: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    uncategorized_text: Option<Option<String>>,
}

#[cfg(feature = "min-ser")]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for DisputeEvidence {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DisputeEvidence>,
        builder: DisputeEvidenceBuilder,
    }

    impl Visitor for Place<DisputeEvidence> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: DisputeEvidenceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for DisputeEvidenceBuilder {
        type Out = DisputeEvidence;
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            #[allow(clippy::match_single_binding)]
            match k {
                "access_activity_log" => Ok(Deserialize::begin(&mut self.access_activity_log)),
                "billing_address" => Ok(Deserialize::begin(&mut self.billing_address)),
                "cancellation_policy" => Ok(Deserialize::begin(&mut self.cancellation_policy)),
                "cancellation_policy_disclosure" => Ok(Deserialize::begin(&mut self.cancellation_policy_disclosure)),
                "cancellation_rebuttal" => Ok(Deserialize::begin(&mut self.cancellation_rebuttal)),
                "customer_communication" => Ok(Deserialize::begin(&mut self.customer_communication)),
                "customer_email_address" => Ok(Deserialize::begin(&mut self.customer_email_address)),
                "customer_name" => Ok(Deserialize::begin(&mut self.customer_name)),
                "customer_purchase_ip" => Ok(Deserialize::begin(&mut self.customer_purchase_ip)),
                "customer_signature" => Ok(Deserialize::begin(&mut self.customer_signature)),
                "duplicate_charge_documentation" => Ok(Deserialize::begin(&mut self.duplicate_charge_documentation)),
                "duplicate_charge_explanation" => Ok(Deserialize::begin(&mut self.duplicate_charge_explanation)),
                "duplicate_charge_id" => Ok(Deserialize::begin(&mut self.duplicate_charge_id)),
                "product_description" => Ok(Deserialize::begin(&mut self.product_description)),
                "receipt" => Ok(Deserialize::begin(&mut self.receipt)),
                "refund_policy" => Ok(Deserialize::begin(&mut self.refund_policy)),
                "refund_policy_disclosure" => Ok(Deserialize::begin(&mut self.refund_policy_disclosure)),
                "refund_refusal_explanation" => Ok(Deserialize::begin(&mut self.refund_refusal_explanation)),
                "service_date" => Ok(Deserialize::begin(&mut self.service_date)),
                "service_documentation" => Ok(Deserialize::begin(&mut self.service_documentation)),
                "shipping_address" => Ok(Deserialize::begin(&mut self.shipping_address)),
                "shipping_carrier" => Ok(Deserialize::begin(&mut self.shipping_carrier)),
                "shipping_date" => Ok(Deserialize::begin(&mut self.shipping_date)),
                "shipping_documentation" => Ok(Deserialize::begin(&mut self.shipping_documentation)),
                "shipping_tracking_number" => Ok(Deserialize::begin(&mut self.shipping_tracking_number)),
                "uncategorized_file" => Ok(Deserialize::begin(&mut self.uncategorized_file)),
                "uncategorized_text" => Ok(Deserialize::begin(&mut self.uncategorized_text)),

                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn deser_default() -> Self {
            Self {
                access_activity_log: Deserialize::default(),
                billing_address: Deserialize::default(),
                cancellation_policy: Deserialize::default(),
                cancellation_policy_disclosure: Deserialize::default(),
                cancellation_rebuttal: Deserialize::default(),
                customer_communication: Deserialize::default(),
                customer_email_address: Deserialize::default(),
                customer_name: Deserialize::default(),
                customer_purchase_ip: Deserialize::default(),
                customer_signature: Deserialize::default(),
                duplicate_charge_documentation: Deserialize::default(),
                duplicate_charge_explanation: Deserialize::default(),
                duplicate_charge_id: Deserialize::default(),
                product_description: Deserialize::default(),
                receipt: Deserialize::default(),
                refund_policy: Deserialize::default(),
                refund_policy_disclosure: Deserialize::default(),
                refund_refusal_explanation: Deserialize::default(),
                service_date: Deserialize::default(),
                service_documentation: Deserialize::default(),
                shipping_address: Deserialize::default(),
                shipping_carrier: Deserialize::default(),
                shipping_date: Deserialize::default(),
                shipping_documentation: Deserialize::default(),
                shipping_tracking_number: Deserialize::default(),
                uncategorized_file: Deserialize::default(),
                uncategorized_text: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let access_activity_log = self.access_activity_log.take()?;
            let billing_address = self.billing_address.take()?;
            let cancellation_policy = self.cancellation_policy.take()?;
            let cancellation_policy_disclosure = self.cancellation_policy_disclosure.take()?;
            let cancellation_rebuttal = self.cancellation_rebuttal.take()?;
            let customer_communication = self.customer_communication.take()?;
            let customer_email_address = self.customer_email_address.take()?;
            let customer_name = self.customer_name.take()?;
            let customer_purchase_ip = self.customer_purchase_ip.take()?;
            let customer_signature = self.customer_signature.take()?;
            let duplicate_charge_documentation = self.duplicate_charge_documentation.take()?;
            let duplicate_charge_explanation = self.duplicate_charge_explanation.take()?;
            let duplicate_charge_id = self.duplicate_charge_id.take()?;
            let product_description = self.product_description.take()?;
            let receipt = self.receipt.take()?;
            let refund_policy = self.refund_policy.take()?;
            let refund_policy_disclosure = self.refund_policy_disclosure.take()?;
            let refund_refusal_explanation = self.refund_refusal_explanation.take()?;
            let service_date = self.service_date.take()?;
            let service_documentation = self.service_documentation.take()?;
            let shipping_address = self.shipping_address.take()?;
            let shipping_carrier = self.shipping_carrier.take()?;
            let shipping_date = self.shipping_date.take()?;
            let shipping_documentation = self.shipping_documentation.take()?;
            let shipping_tracking_number = self.shipping_tracking_number.take()?;
            let uncategorized_file = self.uncategorized_file.take()?;
            let uncategorized_text = self.uncategorized_text.take()?;

            Some(Self::Out {
                access_activity_log,
                billing_address,
                cancellation_policy,
                cancellation_policy_disclosure,
                cancellation_rebuttal,
                customer_communication,
                customer_email_address,
                customer_name,
                customer_purchase_ip,
                customer_signature,
                duplicate_charge_documentation,
                duplicate_charge_explanation,
                duplicate_charge_id,
                product_description,
                receipt,
                refund_policy,
                refund_policy_disclosure,
                refund_refusal_explanation,
                service_date,
                service_documentation,
                shipping_address,
                shipping_carrier,
                shipping_date,
                shipping_documentation,
                shipping_tracking_number,
                uncategorized_file,
                uncategorized_text,
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

    impl ObjectDeser for DisputeEvidence {
        type Builder = DisputeEvidenceBuilder;
    }
};
