use std::hint::black_box;

fn main() {
    let destination: stripe_v2::core::EventDestination =
        miniserde::json::from_str(black_box(include_str!("../fixtures/event_destination.json")))
            .unwrap();
    black_box(destination);
    #[cfg(feature = "v1")]
    black_box(stripe_shared::AccountId::from("acct_example"));
}
