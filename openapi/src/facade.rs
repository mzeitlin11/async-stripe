use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use anyhow::{Context as _, ensure};

use crate::component_catalog::ComponentId;
use crate::crates::{ALL_CRATES, Crate};
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
    let manifest = model_group_manifest(template, &selected, version)?;
    output.write("async-stripe/Cargo.toml", manifest)?;
    output.write("async-stripe/src/generated/mod.rs", exports)
}

/// Update the generated model-group features and dependencies while preserving
/// the handwritten runtime settings shared by the facade and webhook crates.
pub(crate) fn model_group_manifest(
    template: &str,
    groups: &[(String, Vec<Crate>)],
    version: &str,
) -> anyhow::Result<String> {
    let mut manifest: toml::Value = toml::from_str(template)?;
    let previous_groups = manifest
        .get("package")
        .and_then(|package| package.get("metadata"))
        .and_then(|metadata| metadata.get("async-stripe-codegen"))
        .and_then(|metadata| metadata.get("model-groups"))
        .map(|groups| {
            groups
                .as_array()
                .context("generated model-groups metadata is not an array")?
                .iter()
                .map(|group| {
                    group
                        .as_str()
                        .map(str::to_string)
                        .context("generated model group is not a string")
                })
                .collect::<anyhow::Result<Vec<_>>>()
        })
        .transpose()?
        .unwrap_or_default();
    let owned_packages = ALL_CRATES
        .iter()
        .filter(|krate| **krate != Crate::SHARED)
        .map(|krate| krate.crate_name())
        .collect::<BTreeSet<_>>();
    let mut domain_crates = BTreeSet::new();
    let mut group_features = BTreeMap::new();
    for (feature, crates) in groups {
        ensure!(
            !matches!(
                feature.as_str(),
                "full" | "serialize" | "deserialize" | "redact-generated-debug"
            ),
            "model group uses a reserved feature name: {feature}"
        );
        let mut forwarded = vec![];
        for krate in crates.iter().copied().collect::<BTreeSet<_>>() {
            let package = krate.crate_name();
            if krate != Crate::SHARED {
                forwarded.push(toml::Value::String(format!("dep:{package}")));
                domain_crates.insert(krate);
            }
            forwarded.push(toml::Value::String(format!("{package}/{feature}")));
        }
        ensure!(
            group_features.insert(feature.clone(), toml::Value::Array(forwarded)).is_none(),
            "duplicate model group: {feature}"
        );
    }

    let features = manifest
        .get_mut("features")
        .and_then(toml::Value::as_table_mut)
        .context("model-group manifest has no feature table")?;
    for feature in previous_groups {
        features.remove(&feature);
    }
    for (feature, forwarded) in &group_features {
        if let Some(previous) = features.get(feature) {
            ensure!(
                previous == forwarded,
                "model-group feature has a conflicting mapping: {feature}"
            );
        } else {
            features.insert(feature.clone(), forwarded.clone());
        }
    }
    let group_names = group_features.keys().cloned().map(toml::Value::String).collect::<Vec<_>>();
    features.insert("full".to_string(), toml::Value::Array(group_names.clone()));

    for feature in ["serialize", "deserialize", "redact-generated-debug"] {
        let forwarded = features
            .entry(feature.to_string())
            .or_insert_with(|| toml::Value::Array(vec![]))
            .as_array_mut()
            .with_context(|| format!("model-group feature is not an array: {feature}"))?;
        // These entries were generated alongside the optional domain dependencies.
        // Rebuild them so changing the selected groups cannot leave stale links.
        forwarded.retain(|entry| {
            let Some(entry) = entry.as_str() else { return true };
            let package = entry
                .split('/')
                .next()
                .unwrap_or(entry)
                .trim_end_matches('?')
                .trim_start_matches("dep:");
            !owned_packages.contains(package)
        });
        let shared = toml::Value::String(format!("async-stripe-shared/{feature}"));
        if !forwarded.contains(&shared) {
            forwarded.push(shared);
        }
        for krate in &domain_crates {
            forwarded.push(toml::Value::String(format!("{}?/{feature}", krate.crate_name())));
        }
    }

    let dependencies = manifest
        .get_mut("dependencies")
        .and_then(toml::Value::as_table_mut)
        .context("model-group manifest has no dependency table")?;
    dependencies.retain(|package, _| !owned_packages.contains(package));
    for krate in &domain_crates {
        let package = krate.crate_name();
        let dependency = toml::Table::from_iter([
            ("path".to_string(), toml::Value::String(format!("../generated/{package}"))),
            ("version".to_string(), toml::Value::String(version.to_string())),
            ("optional".to_string(), toml::Value::Boolean(true)),
            ("default-features".to_string(), toml::Value::Boolean(false)),
        ]);
        dependencies.insert(package, toml::Value::Table(dependency));
    }

    let package = manifest
        .get_mut("package")
        .and_then(toml::Value::as_table_mut)
        .context("model-group manifest has no package table")?;
    let metadata = package
        .entry("metadata")
        .or_insert_with(|| toml::Value::Table(toml::Table::new()))
        .as_table_mut()
        .context("package metadata is not a table")?;
    let codegen = metadata
        .entry("async-stripe-codegen")
        .or_insert_with(|| toml::Value::Table(toml::Table::new()))
        .as_table_mut()
        .context("codegen metadata is not a table")?;
    codegen.insert("model-groups".into(), toml::Value::Array(group_names));

    Ok(toml::to_string_pretty(&manifest)?)
}

