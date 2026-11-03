use std::fs;

use anyhow::Context;
use openapiv3::OpenAPI;
use regex_lite::Regex;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use serde_json::Value;
use tracing::info;

const SPEC_FILE_PATH: &str = "spec3.sdk.json";
const VERSION_FILE_PATH: &str = "version.json";
const FIXTURES_FILE_PATH: &str = "fixtures.json";

#[derive(Debug, Clone)]
pub enum SpecUpdate {
    Latest,
    Version(String),
}

pub fn parse_spec_update(arg: &str) -> anyhow::Result<SpecUpdate> {
    let as_lower = arg.to_lowercase();
    match as_lower.as_str() {
        "latest" => Ok(SpecUpdate::Latest),
        version => {
            let re = Regex::new(r"^v[1-9]\d*$").unwrap();
            if re.is_match(version) {
                Ok(SpecUpdate::Version(as_lower))
            } else {
                Err(anyhow::anyhow!(
                    "Spec update must be `latest` or match `v###`, for example `v172`"
                ))
            }
        }
    }
}

fn get_latest_openapi_tag(client: &Client) -> anyhow::Result<String> {
    let tags: Value = client
        .get("https://api.github.com/repos/stripe/openapi/releases/latest")
        // Github requires a user agent with the following recommendations:
        // https://docs.github.com/en/rest/overview/resources-in-the-rest-api#user-agent-required
        .header(USER_AGENT, "async-stripe")
        .send()?
        .error_for_status()?
        .json()?;
    Ok(tags.as_object().context("Unexpected response format")?["tag_name"]
        .as_str()
        .context("Unexpected tag name")?
        .to_string())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct VersionFile {
    version: String,
}

fn get_pinned_openapi_tag() -> anyhow::Result<String> {
    let raw = fs::File::open(VERSION_FILE_PATH).context("Could not find version file")?;
    let version_info: VersionFile =
        serde_json::from_reader(&raw).context("Failed to read JSON from version file")?;
    Ok(version_info.version)
}

pub fn load_pinned_spec() -> anyhow::Result<OpenAPI> {
    let client = Client::new();
    let desired_version = get_pinned_openapi_tag()?;
    load_spec_version(&client, &desired_version)
}

pub fn load_generation_spec(update: Option<SpecUpdate>) -> anyhow::Result<OpenAPI> {
    let client = Client::new();
    let updates_pin = update.is_some();
    let desired_version = match update {
        Some(SpecUpdate::Latest) => get_latest_openapi_tag(&client)?,
        Some(SpecUpdate::Version(version)) => version,
        None => get_pinned_openapi_tag()?,
    };

    let spec = load_spec_version(&client, &desired_version)?;
    ensure_correct_fixtures_file(&client, &desired_version)?;
    if updates_pin {
        write_pinned_openapi_tag(&desired_version)?;
    }

    Ok(spec)
}

fn load_spec_version(client: &Client, desired_version: &str) -> anyhow::Result<OpenAPI> {
    if let Some(spec) = fs::File::open(SPEC_FILE_PATH)
        .ok()
        .and_then(|file| serde_json::from_reader(file).ok())
        .filter(|spec| read_x_stripe_tag(spec) == Some(desired_version))
    {
        info!("Using cached OpenAPI spec version {}", desired_version);
        Ok(spec)
    } else {
        fetch_spec(client, desired_version)
    }
}

fn fetch_spec(client: &Client, desired_version: &str) -> anyhow::Result<OpenAPI> {
    info!("Fetching OpenAPI spec version {}", desired_version);
    let url = format!(
        "https://raw.githubusercontent.com/stripe/openapi/{desired_version}/openapi/spec3.sdk.json"
    );
    let mut spec: OpenAPI = client.get(url).send()?.error_for_status()?.json()?;
    write_x_stripe_tag(&mut spec, desired_version);

    let writer = fs::File::create(SPEC_FILE_PATH)?;
    serde_json::to_writer_pretty(writer, &spec)?;
    info!("Wrote OpenAPI spec to {}", SPEC_FILE_PATH);
    Ok(spec)
}

fn write_pinned_openapi_tag(version: &str) -> anyhow::Result<()> {
    if matches!(get_pinned_openapi_tag().as_deref(), Ok(pinned) if pinned == version) {
        return Ok(());
    }
    let writer = fs::File::create(VERSION_FILE_PATH)?;
    serde_json::to_writer_pretty(writer, &VersionFile { version: version.to_string() })?;
    Ok(())
}

fn ensure_correct_fixtures_file(client: &Client, desired_version: &str) -> anyhow::Result<()> {
    let matches = fs::File::open(FIXTURES_FILE_PATH)
        .ok()
        .and_then(|file| serde_json::from_reader::<_, VersionFile>(file).ok())
        .is_some_and(|fixtures| fixtures.version == desired_version);
    if matches { Ok(()) } else { fetch_new_fixtures(client, desired_version) }
}

fn fetch_new_fixtures(client: &Client, desired_version: &str) -> anyhow::Result<()> {
    info!("Fetching new fixtures file");
    let url = format!(
        "https://raw.githubusercontent.com/stripe/openapi/{desired_version}/openapi/fixtures3.json"
    );
    let mut spec: Value = client.get(url).send()?.error_for_status()?.json()?;
    spec.as_object_mut()
        .context("expected object")?
        .insert("version".into(), Value::String(desired_version.into()));

    let writer = fs::File::create(FIXTURES_FILE_PATH)?;
    serde_json::to_writer_pretty(writer, &spec)?;
    info!("Finished writing fixtures file");
    Ok(())
}

fn write_x_stripe_tag(spec: &mut OpenAPI, version: &str) {
    spec.info.extensions.insert("x-stripeTag".to_string(), version.into());
}

fn read_x_stripe_tag(spec: &OpenAPI) -> Option<&str> {
    spec.info.extensions.get("x-stripeTag")?.as_str()
}
