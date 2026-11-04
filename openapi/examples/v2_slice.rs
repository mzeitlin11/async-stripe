//! Explicit-input experiment; never installs output or updates the production pin.
use std::collections::{BTreeSet, VecDeque};
use std::path::PathBuf;

use anyhow::{Context, Result, ensure};
use clap::Parser;
use serde_json::Value;
use stripe_openapi_codegen::codegen::CodeGen;
use stripe_openapi_codegen::spec::Spec;
use stripe_openapi_codegen::url_finder::UrlFinder;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    spec: PathBuf,
    #[arg(long)]
    out: PathBuf,
    #[arg(long, default_value = "experiments/v2/gen_crates.toml")]
    layout: PathBuf,
    #[arg(long, value_delimiter = ',', default_value = "v2.core.event_destination,v2.core.event")]
    resources: Vec<String>,
    /// Thin event schema names, independently of the related resource namespace.
    #[arg(long, value_delimiter = ',')]
    events: Vec<String>,
}

fn references(value: &Value, into: &mut VecDeque<String>) {
    match value {
        Value::Object(object) => {
            if let Some(reference) = object.get("$ref").and_then(Value::as_str)
                && let Some(name) = reference.strip_prefix("#/components/schemas/")
            {
                into.push_back(name.to_owned());
            }
            for value in object.values() {
                references(value, into);
            }
        }
        Value::Array(array) => {
            for value in array {
                references(value, into);
            }
        }
        _ => {}
    }
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let out = std::path::absolute(args.out)?;
    let layout = args.layout.canonicalize()?;
    let mut document: Value = serde_json::from_slice(&std::fs::read(&args.spec)?)?;
    let all_schemas =
        document["components"]["schemas"].as_object().context("missing schemas")?.clone();
    let all_paths = document["paths"].as_object().context("missing paths")?.clone();
    let mut pending = VecDeque::from_iter(args.resources.iter().chain(&args.events).cloned());
    let mut selected = BTreeSet::new();
    let mut paths = serde_json::Map::new();
    while let Some(name) = pending.pop_front() {
        if !selected.insert(name.clone()) {
            continue;
        }
        let schema = all_schemas.get(&name).with_context(|| format!("unknown schema: {name}"))?;
        ensure!(
            name.starts_with("v2.") || schema["x-stripeEvent"]["kind"] == "thin",
            "non-v2 resource in experimental closure: {name}"
        );
        references(schema, &mut pending);
        if let Some(operations) = schema.get("x-stripeOperations").and_then(Value::as_array) {
            for operation in operations {
                let path = operation["path"].as_str().context("missing operation path")?;
                let method = operation["operation"].as_str().context("missing operation method")?;
                let source = all_paths.get(path).context("missing path")?;
                let operation = source.get(method).context("missing method")?.clone();
                references(&operation, &mut pending);
                paths
                    .entry(path.to_owned())
                    .or_insert_with(|| serde_json::json!({}))
                    .as_object_mut()
                    .unwrap()
                    .insert(method.to_owned(), operation);
            }
        }
    }
    for resource in &args.resources {
        ensure!(
            all_schemas[resource].get("x-stripeResource").is_some(),
            "not a resource: {resource}"
        );
    }
    for event in &args.events {
        ensure!(all_schemas[event]["x-stripeEvent"]["kind"] == "thin", "not a thin event: {event}");
    }
    document["paths"] = Value::Object(paths);
    document["components"]["schemas"] = Value::Object(
        all_schemas.into_iter().filter(|(name, _)| selected.contains(name)).collect(),
    );
    let spec = Spec::v2_experiment(serde_json::from_value(document)?)?;
    // The existing config loader is process-scoped. A separate development
    // entry point makes that constraint explicit without changing the normal CLI.
    std::env::set_current_dir(layout.parent().context("layout has no directory")?)?;
    ensure!(
        layout.file_name().is_some_and(|name| name == "gen_crates.toml"),
        "experimental layout must be named gen_crates.toml"
    );
    let codegen = CodeGen::new(spec, UrlFinder::default(), "1.0.0-rc.9".into())?;
    codegen.render_workspace()?.materialize(&out)?;
    codegen
        .analysis_bundle(stripe_openapi_codegen::analysis::GraphDetail::All)
        .write_to(&out.join("analysis"))?;
    println!(
        "Generated {} components and {} request-bearing components into {}",
        codegen.components.components.len(),
        codegen.request_component_count(),
        out.display()
    );
    Ok(())
}
