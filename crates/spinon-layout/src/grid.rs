use std::collections::BTreeMap;

use spinon_core::NodeId;

use crate::grid_placement::normalize_grid_item_placements;
use crate::{
    LayoutDisplay, LayoutError, LayoutInput, LayoutPosition, percentage_basis::DefiniteAxes,
};

/// Taffy 0.14.0 Grid 코드가 축마다 내부적으로 허용하는 최대 track 수입니다.
/// Taffy가 넘친 template를 잘라 처리하므로 그 전에 명시적으로 거부합니다.
pub const GRID_MAX_TRACKS: usize = 10_000;

/// 제한 Grid profile에서 사용하는 명시 track 크기입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GridTrackSize {
    LengthPx(f32),
    Percentage(f32),
    Fraction(f32),
}

/// CSS Grid auto-placement 방향입니다. C11.1은 기본 row만 계산합니다.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum GridAutoFlow {
    #[default]
    Row,
    Column,
    RowDense,
    ColumnDense,
}

/// 동일 revision의 계산된 Grid container 입력입니다.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GridContainerInput {
    pub columns: Vec<GridTrackSize>,
    pub rows: Vec<GridTrackSize>,
    pub auto_flow: GridAutoFlow,
}

pub(super) fn validate_grid_layout(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
    definite_axes: &BTreeMap<NodeId, DefiniteAxes>,
) -> Result<(), LayoutError> {
    for &id in input.grid_containers.keys() {
        if !index.contains_key(&id) {
            return Err(LayoutError::UnknownGridContainer(id));
        }
        if input.nodes[index[&id]].style.display != LayoutDisplay::Grid {
            return Err(LayoutError::UnexpectedGridContainer(id));
        }
    }
    for &id in input.grid_items.keys() {
        if !index.contains_key(&id) {
            return Err(LayoutError::UnknownGridItem(id));
        }
    }

    let normalized_items = normalize_grid_item_placements(input, index)?;

    let mut parent_by_child = BTreeMap::new();
    for node in &input.nodes {
        for &child in &node.children {
            parent_by_child.insert(child, node.id);
        }
    }
    for &id in input.grid_items.keys() {
        let parent = parent_by_child.get(&id).copied();
        if !parent
            .is_some_and(|parent| input.nodes[index[&parent]].style.display == LayoutDisplay::Grid)
        {
            return Err(LayoutError::UnexpectedGridItem(id));
        }
    }

    let mut hidden = BTreeMap::new();
    let mut pending = vec![(input.root, false)];
    while let Some((id, parent_hidden)) = pending.pop() {
        let node = &input.nodes[index[&id]];
        let subtree_hidden = parent_hidden || node.style.display == LayoutDisplay::None;
        hidden.insert(id, subtree_hidden);
        pending.extend(
            node.children
                .iter()
                .rev()
                .copied()
                .map(|child| (child, subtree_hidden)),
        );
    }

    for node in &input.nodes {
        if node.style.display != LayoutDisplay::Grid || hidden[&node.id] {
            continue;
        }
        let Some(container) = input.grid_containers.get(&node.id) else {
            return Err(LayoutError::MissingGridContainer(node.id));
        };
        validate_tracks(node.id, "grid-template-columns", &container.columns)?;
        validate_tracks(node.id, "grid-template-rows", &container.rows)?;
        if container.auto_flow != GridAutoFlow::Row {
            return Err(LayoutError::UnsupportedGridAutoFlow(node.id));
        }

        let definite = definite_axes.get(&node.id).copied().unwrap_or_default();
        if !definite.width || !definite.height {
            return Err(LayoutError::IndefiniteGridBasis {
                node: node.id,
                axis: if !definite.width { "width" } else { "height" },
            });
        }

        let cell_count = container
            .columns
            .len()
            .checked_mul(container.rows.len())
            .ok_or(LayoutError::GridTrackExpansionOverflow(node.id))?;
        let has_explicit_placement = node
            .children
            .iter()
            .any(|child| normalized_items.contains_key(child));
        let mut item_count = 0usize;
        for &child_id in &node.children {
            let child = &input.nodes[index[&child_id]];
            if hidden[&child_id] {
                continue;
            }
            if child.style.order != 0 {
                return Err(LayoutError::UnsupportedGridOrder(child_id));
            }
            if input
                .positioning
                .get(&child_id)
                .is_some_and(|value| value.position != LayoutPosition::Static)
            {
                return Err(LayoutError::UnsupportedGridPositionedItem(child_id));
            }
            let valid_item_size = |size| {
                matches!(
                    (has_explicit_placement, size),
                    (
                        true,
                        crate::LayoutDimension::Auto | crate::LayoutDimension::Fixed(_)
                    ) | (false, crate::LayoutDimension::Fixed(_))
                )
            };
            if !valid_item_size(child.style.width) || !valid_item_size(child.style.height) {
                return Err(LayoutError::UnsupportedGridItemSize(child_id));
            }
            for (property, value) in [
                ("min-width", child.style.min_width),
                ("max-width", child.style.max_width),
                ("min-height", child.style.min_height),
                ("max-height", child.style.max_height),
            ] {
                if value != crate::LayoutDimension::Auto {
                    return Err(LayoutError::UnsupportedGridItemConstraint {
                        node: child_id,
                        property,
                    });
                }
            }
            item_count = item_count
                .checked_add(1)
                .ok_or(LayoutError::GridTrackExpansionOverflow(node.id))?;
        }
        if !has_explicit_placement && item_count > cell_count {
            return Err(LayoutError::GridImplicitTrackRequired(node.id));
        }
    }
    Ok(())
}

fn validate_tracks(
    node: NodeId,
    property: &'static str,
    tracks: &[GridTrackSize],
) -> Result<(), LayoutError> {
    if tracks.is_empty() {
        return Err(LayoutError::InvalidGridTrackList { node, property });
    }
    if tracks.len() > GRID_MAX_TRACKS {
        return Err(LayoutError::GridTrackLimitExceeded {
            node,
            property,
            count: tracks.len(),
            maximum: GRID_MAX_TRACKS,
        });
    }
    for track in tracks {
        let valid = match track {
            GridTrackSize::LengthPx(value) => value.is_finite() && *value >= 0.0,
            GridTrackSize::Percentage(value) => value.is_finite() && *value >= 0.0,
            GridTrackSize::Fraction(value) => value.is_finite() && *value > 0.0,
        };
        if !valid {
            return Err(LayoutError::InvalidGridTrack { node, property });
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "grid_tests.rs"]
mod tests;
