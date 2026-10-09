use super::{RuntimeCalculation, RuntimeUaCascadeKey, WorkRequest};
use spinon_style::{CssMediaEnvironment, CssViewport};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct RuntimeCalculationCacheKey {
    generation: u64,
    render_tree_revision: u64,
    style_revision: u64,
    environment_revision: u64,
    viewport_width_bits: u32,
    viewport_height_bits: u32,
    device_scale_factor_bits: u32,
    media_environment: CssMediaEnvironment,
}

impl RuntimeCalculationCacheKey {
    pub(super) fn for_request(request: &WorkRequest) -> Self {
        Self {
            generation: request.key.generation,
            render_tree_revision: request.key.render_tree_revision,
            style_revision: request.key.style_revision,
            environment_revision: request.key.environment_revision,
            viewport_width_bits: request.viewport.width_css_px.to_bits(),
            viewport_height_bits: request.viewport.height_css_px.to_bits(),
            device_scale_factor_bits: request.viewport.device_scale_factor.to_bits(),
            media_environment: request.viewport.media_environment,
        }
    }
}

pub(super) struct RuntimeCalculationCacheEntry {
    key: RuntimeCalculationCacheKey,
    origin_key: RuntimeUaCascadeKey,
    origin_viewport: CssViewport,
    calculation: RuntimeCalculation,
}

impl RuntimeCalculationCacheEntry {
    pub(super) fn new(request: &WorkRequest, calculation: RuntimeCalculation) -> Self {
        Self {
            key: RuntimeCalculationCacheKey::for_request(request),
            origin_key: request.key,
            origin_viewport: request.viewport,
            calculation,
        }
    }

    pub(super) fn rekey_for(&self, request: &WorkRequest) -> Option<RuntimeCalculation> {
        if self.key != RuntimeCalculationCacheKey::for_request(request)
            || !self
                .calculation
                .matches_origin(self.origin_key, self.origin_viewport)
        {
            return None;
        }
        self.calculation.rekey_for_request(request)
    }
}
