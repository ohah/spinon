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

/// Selects no-area row-flow Grid owners whose visible direct items all lock the same axis.
///
/// The layout mode performs independent track, item, and implicit-extent validation. This
/// selector only chooses the narrow projection route from typed computed placement values.
pub(super) fn candidate_nodes(
    snapshot: &ComputedStyleSnapshot,
    nodes: &RuntimeGridNodes,
    dense_route_nodes: &BTreeSet<NodeId>,
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
            !items.is_empty()
                && items.iter().all(|item| {
                    let Some(placement) = elements
                        .get(item)
                        .and_then(|element| element.grid_item_placement.as_ref())
                    else {
                        return false;
                    };
                    let Some(item_axis) = definite_axis(placement) else {
                        return false;
                    };
                    if axis.is_some_and(|previous| previous != item_axis) {
                        return false;
                    }
                    axis = Some(item_axis);
                    true
                })
        })
        .collect()
}

fn definite_axis(placement: &ComputedCssGridItemPlacement) -> Option<DefiniteAxis> {
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
        (true, false, false, true) => Some(DefiniteAxis::Row),
        (false, true, true, false) => Some(DefiniteAxis::Column),
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
    use super::{DefiniteAxis, definite_axis};
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
}
