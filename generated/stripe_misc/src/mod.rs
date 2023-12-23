#![recursion_limit = "256"]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_html_tags)]

//! This crate provides Rust bindings to the Stripe HTTP API, specifically
//! for requests mentioned in the `Tax`, `Identity`, `Reporting`, `Sigma`, `Financial Connections`
//! and `Webhooks` sections of the [Stripe API docs](https://stripe.com/docs/api)

extern crate self as stripe_misc;
pub use apple_pay_domain::types::*;
pub mod apple_pay_domain;
#[doc(hidden)]
pub mod bank_connections_resource_accountholder;
#[doc(inline)]
pub use bank_connections_resource_accountholder::*;
#[doc(hidden)]
pub mod bank_connections_resource_balance;
#[doc(inline)]
pub use bank_connections_resource_balance::*;
#[doc(hidden)]
pub mod bank_connections_resource_balance_api_resource_cash_balance;
#[doc(inline)]
pub use bank_connections_resource_balance_api_resource_cash_balance::*;
#[doc(hidden)]
pub mod bank_connections_resource_balance_api_resource_credit_balance;
#[doc(inline)]
pub use bank_connections_resource_balance_api_resource_credit_balance::*;
#[doc(hidden)]
pub mod bank_connections_resource_balance_refresh;
#[doc(inline)]
pub use bank_connections_resource_balance_refresh::*;
#[doc(hidden)]
pub mod bank_connections_resource_link_account_session_filters;
#[doc(inline)]
pub use bank_connections_resource_link_account_session_filters::*;
#[doc(hidden)]
pub mod bank_connections_resource_ownership_refresh;
#[doc(inline)]
pub use bank_connections_resource_ownership_refresh::*;
#[doc(hidden)]
pub mod deleted_apple_pay_domain;
#[doc(inline)]
pub use deleted_apple_pay_domain::*;
pub use webhook_endpoint::types::*;
#[doc(hidden)]
pub mod verification_session_redaction;
pub mod webhook_endpoint;
#[doc(inline)]
pub use verification_session_redaction::*;
#[doc(hidden)]
pub mod deleted_webhook_endpoint;
#[doc(inline)]
pub use deleted_webhook_endpoint::*;
pub use ephemeral_key::types::*;
pub mod ephemeral_key;
pub use exchange_rate::types::*;
pub mod exchange_rate;
pub use financial_connections_account::types::*;
pub mod financial_connections_account;
#[doc(hidden)]
pub mod financial_connections_account_owner;
#[doc(inline)]
pub use financial_connections_account_owner::*;
#[doc(hidden)]
pub mod financial_connections_account_ownership;
#[doc(inline)]
pub use financial_connections_account_ownership::*;
pub use financial_connections_session::types::*;
pub mod financial_connections_session;
#[doc(hidden)]
pub mod financial_reporting_finance_report_run_run_parameters;
#[doc(inline)]
pub use financial_reporting_finance_report_run_run_parameters::*;
#[doc(hidden)]
pub mod gelato_data_document_report_date_of_birth;
#[doc(inline)]
pub use gelato_data_document_report_date_of_birth::*;
#[doc(hidden)]
pub mod gelato_data_document_report_expiration_date;
#[doc(inline)]
pub use gelato_data_document_report_expiration_date::*;
#[doc(hidden)]
pub mod gelato_data_document_report_issued_date;
#[doc(inline)]
pub use gelato_data_document_report_issued_date::*;
#[doc(hidden)]
pub mod gelato_data_id_number_report_date;
#[doc(inline)]
pub use gelato_data_id_number_report_date::*;
#[doc(hidden)]
pub mod gelato_data_verified_outputs_date;
#[doc(inline)]
pub use gelato_data_verified_outputs_date::*;
#[doc(hidden)]
pub mod gelato_document_report;
#[doc(inline)]
pub use gelato_document_report::*;
#[doc(hidden)]
pub mod gelato_document_report_error;
#[doc(inline)]
pub use gelato_document_report_error::*;
#[doc(hidden)]
pub mod gelato_id_number_report;
#[doc(inline)]
pub use gelato_id_number_report::*;
#[doc(hidden)]
pub mod gelato_id_number_report_error;
#[doc(inline)]
pub use gelato_id_number_report_error::*;
#[doc(hidden)]
pub mod gelato_report_document_options;
#[doc(inline)]
pub use gelato_report_document_options::*;
#[doc(hidden)]
pub mod gelato_report_id_number_options;
#[doc(inline)]
pub use gelato_report_id_number_options::*;
#[doc(hidden)]
pub mod gelato_selfie_report;
#[doc(inline)]
pub use gelato_selfie_report::*;
#[doc(hidden)]
pub mod gelato_selfie_report_error;
#[doc(inline)]
pub use gelato_selfie_report_error::*;
#[doc(hidden)]
pub mod gelato_session_document_options;
#[doc(inline)]
pub use gelato_session_document_options::*;
#[doc(hidden)]
pub mod gelato_session_id_number_options;
#[doc(inline)]
pub use gelato_session_id_number_options::*;
#[doc(hidden)]
pub mod gelato_session_last_error;
#[doc(inline)]
pub use gelato_session_last_error::*;
#[doc(hidden)]
pub mod gelato_verification_report_options;
#[doc(inline)]
pub use gelato_verification_report_options::*;
#[doc(hidden)]
pub mod gelato_verification_session_options;
#[doc(inline)]
pub use gelato_verification_session_options::*;
#[doc(hidden)]
pub mod gelato_verified_outputs;
#[doc(inline)]
pub use gelato_verified_outputs::*;
pub use identity_verification_report::types::*;
pub mod identity_verification_report;
pub use identity_verification_session::types::*;
pub mod identity_verification_session;
pub use reporting_report_run::types::*;
pub mod reporting_report_run;
pub use reporting_report_type::types::*;
pub mod reporting_report_type;
pub use scheduled_query_run::types::*;
pub mod scheduled_query_run;
#[doc(hidden)]
pub mod sigma_scheduled_query_run_error;
#[doc(inline)]
pub use sigma_scheduled_query_run_error::*;
pub use tax_calculation::types::*;
pub mod tax_calculation;
#[doc(hidden)]
pub mod tax_calculation_line_item;
#[doc(inline)]
pub use tax_calculation_line_item::*;
pub use tax_registration::types::*;
pub mod tax_registration;
pub use tax_settings::types::*;
pub mod tax_settings;
pub use tax_transaction::types::*;
pub mod tax_transaction;
#[doc(hidden)]
pub mod tax_transaction_line_item;
#[doc(inline)]
pub use tax_transaction_line_item::*;
#[doc(hidden)]
pub mod tax_product_registrations_resource_country_options;
#[doc(inline)]
pub use tax_product_registrations_resource_country_options::*;
#[doc(hidden)]
pub mod tax_product_registrations_resource_country_options_ca_province_standard;
#[doc(inline)]
pub use tax_product_registrations_resource_country_options_ca_province_standard::*;
#[doc(hidden)]
pub mod tax_product_registrations_resource_country_options_canada;
#[doc(inline)]
pub use tax_product_registrations_resource_country_options_canada::*;
#[doc(hidden)]
pub mod tax_product_registrations_resource_country_options_default;
#[doc(inline)]
pub use tax_product_registrations_resource_country_options_default::*;
#[doc(hidden)]
pub mod tax_product_registrations_resource_country_options_eu_standard;
#[doc(inline)]
pub use tax_product_registrations_resource_country_options_eu_standard::*;
#[doc(hidden)]
pub mod tax_product_registrations_resource_country_options_europe;
#[doc(inline)]
pub use tax_product_registrations_resource_country_options_europe::*;
#[doc(hidden)]
pub mod tax_product_registrations_resource_country_options_simplified;
#[doc(inline)]
pub use tax_product_registrations_resource_country_options_simplified::*;
#[doc(hidden)]
pub mod tax_product_registrations_resource_country_options_united_states;
#[doc(inline)]
pub use tax_product_registrations_resource_country_options_united_states::*;
#[doc(hidden)]
pub mod tax_product_registrations_resource_country_options_us_local_amusement_tax;
#[doc(inline)]
pub use tax_product_registrations_resource_country_options_us_local_amusement_tax::*;
#[doc(hidden)]
pub mod tax_product_registrations_resource_country_options_us_local_lease_tax;
#[doc(inline)]
pub use tax_product_registrations_resource_country_options_us_local_lease_tax::*;
#[doc(hidden)]
pub mod tax_product_resource_customer_details;
#[doc(inline)]
pub use tax_product_resource_customer_details::*;
#[doc(hidden)]
pub mod tax_product_resource_customer_details_resource_tax_id;
#[doc(inline)]
pub use tax_product_resource_customer_details_resource_tax_id::*;
#[doc(hidden)]
pub mod tax_product_resource_jurisdiction;
#[doc(inline)]
pub use tax_product_resource_jurisdiction::*;
#[doc(hidden)]
pub mod tax_product_resource_line_item_tax_breakdown;
#[doc(inline)]
pub use tax_product_resource_line_item_tax_breakdown::*;
#[doc(hidden)]
pub mod tax_product_resource_line_item_tax_rate_details;
#[doc(inline)]
pub use tax_product_resource_line_item_tax_rate_details::*;
#[doc(hidden)]
pub mod tax_product_resource_postal_address;
#[doc(inline)]
pub use tax_product_resource_postal_address::*;
#[doc(hidden)]
pub mod tax_product_resource_tax_breakdown;
#[doc(inline)]
pub use tax_product_resource_tax_breakdown::*;
#[doc(hidden)]
pub mod tax_product_resource_tax_calculation_shipping_cost;
#[doc(inline)]
pub use tax_product_resource_tax_calculation_shipping_cost::*;
#[doc(hidden)]
pub mod tax_product_resource_tax_rate_details;
#[doc(inline)]
pub use tax_product_resource_tax_rate_details::*;
#[doc(hidden)]
pub mod tax_product_resource_tax_settings_defaults;
#[doc(inline)]
pub use tax_product_resource_tax_settings_defaults::*;
#[doc(hidden)]
pub mod tax_product_resource_tax_settings_head_office;
#[doc(inline)]
pub use tax_product_resource_tax_settings_head_office::*;
#[doc(hidden)]
pub mod tax_product_resource_tax_settings_status_details;
#[doc(inline)]
pub use tax_product_resource_tax_settings_status_details::*;
#[doc(hidden)]
pub mod tax_product_resource_tax_settings_status_details_resource_active;
#[doc(inline)]
pub use tax_product_resource_tax_settings_status_details_resource_active::*;
#[doc(hidden)]
pub mod tax_product_resource_tax_settings_status_details_resource_pending;
#[doc(inline)]
pub use tax_product_resource_tax_settings_status_details_resource_pending::*;
#[doc(hidden)]
pub mod tax_product_resource_tax_transaction_line_item_resource_reversal;
#[doc(inline)]
pub use tax_product_resource_tax_transaction_line_item_resource_reversal::*;
#[doc(hidden)]
pub mod tax_product_resource_tax_transaction_resource_reversal;
#[doc(inline)]
pub use tax_product_resource_tax_transaction_resource_reversal::*;
#[doc(hidden)]
pub mod tax_product_resource_tax_transaction_shipping_cost;
#[doc(inline)]
pub use tax_product_resource_tax_transaction_shipping_cost::*;
