use std::{collections::BTreeSet, sync::Arc};

use spinon_core::NodeId;

#[cfg(feature = "c13-3-gesture-fixture-test")]
use super::RuntimeTouchAction;
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
    input_policy: RuntimeScrollInputPolicy,
}

/// 한 축에서 scroll remainder를 처리하는 제한된 overscroll 정책입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeOverscrollBehavior {
    Auto,
    Contain,
    None,
}

/// 한 scroll owner의 직접 입력 가능 여부와 두 물리 축 정책입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeScrollInputPolicy {
    user_scrollable_x: bool,
    user_scrollable_y: bool,
    overscroll_x: RuntimeOverscrollBehavior,
    overscroll_y: RuntimeOverscrollBehavior,
}

impl RuntimeScrollInputPolicy {
    pub const fn new(
        user_scrollable_x: bool,
        user_scrollable_y: bool,
        overscroll_x: RuntimeOverscrollBehavior,
        overscroll_y: RuntimeOverscrollBehavior,
    ) -> Self {
        Self {
            user_scrollable_x,
            user_scrollable_y,
            overscroll_x,
            overscroll_y,
        }
    }

    /// C13.2 장면은 명시적으로 C13.3 입력 정책을 연결할 때까지 직접 scroll하지 않습니다.
    pub const fn disabled() -> Self {
        Self::new(
            false,
            false,
            RuntimeOverscrollBehavior::Auto,
            RuntimeOverscrollBehavior::Auto,
        )
    }

    pub const fn user_scrollable_x(self) -> bool {
        self.user_scrollable_x
    }

    pub const fn user_scrollable_y(self) -> bool {
        self.user_scrollable_y
    }

    pub const fn overscroll_x(self) -> RuntimeOverscrollBehavior {
        self.overscroll_x
    }

    pub const fn overscroll_y(self) -> RuntimeOverscrollBehavior {
        self.overscroll_y
    }
}

/// 한 ancestor clip과 그 clip을 움직이는 조상 scroll owner 목록입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeScrollClipLayer {
    paint_clip_css_px: CssClip,
    hit_test_clip_css_px: CssClip,
    scroll_ancestors: Arc<[NodeId]>,
}

/// C12.5 제한 top-only sticky box의 immutable scroll constraint입니다.
#[cfg(feature = "c12-5-sticky-fixture-test")]
#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeStickyConstraint {
    owner: NodeId,
    containing_block_frame_css_px: CssRect,
    containing_block_scroll_ancestors: Arc<[NodeId]>,
    containing_block_content_top_inset_css_px: f32,
    containing_block_content_bottom_inset_css_px: f32,
    padding_top_css_px: f32,
    top_inset_css_px: f32,
}

#[cfg(feature = "c12-5-sticky-fixture-test")]
impl RuntimeStickyConstraint {
    pub fn new(
        owner: NodeId,
        containing_block_frame_css_px: CssRect,
        containing_block_scroll_ancestors: Vec<NodeId>,
        containing_block_content_top_inset_css_px: f32,
        containing_block_content_bottom_inset_css_px: f32,
        padding_top_css_px: f32,
        top_inset_css_px: f32,
    ) -> Result<Self, RuntimeRenderError> {
        let content_vertical_inset = containing_block_content_top_inset_css_px
            + containing_block_content_bottom_inset_css_px;
        if !containing_block_content_top_inset_css_px.is_finite()
            || containing_block_content_top_inset_css_px < 0.0
            || !containing_block_content_bottom_inset_css_px.is_finite()
            || containing_block_content_bottom_inset_css_px < 0.0
            || !content_vertical_inset.is_finite()
            || content_vertical_inset >= containing_block_frame_css_px.height()
            || !padding_top_css_px.is_finite()
            || padding_top_css_px < 0.0
            || !top_inset_css_px.is_finite()
            || top_inset_css_px < 0.0
        {
            return Err(RuntimeRenderError::InvalidScrollState);
        }
        Ok(Self {
            owner,
            containing_block_frame_css_px,
            containing_block_scroll_ancestors: containing_block_scroll_ancestors.into(),
            containing_block_content_top_inset_css_px,
            containing_block_content_bottom_inset_css_px,
            padding_top_css_px,
            top_inset_css_px,
        })
    }

    pub const fn owner(&self) -> NodeId {
        self.owner
    }

    pub const fn containing_block_frame_css_px(&self) -> CssRect {
        self.containing_block_frame_css_px
    }

    pub fn containing_block_scroll_ancestors(&self) -> &[NodeId] {
        &self.containing_block_scroll_ancestors
    }

    pub const fn containing_block_content_top_inset_css_px(&self) -> f32 {
        self.containing_block_content_top_inset_css_px
    }

    pub const fn containing_block_content_bottom_inset_css_px(&self) -> f32 {
        self.containing_block_content_bottom_inset_css_px
    }

    pub const fn padding_top_css_px(&self) -> f32 {
        self.padding_top_css_px
    }

