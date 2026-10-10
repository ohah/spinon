use super::*;
use crate::{LayoutBoxSizing, LayoutDisplay};

#[test]
fn percentage_dimensions_resolve_against_parent_content_box_without_clamping() {
    let root = node_id(301);
    let parent = node_id(302);
    let half = node_id(303);
    let over = node_id(304);
    let zero = node_id(305);
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
                children: vec![parent],
                style: LayoutStyle {
                    width: LayoutDimension::Fixed(320.0),
                    height: LayoutDimension::Fixed(800.0),
                    display: LayoutDisplay::Block,
                    ..LayoutStyle::default()
                },
            },
            LayoutNode {
                id: parent,
                children: vec![half, over, zero],
                style: LayoutStyle {
                    width: LayoutDimension::Fixed(100.0),
                    height: LayoutDimension::Fixed(80.0),
                    box_sizing: LayoutBoxSizing::ContentBox,
                    display: LayoutDisplay::Block,
                    padding: LayoutEdges {
                        top: LayoutLengthPercentage::length(10.0),
                        right: LayoutLengthPercentage::length(10.0),
                        bottom: LayoutLengthPercentage::length(10.0),
                        left: LayoutLengthPercentage::length(10.0),
                    },
                    ..LayoutStyle::default()
                },
            },
            percentage_node(half, 0.5, 0.5),
            percentage_node(over, 1.25, 1.25),
            percentage_node(zero, 0.0, 0.0),
        ],
    };

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    let parent_frame = output.frames[&parent];
    let half_frame = output.frames[&half];
    let over_frame = output.frames[&over];
    let zero_frame = output.frames[&zero];
    assert_eq!((parent_frame.width, parent_frame.height), (120.0, 100.0));
    assert_eq!((half_frame.x, half_frame.y), (10.0, 10.0));
    assert_eq!((half_frame.width, half_frame.height), (50.0, 40.0));
    assert_eq!((over_frame.width, over_frame.height), (125.0, 100.0));
    assert_eq!((zero_frame.width, zero_frame.height), (0.0, 0.0));
}

#[test]
fn percentage_flex_basis_uses_the_container_main_axis() {
    let root = node_id(311);
    let row = node_id(312);
    let row_half = node_id(313);
    let row_quarter = node_id(314);
    let column = node_id(315);
    let column_half = node_id(316);
    let column_quarter = node_id(317);
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
            fixed_node_with_children(root, &[row, column], 320.0, 800.0),
            LayoutNode {
                id: row,
                children: vec![row_half, row_quarter],
                style: LayoutStyle {
                    width: LayoutDimension::Fixed(120.0),
                    height: LayoutDimension::Fixed(40.0),
                    display: LayoutDisplay::Flex,
                    flex_direction: FlexDirection::Row,
                    ..LayoutStyle::default()
                },
            },
            flex_basis_node(row_half, 0.5, 20.0, 40.0),
            flex_basis_node(row_quarter, 0.25, 20.0, 40.0),
            LayoutNode {
                id: column,
                children: vec![column_half, column_quarter],
                style: LayoutStyle {
                    width: LayoutDimension::Fixed(80.0),
                    height: LayoutDimension::Fixed(60.0),
                    display: LayoutDisplay::Flex,
                    flex_direction: FlexDirection::Column,
                    ..LayoutStyle::default()
                },
            },
            flex_basis_node(column_half, 0.5, 80.0, 10.0),
            flex_basis_node(column_quarter, 0.25, 80.0, 10.0),
        ],
    };

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    assert_eq!(output.frames[&row_half].width, 60.0);
    assert_eq!(output.frames[&row_quarter].x, 60.0);
    assert_eq!(output.frames[&row_quarter].width, 30.0);
    assert_eq!(output.frames[&column_half].height, 30.0);
    assert_eq!(
        output.frames[&column_quarter].y - output.frames[&column].y,
        30.0
    );
    assert_eq!(output.frames[&column_quarter].height, 15.0);
}

