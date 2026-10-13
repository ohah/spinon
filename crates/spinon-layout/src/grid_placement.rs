use std::collections::{BTreeMap, BTreeSet};

use spinon_core::NodeId;

use crate::{GridPlacementMode, GridTrackSize, LayoutDisplay, LayoutError, LayoutInput};

/// 한 번 계산하는 동안 정규화한 모든 Grid item과 container row입니다.
#[derive(Default)]
pub(crate) struct NormalizedGridLayout {
    pub(crate) item_placements: BTreeMap<NodeId, GridItemInput>,
    pub(crate) expanded_rows: BTreeMap<NodeId, Vec<GridTrackSize>>,
    pub(crate) expanded_columns: BTreeMap<NodeId, Vec<GridTrackSize>>,
    pub(crate) auto_placed_items: BTreeSet<NodeId>,
}

/// 기존 placement profile과 C11.2.2 sparse profile의 입력을 계산 가능한 line으로 변환합니다.
pub(super) fn normalize_grid_layout(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
) -> Result<NormalizedGridLayout, LayoutError> {
    let mut output = NormalizedGridLayout {
        item_placements: normalize_grid_item_placements(input, index)?,
        ..NormalizedGridLayout::default()
    };
    let hidden = hidden_subtrees(input, index);
    for container_node in &input.nodes {
        let Some(container) = input.grid_containers.get(&container_node.id) else {
            continue;
        };
        if container_node.style.display != LayoutDisplay::Grid
            || hidden[&container_node.id]
            || !matches!(
                container.placement_mode,
                GridPlacementMode::SparseRow | GridPlacementMode::DenseAutoPlacement
            )
        {
            continue;
        }
        if container.placement_mode == GridPlacementMode::SparseRow {
            let sparse = super::grid_auto_placement::normalize_sparse_grid_container(
                input,
                &hidden,
                container_node,
                container,
            )?;
            output.item_placements.extend(sparse.item_placements);
            if let Some(rows) = sparse.expanded_rows {
                output.expanded_rows.insert(container_node.id, rows);
            }
            output.auto_placed_items.extend(sparse.auto_placed_items);
        } else {
            let dense = super::grid_auto_placement::normalize_dense_grid_container(
                input,
                &hidden,
                container_node,
                container,
            )?;
            output.item_placements.extend(dense.item_placements);
            if let Some(rows) = dense.expanded_rows {
                output.expanded_rows.insert(container_node.id, rows);
            }
            if let Some(columns) = dense.expanded_columns {
                output.expanded_columns.insert(container_node.id, columns);
            }
            output.auto_placed_items.extend(dense.auto_placed_items);
        }
    }
    Ok(output)
}

/// CSS Grid item의 계산된 배치 끝점입니다.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum GridPlacementInput {
    #[default]
    Auto,
    Line(i32),
    Span(u32),
}

/// 한 in-flow Grid item의 row·column 배치 입력입니다.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GridItemInput {
    pub row_start: GridPlacementInput,
    pub row_end: GridPlacementInput,
    pub column_start: GridPlacementInput,
    pub column_end: GridPlacementInput,
}

impl GridItemInput {
    pub(super) fn is_auto(self) -> bool {
        self.row_start == GridPlacementInput::Auto
            && self.row_end == GridPlacementInput::Auto
            && self.column_start == GridPlacementInput::Auto
            && self.column_end == GridPlacementInput::Auto
    }
}

