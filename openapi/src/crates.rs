use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::path::PathBuf;

use anyhow::{anyhow, bail, ensure};
use lazy_static::lazy_static;
use serde::Deserialize;

use crate::component_catalog::{ComponentCatalog, ComponentId};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Crate {
    base_name: &'static str,
}

impl Crate {
    pub const SHARED: Self = Self { base_name: "shared" };

    pub fn generated_out_path(self) -> String {
        format!("crates/{}", self.crate_name())
    }

    pub fn base_name(self) -> &'static str {
        self.base_name
    }

    pub fn lib_name(self) -> String {
        format!("stripe_{}", self.base_name)
    }

    pub fn crate_name(self) -> String {
        format!("async-stripe-{}", self.base_name)
    }

    fn generate_to(self) -> String {
        let out_path = self.generated_out_path();
        format!("{out_path}/src")
    }

    pub fn get_path(self) -> PathBuf {
        PathBuf::from(self.generate_to())
    }

    pub fn get_path_to_mod(self) -> PathBuf {
        self.get_path().join("mod.rs")
    }
}

/// Information about an automatically generated crate.
#[derive(Deserialize)]
pub struct CrateGen {
    /// Name of the crate
    pub name: String,
    /// Component paths which should live in this crate
    #[serde(default)]
    pub paths: Vec<String>,
    /// Package names which should live in this crate
    #[serde(default)]
    pub packages: Vec<String>,
    /// Used to generate a top-level comment for the crate.
    pub description: String,
}

#[derive(Deserialize)]
pub struct CrateInfo {
    pub crates: Vec<CrateGen>,
}

fn load_crate_info() -> anyhow::Result<CrateInfo> {
    let toml_str = read_to_string("gen_crates.toml")?;
    let loaded = toml::from_str(&toml_str)?;
    Ok(loaded)
}

lazy_static! {
    /// Content of `gen_crates.toml`
    pub static ref CRATE_INFO: CrateInfo = load_crate_info().expect("Could not load crate info");
    /// All crate names
    pub static ref ALL_CRATES: Vec<Crate> =
        CRATE_INFO.crates.iter().map(|c| Crate {base_name: &c.name}).collect();
}

#[track_caller]
fn crate_info_unwrapped(krate: Crate) -> &'static CrateGen {
    CRATE_INFO.crates.iter().find(|k| k.name == krate.base_name).expect("Crate not found")
}

pub fn get_crate_doc_comment(krate: Crate) -> &'static str {
    &crate_info_unwrapped(krate).description
}

/// Validated, identity-based view of `gen_crates.toml` used during inference.
///
/// Resolving this once keeps configuration lookup and validation out of the
/// layout algorithm. Component assignments use catalog identities after this
/// boundary; only package names remain strings because they are external spec
/// labels rather than component identities.
pub struct ResolvedCrateConfig {
    by_component: HashMap<ComponentId, Crate>,
    by_package: HashMap<String, Crate>,
}

impl ResolvedCrateConfig {
    pub fn from_catalog(catalog: &ComponentCatalog) -> anyhow::Result<Self> {
        Self::resolve(&CRATE_INFO, catalog)
    }

    fn resolve(info: &'static CrateInfo, catalog: &ComponentCatalog) -> anyhow::Result<Self> {
        let mut crate_names = HashSet::new();
        let mut by_component = HashMap::new();
        let mut by_package = HashMap::new();

        for configured in &info.crates {
            ensure!(!configured.name.is_empty(), "crate configuration contains an empty name");
            ensure!(
                crate_names.insert(configured.name.as_str()),
                "crate `{}` is configured more than once",
                configured.name
            );
            let krate = Crate { base_name: &configured.name };

            for path in &configured.paths {
                ensure!(
                    !path.is_empty(),
                    "crate `{}` contains an empty component path",
                    krate.base_name()
                );
                let component =
                    catalog.id_str(path).ok_or_else(|| unknown_component(path, catalog))?;
                if let Some(previous) = by_component.insert(component, krate) {
                    bail!(
                        "component `{path}` is assigned to both crate `{}` and crate `{}`",
                        previous.base_name(),
                        krate.base_name()
                    );
                }
            }

            for package in &configured.packages {
                ensure!(
                    !package.is_empty(),
                    "crate `{}` contains an empty package name",
                    krate.base_name()
                );
                if let Some(previous) = by_package.insert(package.clone(), krate) {
                    bail!(
                        "package `{package}` is assigned to both crate `{}` and crate `{}`",
                        previous.base_name(),
                        krate.base_name()
                    );
                }
            }
        }

        ensure!(
            info.crates.iter().filter(|krate| krate.name == Crate::SHARED.base_name()).count() == 1,
            "crate configuration must contain exactly one `shared` crate"
        );

        // Deleted object variants follow the explicit assignment of their
        // non-deleted object, preserving the existing placement rule while
        // making the result a direct ComponentId lookup.
        for (component, name) in catalog.iter() {
            if !name.is_deleted_obj() {
                continue;
            }
            let base_name = name.as_not_deleted();
            let Some(base_component) = catalog.id(&base_name) else {
                continue;
            };
            let Some(&base_crate) = by_component.get(&base_component) else {
                continue;
            };
            if let Some(explicit) = by_component.insert(component, base_crate) {
                ensure!(
                    explicit == base_crate,
                    "deleted component `{name}` is assigned to crate `{}` but its base component `{base_name}` is assigned to crate `{}`",
                    explicit.base_name(),
                    base_crate.base_name()
                );
            }
        }

        Ok(Self { by_component, by_package })
    }

