//! Generate the opaque v1 experiment without installing it.
//!
//! Run from `openapi`: `cargo run --example opaque_model_groups -- OUTPUT [GROUP ...]`.
//! The output is unformatted; format the generated workspace before compiling it.

use std::path::PathBuf;

use anyhow::Context as _;
use stripe_openapi_codegen::analysis::GraphDetail;
use stripe_openapi_codegen::codegen::CodeGen;
use stripe_openapi_codegen::spec::Spec;
use stripe_openapi_codegen::spec_fetch::load_pinned_spec;
use stripe_openapi_codegen::url_finder::UrlFinder;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let output =
        PathBuf::from(args.next().context("usage: opaque_model_groups OUTPUT [GROUP ...]")?);
    let mut groups = args.collect::<Vec<_>>();
    if groups.is_empty() {
        groups = vec!["balance_transaction".to_string(), "charge".to_string()];
    }
    let workspace: toml::Value =
        toml::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../Cargo.toml")))?;
    let version = workspace["workspace"]["package"]["version"]
        .as_str()
        .context("workspace package has no version")?
        .to_string();
    let groups = groups.iter().map(String::as_str).collect::<Vec<_>>();
    let codegen = CodeGen::new_opaque_model_groups(
        Spec::new(load_pinned_spec()?),
        UrlFinder::new()?,
        version,
        &groups,
    )?;
    codegen.render_workspace()?.materialize(&output)?;
    codegen.analysis_bundle(GraphDetail::Default).write_to(&output.join("analysis"))?;
    println!("Generated opaque model groups {} into {}", groups.join(", "), output.display());
    Ok(())
}
