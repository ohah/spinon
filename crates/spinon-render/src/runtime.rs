use std::sync::Arc;

use spinon_core::{
    DocumentGeneration, DocumentRevision, EnvironmentRevision, IntrinsicSizeRevision, NodeId,
    RenderTreeRevision, ScrollStateRevision, StyleRevision,
};

use super::{CssRect, CssSize};

mod error;
mod image;
mod paint;
mod scroll;
mod scroll_gesture;
mod scroll_gesture_session;
mod scroll_input_queue;
mod snapshot;
mod text;
mod viewport;

pub use error::RuntimeRenderError;
pub use image::RuntimeImageResourceKey;
pub use image::{
    RuntimeImageObjectFit, RuntimeImageObjectPosition, RuntimeImageObjectStyle,
    RuntimeImageSampling, RuntimeRenderImage,
};
pub use paint::{RuntimeBorder, RuntimeBorderSide, RuntimePaint, RuntimeSrgbColor};

pub use scroll::{
    RuntimeOverscrollBehavior, RuntimeScrollClipLayer, RuntimeScrollContainer,
    RuntimeScrollInputPolicy, RuntimeScrollRenderMetadata,
};
pub use scroll_gesture::{RuntimeScrollDeltaChange, RuntimeScrollDeltaOutcome};
pub use scroll_gesture_session::{
    RuntimeGestureAxes, RuntimeScrollGestureBegin, RuntimeScrollGestureCancelReason,
    RuntimeScrollGestureError, RuntimeScrollGestureOutcome, RuntimeScrollGesturePanSample,
    RuntimeScrollGestureSession, RuntimeScrollGestureState, RuntimeScrollGestureTerminal,
    RuntimeTouchAction,
};
pub use scroll_input_queue::{
    RuntimeScrollGestureBeginDisposition, RuntimeScrollGestureIdentity,
    RuntimeScrollGestureInputQueue, RuntimeScrollGestureTerminalKind,
    RuntimeScrollInputQueueAdmission, RuntimeScrollInputQueueDiagnostics,
    RuntimeScrollInputQueueError, RuntimeScrollInputQueueEvent, RuntimeScrollMomentumPhase,
    RuntimeScrollWheelSample, RuntimeScrollWheelSource, RuntimeScrollWheelUnit,
};
pub use text::{RuntimeGlyphBitmap, RuntimeRenderText};
pub use viewport::RuntimePageScaleTransform;

#[cfg(test)]
#[path = "runtime/scroll_gesture_session_tests.rs"]
mod scroll_gesture_session_tests;

#[cfg(test)]
#[path = "runtime/scroll_input_queue_tests.rs"]
mod scroll_input_queue_tests;

#[cfg(test)]
#[path = "runtime/text_tests.rs"]
mod text_tests;

/// 한 CSS 좌표축에서 허용되는 반열린 clipping interval입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssAxisClip {
    min: f32,
    max: f32,
}

impl CssAxisClip {
    pub fn new(min: f32, max: f32) -> Result<Self, RuntimeRenderError> {
        if !min.is_finite() || !max.is_finite() || max < min {
            return Err(RuntimeRenderError::InvalidClip);
        }
        Ok(Self { min, max })
    }

    pub const fn min(self) -> f32 {
        self.min
    }

    pub const fn max(self) -> f32 {
        self.max
    }

    pub fn intersect(self, other: Self) -> Self {
        Self {
            min: self.min.max(other.min),
            max: self.max.min(other.max).max(self.min.max(other.min)),
        }
    }

    fn contains(self, value: f32) -> bool {
        value >= self.min && value < self.max
    }
}

/// 누적한 CSS overflow clipping bounds입니다. 생략된 축은 제한하지 않습니다.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CssClip {
    x: Option<CssAxisClip>,
    y: Option<CssAxisClip>,
}

impl CssClip {
    pub const fn unconstrained() -> Self {
        Self { x: None, y: None }
    }

