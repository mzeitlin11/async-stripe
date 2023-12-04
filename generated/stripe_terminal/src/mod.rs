#![recursion_limit = "256"]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::invalid_html_tags)]

//! This crate provides Rust bindings to the Stripe HTTP API, specifically
//! for requests mentioned in the `Terminal` section of the [Stripe API docs](https://stripe.com/docs/api)

extern crate self as stripe_terminal;
pub mod deleted_terminal_configuration;
pub use deleted_terminal_configuration::DeletedTerminalConfiguration;
pub mod deleted_terminal_location;
pub use deleted_terminal_location::DeletedTerminalLocation;
pub mod deleted_terminal_reader;
pub use deleted_terminal_reader::DeletedTerminalReader;
pub mod terminal_configuration;
pub use terminal_configuration::TerminalConfiguration;
pub mod terminal_connection_token;
pub use terminal_connection_token::TerminalConnectionToken;
pub mod terminal_location;
pub use terminal_location::TerminalLocation;
pub mod terminal_reader;
pub use terminal_reader::TerminalReader;
pub mod terminal_configuration_configuration_resource_currency_specific_config;
pub use terminal_configuration_configuration_resource_currency_specific_config::TerminalConfigurationConfigurationResourceCurrencySpecificConfig;
pub mod terminal_configuration_configuration_resource_device_type_specific_config;
pub use terminal_configuration_configuration_resource_device_type_specific_config::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig;
pub mod terminal_configuration_configuration_resource_offline_config;
pub use terminal_configuration_configuration_resource_offline_config::TerminalConfigurationConfigurationResourceOfflineConfig;
pub mod terminal_configuration_configuration_resource_tipping;
pub use terminal_configuration_configuration_resource_tipping::TerminalConfigurationConfigurationResourceTipping;
pub mod terminal_reader_reader_resource_cart;
pub use terminal_reader_reader_resource_cart::TerminalReaderReaderResourceCart;
pub mod terminal_reader_reader_resource_line_item;
pub use terminal_reader_reader_resource_line_item::TerminalReaderReaderResourceLineItem;
pub mod terminal_reader_reader_resource_process_config;
pub use terminal_reader_reader_resource_process_config::TerminalReaderReaderResourceProcessConfig;
pub mod terminal_reader_reader_resource_process_payment_intent_action;
pub use terminal_reader_reader_resource_process_payment_intent_action::TerminalReaderReaderResourceProcessPaymentIntentAction;
pub mod terminal_reader_reader_resource_process_setup_config;
pub use terminal_reader_reader_resource_process_setup_config::TerminalReaderReaderResourceProcessSetupConfig;
pub mod terminal_reader_reader_resource_process_setup_intent_action;
pub use terminal_reader_reader_resource_process_setup_intent_action::TerminalReaderReaderResourceProcessSetupIntentAction;
pub mod terminal_reader_reader_resource_reader_action;
pub use terminal_reader_reader_resource_reader_action::TerminalReaderReaderResourceReaderAction;
pub mod terminal_reader_reader_resource_refund_payment_action;
pub use terminal_reader_reader_resource_refund_payment_action::TerminalReaderReaderResourceRefundPaymentAction;
pub mod terminal_reader_reader_resource_set_reader_display_action;
pub use terminal_reader_reader_resource_set_reader_display_action::TerminalReaderReaderResourceSetReaderDisplayAction;
pub mod terminal_reader_reader_resource_tipping_config;
pub use terminal_reader_reader_resource_tipping_config::TerminalReaderReaderResourceTippingConfig;
