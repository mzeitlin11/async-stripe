fn main() {
    // The same workspace can compile notifications alone, typed fetched events,
    // or related Account DTOs. Availability is selected only by consumer features.
    let notification = stripe_client_core::EventNotification::parse(include_str!(
        "../fixtures/account-notification.json"
    ))
    .unwrap();
    println!("{}", notification.event_type);
    #[cfg(feature = "typed-events")]
    let _ = stripe_events::Notification::from_notification(notification).unwrap();
    #[cfg(feature = "accounts")]
    let _: stripe_v2::core::Account =
        miniserde::json::from_str(include_str!("../fixtures/account.json")).unwrap();
}
