use std::collections::BTreeMap;

use spinon_core::{EnvironmentRevision, NodeId, Revision, StyleRevision};

use crate::{
    LayoutBorder, LayoutBoxSizing, LayoutDimension, LayoutDisplay, LayoutEdges, LayoutEngine,
    LayoutInput, LayoutInputRevision, LayoutLengthPercentage, LayoutNode, LayoutPosition,
    LayoutPositioning, LayoutSourceRevision, LayoutStyle, PositionedContainingBlockOwner,
    RootSizingPolicy, TaffyLayoutEngine, Viewport,
};

fn id(value: u64) -> NodeId {
    NodeId::new(value).unwrap()
}

fn input(nodes: Vec<LayoutNode>, positioning: BTreeMap<NodeId, LayoutPositioning>) -> LayoutInput {
    LayoutInput {
        root: id(1),
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: 100.0,
            height: 100.0,
        },
        root_sizing: RootSizingPolicy::Match,
        nodes,
        css_math: vec![],
        positioning,
    }
}

fn block(id: NodeId, children: Vec<NodeId>, width: f32, height: f32) -> LayoutNode {
    LayoutNode {
        id,
        children,
        style: LayoutStyle {
            display: LayoutDisplay::Block,
            width: LayoutDimension::Fixed(width),
            height: LayoutDimension::Fixed(height),
            ..LayoutStyle::default()
        },
    }
}

fn absolute(left: LayoutLengthPercentage, top: LayoutLengthPercentage) -> LayoutPositioning {
    LayoutPositioning {
        position: LayoutPosition::Absolute,
        inset: LayoutEdges {
            left,
            top,
            ..LayoutEdges::auto()
        },
    }
}

#[test]
fn viewport_absolute_box_leaves_normal_flow_unchanged() {
    let root = id(1);
    let before = id(2);
    let target = id(3);
    let after = id(4);
    let result = TaffyLayoutEngine
        .compute(&input(
            vec![
                block(root, vec![before, target, after], 100.0, 100.0),
                block(before, vec![], 50.0, 10.0),
                block(target, vec![], 20.0, 12.0),
                block(after, vec![], 50.0, 15.0),
            ],
            BTreeMap::from([(
                target,
                absolute(
                    LayoutLengthPercentage::length(12.0),
                    LayoutLengthPercentage::length(20.0),
                ),
            )]),
        ))
        .unwrap();

    assert_eq!(
        (result.frames[&target].x, result.frames[&target].y),
        (12.0, 20.0)
    );
    assert_eq!(
        (result.flow_frames[&target].x, result.flow_frames[&target].y),
        (0.0, 10.0)
    );
    assert_eq!(result.frames[&after].y, 10.0);
    assert_eq!(result.flow_frames[&after].y, 22.0);
    assert_eq!(
        result.positioned_owners[&target],
        PositionedContainingBlockOwner::Viewport
    );
}

#[test]
fn positioned_owner_padding_edge_is_the_absolute_origin() {
    let root = id(1);
    let owner = id(2);
    let wrapper = id(3);
    let target = id(4);
    let mut owner_node = block(owner, vec![wrapper], 80.0, 60.0);
    owner_node.style.box_sizing = LayoutBoxSizing::BorderBox;
    owner_node.style.padding = LayoutEdges {
        top: LayoutLengthPercentage::length(10.0),
        right: LayoutLengthPercentage::length(10.0),
        bottom: LayoutLengthPercentage::length(10.0),
        left: LayoutLengthPercentage::length(10.0),
    };
    owner_node.style.border = LayoutBorder {
        top: 4.0,
        right: 4.0,
        bottom: 4.0,
        left: 4.0,
    };
    let result = TaffyLayoutEngine
        .compute(&input(
            vec![
                block(root, vec![owner], 100.0, 100.0),
                owner_node,
                block(wrapper, vec![target], 40.0, 20.0),
                block(target, vec![], 20.0, 10.0),
            ],
            BTreeMap::from([
                (
                    owner,
                    LayoutPositioning {
                        position: LayoutPosition::Relative,
                        ..Default::default()
                    },
                ),
                (
                    target,
                    absolute(
                        LayoutLengthPercentage::length(5.0),
                        LayoutLengthPercentage::length(7.0),
                    ),
                ),
            ]),
        ))
        .unwrap();

    assert_eq!(
        (result.frames[&target].x, result.frames[&target].y),
        (9.0, 11.0)
    );
    assert_eq!(
        result.positioned_owners[&target],
        PositionedContainingBlockOwner::Node(owner)
    );
    assert_eq!(
        (result.frames[&wrapper].x, result.frames[&wrapper].y),
        (14.0, 14.0)
    );
}

