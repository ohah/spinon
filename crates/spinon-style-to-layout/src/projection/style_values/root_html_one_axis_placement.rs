//! Parsed Root HTML one-axis numeric placement admission for C11.1.

use std::collections::{BTreeMap, BTreeSet};

use spinon_core::NodeId;
use spinon_style::{
    ComputedCssGridAutoFlow, ComputedCssGridItemPlacement, ComputedCssGridPlacement,
    ComputedStyleSnapshot,
};

use super::RuntimeGridNodes;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DefiniteAxis {
    Row,
    Column,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PlacementKind {
    FullyDefinite,
    OneAxis(DefiniteAxis),
    FullyAuto,
}

/// Selects no-area row-flow Grid owners whose visible direct items all lock the same axis.
///
/// The layout mode performs independent track, item, and implicit-extent validation. This
/// selector only chooses the narrow projection route from typed computed placement values.
pub(super) fn candidate_nodes(
    snapshot: &ComputedStyleSnapshot,
    nodes: &RuntimeGridNodes,
    dense_route_nodes: &BTreeSet<NodeId>,
) -> BTreeSet<NodeId> {
    candidate_nodes_with_mixed_explicit(snapshot, nodes, dense_route_nodes, false)
}

/// Selects owners with at least one fully definite item plus same-axis one-axis items.
pub(super) fn mixed_candidate_nodes(
    snapshot: &ComputedStyleSnapshot,
    nodes: &RuntimeGridNodes,
    dense_route_nodes: &BTreeSet<NodeId>,
) -> BTreeSet<NodeId> {
    candidate_nodes_with_mixed_explicit(snapshot, nodes, dense_route_nodes, true)
}

/// Selects owners with row-locked and column-locked items in the same sparse row-flow Grid.
pub(super) fn mixed_cross_axis_candidate_nodes(
    snapshot: &ComputedStyleSnapshot,
    nodes: &RuntimeGridNodes,
    dense_route_nodes: &BTreeSet<NodeId>,
) -> BTreeSet<NodeId> {
    mixed_cross_axis_candidate_nodes_for_flow(
        snapshot,
        nodes,
        dense_route_nodes,
        ComputedCssGridAutoFlow::Row,
    )
}

/// Selects owners with row-locked and column-locked items in sparse column flow.
pub(super) fn mixed_cross_axis_column_candidate_nodes(
    snapshot: &ComputedStyleSnapshot,
    nodes: &RuntimeGridNodes,
    dense_route_nodes: &BTreeSet<NodeId>,
) -> BTreeSet<NodeId> {
    mixed_cross_axis_candidate_nodes_for_flow(
        snapshot,
        nodes,
        dense_route_nodes,
        ComputedCssGridAutoFlow::Column,
    )
}

fn mixed_cross_axis_candidate_nodes_for_flow(
    snapshot: &ComputedStyleSnapshot,
    nodes: &RuntimeGridNodes,
    dense_route_nodes: &BTreeSet<NodeId>,
    flow: ComputedCssGridAutoFlow,
) -> BTreeSet<NodeId> {
    let elements = snapshot
        .elements
        .iter()
        .map(|element| (element.node_id, element))
        .collect::<BTreeMap<_, _>>();
    let mut items_by_owner = BTreeMap::<NodeId, Vec<NodeId>>::new();
    for (item, owner) in &nodes.grid_item_owners {
        items_by_owner.entry(*owner).or_default().push(*item);
    }

    nodes
        .grid_containers
        .iter()
        .copied()
        .filter(|owner| {
            if dense_route_nodes.contains(owner) {
                return false;
            }
            let Some(grid) = elements
                .get(owner)
                .and_then(|element| element.grid_layout.as_ref())
            else {
                return false;
            };
            if grid.template_areas.is_some()
                || grid.auto_flow != flow
                || grid.columns_fixed_repeat_group_count != 0
                || grid.rows_fixed_repeat_group_count != 0
                || grid.columns_auto_repeat.is_some()
                || grid.rows_auto_repeat.is_some()
                || grid
                    .columns_line_names
                    .iter()
                    .chain(&grid.rows_line_names)
                    .any(|group| !group.is_empty())
            {
                return false;
            }
            let Some(items) = items_by_owner.get(owner) else {
                return false;
            };
            let mut row_locked_count = 0usize;
            let mut column_locked_count = 0usize;
            let accepted = !items.is_empty()
                && items.iter().all(|item| {
                    let Some(placement) = elements
                        .get(item)
                        .and_then(|element| element.grid_item_placement.as_ref())
                    else {
                        return false;
                    };
                    match placement_kind(placement) {
                        Some(PlacementKind::FullyDefinite | PlacementKind::FullyAuto) => true,
                        Some(PlacementKind::OneAxis(DefiniteAxis::Row)) => {
                            row_locked_count += 1;
                            true
                        }
                        Some(PlacementKind::OneAxis(DefiniteAxis::Column)) => {
                            column_locked_count += 1;
                            true
                        }
                        None => false,
                    }
                });
            accepted && row_locked_count > 0 && column_locked_count > 0
        })
        .collect()
}

fn candidate_nodes_with_mixed_explicit(
    snapshot: &ComputedStyleSnapshot,
    nodes: &RuntimeGridNodes,
    dense_route_nodes: &BTreeSet<NodeId>,
    allow_fully_definite: bool,
) -> BTreeSet<NodeId> {
    let elements = snapshot
        .elements
        .iter()
        .map(|element| (element.node_id, element))
        .collect::<BTreeMap<_, _>>();
    let mut items_by_owner = BTreeMap::<NodeId, Vec<NodeId>>::new();
    for (item, owner) in &nodes.grid_item_owners {
        items_by_owner.entry(*owner).or_default().push(*item);
    }

    nodes
        .grid_containers
        .iter()
        .copied()
        .filter(|owner| {
            if dense_route_nodes.contains(owner) {
                return false;
            }
            let Some(grid) = elements
                .get(owner)
                .and_then(|element| element.grid_layout.as_ref())
            else {
                return false;
            };
            if grid.template_areas.is_some()
                || grid.auto_flow != ComputedCssGridAutoFlow::Row
                || grid.columns_fixed_repeat_group_count != 0
                || grid.rows_fixed_repeat_group_count != 0
                || grid.columns_auto_repeat.is_some()
                || grid.rows_auto_repeat.is_some()
                || grid
                    .columns_line_names
                    .iter()
                    .chain(&grid.rows_line_names)
                    .any(|group| !group.is_empty())
            {
                return false;
            }
            let Some(items) = items_by_owner.get(owner) else {
                return false;
            };
            let mut axis = None;
            let mut fully_definite_count = 0usize;
            let mut one_axis_count = 0usize;
            let accepted = !items.is_empty()
                && items.iter().all(|item| {
                    let Some(placement) = elements
                        .get(item)
                        .and_then(|element| element.grid_item_placement.as_ref())
                    else {
                        return false;
                    };
                    let Some(kind) = placement_kind(placement) else {
                        return false;
                    };
                    match kind {
                        PlacementKind::FullyDefinite if allow_fully_definite => {
                            fully_definite_count += 1;
                        }
                        PlacementKind::FullyDefinite | PlacementKind::FullyAuto => return false,
                        PlacementKind::OneAxis(item_axis) => {
                            if axis.is_some_and(|previous| previous != item_axis) {
                                return false;
                            }
                            axis = Some(item_axis);
                            one_axis_count += 1;
                        }
                    }
                    true
                });
            accepted
                && if allow_fully_definite {
                    fully_definite_count > 0 && one_axis_count > 0
                } else {
                    one_axis_count == items.len()
                }
        })
        .collect()
}

#[cfg(test)]
fn definite_axis(placement: &ComputedCssGridItemPlacement) -> Option<DefiniteAxis> {
    match placement_kind(placement)? {
        PlacementKind::OneAxis(axis) => Some(axis),
        PlacementKind::FullyDefinite | PlacementKind::FullyAuto => None,
    }
}

fn placement_kind(placement: &ComputedCssGridItemPlacement) -> Option<PlacementKind> {
    let row_has_line = has_line(&placement.row_start, &placement.row_end);
    let column_has_line = has_line(&placement.column_start, &placement.column_end);
    let row_auto = is_auto(&placement.row_start, &placement.row_end);
    let column_auto = is_auto(&placement.column_start, &placement.column_end);
    let endpoints_are_numeric = [
        &placement.row_start,
        &placement.row_end,
        &placement.column_start,
        &placement.column_end,
    ]
    .into_iter()
    .all(|value| {
        matches!(
            value,
            ComputedCssGridPlacement::Auto
                | ComputedCssGridPlacement::Line(_)
                | ComputedCssGridPlacement::Span(_)
        )
    });
    if !endpoints_are_numeric {
        return None;
    }
    match (row_has_line, row_auto, column_has_line, column_auto) {
        (true, false, true, false) => Some(PlacementKind::FullyDefinite),
        (true, false, false, true) => Some(PlacementKind::OneAxis(DefiniteAxis::Row)),
        (false, true, true, false) => Some(PlacementKind::OneAxis(DefiniteAxis::Column)),
        (false, true, false, true) => Some(PlacementKind::FullyAuto),
        _ => None,
    }
}

fn has_line(start: &ComputedCssGridPlacement, end: &ComputedCssGridPlacement) -> bool {
    matches!(start, ComputedCssGridPlacement::Line(_))
        || matches!(end, ComputedCssGridPlacement::Line(_))
}

fn is_auto(start: &ComputedCssGridPlacement, end: &ComputedCssGridPlacement) -> bool {
    matches!(start, ComputedCssGridPlacement::Auto) && matches!(end, ComputedCssGridPlacement::Auto)
}

#[cfg(test)]
mod tests {
    use super::{DefiniteAxis, PlacementKind, definite_axis, placement_kind};
    use spinon_style::{ComputedCssGridItemPlacement, ComputedCssGridPlacement};

    fn placement(
        row_start: ComputedCssGridPlacement,
        row_end: ComputedCssGridPlacement,
        column_start: ComputedCssGridPlacement,
        column_end: ComputedCssGridPlacement,
    ) -> ComputedCssGridItemPlacement {
        ComputedCssGridItemPlacement {
            row_start,
            row_end,
            column_start,
            column_end,
        }
    }

    #[test]
    fn admission_recognizes_numeric_line_and_span_on_one_uniform_axis() {
        assert_eq!(
            definite_axis(&placement(
                ComputedCssGridPlacement::Line(1),
                ComputedCssGridPlacement::Span(2),
                ComputedCssGridPlacement::Auto,
                ComputedCssGridPlacement::Auto,
            )),
            Some(DefiniteAxis::Row)
        );
        assert_eq!(
            definite_axis(&placement(
                ComputedCssGridPlacement::Auto,
                ComputedCssGridPlacement::Auto,
                ComputedCssGridPlacement::Line(-2),
                ComputedCssGridPlacement::Line(-1),
            )),
            Some(DefiniteAxis::Column)
        );
    }

    #[test]
    fn admission_rejects_all_auto_mixed_axes_fully_definite_and_named_lines() {
        let auto = || ComputedCssGridPlacement::Auto;
        assert_eq!(
            definite_axis(&placement(auto(), auto(), auto(), auto())),
            None
        );
        assert_eq!(
            definite_axis(&placement(
                ComputedCssGridPlacement::Line(1),
                ComputedCssGridPlacement::Line(2),
                auto(),
                auto(),
            )),
            Some(DefiniteAxis::Row)
        );
        assert_eq!(
            definite_axis(&placement(
                ComputedCssGridPlacement::Line(1),
                ComputedCssGridPlacement::Line(2),
                ComputedCssGridPlacement::Line(1),
                ComputedCssGridPlacement::Line(2),
            )),
            None
        );
        assert_eq!(
            definite_axis(&placement(
                ComputedCssGridPlacement::Line(1),
                ComputedCssGridPlacement::Line(2),
                ComputedCssGridPlacement::Auto,
                ComputedCssGridPlacement::Line(2),
            )),
            None
        );
    }

    #[test]
    fn mixed_route_distinguishes_fully_definite_from_same_axis_partial_items() {
        let full = placement(
            ComputedCssGridPlacement::Line(1),
            ComputedCssGridPlacement::Line(2),
            ComputedCssGridPlacement::Line(2),
            ComputedCssGridPlacement::Line(3),
        );
        let row_partial = placement(
            ComputedCssGridPlacement::Line(1),
            ComputedCssGridPlacement::Line(2),
            ComputedCssGridPlacement::Auto,
            ComputedCssGridPlacement::Auto,
        );
        let column_partial = placement(
            ComputedCssGridPlacement::Auto,
            ComputedCssGridPlacement::Auto,
            ComputedCssGridPlacement::Line(2),
            ComputedCssGridPlacement::Line(3),
        );

        assert_eq!(placement_kind(&full), Some(PlacementKind::FullyDefinite));
        assert_eq!(
            placement_kind(&row_partial),
            Some(PlacementKind::OneAxis(DefiniteAxis::Row))
        );
        assert_eq!(
            placement_kind(&column_partial),
            Some(PlacementKind::OneAxis(DefiniteAxis::Column))
        );
        assert_eq!(definite_axis(&full), None);
    }

    #[test]
    fn cross_axis_route_classifies_both_locked_axes_and_fully_auto_items() {
        let auto = || ComputedCssGridPlacement::Auto;
        let row_partial = placement(ComputedCssGridPlacement::Line(1), auto(), auto(), auto());
        let column_partial = placement(auto(), auto(), ComputedCssGridPlacement::Line(1), auto());
        let fully_auto = placement(auto(), auto(), auto(), auto());

        assert_eq!(
            placement_kind(&row_partial),
            Some(PlacementKind::OneAxis(DefiniteAxis::Row))
        );
        assert_eq!(
            placement_kind(&column_partial),
            Some(PlacementKind::OneAxis(DefiniteAxis::Column))
        );
        assert_eq!(placement_kind(&fully_auto), Some(PlacementKind::FullyAuto));
    }
}
