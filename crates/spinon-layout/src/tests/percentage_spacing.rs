use super::node_id;
use crate::{
    FlexDirection, LayoutDisplay, LayoutEdges, LayoutEngine, LayoutError, LayoutGap, LayoutInput,
    LayoutInputRevision, LayoutLengthPercentage, LayoutNode, LayoutSourceRevision, LayoutStyle,
    RootSizingPolicy, TaffyLayoutEngine, Viewport,
};
use spinon_core::{EnvironmentRevision, Revision, StyleRevision};

#[test]
fn non_finite_spacing_and_negative_padding_or_gap_fail_with_the_property() {
    let child_id = node_id(382);
    for (field, value) in [
        ("margin.top", f32::NAN),
        ("padding.left", f32::INFINITY),
        ("padding.top", -1.0),
        ("gap.row", f32::NEG_INFINITY),
        ("gap.column", -0.01),
    ] {
        let mut input = spacing_input(LayoutStyle::default());
        let child = input
            .nodes
            .iter_mut()
            .find(|node| node.id == child_id)
            .unwrap();
        match field {
            "margin.top" => child.style.margin.top = LayoutLengthPercentage::length(value),
            "padding.left" => child.style.padding.left = LayoutLengthPercentage::length(value),
            "padding.top" => child.style.padding.top = LayoutLengthPercentage::length(value),
            "gap.row" => child.style.gap.row = LayoutLengthPercentage::length(value),
            "gap.column" => child.style.gap.column = LayoutLengthPercentage::length(value),
            _ => unreachable!(),
        }
        assert_eq!(
            TaffyLayoutEngine.compute(&input),
            Err(LayoutError::InvalidStyle {
                node: child_id,
                field,
            }),
            "{field}={value}는 Taffy 입력으로 전달되면 안 됩니다"
        );
    }

    let mut input = spacing_input(LayoutStyle::default());
    input.nodes[1].style.margin.top = LayoutLengthPercentage::length(-5.0);
    assert!(TaffyLayoutEngine.compute(&input).is_ok());
}

#[test]
fn root_gap_percentage_is_rejected_before_taffy_resolution() {
    let root_id = node_id(391);
    let input = LayoutInput {
        root: root_id,
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: 320.0,
            height: 800.0,
        },
        root_sizing: RootSizingPolicy::Match,
        css_math: vec![],
        positioning: Default::default(),
        nodes: vec![LayoutNode {
            id: root_id,
            children: vec![],
            style: LayoutStyle {
                width: crate::LayoutDimension::Fixed(320.0),
                height: crate::LayoutDimension::Fixed(800.0),
                display: LayoutDisplay::Flex,
                gap: LayoutGap {
                    row: LayoutLengthPercentage::percent(0.1),
                    ..LayoutGap::default()
                },
                ..LayoutStyle::default()
            },
        }],
    };

    assert_eq!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::UnsupportedRootPercentageSpacing {
            node: root_id,
            property: "row-gap",
        })
    );
}

#[test]
fn percentage_edges_with_unprovable_containing_width_fail_closed() {
    let root = node_id(401);
    let auto_flex_parent = node_id(402);
    let child = node_id(403);
    let input = LayoutInput {
        root,
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: 320.0,
            height: 800.0,
        },
        root_sizing: RootSizingPolicy::Match,
        css_math: vec![],
        positioning: Default::default(),
        nodes: vec![
            LayoutNode {
                id: root,
                children: vec![auto_flex_parent],
                style: LayoutStyle {
                    width: crate::LayoutDimension::Fixed(320.0),
                    height: crate::LayoutDimension::Fixed(800.0),
                    display: LayoutDisplay::Flex,
                    flex_direction: FlexDirection::Row,
                    ..LayoutStyle::default()
                },
            },
            LayoutNode {
                id: auto_flex_parent,
                children: vec![child],
                style: LayoutStyle {
                    height: crate::LayoutDimension::Fixed(80.0),
                    display: LayoutDisplay::Flex,
                    flex_direction: FlexDirection::Row,
                    ..LayoutStyle::default()
                },
            },
            LayoutNode {
                id: child,
                children: vec![],
                style: LayoutStyle {
                    width: crate::LayoutDimension::Fixed(20.0),
                    height: crate::LayoutDimension::Fixed(10.0),
                    margin: LayoutEdges {
                        left: LayoutLengthPercentage::percent(0.1),
                        ..LayoutEdges::default()
                    },
                    ..LayoutStyle::default()
                },
            },
        ],
    };

    assert_eq!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::IndefinitePercentageBasis {
            node: child,
            property: "margin-left",
            axis: "containing block width",
        })
    );
}

