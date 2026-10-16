use std::{collections::BTreeSet, sync::Arc};

use spinon_core::{HostDocumentSnapshot, NodeId, ScrollStateRevision};

#[cfg(feature = "c13-3-gesture-fixture-test")]
use super::scroll::RuntimeScrollInputPolicy;
use super::scroll::{
    RuntimeScrollClipLayer, RuntimeScrollContainer, RuntimeScrollRenderMetadata, apply_scroll_state,
};
use super::{
    CssSize, RuntimePaint, RuntimeRenderBox, RuntimeRenderError, RuntimeRenderImage,
    RuntimeRenderKey, RuntimeRenderSnapshot,
};
impl RuntimeRenderSnapshot {
    pub fn new(
        key: RuntimeRenderKey,
        viewport_css_px: CssSize,
        boxes: Vec<RuntimeRenderBox>,
    ) -> Result<Self, RuntimeRenderError> {
        Self::new_with_scroll_containers(key, viewport_css_px, boxes, Vec::new())
    }

    pub fn new_with_scroll_containers(
        key: RuntimeRenderKey,
        viewport_css_px: CssSize,
        boxes: Vec<RuntimeRenderBox>,
        scroll_containers: Vec<RuntimeScrollContainer>,
    ) -> Result<Self, RuntimeRenderError> {
        let metadata = boxes
            .iter()
            .map(|render_box| {
                RuntimeScrollRenderMetadata::new(
                    render_box.frame_css_px,
                    Vec::new(),
                    vec![RuntimeScrollClipLayer::new(
                        render_box.paint_clip_css_px,
                        render_box.hit_test_clip_css_px,
                        Vec::new(),
                    )],
                )
            })
            .collect();
        Self::new_with_scroll_metadata(key, viewport_css_px, boxes, scroll_containers, metadata)
    }

    pub fn new_with_scroll_metadata(
        key: RuntimeRenderKey,
        viewport_css_px: CssSize,
        base_boxes: Vec<RuntimeRenderBox>,
        scroll_containers: Vec<RuntimeScrollContainer>,
        scroll_metadata: Vec<RuntimeScrollRenderMetadata>,
    ) -> Result<Self, RuntimeRenderError> {
        let mut node_ids = BTreeSet::new();
        let mut hit_test_orders = BTreeSet::new();
        for (index, render_box) in base_boxes.iter().enumerate() {
            if !node_ids.insert(render_box.node_id) {
                return Err(RuntimeRenderError::DuplicateNode(render_box.node_id));
            }
            if usize::try_from(render_box.paint_order).ok() != Some(index) {
                return Err(RuntimeRenderError::InvalidPaintOrder);
            }
            if usize::try_from(render_box.hit_test_order)
                .ok()
                .is_none_or(|order| order >= base_boxes.len())
                || !hit_test_orders.insert(render_box.hit_test_order)
            {
                return Err(RuntimeRenderError::InvalidHitTestOrder);
            }
        }
        if scroll_metadata.len() != base_boxes.len() {
            return Err(RuntimeRenderError::InvalidScrollState);
        }
        if base_boxes
            .iter()
            .zip(&scroll_metadata)
            .any(|(render_box, metadata)| render_box.frame_css_px != metadata.base_frame_css_px())
        {
            return Err(RuntimeRenderError::InvalidScrollState);
        }
        let mut scroll_owners = BTreeSet::new();
        for container in &scroll_containers {
            if !scroll_owners.insert(container.owner()) {
                return Err(RuntimeRenderError::DuplicateScrollOwner(container.owner()));
            }
        }
        let base_boxes: Arc<[RuntimeRenderBox]> = base_boxes.into();
        let scroll_containers: Arc<[RuntimeScrollContainer]> = scroll_containers.into();
        let scroll_metadata: Arc<[RuntimeScrollRenderMetadata]> = scroll_metadata.into();
        let boxes = apply_scroll_state(&base_boxes, &scroll_containers, &scroll_metadata)?;
        Ok(Self {
            key,
            viewport_css_px,
            boxes: boxes.into(),
            base_boxes,
            scroll_containers,
            scroll_metadata,
            images: Arc::from([]),
        })
    }

    /// Natural-size image resource를 revision과 frame 검사를 거쳐 장면에 결합합니다.
    pub fn with_images(&self, images: Vec<RuntimeRenderImage>) -> Result<Self, RuntimeRenderError> {
        let mut seen = BTreeSet::new();
        for image in &images {
            if !seen.insert(image.node_id()) {
                return Err(RuntimeRenderError::DuplicateImage(image.node_id()));
            }
            if !image.source().matches(self.key) {
                return Err(RuntimeRenderError::InvalidImageSource(image.node_id()));
            }
            if image.object_style().is_some()
                && image.object_style_revision() != Some(self.key.style_revision())
            {
                return Err(RuntimeRenderError::InvalidImageStyleRevision(
                    image.node_id(),
                ));
            }
            let Some(render_box) = self
                .boxes
                .iter()
                .find(|render_box| render_box.node_id() == image.node_id())
            else {
                return Err(RuntimeRenderError::ImageNodeMissing(image.node_id()));
            };
            let border = render_box.border();
            if [border.top(), border.right(), border.bottom(), border.left()]
                .into_iter()
                .any(|side| {
                    side.width_css_px() > 0.0 && !matches!(side.paint(), RuntimePaint::None)
                })
            {
                return Err(RuntimeRenderError::UnsupportedImageBorder(image.node_id()));
            }
            let frame = render_box.frame_css_px();
            image.object_rect_css_px(frame)?;
        }
        let mut snapshot = self.clone();
        snapshot.images = images.into();
        Ok(snapshot)
    }

