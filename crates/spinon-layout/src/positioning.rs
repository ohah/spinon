use std::collections::BTreeMap;

use spinon_core::NodeId;

use crate::{
    LayoutError, LayoutInput, LayoutLengthPercentage, LayoutPosition,
    PositionedContainingBlockOwner, TextDirection,
};

pub(super) fn needs_flow_pass(input: &LayoutInput) -> bool {
    input.positioning.values().any(|positioning| {
        positioning.position == LayoutPosition::Absolute
            || (positioning.position == LayoutPosition::Relative
                && [
                    positioning.inset.top,
                    positioning.inset.right,
                    positioning.inset.bottom,
                    positioning.inset.left,
                ]
                .into_iter()
                .any(|inset| inset != LayoutLengthPercentage::Auto))
    })
}

pub(super) fn collect_positioned_owners(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
) -> BTreeMap<NodeId, PositionedContainingBlockOwner> {
    let mut owners = BTreeMap::new();
    let mut pending = vec![(input.root, PositionedContainingBlockOwner::Viewport, false)];
    while let Some((id, inherited_owner, ancestor_hidden)) = pending.pop() {
        let node = &input.nodes[index[&id]];
        let hidden = ancestor_hidden || node.style.display == crate::LayoutDisplay::None;
        let owner = if hidden {
            PositionedContainingBlockOwner::NoBox
        } else {
            inherited_owner
        };
        owners.insert(id, owner);
        let positioning = input.positioning.get(&id).copied().unwrap_or_default();
        let child_owner = if hidden {
            PositionedContainingBlockOwner::NoBox
        } else if matches!(
            positioning.position,
            LayoutPosition::Relative | LayoutPosition::Absolute
        ) {
            PositionedContainingBlockOwner::Node(id)
        } else {
            inherited_owner
        };
        pending.extend(
            node.children
                .iter()
                .rev()
                .map(|child| (*child, child_owner, hidden)),
        );
    }
    owners
}

pub(super) fn validate_positioning(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
) -> Result<(), LayoutError> {
    for (&id, positioning) in &input.positioning {
        let Some(&position) = index.get(&id) else {
            return Err(LayoutError::UnknownPositioningNode(id));
        };
        if positioning.position == LayoutPosition::Static {
            continue;
        }
        if positioning.position == LayoutPosition::Absolute && id == input.root {
            return Err(LayoutError::UnsupportedPositioning {
                node: id,
                reason: "레이아웃 root 자체의 absolute 배치는 현재 profile에서 지원하지 않습니다",
            });
        }
        if input.nodes[position].style.direction == TextDirection::Rtl {
            return Err(LayoutError::UnsupportedPositioning {
                node: id,
                reason: "relative·absolute inset은 현재 LTR subset만 지원합니다",
            });
        }
        for (field, value) in [
            ("top", positioning.inset.top),
            ("right", positioning.inset.right),
            ("bottom", positioning.inset.bottom),
            ("left", positioning.inset.left),
        ] {
            let valid = match value {
                LayoutLengthPercentage::Auto | LayoutLengthPercentage::Calc(_) => true,
                LayoutLengthPercentage::LengthPx(value)
                | LayoutLengthPercentage::Percentage(value) => value.is_finite(),
            };
            if !valid {
                return Err(LayoutError::InvalidPositioning { node: id, field });
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LayoutDisplay, LayoutNode, LayoutPositioning, LayoutStyle};

    #[test]
    fn nearest_relative_owner_is_inherited_and_hidden_boxes_have_no_owner() {
        let root = NodeId::new(1).unwrap();
        let relative = NodeId::new(2).unwrap();
        let nested_relative = NodeId::new(3).unwrap();
        let hidden = NodeId::new(4).unwrap();
        let input = LayoutInput {
            root,
            revision: Default::default(),
            viewport: crate::Viewport {
                width: 100.0,
                height: 100.0,
            },
            root_sizing: crate::RootSizingPolicy::Match,
            nodes: vec![
                LayoutNode {
                    id: root,
                    children: vec![relative, hidden],
                    style: LayoutStyle {
                        display: LayoutDisplay::Block,
                        width: crate::LayoutDimension::Fixed(100.0),
                        height: crate::LayoutDimension::Fixed(100.0),
                        ..LayoutStyle::default()
                    },
                },
                LayoutNode {
                    id: relative,
                    children: vec![nested_relative],
                    style: LayoutStyle {
                        display: LayoutDisplay::Block,
                        width: crate::LayoutDimension::Fixed(40.0),
                        height: crate::LayoutDimension::Fixed(40.0),
                        ..LayoutStyle::default()
                    },
                },
                LayoutNode {
                    id: nested_relative,
                    children: vec![],
                    style: LayoutStyle {
                        display: LayoutDisplay::Block,
                        width: crate::LayoutDimension::Fixed(10.0),
                        height: crate::LayoutDimension::Fixed(10.0),
                        ..LayoutStyle::default()
                    },
                },
                LayoutNode {
                    id: hidden,
                    children: vec![],
                    style: LayoutStyle {
                        display: LayoutDisplay::None,
                        ..LayoutStyle::default()
                    },
                },
            ],
            css_math: vec![],
            positioning: BTreeMap::from([
                (
                    relative,
                    LayoutPositioning {
                        position: LayoutPosition::Relative,
                        ..Default::default()
                    },
                ),
                (
                    nested_relative,
                    LayoutPositioning {
                        position: LayoutPosition::Relative,
                        ..Default::default()
                    },
                ),
            ]),
        };
        let index = input
            .nodes
            .iter()
            .enumerate()
            .map(|(position, node)| (node.id, position))
            .collect::<BTreeMap<_, _>>();

        let owners = collect_positioned_owners(&input, &index);

        assert_eq!(owners[&root], PositionedContainingBlockOwner::Viewport);
        assert_eq!(owners[&relative], PositionedContainingBlockOwner::Viewport);
        assert_eq!(
            owners[&nested_relative],
            PositionedContainingBlockOwner::Node(relative)
        );
        assert_eq!(owners[&hidden], PositionedContainingBlockOwner::NoBox);
    }
}
