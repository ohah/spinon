use super::author_stylesheets::collect_runtime_author_stylesheets;
use super::runtime_layout::{RuntimeLayoutContext, RuntimeLayoutFailure, compute_runtime_layout};
use super::{
    CascadeDiagnostic, RuntimeLayoutCompleted, RuntimeUaCascadeRoot, WorkRequest,
    elapsed_microseconds, key_for,
};
use spinon_core::{HostNodeHandle, HostNodeKind, HostParent, StyleRevision};
use spinon_style::{
    ComputedStyleProfile, CssCascadeError, RuntimeCascadeReuseStats, StyloDocumentView,
    compute_runtime_block_formatting_cascade_with_stylesheets,
    compute_runtime_block_paint_cascade_with_stylesheets,
    compute_runtime_block_positioning_cascade_with_stylesheets,
    compute_runtime_flex_custom_properties_cascade_with_stylesheets,
    compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets,
    compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets,
    compute_runtime_incremental_cascade_with_stylesheets,
};
use std::sync::Arc;
use std::time::Instant;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum RuntimeCalculationProfile {
    FlexLayout,
    FlexPaint,
    RegisteredPropertiesPaint,
    BlockPaint,
    BlockFormatting,
    BlockPositioning,
}

impl RuntimeCalculationProfile {
    const fn has_paint(self) -> bool {
        !matches!(self, Self::FlexLayout)
    }
}

#[derive(Clone, Debug)]
pub(super) struct RuntimeCalculation {
    pub(super) roots: Arc<[RuntimeUaCascadeRoot]>,
    pub(super) cascade_duration_us: u128,
    pub(super) cascade_recomputed_style_elements: u64,
    pub(super) cascade_reused_style_elements: u64,
    pub(super) cascade_context_style_elements: u64,
    pub(super) layout: Result<RuntimeLayoutCompleted, RuntimeLayoutFailure>,
    pub(super) layout_diagnostics: Vec<CascadeDiagnostic>,
}

impl RuntimeCalculation {
    pub(super) fn matches_origin(
        &self,
        key: super::RuntimeUaCascadeKey,
        viewport: spinon_style::CssViewport,
    ) -> bool {
        self.roots.iter().all(|root| {
            let styles = &root.styles;
            styles.generation.get() == key.generation
                && styles.document_revision.get() == key.document_revision
                && styles.render_tree_revision.get() == key.render_tree_revision
                && styles.style_revision.get() == key.style_revision
                && styles.viewport.width_css_px.to_bits() == viewport.width_css_px.to_bits()
                && styles.viewport.height_css_px.to_bits() == viewport.height_css_px.to_bits()
                && styles.viewport.device_scale_factor.to_bits()
                    == viewport.device_scale_factor.to_bits()
                && styles.viewport.environment_revision == viewport.environment_revision
                && styles.viewport.media_environment == viewport.media_environment
        }) && self.layout.as_ref().is_ok_and(|layout| {
            if layout.key != key {
                return false;
            }
            layout.render_snapshot.as_ref().is_none_or(|snapshot| {
                let render_key = snapshot.key();
                let scene_viewport = snapshot.viewport_css_px();
                render_key.generation().get() == key.generation
                    && render_key.document_revision().get() == key.document_revision
                    && render_key.render_tree_revision().get() == key.render_tree_revision
                    && render_key.style_revision().get() == key.style_revision
                    && render_key.environment_revision().get() == key.environment_revision
                    && scene_viewport.width().to_bits() == viewport.width_css_px.to_bits()
                    && scene_viewport.height().to_bits() == viewport.height_css_px.to_bits()
            })
        })
    }

    pub(super) fn rekey_for_request(&self, request: &WorkRequest) -> Option<Self> {
        let roots = self
            .roots
            .iter()
            .map(|root| {
                let mut styles = root.styles.clone();
                styles.generation = request.snapshot.generation();
                styles.document_revision = request.snapshot.document_revision();
                styles.render_tree_revision = request.snapshot.render_tree_revision();
                RuntimeUaCascadeRoot {
                    root_node_id: root.root_node_id,
                    styles,
                }
            })
            .collect::<Vec<_>>()
            .into();
        let previous_layout = self.layout.as_ref().ok()?;
        let render_snapshot = match previous_layout.render_snapshot.as_ref() {
            Some(snapshot) => Some(snapshot.with_document_snapshot_revision(&request.snapshot)?),
            None => None,
        };
        let layout = Ok(RuntimeLayoutCompleted {
            key: request.key,
            frames: Arc::clone(&previous_layout.frames),
            render_snapshot,
            projection_duration_us: 0,
            cache_hit: true,
        });
        Some(Self {
            roots,
            cascade_duration_us: 0,
            cascade_recomputed_style_elements: 0,
            cascade_reused_style_elements: self
                .roots
                .iter()
                .map(|root| root.styles.elements.len() as u64)
                .sum(),
            cascade_context_style_elements: 0,
            layout,
            layout_diagnostics: self.layout_diagnostics.clone(),
        })
    }
}