    /// 같은 문서 세대와 연결 트리 revision의 snapshot에서 document revision을 다시 발행합니다.
    pub fn with_document_snapshot_revision(&self, snapshot: &HostDocumentSnapshot) -> Option<Self> {
        if self.key.generation != snapshot.generation()
            || self.key.render_tree_revision != snapshot.render_tree_revision()
            || (!self.images.is_empty()
                && self.key.document_revision != snapshot.document_revision())
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
        .with_intrinsic_size_revision(self.key.intrinsic_size_revision)
        .with_scroll_state_revision(self.key.scroll_state_revision);
        Some(Self {
            key,
            viewport_css_px: self.viewport_css_px,
            boxes: Arc::clone(&self.boxes),
            base_boxes: Arc::clone(&self.base_boxes),
            scroll_containers: Arc::clone(&self.scroll_containers),
            scroll_metadata: Arc::clone(&self.scroll_metadata),
            images: Arc::clone(&self.images),
        })
    }

    /// 같은 immutable scene의 scroll offset을 보존하면서 내부 state revision을 다시 발행합니다.
    pub fn with_scroll_state_revision(&self, revision: ScrollStateRevision) -> Self {
        Self {
            key: self.key.with_scroll_state_revision(revision),
            viewport_css_px: self.viewport_css_px,
            boxes: Arc::clone(&self.boxes),
            base_boxes: Arc::clone(&self.base_boxes),
            scroll_containers: Arc::clone(&self.scroll_containers),
            scroll_metadata: Arc::clone(&self.scroll_metadata),
            images: Arc::clone(&self.images),
        }
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

    pub fn images(&self) -> &[RuntimeRenderImage] {
        &self.images
    }

    pub fn scroll_containers(&self) -> &[RuntimeScrollContainer] {
        &self.scroll_containers
    }

    /// Fixture에서 계산된 입력 정책을 immutable snapshot에 붙입니다.
    #[cfg(feature = "c13-3-gesture-fixture-test")]
    pub fn with_scroll_input_policies(
        &self,
        policies: &[(NodeId, RuntimeScrollInputPolicy)],
    ) -> Result<Self, RuntimeRenderError> {
        let mut seen = BTreeSet::new();
        let mut containers = self.scroll_containers.to_vec();
        for (owner, policy) in policies.iter().copied() {
            if !seen.insert(owner) {
                return Err(RuntimeRenderError::DuplicateScrollRequest(owner));
            }
            let Some(container) = containers
                .iter_mut()
                .find(|container| container.owner() == owner)
            else {
                return Err(RuntimeRenderError::InvalidScrollRequest(owner));
            };
            *container = container.with_input_policy(policy);
        }
        Self::new_with_scroll_metadata(
            self.key,
            self.viewport_css_px,
            self.base_boxes.to_vec(),
            containers,
            self.scroll_metadata.to_vec(),
        )?
        .with_images(self.images.to_vec())
    }

    /// 한 hit target의 scroll owner ancestor 목록을 nearest-to-outer 순서로 반환합니다.
    /// target 자체가 owner인 경우 이 목록에는 포함되지 않으며 입력 라우터가 별도로 처리합니다.
    pub fn scroll_ancestors_for_node(&self, node: NodeId) -> Option<&[NodeId]> {
        let index = self
            .base_boxes
            .iter()
            .position(|render_box| render_box.node_id() == node)?;
        Some(self.scroll_metadata.get(index)?.scroll_ancestors())
    }

    /// Creates a new immutable render and hit-test snapshot from validated internal offsets.
    pub fn with_scroll_offsets(
        &self,
        expected_revision: ScrollStateRevision,
        updates: &[(NodeId, f32, f32)],
    ) -> Result<Self, RuntimeRenderError> {
        if expected_revision != self.key.scroll_state_revision {
            return Err(RuntimeRenderError::StaleScrollRevision);
        }
        let mut seen = BTreeSet::new();
        let mut containers = self.scroll_containers.to_vec();
        let mut changed = false;
        for (owner, requested_x, requested_y) in updates.iter().copied() {
            if !seen.insert(owner) {
                return Err(RuntimeRenderError::DuplicateScrollRequest(owner));
            }
            if !requested_x.is_finite() || !requested_y.is_finite() {
                return Err(RuntimeRenderError::InvalidScrollRequest(owner));
            }
            let Some(container) = containers
                .iter_mut()
                .find(|container| container.owner() == owner)
            else {
                return Err(RuntimeRenderError::InvalidScrollRequest(owner));
            };
            let next_x = requested_x.clamp(0.0, container.max_offset_x_css_px());
            let next_y = requested_y.clamp(0.0, container.max_offset_y_css_px());
            if next_x != container.offset_x_css_px() || next_y != container.offset_y_css_px() {
                changed = true;
                *container = container.with_offsets(next_x, next_y)?;
            }
        }
        let scroll_revision = if changed {
            self.key
                .scroll_state_revision
                .checked_next()
                .ok_or(RuntimeRenderError::ScrollRevisionExhausted)?
        } else {
            self.key.scroll_state_revision
        };
        let key = self.key.with_scroll_state_revision(scroll_revision);
        Self::new_with_scroll_metadata(
            key,
            self.viewport_css_px,
            self.base_boxes.to_vec(),
            containers,
            self.scroll_metadata.to_vec(),
        )?
        .with_images(self.images.to_vec())
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
