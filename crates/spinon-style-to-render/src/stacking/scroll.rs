use std::collections::BTreeMap;

use spinon_core::{
    DocumentGeneration, HostDocumentSnapshot, HostNodeHandle, NodeId, ScrollStateRevision,
};
use spinon_render::{RuntimeRenderSnapshot, RuntimeScrollContainer};
use spinon_style::{ComputedCssPosition, ComputedElementStyle, CssViewport};
use spinon_style_to_layout::StyleLayoutOutput;

use super::reserve::FallibleReserve;
use crate::StyleRenderError;

mod geometry;
pub(super) use geometry::runtime_scroll_containers;

pub(super) struct RuntimeScrollGeometryContext<'a> {
    pub(super) document: &'a HostDocumentSnapshot,
    pub(super) root: HostNodeHandle,
    pub(super) document_nodes: &'a [NodeId],
    pub(super) parent_by_node: &'a BTreeMap<NodeId, NodeId>,
    pub(super) styles: &'a BTreeMap<NodeId, &'a ComputedElementStyle>,
    pub(super) output: &'a StyleLayoutOutput,
    pub(super) viewport: CssViewport,
}

pub(super) struct RuntimeScrollSnapshotState {
    pub(super) containers: Vec<RuntimeScrollContainer>,
    pub(super) revision: ScrollStateRevision,
    pub(super) offsets: BTreeMap<NodeId, (f32, f32)>,
}

/// Internal fixture request to set one owner's scroll offset against a known state revision.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RuntimeScrollOffsetRequest {
    owner: NodeId,
    expected_revision: ScrollStateRevision,
    offset_x_css_px: f32,
    offset_y_css_px: f32,
}

pub(super) fn reconcile_scroll_state<R: FallibleReserve>(
    context: Option<&RuntimeScrollGeometryContext<'_>>,
    previous: Option<&RuntimeRenderSnapshot>,
    requests: &[RuntimeScrollOffsetRequest],
    generation: DocumentGeneration,
    reserve: &mut R,
) -> Result<RuntimeScrollSnapshotState, StyleRenderError> {
    let candidates = match context {
        Some(context) => runtime_scroll_containers(context, reserve)?,
        None if !requests.is_empty() => return Err(StyleRenderError::UnsupportedRuntimeProfile),
        None => Vec::new(),
    };
    let (containers, revision) =
        reconcile_scroll_containers(candidates, previous, requests, generation)?;
    let offsets = containers
        .iter()
        .map(|container| {
            (
                container.owner(),
                (container.offset_x_css_px(), container.offset_y_css_px()),
            )
        })
        .collect();
    Ok(RuntimeScrollSnapshotState {
        containers,
        revision,
        offsets,
    })
}

impl RuntimeScrollOffsetRequest {
    pub const fn new(
        owner: NodeId,
        expected_revision: ScrollStateRevision,
        offset_x_css_px: f32,
        offset_y_css_px: f32,
    ) -> Self {
        Self {
            owner,
            expected_revision,
            offset_x_css_px,
            offset_y_css_px,
        }
    }

    pub const fn owner(self) -> NodeId {
        self.owner
    }

    pub const fn expected_revision(self) -> ScrollStateRevision {
        self.expected_revision
    }

    pub const fn offset_x_css_px(self) -> f32 {
        self.offset_x_css_px
    }

    pub const fn offset_y_css_px(self) -> f32 {
        self.offset_y_css_px
    }
}

