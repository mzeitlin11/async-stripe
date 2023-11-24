#![recursion_limit = "256"]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::invalid_html_tags)]
extern crate self as stripe_billing;
pub use stripe_types::AutomaticTax;
pub mod portal_configuration;
pub use portal_configuration::PortalConfiguration;
pub mod portal_session;
pub use portal_session::PortalSession;
pub use stripe_types::CancellationDetails;
pub use stripe_types::CreditNote;
pub mod credit_note;
pub use stripe_types::CreditNoteLineItem;
pub mod credit_note_line_item;
pub use stripe_types::CreditNoteTaxAmount;
pub use stripe_types::DeletedInvoice;
pub use stripe_types::DeletedInvoiceItem;
pub use stripe_types::DeletedPlan;
pub use stripe_types::DeletedSubscriptionItem;
pub use stripe_types::DeletedTaxId;
pub use stripe_types::DeletedTestClock;
pub use stripe_types::DiscountsResourceDiscountAmount;
pub use stripe_types::Invoice;
pub mod invoice;
pub use stripe_types::InvoiceInstallmentsCard;
pub use stripe_types::InvoiceItem;
pub use stripe_types::InvoiceItemThresholdReason;
pub use stripe_types::InvoiceLineItemPeriod;
pub use stripe_types::InvoiceMandateOptionsCard;
pub use stripe_types::InvoicePaymentMethodOptionsAcssDebit;
pub use stripe_types::InvoicePaymentMethodOptionsAcssDebitMandateOptions;
pub use stripe_types::InvoicePaymentMethodOptionsBancontact;
pub use stripe_types::InvoicePaymentMethodOptionsCard;
pub use stripe_types::InvoicePaymentMethodOptionsCustomerBalance;
pub use stripe_types::InvoicePaymentMethodOptionsCustomerBalanceBankTransfer;
pub use stripe_types::InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer;
pub use stripe_types::InvoicePaymentMethodOptionsKonbini;
pub use stripe_types::InvoicePaymentMethodOptionsUsBankAccount;
pub use stripe_types::InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions;
pub use stripe_types::InvoiceRenderingPdf;
pub use stripe_types::InvoiceSettingCustomField;
pub use stripe_types::InvoiceSettingCustomerSetting;
pub use stripe_types::InvoiceSettingQuoteSetting;
pub use stripe_types::InvoiceSettingRenderingOptions;
pub use stripe_types::InvoiceSettingSubscriptionSchedulePhaseSetting;
pub use stripe_types::InvoiceSettingSubscriptionScheduleSetting;
pub use stripe_types::InvoiceTaxAmount;
pub use stripe_types::InvoiceThresholdReason;
pub use stripe_types::InvoiceTransferData;
pub mod invoice_item;
pub use stripe_types::InvoiceLineItem;
pub use stripe_types::InvoicesFromInvoice;
pub use stripe_types::InvoicesInvoiceRendering;
pub use stripe_types::InvoicesPaymentMethodOptions;
pub use stripe_types::InvoicesPaymentSettings;
pub use stripe_types::InvoicesResourceInvoiceTaxId;
pub use stripe_types::InvoicesResourceLineItemsCreditedItems;
pub use stripe_types::InvoicesResourceLineItemsProrationDetails;
pub use stripe_types::InvoicesShippingCost;
pub use stripe_types::InvoicesStatusTransitions;
pub use stripe_types::LineItem;
pub mod invoice_line_item;
pub use stripe_types::UsageRecordSummary;
pub mod usage_record_summary;
pub use stripe_types::Period;
pub use stripe_types::Plan;
pub mod plan;
pub use stripe_types::PlanTier;
pub mod portal_business_profile;
pub use portal_business_profile::PortalBusinessProfile;
pub mod portal_customer_update;
pub use portal_customer_update::PortalCustomerUpdate;
pub mod portal_features;
pub use portal_features::PortalFeatures;
pub mod portal_flows_after_completion_hosted_confirmation;
pub use portal_flows_after_completion_hosted_confirmation::PortalFlowsAfterCompletionHostedConfirmation;
pub mod portal_flows_after_completion_redirect;
pub use portal_flows_after_completion_redirect::PortalFlowsAfterCompletionRedirect;
pub mod portal_flows_coupon_offer;
pub use portal_flows_coupon_offer::PortalFlowsCouponOffer;
pub mod portal_flows_flow;
pub use portal_flows_flow::PortalFlowsFlow;
pub mod portal_flows_flow_after_completion;
pub use portal_flows_flow_after_completion::PortalFlowsFlowAfterCompletion;
pub mod portal_flows_flow_subscription_cancel;
pub use portal_flows_flow_subscription_cancel::PortalFlowsFlowSubscriptionCancel;
pub mod portal_flows_flow_subscription_update;
pub use portal_flows_flow_subscription_update::PortalFlowsFlowSubscriptionUpdate;
pub mod portal_flows_flow_subscription_update_confirm;
pub use portal_flows_flow_subscription_update_confirm::PortalFlowsFlowSubscriptionUpdateConfirm;
pub mod portal_flows_retention;
pub use portal_flows_retention::PortalFlowsRetention;
pub mod portal_flows_subscription_update_confirm_discount;
pub use portal_flows_subscription_update_confirm_discount::PortalFlowsSubscriptionUpdateConfirmDiscount;
pub mod portal_flows_subscription_update_confirm_item;
pub use portal_flows_subscription_update_confirm_item::PortalFlowsSubscriptionUpdateConfirmItem;
pub mod portal_invoice_list;
pub use portal_invoice_list::PortalInvoiceList;
pub mod portal_login_page;
pub use portal_login_page::PortalLoginPage;
pub mod portal_payment_method_update;
pub use portal_payment_method_update::PortalPaymentMethodUpdate;
pub mod portal_subscription_cancel;
pub use portal_subscription_cancel::PortalSubscriptionCancel;
pub mod portal_subscription_cancellation_reason;
pub use portal_subscription_cancellation_reason::PortalSubscriptionCancellationReason;
pub mod portal_subscription_pause;
pub use portal_subscription_pause::PortalSubscriptionPause;
pub mod portal_subscription_update;
pub use portal_subscription_update::PortalSubscriptionUpdate;
pub mod portal_subscription_update_product;
pub use portal_subscription_update_product::PortalSubscriptionUpdateProduct;
pub use stripe_types::Quote;
pub mod quote;
pub use stripe_types::QuotesResourceAutomaticTax;
pub use stripe_types::QuotesResourceComputed;
pub use stripe_types::QuotesResourceFromQuote;
pub use stripe_types::QuotesResourceRecurring;
pub use stripe_types::QuotesResourceStatusTransitions;
pub use stripe_types::QuotesResourceSubscriptionDataSubscriptionData;
pub use stripe_types::QuotesResourceTotalDetails;
pub use stripe_types::QuotesResourceTotalDetailsResourceBreakdown;
pub use stripe_types::QuotesResourceTransferData;
pub use stripe_types::QuotesResourceUpfront;
pub use stripe_types::SchedulesPhaseAutomaticTax;
pub mod usage_record;
pub use stripe_types::Subscription;
pub use usage_record::UsageRecord;
pub mod subscription;
pub use stripe_types::SubscriptionAutomaticTax;
pub use stripe_types::SubscriptionBillingThresholds;
pub use stripe_types::SubscriptionDetailsData;
pub use stripe_types::SubscriptionItem;
pub mod subscription_item;
pub use stripe_types::SubscriptionItemBillingThresholds;
pub use stripe_types::SubscriptionPaymentMethodOptionsCard;
pub use stripe_types::SubscriptionPendingInvoiceItemInterval;
pub use stripe_types::SubscriptionSchedule;
pub mod subscription_schedule;
pub use stripe_types::SubscriptionScheduleAddInvoiceItem;
pub use stripe_types::SubscriptionScheduleConfigurationItem;
pub use stripe_types::SubscriptionScheduleCurrentPhase;
pub use stripe_types::SubscriptionSchedulePhaseConfiguration;
pub use stripe_types::SubscriptionSchedulesResourceDefaultSettings;
pub use stripe_types::SubscriptionSchedulesResourceDefaultSettingsAutomaticTax;
pub use stripe_types::SubscriptionTransferData;
pub use stripe_types::SubscriptionsResourcePauseCollection;
pub use stripe_types::SubscriptionsResourcePaymentMethodOptions;
pub use stripe_types::SubscriptionsResourcePaymentSettings;
pub use stripe_types::SubscriptionsResourcePendingUpdate;
pub use stripe_types::SubscriptionsTrialsResourceEndBehavior;
pub use stripe_types::SubscriptionsTrialsResourceTrialSettings;
pub use stripe_types::TaxId;
pub mod tax_id;
pub use stripe_types::TaxIdVerification;
pub use stripe_types::TestClock;
pub mod test_clock;
pub use stripe_types::TransformUsage;