#[test]
fn automatic_insets_retain_static_position_when_source_parent_differs_from_owner() {
    let root = id(1);
    let owner = id(2);
    let wrapper = id(3);
    let before = id(4);
    let target = id(5);
    let after = id(6);
    let mut owner_node = block(owner, vec![wrapper], 90.0, 80.0);
    owner_node.style.padding.left = LayoutLengthPercentage::length(8.0);
    owner_node.style.padding.top = LayoutLengthPercentage::length(8.0);
    owner_node.style.border.left = 2.0;
    owner_node.style.border.top = 2.0;
    let mut target_node = block(target, vec![], 20.0, 10.0);
    target_node.style.margin.left = LayoutLengthPercentage::length(3.0);
    target_node.style.margin.top = LayoutLengthPercentage::length(2.0);
    let result = TaffyLayoutEngine
        .compute(&input(
            vec![
                block(root, vec![owner], 100.0, 100.0),
                owner_node,
                block(wrapper, vec![before, target, after], 60.0, 50.0),
                block(before, vec![], 40.0, 12.0),
                target_node,
                block(after, vec![], 30.0, 14.0),
            ],
            BTreeMap::from([
                (
                    owner,
                    LayoutPositioning {
                        position: LayoutPosition::Relative,
                        ..Default::default()
                    },
                ),
                (
                    target,
                    LayoutPositioning {
                        position: LayoutPosition::Absolute,
                        ..Default::default()
                    },
                ),
            ]),
        ))
        .unwrap();

    assert_eq!(
        result.positioned_owners[&target],
        PositionedContainingBlockOwner::Node(owner)
    );
    assert_eq!(
        (result.flow_frames[&target].x, result.flow_frames[&target].y),
        (13.0, 24.0)
    );
    assert_eq!(
        (result.frames[&target].x, result.frames[&target].y),
        (13.0, 24.0)
    );
    assert_eq!(result.frames[&after].y, 22.0);
    assert_eq!(result.flow_frames[&after].y, 34.0);
}

#[test]
fn absolute_ancestor_establishes_the_nested_containing_block() {
    let root = id(1);
    let owner = id(2);
    let target = id(3);
    let nested = id(4);
    let result = TaffyLayoutEngine
        .compute(&input(
            vec![
                block(root, vec![owner], 100.0, 100.0),
                block(owner, vec![target], 80.0, 60.0),
                block(target, vec![nested], 30.0, 20.0),
                block(nested, vec![], 10.0, 8.0),
            ],
            BTreeMap::from([
                (
                    owner,
                    LayoutPositioning {
                        position: LayoutPosition::Relative,
                        ..Default::default()
                    },
                ),
                (
                    target,
                    absolute(
                        LayoutLengthPercentage::length(15.0),
                        LayoutLengthPercentage::length(8.0),
                    ),
                ),
                (
                    nested,
                    absolute(
                        LayoutLengthPercentage::length(4.0),
                        LayoutLengthPercentage::length(3.0),
                    ),
                ),
            ]),
        ))
        .unwrap();

    assert_eq!(
        (result.frames[&target].x, result.frames[&target].y),
        (15.0, 8.0)
    );
    assert_eq!(
        (result.frames[&nested].x, result.frames[&nested].y),
        (19.0, 11.0)
    );
    assert_eq!(
        result.positioned_owners[&nested],
        PositionedContainingBlockOwner::Node(target)
    );
}