pub(super) fn reconcile_scroll_containers(
    mut current: Vec<RuntimeScrollContainer>,
    previous: Option<&RuntimeRenderSnapshot>,
    requests: &[RuntimeScrollOffsetRequest],
    generation: spinon_core::DocumentGeneration,
) -> Result<(Vec<RuntimeScrollContainer>, ScrollStateRevision), StyleRenderError> {
    if previous.is_some_and(|snapshot| snapshot.key().generation() != generation) {
        return Err(StyleRenderError::StaleScrollState);
    }
    let base_revision = previous.map_or(ScrollStateRevision::INITIAL, |snapshot| {
        snapshot.key().scroll_state_revision()
    });
    if requests
        .iter()
        .any(|request| request.expected_revision() != base_revision)
    {
        return Err(StyleRenderError::StaleScrollState);
    }

    let mut requested_owners = std::collections::BTreeSet::new();
    for request in requests {
        if !requested_owners.insert(request.owner()) {
            return Err(StyleRenderError::DuplicateScrollRequest(request.owner()));
        }
        if !request.offset_x_css_px().is_finite() || !request.offset_y_css_px().is_finite() {
            return Err(StyleRenderError::InvalidScrollRequest(request.owner()));
        }
    }

    let previous_containers = previous.map_or(&[][..], RuntimeRenderSnapshot::scroll_containers);
    let mut changed = previous_containers.len() != current.len();
    for container in &mut current {
        let previous_container = previous_containers
            .iter()
            .find(|previous| previous.owner() == container.owner());
        let mut offset_x = 0.0;
        let mut offset_y = 0.0;
        if let Some(previous) = previous_container {
            if !same_scroll_geometry(previous, container) {
                changed = true;
            }
            if previous.parent() == container.parent() {
                offset_x = previous
                    .offset_x_css_px()
                    .min(container.max_offset_x_css_px());
                offset_y = previous
                    .offset_y_css_px()
                    .min(container.max_offset_y_css_px());
            }
        } else if previous.is_some() {
            changed = true;
        }
        if let Some(request) = requests
            .iter()
            .find(|request| request.owner() == container.owner())
        {
            offset_x = request
                .offset_x_css_px()
                .clamp(0.0, container.max_offset_x_css_px());
            offset_y = request
                .offset_y_css_px()
                .clamp(0.0, container.max_offset_y_css_px());
        }
        let updated = container.with_offsets(offset_x, offset_y)?;
        if let Some(previous) = previous_container {
            if previous.offset_x_css_px() != updated.offset_x_css_px()
                || previous.offset_y_css_px() != updated.offset_y_css_px()
            {
                changed = true;
            }
        } else if updated.offset_x_css_px() != 0.0 || updated.offset_y_css_px() != 0.0 {
            changed = true;
        }
        *container = updated;
    }

    if let Some(request) = requests.iter().find(|request| {
        !current
            .iter()
            .any(|container| container.owner() == request.owner())
    }) {
        return Err(StyleRenderError::InvalidScrollRequest(request.owner()));
    }

    if previous.is_some() {
        changed |= previous_containers.iter().any(|previous| {
            !current
                .iter()
                .any(|container| container.owner() == previous.owner())
        });
    } else {
        changed = current.iter().any(|container| {
            container.offset_x_css_px() != 0.0 || container.offset_y_css_px() != 0.0
        });
    }

    let revision = if changed {
        base_revision
            .checked_next()
            .ok_or(StyleRenderError::ScrollStateRevisionExhausted)?
    } else {
        base_revision
    };
    Ok((current, revision))
}

fn same_scroll_geometry(left: &RuntimeScrollContainer, right: &RuntimeScrollContainer) -> bool {
    left.owner() == right.owner()
        && left.parent() == right.parent()
        && left.scrollport_css_px() == right.scrollport_css_px()
        && left.scrollable_width_css_px() == right.scrollable_width_css_px()
        && left.scrollable_height_css_px() == right.scrollable_height_css_px()
}

pub(super) fn scroll_ancestors_for_node<R: FallibleReserve>(
    node: NodeId,
    style: &ComputedElementStyle,
    parent_by_node: &BTreeMap<NodeId, NodeId>,
    offsets: &BTreeMap<NodeId, (f32, f32)>,
    reserve: &mut R,
) -> Result<Vec<NodeId>, StyleRenderError> {
    if style.layout_position == ComputedCssPosition::Fixed {
        return Ok(Vec::new());
    }
    let mut ancestors = Vec::new();
    let mut cursor = parent_by_node.get(&node).copied();
    let mut visited = 0_usize;
    while let Some(parent) = cursor {
        visited = visited
            .checked_add(1)
            .ok_or(StyleRenderError::InvalidRoot)?;
        if visited > parent_by_node.len() {
            return Err(StyleRenderError::InvalidRoot);
        }
        if offsets.contains_key(&parent) {
            reserve.reserve(&mut ancestors, 1, "scroll render ancestry")?;
            ancestors.push(parent);
        }
        cursor = parent_by_node.get(&parent).copied();
    }
    Ok(ancestors)
}
