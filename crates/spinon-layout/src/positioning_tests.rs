use std::collections::BTreeMap;

use spinon_core::{EnvironmentRevision, NodeId, Revision, StyleRevision};

use crate::{
    FlexDirection, LayoutDimension, LayoutDisplay, LayoutEdges, LayoutEngine, LayoutInput,
    LayoutInputRevision, LayoutLengthPercentage, LayoutNode, LayoutPosition, LayoutPositioning,
    LayoutSourceRevision, LayoutStyle, PositionedContainingBlockOwner, RootSizingPolicy,
    TaffyLayoutEngine, TextDirection, Viewport,
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

fn fixed_block(id: NodeId, children: Vec<NodeId>, width: f32, height: f32) -> LayoutNode {
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

#[test]
fn relative_flex_item_moves_visually_without_moving_its_sibling() {
    let root = id(1);
    let row = id(2);
    let target = id(3);
    let sibling = id(4);
    let mut root_node = fixed_block(root, vec![row], 100.0, 100.0);
    root_node.style.flex_direction = FlexDirection::Row;
    let mut row_node = fixed_block(row, vec![target, sibling], 80.0, 40.0);
    row_node.style.display = LayoutDisplay::Flex;
    row_node.style.flex_direction = FlexDirection::Row;
    let target_node = fixed_block(target, vec![], 20.0, 10.0);
    let sibling_node = fixed_block(sibling, vec![], 20.0, 10.0);
    let result = TaffyLayoutEngine
        .compute(&input(
            vec![root_node, row_node, target_node, sibling_node],
            BTreeMap::from([(
                target,
                LayoutPositioning {
                    position: LayoutPosition::Relative,
                    inset: LayoutEdges {
                        left: LayoutLengthPercentage::length(12.0),
                        top: LayoutLengthPercentage::length(-3.0),
                        ..LayoutEdges::auto()
                    },
                },
            )]),
        ))
        .unwrap();

    let target_flow = result.flow_frames[&target];
    let target_visual = result.frames[&target];
    assert_eq!((target_flow.x, target_flow.y), (0.0, 0.0));
    assert_eq!((target_visual.x, target_visual.y), (12.0, -3.0));
    assert_eq!(result.flow_frames[&sibling].x, 20.0);
    assert_eq!(result.frames[&sibling].x, 20.0);
    assert_eq!(
        result.positioned_owners[&target],
        PositionedContainingBlockOwner::Viewport
    );
}

#[test]
fn static_insets_are_ignored_and_nested_relative_offsets_propagate_once() {
    let root = id(1);
    let parent = id(2);
    let child = id(3);
    let grandchild = id(4);
    let after = id(5);
    let nodes = vec![
        fixed_block(root, vec![parent, after], 100.0, 100.0),
        fixed_block(parent, vec![child], 40.0, 20.0),
        fixed_block(child, vec![grandchild], 20.0, 10.0),
        fixed_block(grandchild, vec![], 5.0, 5.0),
        fixed_block(after, vec![], 10.0, 10.0),
    ];
    let result = TaffyLayoutEngine
        .compute(&input(
            nodes.clone(),
            BTreeMap::from([
                (
                    parent,
                    LayoutPositioning {
                        position: LayoutPosition::Relative,
                        inset: LayoutEdges {
                            left: LayoutLengthPercentage::length(7.0),
                            top: LayoutLengthPercentage::length(5.0),
                            ..LayoutEdges::auto()
                        },
                    },
                ),
                (
                    child,
                    LayoutPositioning {
                        position: LayoutPosition::Relative,
                        inset: LayoutEdges {
                            left: LayoutLengthPercentage::length(3.0),
                            top: LayoutLengthPercentage::length(2.0),
                            ..LayoutEdges::auto()
                        },
                    },
                ),
            ]),
        ))
        .unwrap();
    assert_eq!(
        (result.frames[&parent].x, result.frames[&parent].y),
        (7.0, 5.0)
    );
    assert_eq!(
        (result.frames[&child].x, result.frames[&child].y),
        (10.0, 7.0)
    );
    assert_eq!(
        (
            result.flow_frames[&grandchild].x,
            result.flow_frames[&grandchild].y
        ),
        (0.0, 0.0)
    );
    assert_eq!(
        (result.frames[&grandchild].x, result.frames[&grandchild].y),
        (10.0, 7.0)
    );
    assert_eq!(result.flow_frames[&after].y, 20.0);
    assert_eq!(result.frames[&after].y, 20.0);

    let mut static_position = LayoutPositioning::default();
    static_position.inset.left = LayoutLengthPercentage::length(15.0);
    static_position.inset.top = LayoutLengthPercentage::length(6.0);
    let static_result = TaffyLayoutEngine
        .compute(&input(nodes, BTreeMap::from([(child, static_position)])))
        .unwrap();
    assert_eq!(static_result.frames, static_result.flow_frames);
}

#[test]
fn relative_percentage_insets_use_the_containing_block_axis() {
    let root = id(1);
    let parent = id(2);
    let child = id(3);
    let result = TaffyLayoutEngine
        .compute(&input(
            vec![
                fixed_block(root, vec![parent], 100.0, 100.0),
                fixed_block(parent, vec![child], 80.0, 60.0),
                fixed_block(child, vec![], 20.0, 10.0),
            ],
            BTreeMap::from([(
                child,
                LayoutPositioning {
                    position: LayoutPosition::Relative,
                    inset: LayoutEdges {
                        left: LayoutLengthPercentage::percent(0.1),
                        top: LayoutLengthPercentage::percent(0.25),
                        ..LayoutEdges::auto()
                    },
                },
            )]),
        ))
        .unwrap();

    assert_eq!(result.frames[&child].x, 8.0);
    assert_eq!(result.frames[&child].y, 15.0);
}

#[test]
fn relative_layout_root_uses_the_viewport_as_its_percentage_basis() {
    let root = id(1);
    let child = id(2);
    let result = TaffyLayoutEngine
        .compute(&input(
            vec![
                fixed_block(root, vec![child], 100.0, 100.0),
                fixed_block(child, vec![], 20.0, 10.0),
            ],
            BTreeMap::from([(
                root,
                LayoutPositioning {
                    position: LayoutPosition::Relative,
                    inset: LayoutEdges {
                        left: LayoutLengthPercentage::percent(0.1),
                        top: LayoutLengthPercentage::percent(0.25),
                        ..LayoutEdges::auto()
                    },
                },
            )]),
        ))
        .unwrap();

    assert_eq!(
        (result.flow_frames[&root].x, result.flow_frames[&root].y),
        (0.0, 0.0)
    );
    assert_eq!(
        (result.frames[&root].x, result.frames[&root].y),
        (10.0, 25.0)
    );
    assert_eq!(
        (result.flow_frames[&child].x, result.flow_frames[&child].y),
        (0.0, 0.0)
    );
    assert_eq!(
        (result.frames[&child].x, result.frames[&child].y),
        (10.0, 25.0)
    );
}

#[test]
fn malformed_position_inputs_fail_closed_before_frame_publication() {
    let root = id(1);
    let child = id(2);
    let nodes = vec![
        fixed_block(root, vec![child], 100.0, 100.0),
        fixed_block(child, vec![], 20.0, 10.0),
    ];

    let unknown_node = TaffyLayoutEngine.compute(&input(
        nodes.clone(),
        BTreeMap::from([(id(3), LayoutPositioning::default())]),
    ));
    assert_eq!(
        unknown_node,
        Err(crate::LayoutError::UnknownPositioningNode(id(3)))
    );

    for (field, value) in [("top", f32::NAN), ("left", f32::INFINITY)] {
        let inset = match field {
            "top" => LayoutEdges {
                top: LayoutLengthPercentage::length(value),
                ..LayoutEdges::auto()
            },
            "left" => LayoutEdges {
                left: LayoutLengthPercentage::percent(value),
                ..LayoutEdges::auto()
            },
            _ => unreachable!("테스트 표에만 쓰는 inset 속성입니다"),
        };
        let malformed = TaffyLayoutEngine.compute(&input(
            nodes.clone(),
            BTreeMap::from([(
                child,
                LayoutPositioning {
                    position: LayoutPosition::Relative,
                    inset,
                },
            )]),
        ));
        assert_eq!(
            malformed,
            Err(crate::LayoutError::InvalidPositioning { node: child, field })
        );
    }

    let mut rtl_child = fixed_block(child, vec![], 20.0, 10.0);
    rtl_child.style.direction = TextDirection::Rtl;
    let rtl = TaffyLayoutEngine.compute(&input(
        vec![fixed_block(root, vec![child], 100.0, 100.0), rtl_child],
        BTreeMap::from([(
            child,
            LayoutPositioning {
                position: LayoutPosition::Relative,
                inset: LayoutEdges {
                    left: LayoutLengthPercentage::length(1.0),
                    ..LayoutEdges::auto()
                },
            },
        )]),
    ));
    assert!(matches!(
        rtl,
        Err(crate::LayoutError::UnsupportedPositioning { node, .. }) if node == child
    ));
}

#[test]
fn absolute_child_of_flex_source_parent_fails_closed_until_c1035() {
    let root = id(1);
    let flex_parent = id(2);
    let target = id(3);
    let mut flex_node = fixed_block(flex_parent, vec![target], 80.0, 40.0);
    flex_node.style.display = LayoutDisplay::Flex;
    let result = TaffyLayoutEngine.compute(&input(
        vec![
            fixed_block(root, vec![flex_parent], 100.0, 100.0),
            flex_node,
            fixed_block(target, vec![], 20.0, 10.0),
        ],
        BTreeMap::from([(
            target,
            LayoutPositioning {
                position: LayoutPosition::Absolute,
                inset: LayoutEdges {
                    left: LayoutLengthPercentage::length(4.0),
                    top: LayoutLengthPercentage::length(3.0),
                    ..LayoutEdges::auto()
                },
            },
        )]),
    ));

    assert!(matches!(
        result,
        Err(crate::LayoutError::UnsupportedPositioning { node, reason })
            if node == target && reason.contains("C10.3.5")
    ));
}

#[test]
fn absolute_child_with_flex_containing_block_fails_closed_until_c1035() {
    let root = id(1);
    let wrapper = id(2);
    let target = id(3);
    let mut root_node = fixed_block(root, vec![wrapper], 100.0, 100.0);
    root_node.style.display = LayoutDisplay::Flex;
    let result = TaffyLayoutEngine.compute(&input(
        vec![
            root_node,
            fixed_block(wrapper, vec![target], 40.0, 20.0),
            fixed_block(target, vec![], 20.0, 10.0),
        ],
        BTreeMap::from([
            (
                root,
                LayoutPositioning {
                    position: LayoutPosition::Relative,
                    ..Default::default()
                },
            ),
            (
                target,
                LayoutPositioning {
                    position: LayoutPosition::Absolute,
                    inset: LayoutEdges {
                        left: LayoutLengthPercentage::length(4.0),
                        top: LayoutLengthPercentage::length(3.0),
                        ..LayoutEdges::auto()
                    },
                },
            ),
        ]),
    ));

    assert!(matches!(
        result,
        Err(crate::LayoutError::UnsupportedPositioning { node, reason })
            if node == target && reason.contains("C10.3.5")
    ));
}

#[test]
fn absolute_layout_root_fails_closed_before_a_partial_frame_map_exists() {
    let root = id(1);
    let result = TaffyLayoutEngine.compute(&input(
        vec![fixed_block(root, vec![], 100.0, 100.0)],
        BTreeMap::from([(
            root,
            LayoutPositioning {
                position: LayoutPosition::Absolute,
                inset: LayoutEdges {
                    left: LayoutLengthPercentage::length(4.0),
                    top: LayoutLengthPercentage::length(3.0),
                    ..LayoutEdges::auto()
                },
            },
        )]),
    ));

    assert!(matches!(
        result,
        Err(crate::LayoutError::UnsupportedPositioning { node, .. }) if node == root
    ));
}