#[test]
fn viewport_resolving_root_accepts_full_and_partial_percentage_dimensions() {
    let root = node_id(321);
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
        root_sizing: RootSizingPolicy::ResolveWithin,
        css_math: vec![],
        positioning: Default::default(),
        nodes: vec![LayoutNode {
            id: root,
            children: vec![],
            style: LayoutStyle {
                width: LayoutDimension::Percent(1.0),
                height: LayoutDimension::Percent(0.5),
                display: LayoutDisplay::Block,
                ..LayoutStyle::default()
            },
        }],
    };
    let output = TaffyLayoutEngine.compute(&input).unwrap();
    assert_eq!(output.frames[&root].width, 320.0);
    assert_eq!(output.frames[&root].height, 400.0);

    let mut fixed_root_input = input;
    fixed_root_input.root_sizing = RootSizingPolicy::Match;
    assert_eq!(
        TaffyLayoutEngine.compute(&fixed_root_input),
        Err(LayoutError::RootSizeMismatch { axis: "width" })
    );
}

#[test]
fn percentage_margin_and_padding_keep_the_containing_block_width_basis() {
    let root = node_id(341);
    let parent = node_id(342);
    let child = node_id(343);
    let mut child_style = LayoutStyle {
        width: LayoutDimension::Fixed(20.0),
        height: LayoutDimension::Fixed(10.0),
        box_sizing: LayoutBoxSizing::ContentBox,
        display: LayoutDisplay::Block,
        margin: LayoutEdges {
            top: LayoutLengthPercentage::percent(0.1),
            right: LayoutLengthPercentage::percent(0.1),
            bottom: LayoutLengthPercentage::percent(0.1),
            left: LayoutLengthPercentage::percent(0.1),
        },
        padding: LayoutEdges {
            top: LayoutLengthPercentage::percent(0.1),
            right: LayoutLengthPercentage::percent(0.1),
            bottom: LayoutLengthPercentage::percent(0.1),
            left: LayoutLengthPercentage::percent(0.1),
        },
        ..LayoutStyle::default()
    };
    child_style.margin.left = LayoutLengthPercentage::percent(-0.05);
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
            fixed_node_with_children(root, &[parent], 320.0, 800.0),
            LayoutNode {
                id: parent,
                children: vec![child],
                style: LayoutStyle {
                    width: LayoutDimension::Fixed(200.0),
                    height: LayoutDimension::Fixed(100.0),
                    display: LayoutDisplay::Flex,
                    ..LayoutStyle::default()
                },
            },
            LayoutNode {
                id: child,
                children: vec![],
                style: child_style,
            },
        ],
    };

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    let frame = output.frames[&child];
    assert_eq!((frame.x, frame.y), (-10.0, 20.0));
    assert_eq!((frame.width, frame.height), (60.0, 50.0));
}

#[test]
fn percentage_gap_uses_the_definite_main_axis_content_size() {
    let root = node_id(351);
    let row = node_id(352);
    let row_first = node_id(353);
    let row_second = node_id(354);
    let column = node_id(355);
    let column_first = node_id(356);
    let column_second = node_id(357);
    let row_cross_gap = node_id(358);
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
            fixed_node_with_children(root, &[row, column, row_cross_gap], 320.0, 800.0),
            LayoutNode {
                id: row,
                children: vec![row_first, row_second],
                style: LayoutStyle {
                    width: LayoutDimension::Fixed(200.0),
                    height: LayoutDimension::Fixed(60.0),
                    flex_direction: FlexDirection::Row,
                    gap: LayoutGap {
                        column: LayoutLengthPercentage::percent(0.1),
                        ..LayoutGap::default()
                    },
                    ..LayoutStyle::default()
                },
            },
            fixed_node_with_children(row_first, &[], 20.0, 10.0),
            fixed_node_with_children(row_second, &[], 20.0, 10.0),
            LayoutNode {
                id: column,
                children: vec![column_first, column_second],
                style: LayoutStyle {
                    width: LayoutDimension::Fixed(80.0),
                    height: LayoutDimension::Fixed(100.0),
                    flex_direction: FlexDirection::Column,
                    gap: LayoutGap {
                        row: LayoutLengthPercentage::percent(0.1),
                        ..LayoutGap::default()
                    },
                    ..LayoutStyle::default()
                },
            },
            fixed_node_with_children(column_first, &[], 10.0, 10.0),
            fixed_node_with_children(column_second, &[], 10.0, 10.0),
            LayoutNode {
                id: row_cross_gap,
                children: vec![],
                style: LayoutStyle {
                    width: LayoutDimension::Fixed(200.0),
                    flex_direction: FlexDirection::Row,
                    gap: LayoutGap {
                        row: LayoutLengthPercentage::percent(0.5),
                        ..LayoutGap::default()
                    },
                    ..LayoutStyle::default()
                },
            },
        ],
    };

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    assert_eq!(
        output.frames[&row_second].x - output.frames[&row_first].x,
        40.0
    );
    assert_eq!(
        output.frames[&column_second].y - output.frames[&column_first].y,
        20.0
    );
}

