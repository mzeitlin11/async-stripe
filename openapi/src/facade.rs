use std::fmt::Write as _;

use anyhow::{Context as _, ensure};

use crate::component_catalog::ComponentId;
use crate::crates::Crate;
use crate::item_catalog::{ItemCatalog, RustItem};
use crate::item_layout::{EmissionSite, ItemAddress, ItemLayout};
use crate::output::GeneratedWorkspace;
use crate::output_locations::OutputLocations;
use crate::rust_object::RustObject;

/// Use the selected model groups for both facade features and public exports.
/// The handwritten manifest remains the source of transport/runtime settings.
pub(crate) fn write_facade(
    locations: OutputLocations<'_>,
    items: &ItemCatalog,
    layout: &ItemLayout,
    groups: &[(String, ComponentId)],
    version: &str,
    output: &mut GeneratedWorkspace,
) -> anyhow::Result<()> {
    let template = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../async-stripe/Cargo.toml"));
    let mut selected = Vec::new();
    let mut exports = String::from("// Generated from the selected model groups.\n");

    let references = items
        .iter()
        .filter(|(_, record)| record.is_reference_foundation())
        .map(|(item, record)| {
            ensure!(
                layout.placement(item).address() == ItemAddress::CrateRoot(Crate::SHARED),
                "reference foundation must be placed in shared: {}",
                record.metadata.ident
            );
            Ok(record.metadata.ident.to_string())
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    if !references.is_empty() {
        // Existing handwritten root aliases may overlap these generated exports.
        let _ = writeln!(
            exports,
            "#[allow(unused_imports)]\npub use stripe_shared::{{{}}};",
            references.join(",")
        );
    }

    for (feature, component) in groups {
        let request_crate = locations.request_crate(*component);
        let item = items.component_root(*component);
        let placement = layout.placement(item);
        let ItemAddress::CrateRoot(model_crate) = placement.address() else {
            anyhow::bail!("facade model is not publicly addressable: {component:?}");
        };
        let mut crates = std::collections::BTreeSet::from([model_crate]);
        if locations.components.has_requests(*component) {
            crates.insert(request_crate);
        }
        selected.push((feature.clone(), crates.into_iter().collect::<Vec<_>>()));

        let paths = if items.get(item).is_reference_foundation() {
            // Lightweight references are already exported without a feature.
            Vec::new()
        } else {
            match placement.emission_site() {
                EmissionSite::ComponentTypes(owner) => {
                    if !locations.components.has_requests(owner)
                        || locations.types_split_from_requests(owner)
                    {
                        let module = locations.components.get_by_id(owner).mod_path();
                        vec![format!("{}::{module}::*", model_crate.lib_name())]
                    } else {
                        // The private `component::types` module is reexported at the
                        // model crate root. Export public declarations from that site.
                        items
                            .iter()
                            .filter(|(id, record)| {
                                layout.placement(*id).emission_site() == placement.emission_site()
                                    && match &record.item {
                                        RustItem::Object(object) => match &object.definition {
                                            RustObject::Struct(strukt) => strukt.vis.is_public(),
                                            RustObject::Enum(_) | RustObject::FieldlessEnum(_) => {
                                                true
                                            }
                                        },
                                        RustItem::IdNewtype | RustItem::OpaqueObject => true,
                                    }
                            })
                            .map(|(_, record)| {
                                format!("{}::{}", model_crate.lib_name(), record.metadata.ident)
                            })
                            .collect()
                    }
                }
                EmissionSite::Standalone { .. } => {
                    vec![format!("{}::{}", model_crate.lib_name(), items.get(item).metadata.ident)]
                }
                EmissionSite::ComponentRequests(_) | EmissionSite::Webhooks => {
                    anyhow::bail!("facade model has a local-only emission site: {component:?}");
                }
            }
        };
        for path in paths {
            let _ = writeln!(exports, "#[cfg(feature = {feature:?})]\npub use {path};");
        }
        if locations.components.has_requests(*component) {
            let module = locations.components.get_by_id(*component).mod_path();
            let _ = writeln!(
                exports,
                "#[cfg(feature = {feature:?})]\npub use {}::{module};",
                request_crate.lib_name(),
            );
            // Endpoint builders have stable root names. Helpers remain under
            // the resource module, where names shared by other groups are unambiguous.
            let builders = locations
                .components
                .requests_for_component(*component)
                .iter()
                .map(|request| request.ident.to_string())
                .collect::<Vec<_>>();
            let _ = writeln!(
                exports,
                "#[cfg(feature = {feature:?})]\npub use {}::{module}::{{{}}};",
                request_crate.lib_name(),
                builders.join(","),
            );
        }
    }
    let manifest = facade_manifest(template, &selected, version)?;
    output.write("async-stripe/Cargo.toml", manifest)?;
    output.write("async-stripe/src/generated/mod.rs", exports)
}

fn facade_manifest(
    template: &str,
    groups: &[(String, Vec<Crate>)],
    version: &str,
) -> anyhow::Result<String> {
    let mut manifest: toml::Value = toml::from_str(template)?;
    let mut domain_crates = std::collections::BTreeSet::new();
    for (feature, crates) in groups {
        let mut forwarded = vec![];
        for krate in crates {
            let package = krate.crate_name();
            if *krate != Crate::SHARED {
                forwarded.push(toml::Value::String(format!("dep:{package}")));
                domain_crates.insert(*krate);
            }
            forwarded.push(toml::Value::String(format!("{package}/{feature}")));
        }
        let forwarded = toml::Value::Array(forwarded);
        let features = manifest
            .get_mut("features")
            .and_then(toml::Value::as_table_mut)
            .context("facade manifest has no feature table")?;
        if let Some(previous) = features.get(feature) {
            ensure!(previous == &forwarded, "facade feature has a conflicting mapping: {feature}");
        } else {
            features.insert(feature.clone(), forwarded);
        }
    }

    for krate in domain_crates {
        let package = krate.crate_name();
        let dependency = toml::Table::from_iter([
            ("path".to_string(), toml::Value::String(format!("../generated/{package}"))),
            ("version".to_string(), toml::Value::String(version.to_string())),
            ("optional".to_string(), toml::Value::Boolean(true)),
            ("default-features".to_string(), toml::Value::Boolean(false)),
        ]);
        let dependencies = manifest
            .get_mut("dependencies")
            .and_then(toml::Value::as_table_mut)
            .context("facade manifest has no dependency table")?;
        let dependency = toml::Value::Table(dependency);
        if let Some(previous) = dependencies.get(&package) {
            ensure!(
                previous == &dependency,
                "facade dependency has a conflicting mapping: {package}"
            );
        } else {
            dependencies.insert(package.clone(), dependency);
        }

        let features = manifest.get_mut("features").and_then(toml::Value::as_table_mut).unwrap();
        for feature in ["serialize", "deserialize", "redact-generated-debug"] {
            let forwarded = features
                .entry(feature.to_string())
                .or_insert_with(|| toml::Value::Array(vec![]))
                .as_array_mut()
                .with_context(|| format!("facade feature is not an array: {feature}"))?;
            let entry = toml::Value::String(format!("{package}?/{feature}"));
            if !forwarded.contains(&entry) {
                forwarded.push(entry);
            }
        }
    }
    let features = manifest.get_mut("features").and_then(toml::Value::as_table_mut).unwrap();
    for feature in ["serialize", "deserialize"] {
        let forwarded = features
            .entry(feature.to_string())
            .or_insert_with(|| toml::Value::Array(vec![]))
            .as_array_mut()
            .with_context(|| format!("facade feature is not an array: {feature}"))?;
        let entry = toml::Value::String(format!("async-stripe-shared/{feature}"));
        if !forwarded.contains(&entry) {
            forwarded.push(entry);
        }
    }

    Ok(toml::to_string_pretty(&manifest)?)
}

#[cfg(test)]
mod tests {
    use super::facade_manifest;
    use crate::crates::ALL_CRATES;

    const TEMPLATE: &str = r#"
[package]
name = "example"
[dependencies]
custom-runtime = "1"
[features]
default = ["tls"]
tls = []
custom = ["tls"]
redact-generated-debug = ["custom"]
"#;

    fn groups() -> Vec<(String, Vec<crate::crates::Crate>)> {
        let core = *ALL_CRATES.iter().find(|krate| krate.base_name() == "core").unwrap();
        vec![("balance_transaction".to_string(), vec![core]), ("charge".to_string(), vec![core])]
    }

    #[test]
    fn repeated_generation_preserves_runtime_settings_and_adds_features_once() {
        let first = facade_manifest(TEMPLATE, &groups(), "1.0.0").unwrap();
        let second = facade_manifest(&first, &groups(), "1.0.0").unwrap();
        assert_eq!(first, second);
        let manifest: toml::Value = toml::from_str(&second).unwrap();
        assert_eq!(manifest["features"]["default"][0].as_str(), Some("tls"));
        assert_eq!(manifest["features"]["custom"][0].as_str(), Some("tls"));
        assert_eq!(manifest["dependencies"]["custom-runtime"].as_str(), Some("1"));
        let forwarded = manifest["features"]["deserialize"].as_array().unwrap();
        assert_eq!(forwarded.len(), 2);
        assert!(
            forwarded
                .iter()
                .any(|feature| feature.as_str() == Some("async-stripe-core?/deserialize"))
        );
    }

    #[test]
    fn conflicting_handwritten_feature_is_reported() {
        let template = format!("{TEMPLATE}\ncharge = [\"tls\"]\n");
        let error = facade_manifest(&template, &groups(), "1.0.0").unwrap_err();
        assert!(error.to_string().contains("conflicting mapping: charge"));
    }
}
