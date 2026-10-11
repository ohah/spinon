use std::collections::BTreeMap;

use spinon_core::{EnvironmentRevision, NodeId, Revision, StyleRevision};

use crate::{
    FixedContainingBlockOwner, LayoutDimension, LayoutDisplay, LayoutEdges, LayoutEngine,
    LayoutInput, LayoutInputRevision, LayoutLengthPercentage, LayoutNode, LayoutPosition,
    LayoutPositioning, LayoutSourceRevision, LayoutStyle, PositionedContainingBlockOwner,
    RootSizingPolicy, TaffyLayoutEngine, Viewport,
};

fn id(value: u64) -> NodeId {
    NodeId::new(value).unwrap()
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
        css_math: Vec::new(),
        positioning,
    }
}

fn fixed(left: LayoutLengthPercentage, top: LayoutLengthPercentage) -> LayoutPositioning {
    LayoutPositioning {
        position: LayoutPosition::Fixed,
        inset: LayoutEdges {
            left,
            top,
            ..LayoutEdges::auto()
        },
    }
}

#[test]
fn fixed_descendants_use_viewport_while_absolute_descendants_use_fixed_box() {
    let root = id(1);
    let wrapper = id(2);
    let fixed_parent = id(3);
    let absolute_child = id(4);
    let fixed_child = id(5);
    let mut wrapper_node = block(wrapper, vec![fixed_parent], 40.0, 30.0);
    wrapper_node.style.margin = LayoutEdges {
        left: LayoutLengthPercentage::length(28.0),
        top: LayoutLengthPercentage::length(19.0),
        ..LayoutEdges::auto()
    };
    let result = TaffyLayoutEngine
        .compute(&input(
            vec![
                block(root, vec![wrapper], 100.0, 100.0),
                wrapper_node,
                block(fixed_parent, vec![absolute_child, fixed_child], 40.0, 30.0),
                block(absolute_child, Vec::new(), 8.0, 6.0),
                block(fixed_child, Vec::new(), 7.0, 5.0),
            ],
            BTreeMap::from([
                (
                    fixed_parent,
                    fixed(
                        LayoutLengthPercentage::length(20.0),
                        LayoutLengthPercentage::length(10.0),
                    ),
                ),
                (
                    absolute_child,
                    LayoutPositioning {
                        position: LayoutPosition::Absolute,
                        inset: LayoutEdges {
                            left: LayoutLengthPercentage::length(3.0),
                            top: LayoutLengthPercentage::length(4.0),
                            ..LayoutEdges::auto()
                        },
                    },
                ),
                (
                    fixed_child,
                    fixed(
                        LayoutLengthPercentage::length(60.0),
                        LayoutLengthPercentage::length(50.0),
                    ),
                ),
            ]),
        ))
        .unwrap();

    assert_eq!(
        (
            result.frames[&fixed_parent].x,
            result.frames[&fixed_parent].y
        ),
        (20.0, 10.0)
    );
    assert_eq!(
        (
            result.frames[&absolute_child].x,
            result.frames[&absolute_child].y
        ),
        (23.0, 14.0)
    );
    assert_eq!(
        (result.frames[&fixed_child].x, result.frames[&fixed_child].y),
        (60.0, 50.0)
    );
    assert_eq!(
        result.positioned_owners[&absolute_child],
        PositionedContainingBlockOwner::Node(fixed_parent)
    );
    assert_eq!(
        result.fixed_owners[&fixed_parent],
        FixedContainingBlockOwner::Viewport
    );
    assert_eq!(
        result.fixed_owners[&fixed_child],
        FixedContainingBlockOwner::Viewport
    );
}

#[test]
fn fixed_percentages_use_viewport_axes_instead_of_source_parent_size() {
    let root = id(1);
    let wrapper = id(2);
    let target = id(3);
    let mut target_node = block(target, Vec::new(), 1.0, 10.0);
    target_node.style.width = LayoutDimension::Percent(0.5);
    let result = TaffyLayoutEngine
        .compute(&input(
            vec![
                block(root, vec![wrapper], 100.0, 100.0),
                block(wrapper, vec![target], 40.0, 30.0),
                target_node,
            ],
            BTreeMap::from([(
                target,
                fixed(
                    LayoutLengthPercentage::Percentage(0.25),
                    LayoutLengthPercentage::Percentage(0.10),
                ),
            )]),
        ))
        .unwrap();

    assert_eq!(
        (
            result.frames[&target].x,
            result.frames[&target].y,
            result.frames[&target].width,
            result.frames[&target].height,
        ),
        (25.0, 10.0, 50.0, 10.0)
    );
}

#[test]
fn hidden_fixed_subtree_is_not_promoted_to_viewport_root() {
    let root = id(1);
    let hidden = id(2);
    let target = id(3);
    let mut hidden_node = block(hidden, vec![target], 40.0, 20.0);
    hidden_node.style.display = LayoutDisplay::None;
    let result = TaffyLayoutEngine
        .compute(&input(
            vec![
                block(root, vec![hidden], 100.0, 100.0),
                hidden_node,
                block(target, Vec::new(), 20.0, 10.0),
            ],
            BTreeMap::from([(
                target,
                fixed(
                    LayoutLengthPercentage::length(4.0),
                    LayoutLengthPercentage::length(3.0),
                ),
            )]),
        ))
        .unwrap();

    assert_eq!(
        result.fixed_owners[&target],
        FixedContainingBlockOwner::NoBox
    );
    assert_eq!(
        (result.frames[&target].width, result.frames[&target].height),
        (0.0, 0.0)
    );
}

#[test]
fn fixed_static_position_and_fixed_root_fail_closed() {
    let root = id(1);
    let target = id(2);
    let nodes = vec![
        block(root, vec![target], 100.0, 100.0),
        block(target, Vec::new(), 10.0, 10.0),
    ];
    let no_horizontal_insets = LayoutPositioning {
        position: LayoutPosition::Fixed,
        inset: LayoutEdges {
            top: LayoutLengthPercentage::length(4.0),
            ..LayoutEdges::auto()
        },
    };
    assert!(matches!(
        TaffyLayoutEngine.compute(&input(
            nodes.clone(),
            BTreeMap::from([(target, no_horizontal_insets)]),
        )),
        Err(crate::LayoutError::UnsupportedPositioning { node, .. }) if node == target
    ));
    assert!(matches!(
        TaffyLayoutEngine.compute(&input(
            vec![block(root, Vec::new(), 100.0, 100.0)],
            BTreeMap::from([(root, fixed(LayoutLengthPercentage::length(1.0), LayoutLengthPercentage::length(1.0)))]),
        )),
        Err(crate::LayoutError::UnsupportedPositioning { node, .. }) if node == root
    ));
}
