use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, NodeId};
use spinon_style::{ComputedCssPosition, ComputedElementStyle, ComputedStyleSnapshot};

use crate::StyleLayoutError;

pub(super) fn validate_fixed_ancestor_effects(
    tree: &HostDocumentSnapshot,
    root: HostNodeHandle,
    snapshot: &ComputedStyleSnapshot,
) -> Result<(), StyleLayoutError> {
    let mut computed = BTreeMap::<NodeId, &ComputedElementStyle>::new();
    for element in snapshot.elements.iter() {
        computed.entry(element.node_id).or_insert(element);
    }

    let mut parents = BTreeMap::new();
    let mut visited = BTreeSet::new();
    let mut pending = vec![root];
    while let Some(parent) = pending.pop() {
        if !visited.insert(parent.id()) {
            continue;
        }
        let children = tree
            .children(parent)
            .ok_or(StyleLayoutError::MissingComputedElement(parent.id()))?;
        for child in children {
            parents.insert(child.id(), parent.id());
            pending.push(child);
        }
    }

    for element in snapshot
        .elements
        .iter()
        .filter(|element| element.layout_position == ComputedCssPosition::Fixed)
    {
        let mut ancestor = parents.get(&element.node_id).copied();
        while let Some(ancestor_id) = ancestor {
            let ancestor_style = computed
                .get(&ancestor_id)
                .ok_or(StyleLayoutError::MissingComputedElement(ancestor_id))?;
            if let Some(&effect) = ancestor_style.fixed_containing_block_effects.first() {
                return Err(StyleLayoutError::UnsupportedFixedContainingBlockEffect {
                    node: element.node_id,
                    ancestor: ancestor_id,
                    effect,
                });
            }
            ancestor = parents.get(&ancestor_id).copied();
        }
    }
    Ok(())
}
