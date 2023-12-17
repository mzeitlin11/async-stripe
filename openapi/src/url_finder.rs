use anyhow::{anyhow, Result};
use heck::ToSnakeCase;
use reqwest::blocking::Client;
use tracing::warn;

// we use a common user agent, otherwise stripe rejects the connection
const APP_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.71 Safari/537.36";

#[derive(Debug)]
pub struct UrlFinder {
    flattened_api_sections: serde_json::Map<String, serde_json::Value>,
}

impl UrlFinder {
    pub fn new() -> Result<Self> {
        let client = Client::builder().user_agent(APP_USER_AGENT).build()?;
        let resp = client.get("https://stripe.com/docs/api").send()?;

        if resp.status().is_success() {
            let text = resp.text()?;
            if let Some(line) = text.lines().find(|l| l.contains("flattenedAPISections: {")) {
                Ok(Self {
                    flattened_api_sections: serde_json::from_str(
                        line.trim()
                            .trim_start_matches("flattenedAPISections: ")
                            .trim_end_matches(','),
                    )
                    .expect("should be valid json"),
                })
            } else {
                warn!("Stripe API returned unexpected document, not collecting doc URL's");
                Ok(Self { flattened_api_sections: serde_json::Map::new() })
            }
        } else {
            tracing::error!("{}", resp.text()?);
            Err(anyhow!("request to stripe api returned non-200 status code"))
        }
    }

    /// Create a stub `UrlFinder` which does not require a network request. Only meant to
    /// be used for testing since no `doc_url`'s will be found.
    pub fn stub() -> Self {
        Self { flattened_api_sections: serde_json::Map::new() }
    }

    pub fn url_for_object(&self, object: &str) -> Option<String> {
        let object_name = object.replace('.', "_").to_snake_case();
        let object_names = [format!("{}_object", object_name), object_name];
        for name in object_names {
            if let Some(path) = self
                .flattened_api_sections
                .get(&name)
                .and_then(|o| o.as_object().expect("this should be an object").get("path"))
                .and_then(|s| s.as_str())
            {
                return Some(format!("https://stripe.com/docs/api{}", path));
            }
        }

        // Only warn if not the stub `UrlFinder`
        if !self.flattened_api_sections.is_empty() {
            tracing::debug!("{} not in html", object);
        }

        None
    }
}
