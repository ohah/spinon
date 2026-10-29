use super::author_stylesheets::collect_runtime_author_stylesheets;
use super::runtime_layout::{
    RuntimeLayoutContext, RuntimeLayoutFailure, compute_runtime_layout, layout_failure,
};
use super::{
    CascadeDiagnostic, RuntimeLayoutCompleted, RuntimeUaCascadeRoot, WorkRequest,
    elapsed_microseconds, key_for_with_style_revision,
};
use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, NodeId};
use spinon_layout::MAX_LAYOUT_TREE_DEPTH;
#[cfg(feature = "c19-2-box-paint-opacity-fixture-test")]
use spinon_style::compute_c19_box_paint_opacity_cascade_with_stylesheets;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
use spinon_style::compute_runtime_block_flex_grid_track_sizing_visibility_collapse_fixture_test_cascade_with_stylesheets;
#[cfg(feature = "c13-3-gesture-fixture-test")]
use spinon_style::compute_runtime_block_flex_scroll_gesture_cascade_with_stylesheets;
use spinon_style::{
    ComputedStyleProfile, CssCascadeError, RuntimeCascadeReuseStats, StyloDocumentView,
    compute_runtime_block_flex_grid_auto_placement_cascade_with_stylesheets,
    compute_runtime_block_flex_grid_cascade_with_stylesheets,
    compute_runtime_block_flex_grid_dense_auto_placement_cascade_with_stylesheets,
    compute_runtime_block_flex_grid_placement_cascade_with_stylesheets,
    compute_runtime_block_flex_grid_template_cascade_with_stylesheets,
    compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets,
    compute_runtime_block_flex_overflow_clip_cascade_with_stylesheets,
    compute_runtime_block_flex_stacking_cascade_with_stylesheets,
    compute_runtime_block_formatting_cascade_with_stylesheets,
    compute_runtime_block_paint_cascade_with_stylesheets,
    compute_runtime_block_positioning_cascade_with_stylesheets,
    compute_runtime_flex_custom_properties_cascade_with_stylesheets,
    compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets,
    compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets,
    compute_runtime_incremental_cascade_with_stylesheets,
};
use spinon_style_to_render::build_runtime_render_snapshot;
use std::sync::Arc;
use std::time::Instant;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum RuntimeCalculationProfile {
    FlexLayout,
    RootHtmlDocument,
    RootHtmlDefaultRender,
    RootHtmlBlockPaint,
    #[cfg(feature = "c04-12-root-html-text-paint-fixture-test")]
    RootHtmlTextPaint,
    FlexPaint,
    RegisteredPropertiesPaint,
    BlockPaint,
    BlockImage,
    BlockFormatting,
    BlockPositioning,
    BlockFlexStacking,
    BlockFlexOverflowClip,
    #[cfg(feature = "c19-2-box-paint-opacity-fixture-test")]
    C19BoxPaintOpacity,
    #[cfg(feature = "c13-3-gesture-fixture-test")]
    BlockFlexScrollGesture,
    BlockFlexGrid,
    BlockFlexGridPlacement,
    BlockFlexGridAutoPlacement,
    BlockFlexGridDenseAutoPlacement,
    BlockFlexGridTemplate,
    #[cfg(feature = "c11-3-3-grid-implicit-named-lines-fixture-test")]
    BlockFlexGridTemplateImplicitNamedLines,
    BlockFlexGridTrackSizing,
}

impl RuntimeCalculationProfile {
    const fn has_paint(self) -> bool {
        !matches!(self, Self::FlexLayout | Self::RootHtmlDocument)
    }
}

fn is_root_html_paint_profile(profile: RuntimeCalculationProfile) -> bool {
    match profile {
        RuntimeCalculationProfile::RootHtmlBlockPaint => true,
        RuntimeCalculationProfile::RootHtmlDefaultRender => true,
        #[cfg(feature = "c04-12-root-html-text-paint-fixture-test")]
        RuntimeCalculationProfile::RootHtmlTextPaint => true,
        _ => false,
    }
}