#[cfg(test)]
mod tests {
    use super::model_group_manifest;
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
        let first = model_group_manifest(TEMPLATE, &groups(), "1.0.0").unwrap();
        let second = model_group_manifest(&first, &groups(), "1.0.0").unwrap();
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
        let error = model_group_manifest(&template, &groups(), "1.0.0").unwrap_err();
        assert!(error.to_string().contains("conflicting mapping: charge"));
    }

    #[test]
    fn changing_selection_removes_owned_features_dependencies_and_forwarding() {
        let first = model_group_manifest(TEMPLATE, &groups(), "1.0.0").unwrap();
        let billing = *ALL_CRATES.iter().find(|krate| krate.base_name() == "billing").unwrap();
        let selected = vec![("invoice".to_string(), vec![billing])];
        let second = model_group_manifest(&first, &selected, "2.0.0").unwrap();
        assert_eq!(second, model_group_manifest(&second, &selected, "2.0.0").unwrap());
        let manifest: toml::Value = toml::from_str(&second).unwrap();
        assert!(manifest["features"].get("balance_transaction").is_none());
        assert!(manifest["features"].get("charge").is_none());
        assert_eq!(
            manifest["features"]["full"].as_array().unwrap(),
            &[toml::Value::String("invoice".into())]
        );
        assert!(manifest["dependencies"].get("async-stripe-core").is_none());
        let dependency = &manifest["dependencies"]["async-stripe-billing"];
        assert_eq!(dependency["version"].as_str(), Some("2.0.0"));
        assert_eq!(dependency["optional"].as_bool(), Some(true));
        assert_eq!(dependency["default-features"].as_bool(), Some(false));
        assert_eq!(manifest["dependencies"]["custom-runtime"].as_str(), Some("1"));
        assert_eq!(manifest["features"]["custom"][0].as_str(), Some("tls"));
        for feature in ["serialize", "deserialize", "redact-generated-debug"] {
            let forwarded = manifest["features"][feature].as_array().unwrap();
            assert!(
                forwarded
                    .contains(&toml::Value::String(format!("async-stripe-billing?/{feature}")))
            );
            assert!(
                !forwarded
                    .iter()
                    .any(|entry| entry.as_str().unwrap().contains("async-stripe-core"))
            );
        }
        assert_eq!(manifest["features"]["redact-generated-debug"][0].as_str(), Some("custom"));
    }

    #[test]
    fn changing_version_updates_existing_owned_dependencies() {
        let first = model_group_manifest(TEMPLATE, &groups(), "1.0.0").unwrap();
        let second = model_group_manifest(&first, &groups(), "2.0.0").unwrap();
        let manifest: toml::Value = toml::from_str(&second).unwrap();
        assert_eq!(
            manifest["dependencies"]["async-stripe-core"]["version"].as_str(),
            Some("2.0.0")
        );
    }

    #[test]
    fn webhook_domain_dependencies_and_full_are_replaced_by_model_groups() {
        let template = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../async-stripe-webhook/Cargo.toml"
        ));
        let generated = model_group_manifest(template, &groups(), "2.0.0").unwrap();
        let manifest: toml::Value = toml::from_str(&generated).unwrap();
        assert_eq!(manifest["features"]["full"][0].as_str(), Some("balance_transaction"));
        assert_eq!(manifest["features"]["full"][1].as_str(), Some("charge"));
        assert!(manifest["dependencies"].get("async-stripe-billing").is_none());
        assert_eq!(
            manifest["features"]["detailed-errors"][0].as_str(),
            Some("dep:serde_path_to_error")
        );
        assert_eq!(generated, model_group_manifest(&generated, &groups(), "2.0.0").unwrap());
    }
}