    pub const fn x(self) -> Option<CssAxisClip> {
        self.x
    }

    pub const fn y(self) -> Option<CssAxisClip> {
        self.y
    }

    pub fn with_x(mut self, clip: CssAxisClip) -> Self {
        self.x = Some(self.x.map_or(clip, |current| current.intersect(clip)));
        self
    }

    pub fn with_y(mut self, clip: CssAxisClip) -> Self {
        self.y = Some(self.y.map_or(clip, |current| current.intersect(clip)));
        self
    }

    pub fn contains(self, x: f32, y: f32) -> bool {
        self.x.is_none_or(|clip| clip.contains(x)) && self.y.is_none_or(|clip| clip.contains(y))
    }

    pub fn intersect(self, other: Self) -> Self {
        let x = match (self.x, other.x) {
            (Some(left), Some(right)) => Some(left.intersect(right)),
            (Some(clip), None) | (None, Some(clip)) => Some(clip),
            (None, None) => None,
        };
        let y = match (self.y, other.y) {
            (Some(left), Some(right)) => Some(left.intersect(right)),
            (Some(clip), None) | (None, Some(clip)) => Some(clip),
            (None, None) => None,
        };
        Self { x, y }
    }

    pub fn translated(self, x_css_px: f32, y_css_px: f32) -> Result<Self, RuntimeRenderError> {
        if !x_css_px.is_finite() || !y_css_px.is_finite() {
            return Err(RuntimeRenderError::InvalidScrollState);
        }
        let x = self
            .x
            .map(|clip| CssAxisClip::new(clip.min() + x_css_px, clip.max() + x_css_px))
            .transpose()?;
        let y = self
            .y
            .map(|clip| CssAxisClip::new(clip.min() + y_css_px, clip.max() + y_css_px))
            .transpose()?;
        Ok(Self { x, y })
    }
}

/// 한 runtime 장면이 사용한 문서·스타일·환경 입력 revision입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeRenderKey {
    generation: DocumentGeneration,
    document_revision: DocumentRevision,
    render_tree_revision: RenderTreeRevision,
    style_revision: StyleRevision,
    environment_revision: EnvironmentRevision,
    intrinsic_size_revision: IntrinsicSizeRevision,
    scroll_state_revision: ScrollStateRevision,
}

impl RuntimeRenderKey {
    pub const fn new(
        generation: DocumentGeneration,
        document_revision: DocumentRevision,
        render_tree_revision: RenderTreeRevision,
        style_revision: StyleRevision,
        environment_revision: EnvironmentRevision,
    ) -> Self {
        Self {
            generation,
            document_revision,
            render_tree_revision,
            style_revision,
            environment_revision,
            intrinsic_size_revision: IntrinsicSizeRevision::INITIAL,
            scroll_state_revision: ScrollStateRevision::INITIAL,
        }
    }

    pub const fn with_intrinsic_size_revision(mut self, revision: IntrinsicSizeRevision) -> Self {
        self.intrinsic_size_revision = revision;
        self
    }

    pub const fn with_scroll_state_revision(mut self, revision: ScrollStateRevision) -> Self {
        self.scroll_state_revision = revision;
        self
    }

    pub const fn generation(self) -> DocumentGeneration {
        self.generation
    }

    pub const fn document_revision(self) -> DocumentRevision {
        self.document_revision
    }

    pub const fn render_tree_revision(self) -> RenderTreeRevision {
        self.render_tree_revision
    }

    pub const fn style_revision(self) -> StyleRevision {
        self.style_revision
    }

    pub const fn environment_revision(self) -> EnvironmentRevision {
        self.environment_revision
    }

    pub const fn intrinsic_size_revision(self) -> IntrinsicSizeRevision {
        self.intrinsic_size_revision
    }

    pub const fn scroll_state_revision(self) -> ScrollStateRevision {
        self.scroll_state_revision
    }

