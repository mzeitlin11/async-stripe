use tracing::{debug, info};

use crate::components::{Components, infer_components};
use crate::crate_inference::CrateInferenceWarning;
use crate::crates::CrateConfig;
use crate::dependencies::EdgePolicy;
use crate::layout::ComponentLayoutDraft;
use crate::spec::Spec;

pub(crate) struct PipelineOutput {
    pub components: Components,
    pub request_owners: ComponentLayoutDraft,
    pub crate_inference_warnings: Vec<CrateInferenceWarning>,
}

pub(crate) fn run_pipeline(
    spec: &Spec,
    policy: EdgePolicy,
    config: &CrateConfig,
) -> anyhow::Result<PipelineOutput> {
    let (mut components, mut request_owners) = infer_components(spec, config)?;
    components.filter_unused_components();
    components.validate_catalog()?;

    // Service ownership is inferred before rewrites for compatibility. Final
    // model placement is selected separately, after Rust-model construction.
    let crate_inference_warnings =
        components.infer_request_owners(&mut request_owners, policy, config)?;
    info!("Finished inferring request owners");

    components.apply_overrides()?;
    debug!("Finished applying overrides");
    components.run_deduplication_pass();
    info!("Finished deduplication pass");

    Ok(PipelineOutput { components, request_owners, crate_inference_warnings })
}
