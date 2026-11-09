use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::{Args, Parser, Subcommand};
use stripe_openapi_codegen::codegen::{CodeGen, EdgePolicy};
use stripe_openapi_codegen::crate_inference::CrateInferenceWarning;
use stripe_openapi_codegen::crates::CrateConfig;
use stripe_openapi_codegen::output::{GeneratedWorkspace, install_to_repository};
use stripe_openapi_codegen::spec::Spec;
use stripe_openapi_codegen::spec_fetch;
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
    /// Generate, format, and install bindings into the repository.
    Generate(GenerateArgs),
    /// Write the original component and crate graphs to out/graphs.
    Graph(VersionArgs),
    /// Refresh the Stripe documentation URL mapping.
    UpdateApiDocs(UpdateApiDocsArgs),
}

#[derive(Debug, Args)]
struct VersionArgs {
    /// Version of the library to generate. Defaults to the root workspace version.
    #[arg(long)]
    version: Option<String>,
}

#[derive(Debug, Args)]
struct GenerateArgs {
    #[command(flatten)]
    version: VersionArgs,
    /// Staging directory for generated code.
    #[arg(long, default_value = "out")]
    out: PathBuf,
    /// Update the pinned Stripe OpenAPI spec to `latest` or a release such as `v171`.
    #[arg(long, value_parser = spec_fetch::parse_spec_update)]
    update_spec: Option<spec_fetch::SpecUpdate>,
    /// Generate and format staging output without installing it into the repository.
    #[arg(long, alias = "dry-run")]
    no_install: bool,
    /// Write crate inference warnings to <out>/crate_warnings.md.
    #[arg(long)]
    emit_warnings: bool,
    /// Retain expanded JSON and decode expandable resource models on demand.
    #[arg(long)]
    lazy_expandables: bool,
}

#[derive(Debug, Args)]
struct UpdateApiDocsArgs {
    /// Stripe API documentation URL.
    #[arg(long, default_value = "https://stripe.com/docs/api")]
    api_docs_url: String,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    match cli.command {
        Command::Generate(args) => run_generate(args),
        Command::Graph(args) => {
            let spec = Spec::new(spec_fetch::load_pinned_spec()?);
            let codegen = build_codegen(args.version, EdgePolicy::current_layout(), spec, false)?;
            codegen.legacy_graphs()?.materialize(Path::new("out"))
        }
        Command::UpdateApiDocs(args) => run_update_api_docs(args),
    }
}

fn run_generate(args: GenerateArgs) -> Result<()> {
    let spec = Spec::new(spec_fetch::load_generation_spec(args.update_spec)?);
    let codegen = build_codegen(
        args.version.version,
        EdgePolicy::current_layout(),
        spec,
        args.lazy_expandables,
    )?;
    let workspace = codegen.render_workspace()?;
    print_summary(&codegen);

    materialize_and_format(&workspace, &args.out, codegen.crate_config())?;

    if args.emit_warnings {
        write_crate_inference_warnings(codegen.crate_inference_warnings(), &args.out)?;
    }
    if !args.no_install {
        install_to_repository(&args.out, Path::new(".."), codegen.crate_config())?;
        info!("Installed generated files into the repository");
    }
    Ok(())
}

fn run_update_api_docs(args: UpdateApiDocsArgs) -> Result<()> {
    let spec = Spec::new(spec_fetch::load_pinned_spec()?);
    let codegen = build_codegen(None, EdgePolicy::current_layout(), spec, false)?;
    update_api_doc_data(&args.api_docs_url, codegen.components())
}

fn build_codegen(
    version: Option<String>,
    edge_policy: EdgePolicy,
    spec: Spec,
    lazy_expandables: bool,
) -> Result<CodeGen> {
    let url_finder = UrlFinder::new().context("could not initialize URL finder")?;
    let version = version.map_or_else(parse_workspace_version, Ok)?;
    let build =
        if lazy_expandables { CodeGen::with_lazy_expandable_config } else { CodeGen::with_config };
    build(spec, url_finder, version, edge_policy, CrateConfig::load(Path::new("gen_crates.toml"))?)
}

fn materialize_and_format(
    workspace: &GeneratedWorkspace,
    out: &Path,
    config: &CrateConfig,
) -> Result<()> {
    info!(output = %out.display(), "Materializing generated workspace");
    workspace.materialize(out)?;
    run_rustfmt(out, config)
}

fn run_rustfmt(out: &Path, config: &CrateConfig) -> Result<()> {
    let mut command = std::process::Command::new("rustfmt");
    let rustfmt_config = Path::new("../.rustfmt.toml").canonicalize()?;
    command.arg("+nightly").arg("--edition").arg("2024").arg("--config-path").arg(rustfmt_config);
    for krate in config.all() {
        command.arg(out.join(krate.generated_out_path(config)).join("src/mod.rs"));
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

fn print_summary(codegen: &CodeGen) {
    println!(
        "Modeled {} components ({} with requests).",
        codegen.components().components.len(),
        codegen.request_component_count()
    );
    if !codegen.crate_inference_warnings().is_empty() {
        println!("Crate inference warnings: {}", codegen.crate_inference_warnings().len());
    }
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

fn write_crate_inference_warnings(warnings: &[CrateInferenceWarning], output: &Path) -> Result<()> {
    if !warnings.is_empty() {
        let mut body = String::from(
            "## Crate Inference Warnings\n\n\
             The following components could not have their crate automatically inferred \
             and were auto-assigned. Consider adding explicit path entries in \
             `openapi/gen_crates.toml` to make these assignments permanent.\n\n\
             | Component | Assigned Crate | Depended On By | Candidate Crates |\n\
             |-----------|---------------|----------------|------------------|\n",
        );
        for w in warnings {
            let deps =
                w.depended_on_by.iter().map(|p| format!("`{p}`")).collect::<Vec<_>>().join(", ");
            let crates = w
                .candidate_crates
                .iter()
                .map(|c| format!("`{}`", c))
                .collect::<Vec<_>>()
                .join(", ");
            body.push_str(&format!(
                "| `{}` | `{}` | {} | {} |\n",
                w.component, w.chosen_crate, deps, crates,
            ));
        }
        body.push_str("\n### Suggested `gen_crates.toml` entries\n\n");
        body.push_str(
            "Add the following paths to the appropriate crate's `paths` array:\n\n```toml\n",
        );
        for w in warnings {
            body.push_str(&format!(
                "# currently auto-assigned to \"{}\"\n\"{}\",\n",
                w.chosen_crate, w.component,
            ));
        }
        body.push_str("```\n");
        fs::write(output.join("crate_warnings.md"), body)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::{Cli, Command};

    #[test]
    fn accepts_existing_graph_and_warning_commands() {
        assert!(matches!(
            Cli::try_parse_from(["codegen", "graph"]).unwrap().command,
            Command::Graph(_)
        ));
        let cli =
            Cli::try_parse_from(["codegen", "generate", "--emit-warnings", "--dry-run"]).unwrap();
        let Command::Generate(args) = cli.command else {
            panic!("expected generation");
        };
        assert!(args.emit_warnings);
        assert!(args.no_install);
    }
}
