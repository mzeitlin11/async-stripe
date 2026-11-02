fn main() {
    let details: stripe::PaymentMethodDetails =
        miniserde::json::from_str(r#"{"type":"card"}"#).unwrap();
    let _card = details.card();
}
