use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::{Args, Parser, Subcommand, ValueEnum};
use stripe_openapi_codegen::analysis::GraphDetail;
use stripe_openapi_codegen::codegen::{CodeGen, CodeGenOptions, EdgePolicy};
use stripe_openapi_codegen::crates::ALL_CRATES;
use stripe_openapi_codegen::output::{GeneratedWorkspace, install_to_repository};
use stripe_openapi_codegen::spec::Spec;
use stripe_openapi_codegen::spec_fetch;
use stripe_openapi_codegen::url_finder::{UrlFinder, update_api_doc_data};
use tracing::info;

#[derive(Debug, Parser)]
#[command(about = "Generate and analyze async-stripe's OpenAPI bindings")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Generate, format, and install bindings into the repository.
    Generate(GenerateArgs),
    /// Analyze the API model and write reports without installing generated files.
    Analyze(AnalyzeArgs),
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
    /// Write detailed analysis reports to this directory.
    #[arg(long)]
    report_dir: Option<PathBuf>,
}

#[derive(Debug, Copy, Clone, Default, ValueEnum)]
enum GraphSelection {
    None,
    #[default]
    Default,
    All,
}

#[derive(Debug, Args)]
struct AnalyzeArgs {
    #[command(flatten)]
    version: VersionArgs,
    /// Graph detail to include in the analysis bundle.
    #[arg(long, value_enum, default_value_t)]
    graphs: GraphSelection,
    /// Analysis-only counterfactual: do not use Expandable<T> references for placement.
    #[arg(long)]
    without_expandables: bool,
    /// Directory for reports.
    #[arg(long, default_value = "out/analysis")]
    report_dir: PathBuf,
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
        Command::Analyze(args) => run_analyze(args),
        Command::UpdateApiDocs(args) => run_update_api_docs(args),
    }
}

fn run_generate(args: GenerateArgs) -> Result<()> {
    let spec = Spec::new(spec_fetch::load_generation_spec(args.update_spec)?);
    let codegen = build_codegen(args.version.version, EdgePolicy::current_layout(), spec)?;
    let workspace = codegen.render_workspace()?;
    print_summary(&codegen);

    materialize_and_format(&workspace, &args.out)?;

    if let Some(report_dir) = args.report_dir {
        write_analysis(&codegen, &report_dir, GraphSelection::Default)?;
    }
    if !args.no_install {
        install_to_repository(&args.out, Path::new(".."))?;
        info!("Installed generated files into the repository");
    }
    Ok(())
}

fn run_analyze(args: AnalyzeArgs) -> Result<()> {
    let policy = if args.without_expandables {
        EdgePolicy::without_expandables()
    } else {
        EdgePolicy::current_layout()
    };
    let spec = Spec::new(spec_fetch::load_pinned_spec()?);
    let codegen = build_codegen(args.version.version, policy, spec)?;
    print_summary(&codegen);
    write_analysis(&codegen, &args.report_dir, args.graphs)?;
    println!("Analysis written to {}", args.report_dir.display());
    Ok(())
}

fn run_update_api_docs(args: UpdateApiDocsArgs) -> Result<()> {
    let spec = Spec::new(spec_fetch::load_pinned_spec()?);
    let codegen = build_codegen(None, EdgePolicy::current_layout(), spec)?;
    update_api_doc_data(&args.api_docs_url, &codegen.components)
}

fn build_codegen(version: Option<String>, edge_policy: EdgePolicy, spec: Spec) -> Result<CodeGen> {
    let url_finder = UrlFinder::new().context("could not initialize URL finder")?;
    let version = version.map_or_else(parse_workspace_version, Ok)?;
    CodeGen::new_with_options(spec, url_finder, version, CodeGenOptions { edge_policy })
}

fn materialize_and_format(workspace: &GeneratedWorkspace, out: &Path) -> Result<()> {
    info!(output = %out.display(), "Materializing generated workspace");
    workspace.materialize(out)?;
    run_rustfmt(out)
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

fn print_summary(codegen: &CodeGen) {
    println!(
        "Modeled {} components ({} with requests).",
        codegen.components.components.len(),
        codegen.request_component_count()
    );
    if !codegen.crate_inference_warnings.is_empty() {
        println!("Crate inference warnings: {}", codegen.crate_inference_warnings.len());
    }
}

fn write_analysis(codegen: &CodeGen, report_dir: &Path, graphs: GraphSelection) -> Result<()> {
    let detail = match graphs {
        GraphSelection::None => GraphDetail::None,
        GraphSelection::Default => GraphDetail::Default,
        GraphSelection::All => GraphDetail::All,
    };
    codegen.analysis_bundle(detail).write_to(report_dir)
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