pub(super) fn compute_request(request: &WorkRequest) -> Result<RuntimeCalculation, String> {
    compute_request_with_profile(request, RuntimeCalculationProfile::FlexLayout)
}

pub(super) fn compute_request_for_runtime_gpu(
    request: &WorkRequest,
) -> Result<RuntimeCalculation, String> {
    compute_request_with_profile(request, RuntimeCalculationProfile::FlexPaint)
}

pub(super) fn compute_request_for_registered_properties_gpu(
    request: &WorkRequest,
) -> Result<RuntimeCalculation, String> {
    compute_request_with_profile(
        request,
        RuntimeCalculationProfile::RegisteredPropertiesPaint,
    )
}

pub(super) fn compute_request_for_block_paint(
    request: &WorkRequest,
) -> Result<RuntimeCalculation, String> {
    compute_request_with_profile(request, RuntimeCalculationProfile::BlockPaint)
}

pub(super) fn compute_request_for_block_formatting(
    request: &WorkRequest,
) -> Result<RuntimeCalculation, String> {
    compute_request_with_profile(request, RuntimeCalculationProfile::BlockFormatting)
}

pub(super) fn compute_request_for_block_positioning(
    request: &WorkRequest,
) -> Result<RuntimeCalculation, String> {
    compute_request_with_profile(request, RuntimeCalculationProfile::BlockPositioning)
}

fn compute_request_with_profile(
    request: &WorkRequest,
    profile: RuntimeCalculationProfile,
) -> Result<RuntimeCalculation, String> {
    let mut roots = Vec::new();
    let mut layout_context: Option<RuntimeLayoutContext> = None;
    let mut cascade_duration_us = 0_u128;
    let mut cascade_recomputed_style_elements = 0_u64;
    let mut cascade_reused_style_elements = 0_u64;
    let mut cascade_context_style_elements = 0_u64;
    let mut layout_diagnostics = Vec::new();
    let author_stylesheets =
        collect_runtime_author_stylesheets(&request.snapshot).map_err(|error| error.to_string())?;
    for root in request.snapshot.root_children() {
        let Some(node) = request.snapshot.node(root) else {
            return Err("HostRoot가 snapshot에 없는 노드를 가리킵니다".to_owned());
        };
        match node.kind() {
            HostNodeKind::Text(_) => {
                return Err(format!(
                    "HostRoot 직속 텍스트 노드 {}는 지원하지 않습니다",
                    root.id()
                ));
            }
            HostNodeKind::Element(_) => {
                if request.snapshot.parent(root) != Some(HostParent::Root) {
                    return Err(format!(
                        "cascade root {}의 부모가 HostRoot가 아닙니다",
                        root.id()
                    ));
                }
                let cascade_started = Instant::now();
                let (computed_root, view, reuse_stats) =
                    compute_root(request, root, profile, &author_stylesheets)?;
                cascade_duration_us =
                    cascade_duration_us.saturating_add(elapsed_microseconds(cascade_started));
                cascade_recomputed_style_elements = cascade_recomputed_style_elements
                    .saturating_add(reuse_stats.recomputed_style_elements);
                cascade_reused_style_elements =
                    cascade_reused_style_elements.saturating_add(reuse_stats.reused_style_elements);
                cascade_context_style_elements = cascade_context_style_elements
                    .saturating_add(reuse_stats.context_style_elements);
                layout_diagnostics.extend(computed_root.styles.diagnostics.iter().cloned());
                if roots.is_empty() {
                    layout_context = Some((root, view, computed_root.styles.clone()));
                } else {
                    layout_context = None;
                }
                roots.push(computed_root);
            }
        }
    }

    let layout = compute_runtime_layout(
        request,
        roots.len(),
        layout_context,
        &author_stylesheets,
        profile.has_paint(),
        profile,
    );

    Ok(RuntimeCalculation {
        roots: roots.into(),
        cascade_duration_us,
        cascade_recomputed_style_elements,
        cascade_reused_style_elements,
        cascade_context_style_elements,
        layout,
        layout_diagnostics,
    })
}

