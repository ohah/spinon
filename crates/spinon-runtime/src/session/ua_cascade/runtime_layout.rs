use super::{
    RuntimeUaCascadeHandle, RuntimeUaCascadeKey, State, WorkRequest, elapsed_microseconds, lock,
};
use spinon_core::{HostDocumentSnapshot, HostNodeHandle};
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
use spinon_layout::LayoutTextSnapshot;
use spinon_layout::{LayoutError, LayoutFrame, LayoutOutput};
use spinon_render::RuntimeRenderSnapshot;
#[cfg(feature = "c13-3-gesture-fixture-test")]
use spinon_style::first_unsupported_runtime_block_flex_scroll_gesture_inline_property;
use spinon_style::{
    CascadeDiagnostic, ComputedStyleSnapshot, StylesheetSource, StyloDocumentView,
    first_unsupported_runtime_block_display_inline_value,
    first_unsupported_runtime_block_display_stylesheet_value,
    first_unsupported_runtime_block_flex_grid_auto_placement_inline_property,
    first_unsupported_runtime_block_flex_grid_dense_auto_placement_inline_property,
    first_unsupported_runtime_block_flex_grid_inline_property,
    first_unsupported_runtime_block_flex_overflow_clip_inline_property,
    first_unsupported_runtime_block_paint_inline_property,
    first_unsupported_runtime_block_positioning_inline_property,
    first_unsupported_runtime_custom_properties_inline_property,
    first_unsupported_runtime_custom_properties_paint_inline_property,
};
use spinon_style_to_layout::{StyleLayoutError, StyleLayoutOutput, compute_runtime_style_layout};
use spinon_style_to_render::{CurrentLayoutInputs, StyleRenderError};
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "runtime_layout/fixtures.rs"]
mod fixtures;
#[path = "runtime_layout/root_html_default_render.rs"]
mod root_html_default_render;
#[cfg(feature = "c04-12-root-html-text-paint-fixture-test")]
#[path = "runtime_layout/root_html_text_paint.rs"]
mod root_html_text_paint;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeLayoutState {
    NotConfigured,
    Pending,
    Ready,
    Empty,
    Failed,
}