#[test]
fn cyclic_column_gap_fails_with_node_property_and_axis() {
    let root = node_id(361);
    let column = node_id(362);
    let child = node_id(363);
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
            fixed_node_with_children(root, &[column], 320.0, 800.0),
            LayoutNode {
                id: column,
                children: vec![child],
                style: LayoutStyle {
                    width: LayoutDimension::Fixed(100.0),
                    display: LayoutDisplay::Flex,
                    flex_direction: FlexDirection::Column,
                    gap: LayoutGap {
                        row: LayoutLengthPercentage::percent(0.1),
                        ..LayoutGap::default()
                    },
                    ..LayoutStyle::default()
                },
            },
            fixed_node_with_children(child, &[], 20.0, 10.0),
        ],
    };
    assert_eq!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::IndefinitePercentageBasis {
            node: column,
            property: "row-gap",
            axis: "height",
        })
    );
}

#[test]
fn percentage_dimensions_reject_negative_and_non_finite_values_but_allow_overflow() {
    let valid = simple_percent_input(1.25);
    assert!(TaffyLayoutEngine.compute(&valid).is_ok());

    for value in [-0.1, f32::NAN, f32::INFINITY] {
        let invalid = simple_percent_input(value);
        assert_eq!(
            TaffyLayoutEngine.compute(&invalid),
            Err(LayoutError::InvalidStyle {
                node: node_id(332),
                field: "width",
            })
        );
    }
}

fn percentage_node(id: NodeId, width: f32, height: f32) -> LayoutNode {
    LayoutNode {
        id,
        children: vec![],
        style: LayoutStyle {
            width: LayoutDimension::Percent(width),
            height: LayoutDimension::Percent(height),
            display: LayoutDisplay::Block,
            ..LayoutStyle::default()
        },
    }
}

fn flex_basis_node(id: NodeId, basis: f32, width: f32, height: f32) -> LayoutNode {
    LayoutNode {
        id,
        children: vec![],
        style: LayoutStyle {
            width: LayoutDimension::Fixed(width),
            height: LayoutDimension::Fixed(height),
            flex_basis: LayoutDimension::Percent(basis),
            ..LayoutStyle::default()
        },
    }
}

fn fixed_node_with_children(
    id: NodeId,
    children: &[NodeId],
    width: f32,
    height: f32,
) -> LayoutNode {
    LayoutNode {
        id,
        children: children.to_vec(),
        style: LayoutStyle {
            width: LayoutDimension::Fixed(width),
            height: LayoutDimension::Fixed(height),
            display: LayoutDisplay::Block,
            ..LayoutStyle::default()
        },
    }
}

fn simple_percent_input(value: f32) -> LayoutInput {
    let root = node_id(331);
    let child = node_id(332);
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
                    width: LayoutDimension::Fixed(320.0),
                    height: LayoutDimension::Fixed(800.0),
                    ..LayoutStyle::default()
                },
            },
            LayoutNode {
                id: child,
                children: vec![],
                style: LayoutStyle {
                    width: LayoutDimension::Percent(value),
                    height: LayoutDimension::Auto,
                    ..LayoutStyle::default()
                },
            },
        ],
    }
}