#[derive(Clone, Debug)]
pub(super) struct RuntimeCalculation {
    pub(super) roots: Arc<[RuntimeUaCascadeRoot]>,
    pub(super) cascade_error: Option<String>,
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
                && styles.style_revision == key.style_revision
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
                    && render_key.style_revision() == key.style_revision
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
                    body_node_id: root.body_node_id,
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
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        let grid_track_sizing_diagnostics = previous_layout
            .grid_track_sizing_diagnostics
            .as_ref()
            .map(|diagnostics| {
                let revision = spinon_layout::LayoutInputRevision::new(
                    spinon_layout::LayoutSourceRevision::HostDocument {
                        generation: request.snapshot.generation(),
                        document: request.snapshot.document_revision(),
                        render_tree: request.snapshot.render_tree_revision(),
                    },
                    request.key.style_revision,
                    request.viewport.environment_revision,
                )
                .with_intrinsic_size_revision(diagnostics.revision.intrinsic_size());
                Arc::new(diagnostics.with_revision(revision))
            });
        let layout = Ok(RuntimeLayoutCompleted {
            key: request.key,
            frames: Arc::clone(&previous_layout.frames),
            render_snapshot,
            projection_duration_us: 0,
            cache_hit: true,
            #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
            grid_track_sizing_diagnostics,
        });
        Some(Self {
            roots,
            cascade_error: self.cascade_error.clone(),
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

pub(super) fn compute_request_for_root_html(
    request: &WorkRequest,
) -> Result<RuntimeCalculation, String> {
    compute_request_with_profile(request, RuntimeCalculationProfile::RootHtmlDocument)
}

pub(super) fn compute_request_for_root_html_default_render(
    request: &WorkRequest,
) -> Result<RuntimeCalculation, String> {
    compute_request_with_profile(request, RuntimeCalculationProfile::RootHtmlDefaultRender)
}

#[cfg(feature = "c04-12-root-html-block-paint-fixture-test")]
pub(super) fn compute_request_for_root_html_block_paint(
    request: &WorkRequest,
) -> Result<RuntimeCalculation, String> {
    compute_request_with_profile(request, RuntimeCalculationProfile::RootHtmlBlockPaint)
}

#[cfg(feature = "c04-12-root-html-text-paint-fixture-test")]
pub(super) fn compute_request_for_root_html_text_paint(
    request: &WorkRequest,
) -> Result<RuntimeCalculation, String> {
    compute_request_with_profile(request, RuntimeCalculationProfile::RootHtmlTextPaint)
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

pub(super) fn compute_request_for_block_image(
    request: &WorkRequest,
) -> Result<RuntimeCalculation, String> {
    compute_request_with_profile(request, RuntimeCalculationProfile::BlockImage)
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

pub(super) fn compute_request_for_block_flex_stacking(
    request: &WorkRequest,
) -> Result<RuntimeCalculation, String> {
    compute_request_with_profile(request, RuntimeCalculationProfile::BlockFlexStacking)
}

pub(super) fn compute_request_for_block_flex_overflow_clip(
    request: &WorkRequest,
) -> Result<RuntimeCalculation, String> {
    compute_request_with_profile(request, RuntimeCalculationProfile::BlockFlexOverflowClip)
}

#[cfg(feature = "c19-2-box-paint-opacity-fixture-test")]
pub(super) fn compute_request_for_c19_box_paint_opacity(
    request: &WorkRequest,
) -> Result<RuntimeCalculation, String> {
    compute_request_with_profile(request, RuntimeCalculationProfile::C19BoxPaintOpacity)
}

#[cfg(feature = "c13-3-gesture-fixture-test")]
pub(super) fn compute_request_for_block_flex_scroll_gesture(
    request: &WorkRequest,
) -> Result<RuntimeCalculation, String> {
    compute_request_with_profile(request, RuntimeCalculationProfile::BlockFlexScrollGesture)
}

pub(super) fn compute_request_for_block_flex_grid(
    request: &WorkRequest,
) -> Result<RuntimeCalculation, String> {
    compute_request_with_profile(request, RuntimeCalculationProfile::BlockFlexGrid)
}

pub(super) fn compute_request_for_block_flex_grid_placement(
    request: &WorkRequest,
) -> Result<RuntimeCalculation, String> {
    compute_request_with_profile(request, RuntimeCalculationProfile::BlockFlexGridPlacement)
}

pub(super) fn compute_request_for_block_flex_grid_auto_placement(
    request: &WorkRequest,
) -> Result<RuntimeCalculation, String> {
    compute_request_with_profile(
        request,
        RuntimeCalculationProfile::BlockFlexGridAutoPlacement,
    )
}

pub(super) fn compute_request_for_block_flex_grid_dense_auto_placement(
    request: &WorkRequest,
) -> Result<RuntimeCalculation, String> {
    compute_request_with_profile(
        request,
        RuntimeCalculationProfile::BlockFlexGridDenseAutoPlacement,
    )
}

pub(super) fn compute_request_for_block_flex_grid_template(
    request: &WorkRequest,
) -> Result<RuntimeCalculation, String> {
    compute_request_with_profile(request, RuntimeCalculationProfile::BlockFlexGridTemplate)
}

#[cfg(feature = "c11-3-3-grid-implicit-named-lines-fixture-test")]
pub(super) fn compute_request_for_block_flex_grid_template_implicit_named_lines(
    request: &WorkRequest,
) -> Result<RuntimeCalculation, String> {
    compute_request_with_profile(
        request,
        RuntimeCalculationProfile::BlockFlexGridTemplateImplicitNamedLines,
    )
}

pub(super) fn compute_request_for_block_flex_grid_track_sizing(
    request: &WorkRequest,
) -> Result<RuntimeCalculation, String> {
    compute_request_with_profile(request, RuntimeCalculationProfile::BlockFlexGridTrackSizing)
}

fn compute_request_with_profile(
    request: &WorkRequest,
    profile: RuntimeCalculationProfile,
) -> Result<RuntimeCalculation, String> {
    let mut render_snapshot_builder = build_runtime_render_snapshot;
    compute_request_with_profile_and_render_builder(request, profile, &mut render_snapshot_builder)
}

pub(super) fn compute_request_with_profile_and_render_builder<F>(
    request: &WorkRequest,
    profile: RuntimeCalculationProfile,
    render_snapshot_builder: &mut F,
) -> Result<RuntimeCalculation, String>
where
    F: FnMut(
        &spinon_core::HostDocumentSnapshot,
        HostNodeHandle,
        &spinon_style_to_layout::StyleLayoutOutput,
        spinon_style_to_render::CurrentLayoutInputs,
    ) -> Result<
        spinon_render::RuntimeRenderSnapshot,
        spinon_style_to_render::StyleRenderError,
    >,
{
    if let Some((node, depth)) = first_node_past_layout_depth_limit(&request.snapshot)? {
        return Ok(tree_depth_failure(node, depth));
    }

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
        render_snapshot_builder,
    );

    Ok(RuntimeCalculation {
        roots: roots.into(),
        cascade_error: None,
        cascade_duration_us,
        cascade_recomputed_style_elements,
        cascade_reused_style_elements,
        cascade_context_style_elements,
        layout,
        layout_diagnostics,
    })
}

fn first_node_past_layout_depth_limit(
    snapshot: &spinon_core::HostDocumentSnapshot,
) -> Result<Option<(NodeId, usize)>, String> {
    let mut pending = snapshot
        .root_children()
        .map(|root| (root, 1_usize))
        .collect::<Vec<_>>();
    pending.reverse();
    let mut visited = std::collections::BTreeSet::new();

    while let Some((handle, depth)) = pending.pop() {
        if !visited.insert(handle.id()) {
            continue;
        }
        if snapshot.node(handle).is_none() {
            return Err(format!(
                "레이아웃 깊이 사전 검사에서 HostDocument 노드 {}를 찾을 수 없습니다",
                handle.id()
            ));
        }
        if depth > MAX_LAYOUT_TREE_DEPTH {
            return Ok(Some((handle.id(), depth)));
        }
        let Some(children) = snapshot.children(handle) else {
            return Err(format!(
                "레이아웃 깊이 사전 검사에서 HostDocument 자식 목록을 읽을 수 없습니다: {}",
                handle.id()
            ));
        };
        let mut children = children.collect::<Vec<_>>();
        pending.extend(
            children
                .drain(..)
                .rev()
                .map(|child| (child, depth.saturating_add(1))),
        );
    }
    Ok(None)
}

fn tree_depth_failure(node: NodeId, depth: usize) -> RuntimeCalculation {
    RuntimeCalculation {
        roots: Vec::new().into(),
        cascade_error: Some(format!(
            "레이아웃 트리 깊이 {depth}가 최대 {MAX_LAYOUT_TREE_DEPTH}를 초과했습니다 (노드 {node})"
        )),
        cascade_duration_us: 0,
        cascade_recomputed_style_elements: 0,
        cascade_reused_style_elements: 0,
        cascade_context_style_elements: 0,
        layout: Err(layout_failure(
            "tree_depth_exceeded",
            Some(node.get()),
            None,
        )),
        layout_diagnostics: Vec::new(),
    }
}

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

fn root_html_body_node_id(snapshot: &HostDocumentSnapshot, root: HostNodeHandle) -> Option<u64> {
    let HostNodeKind::Element(root_element) = snapshot.node(root)?.kind() else {
        return None;
    };
    if root_element.namespace() != HTML_NAMESPACE || root_element.local_name() != "html" {
        return None;
    }
    snapshot.children(root)?.find_map(|child| {
        let HostNodeKind::Element(element) = snapshot.node(child)?.kind() else {
            return None;
        };
        (element.namespace() == HTML_NAMESPACE && element.local_name() == "body")
            .then(|| child.id().get())
    })
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn c11_4_auto_minmax_span_overflow_root(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
) -> bool {
    const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
    const ROOT_ID_PREFIX: &str = "c114-auto-minmax-span-overflow-";
    const SINGLE_TRACK_ROOT_ID_PREFIX: &str = "c114-auto-min-scrollable-overflow-";

    let Some(node) = snapshot.node(root) else {
        return false;
    };
    let HostNodeKind::Element(element) = node.kind() else {
        return false;
    };
    if element.namespace() != HTML_NAMESPACE || element.local_name() != "div" {
        return false;
    }
    element.attributes().iter().any(|(name, value)| {
        name.namespace().is_none()
            && name.local_name() == "id"
            && (value.to_string_lossy().starts_with(ROOT_ID_PREFIX)
                || value
                    .to_string_lossy()
                    .starts_with(SINGLE_TRACK_ROOT_ID_PREFIX))
    })
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub(super) fn is_c11_4_fit_content_zoom_root(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
) -> bool {
    const SCENE_ROOT_ID: &str = "c114-fit-content-zoomed-grid";
    let Some(node) = snapshot.node(root) else {
        return false;
    };
    let HostNodeKind::Element(element) = node.kind() else {
        return false;
    };
    if element.namespace() != HTML_NAMESPACE
        || element.local_name() != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
    {
        return false;
    }
    element.attributes().iter().any(|(name, value)| {
        name.namespace().is_none()
            && name.local_name() == "id"
            && value.to_string_lossy() == SCENE_ROOT_ID
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
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    let c11_4_auto_minmax_span_overflow_root = calculation_profile
        == RuntimeCalculationProfile::BlockFlexGridTrackSizing
        && c11_4_auto_minmax_span_overflow_root(&request.snapshot, root);
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    let c11_4_fit_content_zoom_root = calculation_profile
        == RuntimeCalculationProfile::BlockFlexGridTrackSizing
        && is_c11_4_fit_content_zoom_root(&request.snapshot, root);
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    let c11_4_visibility_collapse_root = calculation_profile
        == RuntimeCalculationProfile::BlockFlexGridTrackSizing
        && super::runtime_layout::is_c11_4_visibility_collapse_root(&request.snapshot, root);
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    let view = if c11_4_fit_content_zoom_root {
        StyloDocumentView::new_c11_4_fit_content_zoom_runtime_mount_shared(
            Arc::clone(&request.snapshot),
            root,
        )
    } else if is_root_html_paint_profile(calculation_profile) {
        StyloDocumentView::new_html_shared(Arc::clone(&request.snapshot), root)
    } else {
        StyloDocumentView::new_html_runtime_mount_shared(Arc::clone(&request.snapshot), root)
    }
    .map_err(|error| error.to_string())?;
    #[cfg(not(feature = "c11-4-grid-track-sizing-fixture-test"))]
    let view = if is_root_html_paint_profile(calculation_profile) {
        StyloDocumentView::new_html_shared(Arc::clone(&request.snapshot), root)
    } else {
        StyloDocumentView::new_html_runtime_mount_shared(Arc::clone(&request.snapshot), root)
    }
    .map_err(|error| error.to_string())?;
    let profile = match calculation_profile {
        RuntimeCalculationProfile::FlexLayout => {
            ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
        }
        RuntimeCalculationProfile::RootHtmlDocument => {
            ComputedStyleProfile::RuntimeFlexCustomPropertiesMediaEnvironmentV1
        }
        RuntimeCalculationProfile::RootHtmlBlockPaint => {
            ComputedStyleProfile::RuntimeRootHtmlBlockPaintV1
        }
        RuntimeCalculationProfile::RootHtmlDefaultRender => {
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
        }
        #[cfg(feature = "c04-12-root-html-text-paint-fixture-test")]
        RuntimeCalculationProfile::RootHtmlTextPaint => {
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
        }
        RuntimeCalculationProfile::FlexPaint => {
            ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
        }
        RuntimeCalculationProfile::RegisteredPropertiesPaint => {
            ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
        }
        RuntimeCalculationProfile::BlockPaint => ComputedStyleProfile::RuntimeBlockPaintV1,
        RuntimeCalculationProfile::BlockImage => ComputedStyleProfile::RuntimeBlockImageV1,
        RuntimeCalculationProfile::BlockFormatting => {
            ComputedStyleProfile::RuntimeBlockFormattingV1
        }
        RuntimeCalculationProfile::BlockPositioning => {
            ComputedStyleProfile::RuntimeBlockPositioningV1
        }
        RuntimeCalculationProfile::BlockFlexStacking => {
            ComputedStyleProfile::RuntimeBlockFlexStackingV1
        }
        RuntimeCalculationProfile::BlockFlexOverflowClip => {
            ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
        }
        #[cfg(feature = "c19-2-box-paint-opacity-fixture-test")]
        RuntimeCalculationProfile::C19BoxPaintOpacity => ComputedStyleProfile::C19BoxPaintOpacityV1,
        #[cfg(feature = "c13-3-gesture-fixture-test")]
        RuntimeCalculationProfile::BlockFlexScrollGesture => {
            ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
        }
        RuntimeCalculationProfile::BlockFlexGrid => ComputedStyleProfile::RuntimeBlockFlexGridV1,
        RuntimeCalculationProfile::BlockFlexGridPlacement => {
            ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
        }
        RuntimeCalculationProfile::BlockFlexGridAutoPlacement => {
            ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
        }
        RuntimeCalculationProfile::BlockFlexGridDenseAutoPlacement => {
            ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
        }
        RuntimeCalculationProfile::BlockFlexGridTemplate => {
            ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
        }
        #[cfg(feature = "c11-3-3-grid-implicit-named-lines-fixture-test")]
        RuntimeCalculationProfile::BlockFlexGridTemplateImplicitNamedLines => {
            ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
        }
        RuntimeCalculationProfile::BlockFlexGridTrackSizing => {
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        }
    };
    let reuse_previous_styles = !is_root_html_paint_profile(calculation_profile) && {
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        {
            !c11_4_auto_minmax_span_overflow_root
                && !c11_4_fit_content_zoom_root
                && !c11_4_visibility_collapse_root
        }
        #[cfg(not(feature = "c11-4-grid-track-sizing-fixture-test"))]
        {
            true
        }
    };
    let previous_style_root = request
        .previous_styles
        .as_deref()
        .filter(|_| reuse_previous_styles)
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
                request.key.style_revision,
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
                    request.key.style_revision,
                )
            }
            RuntimeCalculationProfile::RootHtmlDocument => {
                spinon_style::compute_runtime_flex_custom_properties_media_environment_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    request.key.style_revision,
                )
            }
            RuntimeCalculationProfile::RootHtmlBlockPaint => {
                spinon_style::compute_runtime_root_html_block_paint_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    request.key.style_revision,
                )
            }
            RuntimeCalculationProfile::RootHtmlDefaultRender => {
                spinon_style::compute_runtime_root_html_text_paint_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    request.key.style_revision,
                )
            }
            #[cfg(feature = "c04-12-root-html-text-paint-fixture-test")]
            RuntimeCalculationProfile::RootHtmlTextPaint => {
                spinon_style::compute_runtime_root_html_text_paint_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    request.key.style_revision,
                )
            }
            RuntimeCalculationProfile::FlexPaint => {
                compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    request.key.style_revision,
                )
            }
            RuntimeCalculationProfile::RegisteredPropertiesPaint => {
                compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    request.key.style_revision,
                )
            }
            RuntimeCalculationProfile::BlockPaint => {
                compute_runtime_block_paint_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    request.key.style_revision,
                )
            }
            RuntimeCalculationProfile::BlockImage => {
                spinon_style::compute_runtime_block_image_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    request.key.style_revision,
                )
            }
            RuntimeCalculationProfile::BlockFormatting => {
                compute_runtime_block_formatting_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    request.key.style_revision,
                )
            }
            RuntimeCalculationProfile::BlockPositioning => {
                compute_runtime_block_positioning_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    request.key.style_revision,
                )
            }
            RuntimeCalculationProfile::BlockFlexStacking => {
                compute_runtime_block_flex_stacking_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    request.key.style_revision,
                )
            }
            RuntimeCalculationProfile::BlockFlexOverflowClip => {
                compute_runtime_block_flex_overflow_clip_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    request.key.style_revision,
                )
            }
            #[cfg(feature = "c19-2-box-paint-opacity-fixture-test")]
            RuntimeCalculationProfile::C19BoxPaintOpacity => {
                compute_c19_box_paint_opacity_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    request.key.style_revision,
                )
            }
            #[cfg(feature = "c13-3-gesture-fixture-test")]
            RuntimeCalculationProfile::BlockFlexScrollGesture => {
                compute_runtime_block_flex_scroll_gesture_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    request.key.style_revision,
                )
            }
            RuntimeCalculationProfile::BlockFlexGrid => {
                compute_runtime_block_flex_grid_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    request.key.style_revision,
                )
            }
            RuntimeCalculationProfile::BlockFlexGridPlacement => {
                compute_runtime_block_flex_grid_placement_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    request.key.style_revision,
                )
            }
            RuntimeCalculationProfile::BlockFlexGridAutoPlacement => {
                compute_runtime_block_flex_grid_auto_placement_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    request.key.style_revision,
                )
            }
            RuntimeCalculationProfile::BlockFlexGridDenseAutoPlacement => {
                compute_runtime_block_flex_grid_dense_auto_placement_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    request.key.style_revision,
                )
            }
            RuntimeCalculationProfile::BlockFlexGridTemplate => {
                compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    request.key.style_revision,
                )
            }
            #[cfg(feature = "c11-3-3-grid-implicit-named-lines-fixture-test")]
            RuntimeCalculationProfile::BlockFlexGridTemplateImplicitNamedLines => {
                compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    request.key.style_revision,
                )
            }
            RuntimeCalculationProfile::BlockFlexGridTrackSizing => {
                #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
                if c11_4_visibility_collapse_root {
                    compute_runtime_block_flex_grid_track_sizing_visibility_collapse_fixture_test_cascade_with_stylesheets(
                        &view,
                        author_stylesheets,
                        request.viewport,
                        request.key.style_revision,
                    )
                } else if c11_4_auto_minmax_span_overflow_root {
                    spinon_style::compute_runtime_block_flex_grid_track_sizing_overflow_fixture_test_cascade_with_stylesheets(
                        &view,
                        author_stylesheets,
                        request.viewport,
                        request.key.style_revision,
                    )
                } else if c11_4_fit_content_zoom_root {
                    spinon_style::compute_runtime_block_flex_grid_track_sizing_zoom_fixture_test_cascade_with_stylesheets(
                        &view,
                        author_stylesheets,
                        request.viewport,
                        request.key.style_revision,
                    )
                } else {
                    compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
                        &view,
                        author_stylesheets,
                        request.viewport,
                        request.key.style_revision,
                    )
                }
                #[cfg(not(feature = "c11-4-grid-track-sizing-fixture-test"))]
                compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
                    &view,
                    author_stylesheets,
                    request.viewport,
                    request.key.style_revision,
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
    if key_for_with_style_revision(
        &request.snapshot,
        request.viewport.environment_revision,
        request.key.style_revision,
    ) != request.key
        || styles.generation.get() != request.key.generation
        || styles.document_revision.get() != request.key.document_revision
        || styles.render_tree_revision.get() != request.key.render_tree_revision
        || styles.style_revision != request.key.style_revision
    {
        return Err("Stylo 결과 revision이 요청 key와 다릅니다".to_owned());
    }
    Ok((
        RuntimeUaCascadeRoot {
            root_node_id: root.id().get(),
            body_node_id: root_html_body_node_id(&request.snapshot, root),
            styles,
        },
        view,
        reuse_stats,
    ))
}