#[test]
fn hidden_absolute_subtree_is_not_promoted_to_the_viewport() {
    let root = id(1);
    let hidden = id(2);
    let target = id(3);
    let after = id(4);
    let mut hidden_node = block(hidden, vec![target], 40.0, 20.0);
    hidden_node.style.display = LayoutDisplay::None;
    let result = TaffyLayoutEngine
        .compute(&input(
            vec![
                block(root, vec![hidden, after], 100.0, 100.0),
                hidden_node,
                block(target, vec![], 20.0, 10.0),
                block(after, vec![], 30.0, 12.0),
            ],
            BTreeMap::from([
                (
                    hidden,
                    LayoutPositioning {
                        position: LayoutPosition::Relative,
                        ..Default::default()
                    },
                ),
                (
                    target,
                    absolute(
                        LayoutLengthPercentage::length(4.0),
                        LayoutLengthPercentage::length(3.0),
                    ),
                ),
            ]),
        ))
        .unwrap();

    assert_eq!(
        result.positioned_owners[&target],
        PositionedContainingBlockOwner::NoBox
    );
    assert_eq!(
        (result.frames[&target].width, result.frames[&target].height),
        (0.0, 0.0)
    );
    assert_eq!(
        (result.frames[&after].x, result.frames[&after].y),
        (0.0, 0.0)
    );
}

#[test]
fn absolute_percentages_use_positioned_owner_instead_of_source_parent() {
    let root = id(1);
    let owner = id(2);
    let wrapper = id(3);
    let target = id(4);
    let mut owner_node = block(owner, vec![wrapper], 80.0, 60.0);
    owner_node.style.box_sizing = LayoutBoxSizing::BorderBox;
    owner_node.style.padding = LayoutEdges {
        top: LayoutLengthPercentage::length(10.0),
        right: LayoutLengthPercentage::length(10.0),
        bottom: LayoutLengthPercentage::length(10.0),
        left: LayoutLengthPercentage::length(10.0),
    };
    owner_node.style.border = LayoutBorder {
        top: 4.0,
        right: 4.0,
        bottom: 4.0,
        left: 4.0,
    };
    let mut target_node = block(target, vec![], 20.0, 10.0);
    target_node.style.width = LayoutDimension::Percent(0.5);
    let result = TaffyLayoutEngine
        .compute(&input(
            vec![
                block(root, vec![owner], 100.0, 100.0),
                owner_node,
                block(wrapper, vec![target], 40.0, 20.0),
                target_node,
            ],
            BTreeMap::from([
                (
                    owner,
                    LayoutPositioning {
                        position: LayoutPosition::Relative,
                        ..Default::default()
                    },
                ),
                (
                    target,
                    absolute(
                        LayoutLengthPercentage::Percentage(0.5),
                        LayoutLengthPercentage::Percentage(0.5),
                    ),
                ),
            ]),
        ))
        .unwrap();

    assert_eq!(
        (
            result.frames[&target].x,
            result.frames[&target].y,
            result.frames[&target].width,
        ),
        (40.0, 30.0, 36.0)
    );
    assert_eq!(
        result.positioned_owners[&target],
        PositionedContainingBlockOwner::Node(owner)
    );
}

#[test]
fn absolute_percentage_height_rejects_an_indefinite_owner_height() {
    let root = id(1);
    let owner = id(2);
    let target = id(3);
    let mut owner_node = block(owner, vec![target], 80.0, 60.0);
    owner_node.style.height = LayoutDimension::Auto;
    let mut target_node = block(target, vec![], 20.0, 10.0);
    target_node.style.height = LayoutDimension::Percent(0.5);
    let error = TaffyLayoutEngine.compute(&input(
        vec![
            block(root, vec![owner], 100.0, 100.0),
            owner_node,
            target_node,
        ],
        BTreeMap::from([
            (
                owner,
                LayoutPositioning {
                    position: LayoutPosition::Relative,
                    ..Default::default()
                },
            ),
            (
                target,
                absolute(
                    LayoutLengthPercentage::length(0.0),
                    LayoutLengthPercentage::length(0.0),
                ),
            ),
        ]),
    ));

    assert!(matches!(
        error,
        Err(crate::LayoutError::IndefinitePercentageBasis {
            node,
            property: "height",
            axis: "height",
        }) if node == target
    ));
}
