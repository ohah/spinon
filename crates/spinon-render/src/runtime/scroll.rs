use std::sync::Arc;

use spinon_core::NodeId;

use super::{CssClip, CssRect, RuntimeRenderBox, RuntimeRenderError};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RuntimeScrollContainer {
    owner: NodeId,
    parent: Option<NodeId>,
    scrollport_css_px: CssRect,
    scrollable_width_css_px: f32,
    scrollable_height_css_px: f32,
    offset_x_css_px: f32,
    offset_y_css_px: f32,
}

/// 한 ancestor clip과 그 clip을 움직이는 조상 scroll owner 목록입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeScrollClipLayer {
    paint_clip_css_px: CssClip,
    hit_test_clip_css_px: CssClip,
    scroll_ancestors: Arc<[NodeId]>,
}

impl RuntimeScrollClipLayer {
    pub fn new(
        paint_clip_css_px: CssClip,
        hit_test_clip_css_px: CssClip,
        scroll_ancestors: Vec<NodeId>,
    ) -> Self {
        Self {
            paint_clip_css_px,
            hit_test_clip_css_px,
            scroll_ancestors: scroll_ancestors.into(),
        }
    }

    pub const fn paint_clip_css_px(&self) -> CssClip {
        self.paint_clip_css_px
    }

    pub const fn hit_test_clip_css_px(&self) -> CssClip {
        self.hit_test_clip_css_px
    }

    pub fn scroll_ancestors(&self) -> &[NodeId] {
        &self.scroll_ancestors
    }
}

/// 원본 frame과 조상 clip을 보존해 새 immutable scroll snapshot을 만들 때 재사용합니다.
#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeScrollRenderMetadata {
    base_frame_css_px: CssRect,
    scroll_ancestors: Arc<[NodeId]>,
    clip_layers: Arc<[RuntimeScrollClipLayer]>,
}

impl RuntimeScrollRenderMetadata {
    pub fn new(
        base_frame_css_px: CssRect,
        scroll_ancestors: Vec<NodeId>,
        clip_layers: Vec<RuntimeScrollClipLayer>,
    ) -> Self {
        Self {
            base_frame_css_px,
            scroll_ancestors: scroll_ancestors.into(),
            clip_layers: clip_layers.into(),
        }
    }

    pub const fn base_frame_css_px(&self) -> CssRect {
        self.base_frame_css_px
    }

    pub fn scroll_ancestors(&self) -> &[NodeId] {
        &self.scroll_ancestors
    }

    pub fn clip_layers(&self) -> &[RuntimeScrollClipLayer] {
        &self.clip_layers
    }
}

impl RuntimeScrollContainer {
    pub fn new(
        owner: NodeId,
        scrollport_css_px: CssRect,
        scrollable_width_css_px: f32,
        scrollable_height_css_px: f32,
        offset_x_css_px: f32,
        offset_y_css_px: f32,
    ) -> Result<Self, RuntimeRenderError> {
        let max_x = (scrollable_width_css_px - scrollport_css_px.width()).max(0.0);
        let max_y = (scrollable_height_css_px - scrollport_css_px.height()).max(0.0);
        if !scrollable_width_css_px.is_finite()
            || !scrollable_height_css_px.is_finite()
            || scrollable_width_css_px < 0.0
            || scrollable_height_css_px < 0.0
            || scrollable_width_css_px < scrollport_css_px.width()
            || scrollable_height_css_px < scrollport_css_px.height()
            || !offset_x_css_px.is_finite()
            || !offset_y_css_px.is_finite()
            || offset_x_css_px < 0.0
            || offset_y_css_px < 0.0
            || offset_x_css_px > max_x
            || offset_y_css_px > max_y
        {
            return Err(RuntimeRenderError::InvalidScrollState);
        }
        Ok(Self {
            owner,
            parent: None,
            scrollport_css_px,
            scrollable_width_css_px,
            scrollable_height_css_px,
            offset_x_css_px,
            offset_y_css_px,
        })
    }

    pub const fn owner(self) -> NodeId {
        self.owner
    }

