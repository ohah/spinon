use super::invalidation::RuntimeStyleInvalidation;
use super::{RuntimeCalculation, RuntimeUaCascadeKey, RuntimeUaCascadeRoot, WorkRequest};
use spinon_style::{CssMediaEnvironment, CssViewport};
use std::sync::Arc;

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

/// 직전 성공 cascade의 직렬화 결과만 보관하는 제한된 cache입니다.
/// HostDocumentSnapshot, DOM handle, Stylo 내부 ComputedValues는 보관하지 않습니다.
pub(super) struct RuntimeStyleCacheEntry {
    generation: u64,
    document_revision: u64,
    style_revision: u64,
    environment_revision: u64,
    viewport: CssViewport,
    roots: Arc<[RuntimeUaCascadeRoot]>,
}

impl RuntimeStyleCacheEntry {
    pub(super) fn new(request: &WorkRequest, calculation: &RuntimeCalculation) -> Option<Self> {
        if calculation.roots.len() != 1
            || calculation.roots[0].styles.generation.get() != request.key.generation
            || calculation.roots[0].styles.document_revision.get() != request.key.document_revision
            || calculation.roots[0].styles.style_revision.get() != request.key.style_revision
        {
            return None;
        }
        Some(Self {
            generation: request.key.generation,
            document_revision: request.key.document_revision,
            style_revision: request.key.style_revision,
            environment_revision: request.key.environment_revision,
            viewport: request.viewport,
            roots: Arc::clone(&calculation.roots),
        })
    }

    pub(super) fn styles_for_request(
        &self,
        request: &WorkRequest,
    ) -> Option<Arc<[RuntimeUaCascadeRoot]>> {
        let (source_generation, source_revision) =
            request.invalidation.as_ref()?.base_revision()?;
        if source_generation != self.generation
            || source_revision.get() != self.document_revision
            || request.key.generation != self.generation
            || request.key.style_revision != self.style_revision
            || request.key.environment_revision != self.environment_revision
            || !same_viewport(self.viewport, request.viewport)
            || !matches!(
                request.invalidation.as_ref(),
                Some(RuntimeStyleInvalidation::Unchanged { .. })
                    | Some(RuntimeStyleInvalidation::Subtrees { .. })
            )
        {
            return None;
        }
        Some(Arc::clone(&self.roots))
    }
}

fn same_viewport(left: CssViewport, right: CssViewport) -> bool {
    left.width_css_px.to_bits() == right.width_css_px.to_bits()
        && left.height_css_px.to_bits() == right.height_css_px.to_bits()
        && left.device_scale_factor.to_bits() == right.device_scale_factor.to_bits()
        && left.environment_revision == right.environment_revision
        && left.media_environment == right.media_environment
}
