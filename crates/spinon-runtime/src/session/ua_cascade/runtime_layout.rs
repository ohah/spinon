use super::{RuntimeUaCascadeKey, WorkRequest, elapsed_microseconds};
use spinon_core::HostNodeHandle;
use spinon_layout::{LayoutError, LayoutFrame, LayoutOutput};
use spinon_style::{
    CascadeDiagnostic, ComputedStyleSnapshot, StyloDocumentView,
    first_unsupported_runtime_layout_inline_property,
};
use spinon_style_to_layout::{StyleLayoutError, compute_runtime_style_layout};
use std::sync::Arc;
use std::time::Instant;

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
    pub frames: Vec<RuntimeLayoutFrame>,
    pub projection_duration_us: u128,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeLayoutSnapshot {
    pub state: RuntimeLayoutState,
    pub requested: Option<RuntimeUaCascadeKey>,
    pub completed: Option<Arc<RuntimeLayoutCompleted>>,
    pub diagnostics: Vec<CascadeDiagnostic>,
    pub error: Option<RuntimeLayoutFailure>,
}

pub(super) type RuntimeLayoutContext = (HostNodeHandle, StyloDocumentView, ComputedStyleSnapshot);

pub(super) fn compute_runtime_layout(
    request: &WorkRequest,
    root_count: usize,
    layout_context: Option<RuntimeLayoutContext>,
) -> Result<RuntimeLayoutCompleted, RuntimeLayoutFailure> {
    if root_count == 0 {
        return Ok(RuntimeLayoutCompleted {
            key: request.key,
            frames: Vec::new(),
            projection_duration_us: 0,
        });
    }
    if root_count > 1 {
        return Err(layout_failure("multiple_host_roots", None, None));
    }

    let (root, view, styles) =
        layout_context.expect("단일 스타일 root의 layout 입력이 있어야 합니다");
    if let Some((node, property)) = first_unsupported_runtime_layout_inline_property(&view) {
        return Err(layout_failure(
            "unsupported_inline_property",
            Some(node.get()),
            Some(property),
        ));
    }
    let projection_started = Instant::now();
    let output =
        compute_runtime_style_layout(&request.snapshot, root, styles.clone(), request.viewport)
            .map_err(layout_failure_from_error)?;
    let frames = runtime_frames(&request.snapshot, root, &styles, &output.layout)?;
    Ok(RuntimeLayoutCompleted {
        key: request.key,
        frames,
        projection_duration_us: elapsed_microseconds(projection_started),
    })
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