/// 각 explicit-only Grid의 CSS line/span 입력을 Taffy에 넘길 양수 line으로 정규화합니다.
pub(super) fn normalize_grid_item_placements(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
) -> Result<BTreeMap<NodeId, GridItemInput>, LayoutError> {
    let hidden = hidden_subtrees(input, index);
    let mut normalized = BTreeMap::new();

    for container_node in &input.nodes {
        if container_node.style.display != LayoutDisplay::Grid || hidden[&container_node.id] {
            continue;
        }
        let container = input
            .grid_containers
            .get(&container_node.id)
            .ok_or(LayoutError::MissingGridContainer(container_node.id))?;
        if matches!(
            container.placement_mode,
            GridPlacementMode::SparseRow | GridPlacementMode::DenseAutoPlacement
        ) {
            continue;
        }
        let visible_items = container_node
            .children
            .iter()
            .copied()
            .filter(|child| !hidden[child])
            .collect::<Vec<_>>();
        let all_auto = visible_items.iter().all(|child| {
            input
                .grid_items
                .get(child)
                .copied()
                .unwrap_or_default()
                .is_auto()
        });
        if all_auto {
            continue;
        }

        for child in visible_items {
            let placement = input.grid_items.get(&child).copied().unwrap_or_default();
            if placement.is_auto() {
                return Err(invalid_placement(
                    child,
                    "grid-row-start",
                    "같은 Grid 안에서 auto item과 명시 배치 item을 섞을 수 없습니다",
                ));
            }
            let (row_start, row_end) = normalize_axis(
                child,
                "grid-row-start",
                "grid-row-end",
                placement.row_start,
                placement.row_end,
                container.rows.len(),
            )?;
            let (column_start, column_end) = normalize_axis(
                child,
                "grid-column-start",
                "grid-column-end",
                placement.column_start,
                placement.column_end,
                container.columns.len(),
            )?;
            normalized.insert(
                child,
                GridItemInput {
                    row_start: GridPlacementInput::Line(i32::from(row_start)),
                    row_end: GridPlacementInput::Line(i32::from(row_end)),
                    column_start: GridPlacementInput::Line(i32::from(column_start)),
                    column_end: GridPlacementInput::Line(i32::from(column_end)),
                },
            );
        }
    }
    Ok(normalized)
}

pub(super) fn hidden_subtrees(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
) -> BTreeMap<NodeId, bool> {
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
    hidden
}

fn normalize_axis(
    node: NodeId,
    start_property: &'static str,
    end_property: &'static str,
    start: GridPlacementInput,
    end: GridPlacementInput,
    track_count: usize,
) -> Result<(i16, i16), LayoutError> {
    let start_line = match start {
        GridPlacementInput::Line(value) => {
            Some(explicit_line(node, start_property, value, track_count)?)
        }
        GridPlacementInput::Auto | GridPlacementInput::Span(_) => None,
    };
    let end_line = match end {
        GridPlacementInput::Line(value) => {
            Some(explicit_line(node, end_property, value, track_count)?)
        }
        GridPlacementInput::Auto | GridPlacementInput::Span(_) => None,
    };

    let bounds = explicit_line_bounds(track_count);
    let (start, end) = match (start, end, start_line, end_line) {
        (GridPlacementInput::Line(_), GridPlacementInput::Line(_), Some(start), Some(end)) => {
            if start == end {
                return Err(invalid_placement(
                    node,
                    end_property,
                    "같은 start/end line은 CSS auto placement를 요구합니다",
                ));
            }
            if start < end {
                (start, end)
            } else {
                (end, start)
            }
        }
        (GridPlacementInput::Line(_), GridPlacementInput::Span(span), Some(start), None) => {
            let span = positive_span(node, end_property, span)?;
            let end = start.checked_add(i64::from(span)).ok_or_else(|| {
                invalid_placement(
                    node,
                    end_property,
                    "span line 계산이 정수 범위를 넘었습니다",
                )
            })?;
            (start, end)
        }
        (GridPlacementInput::Span(span), GridPlacementInput::Line(_), None, Some(end)) => {
            let span = positive_span(node, start_property, span)?;
            let start = end.checked_sub(i64::from(span)).ok_or_else(|| {
                invalid_placement(
                    node,
                    start_property,
                    "span line 계산이 정수 범위를 넘었습니다",
                )
            })?;
            (start, end)
        }
        (GridPlacementInput::Span(_), GridPlacementInput::Span(_), _, _) => {
            return Err(invalid_placement(
                node,
                start_property,
                "한 축의 두 끝점을 span으로만 정할 수 없습니다",
            ));
        }
        _ => {
            return Err(invalid_placement(
                node,
                start_property,
                "한 축의 두 끝점은 모두 definite line이어야 합니다",
            ));
        }
    };

    if start < bounds.0 || end > bounds.1 || start >= end {
        return Err(invalid_placement(
            node,
            start_property,
            "정규화한 범위가 explicit track 밖에 있어 implicit track이 필요합니다",
        ));
    }
    Ok((
        i16::try_from(start).map_err(|_| {
            invalid_placement(
                node,
                start_property,
                "정규화한 start line이 Taffy 범위를 넘었습니다",
            )
        })?,
        i16::try_from(end).map_err(|_| {
            invalid_placement(
                node,
                end_property,
                "정규화한 end line이 Taffy 범위를 넘었습니다",
            )
        })?,
    ))
}

