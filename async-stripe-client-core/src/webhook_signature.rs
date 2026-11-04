use std::time::{Duration, SystemTime, UNIX_EPOCH};

use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

/// Failure to authenticate the original bytes of a webhook delivery.
#[derive(Debug, thiserror::Error)]
pub enum SignatureError {
    /// The header has no usable timestamp or signature.
    #[error("invalid webhook signature header")]
    BadSignature,
    /// The header timestamp is invalid.
    #[error("invalid webhook timestamp: {0}")]
    BadHeader(#[from] std::num::ParseIntError),
    /// The signing key could not be used.
    #[error("invalid signing key")]
    BadKey,
    /// The authenticated delivery timestamp is outside the allowed tolerance.
    #[error("webhook timestamp {0} is outside tolerance")]
    BadTimestamp(i64),
    /// The system clock predates the Unix epoch.
    #[error("system clock predates the Unix epoch")]
    BadClock,
}

/// Model-independent Stripe webhook signature verification.
#[derive(Debug)]
pub struct WebhookSignature;

impl WebhookSignature {
    /// Authenticate the raw body with the standard five-minute timestamp tolerance.
    ///
    /// # Errors
    /// Returns an error for an invalid signature, header, or expired delivery.
    pub fn verify(payload: &[u8], header: &str, secret: &str) -> Result<(), SignatureError> {
        let now =
            SystemTime::now().duration_since(UNIX_EPOCH).map_err(|_| SignatureError::BadClock)?;
        let now = i64::try_from(now.as_secs()).map_err(|_| SignatureError::BadClock)?;
        Self::verify_at(payload, header, secret, now, Duration::from_secs(300))
    }

    /// Authenticate using an explicit current time and tolerance.
    /// All `v1` signatures are considered to support signing-secret rotation.
    /// The signature scheme name is independent of the API namespace.
    ///
    /// # Errors
    /// Returns an error for an invalid signature, header, or delivery timestamp.
    pub fn verify_at(
        payload: &[u8],
        header: &str,
        secret: &str,
        now: i64,
        tolerance: Duration,
    ) -> Result<(), SignatureError> {
        let mut timestamp = None;
        let mut signatures = Vec::new();
        for pair in header.split(',') {
            let (key, value) = pair.split_once('=').ok_or(SignatureError::BadSignature)?;
            match key {
                "t" => timestamp = Some(value.parse::<i64>()?),
                "v1" => signatures.push(value),
                _ => {}
            }
        }
        let timestamp = timestamp.ok_or(SignatureError::BadSignature)?;
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
            .map_err(|_| SignatureError::BadKey)?;
        mac.update(format!("{timestamp}.").as_bytes());
        mac.update(payload);
        if !signatures
            .iter()
            .filter_map(|signature| hex::decode(signature).ok())
            .any(|signature| mac.clone().verify_slice(&signature).is_ok())
        {
            return Err(SignatureError::BadSignature);
        }
        if now.abs_diff(timestamp) > tolerance.as_secs() {
            return Err(SignatureError::BadTimestamp(timestamp));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Independently generated Python hmac test vector; the raw body includes whitespace.
    const BODY: &[u8] = b"{\"id\": \"evt_test\"}\n";
    const HEADER: &str =
        "t=1700000000,v1=15ad4a4db121468cadb0bd79e49e47c8426431043f6466d86e16555ea08e512d";

    #[test]
    fn authenticates_original_bytes_and_rotation_signatures() {
        let verify = |body, header: &str, now| {
            WebhookSignature::verify_at(body, header, "whsec_test", now, Duration::from_secs(300))
        };
        verify(BODY, HEADER, 1700000300).unwrap();
        verify(BODY, &format!("{HEADER},v1=not-hex,v0=ignored"), 1700000000).unwrap();
        verify(BODY, &HEADER.replace("v1=", "v1=bad,v1="), 1700000000).unwrap();
        assert!(verify(b"{\"id\":\"evt_test\"}", HEADER, 1700000000).is_err());
        assert!(matches!(verify(BODY, HEADER, 1700000301), Err(SignatureError::BadTimestamp(_))));
        assert!(verify(BODY, HEADER, 1699999699).is_err());
        assert!(verify(BODY, "t=1700000000,v0=ignored", 1700000000).is_err());
        assert!(verify(BODY, "t=oops,v1=00", 1700000000).is_err());
        assert!(verify(BODY, "v1=00", 1700000000).is_err());
        assert!(
            WebhookSignature::verify_at(
                BODY,
                HEADER,
                "wrong",
                1700000000,
                Duration::from_secs(300)
            )
            .is_err()
        );
    }
}
