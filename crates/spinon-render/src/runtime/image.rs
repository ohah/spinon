use std::sync::Arc;

use spinon_core::{
    DocumentGeneration, DocumentRevision, IntrinsicSizeRevision, NodeId, RenderTreeRevision,
};

use super::{RuntimeRenderError, RuntimeRenderKey};

/// 디코드 이미지 자원을 요청한 문서·연결 트리·intrinsic-size revision입니다.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RuntimeImageResourceKey {
    generation: DocumentGeneration,
    document_revision: DocumentRevision,
    render_tree_revision: RenderTreeRevision,
    intrinsic_size_revision: IntrinsicSizeRevision,
}

impl RuntimeImageResourceKey {
    pub const fn new(
        generation: DocumentGeneration,
        document_revision: DocumentRevision,
        render_tree_revision: RenderTreeRevision,
        intrinsic_size_revision: IntrinsicSizeRevision,
    ) -> Self {
        Self {
            generation,
            document_revision,
            render_tree_revision,
            intrinsic_size_revision,
        }
    }

    pub const fn matches(self, key: RuntimeRenderKey) -> bool {
        self.generation.get() == key.generation().get()
            && self.document_revision.get() == key.document_revision().get()
            && self.render_tree_revision.get() == key.render_tree_revision().get()
            && self.intrinsic_size_revision.get() == key.intrinsic_size_revision().get()
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

    pub const fn intrinsic_size_revision(self) -> IntrinsicSizeRevision {
        self.intrinsic_size_revision
    }
}

/// 자연 크기 RGBA8 픽셀을 source revision에 묶어 전달합니다.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeRenderImage {
    node_id: NodeId,
    source: RuntimeImageResourceKey,
    width: u32,
    height: u32,
    rgba8: Arc<[u8]>,
}

impl RuntimeRenderImage {
    pub fn new(
        node_id: NodeId,
        source: RuntimeImageResourceKey,
        width: u32,
        height: u32,
        rgba8: Arc<[u8]>,
    ) -> Result<Self, RuntimeRenderError> {
        let expected_bytes = width
            .checked_mul(height)
            .and_then(|pixels| pixels.checked_mul(4))
            .and_then(|bytes| usize::try_from(bytes).ok());
        if width == 0 || height == 0 || expected_bytes != Some(rgba8.len()) {
            return Err(RuntimeRenderError::InvalidImageBytes(node_id));
        }
        Ok(Self {
            node_id,
            source,
            width,
            height,
            rgba8,
        })
    }

    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub const fn source(&self) -> RuntimeImageResourceKey {
        self.source
    }

    pub const fn width(&self) -> u32 {
        self.width
    }

    pub const fn height(&self) -> u32 {
        self.height
    }

    pub fn rgba8(&self) -> &[u8] {
        &self.rgba8
    }
}