    pub const fn top_inset_css_px(&self) -> f32 {
        self.top_inset_css_px
    }
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
    #[cfg(feature = "c12-5-sticky-fixture-test")]
    sticky_constraint: Option<RuntimeStickyConstraint>,
    #[cfg(feature = "c13-3-gesture-fixture-test")]
    touch_action_chain: Arc<[RuntimeTouchAction]>,
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
            #[cfg(feature = "c12-5-sticky-fixture-test")]
            sticky_constraint: None,
            #[cfg(feature = "c13-3-gesture-fixture-test")]
            touch_action_chain: Arc::from([]),
        }
    }

    /// C13.3 hit target부터 가장 가까운 scroll owner까지의 computed touch-action입니다.
    #[cfg(feature = "c13-3-gesture-fixture-test")]
    pub fn with_touch_action_chain(mut self, touch_action_chain: Vec<RuntimeTouchAction>) -> Self {
        self.touch_action_chain = touch_action_chain.into();
        self
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

    #[cfg(feature = "c12-5-sticky-fixture-test")]
    pub const fn sticky_constraint(&self) -> Option<&RuntimeStickyConstraint> {
        self.sticky_constraint.as_ref()
    }

    #[cfg(feature = "c12-5-sticky-fixture-test")]
    pub fn with_sticky_constraint(mut self, sticky_constraint: RuntimeStickyConstraint) -> Self {
        self.sticky_constraint = Some(sticky_constraint);
        self
    }

    #[cfg(feature = "c13-3-gesture-fixture-test")]
    pub fn touch_action_chain(&self) -> &[RuntimeTouchAction] {
        &self.touch_action_chain
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
            input_policy: RuntimeScrollInputPolicy::disabled(),
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

    pub const fn input_policy(self) -> RuntimeScrollInputPolicy {
        self.input_policy
    }

    pub const fn with_input_policy(mut self, input_policy: RuntimeScrollInputPolicy) -> Self {
        self.input_policy = input_policy;
        self
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
        updated.input_policy = self.input_policy;
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
        #[cfg(feature = "c12-5-sticky-fixture-test")]
        let mut frame = frame;
        #[cfg(feature = "c12-5-sticky-fixture-test")]
        if let Some(sticky) = metadata.sticky_constraint() {
            let nearest_owner_index = metadata
                .scroll_ancestors()
                .iter()
                .position(|owner| *owner == sticky.owner())
                .ok_or(RuntimeRenderError::InvalidScrollState)?;
            let owner = scroll_containers
                .iter()
                .find(|container| container.owner() == sticky.owner())
                .ok_or(RuntimeRenderError::InvalidScrollState)?;
            if owner.max_offset_y_css_px() <= 0.0 {
                return Err(RuntimeRenderError::InvalidScrollState);
            }
            let (_farther_x, farther_y) = scroll_offset_sum(
                &metadata.scroll_ancestors()[nearest_owner_index + 1..],
                &offsets,
            )?;
            let scrollport = owner.scrollport_css_px();
            let sticky_threshold = scrollport.y() - farther_y
                + sticky.padding_top_css_px()
                + sticky.top_inset_css_px();
            let (containing_block_x, containing_block_y) =
                scroll_offset_sum(sticky.containing_block_scroll_ancestors(), &offsets)?;
            if !containing_block_x.is_finite() {
                return Err(RuntimeRenderError::InvalidScrollState);
            }
            let containing_block = sticky.containing_block_frame_css_px();
            let containing_block_bottom = containing_block.y() + containing_block.height()
                - sticky.containing_block_content_bottom_inset_css_px()
                - containing_block_y;
            let containing_block_content_height = containing_block.height()
                - sticky.containing_block_content_top_inset_css_px()
                - sticky.containing_block_content_bottom_inset_css_px();
            let max_sticky_top = containing_block_bottom - frame.height();
            let sticky_top = (frame.y().max(sticky_threshold)).min(max_sticky_top);
            if !sticky_threshold.is_finite()
                || !containing_block_bottom.is_finite()
                || !containing_block_content_height.is_finite()
                || containing_block_content_height <= 0.0
                || frame.height() > containing_block_content_height
                || !max_sticky_top.is_finite()
                || !sticky_top.is_finite()
            {
                return Err(RuntimeRenderError::InvalidScrollState);
            }
            frame = CssRect::new(frame.x(), sticky_top, frame.width(), frame.height())
                .map_err(|_| RuntimeRenderError::InvalidScrollState)?;
        }
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

pub(super) fn scroll_offset_sum(
    ancestors: &[NodeId],
    offsets: &std::collections::BTreeMap<NodeId, (f32, f32)>,
) -> Result<(f32, f32), RuntimeRenderError> {
    let mut x = 0.0_f32;
    let mut y = 0.0_f32;
    let mut seen = BTreeSet::new();
    for ancestor in ancestors {
        if !seen.insert(*ancestor) {
            return Err(RuntimeRenderError::InvalidScrollState);
        }
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
