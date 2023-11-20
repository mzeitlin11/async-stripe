#![recursion_limit = "256"]
extern crate self as stripe_checkout;
pub mod session;
pub use session::Session;
pub mod checkout_acss_debit_mandate_options;
pub use checkout_acss_debit_mandate_options::CheckoutAcssDebitMandateOptions;
pub mod checkout_acss_debit_payment_method_options;
pub use checkout_acss_debit_payment_method_options::CheckoutAcssDebitPaymentMethodOptions;
pub mod checkout_affirm_payment_method_options;
pub use checkout_affirm_payment_method_options::CheckoutAffirmPaymentMethodOptions;
pub mod checkout_afterpay_clearpay_payment_method_options;
pub use checkout_afterpay_clearpay_payment_method_options::CheckoutAfterpayClearpayPaymentMethodOptions;
pub mod checkout_alipay_payment_method_options;
pub use checkout_alipay_payment_method_options::CheckoutAlipayPaymentMethodOptions;
pub mod checkout_au_becs_debit_payment_method_options;
pub use checkout_au_becs_debit_payment_method_options::CheckoutAuBecsDebitPaymentMethodOptions;
pub mod checkout_bacs_debit_payment_method_options;
pub use checkout_bacs_debit_payment_method_options::CheckoutBacsDebitPaymentMethodOptions;
pub mod checkout_bancontact_payment_method_options;
pub use checkout_bancontact_payment_method_options::CheckoutBancontactPaymentMethodOptions;
pub mod checkout_boleto_payment_method_options;
pub use checkout_boleto_payment_method_options::CheckoutBoletoPaymentMethodOptions;
pub mod checkout_card_installments_options;
pub use checkout_card_installments_options::CheckoutCardInstallmentsOptions;
pub mod checkout_card_payment_method_options;
pub use checkout_card_payment_method_options::CheckoutCardPaymentMethodOptions;
pub mod checkout_cashapp_payment_method_options;
pub use checkout_cashapp_payment_method_options::CheckoutCashappPaymentMethodOptions;
pub mod checkout_customer_balance_bank_transfer_payment_method_options;
pub use checkout_customer_balance_bank_transfer_payment_method_options::CheckoutCustomerBalanceBankTransferPaymentMethodOptions;
pub mod checkout_customer_balance_payment_method_options;
pub use checkout_customer_balance_payment_method_options::CheckoutCustomerBalancePaymentMethodOptions;
pub mod checkout_eps_payment_method_options;
pub use checkout_eps_payment_method_options::CheckoutEpsPaymentMethodOptions;
pub mod checkout_fpx_payment_method_options;
pub use checkout_fpx_payment_method_options::CheckoutFpxPaymentMethodOptions;
pub mod checkout_giropay_payment_method_options;
pub use checkout_giropay_payment_method_options::CheckoutGiropayPaymentMethodOptions;
pub mod checkout_grab_pay_payment_method_options;
pub use checkout_grab_pay_payment_method_options::CheckoutGrabPayPaymentMethodOptions;
pub mod checkout_ideal_payment_method_options;
pub use checkout_ideal_payment_method_options::CheckoutIdealPaymentMethodOptions;
pub mod checkout_klarna_payment_method_options;
pub use checkout_klarna_payment_method_options::CheckoutKlarnaPaymentMethodOptions;
pub mod checkout_konbini_payment_method_options;
pub use checkout_konbini_payment_method_options::CheckoutKonbiniPaymentMethodOptions;
pub mod checkout_link_payment_method_options;
pub use checkout_link_payment_method_options::CheckoutLinkPaymentMethodOptions;
pub mod checkout_oxxo_payment_method_options;
pub use checkout_oxxo_payment_method_options::CheckoutOxxoPaymentMethodOptions;
pub mod checkout_p24_payment_method_options;
pub use checkout_p24_payment_method_options::CheckoutP24PaymentMethodOptions;
pub mod checkout_paynow_payment_method_options;
pub use checkout_paynow_payment_method_options::CheckoutPaynowPaymentMethodOptions;
pub mod checkout_paypal_payment_method_options;
pub use checkout_paypal_payment_method_options::CheckoutPaypalPaymentMethodOptions;
pub mod checkout_pix_payment_method_options;
pub use checkout_pix_payment_method_options::CheckoutPixPaymentMethodOptions;
pub mod checkout_revolut_pay_payment_method_options;
pub use checkout_revolut_pay_payment_method_options::CheckoutRevolutPayPaymentMethodOptions;
pub mod checkout_sepa_debit_payment_method_options;
pub use checkout_sepa_debit_payment_method_options::CheckoutSepaDebitPaymentMethodOptions;
pub mod checkout_session_payment_method_options;
pub use checkout_session_payment_method_options::CheckoutSessionPaymentMethodOptions;
pub mod checkout_sofort_payment_method_options;
pub use checkout_sofort_payment_method_options::CheckoutSofortPaymentMethodOptions;
pub mod checkout_us_bank_account_payment_method_options;
pub use checkout_us_bank_account_payment_method_options::CheckoutUsBankAccountPaymentMethodOptions;
pub mod payment_pages_checkout_session_after_expiration;
pub use payment_pages_checkout_session_after_expiration::PaymentPagesCheckoutSessionAfterExpiration;
pub mod payment_pages_checkout_session_after_expiration_recovery;
pub use payment_pages_checkout_session_after_expiration_recovery::PaymentPagesCheckoutSessionAfterExpirationRecovery;
pub mod payment_pages_checkout_session_automatic_tax;
pub use payment_pages_checkout_session_automatic_tax::PaymentPagesCheckoutSessionAutomaticTax;
pub mod payment_pages_checkout_session_consent;
pub use payment_pages_checkout_session_consent::PaymentPagesCheckoutSessionConsent;
pub mod payment_pages_checkout_session_consent_collection;
pub use payment_pages_checkout_session_consent_collection::PaymentPagesCheckoutSessionConsentCollection;
pub mod payment_pages_checkout_session_currency_conversion;
pub use payment_pages_checkout_session_currency_conversion::PaymentPagesCheckoutSessionCurrencyConversion;
pub mod payment_pages_checkout_session_custom_fields;
pub use payment_pages_checkout_session_custom_fields::PaymentPagesCheckoutSessionCustomFields;
pub mod payment_pages_checkout_session_custom_fields_dropdown;
pub use payment_pages_checkout_session_custom_fields_dropdown::PaymentPagesCheckoutSessionCustomFieldsDropdown;
pub mod payment_pages_checkout_session_custom_fields_label;
pub use payment_pages_checkout_session_custom_fields_label::PaymentPagesCheckoutSessionCustomFieldsLabel;
pub mod payment_pages_checkout_session_custom_fields_numeric;
pub use payment_pages_checkout_session_custom_fields_numeric::PaymentPagesCheckoutSessionCustomFieldsNumeric;
pub mod payment_pages_checkout_session_custom_fields_option;
pub use payment_pages_checkout_session_custom_fields_option::PaymentPagesCheckoutSessionCustomFieldsOption;
pub mod payment_pages_checkout_session_custom_fields_text;
pub use payment_pages_checkout_session_custom_fields_text::PaymentPagesCheckoutSessionCustomFieldsText;
pub mod payment_pages_checkout_session_custom_text;
pub use payment_pages_checkout_session_custom_text::PaymentPagesCheckoutSessionCustomText;
pub mod payment_pages_checkout_session_custom_text_position;
pub use payment_pages_checkout_session_custom_text_position::PaymentPagesCheckoutSessionCustomTextPosition;
pub mod payment_pages_checkout_session_customer_details;
pub use payment_pages_checkout_session_customer_details::PaymentPagesCheckoutSessionCustomerDetails;
pub mod payment_pages_checkout_session_invoice_creation;
pub use payment_pages_checkout_session_invoice_creation::PaymentPagesCheckoutSessionInvoiceCreation;
pub mod payment_pages_checkout_session_invoice_settings;
pub use payment_pages_checkout_session_invoice_settings::PaymentPagesCheckoutSessionInvoiceSettings;
pub mod payment_pages_checkout_session_phone_number_collection;
pub use payment_pages_checkout_session_phone_number_collection::PaymentPagesCheckoutSessionPhoneNumberCollection;
pub mod payment_pages_checkout_session_shipping_address_collection;
pub use payment_pages_checkout_session_shipping_address_collection::PaymentPagesCheckoutSessionShippingAddressCollection;
pub mod payment_pages_checkout_session_shipping_cost;
pub use payment_pages_checkout_session_shipping_cost::PaymentPagesCheckoutSessionShippingCost;
pub mod payment_pages_checkout_session_shipping_option;
pub use payment_pages_checkout_session_shipping_option::PaymentPagesCheckoutSessionShippingOption;
pub mod payment_pages_checkout_session_tax_id;
pub use payment_pages_checkout_session_tax_id::PaymentPagesCheckoutSessionTaxId;
pub mod payment_pages_checkout_session_tax_id_collection;
pub use payment_pages_checkout_session_tax_id_collection::PaymentPagesCheckoutSessionTaxIdCollection;
pub mod payment_pages_checkout_session_total_details;
pub use payment_pages_checkout_session_total_details::PaymentPagesCheckoutSessionTotalDetails;
pub mod payment_pages_checkout_session_total_details_resource_breakdown;
pub use payment_pages_checkout_session_total_details_resource_breakdown::PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown;
