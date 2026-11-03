//! Parsed Root HTML numeric explicit-placement admission for C11.1.

use std::collections::{BTreeMap, BTreeSet};

use spinon_core::NodeId;
use spinon_style::{
    ComputedCssGridAutoFlow, ComputedCssGridItemPlacement, ComputedCssGridPlacement,
    ComputedStyleSnapshot,
};

use super::RuntimeGridNodes;

/// Returns no-area row-flow Grid owners whose visible children all carry numeric placement.
///
/// This only selects the projection route. C11.2.1's layout normalizer remains responsible for
/// checking definite axes, mixed auto, spans, arithmetic bounds, and explicit-track extents.
pub(super) fn candidate_nodes(
    snapshot: &ComputedStyleSnapshot,
    nodes: &RuntimeGridNodes,
    dense_placement_route_nodes: &BTreeSet<NodeId>,
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
            if dense_placement_route_nodes.contains(owner) {
                return false;
            }
            let Some(grid) = elements
                .get(owner)
                .and_then(|element| element.grid_layout.as_ref())
            else {
                return false;
            };
            if grid.template_areas.is_some() || grid.auto_flow != ComputedCssGridAutoFlow::Row {
                return false;
            }
            let Some(items) = items_by_owner.get(owner) else {
                return false;
            };
            !items.is_empty()
                && items.iter().all(|item| {
                    elements
                        .get(item)
                        .and_then(|element| element.grid_item_placement.as_ref())
                        .is_some_and(has_numeric_placement)
                })
        })
        .collect()
}

fn has_numeric_placement(placement: &ComputedCssGridItemPlacement) -> bool {
    [
        &placement.row_start,
        &placement.row_end,
        &placement.column_start,
        &placement.column_end,
    ]
    .into_iter()
    .all(|value| {
        matches!(
            value,
            ComputedCssGridPlacement::Line(_) | ComputedCssGridPlacement::Span(_)
        )
    })
}
