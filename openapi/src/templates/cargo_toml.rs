use std::collections::BTreeMap;
use std::fmt::Write;

use indoc::formatdoc;

use crate::crates::Crate;

fn get_serialization_feature(deps: &[Crate], kind: &str) -> String {
    let mut features = vec![format!("async-stripe-types/{kind}")];
    for dep in deps {
        features.push(format!("{}/{kind}", dep.crate_name()));
    }
    features.into_iter().map(|f| format!(r#""{f}""#)).collect::<Vec<_>>().join(",")
}

pub fn gen_crate_toml(
    krate: Crate,
    crate_deps: Vec<Crate>,
    crate_features: Option<BTreeMap<String, Vec<String>>>,
    version: &str,
) -> String {
    let mut crate_dep_section = String::new();

    for dep in &crate_deps {
        let dep_name = dep.crate_name();
        let dep_path = format!("../../generated/{dep_name}");
        let _ = writeln!(
            crate_dep_section,
            // release-plz adds an extra space. let's add it here also to prevent churn
            r#"{dep_name} = {{path = "{dep_path}", version = "{version}" }}"#
        );
    }

    // Dependencies only needed for libraries which implement Stripe requests
    let request_deps = if krate == Crate::SHARED {
        None
    } else {
        Some(format!(
            // release-plz adds an extra space. let's add it here also to prevent churn
            r#"async-stripe-client-core = {{path = "../../async-stripe-client-core", version = "{version}" }}"#,
        ))
    };
    let request_deps = request_deps.as_deref().unwrap_or_default();

    let ser_features = get_serialization_feature(&crate_deps, "serialize");
    let deser_features = get_serialization_feature(&crate_deps, "deserialize");
    let redact_debug_features = get_serialization_feature(&crate_deps, "redact-generated-debug");

    let features = crate_features.map(gen_feature_section).unwrap_or_default();

    let crate_name = krate.crate_name();
    let lib_name = krate.lib_name();
    formatdoc! {
        r#"
        [package]
        name = "{crate_name}"
        version.workspace = true
        description.workspace = true
        edition.workspace = true
        rust-version.workspace = true
        authors.workspace = true
        license.workspace = true
        homepage.workspace = true
        repository.workspace = true
        keywords.workspace = true
        categories.workspace = true

        [lib]
        path = "src/mod.rs"
        name = "{lib_name}"

        [dependencies]
        serde.workspace = true
        serde_json = {{ workspace = true, optional = true }}
        smol_str.workspace = true
        miniserde.workspace = true
        tracing = {{ version = "0.1", default-features = false }}
        async-stripe-types = {{path = "../../async-stripe-types", version = "{version}" }}
        {request_deps}

        {crate_dep_section}

        [features]
        serialize = [{ser_features}]
        deserialize = [{deser_features}, "dep:serde_json"]
        redact-generated-debug = [{redact_debug_features}]
        {features}
        "#
    }
}

fn gen_feature_section(crate_features: BTreeMap<String, Vec<String>>) -> String {
    let mut feature_section = String::new();

    for (feature, forwarded) in &crate_features {
        let forwarded = forwarded.iter().map(|value| format!(r#""{value}""#)).collect::<Vec<_>>();
        let _ = writeln!(feature_section, "{feature} = [{}]", forwarded.join(","));
    }
    feature_section.push('\n');
    let _ = writeln!(
        feature_section,
        "full = [{}]",
        crate_features
            .keys()
            .map(|feature| format!(r#""{feature}""#))
            .collect::<Vec<_>>()
            .join(",\n")
    );

    formatdoc! {
        r#"
        {feature_section}

        [package.metadata.docs.rs]
        features = ["full"]
        "#
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{gen_crate_toml, gen_feature_section};
    use crate::crates::Crate;

    #[test]
    fn request_features_retain_the_existing_text_layout() {
        assert_eq!(
            gen_feature_section(BTreeMap::from([
                ("customer".into(), vec![]),
                ("charge".into(), vec![])
            ])),
            concat!(
                "charge = []\ncustomer = []\n\nfull = [\"charge\",\n\"customer\"]\n\n\n",
                "[package.metadata.docs.rs]\nfeatures = [\"full\"]\n"
            )
        );
    }

    #[test]
    fn model_features_render_forwarding_and_docs_for_shared() {
        let manifest = gen_crate_toml(
            Crate::SHARED,
            vec![],
            Some(BTreeMap::from([("charge".into(), vec!["async-stripe-core/charge".into()])])),
            "1.0.0",
        );
        let document: toml::Value = toml::from_str(&manifest).unwrap();
        assert_eq!(document["features"]["charge"][0].as_str(), Some("async-stripe-core/charge"));
        assert_eq!(document["features"]["full"][0].as_str(), Some("charge"));
        assert_eq!(
            document["package"]["metadata"]["docs"]["rs"]["features"][0].as_str(),
            Some("full")
        );
    }

    #[test]
    fn normal_shared_has_no_resource_features_or_docs_flags() {
        let manifest = gen_crate_toml(Crate::SHARED, vec![], None, "1.0.0");
        let document: toml::Value = toml::from_str(&manifest).unwrap();
        assert!(document["features"].get("full").is_none());
        assert!(document["package"].get("metadata").is_none());
    }
}
