#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetailsPromptpay {
    /// Bill reference generated by PromptPay
    pub reference: Option<String>,
}