pub(super) fn explicit_line(
    node: NodeId,
    property: &'static str,
    value: i32,
    track_count: usize,
) -> Result<i64, LayoutError> {
    if value == 0 {
        return Err(invalid_placement(
            node,
            property,
            "line 0은 CSS Grid에서 유효하지 않습니다",
        ));
    }
    let (minimum, maximum) = explicit_line_bounds(track_count);
    let line = i64::from(value);
    let normalized = if value < 0 { line + maximum + 1 } else { line };
    if normalized < minimum || normalized > maximum {
        return Err(invalid_placement(
            node,
            property,
            "line이 explicit track 범위를 벗어나 implicit track이 필요합니다",
        ));
    }
    Ok(normalized)
}

pub(super) fn positive_span(
    node: NodeId,
    property: &'static str,
    span: u32,
) -> Result<u32, LayoutError> {
    if span == 0 {
        return Err(invalid_placement(
            node,
            property,
            "span은 양의 정수여야 합니다",
        ));
    }
    Ok(span)
}

fn explicit_line_bounds(track_count: usize) -> (i64, i64) {
    let last_line = i64::try_from(track_count).unwrap_or(i64::MAX - 1) + 1;
    (1, last_line)
}

pub(super) fn invalid_placement(
    node: NodeId,
    property: &'static str,
    reason: &'static str,
) -> LayoutError {
    LayoutError::InvalidGridPlacement {
        node,
        property,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::{GridPlacementInput as P, normalize_axis};
    use spinon_core::NodeId;

    fn id() -> NodeId {
        NodeId::new(11_221).unwrap()
    }

    fn axis(start: P, end: P, tracks: usize) -> Result<(i16, i16), crate::LayoutError> {
        normalize_axis(
            id(),
            "grid-column-start",
            "grid-column-end",
            start,
            end,
            tracks,
        )
    }

    #[test]
    fn positive_and_negative_lines_resolve_from_explicit_grid_edges() {
        assert_eq!(axis(P::Line(1), P::Line(3), 3), Ok((1, 3)));
        assert_eq!(axis(P::Line(-2), P::Line(-1), 3), Ok((3, 4)));
        assert_eq!(axis(P::Line(-(3 + 1)), P::Line(-2), 3), Ok((1, 3)));
    }

    #[test]
    fn positive_spans_resolve_from_either_definite_endpoint() {
        assert_eq!(axis(P::Line(1), P::Span(2), 3), Ok((1, 3)));
        assert_eq!(axis(P::Span(2), P::Line(4), 3), Ok((2, 4)));
    }

    #[test]
    fn reversed_lines_swap_and_equal_lines_fail_closed() {
        assert_eq!(axis(P::Line(3), P::Line(1), 3), Ok((1, 3)));
        assert!(matches!(
            axis(P::Line(2), P::Line(2), 3),
            Err(crate::LayoutError::InvalidGridPlacement {
                property: "grid-column-end",
                ..
            })
        ));
    }

    #[test]
    fn zero_negative_span_auto_mixture_and_overflowing_ranges_fail_closed() {
        for (start, end) in [
            (P::Line(0), P::Line(2)),
            (P::Line(1), P::Span(0)),
            (P::Auto, P::Line(2)),
            (P::Span(1), P::Span(2)),
            (P::Line(1), P::Span(u32::MAX)),
            (P::Line(-5), P::Line(-1)),
            (P::Line(1), P::Line(5)),
        ] {
            assert!(
                axis(start, end, 3).is_err(),
                "unexpectedly accepted {start:?}/{end:?}"
            );
        }
    }
}
