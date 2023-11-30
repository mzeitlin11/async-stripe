#![recursion_limit = "256"]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::invalid_html_tags)]
extern crate self as stripe_connect;
pub use stripe_types::Account;
pub mod account;
pub use stripe_types::AccountBacsDebitPaymentsSettings;
pub mod account_bacs_debit_payments_settings;
pub use stripe_types::AccountBrandingSettings;
pub mod account_branding_settings;
pub use stripe_types::AccountBusinessProfile;
pub mod account_business_profile;
pub use stripe_types::AccountCapabilities;
pub mod account_capabilities;
pub use stripe_types::AccountCapabilityFutureRequirements;
pub mod account_capability_future_requirements;
pub use stripe_types::AccountCapabilityRequirements;
pub mod account_capability_requirements;
pub use stripe_types::AccountCardIssuingSettings;
pub mod account_card_issuing_settings;
pub use stripe_types::AccountCardPaymentsSettings;
pub mod account_card_payments_settings;
pub use stripe_types::AccountDashboardSettings;
pub mod account_dashboard_settings;
pub use stripe_types::AccountDeclineChargeOn;
pub mod account_decline_charge_on;
pub use stripe_types::AccountFutureRequirements;
pub mod account_future_requirements;
pub mod account_link;
pub use account_link::AccountLink;
pub use stripe_types::AccountMonthlyEstimatedRevenue;
pub mod account_monthly_estimated_revenue;
pub use stripe_types::AccountPaymentsSettings;
pub mod account_payments_settings;
pub use stripe_types::AccountPayoutSettings;
pub mod account_payout_settings;
pub use stripe_types::AccountRequirements;
pub mod account_requirements;
pub use stripe_types::AccountRequirementsAlternative;
pub mod account_requirements_alternative;
pub use stripe_types::AccountRequirementsError;
pub mod account_requirements_error;
pub use stripe_types::AccountSepaDebitPaymentsSettings;
pub mod account_sepa_debit_payments_settings;
pub mod account_session;
pub use account_session::AccountSession;
pub use stripe_types::AccountSettings;
pub mod account_settings;
pub use stripe_types::AccountTermsOfService;
pub mod account_terms_of_service;
pub use stripe_types::AccountTosAcceptance;
pub mod account_tos_acceptance;
pub use stripe_types::AccountTreasurySettings;
pub mod account_treasury_settings;
pub use stripe_types::AccountUnificationAccountController;
pub mod account_unification_account_controller;
pub use stripe_types::Application;
pub mod application;
pub use stripe_types::ApplicationFee;
pub mod application_fee;
pub mod apps_secret;
pub use apps_secret::AppsSecret;
pub use stripe_types::Capability;
pub mod capability;
pub mod connect_embedded_account_session_create_components;
pub use connect_embedded_account_session_create_components::ConnectEmbeddedAccountSessionCreateComponents;
pub mod connect_embedded_base_config_claim;
pub use connect_embedded_base_config_claim::ConnectEmbeddedBaseConfigClaim;
pub mod country_spec;
pub use country_spec::CountrySpec;
pub mod country_spec_verification_field_details;
pub use country_spec_verification_field_details::CountrySpecVerificationFieldDetails;
pub mod country_spec_verification_fields;
pub use country_spec_verification_fields::CountrySpecVerificationFields;
pub use stripe_types::DeletedAccount;
pub mod deleted_account;
pub use stripe_types::DeletedPerson;
pub mod deleted_person;
pub use stripe_types::ExternalAccount;
pub mod external_account;
pub use stripe_types::ApplicationFeeRefund;
pub mod application_fee_refund;
pub use stripe_types::LegalEntityCompany;
pub mod legal_entity_company;
pub use stripe_types::LegalEntityCompanyVerification;
pub mod legal_entity_company_verification;
pub use stripe_types::LegalEntityCompanyVerificationDocument;
pub mod legal_entity_company_verification_document;
pub use stripe_types::LegalEntityDob;
pub mod legal_entity_dob;
pub use stripe_types::LegalEntityJapanAddress;
pub mod legal_entity_japan_address;
pub use stripe_types::LegalEntityPersonVerification;
pub mod legal_entity_person_verification;
pub use stripe_types::LegalEntityPersonVerificationDocument;
pub mod legal_entity_person_verification_document;
pub use stripe_types::LegalEntityUboDeclaration;
pub mod legal_entity_ubo_declaration;
pub mod login_link;
pub use login_link::LoginLink;
pub use stripe_types::Person;
pub mod person;
pub use stripe_types::PersonAdditionalTosAcceptance;
pub mod person_additional_tos_acceptance;
pub use stripe_types::PersonAdditionalTosAcceptances;
pub mod person_additional_tos_acceptances;
pub use stripe_types::PersonFutureRequirements;
pub mod person_future_requirements;
pub use stripe_types::PersonRelationship;
pub mod person_relationship;
pub use stripe_types::PersonRequirements;
pub mod person_requirements;
pub mod secret_service_resource_scope;
pub use secret_service_resource_scope::SecretServiceResourceScope;
pub use stripe_types::Topup;
pub mod topup;
pub use stripe_types::Transfer;
pub mod transfer;
pub use stripe_types::TransferData;
pub mod transfer_data;
pub use stripe_types::TransferReversal;
pub mod transfer_reversal;
pub use stripe_types::TransferSchedule;
pub mod transfer_schedule;
