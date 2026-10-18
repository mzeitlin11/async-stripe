use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::{Args, Parser, Subcommand};
use petgraph::dot::{Config, Dot};
use stripe_openapi_codegen::codegen::CodeGen;
use stripe_openapi_codegen::crates::ALL_CRATES;
use stripe_openapi_codegen::output::{OutputDirectory, install_to_repository};
use stripe_openapi_codegen::spec::Spec;
use stripe_openapi_codegen::spec_fetch::{self, SpecUpdate};
use stripe_openapi_codegen::url_finder::{UrlFinder, update_api_doc_data};
use tracing::info;

#[derive(Debug, Parser)]
#[command(about = "Generate async-stripe's OpenAPI bindings and dependency graphs")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Generate bindings and install them into the repository.
    Generate(GenerateArgs),
    /// Write component and crate dependency graphs to out/graphs.
    Graph(VersionArgs),
    /// Refresh the Stripe documentation URL mapping.
    UpdateApiDocs(UpdateApiDocsArgs),
}

#[derive(Debug, Args)]
struct VersionArgs {
    /// Library version to generate. Defaults to the root workspace version.
    #[arg(long)]
    version: Option<String>,
}

#[derive(Debug, Args)]
struct GenerateArgs {
    #[command(flatten)]
    version: VersionArgs,
    /// Output directory for generated code.
    #[arg(long, default_value = "out")]
    out: PathBuf,
    /// Update the pinned spec to `latest` or a release such as `v171`.
    #[arg(long, value_parser = spec_fetch::parse_spec_update)]
    update_spec: Option<SpecUpdate>,
    /// Generate and format staging output without installing it into the repository.
    #[arg(long, alias = "dry-run")]
    no_install: bool,
    /// Write crate inference warnings to <out>/crate_warnings.md.
    #[arg(long)]
    emit_warnings: bool,
}

#[derive(Debug, Args)]
struct UpdateApiDocsArgs {
    /// Stripe API documentation URL.
    #[arg(long, default_value = "https://stripe.com/docs/api")]
    api_docs_url: String,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    match Cli::parse().command {
        Command::Generate(args) => run_generate(args),
        Command::Graph(args) => {
            let spec = Spec::new(spec_fetch::load_pinned_spec()?);
            let codegen = build_codegen(args.version, spec)?;
            let output = OutputDirectory::prepare(Path::new("out"))?;
            let components = codegen.components.gen_component_dep_graph();
            output.write(
                format!("{:?}", Dot::with_config(&components, &[Config::EdgeNoLabel])),
                "graphs/components_graph.txt",
            )?;
            let crates = codegen.components.gen_crate_dep_graph();
            output.write(
                format!("{:?}", Dot::with_config(&crates, &[Config::EdgeNoLabel])),
                "graphs/crate_graph.txt",
            )
        }
        Command::UpdateApiDocs(args) => {
            let spec = Spec::new(spec_fetch::load_pinned_spec()?);
            let codegen = build_codegen(None, spec)?;
            update_api_doc_data(&args.api_docs_url, &codegen.components)
        }
    }
}

fn build_codegen(version: Option<String>, spec: Spec) -> Result<CodeGen> {
    let url_finder = UrlFinder::new().context("Could not initialize URL finder")?;
    let version = version.map_or_else(parse_workspace_version, Ok)?;
    CodeGen::new(spec, url_finder, version)
}

fn write_crate_inference_warnings(codegen: &CodeGen, output: &OutputDirectory) -> Result<()> {
    if !codegen.crate_inference_warnings.is_empty() {
        let mut body = String::from(
            "## Crate Inference Warnings\n\n\
             The following components could not have their crate automatically inferred \
             and were auto-assigned. Consider adding explicit path entries in \
             `openapi/gen_crates.toml` to make these assignments permanent.\n\n\
             | Component | Assigned Crate | Depended On By | Candidate Crates |\n\
             |-----------|---------------|----------------|------------------|\n",
        );
        for w in &codegen.crate_inference_warnings {
            let deps =
                w.depended_on_by.iter().map(|p| format!("`{p}`")).collect::<Vec<_>>().join(", ");
            let crates = w
                .candidate_crates
                .iter()
                .map(|c| format!("`{}`", c.base_name()))
                .collect::<Vec<_>>()
                .join(", ");
            body.push_str(&format!(
                "| `{}` | `{}` | {} | {} |\n",
                w.component,
                w.chosen_crate.base_name(),
                deps,
                crates,
            ));
        }
        body.push_str("\n### Suggested `gen_crates.toml` entries\n\n");
        body.push_str(
            "Add the following paths to the appropriate crate's `paths` array:\n\n```toml\n",
        );
        for w in &codegen.crate_inference_warnings {
            body.push_str(&format!(
                "# currently auto-assigned to \"{}\"\n\"{}\",\n",
                w.chosen_crate.base_name(),
                w.component,
            ));
        }
        body.push_str("```\n");
        output.write(body, "crate_warnings.md")?;
    }

    Ok(())
}

fn run_generate(args: GenerateArgs) -> Result<()> {
    let spec = Spec::new(spec_fetch::load_generation_spec(args.update_spec)?);
    let codegen = build_codegen(args.version.version, spec)?;
    let output = OutputDirectory::prepare(&args.out)?;
    if args.emit_warnings {
        write_crate_inference_warnings(&codegen, &output)?;
    }
    codegen.write_files(&output)?;
    run_rustfmt(output.root())?;
    if !args.no_install {
        install_to_repository(output.root(), Path::new(".."))?;
        info!("Installed generated files into the repository");
    }
    Ok(())
}

fn run_rustfmt(out: &Path) -> Result<()> {
    let mut command = std::process::Command::new("rustfmt");
    let config = Path::new("../.rustfmt.toml").canonicalize()?;
    command.arg("+nightly").arg("--edition").arg("2024").arg("--config-path").arg(config);
    for krate in &*ALL_CRATES {
        command.arg(out.join(krate.generated_out_path()).join("src/mod.rs"));
    }
    command.arg(out.join("async-stripe-webhook/mod.rs"));
    command.arg(out.join("tests/mod.rs"));

    let formatted = command.output().context("failed to invoke rustfmt")?;
    if !formatted.status.success() {
        bail!(
            "rustfmt failed:\n{}{}",
            String::from_utf8_lossy(&formatted.stdout),
            String::from_utf8_lossy(&formatted.stderr)
        );
    }
    let checked = command.arg("--check").output().context("failed to verify rustfmt output")?;
    if !checked.status.success() {
        bail!(
            "rustfmt did not reach a fixed point after one pass:\n{}{}",
            String::from_utf8_lossy(&checked.stdout),
            String::from_utf8_lossy(&checked.stderr)
        );
    }
    Ok(())
}

fn parse_workspace_version() -> Result<String> {
    #[derive(serde::Deserialize)]
    struct Manifest {
        workspace: Workspace,
    }
    #[derive(serde::Deserialize)]
    struct Workspace {
        package: WorkspacePackage,
    }
    #[derive(serde::Deserialize)]
    struct WorkspacePackage {
        version: String,
    }
    let manifest = fs::read_to_string("../Cargo.toml")?;
    Ok(toml::from_str::<Manifest>(&manifest)?.workspace.package.version)
}
