use std::{collections::BTreeSet, error::Error, fmt};

use spinon_core::{
    DocumentGeneration, DocumentRevision, EnvironmentRevision, NodeId, RenderTreeRevision,
    StyleRevision,
};

use super::{CssRect, CssSize, OpaqueCssSrgb};

/// 한 runtime 장면이 사용한 문서·스타일·환경 입력 revision입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeRenderKey {
    generation: DocumentGeneration,
    document_revision: DocumentRevision,
    render_tree_revision: RenderTreeRevision,
    style_revision: StyleRevision,
    environment_revision: EnvironmentRevision,
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
        }
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
}

/// 그리지 않거나 완전 불투명한 sRGB 단색으로 채우는 배경 paint입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimePaint {
    None,
    Opaque(OpaqueCssSrgb),
}

/// 렌더할 수 있는 DOM 요소 하나의 불변 geometry와 paint입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RuntimeRenderBox {
    node_id: NodeId,
    frame_css_px: CssRect,
    paint: RuntimePaint,
    paint_order: u32,
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
            paint_order,
        }
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

    pub const fn paint_order(self) -> u32 {
        self.paint_order
    }
}

/// Fixture provenance를 요구하지 않는 동적 runtime 장면입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeRenderSnapshot {
    key: RuntimeRenderKey,
    viewport_css_px: CssSize,
    boxes: Vec<RuntimeRenderBox>,
}

impl RuntimeRenderSnapshot {
    pub fn new(
        key: RuntimeRenderKey,
        viewport_css_px: CssSize,
        boxes: Vec<RuntimeRenderBox>,
    ) -> Result<Self, RuntimeRenderError> {
        let mut node_ids = BTreeSet::new();
        for (index, render_box) in boxes.iter().enumerate() {
            if !node_ids.insert(render_box.node_id) {
                return Err(RuntimeRenderError::DuplicateNode(render_box.node_id));
            }
            if usize::try_from(render_box.paint_order).ok() != Some(index) {
                return Err(RuntimeRenderError::InvalidPaintOrder);
            }
        }
        Ok(Self {
            key,
            viewport_css_px,
            boxes,
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
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeRenderError {
    DuplicateNode(NodeId),
    InvalidPaintOrder,
}

impl fmt::Display for RuntimeRenderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateNode(node) => write!(formatter, "runtime 렌더 노드 {node}가 중복됩니다"),
            Self::InvalidPaintOrder => {
                formatter.write_str("runtime paint_order가 DOM preorder의 0부터 연속값이 아닙니다")
            }
        }
    }
}

impl Error for RuntimeRenderError {}

#[cfg(test)]
#[path = "runtime_tests.rs"]
mod tests;