#[test]
fn flow_root_containing_block_keeps_nested_auto_block_width_definite() {
    let root = node_id(411);
    let flow_root = node_id(412);
    let auto_block = node_id(413);
    let leaf = node_id(414);
    let input = LayoutInput {
        root,
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: 320.0,
            height: 240.0,
        },
        root_sizing: RootSizingPolicy::Match,
        css_math: vec![],
        positioning: Default::default(),
        nodes: vec![
            LayoutNode {
                id: root,
                children: vec![flow_root],
                style: LayoutStyle {
                    display: LayoutDisplay::Block,
                    width: crate::LayoutDimension::Fixed(320.0),
                    height: crate::LayoutDimension::Fixed(240.0),
                    ..LayoutStyle::default()
                },
            },
            LayoutNode {
                id: flow_root,
                children: vec![auto_block],
                style: LayoutStyle {
                    display: LayoutDisplay::FlowRoot,
                    width: crate::LayoutDimension::Fixed(200.0),
                    ..LayoutStyle::default()
                },
            },
            LayoutNode {
                id: auto_block,
                children: vec![leaf],
                style: LayoutStyle {
                    display: LayoutDisplay::Block,
                    ..LayoutStyle::default()
                },
            },
            LayoutNode {
                id: leaf,
                children: vec![],
                style: LayoutStyle {
                    display: LayoutDisplay::Block,
                    width: crate::LayoutDimension::Fixed(40.0),
                    height: crate::LayoutDimension::Fixed(10.0),
                    margin: LayoutEdges {
                        left: LayoutLengthPercentage::percent(0.1),
                        ..LayoutEdges::default()
                    },
                    ..LayoutStyle::default()
                },
            },
        ],
    };

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    assert_eq!(output.frames[&flow_root].width, 200.0);
    assert_eq!(output.frames[&auto_block].width, 200.0);
    assert_eq!(output.frames[&leaf].x, 20.0);
}

#[test]
fn block_formatting_root_flow_root_uses_viewport_width_as_definite_basis() {
    let root = node_id(421);
    let auto_block = node_id(422);
    let leaf = node_id(423);
    let input = LayoutInput {
        root,
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: 320.0,
            height: 240.0,
        },
        root_sizing: RootSizingPolicy::BlockFormatting,
        css_math: vec![],
        positioning: Default::default(),
        nodes: vec![
            LayoutNode {
                id: root,
                children: vec![auto_block],
                style: LayoutStyle {
                    display: LayoutDisplay::FlowRoot,
                    ..LayoutStyle::default()
                },
            },
            LayoutNode {
                id: auto_block,
                children: vec![leaf],
                style: LayoutStyle {
                    display: LayoutDisplay::Block,
                    ..LayoutStyle::default()
                },
            },
            LayoutNode {
                id: leaf,
                children: vec![],
                style: LayoutStyle {
                    display: LayoutDisplay::Block,
                    width: crate::LayoutDimension::Fixed(40.0),
                    height: crate::LayoutDimension::Fixed(10.0),
                    margin: LayoutEdges {
                        left: LayoutLengthPercentage::percent(0.1),
                        ..LayoutEdges::default()
                    },
                    ..LayoutStyle::default()
                },
            },
        ],
    };

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    assert_eq!(output.frames[&root].width, 320.0);
    assert_eq!(output.frames[&auto_block].width, 320.0);
    assert_eq!(output.frames[&leaf].x, 32.0);
}

#[test]
fn childless_flow_root_uses_the_leaf_layout_path() {
    let root = node_id(431);
    let leaf = node_id(432);
    let input = LayoutInput {
        root,
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: 320.0,
            height: 240.0,
        },
        root_sizing: RootSizingPolicy::Match,
        css_math: vec![],
        positioning: Default::default(),
        nodes: vec![
            LayoutNode {
                id: root,
                children: vec![leaf],
                style: LayoutStyle {
                    display: LayoutDisplay::Block,
                    width: crate::LayoutDimension::Fixed(320.0),
                    height: crate::LayoutDimension::Fixed(240.0),
                    ..LayoutStyle::default()
                },
            },
            LayoutNode {
                id: leaf,
                children: vec![],
                style: LayoutStyle {
                    display: LayoutDisplay::FlowRoot,
                    width: crate::LayoutDimension::Fixed(60.0),
                    height: crate::LayoutDimension::Fixed(40.0),
                    ..LayoutStyle::default()
                },
            },
        ],
    };

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    assert_eq!(
        (output.frames[&leaf].width, output.frames[&leaf].height),
        (60.0, 40.0)
    );
}

#[test]
fn finite_percentage_that_overflows_geometry_is_rejected_before_commit() {
    let child = node_id(382);
    let mut input = spacing_input(LayoutStyle {
        width: crate::LayoutDimension::Fixed(20.0),
        height: crate::LayoutDimension::Fixed(10.0),
        ..LayoutStyle::default()
    });
    input.nodes[0].style.display = LayoutDisplay::Flex;
    input.nodes[1].style.margin.left = LayoutLengthPercentage::percent(f32::MAX);

    assert_eq!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::NonFiniteFrame(child))
    );
}

fn spacing_input(child_style: LayoutStyle) -> LayoutInput {
    let root = node_id(381);
    let child = node_id(382);
    LayoutInput {
        root,
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: 320.0,
            height: 800.0,
        },
        root_sizing: RootSizingPolicy::Match,
        css_math: vec![],
        positioning: Default::default(),
        nodes: vec![
            LayoutNode {
                id: root,
                children: vec![child],
                style: LayoutStyle {
                    width: crate::LayoutDimension::Fixed(320.0),
                    height: crate::LayoutDimension::Fixed(800.0),
                    display: LayoutDisplay::Block,
                    ..LayoutStyle::default()
                },
            },
            LayoutNode {
                id: child,
                children: vec![],
                style: child_style,
            },
        ],
    }
}
