use super::{
    RuntimeUaCascadeHandle, RuntimeUaCascadeKey, State, WorkRequest, elapsed_microseconds, lock,
};
use spinon_core::HostNodeHandle;
use spinon_layout::{LayoutError, LayoutFrame, LayoutOutput};
use spinon_render::RuntimeRenderSnapshot;
use spinon_style::{
    CascadeDiagnostic, ComputedStyleSnapshot, StyloDocumentView,
    first_unsupported_runtime_block_paint_inline_property,
    first_unsupported_runtime_custom_properties_inline_property,
    first_unsupported_runtime_custom_properties_paint_inline_property,
};
use spinon_style_to_layout::{StyleLayoutError, compute_runtime_style_layout};
use spinon_style_to_render::{CurrentLayoutInputs, build_runtime_render_snapshot};
use std::sync::Arc;
use std::time::{Duration, Instant};

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

pub(super) fn compute_runtime_layout(
    request: &WorkRequest,
    root_count: usize,
    layout_context: Option<RuntimeLayoutContext>,
    runtime_paint_enabled: bool,
    profile: super::calculation::RuntimeCalculationProfile,
) -> Result<RuntimeLayoutCompleted, RuntimeLayoutFailure> {
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
        });
    }
    if root_count > 1 {
        return Err(layout_failure("multiple_host_roots", None, None));
    }

    let (root, view, styles) =
        layout_context.expect("단일 스타일 root의 layout 입력이 있어야 합니다");
    let unsupported_property = match profile {
        super::calculation::RuntimeCalculationProfile::BlockPaint => {
            first_unsupported_runtime_block_paint_inline_property(&view)
        }
        super::calculation::RuntimeCalculationProfile::FlexPaint
        | super::calculation::RuntimeCalculationProfile::RegisteredPropertiesPaint => {
            first_unsupported_runtime_custom_properties_paint_inline_property(&view)
        }
        super::calculation::RuntimeCalculationProfile::FlexLayout => {
            first_unsupported_runtime_custom_properties_inline_property(&view)
        }
    };
    if let Some((node, property)) = unsupported_property {
        return Err(layout_failure(
            "unsupported_inline_property",
            Some(node.get()),
            Some(property),
        ));
    }
    let projection_started = Instant::now();
    if runtime_paint_enabled && !styles.diagnostics.is_empty() {
        return Err(layout_failure("cascade_diagnostics", None, None));
    }
    let output =
        compute_runtime_style_layout(&request.snapshot, root, styles.clone(), request.viewport)
            .map_err(layout_failure_from_error)?;
    let frames = runtime_frames(&request.snapshot, root, &styles, &output.layout)?;
    let render_snapshot = if runtime_paint_enabled {
        let current = CurrentLayoutInputs::for_host_document(
            &request.snapshot,
            styles.style_revision,
            request.viewport,
        );
        Some(
            build_runtime_render_snapshot(&request.snapshot, root, &output, current)
                .map_err(|_| layout_failure("render_snapshot_failed", None, None))?,
        )
    } else {
        None
    };
    Ok(RuntimeLayoutCompleted {
        key: request.key,
        frames: frames.into(),
        render_snapshot,
        projection_duration_us: elapsed_microseconds(projection_started),
        cache_hit: false,
    })
}

fn empty_runtime_render_snapshot(
    request: &WorkRequest,
) -> Result<RuntimeRenderSnapshot, RuntimeLayoutFailure> {
    use spinon_render::{CssSize, RuntimeRenderKey};

    if request.key.generation != request.snapshot.generation().get()
        || request.key.document_revision != request.snapshot.document_revision().get()
        || request.key.render_tree_revision != request.snapshot.render_tree_revision().get()
        || request.key.style_revision != spinon_core::StyleRevision::INITIAL.get()
        || request.key.environment_revision != request.viewport.environment_revision.get()
    {
        return Err(layout_failure("invalid_revision", None, None));
    }
    let key = RuntimeRenderKey::new(
        request.snapshot.generation(),
        request.snapshot.document_revision(),
        request.snapshot.render_tree_revision(),
        spinon_core::StyleRevision::INITIAL,
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
    if output.frames.len() != styles.elements.len() {
        return Err(layout_failure("frame_set_mismatch", None, None));
    }
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
        if let Some(children) = snapshot.children(handle) {
            let children = children.collect::<Vec<_>>();
            pending.extend(children.into_iter().rev());
        }
    }
    if frames.len() != output.frames.len() {
        return Err(layout_failure("frame_set_mismatch", None, None));
    }
    Ok(frames)
}

fn valid_frame(frame: LayoutFrame) -> bool {
    frame.x.is_finite()
        && frame.y.is_finite()
        && frame.width.is_finite()
        && frame.height.is_finite()
        && frame.width >= 0.0
        && frame.height >= 0.0
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
        StyleLayoutError::UnsupportedRootMargin(node) => layout_failure(
            "unsupported_root_margin",
            Some(node.get()),
            Some("margin".to_owned()),
        ),
        StyleLayoutError::Layout(LayoutError::UnsupportedTextNode(node)) => {
            layout_failure("unsupported_text_node", Some(node.get()), None)
        }
        StyleLayoutError::Layout(LayoutError::InvalidHostDocumentRoot(node)) => {
            layout_failure("invalid_root", Some(node.get()), None)
        }
        StyleLayoutError::SnapshotMismatch { .. } => {
            layout_failure("snapshot_mismatch", None, None)
        }
        _ => layout_failure("layout_failed", None, None),
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
    use super::valid_frame;
    use spinon_layout::LayoutFrame;

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
}