    pub(crate) fn matches_text_source(self, other: Self) -> bool {
        self.generation == other.generation
            && self.document_revision == other.document_revision
            && self.render_tree_revision == other.render_tree_revision
            && self.style_revision == other.style_revision
            && self.environment_revision == other.environment_revision
            && self.intrinsic_size_revision == other.intrinsic_size_revision
    }
}

/// 렌더할 수 있는 DOM 요소 하나의 불변 geometry와 paint입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RuntimeRenderBox {
    node_id: NodeId,
    frame_css_px: CssRect,
    paint: RuntimePaint,
    border: RuntimeBorder,
    paint_order: u32,
    hit_test_order: u32,
    paint_clip_css_px: CssClip,
    hit_test_clip_css_px: CssClip,
}

impl RuntimeRenderBox {
    pub const fn new(
        node_id: NodeId,
        frame_css_px: CssRect,
        paint: RuntimePaint,
        paint_order: u32,
    ) -> Self {
        Self {
            node_id,
            frame_css_px,
            paint,
            border: RuntimeBorder::none(),
            paint_order,
            hit_test_order: paint_order,
            paint_clip_css_px: CssClip::unconstrained(),
            hit_test_clip_css_px: CssClip::unconstrained(),
        }
    }

    /// Paint 순서와 독립된 CSS hit-test 우선순위를 지정합니다.
    pub const fn with_hit_test_order(mut self, order: u32) -> Self {
        self.hit_test_order = order;
        self
    }

    pub const fn with_border(mut self, border: RuntimeBorder) -> Self {
        self.border = border;
        self
    }

    pub const fn with_clip(mut self, clip_css_px: CssClip) -> Self {
        self.paint_clip_css_px = clip_css_px;
        self.hit_test_clip_css_px = clip_css_px;
        self
    }

    pub const fn with_hit_test_clip(mut self, clip_css_px: CssClip) -> Self {
        self.hit_test_clip_css_px = clip_css_px;
        self
    }

    pub const fn with_frame_css_px(mut self, frame_css_px: CssRect) -> Self {
        self.frame_css_px = frame_css_px;
        self
    }

    pub const fn node_id(self) -> NodeId {
        self.node_id
    }

    pub const fn frame_css_px(self) -> CssRect {
        self.frame_css_px
    }

    pub const fn paint(self) -> RuntimePaint {
        self.paint
    }

    pub const fn border(self) -> RuntimeBorder {
        self.border
    }

    pub const fn paint_order(self) -> u32 {
        self.paint_order
    }

    pub const fn hit_test_order(self) -> u32 {
        self.hit_test_order
    }

    pub const fn clip_css_px(self) -> CssClip {
        self.paint_clip_css_px
    }

    pub const fn hit_test_clip_css_px(self) -> CssClip {
        self.hit_test_clip_css_px
    }
}

/// 한 element scroll owner의 client scrollport, scrollable extent와 CSS px offset입니다.
/// Fixture provenance를 요구하지 않는 동적 runtime 장면입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeRenderSnapshot {
    key: RuntimeRenderKey,
    viewport_css_px: CssSize,
    page_scale_transform: Option<RuntimePageScaleTransform>,
    boxes: Arc<[RuntimeRenderBox]>,
    base_boxes: Arc<[RuntimeRenderBox]>,
    scroll_containers: Arc<[RuntimeScrollContainer]>,
    scroll_metadata: Arc<[RuntimeScrollRenderMetadata]>,
    images: Arc<[RuntimeRenderImage]>,
    texts: Arc<[RuntimeRenderText]>,
    paint_commands: Arc<[RuntimePaintCommand]>,
}

/// Runtime box paint 순서에서 opacity group 경계를 보존하는 명령입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RuntimePaintCommand {
    BeginGroup { opacity: f32 },
    DrawBox { index: usize },
    DrawText { index: usize },
    EndGroup,
}

#[cfg(test)]
#[path = "runtime/scroll_gesture_tests.rs"]
mod scroll_gesture_tests;
#[cfg(test)]
#[path = "runtime_tests.rs"]
mod tests;
