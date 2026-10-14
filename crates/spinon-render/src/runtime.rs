use std::sync::Arc;
use std::{collections::BTreeSet, error::Error, fmt};

use spinon_core::{
    DocumentGeneration, DocumentRevision, EnvironmentRevision, HostDocumentSnapshot,
    IntrinsicSizeRevision, NodeId, RenderTreeRevision, StyleRevision,
};

use super::{CssRect, CssSize, OpaqueCssSrgb};

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
        }
    }

    pub const fn with_intrinsic_size_revision(mut self, revision: IntrinsicSizeRevision) -> Self {
        self.intrinsic_size_revision = revision;
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
}

/// 그리지 않거나 완전 불투명한 sRGB 단색으로 채우는 배경 paint입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimePaint {
    None,
    Opaque(OpaqueCssSrgb),
}

/// 단색으로 그릴 한 물리 방향 border 변입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RuntimeBorderSide {
    width_css_px: f32,
    paint: RuntimePaint,
}

impl RuntimeBorderSide {
    pub fn new(width_css_px: f32, paint: RuntimePaint) -> Result<Self, RuntimeRenderError> {
        if !width_css_px.is_finite() || width_css_px < 0.0 {
            return Err(RuntimeRenderError::InvalidBorderWidth);
        }
        Ok(Self {
            width_css_px,
            paint,
        })
    }

    pub const fn none() -> Self {
        Self {
            width_css_px: 0.0,
            paint: RuntimePaint::None,
        }
    }

    pub const fn width_css_px(self) -> f32 {
        self.width_css_px
    }

    pub const fn paint(self) -> RuntimePaint {
        self.paint
    }
}

/// 네 면의 단색 CSS border geometry와 paint입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RuntimeBorder {
    top: RuntimeBorderSide,
    right: RuntimeBorderSide,
    bottom: RuntimeBorderSide,
    left: RuntimeBorderSide,
}

impl RuntimeBorder {
    pub const fn none() -> Self {
        Self {
            top: RuntimeBorderSide::none(),
            right: RuntimeBorderSide::none(),
            bottom: RuntimeBorderSide::none(),
            left: RuntimeBorderSide::none(),
        }
    }

    pub const fn new(
        top: RuntimeBorderSide,
        right: RuntimeBorderSide,
        bottom: RuntimeBorderSide,
        left: RuntimeBorderSide,
    ) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub const fn top(self) -> RuntimeBorderSide {
        self.top
    }

    pub const fn right(self) -> RuntimeBorderSide {
        self.right
    }

    pub const fn bottom(self) -> RuntimeBorderSide {
        self.bottom
    }

    pub const fn left(self) -> RuntimeBorderSide {
        self.left
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

/// Fixture provenance를 요구하지 않는 동적 runtime 장면입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeRenderSnapshot {
    key: RuntimeRenderKey,
    viewport_css_px: CssSize,
    boxes: Arc<[RuntimeRenderBox]>,
}

impl RuntimeRenderSnapshot {
    pub fn new(
        key: RuntimeRenderKey,
        viewport_css_px: CssSize,
        boxes: Vec<RuntimeRenderBox>,
    ) -> Result<Self, RuntimeRenderError> {
        let mut node_ids = BTreeSet::new();
        let mut hit_test_orders = BTreeSet::new();
        for (index, render_box) in boxes.iter().enumerate() {
            if !node_ids.insert(render_box.node_id) {
                return Err(RuntimeRenderError::DuplicateNode(render_box.node_id));
            }
            if usize::try_from(render_box.paint_order).ok() != Some(index) {
                return Err(RuntimeRenderError::InvalidPaintOrder);
            }
            if usize::try_from(render_box.hit_test_order)
                .ok()
                .is_none_or(|order| order >= boxes.len())
                || !hit_test_orders.insert(render_box.hit_test_order)
            {
                return Err(RuntimeRenderError::InvalidHitTestOrder);
            }
        }
        Ok(Self {
            key,
            viewport_css_px,
            boxes: boxes.into(),
        })
    }

    /// 같은 문서 세대와 연결 트리 revision의 snapshot에서 document revision을 다시 발행합니다.
    pub fn with_document_snapshot_revision(&self, snapshot: &HostDocumentSnapshot) -> Option<Self> {
        if self.key.generation != snapshot.generation()
            || self.key.render_tree_revision != snapshot.render_tree_revision()
        {
            return None;
        }
        let key = RuntimeRenderKey::new(
            self.key.generation,
            snapshot.document_revision(),
            self.key.render_tree_revision,
            self.key.style_revision,
            self.key.environment_revision,
        )
        .with_intrinsic_size_revision(self.key.intrinsic_size_revision);
        Some(Self {
            key,
            viewport_css_px: self.viewport_css_px,
            boxes: Arc::clone(&self.boxes),
        })
    }

    pub const fn key(&self) -> RuntimeRenderKey {
        self.key
    }

    pub const fn viewport_css_px(&self) -> CssSize {
        self.viewport_css_px
    }

    pub fn boxes(&self) -> &[RuntimeRenderBox] {
        &self.boxes
    }

    /// hit-test 순서, clip, viewport를 통과한 가장 앞의 CSS event target을 반환합니다.
    pub fn hit_test_css_point(&self, x: f32, y: f32) -> Option<&RuntimeRenderBox> {
        if !x.is_finite()
            || !y.is_finite()
            || x < 0.0
            || x >= self.viewport_css_px.width()
            || y < 0.0
            || y >= self.viewport_css_px.height()
        {
            return None;
        }
        self.boxes
            .iter()
            .filter(|render_box| {
                let frame = render_box.frame_css_px();
                x >= frame.x()
                    && x < frame.x() + frame.width()
                    && y >= frame.y()
                    && y < frame.y() + frame.height()
                    && render_box.hit_test_clip_css_px().contains(x, y)
            })
            .max_by_key(|render_box| render_box.hit_test_order())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeRenderError {
    DuplicateNode(NodeId),
    InvalidPaintOrder,
    InvalidHitTestOrder,
    InvalidClip,
    InvalidBorderWidth,
}

impl fmt::Display for RuntimeRenderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateNode(node) => write!(formatter, "runtime 렌더 노드 {node}가 중복됩니다"),
            Self::InvalidPaintOrder => {
                formatter.write_str("runtime paint_order가 장면 순서의 0부터 연속값이 아닙니다")
            }
            Self::InvalidHitTestOrder => {
                formatter.write_str("runtime hit_test_order가 중복되거나 장면 범위를 벗어났습니다")
            }
            Self::InvalidClip => {
                formatter.write_str("runtime clip은 유한하고 순서가 맞는 경계여야 합니다")
            }
            Self::InvalidBorderWidth => {
                formatter.write_str("runtime border 폭은 유한한 0 이상의 CSS px여야 합니다")
            }
        }
    }
}

impl Error for RuntimeRenderError {}

#[cfg(test)]
#[path = "runtime_tests.rs"]
mod tests;