fn compute_root(
    request: &WorkRequest,
    root: HostNodeHandle,
    calculation_profile: RuntimeCalculationProfile,
    author_stylesheets: &[spinon_style::StylesheetSource],
) -> Result<
    (
        RuntimeUaCascadeRoot,
        StyloDocumentView,
        RuntimeCascadeReuseStats,
    ),
    String,
> {
    let view =
        StyloDocumentView::new_html_runtime_mount_shared(Arc::clone(&request.snapshot), root)
            .map_err(|error| error.to_string())?;
    let profile = match calculation_profile {
        RuntimeCalculationProfile::FlexLayout => {
            ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
        }
        RuntimeCalculationProfile::FlexPaint => {
            ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
        }
        RuntimeCalculationProfile::RegisteredPropertiesPaint => {
            ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
        }
        RuntimeCalculationProfile::BlockPaint => ComputedStyleProfile::RuntimeBlockPaintV1,
        RuntimeCalculationProfile::BlockFormatting => {
            ComputedStyleProfile::RuntimeBlockFormattingV1
        }
        RuntimeCalculationProfile::BlockPositioning => {
            ComputedStyleProfile::RuntimeBlockPositioningV1
        }
    };
    let previous_style_root = request
        .previous_styles
        .as_deref()
        .and_then(|roots| (roots.len() == 1).then_some(&roots[0]))
        .filter(|cached| cached.root_node_id == root.id().get());
    let dirty_root_ids = request
        .invalidation
        .as_ref()
        .map(super::invalidation::RuntimeStyleInvalidation::dirty_roots)
        .unwrap_or_default()
        .iter()
        .map(|handle| handle.id())
        .collect::<Vec<_>>();
    let incremental = if let Some(previous) = previous_style_root {
        if matches!(
            request.invalidation.as_ref(),
            Some(super::invalidation::RuntimeStyleInvalidation::Unchanged { .. })
                | Some(super::invalidation::RuntimeStyleInvalidation::Subtrees { .. })
        ) {
            compute_runtime_incremental_cascade_with_stylesheets(
                &view,
                author_stylesheets,
                request.viewport,
                StyleRevision::INITIAL,
                profile,
                &previous.styles,
                &dirty_root_ids,
            )
            .map_err(|error| error.to_string())?
        } else {
            None
        }
    } else {
        None
    };
    let (styles, reuse_stats) = if let Some((styles, stats)) = incremental {
        (styles, stats)
    } else {
        let styles = match calculation_profile {
            RuntimeCalculationProfile::FlexLayout => {
                compute_runtime_flex_custom_properties_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    StyleRevision::INITIAL,
                )
            }
            RuntimeCalculationProfile::FlexPaint => {
                compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    StyleRevision::INITIAL,
                )
            }
            RuntimeCalculationProfile::RegisteredPropertiesPaint => {
                compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    StyleRevision::INITIAL,
                )
            }
            RuntimeCalculationProfile::BlockPaint => {
                compute_runtime_block_paint_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    StyleRevision::INITIAL,
                )
            }
            RuntimeCalculationProfile::BlockFormatting => {
                compute_runtime_block_formatting_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    StyleRevision::INITIAL,
                )
            }
            RuntimeCalculationProfile::BlockPositioning => {
                compute_runtime_block_positioning_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    StyleRevision::INITIAL,
                )
            }
        }
        .map_err(|error: CssCascadeError| error.to_string())?;
        let full_count = styles.elements.len() as u64;
        (
            styles,
            RuntimeCascadeReuseStats {
                recomputed_style_elements: full_count,
                reused_style_elements: 0,
                context_style_elements: 0,
            },
        )
    };
    if key_for(&request.snapshot, request.viewport.environment_revision) != request.key
        || styles.generation.get() != request.key.generation
        || styles.document_revision.get() != request.key.document_revision
        || styles.render_tree_revision.get() != request.key.render_tree_revision
        || styles.style_revision.get() != request.key.style_revision
    {
        return Err("Stylo 결과 revision이 요청 key와 다릅니다".to_owned());
    }
    Ok((
        RuntimeUaCascadeRoot {
            root_node_id: root.id().get(),
            styles,
        },
        view,
        reuse_stats,
    ))
}
