use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail, ensure};
use clap::{Args, Parser, Subcommand, ValueEnum};
use stripe_openapi_codegen::analysis::GraphDetail;
use stripe_openapi_codegen::codegen::{CodeGen, CodeGenOptions, EdgePolicy};
use stripe_openapi_codegen::crates::ALL_CRATES;
use stripe_openapi_codegen::output::{
    OutputInventory, compare_with_repository, install_to_repository, snapshot_files,
};
use stripe_openapi_codegen::spec::Spec;
use stripe_openapi_codegen::spec_fetch;
use stripe_openapi_codegen::spec_fetch::fetch_spec;
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
    /// Regenerate bindings and compare every generated artifact with the repository.
    Check(CheckArgs),
    /// Analyze the API model and write reports without installing generated files.
    Analyze(AnalyzeArgs),
    /// Refresh the Stripe documentation URL mapping.
    UpdateApiDocs(UpdateApiDocsArgs),
}

#[derive(Debug, Clone, Args)]
struct CommonArgs {
    /// Version of the library to generate. Defaults to the root workspace version.
    #[arg(long)]
    version: Option<String>,
    /// Input path for the OpenAPI spec.
    #[arg(default_value = "spec3.sdk.json")]
    spec_path: String,
    /// Staging directory for generated code.
    #[arg(long, default_value = "out")]
    out: PathBuf,
    /// Fetch `latest`, `current`, or a specific Stripe OpenAPI release such as `v171`.
    #[arg(long, value_parser = spec_fetch::parse_spec_version)]
    fetch: Option<spec_fetch::SpecVersion>,
}