impl RuntimeLayoutState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotConfigured => "not_configured",
            Self::Pending => "pending",
            Self::Ready => "ready",
            Self::Empty => "empty",
            Self::Failed => "failed",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RuntimeLayoutFrame {
    pub node_id: u64,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeLayoutFailure {
    pub code: &'static str,
    pub node_id: Option<u64>,
    pub property: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeLayoutCompleted {
    pub key: RuntimeUaCascadeKey,
    pub frames: Arc<[RuntimeLayoutFrame]>,
    pub render_snapshot: Option<RuntimeRenderSnapshot>,
    pub projection_duration_us: u128,
    pub cache_hit: bool,
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    pub grid_track_sizing_diagnostics: Option<Arc<RuntimeGridTrackSizingDiagnostics>>,
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeGridTrackSizingDiagnostics {
    pub revision: spinon_layout::LayoutInputRevision,
    pub intrinsic_measurements: Arc<[spinon_layout::LayoutIntrinsicMeasurement]>,
    pub item_inputs: Arc<[spinon_layout::LayoutGridItemIntrinsicInput]>,
    pub item_text_layouts: Arc<[LayoutTextSnapshot]>,
    pub item_frames: Arc<[(spinon_core::NodeId, LayoutFrame)]>,
    pub traces: Arc<[spinon_layout::LayoutGridTrackSizingTrace]>,
    pub fit_content_auto_neighbor: Option<RuntimeC11_4FitContentAutoNeighborObservation>,
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeC11_4FitContentAutoNeighborObservation {
    pub style_snapshot: ComputedStyleSnapshot,
    pub grid_root: spinon_core::NodeId,
    pub grid_container_input: spinon_layout::GridContainerInput,
    pub node_ids: [spinon_core::NodeId; 5],
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
impl RuntimeGridTrackSizingDiagnostics {
    pub(crate) fn with_revision(&self, revision: spinon_layout::LayoutInputRevision) -> Self {
        let intrinsic_measurements = self
            .intrinsic_measurements
            .iter()
            .cloned()
            .map(|mut measurement| {
                measurement.revision = revision;
                measurement
            })
            .collect::<Vec<_>>()
            .into();
        let item_inputs = self
            .item_inputs
            .iter()
            .cloned()
            .map(|mut input| {
                input.revision = revision;
                input
            })
            .collect::<Vec<_>>()
            .into();
        let item_text_layouts = self
            .item_text_layouts
            .iter()
            .cloned()
            .map(|mut text_layout| {
                text_layout.revision = revision;
                text_layout
            })
            .collect::<Vec<_>>()
            .into();
        Self {
            revision,
            intrinsic_measurements,
            item_inputs,
            item_text_layouts,
            item_frames: Arc::clone(&self.item_frames),
            traces: Arc::clone(&self.traces),
            fit_content_auto_neighbor: self.fit_content_auto_neighbor.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeLayoutSnapshot {
    pub state: RuntimeLayoutState,
    pub requested: Option<RuntimeUaCascadeKey>,
    pub completed: Option<Arc<RuntimeLayoutCompleted>>,
    pub diagnostics: Vec<CascadeDiagnostic>,
    pub error: Option<RuntimeLayoutFailure>,
}

impl RuntimeUaCascadeHandle {
    pub(in crate::session) fn layout_snapshot(&self) -> RuntimeLayoutSnapshot {
        let state = lock(&self.shared.state);
        layout_snapshot_from_state(&state)
    }

    pub(in crate::session) fn wait_for_layout_snapshot(
        &self,
        timeout: Duration,
    ) -> RuntimeLayoutSnapshot {
        let deadline = Instant::now() + timeout;
        let mut state = lock(&self.shared.state);
        while state.layout_status == RuntimeLayoutState::Pending && !state.closing {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                break;
            }
            let (next, timed_out) = self
                .shared
                .wake
                .wait_timeout(state, remaining)
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            state = next;
            if timed_out.timed_out() {
                break;
            }
        }
        layout_snapshot_from_state(&state)
    }
}

fn layout_snapshot_from_state(state: &State) -> RuntimeLayoutSnapshot {
    RuntimeLayoutSnapshot {
        state: state.layout_status,
        requested: state.requested,
        completed: state.layout_completed.clone(),
        diagnostics: state.layout_diagnostics.clone(),
        error: state.layout_error.clone(),
    }
}

pub(super) type RuntimeLayoutContext = (HostNodeHandle, StyloDocumentView, ComputedStyleSnapshot);

pub(super) fn compute_runtime_layout<F>(
    request: &WorkRequest,
    root_count: usize,
    layout_context: Option<RuntimeLayoutContext>,
    author_stylesheets: &[StylesheetSource],
    runtime_paint_enabled: bool,
    profile: super::calculation::RuntimeCalculationProfile,
    render_snapshot_builder: &mut F,
) -> Result<RuntimeLayoutCompleted, RuntimeLayoutFailure>
where
    F: FnMut(
        &HostDocumentSnapshot,
        HostNodeHandle,
        &StyleLayoutOutput,
        CurrentLayoutInputs,
    ) -> Result<RuntimeRenderSnapshot, StyleRenderError>,
{
    if root_count == 0 {
        let render_snapshot = if runtime_paint_enabled {
            Some(empty_runtime_render_snapshot(request)?)
        } else {
            None
        };
        return Ok(RuntimeLayoutCompleted {
            key: request.key,
            frames: Vec::new().into(),
            render_snapshot,
            projection_duration_us: 0,
            cache_hit: false,
            #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
            grid_track_sizing_diagnostics: None,
        });
    }
    if root_count > 1 {
        return Err(layout_failure("multiple_host_roots", None, None));
    }

    let (root, view, styles) =
        layout_context.expect("단일 스타일 root의 layout 입력이 있어야 합니다");
    if profile == super::calculation::RuntimeCalculationProfile::BlockFormatting {
        if let Some((node, value)) = first_unsupported_runtime_block_display_inline_value(&view) {
            return Err(layout_failure(
                "unsupported_block_display",
                Some(node.get()),
                Some(value),
            ));
        }
        for stylesheet in author_stylesheets {
            if let Some(value) =
                first_unsupported_runtime_block_display_stylesheet_value(&stylesheet.css)
            {
                return Err(layout_failure(
                    "unsupported_block_stylesheet",
                    None,
                    Some(format!("{}: {value}", stylesheet.id)),
                ));
            }
        }
    }
    let unsupported_property = match profile {
        super::calculation::RuntimeCalculationProfile::BlockPaint => {
            first_unsupported_runtime_block_paint_inline_property(&view)
        }
        super::calculation::RuntimeCalculationProfile::RootHtmlBlockPaint => {
            spinon_style::first_unsupported_runtime_root_html_block_paint_inline_property(&view)
        }
        super::calculation::RuntimeCalculationProfile::RootHtmlDefaultRender => {
            spinon_style::first_unsupported_runtime_root_html_text_paint_inline_property(&view)
        }
        #[cfg(feature = "c04-12-root-html-text-paint-fixture-test")]
        super::calculation::RuntimeCalculationProfile::RootHtmlTextPaint => {
            spinon_style::first_unsupported_runtime_root_html_text_paint_inline_property(&view)
        }
        super::calculation::RuntimeCalculationProfile::BlockImage => {
            spinon_style::first_unsupported_runtime_block_image_inline_property(&view)
        }
        super::calculation::RuntimeCalculationProfile::BlockFormatting => {
            spinon_style::first_unsupported_runtime_block_formatting_inline_property(&view)
        }
        super::calculation::RuntimeCalculationProfile::BlockPositioning => {
            first_unsupported_runtime_block_positioning_inline_property(&view)
        }
        super::calculation::RuntimeCalculationProfile::BlockFlexStacking => {
            spinon_style::first_unsupported_runtime_block_flex_stacking_inline_property(&view)
        }
        super::calculation::RuntimeCalculationProfile::BlockFlexOverflowClip => {
            first_unsupported_runtime_block_flex_overflow_clip_inline_property(&view)
        }
        #[cfg(feature = "c19-2-box-paint-opacity-fixture-test")]
        super::calculation::RuntimeCalculationProfile::C19BoxPaintOpacity => {
            first_unsupported_runtime_block_flex_overflow_clip_inline_property(&view)
        }
        #[cfg(feature = "c13-3-gesture-fixture-test")]
        super::calculation::RuntimeCalculationProfile::BlockFlexScrollGesture => {
            first_unsupported_runtime_block_flex_scroll_gesture_inline_property(&view)
        }
        super::calculation::RuntimeCalculationProfile::BlockFlexGrid => {
            first_unsupported_runtime_block_flex_grid_inline_property(&view)
        }
        super::calculation::RuntimeCalculationProfile::BlockFlexGridPlacement => {
            spinon_style::first_unsupported_runtime_block_flex_grid_placement_inline_property(&view)
        }
        super::calculation::RuntimeCalculationProfile::BlockFlexGridAutoPlacement => {
            first_unsupported_runtime_block_flex_grid_auto_placement_inline_property(&view)
        }
        super::calculation::RuntimeCalculationProfile::BlockFlexGridDenseAutoPlacement => {
            first_unsupported_runtime_block_flex_grid_dense_auto_placement_inline_property(&view)
        }
        super::calculation::RuntimeCalculationProfile::BlockFlexGridTemplate => {
            spinon_style::first_unsupported_runtime_block_flex_grid_template_inline_property(&view)
        }
        #[cfg(feature = "c11-3-3-grid-implicit-named-lines-fixture-test")]
        super::calculation::RuntimeCalculationProfile::BlockFlexGridTemplateImplicitNamedLines => {
            spinon_style::first_unsupported_runtime_block_flex_grid_template_inline_property(&view)
        }
        super::calculation::RuntimeCalculationProfile::BlockFlexGridTrackSizing => {
            spinon_style::first_unsupported_runtime_block_flex_grid_track_sizing_inline_property(
                &view,
            )
        }
        super::calculation::RuntimeCalculationProfile::FlexPaint
        | super::calculation::RuntimeCalculationProfile::RegisteredPropertiesPaint => {
            first_unsupported_runtime_custom_properties_paint_inline_property(&view)
        }
        super::calculation::RuntimeCalculationProfile::FlexLayout => {
            first_unsupported_runtime_custom_properties_inline_property(&view)
        }
        super::calculation::RuntimeCalculationProfile::RootHtmlDocument => {
            spinon_style::first_unsupported_runtime_root_html_inline_property(&view)
        }
    };
    if let Some((node, property)) = unsupported_property {
        return Err(layout_failure(
            "unsupported_inline_property",
            Some(node.get()),
            Some(property),
        ));
    }
    if profile == super::calculation::RuntimeCalculationProfile::RootHtmlBlockPaint {
        validate_root_html_block_paint_tree(&request.snapshot, root, &styles)?;
    }
    let root_html_default_render_input =
        if profile == super::calculation::RuntimeCalculationProfile::RootHtmlDefaultRender {
            Some(root_html_default_render::prepare(
                &request.snapshot,
                root,
                &styles,
            )?)
        } else {
            None
        };
    #[cfg(feature = "c04-12-root-html-text-paint-fixture-test")]
    let root_html_text_input =
        if profile == super::calculation::RuntimeCalculationProfile::RootHtmlTextPaint {
            Some(root_html_text_paint::prepare(
                &request.snapshot,
                root,
                &styles,
            )?)
        } else {
            None
        };
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    let c11_4_nested_grid_replaced_item_input =
        if profile == super::calculation::RuntimeCalculationProfile::BlockFlexGridTrackSizing {
            fixtures::prepare_c11_4_nested_grid_replaced_item(&request.snapshot, root, &styles)?
        } else {
            None
        };
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    let c11_4_nested_grid_depth_three_anonymous_image_input =
        if profile == super::calculation::RuntimeCalculationProfile::BlockFlexGridTrackSizing {
            fixtures::prepare_c11_4_nested_grid_depth_three_anonymous_image(
                &request.snapshot,
                root,
                &styles,
            )?
        } else {
            None
        };
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    let c11_4_anonymous_text_multi_image_input = if profile
        == super::calculation::RuntimeCalculationProfile::BlockFlexGridTrackSizing
        && c11_4_nested_grid_replaced_item_input.is_none()
        && c11_4_nested_grid_depth_three_anonymous_image_input.is_none()
    {
        if let Some(input) = fixtures::prepare_c11_4_anonymous_grid_border_overflow(
            &request.snapshot,
            root,
            &styles,
        )? {
            Some(input)
        } else {
            fixtures::prepare_c11_4_anonymous_text_multi_image(&request.snapshot, root, &styles)?
        }
    } else {
        None
    };
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    let c11_4_anonymous_text_image_input = if c11_4_nested_grid_replaced_item_input.is_none()
        && c11_4_nested_grid_depth_three_anonymous_image_input.is_none()
        && c11_4_anonymous_text_multi_image_input.is_none()
        && profile == super::calculation::RuntimeCalculationProfile::BlockFlexGridTrackSizing
    {
        fixtures::prepare_c11_4_anonymous_text_image(&request.snapshot, root, &styles)?
    } else {
        None
    };
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    let c11_4_mixed_input = if c11_4_nested_grid_replaced_item_input.is_none()
        && c11_4_nested_grid_depth_three_anonymous_image_input.is_none()
        && c11_4_anonymous_text_multi_image_input.is_none()
        && c11_4_anonymous_text_image_input.is_none()
        && profile == super::calculation::RuntimeCalculationProfile::BlockFlexGridTrackSizing
    {
        fixtures::prepare_c11_4_mixed(&request.snapshot, root, &styles)?
    } else {
        None
    };
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    let c11_4_anonymous_text_input = if c11_4_nested_grid_replaced_item_input.is_none()
        && c11_4_nested_grid_depth_three_anonymous_image_input.is_none()
        && c11_4_anonymous_text_multi_image_input.is_none()
        && c11_4_anonymous_text_image_input.is_none()
        && c11_4_mixed_input.is_none()
        && profile == super::calculation::RuntimeCalculationProfile::BlockFlexGridTrackSizing
    {
        fixtures::prepare_c11_4_anonymous_text(&request.snapshot, root, &styles)?
    } else {
        None
    };
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    let c11_4_text_input = if c11_4_nested_grid_replaced_item_input.is_none()
        && c11_4_nested_grid_depth_three_anonymous_image_input.is_none()
        && c11_4_anonymous_text_multi_image_input.is_none()
        && c11_4_anonymous_text_image_input.is_none()
        && c11_4_mixed_input.is_none()
        && c11_4_anonymous_text_input.is_none()
        && profile == super::calculation::RuntimeCalculationProfile::BlockFlexGridTrackSizing
    {
        fixtures::prepare_c11_4_text(&request.snapshot, root, &styles)?
    } else {
        None
    };
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    let c11_4_fit_content_calc_render_input = if runtime_paint_enabled
        && fixtures::is_cyclic_fit_content_calc_scene_root(&request.snapshot, root)?
    {
        let text_input = c11_4_text_input.as_ref().ok_or_else(|| {
            layout_failure(
                "c114_fit_content_calc_text_input_missing",
                Some(root.id().get()),
                None,
            )
        })?;
        let owners = text_input
            .paragraph_styles
            .keys()
            .copied()
            .collect::<Vec<_>>();
        if owners.is_empty() {
            return Err(layout_failure(
                "c114_fit_content_calc_text_owner_count",
                Some(root.id().get()),
                None,
            ));
        }
        Some(fixtures::prepare_cyclic_fit_content_calc_render_input(
            &request.snapshot,
            root,
            &styles,
            &owners,
            &text_input.engine,
        )?)
    } else {
        None
    };
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    let c11_4_image_input = if c11_4_nested_grid_replaced_item_input.is_none()
        && c11_4_nested_grid_depth_three_anonymous_image_input.is_none()
        && c11_4_anonymous_text_multi_image_input.is_none()
        && c11_4_anonymous_text_image_input.is_none()
        && c11_4_mixed_input.is_none()
        && profile == super::calculation::RuntimeCalculationProfile::BlockFlexGridTrackSizing
    {
        fixtures::prepare_c11_4_image(&request.snapshot, root, &styles)?
    } else {
        None
    };
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    let c11_4_fit_content_auto_neighbor_node_ids =
        if profile == super::calculation::RuntimeCalculationProfile::BlockFlexGridTrackSizing {
            fixtures::prepare_c11_4_fit_content_auto_neighbor(&request.snapshot, root, &styles)?
        } else {
            None
        };
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    if c11_4_nested_grid_depth_three_anonymous_image_input.is_none()
        && (c11_4_text_input.is_some() || c11_4_anonymous_text_input.is_some())
        && c11_4_image_input.is_some()
    {
        return Err(layout_failure(
            "unsupported_c114_mixed_intrinsic_fixture",
            Some(root.id().get()),
            Some("text와 image contribution 동시 입력은 아직 지원하지 않습니다".to_owned()),
        ));
    }
    let projection_started = Instant::now();
    if runtime_paint_enabled && let Some(diagnostic) = styles.diagnostics.first() {
        return Err(layout_failure(
            "cascade_diagnostics",
            diagnostic.node_id.map(|node| node.get()),
            Some(format!(
                "{}:{}:{} {}",
                diagnostic.source_id,
                diagnostic.diagnostic.line,
                diagnostic.diagnostic.column,
                diagnostic.diagnostic.message
            )),
        ));
    }
    let mut output = if let Some(text_input) = &root_html_default_render_input {
        spinon_style_to_layout::compute_runtime_style_layout_with_text_profiles(
            &request.snapshot,
            root,
            styles.clone(),
            request.viewport,
            &text_input.paragraph_styles,
            &text_input.inline_run_styles,
            &text_input.engine,
        )
    } else {
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        if let Some(mixed_input) = &c11_4_nested_grid_replaced_item_input {
            spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_and_intrinsic_sizes(
                &request.snapshot,
                root,
                styles.clone(),
                request.viewport,
                &mixed_input.paragraph_styles,
                &mixed_input.engine,
                mixed_input.intrinsic_sizes.clone(),
            )
        } else if let Some(mixed_input) = &c11_4_nested_grid_depth_three_anonymous_image_input {
            spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_anonymous_text_and_intrinsic_sizes_and_two_axis_nested_grid(
                &request.snapshot,
                root,
                styles.clone(),
                request.viewport,
                &mixed_input.paragraph_styles,
                &mixed_input.engine,
                mixed_input.intrinsic_sizes.clone(),
            )
        } else if let Some(mixed_input) = &c11_4_anonymous_text_multi_image_input {
            spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_anonymous_text_and_intrinsic_sizes(
                &request.snapshot,
                root,
                styles.clone(),
                request.viewport,
                &mixed_input.paragraph_styles,
                &mixed_input.engine,
                mixed_input.intrinsic_sizes.clone(),
            )
        } else if let Some(mixed_input) = &c11_4_anonymous_text_image_input {
            spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_anonymous_text_and_intrinsic_sizes(
                &request.snapshot,
                root,
                styles.clone(),
                request.viewport,
                &mixed_input.paragraph_styles,
                &mixed_input.engine,
                mixed_input.intrinsic_sizes.clone(),
            )
        } else if let Some(mixed_input) = &c11_4_mixed_input {
            if mixed_input.allow_two_axis_root_intrinsic {
                spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_and_intrinsic_sizes_and_two_axis_grid_fixture(
                    &request.snapshot,
                    root,
                    styles.clone(),
                    request.viewport,
                    &mixed_input.paragraph_styles,
                    &mixed_input.engine,
                    mixed_input.intrinsic_sizes.clone(),
                )
            } else {
                spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_and_intrinsic_sizes(
                    &request.snapshot,
                    root,
                    styles.clone(),
                    request.viewport,
                    &mixed_input.paragraph_styles,
                    &mixed_input.engine,
                    mixed_input.intrinsic_sizes.clone(),
                )
            }
        } else if let Some(text_input) = &c11_4_anonymous_text_input {
            spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs(
                &request.snapshot,
                root,
                styles.clone(),
                request.viewport,
                &text_input.paragraph_styles,
                &text_input.engine,
            )
        } else if let Some(text_input) = &c11_4_text_input {
            if text_input.two_axis_nested_grid {
                spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_two_axis_nested_grid_text_paragraphs(
                    &request.snapshot,
                    root,
                    styles.clone(),
                    request.viewport,
                    &text_input.paragraph_styles,
                    &text_input.engine,
                )
            } else {
                spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs(
                    &request.snapshot,
                    root,
                    styles.clone(),
                    request.viewport,
                    &text_input.paragraph_styles,
                    &text_input.engine,
                )
            }
        } else if let Some(image_input) = &c11_4_image_input {
            if image_input.allow_two_axis_root_intrinsic {
                spinon_style_to_layout::compute_runtime_style_layout_with_intrinsic_size_snapshot_for_c11_4_two_axis_grid_fixture(
                    &request.snapshot,
                    root,
                    styles.clone(),
                    request.viewport,
                    image_input.intrinsic_sizes.clone(),
                )
            } else {
                spinon_style_to_layout::compute_runtime_style_layout_with_intrinsic_size_snapshot(
                    &request.snapshot,
                    root,
                    styles.clone(),
                    request.viewport,
                    image_input.intrinsic_sizes.clone(),
                )
            }
        } else {
            #[cfg(feature = "c04-12-root-html-text-paint-fixture-test")]
            if let Some(text_input) = &root_html_text_input {
                spinon_style_to_layout::compute_runtime_style_layout_with_text_paragraphs(
                    &request.snapshot,
                    root,
                    styles.clone(),
                    request.viewport,
                    &text_input.paragraph_styles,
                    &text_input.engine,
                )
            } else {
                compute_runtime_style_layout_for_profile(
                    &request.snapshot,
                    root,
                    &styles,
                    request.viewport,
                    profile,
                )
            }
            #[cfg(not(feature = "c04-12-root-html-text-paint-fixture-test"))]
            {
                compute_runtime_style_layout_for_profile(
                    &request.snapshot,
                    root,
                    &styles,
                    request.viewport,
                    profile,
                )
            }
        }
        #[cfg(not(feature = "c11-4-grid-track-sizing-fixture-test"))]
        #[cfg(feature = "c04-12-root-html-text-paint-fixture-test")]
        if let Some(text_input) = &root_html_text_input {
            spinon_style_to_layout::compute_runtime_style_layout_with_text_paragraphs(
                &request.snapshot,
                root,
                styles.clone(),
                request.viewport,
                &text_input.paragraph_styles,
                &text_input.engine,
            )
        } else {
            compute_runtime_style_layout_for_profile(
                &request.snapshot,
                root,
                &styles,
                request.viewport,
                profile,
            )
        }
        #[cfg(not(feature = "c11-4-grid-track-sizing-fixture-test"))]
        #[cfg(not(feature = "c04-12-root-html-text-paint-fixture-test"))]
        {
            compute_runtime_style_layout_for_profile(
                &request.snapshot,
                root,
                &styles,
                request.viewport,
                profile,
            )
        }
    }
    .map_err(layout_failure_from_error)?;
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    let fit_content_auto_neighbor_observation =
        if let Some(node_ids) = c11_4_fit_content_auto_neighbor_node_ids {
            Some(fixtures::c11_4_fit_content_auto_neighbor_observation(
                root,
                &output.computed_styles,
                &output.grid_container_inputs,
                node_ids,
            )?)
        } else {
            None
        };
    let frames = runtime_frames(&request.snapshot, root, &styles, &output.layout)?;
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    let anonymous_grid_text_item_frames =
        detach_anonymous_grid_text_item_frames(&styles, &mut output.layout)?;
    if styles.profile == spinon_style::ComputedStyleProfile::RuntimeRootHtmlTextPaintV1 {
        let document_elements = host_document_element_ids(&request.snapshot, root)?;
        if let Some(node) = output
            .layout
            .frames
            .keys()
            .find(|node| !document_elements.contains(node))
        {
            return Err(layout_failure(
                "root_html_default_render_frame_outside_document",
                Some(node.get()),
                None,
            ));
        }
        let paint_nodes = frames
            .iter()
            .map(|frame| frame.node_id)
            .collect::<BTreeSet<_>>();
        output
            .layout
            .frames
            .retain(|node, _| paint_nodes.contains(&node.get()));
    }
    let render_snapshot = if runtime_paint_enabled {
        let current = CurrentLayoutInputs::for_host_document(
            &request.snapshot,
            styles.style_revision,
            request.viewport,
        );
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        let current = if let Some(mixed_input) = &c11_4_nested_grid_replaced_item_input {
            current.with_intrinsic_size_revision(mixed_input.intrinsic_sizes.revision())
        } else if let Some(mixed_input) = &c11_4_nested_grid_depth_three_anonymous_image_input {
            current.with_intrinsic_size_revision(mixed_input.intrinsic_sizes.revision())
        } else if let Some(mixed_input) = &c11_4_anonymous_text_multi_image_input {
            current.with_intrinsic_size_revision(mixed_input.intrinsic_sizes.revision())
        } else if let Some(mixed_input) = &c11_4_anonymous_text_image_input {
            current.with_intrinsic_size_revision(mixed_input.intrinsic_sizes.revision())
        } else if let Some(mixed_input) = &c11_4_mixed_input {
            current.with_intrinsic_size_revision(mixed_input.intrinsic_sizes.revision())
        } else if let Some(image_input) = &c11_4_image_input {
            current.with_intrinsic_size_revision(image_input.intrinsic_sizes.revision())
        } else {
            current
        };
        let scene = render_snapshot_builder(&request.snapshot, root, &output, current)
            .map_err(layout_failure_from_render_error)?;
        #[cfg(feature = "c04-12-root-html-text-paint-fixture-test")]
        let scene = if let Some(text_input) = &root_html_text_input {
            root_html_text_paint::attach(&request.snapshot, root, &output, text_input, scene)?
        } else {
            scene
        };
        let scene = if let Some(text_input) = &root_html_default_render_input {
            root_html_default_render::attach(&request.snapshot, root, &output, text_input, scene)?
        } else {
            scene
        };
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        let scene = if let Some(text_input) = &c11_4_fit_content_calc_render_input {
            root_html_default_render::attach(&request.snapshot, root, &output, text_input, scene)?
        } else {
            scene
        };
        Some(scene)
    } else {
        None
    };
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    let grid_track_sizing_diagnostics = {
        let intrinsic_measurements = std::mem::take(&mut output.layout.intrinsic_measurements);
        let item_inputs = std::mem::take(&mut output.layout.grid_item_intrinsic_inputs);
        let traces = std::mem::take(&mut output.layout.grid_track_sizing_traces);
        if profile != super::calculation::RuntimeCalculationProfile::BlockFlexGridTrackSizing
            && intrinsic_measurements.is_empty()
            && item_inputs.is_empty()
            && traces.is_empty()
        {
            None
        } else {
            let item_frames = item_inputs
                .iter()
                .map(|input| {
                    output
                        .layout
                        .frames
                        .get(&input.node)
                        .copied()
                        .or_else(|| anonymous_grid_text_item_frames.get(&input.node).copied())
                        .map(|frame| (input.node, frame))
                        .ok_or_else(|| {
                            layout_failure("missing_grid_item_frame", Some(input.node.get()), None)
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            let item_text_layouts = item_inputs
                .iter()
                .filter_map(|input| output.layout.text_layouts.get(&input.node).cloned())
                .collect::<Vec<_>>();
            Some(Arc::new(RuntimeGridTrackSizingDiagnostics {
                revision: output.layout.revision,
                intrinsic_measurements: intrinsic_measurements.into(),
                item_inputs: item_inputs.into(),
                item_text_layouts: item_text_layouts.into(),
                item_frames: item_frames.into(),
                traces: traces.into(),
                fit_content_auto_neighbor: fit_content_auto_neighbor_observation,
            }))
        }
    };
    let completed = RuntimeLayoutCompleted {
        key: request.key,
        frames: frames.into(),
        render_snapshot,
        projection_duration_us: elapsed_microseconds(projection_started),
        cache_hit: false,
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        grid_track_sizing_diagnostics,
    };
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    if c11_4_fit_content_auto_neighbor_node_ids.is_some() {
        fixtures::validate_c11_4_fit_content_auto_neighbor_output(&completed)?;
    }
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    if let Some(mixed_input) = &c11_4_mixed_input {
        fixtures::validate_c11_4_orthogonal_rows_output(
            &request.snapshot,
            root,
            &styles,
            mixed_input.intrinsic_sizes.revision(),
            &completed,
        )?;
    }
    Ok(completed)
}

fn validate_root_html_block_paint_tree(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<(), RuntimeLayoutFailure> {
    const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
    let style_by_node = styles
        .elements
        .iter()
        .map(|style| (style.node_id, style))
        .collect::<std::collections::BTreeMap<_, _>>();
    let root_element = snapshot
        .node(root)
        .and_then(|node| match node.kind() {
            spinon_core::HostNodeKind::Element(element) => Some(element),
            spinon_core::HostNodeKind::Text(_) => None,
        })
        .ok_or_else(|| layout_failure("invalid_root_html", Some(root.id().get()), None))?;
    if root_element.namespace() != HTML_NAMESPACE || root_element.local_name() != "html" {
        return Err(layout_failure(
            "invalid_root_html",
            Some(root.id().get()),
            Some("documentElement must be HTML html".to_owned()),
        ));
    }

    let mut pending = vec![(root, false)];
    while let Some((handle, ancestor_hidden)) = pending.pop() {
        let node = snapshot
            .node(handle)
            .ok_or_else(|| layout_failure("missing_node", Some(handle.id().get()), None))?;
        if let spinon_core::HostNodeKind::Text(text) = node.kind() {
            if !ancestor_hidden && !text.is_only_collapsible_html_whitespace() {
                return Err(layout_failure(
                    "visible_root_html_text",
                    Some(node.id().get()),
                    None,
                ));
            }
            continue;
        }
        let style = style_by_node
            .get(&node.id())
            .ok_or_else(|| layout_failure("missing_computed_style", Some(node.id().get()), None))?;
        let display = style
            .properties
            .get("display")
            .map(String::as_str)
            .ok_or_else(|| {
                layout_failure(
                    "missing_computed_property",
                    Some(node.id().get()),
                    Some("display".to_owned()),
                )
            })?;
        let hidden = ancestor_hidden || display == "none";
        if !hidden {
            let spinon_core::HostNodeKind::Element(element) = node.kind() else {
                unreachable!("text nodes returned above")
            };
            if element.namespace() != HTML_NAMESPACE
                || !matches!(element.local_name(), "html" | "body" | "main" | "div")
            {
                return Err(layout_failure(
                    "unsupported_visible_root_html_element",
                    Some(node.id().get()),
                    Some(format!("{}:{}", element.namespace(), element.local_name())),
                ));
            }
            if display != "block" {
                return Err(layout_failure(
                    "unsupported_root_html_display",
                    Some(node.id().get()),
                    Some(display.to_owned()),
                ));
            }
            for (property, value) in [
                ("margin-top", style.layout_spacing.margin.top),
                ("margin-right", style.layout_spacing.margin.right),
                ("margin-bottom", style.layout_spacing.margin.bottom),
                ("margin-left", style.layout_spacing.margin.left),
            ] {
                if value != spinon_style::ComputedCssSpacingValue::LengthPx(0.0) {
                    return Err(layout_failure(
                        "unsupported_root_html_margin",
                        Some(node.id().get()),
                        Some(property.to_owned()),
                    ));
                }
            }
            for (property, value) in [
                ("width", style.layout_dimensions.width),
                ("height", style.layout_dimensions.height),
            ] {
                if !matches!(
                    value,
                    spinon_style::ComputedCssDimension::Auto
                        | spinon_style::ComputedCssDimension::LengthPx(_)
                ) {
                    return Err(layout_failure(
                        "unsupported_root_html_dimension",
                        Some(node.id().get()),
                        Some(property.to_owned()),
                    ));
                }
            }
        }
        if let Some(children) = snapshot.children(handle) {
            pending.extend(
                children
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .map(|child| (child, hidden)),
            );
        }
    }
    Ok(())
}

fn empty_runtime_render_snapshot(
    request: &WorkRequest,
) -> Result<RuntimeRenderSnapshot, RuntimeLayoutFailure> {
    use spinon_render::{CssSize, RuntimeRenderKey};

    if request.key.generation != request.snapshot.generation().get()
        || request.key.document_revision != request.snapshot.document_revision().get()
        || request.key.render_tree_revision != request.snapshot.render_tree_revision().get()
        || request.key.environment_revision != request.viewport.environment_revision.get()
    {
        return Err(layout_failure("invalid_revision", None, None));
    }
    let key = RuntimeRenderKey::new(
        request.snapshot.generation(),
        request.snapshot.document_revision(),
        request.snapshot.render_tree_revision(),
        request.key.style_revision,
        request.viewport.environment_revision,
    );
    let viewport = CssSize::new(
        request.viewport.width_css_px,
        request.viewport.height_css_px,
    )
    .map_err(|_| layout_failure("invalid_viewport", None, None))?;
    RuntimeRenderSnapshot::new(key, viewport, Vec::new())
        .map_err(|_| layout_failure("render_snapshot_failed", None, None))
}

fn runtime_frames(
    snapshot: &spinon_core::HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    output: &LayoutOutput,
) -> Result<Vec<RuntimeLayoutFrame>, RuntimeLayoutFailure> {
    let root_html_text_profile =
        styles.profile == spinon_style::ComputedStyleProfile::RuntimeRootHtmlTextPaintV1;
    let anonymous_text_item_ids = anonymous_grid_text_item_ids(styles, output);
    for node in &anonymous_text_item_ids {
        let Some(frame) = output.frames.get(node) else {
            return Err(layout_failure(
                "missing_grid_item_frame",
                Some(node.get()),
                None,
            ));
        };
        if !valid_frame(*frame) {
            return Err(layout_failure(
                "invalid_grid_item_frame",
                Some(node.get()),
                None,
            ));
        }
    }
    let styles_by_node = styles
        .elements
        .iter()
        .map(|style| (style.node_id, style))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut frames = Vec::with_capacity(output.frames.len());
    let mut pending = vec![root];
    while let Some(handle) = pending.pop() {
        let Some(node) = snapshot.node(handle) else {
            return Err(layout_failure(
                "missing_node",
                Some(handle.id().get()),
                None,
            ));
        };
        if matches!(node.kind(), spinon_core::HostNodeKind::Text(_)) {
            continue;
        }
        let style = styles_by_node.get(&handle.id()).ok_or_else(|| {
            layout_failure("missing_computed_style", Some(handle.id().get()), None)
        })?;
        let display = style.properties.get("display").map(String::as_str);
        if display == Some("none")
            && (root_html_text_profile || !output.frames.contains_key(&handle.id()))
        {
            continue;
        }
        let include_frame = if root_html_text_profile {
            match display {
                Some("inline") => false,
                Some("block") => true,
                Some(value) => {
                    return Err(layout_failure(
                        "unsupported_root_html_default_render_display",
                        Some(handle.id().get()),
                        Some(value.to_owned()),
                    ));
                }
                None => {
                    return Err(layout_failure(
                        "missing_computed_style",
                        Some(handle.id().get()),
                        Some("display".to_owned()),
                    ));
                }
            }
        } else {
            true
        };
        if include_frame {
            let Some(frame) = output.frames.get(&handle.id()) else {
                return Err(layout_failure(
                    "missing_frame",
                    Some(handle.id().get()),
                    None,
                ));
            };
            if !valid_frame(*frame) {
                return Err(layout_failure(
                    "invalid_frame",
                    Some(handle.id().get()),
                    None,
                ));
            }
            frames.push(RuntimeLayoutFrame {
                node_id: handle.id().get(),
                x: frame.x,
                y: frame.y,
                width: frame.width,
                height: frame.height,
            });
        }
        if let Some(children) = snapshot.children(handle) {
            let children = children.collect::<Vec<_>>();
            pending.extend(children.into_iter().rev());
        }
    }
    if !root_html_text_profile
        && frames.len() + anonymous_text_item_ids.len() != output.frames.len()
    {
        return Err(layout_failure("frame_set_mismatch", None, None));
    }
    Ok(frames)
}

fn anonymous_grid_text_item_ids(
    styles: &ComputedStyleSnapshot,
    output: &LayoutOutput,
) -> BTreeSet<spinon_core::NodeId> {
    let element_ids = styles
        .elements
        .iter()
        .map(|style| style.node_id)
        .collect::<BTreeSet<_>>();
    output
        .grid_item_intrinsic_inputs
        .iter()
        .filter(|input| !input.text_sources.is_empty() && !element_ids.contains(&input.node))
        .map(|input| input.node)
        .collect()
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn detach_anonymous_grid_text_item_frames(
    styles: &ComputedStyleSnapshot,
    output: &mut LayoutOutput,
) -> Result<BTreeMap<spinon_core::NodeId, LayoutFrame>, RuntimeLayoutFailure> {
    let item_ids = anonymous_grid_text_item_ids(styles, output);
    item_ids
        .into_iter()
        .map(|node| {
            let frame = output
                .frames
                .remove(&node)
                .ok_or_else(|| layout_failure("missing_grid_item_frame", Some(node.get()), None))?;
            if !valid_frame(frame) {
                return Err(layout_failure(
                    "invalid_grid_item_frame",
                    Some(node.get()),
                    None,
                ));
            }
            Ok((node, frame))
        })
        .collect()
}

fn host_document_element_ids(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
) -> Result<BTreeSet<spinon_core::NodeId>, RuntimeLayoutFailure> {
    let mut ids = BTreeSet::new();
    let mut pending = vec![root];
    while let Some(handle) = pending.pop() {
        let node = snapshot
            .node(handle)
            .ok_or_else(|| layout_failure("missing_node", Some(handle.id().get()), None))?;
        if matches!(node.kind(), spinon_core::HostNodeKind::Element(_)) {
            ids.insert(handle.id());
        }
        if let Some(children) = snapshot.children(handle) {
            pending.extend(children);
        }
    }
    Ok(ids)
}

fn valid_frame(frame: LayoutFrame) -> bool {
    frame.x.is_finite()
        && frame.y.is_finite()
        && frame.width.is_finite()
        && frame.height.is_finite()
        && frame.width >= 0.0
        && frame.height >= 0.0
}

fn compute_runtime_style_layout_for_profile(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    viewport: spinon_style::CssViewport,
    profile: super::calculation::RuntimeCalculationProfile,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    #[cfg(feature = "c11-3-3-grid-implicit-named-lines-fixture-test")]
    if profile
        == super::calculation::RuntimeCalculationProfile::BlockFlexGridTemplateImplicitNamedLines
    {
        return spinon_style_to_layout::compute_runtime_style_layout_with_implicit_named_lines(
            snapshot,
            root,
            styles.clone(),
            viewport,
            true,
        );
    }
    let _ = profile;
    compute_runtime_style_layout(snapshot, root, styles.clone(), viewport)
}

fn layout_failure_from_error(error: StyleLayoutError) -> RuntimeLayoutFailure {
    match error {
        StyleLayoutError::UnsupportedBlockDisplay { node, value } => {
            layout_failure("unsupported_block_display", Some(node.get()), Some(value))
        }
        StyleLayoutError::UnsupportedComputedValue { node, property, .. } => layout_failure(
            "unsupported_computed_value",
            Some(node.get()),
            Some(property.to_owned()),
        ),
        StyleLayoutError::UnsupportedNamedGridLine { node, property, .. } => layout_failure(
            "unsupported_named_grid_line",
            Some(node.get()),
            Some(property.to_owned()),
        ),
        StyleLayoutError::UnsupportedInlineProperty { node, property } => layout_failure(
            "unsupported_inline_property",
            Some(node.get()),
            Some(property),
        ),
        StyleLayoutError::UnsupportedRootMargin(node) => layout_failure(
            "unsupported_root_margin",
            Some(node.get()),
            Some("margin".to_owned()),
        ),
        StyleLayoutError::Layout(LayoutError::UnsupportedTextNode(node)) => {
            layout_failure("unsupported_text_node", Some(node.get()), None)
        }
        StyleLayoutError::Layout(LayoutError::TreeDepthExceeded { node, .. }) => {
            layout_failure("tree_depth_exceeded", Some(node.get()), None)
        }
        StyleLayoutError::Layout(LayoutError::InvalidHostDocumentRoot(node)) => {
            layout_failure("invalid_root", Some(node.get()), None)
        }
        StyleLayoutError::Layout(LayoutError::InvalidGridTrackList { node, property }) => {
            layout_failure(
                "invalid_grid_track_list",
                Some(node.get()),
                Some(property.to_owned()),
            )
        }
        StyleLayoutError::Layout(LayoutError::InvalidGridPlacement {
            node,
            property,
            reason,
        }) => layout_failure(
            "invalid_grid_placement",
            Some(node.get()),
            Some(format!("{property}: {reason}")),
        ),
        StyleLayoutError::Layout(LayoutError::UnsupportedGridAutoFlow(node)) => layout_failure(
            "unsupported_grid_auto_flow",
            Some(node.get()),
            Some("grid-auto-flow".to_owned()),
        ),
        StyleLayoutError::Layout(LayoutError::UnsupportedGridAutoRows { node, .. }) => {
            layout_failure(
                "unsupported_grid_auto_rows",
                Some(node.get()),
                Some("grid-auto-rows".to_owned()),
            )
        }
        StyleLayoutError::Layout(LayoutError::UnsupportedGridAutoColumns { node, .. }) => {
            layout_failure(
                "unsupported_grid_auto_columns",
                Some(node.get()),
                Some("grid-auto-columns".to_owned()),
            )
        }
        StyleLayoutError::Layout(LayoutError::GridTrackLimitExceeded {
            node,
            property,
            count,
            maximum,
        }) => layout_failure(
            "grid_track_limit_exceeded",
            Some(node.get()),
            Some(format!("{property}: {count} > {maximum}")),
        ),
        StyleLayoutError::SnapshotMismatch { .. } => {
            layout_failure("snapshot_mismatch", None, None)
        }
        other => layout_failure("layout_failed", None, Some(other.to_string())),
    }
}

fn layout_failure_from_render_error(error: StyleRenderError) -> RuntimeLayoutFailure {
    match error {
        StyleRenderError::UnsupportedComputedValue { node, property, .. } => layout_failure(
            "unsupported_computed_value",
            Some(node.get()),
            Some(property.to_owned()),
        ),
        StyleRenderError::MissingComputedProperty { node, property } => layout_failure(
            "missing_computed_property",
            Some(node.get()),
            Some(property.to_owned()),
        ),
        StyleRenderError::UnsupportedTextNode(node) => {
            layout_failure("unsupported_text_node", Some(node.get()), None)
        }
        StyleRenderError::MissingLayoutFrame(node) => {
            layout_failure("missing_layout_frame", Some(node.get()), None)
        }
        StyleRenderError::InvalidFrame(node) => {
            layout_failure("invalid_layout_frame", Some(node.get()), None)
        }
        StyleRenderError::SnapshotMismatch { field } => {
            layout_failure("snapshot_mismatch", None, Some(field.to_owned()))
        }
        StyleRenderError::AllocationFailed { stage } => {
            layout_failure("render_allocation_failed", None, Some(stage.to_owned()))
        }
        other => layout_failure("render_snapshot_failed", None, Some(other.to_string())),
    }
}

pub(super) fn layout_failure(
    code: &'static str,
    node_id: Option<u64>,
    property: Option<String>,
) -> RuntimeLayoutFailure {
    RuntimeLayoutFailure {
        code,
        node_id,
        property,
    }
}

#[cfg(test)]
mod tests {
    use super::{layout_failure_from_error, layout_failure_from_render_error, valid_frame};
    use spinon_core::NodeId;
    use spinon_layout::{LayoutError, LayoutFrame};
    use spinon_style_to_layout::StyleLayoutError;
    use spinon_style_to_render::StyleRenderError;

    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    #[test]
    fn rekeying_grid_diagnostics_preserves_computed_style_provenance() {
        use super::{
            RuntimeC11_4FitContentAutoNeighborObservation, RuntimeGridTrackSizingDiagnostics,
        };
        use spinon_core::{
            DocumentRevision, EnvironmentRevision, RenderTreeRevision, StyleRevision,
        };
        use spinon_layout::{GridContainerInput, LayoutInputRevision, LayoutSourceRevision};
        use spinon_style::{ComputedStyleProfile, ComputedStyleSnapshot, CssViewport};
        use std::sync::Arc;

        let document = spinon_core::HostDocument::new().unwrap();
        let document = document.snapshot();
        let root = NodeId::new(1).unwrap();
        let style_snapshot = ComputedStyleSnapshot {
            profile: ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
            viewport: CssViewport::C04_FIXTURE,
            style_revision: StyleRevision::INITIAL,
            generation: document.generation(),
            document_revision: document.document_revision(),
            render_tree_revision: document.render_tree_revision(),
            document_root_font_size_css_px: 16.0,
            elements: Arc::from([]),
            writing_styles: Arc::from([]),
            diagnostics: Vec::new(),
        };
        let source = LayoutSourceRevision::HostDocument {
            generation: document.generation(),
            document: document.document_revision(),
            render_tree: document.render_tree_revision(),
        };
        let original_revision =
            LayoutInputRevision::new(source, StyleRevision::INITIAL, EnvironmentRevision::INITIAL);
        let diagnostics = RuntimeGridTrackSizingDiagnostics {
            revision: original_revision,
            intrinsic_measurements: Arc::from([]),
            item_inputs: Arc::from([]),
            item_text_layouts: Arc::from([]),
            item_frames: Arc::from([]),
            traces: Arc::from([]),
            fit_content_auto_neighbor: Some(RuntimeC11_4FitContentAutoNeighborObservation {
                style_snapshot: style_snapshot.clone(),
                grid_root: root,
                grid_container_input: GridContainerInput::default(),
                node_ids: [root; 5],
            }),
        };
        let rekeyed_revision = LayoutInputRevision::new(
            LayoutSourceRevision::HostDocument {
                generation: document.generation(),
                document: DocumentRevision::default(),
                render_tree: RenderTreeRevision::default(),
            },
            StyleRevision::INITIAL.checked_next().unwrap(),
            EnvironmentRevision::INITIAL,
        );

        let rekeyed = diagnostics.with_revision(rekeyed_revision);
        let observation = rekeyed.fit_content_auto_neighbor.unwrap();
        assert_eq!(observation.style_snapshot, style_snapshot);
        assert_eq!(
            observation.style_snapshot.style_revision,
            StyleRevision::INITIAL
        );
        assert_eq!(rekeyed.revision, rekeyed_revision);
    }

    #[test]
    fn runtime_frames_reject_non_finite_coordinates_and_negative_sizes() {
        assert!(valid_frame(LayoutFrame {
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
        }));
        assert!(!valid_frame(LayoutFrame {
            x: f32::NAN,
            y: 0.0,
            width: 1.0,
            height: 1.0,
        }));
        assert!(!valid_frame(LayoutFrame {
            x: 0.0,
            y: f32::INFINITY,
            width: 1.0,
            height: 1.0,
        }));
        assert!(!valid_frame(LayoutFrame {
            x: 0.0,
            y: 0.0,
            width: -1.0,
            height: 1.0,
        }));
        assert!(!valid_frame(LayoutFrame {
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: -1.0,
        }));
    }

    #[test]
    fn render_snapshot_failure_keeps_node_and_unsupported_property_diagnostic() {
        let node = NodeId::new(42).unwrap();
        let failure =
            layout_failure_from_render_error(StyleRenderError::UnsupportedComputedValue {
                node,
                property: "visibility",
                value: "collapse".to_owned(),
            });

        assert_eq!(failure.code, "unsupported_computed_value");
        assert_eq!(failure.node_id, Some(42));
        assert_eq!(failure.property.as_deref(), Some("visibility"));
    }

    #[test]
    fn grid_auto_placement_rejections_keep_stable_codes_and_css_properties() {
        let node = NodeId::new(42).unwrap();
        let cases = [
            (
                StyleLayoutError::Layout(LayoutError::InvalidGridTrackList {
                    node,
                    property: "grid-template-columns",
                }),
                "invalid_grid_track_list",
                "grid-template-columns",
            ),
            (
                StyleLayoutError::Layout(LayoutError::UnsupportedGridAutoFlow(node)),
                "unsupported_grid_auto_flow",
                "grid-auto-flow",
            ),
            (
                StyleLayoutError::Layout(LayoutError::UnsupportedGridAutoRows {
                    node,
                    reason: "암시 행에 auto sizing이 필요합니다",
                }),
                "unsupported_grid_auto_rows",
                "grid-auto-rows",
            ),
            (
                StyleLayoutError::Layout(LayoutError::UnsupportedGridAutoColumns {
                    node,
                    reason: "암시 열에 auto sizing이 필요합니다",
                }),
                "unsupported_grid_auto_columns",
                "grid-auto-columns",
            ),
            (
                StyleLayoutError::Layout(LayoutError::GridTrackLimitExceeded {
                    node,
                    property: "grid-template-columns",
                    count: 10_001,
                    maximum: 10_000,
                }),
                "grid_track_limit_exceeded",
                "grid-template-columns: 10001 > 10000",
            ),
            (
                StyleLayoutError::UnsupportedInlineProperty {
                    node,
                    property: "grid-auto-columns".to_owned(),
                },
                "unsupported_inline_property",
                "grid-auto-columns",
            ),
            (
                StyleLayoutError::UnsupportedComputedValue {
                    node,
                    property: "grid-auto-rows",
                    value: "12px 18px".to_owned(),
                },
                "unsupported_computed_value",
                "grid-auto-rows",
            ),
            (
                StyleLayoutError::UnsupportedNamedGridLine {
                    node,
                    property: "grid-column-start",
                    name: "missing".to_owned(),
                    reason: "explicit grid line is absent",
                },
                "unsupported_named_grid_line",
                "grid-column-start",
            ),
        ];

        for (error, expected_code, expected_property) in cases {
            let failure = layout_failure_from_error(error);
            assert_eq!(failure.code, expected_code);
            assert_eq!(failure.node_id, Some(42));
            assert_eq!(failure.property.as_deref(), Some(expected_property));
        }
    }
}
