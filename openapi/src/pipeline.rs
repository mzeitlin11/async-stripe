use tracing::{debug, info};

use crate::components::{Components, infer_components};
use crate::crate_inference::CrateInferenceWarning;
use crate::dependencies::EdgePolicy;
use crate::layout::{ComponentLayout, ComponentLayoutDraft};
use crate::spec::Spec;

pub struct PipelineOutput {
    pub components: Components,
    pub component_layout: ComponentLayout,
    pub crate_inference_warnings: Vec<CrateInferenceWarning>,
}

struct InferredApi {
    components: Components,
    component_layout: ComponentLayoutDraft,
}

struct RetainedApi {
    components: Components,
    component_layout: ComponentLayoutDraft,
}

struct LaidOutApi {
    components: Components,
    component_layout: ComponentLayout,
    crate_inference_warnings: Vec<CrateInferenceWarning>,
}

impl InferredApi {
    fn from_spec(spec: &Spec) -> anyhow::Result<Self> {
        let (components, component_layout) = infer_components(spec)?;
        Ok(Self { components, component_layout })
    }

    fn retain_reachable(mut self) -> anyhow::Result<RetainedApi> {
        self.components.filter_unused_components();
        self.components.validate_catalog()?;
        Ok(RetainedApi { components: self.components, component_layout: self.component_layout })
    }
}

impl RetainedApi {
    fn assign_component_layout(mut self, policy: EdgePolicy) -> anyhow::Result<LaidOutApi> {
        let crate_inference_warnings =
            self.components.infer_all_crate_assignments(&mut self.component_layout, policy)?;
        let component_layout =
            ComponentLayout::from_draft(&self.components, &self.component_layout)?;
        info!("Finished inferring component crates");
        Ok(LaidOutApi { components: self.components, component_layout, crate_inference_warnings })
    }
}

impl LaidOutApi {
    fn apply_model_rewrites(mut self) -> anyhow::Result<PipelineOutput> {
        self.components.apply_overrides()?;
        debug!("Finished applying overrides");

        self.components.run_deduplication_pass();
        info!("Finished deduplication pass");

        Ok(PipelineOutput {
            components: self.components,
            component_layout: self.component_layout,
            crate_inference_warnings: self.crate_inference_warnings,
        })
    }
}

pub fn run_pipeline(spec: &Spec, policy: EdgePolicy) -> anyhow::Result<PipelineOutput> {
    InferredApi::from_spec(spec)?
        .retain_reachable()?
        .assign_component_layout(policy)?
        .apply_model_rewrites()
}