#[derive(Debug, Args)]
struct GenerateArgs {
    #[command(flatten)]
    common: CommonArgs,
    /// Generate and format staging output without installing it into the repository.
    #[arg(long, alias = "dry-run")]
    no_install: bool,
    /// Write detailed analysis reports to this directory.
    #[arg(long)]
    report_dir: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct CheckArgs {
    #[command(flatten)]
    common: CommonArgs,
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
    common: CommonArgs,
    /// Graph detail to include in the analysis bundle.
    #[arg(long, value_enum, default_value_t)]
    graphs: GraphSelection,
    /// Analysis-only counterfactual: do not use Expandable<T> references for placement.
    #[arg(long)]
    without_expandables: bool,
    /// Directory for reports. Defaults to `<out>/analysis`.
    #[arg(long)]
    report_dir: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct UpdateApiDocsArgs {
    #[command(flatten)]
    common: CommonArgs,
    /// Stripe API documentation URL.
    #[arg(long, default_value = "https://stripe.com/docs/api")]
    api_docs_url: String,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    match cli.command {
        Command::Generate(args) => run_generate(args),
        Command::Check(args) => run_check(args),
        Command::Analyze(args) => run_analyze(args),
        Command::UpdateApiDocs(args) => run_update_api_docs(args),
    }
}

fn run_generate(args: GenerateArgs) -> Result<()> {
    let codegen = build_codegen(&args.common, EdgePolicy::current_layout())?;
    let inventory = render_and_format(&codegen, &args.common.out)?;
    print_summary(&codegen, &inventory);

    if let Some(report_dir) = args.report_dir {
        write_analysis(&codegen, &inventory, &report_dir, GraphSelection::Default)?;
    }
    if !args.no_install {
        install_to_repository(&args.common.out, Path::new(".."))?;
        info!("Installed generated files into the repository");
    }
    Ok(())
}

fn run_check(args: CheckArgs) -> Result<()> {
    let codegen = build_codegen(&args.common, EdgePolicy::current_layout())?;
    let inventory = render_and_format(&codegen, &args.common.out)?;
    print_summary(&codegen, &inventory);
    compare_with_repository(&args.common.out, Path::new(".."))?;
    println!("Generated output matches all checked-in artifacts.");
    Ok(())
}

fn run_analyze(args: AnalyzeArgs) -> Result<()> {
    let policy = if args.without_expandables {
        EdgePolicy::without_expandables()
    } else {
        EdgePolicy::current_layout()
    };
    let codegen = build_codegen(&args.common, policy)?;
    let inventory = render_and_format(&codegen, &args.common.out)?;
    print_summary(&codegen, &inventory);
    let report_dir = args.report_dir.unwrap_or_else(|| args.common.out.join("analysis"));
    write_analysis(&codegen, &inventory, &report_dir, args.graphs)?;
    println!("Analysis written to {}", report_dir.display());
    Ok(())
}

fn run_update_api_docs(args: UpdateApiDocsArgs) -> Result<()> {
    let codegen = build_codegen(&args.common, EdgePolicy::current_layout())?;
    update_api_doc_data(&args.api_docs_url, &codegen.components)
}

fn build_codegen(args: &CommonArgs, edge_policy: EdgePolicy) -> Result<CodeGen> {
    info!("Loading {}", args.spec_path);
    let spec = if let Some(version) = args.fetch.clone() {
        let raw = fetch_spec(version, &args.spec_path)?;
        Spec::new(serde_json::from_value(raw)?)
    } else {
        let raw = File::open(&args.spec_path).context("failed to load the OpenAPI spec")?;
        Spec::new(serde_json::from_reader(&raw).context("failed to parse the OpenAPI spec")?)
    };
    let url_finder = UrlFinder::new().context("could not initialize URL finder")?;
    let version = args.version.clone().map_or_else(parse_workspace_version, Ok)?;
    CodeGen::new_with_options(spec, url_finder, version, CodeGenOptions { edge_policy })
}

fn render_and_format(codegen: &CodeGen, out: &Path) -> Result<OutputInventory> {
    let workspace = codegen.render_workspace()?;
    info!(files = workspace.file_count(), output = %out.display(), "Materializing generated workspace");
    workspace.materialize(out)?;
    run_rustfmt(out)?;
    OutputInventory::collect(out)
}

fn run_rustfmt(out: &Path) -> Result<()> {
    let mut command = std::process::Command::new("cargo");
    let rustfmt_config = Path::new("../.rustfmt.toml").canonicalize()?;
    command.arg("+nightly").arg("fmt").arg("--").arg("--config-path").arg(rustfmt_config);
    for krate in &*ALL_CRATES {
        command.arg(out.join(krate.generated_out_path()).join("src/mod.rs"));
    }
    command.arg(out.join("async-stripe-webhook/mod.rs"));
    command.arg(out.join("tests/mod.rs"));

    let first = command.output().context("failed to invoke rustfmt")?;
    if !first.status.success() {
        bail!("rustfmt failed: {}", String::from_utf8_lossy(&first.stderr));
    }
    let once = snapshot_files(out)?;
    let second = command.output().context("failed to invoke rustfmt a second time")?;
    if !second.status.success() {
        bail!("rustfmt failed: {}", String::from_utf8_lossy(&second.stderr));
    }
    ensure!(once == snapshot_files(out)?, "rustfmt did not reach a fixed point after one pass");
    Ok(())
}

fn print_summary(codegen: &CodeGen, inventory: &OutputInventory) {
    println!(
        "Generated {} components ({} with requests) into {} files, {} lines, {} bytes.",
        codegen.components.components.len(),
        codegen.request_component_count(),
        inventory.total.files,
        inventory.total.lines,
        inventory.total.bytes
    );
    if !codegen.crate_inference_warnings.is_empty() {
        println!("Crate inference warnings: {}", codegen.crate_inference_warnings.len());
    }
}

fn write_analysis(
    codegen: &CodeGen,
    inventory: &OutputInventory,
    report_dir: &Path,
    graphs: GraphSelection,
) -> Result<()> {
    let detail = match graphs {
        GraphSelection::None => GraphDetail::None,
        GraphSelection::Default => GraphDetail::Default,
        GraphSelection::All => GraphDetail::All,
    };
    codegen.analysis_bundle(inventory, detail).write_to(report_dir)
}

fn parse_workspace_version() -> Result<String> {
    let reader = BufReader::new(File::open("../Cargo.toml")?);
    for line in reader.lines() {
        let line = line?;
        let line = line.trim_ascii();
        if line.starts_with("version") {
            let (_, version) = line.split_once('=').context("invalid workspace version line")?;
            return Ok(version.trim_ascii().replace('"', ""));
        }
    }
    bail!("No workspace version key found")
}