    pub const fn parent(self) -> Option<NodeId> {
        self.parent
    }

    pub const fn with_parent(mut self, parent: Option<NodeId>) -> Self {
        self.parent = parent;
        self
    }

    pub const fn scrollport_css_px(self) -> CssRect {
        self.scrollport_css_px
    }

    pub const fn scrollable_width_css_px(self) -> f32 {
        self.scrollable_width_css_px
    }

    pub const fn scrollable_height_css_px(self) -> f32 {
        self.scrollable_height_css_px
    }

    pub const fn offset_x_css_px(self) -> f32 {
        self.offset_x_css_px
    }

    pub const fn offset_y_css_px(self) -> f32 {
        self.offset_y_css_px
    }

    pub fn with_offsets(
        self,
        offset_x_css_px: f32,
        offset_y_css_px: f32,
    ) -> Result<Self, RuntimeRenderError> {
        let mut updated = Self::new(
            self.owner,
            self.scrollport_css_px,
            self.scrollable_width_css_px,
            self.scrollable_height_css_px,
            offset_x_css_px,
            offset_y_css_px,
        )?;
        updated.parent = self.parent;
        Ok(updated)
    }

    pub fn max_offset_x_css_px(self) -> f32 {
        (self.scrollable_width_css_px - self.scrollport_css_px.width()).max(0.0)
    }

    pub fn max_offset_y_css_px(self) -> f32 {
        (self.scrollable_height_css_px - self.scrollport_css_px.height()).max(0.0)
    }
}

pub(super) fn apply_scroll_state(
    base_boxes: &[RuntimeRenderBox],
    scroll_containers: &[RuntimeScrollContainer],
    metadata: &[RuntimeScrollRenderMetadata],
) -> Result<Vec<RuntimeRenderBox>, RuntimeRenderError> {
    let offsets = scroll_containers
        .iter()
        .map(|container| {
            (
                container.owner(),
                (container.offset_x_css_px(), container.offset_y_css_px()),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut boxes = Vec::with_capacity(base_boxes.len());
    for (base_box, metadata) in base_boxes.iter().zip(metadata) {
        let (scroll_x, scroll_y) = scroll_offset_sum(metadata.scroll_ancestors(), &offsets)?;
        let base_frame = metadata.base_frame_css_px();
        let frame = CssRect::new(
            base_frame.x() - scroll_x,
            base_frame.y() - scroll_y,
            base_frame.width(),
            base_frame.height(),
        )
        .map_err(|_| RuntimeRenderError::InvalidScrollState)?;
        let mut paint_clip = CssClip::unconstrained();
        let mut hit_test_clip = CssClip::unconstrained();
        for layer in metadata.clip_layers() {
            let (clip_x, clip_y) = scroll_offset_sum(layer.scroll_ancestors(), &offsets)?;
            paint_clip =
                paint_clip.intersect(layer.paint_clip_css_px().translated(-clip_x, -clip_y)?);
            hit_test_clip =
                hit_test_clip.intersect(layer.hit_test_clip_css_px().translated(-clip_x, -clip_y)?);
        }
        let mut render_box = *base_box;
        render_box.frame_css_px = frame;
        render_box.paint_clip_css_px = paint_clip;
        render_box.hit_test_clip_css_px = hit_test_clip;
        boxes.push(render_box);
    }
    Ok(boxes)
}

fn scroll_offset_sum(
    ancestors: &[NodeId],
    offsets: &std::collections::BTreeMap<NodeId, (f32, f32)>,
) -> Result<(f32, f32), RuntimeRenderError> {
    let mut x = 0.0_f32;
    let mut y = 0.0_f32;
    for ancestor in ancestors {
        let Some((offset_x, offset_y)) = offsets.get(ancestor).copied() else {
            return Err(RuntimeRenderError::InvalidScrollState);
        };
        x += offset_x;
        y += offset_y;
    }
    if !x.is_finite() || !y.is_finite() {
        return Err(RuntimeRenderError::InvalidScrollState);
    }
    Ok((x, y))
}