    /// Assign the crate a package should live in. Every package is expected to
    /// be specified in `gen_crates.toml`.
    pub fn crate_for_package(&self, package: &str) -> anyhow::Result<Crate> {
        self.by_package.get(package).copied().ok_or_else(|| {
            let mut configured = self.by_package.keys().map(String::as_str).collect::<Vec<_>>();
            configured.sort_unstable();
            anyhow!(
                "no crate mapping found for package `{package}`; add it to gen_crates.toml (configured packages: {})",
                configured.join(", ")
            )
        })
    }

    pub fn crate_for_component(&self, component: ComponentId) -> Option<Crate> {
        self.by_component.get(&component).copied()
    }
}

fn unknown_component(path: &str, catalog: &ComponentCatalog) -> anyhow::Error {
    let mut candidates = catalog
        .iter()
        .map(|(_, name)| (edit_distance(path, name), name.as_ref()))
        .collect::<Vec<_>>();
    candidates.sort_unstable();
    let candidates = candidates
        .into_iter()
        .take(3)
        .map(|(_, name)| format!("`{name}`"))
        .collect::<Vec<_>>()
        .join(", ");
    anyhow!("crate configuration references unknown component `{path}`; nearest: {candidates}")
}

fn edit_distance(left: &str, right: &str) -> usize {
    let mut previous = (0..=right.chars().count()).collect::<Vec<_>>();
    let mut current = vec![0; previous.len()];
    for (left_index, left_char) in left.chars().enumerate() {
        current[0] = left_index + 1;
        for (right_index, right_char) in right.chars().enumerate() {
            current[right_index + 1] = (previous[right_index + 1] + 1)
                .min(current[right_index] + 1)
                .min(previous[right_index] + usize::from(left_char != right_char));
        }
        std::mem::swap(&mut previous, &mut current);
    }
    previous[right.chars().count()]
}

#[cfg(test)]
mod tests {
    use super::{CrateGen, CrateInfo, ResolvedCrateConfig};
    use crate::component_catalog::ComponentCatalog;
    use crate::types::ComponentName;

    fn catalog() -> ComponentCatalog {
        ComponentCatalog::from_names([
            ComponentName::new("charge".into()),
            ComponentName::new("invoice".into()),
            ComponentName::new("deleted_charge".into()),
        ])
        .unwrap()
    }

    fn config(crates: Vec<CrateGen>) -> &'static CrateInfo {
        Box::leak(Box::new(CrateInfo { crates }))
    }

    fn krate(name: &str, paths: &[&str], packages: &[&str]) -> CrateGen {
        CrateGen {
            name: name.into(),
            paths: paths.iter().map(|value| (*value).into()).collect(),
            packages: packages.iter().map(|value| (*value).into()).collect(),
            description: String::new(),
        }
    }

    #[test]
    fn resolves_paths_to_component_ids_and_propagates_deleted_placement() {
        let catalog = catalog();
        let info = config(vec![krate("core", &["charge"], &["core"]), krate("shared", &[], &[])]);
        let resolved = ResolvedCrateConfig::resolve(info, &catalog).unwrap();

        let charge = catalog.id_str("charge").unwrap();
        let deleted_charge = catalog.id_str("deleted_charge").unwrap();
        assert_eq!(resolved.crate_for_component(charge).unwrap().base_name(), "core");
        assert_eq!(resolved.crate_for_component(deleted_charge).unwrap().base_name(), "core");
        assert_eq!(resolved.crate_for_package("core").unwrap().base_name(), "core");
    }

    #[test]
    fn rejects_duplicate_component_owners() {
        let info = config(vec![
            krate("core", &["charge"], &[]),
            krate("payment", &["charge"], &[]),
            krate("shared", &[], &[]),
        ]);
        let error = ResolvedCrateConfig::resolve(info, &catalog()).err().unwrap().to_string();

        assert!(error.contains("assigned to both crate `core` and crate `payment`"));
    }

    #[test]
    fn unknown_component_error_suggests_nearby_names() {
        let info = config(vec![krate("shared", &["charg"], &[])]);
        let error = ResolvedCrateConfig::resolve(info, &catalog()).err().unwrap().to_string();

        assert!(error.contains("unknown component `charg`"));
        assert!(error.contains("`charge`"));
    }
}
