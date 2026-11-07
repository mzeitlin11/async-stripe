use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use anyhow::{Context, Result, anyhow, bail};
use heck::ToSnakeCase;
use reqwest::blocking::Client;
use tracing::{info, warn};

use crate::components::Components;

// we use a common user agent, otherwise stripe rejects the connection
const APP_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.71 Safari/537.36";

#[derive(Debug, Default)]
pub struct UrlFinder {
    /// Map of object name to stripe URL path (unprefixed)
    doc_links: HashMap<String, String>,
}

impl UrlFinder {
    pub fn new() -> Result<Self> {
        Self::load(Path::new("doc_urls.json"))
    }

    pub fn load(path: &Path) -> Result<Self> {
        let file = File::open(path).with_context(|| {
            format!("could not open documentation URL mapping {}", path.display())
        })?;
        let doc_links = serde_json::from_reader(file).with_context(|| {
            format!("could not parse documentation URL mapping {}", path.display())
        })?;
        Ok(Self { doc_links })
    }

    pub fn url_for_object(&self, object: &str) -> Option<String> {
        let unprefixed_link = self.doc_links.get(object)?;
        Some(format!("https://stripe.com/docs/api{unprefixed_link}"))
    }
}

fn url_for_object(
    object: &str,
    raw_data: &serde_json::Map<String, serde_json::Value>,
) -> Option<String> {
    let object_name = object.replace('.', "_").to_snake_case();
    let object_names = [format!("{object_name}_object"), object_name];
    for name in object_names {
        if let Some(path) = raw_data
            .get(&name)
            .and_then(|o| o.as_object().expect("this should be an object").get("path"))
            .and_then(|s| s.as_str())
        {
            return Some(path.to_string());
        }
    }
    None
}

pub fn update_api_doc_data(api_url: &str, components: &Components) -> Result<()> {
    let flattened = fetch_flattened_api_data(api_url)?;

    let mut new_doc_data: HashMap<String, String> = HashMap::new();
    for &component in components.components.keys() {
        let path = components.component_name(component);
        if let Some(doc_url) = url_for_object(path.as_ref(), &flattened) {
            new_doc_data.insert(path.to_string(), doc_url);
        } else {
            warn!("No doc url found for path `{path}`");
        }
    }

    serde_json::to_writer_pretty(File::create("doc_urls.json")?, &new_doc_data)?;
    info!("Wrote updated data to `doc_urls.json`");
    Ok(())
}

fn fetch_flattened_api_data(api_url: &str) -> Result<serde_json::Map<String, serde_json::Value>> {
    let client = Client::builder().user_agent(APP_USER_AGENT).build()?;
    let resp = client.get(api_url).send()?;

    if resp.status().is_success() {
        let text = resp.text()?;
        let Some(line) = text.lines().find(|l| l.contains("flattenedAPISections: {")) else {
            bail!("Unexpected Stripe API doc format")
        };
        let api_sections = serde_json::from_str(
            line.trim().trim_start_matches("flattenedAPISections: ").trim_end_matches(','),
        )?;
        Ok(api_sections)
    } else {
        tracing::error!("{}", resp.text()?);
        Err(anyhow!("request to stripe api returned non-200 status code"))
    }
}

#[cfg(test)]
mod tests {
    use super::UrlFinder;

    #[test]
    fn default_mapping_needs_no_documentation_file() {
        assert!(UrlFinder::default().url_for_object("customer").is_none());
    }

    #[test]
    fn loads_documentation_mapping_from_an_explicit_path() {
        let path = std::env::temp_dir().join(format!(
            "async-stripe-doc-urls-{}-{}.json",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        std::fs::write(&path, r#"{"customer":"/customers/object"}"#).unwrap();
        let result = UrlFinder::load(&path);
        std::fs::remove_file(path).unwrap();
        let finder = result.unwrap();
        assert_eq!(
            finder.url_for_object("customer").as_deref(),
            Some("https://stripe.com/docs/api/customers/object")
        );
        assert!(finder.url_for_object("unknown").is_none());
    }
}
