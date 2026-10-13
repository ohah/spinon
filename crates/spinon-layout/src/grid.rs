use std::collections::BTreeMap;

use spinon_core::NodeId;

use crate::grid_placement::normalize_grid_layout;
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

/// Grid container가 사용하는 item placement 계약입니다.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum GridPlacementMode {
    /// C11.1의 all-auto 또는 기존 explicit-only 분류를 사용합니다.
    #[default]
    Existing,
    /// C11.2.1의 모든 visible item이 definite placement여야 합니다.
    ExplicitOnly,
    /// C11.2.2의 row sparse auto-placement와 양의 implicit row를 계산합니다.
    SparseRow,
    /// C11.2.3의 row/column sparse·dense 자동 배치와 양의 implicit track을 계산합니다.
    DenseAutoPlacement,
}

/// 제한 profile에서 허용하는 implicit track sizing 입력입니다.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum GridAutoTrackSize {
    /// `auto`; 해당 축에서 implicit track이 필요하지 않을 때만 사용할 수 있습니다.
    #[default]
    Auto,
    /// 모든 positive implicit track에 반복할 하나의 고정 CSS px track입니다.
    LengthPx(f32),
}

/// 제한 profile에서 허용하는 implicit row sizing 입력입니다.
pub type GridAutoRows = GridAutoTrackSize;

/// 제한 profile에서 허용하는 implicit column sizing 입력입니다.
pub type GridAutoColumns = GridAutoTrackSize;

/// 동일 revision의 계산된 Grid container 입력입니다.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GridContainerInput {
    pub columns: Vec<GridTrackSize>,
    pub rows: Vec<GridTrackSize>,
    /// CSS Grid의 explicit column 경계 이름입니다. 배열은 track 수보다 하나 큽니다.
    pub columns_line_names: Vec<Vec<String>>,
    /// CSS Grid의 explicit row 경계 이름입니다. 배열은 track 수보다 하나 큽니다.
    pub rows_line_names: Vec<Vec<String>>,
    pub auto_flow: GridAutoFlow,
    pub auto_rows: GridAutoRows,
    pub auto_columns: GridAutoColumns,
    pub placement_mode: GridPlacementMode,
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

    for (&id, container) in &input.grid_containers {
        match (container.placement_mode, container.auto_rows) {
            (
                GridPlacementMode::SparseRow | GridPlacementMode::DenseAutoPlacement,
                GridAutoRows::Auto | GridAutoRows::LengthPx(_),
            )
            | (_, GridAutoRows::Auto) => {}
            (_, GridAutoRows::LengthPx(_)) => {
                return Err(LayoutError::UnsupportedGridAutoRows {
                    node: id,
                    reason: "grid-auto-rows는 자동 배치 profile에서만 사용할 수 있습니다",
                });
            }
        }
        match (container.placement_mode, container.auto_columns) {
            (
                GridPlacementMode::DenseAutoPlacement,
                GridAutoColumns::Auto | GridAutoColumns::LengthPx(_),
            )
            | (_, GridAutoColumns::Auto) => {}
            (_, GridAutoColumns::LengthPx(_)) => {
                return Err(LayoutError::UnsupportedGridAutoColumns {
                    node: id,
                    reason: "grid-auto-columns는 C11.2.3 profile에서만 사용할 수 있습니다",
                });
            }
        }
        if let GridAutoRows::LengthPx(value) = container.auto_rows
            && (!value.is_finite() || value < 0.0)
        {
            return Err(LayoutError::InvalidGridTrack {
                node: id,
                property: "grid-auto-rows",
            });
        }
        if let GridAutoColumns::LengthPx(value) = container.auto_columns
            && (!value.is_finite() || value < 0.0)
        {
            return Err(LayoutError::InvalidGridTrack {
                node: id,
                property: "grid-auto-columns",
            });
        }
    }

    let normalized_grid = normalize_grid_layout(input, index)?;

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
        if (!container.columns_line_names.is_empty()
            && container.columns.len().checked_add(1) != Some(container.columns_line_names.len()))
            || (!container.rows_line_names.is_empty()
                && container.rows.len().checked_add(1) != Some(container.rows_line_names.len()))
        {
            return Err(LayoutError::InvalidGridTrackList {
                node: node.id,
                property: if container.columns_line_names.is_empty() {
                    "grid-template-rows"
                } else {
                    "grid-template-columns"
                },
            });
        }
        if container.placement_mode == GridPlacementMode::SparseRow
            && container.auto_flow != GridAutoFlow::Row
        {
            return Err(LayoutError::UnsupportedGridAutoFlow(node.id));
        }
        if !matches!(
            container.placement_mode,
            GridPlacementMode::SparseRow | GridPlacementMode::DenseAutoPlacement
        ) && container.auto_flow != GridAutoFlow::Row
        {
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
            .any(|child| normalized_grid.item_placements.contains_key(child));
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
            let valid_item_size = |size| match container.placement_mode {
                GridPlacementMode::SparseRow | GridPlacementMode::DenseAutoPlacement
                    if normalized_grid.auto_placed_items.contains(&child_id) =>
                {
                    matches!(size, crate::LayoutDimension::Fixed(_))
                }
                GridPlacementMode::Existing | GridPlacementMode::ExplicitOnly
                    if has_explicit_placement =>
                {
                    matches!(
                        size,
                        crate::LayoutDimension::Auto | crate::LayoutDimension::Fixed(_)
                    )
                }
                _ => matches!(size, crate::LayoutDimension::Fixed(_)),
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
        if !matches!(
            container.placement_mode,
            GridPlacementMode::SparseRow | GridPlacementMode::DenseAutoPlacement
        ) && !has_explicit_placement
            && item_count > cell_count
        {
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

#[cfg(test)]
#[path = "grid_auto_placement_tests.rs"]
mod auto_placement_tests;

#[cfg(test)]
#[path = "grid_auto_placement/dense_column_tests.rs"]
mod dense_column_tests;
