use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use anyhow::{Context as _, ensure};

use crate::component_catalog::ComponentId;
use crate::crates::{Crate, CrateConfig};
use crate::item_catalog::{ItemCatalog, ItemId, RustItem};
use crate::item_layout::{EmissionSite, ItemAddress, ItemLayout};
use crate::model_availability::{Availability, ModelAvailability};
use crate::output::GeneratedWorkspace;
use crate::output_locations::OutputLocations;
use crate::rust_object::RustObject;
use crate::templates::utils::write_doc_comment;

/// Export available model modules and selected endpoint groups.
/// The handwritten manifest remains the source of transport/runtime settings.
pub(crate) fn write_facade(
    locations: OutputLocations<'_>,
    items: &ItemCatalog,
    layout: &ItemLayout,
    availability: &ModelAvailability,
    groups: &[(String, ComponentId)],
    version: &str,
    output: &mut GeneratedWorkspace,
) -> anyhow::Result<()> {
    let config = locations.config;
    let template = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../async-stripe/Cargo.toml"));
    let mut selected = availability
        .features()
        .iter()
        .map(|feature| (feature.clone(), BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
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

    let mut emitted_sites = std::collections::HashSet::new();
    for (item, record) in items.iter() {
        let site = layout.placement(item).emission_site();
        if !emitted_sites.insert(site) {
            continue;
        }
        let condition = availability.public_site(site);
        if *condition == Availability::Disabled {
            continue;
        }
        let (model_crate, paths) = match site {
            EmissionSite::ComponentTypes(owner) => {
                let krate = locations.types_crate(owner);
                let paths = if !locations.components.has_requests(owner)
                    || locations.types_split_from_requests(owner)
                {
                    let module = locations.components.get_by_id(owner).mod_path();
                    vec![format!("{}::{module}::*", krate.lib_name(config))]
                } else {
                    // Public helpers such as ChargeStatus have local render
                    // addresses, but this module reexports them at the crate root.
                    items
                        .iter()
                        .filter(|(id, record)| {
                            layout.placement(*id).emission_site() == site
                                && match &record.item {
                                    RustItem::Object(object) => match &object.definition {
                                        RustObject::Struct(strukt) => strukt.vis.is_public(),
                                        RustObject::Enum(_) | RustObject::FieldlessEnum(_) => true,
                                    },
                                    RustItem::IdNewtype
                                    | RustItem::OpaqueObject
                                    | RustItem::OpaqueRecord(_) => true,
                                }
                        })
                        .map(|(_, record)| {
                            format!("{}::{}", krate.lib_name(config), record.metadata.ident)
                        })
                        .collect()
                };
                (krate, paths)
            }
            EmissionSite::Standalone { item: owner, krate } => {
                if items.get(owner).is_reference_foundation() {
                    // Reference declarations were exported together above.
                    continue;
                }
                (
                    krate,
                    vec![format!(
                        "{}::{}",
                        krate.lib_name(config),
                        items.get(owner).metadata.ident
                    )],
                )
            }
            EmissionSite::ComponentRequests(_) | EmissionSite::Webhooks => continue,
        };
        match condition {
            Availability::Features(names) => {
                for name in names {
                    selected
                        .get_mut(name)
                        .expect("export feature was not registered")
                        .insert(model_crate);
                }
            }
            Availability::Always => ensure!(
                model_crate == Crate::SHARED,
                "unconditional facade model must be placed in shared: {}",
                record.metadata.ident
            ),
            Availability::Disabled => unreachable!(),
        }
        for path in paths {
            let _ = writeln!(exports, "{}pub use {path};", condition.cfg_attribute());
        }
    }

    for (feature, component) in groups {
        if locations.components.has_requests(*component) {
            let request_crate = locations.request_crate(*component);
            selected
                .get_mut(feature)
                .with_context(|| format!("facade request feature was not registered: {feature}"))?
                .insert(request_crate);
            let module = locations.components.get_by_id(*component).mod_path();
            let _ = writeln!(
                exports,
                "#[cfg(feature = {feature:?})]\npub use {}::{module};",
                request_crate.lib_name(config),
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
                request_crate.lib_name(config),
                builders.join(","),
            );
        }
    }
    write_record_extensions(items, layout, availability, config, &mut exports)?;
    let selected = selected
        .into_iter()
        .map(|(feature, crates)| {
            let targets = crates.into_iter().map(|krate| (krate, feature.clone())).collect();
            (feature, targets)
        })
        .collect::<Vec<_>>();
    let manifest = model_group_manifest(template, &selected, version, config)?;
    output.write("async-stripe/Cargo.toml", manifest)?;
    output.write("async-stripe/src/generated/mod.rs", exports)
}

/// Extension traits belong to the facade, which already depends on both model
/// packages. Typed access therefore creates no dependency between those packages.
fn write_record_extensions(
    items: &ItemCatalog,
    layout: &ItemLayout,
    availability: &ModelAvailability,
    config: &CrateConfig,
    out: &mut String,
) -> anyhow::Result<()> {
    let public_type = |item: ItemId| -> anyhow::Result<String> {
        let ItemAddress::CrateRoot(krate) = layout.placement(item).address() else {
            anyhow::bail!("extension decoder type must have a public model address: {:?}", item);
        };
        Ok(format!("{}::{}", krate.lib_name(config), items.get(item).metadata.ident))
    };
    let mut records = BTreeMap::<ItemId, Vec<_>>::new();
    for accessor in items.record_accessors() {
        if *availability.public_site(layout.placement(accessor.owner).emission_site())
            != Availability::Disabled
            && *availability.public_site(layout.placement(accessor.payload).emission_site())
                != Availability::Disabled
        {
            records.entry(accessor.owner).or_default().push(accessor);
        }
    }
    let mut prelude = String::from(
        "/// Traits for optional typed decoding of retained response fields.\npub mod prelude {\n",
    );
    for (owner, fields) in records {
        let owner_type = public_type(owner)?;
        let name = format!("{}Ext", items.get(owner).metadata.ident);
        let owner_cfg =
            availability.public_site(layout.placement(owner).emission_site()).cfg_attribute();
        let mut declarations = String::new();
        let mut implementations = String::new();
        for accessor in fields {
            let payload_type = public_type(accessor.payload)?;
            let method = accessor.field.variable_name();
            let cfg = availability
                .public_site(layout.placement(accessor.payload).emission_site())
                .cfg_attribute();
            let signature = format!(
                "fn {method}(&self) -> Result<Option<{payload_type}>, stripe_types::RecordDecodeError>"
            );
            if let Some(doc) = &accessor.field.doc_comment {
                declarations.push_str(&write_doc_comment(doc, 4));
            }
            declarations.push_str(&write_doc_comment("Decode this optional retained field. Each call clones and decodes only that subtree.\n\n# Errors\nReturns an error if a present non-null payload does not satisfy the selected model.", 4));
            let _ = writeln!(declarations, "{cfg}{signature};");
            let _ = writeln!(
                implementations,
                "{cfg}{signature} {{ self.field_as::<{payload_type}>({:?}) }}",
                accessor.field.wire_name()
            );
        }
        let _ = writeln!(
            out,
            "/// Optional typed field decoders. Import this trait or `stripe::prelude::*`.\n{owner_cfg}pub trait {name} {{\n{declarations}}}\n{owner_cfg}impl {name} for {owner_type} {{\n{implementations}}}"
        );
        let _ = writeln!(prelude, "{owner_cfg}pub use super::{name};");
    }
    prelude.push_str("}\n");
    out.push_str(&prelude);
    Ok(())
}

/// Update the generated model-group features and dependencies while preserving
/// the handwritten runtime settings shared by the facade and webhook crates.
pub(crate) fn model_group_manifest(
    template: &str,
    groups: &[(String, Vec<(Crate, String)>)],
    version: &str,
    config: &CrateConfig,
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
    let owned_packages = config
        .all()
        .filter(|krate| *krate != Crate::SHARED)
        .map(|krate| krate.crate_name(config))
        .collect::<BTreeSet<_>>();
    let mut domain_crates = BTreeSet::new();
    let mut group_features = BTreeMap::new();
    for (feature, targets) in groups {
        ensure!(
            !matches!(
                feature.as_str(),
                "full" | "serialize" | "deserialize" | "redact-generated-debug"
            ),
            "model group uses a reserved feature name: {feature}"
        );
        let mut forwarded = vec![];
        for (krate, target) in targets.iter().cloned().collect::<BTreeSet<_>>() {
            let package = krate.crate_name(config);
            if krate != Crate::SHARED {
                forwarded.push(toml::Value::String(format!("dep:{package}")));
                domain_crates.insert(krate);
            }
            forwarded.push(toml::Value::String(format!("{package}/{target}")));
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
            forwarded.push(toml::Value::String(format!("{}?/{feature}", krate.crate_name(config))));
        }
    }

    let dependencies = manifest
        .get_mut("dependencies")
        .and_then(toml::Value::as_table_mut)
        .context("model-group manifest has no dependency table")?;
    dependencies.retain(|package, _| !owned_packages.contains(package));
    for krate in &domain_crates {
        let package = krate.crate_name(config);
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

    fn groups() -> Vec<(String, Vec<(crate::crates::Crate, String)>)> {
        let core = crate::crates::test_config()
            .all()
            .find(|krate| krate.base_name(&crate::crates::test_config()) == "core")
            .unwrap();
        vec![
            ("balance_transaction".into(), vec![(core, "balance_transaction".into())]),
            ("charge".into(), vec![(core, "charge".into())]),
        ]
    }

    #[test]
    fn repeated_generation_preserves_runtime_settings_and_adds_features_once() {
        let first =
            model_group_manifest(TEMPLATE, &groups(), "1.0.0", &crate::crates::test_config())
                .unwrap();
        let second =
            model_group_manifest(&first, &groups(), "1.0.0", &crate::crates::test_config())
                .unwrap();
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
        let error =
            model_group_manifest(&template, &groups(), "1.0.0", &crate::crates::test_config())
                .unwrap_err();
        assert!(error.to_string().contains("conflicting mapping: charge"));
    }

    #[test]
    fn changing_selection_removes_owned_features_dependencies_and_forwarding() {
        let first =
            model_group_manifest(TEMPLATE, &groups(), "1.0.0", &crate::crates::test_config())
                .unwrap();
        let billing = crate::crates::test_config()
            .all()
            .find(|krate| krate.base_name(&crate::crates::test_config()) == "billing")
            .unwrap();
        let selected = vec![("invoice".to_string(), vec![(billing, "invoice".into())])];
        let second =
            model_group_manifest(&first, &selected, "2.0.0", &crate::crates::test_config())
                .unwrap();
        assert_eq!(
            second,
            model_group_manifest(&second, &selected, "2.0.0", &crate::crates::test_config())
                .unwrap()
        );
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
        let first =
            model_group_manifest(TEMPLATE, &groups(), "1.0.0", &crate::crates::test_config())
                .unwrap();
        let second =
            model_group_manifest(&first, &groups(), "2.0.0", &crate::crates::test_config())
                .unwrap();
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
        let groups = groups()
            .into_iter()
            .map(|(group, targets)| {
                (
                    group,
                    targets
                        .into_iter()
                        .map(|(krate, target)| (krate, format!("__model_{target}")))
                        .collect(),
                )
            })
            .collect::<Vec<_>>();
        let generated =
            model_group_manifest(template, &groups, "2.0.0", &crate::crates::test_config())
                .unwrap();
        let manifest: toml::Value = toml::from_str(&generated).unwrap();
        assert_eq!(manifest["features"]["full"][0].as_str(), Some("balance_transaction"));
        assert_eq!(manifest["features"]["full"][1].as_str(), Some("charge"));
        assert!(
            manifest["features"]["charge"]
                .as_array()
                .unwrap()
                .contains(&toml::Value::String("async-stripe-core/__model_charge".into()))
        );
        assert!(manifest["dependencies"].get("async-stripe-billing").is_none());
        assert_eq!(
            manifest["features"]["detailed-errors"][0].as_str(),
            Some("dep:serde_path_to_error")
        );
        assert_eq!(
            generated,
            model_group_manifest(&generated, &groups, "2.0.0", &crate::crates::test_config())
                .unwrap()
        );
    }
}
