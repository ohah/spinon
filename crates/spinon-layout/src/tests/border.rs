use super::node_id;
use crate::{
    FlexDirection, LayoutBorder, LayoutBoxSizing, LayoutDimension, LayoutDisplay, LayoutEngine,
    LayoutError, LayoutInput, LayoutInputRevision, LayoutNode, LayoutSourceRevision, LayoutStyle,
    RootSizingPolicy, TaffyLayoutEngine, Viewport,
};
use spinon_core::{EnvironmentRevision, Revision, StyleRevision};

fn border_input() -> LayoutInput {
    let root = node_id(911);
    let content_box = node_id(912);
    let border_box = node_id(913);
    LayoutInput {
        root,
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: 320.0,
            height: 100.0,
        },
        root_sizing: RootSizingPolicy::Match,
        css_math: vec![],
        nodes: vec![
            LayoutNode {
                id: root,
                children: vec![content_box, border_box],
                style: LayoutStyle {
                    width: LayoutDimension::Fixed(320.0),
                    height: LayoutDimension::Fixed(100.0),
                    display: LayoutDisplay::Flex,
                    flex_direction: FlexDirection::Row,
                    ..LayoutStyle::default()
                },
            },
            LayoutNode {
                id: content_box,
                children: vec![],
                style: LayoutStyle {
                    width: LayoutDimension::Fixed(100.0),
                    height: LayoutDimension::Fixed(20.0),
                    box_sizing: LayoutBoxSizing::ContentBox,
                    padding: crate::LayoutEdges {
                        left: crate::LayoutLengthPercentage::length(5.0),
                        right: crate::LayoutLengthPercentage::length(7.0),
                        ..crate::LayoutEdges::default()
                    },
                    border: LayoutBorder {
                        top: 1.0,
                        right: 4.0,
                        bottom: 2.0,
                        left: 3.0,
                    },
                    ..LayoutStyle::default()
                },
            },
            LayoutNode {
                id: border_box,
                children: vec![],
                style: LayoutStyle {
                    width: LayoutDimension::Fixed(100.0),
                    height: LayoutDimension::Fixed(20.0),
                    box_sizing: LayoutBoxSizing::BorderBox,
                    padding: crate::LayoutEdges {
                        left: crate::LayoutLengthPercentage::length(5.0),
                        right: crate::LayoutLengthPercentage::length(7.0),
                        ..crate::LayoutEdges::default()
                    },
                    border: LayoutBorder {
                        top: 1.0,
                        right: 4.0,
                        bottom: 2.0,
                        left: 3.0,
                    },
                    ..LayoutStyle::default()
                },
            },
        ],
    }
}

#[test]
fn border_width_is_included_in_content_and_border_box_geometry() {
    let output = TaffyLayoutEngine.compute(&border_input()).unwrap();
    let content_box = output.frames[&node_id(912)];
    let border_box = output.frames[&node_id(913)];

    assert_eq!(content_box.width, 119.0);
    assert_eq!(content_box.height, 23.0);
    assert_eq!(border_box.x, 119.0);
    assert_eq!(border_box.width, 100.0);
    assert_eq!(border_box.height, 20.0);
}

#[test]
fn negative_and_nonfinite_border_widths_fail_before_taffy() {
    let node = node_id(912);
    for (field, value) in [
        ("border.top", -1.0),
        ("border.right", f32::NAN),
        ("border.bottom", f32::INFINITY),
        ("border.left", f32::NEG_INFINITY),
    ] {
        let mut input = border_input();
        let content_box = input
            .nodes
            .iter_mut()
            .find(|entry| entry.id == node)
            .unwrap();
        match field {
            "border.top" => content_box.style.border.top = value,
            "border.right" => content_box.style.border.right = value,
            "border.bottom" => content_box.style.border.bottom = value,
            "border.left" => content_box.style.border.left = value,
            _ => unreachable!(),
        }
        assert_eq!(
            TaffyLayoutEngine.compute(&input),
            Err(LayoutError::InvalidStyle { node, field }),
            "{field}={value}를 거부해야 합니다"
        );
    }
}
