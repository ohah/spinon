use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;
use spinon_core::{EnvironmentRevision, NodeId, Revision, StyleRevision};

use crate::{
    FlexDirection, FlexWrap, LayoutAlignItems, LayoutBorder, LayoutBoxSizing, LayoutDimension,
    LayoutDisplay, LayoutEdges, LayoutEngine, LayoutError, LayoutGap, LayoutInput,
    LayoutInputRevision, LayoutJustifyContent, LayoutLengthPercentage, LayoutSourceRevision,
    LayoutStyle, RootSizingPolicy, TaffyLayoutEngine, TextDirection, Viewport,
};

const REFERENCE: &str =
    include_str!("../../../../tests/fixtures/css/references/c10-flex-distribution-v1.json");
const MAX_ERROR_CSS_PX: f32 = 0.5;

#[derive(Debug, Deserialize)]
struct ReferenceFile {
    observations: Vec<Observation>,
}

#[derive(Debug, Deserialize)]
struct Observation {
    cases: Vec<CapturedCase>,
}

#[derive(Debug, Deserialize)]
struct CapturedCase {
    id: String,
    nodes: Vec<CapturedNode>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CapturedNode {
    id: String,
    children: Vec<String>,
    source_size: SourceSize,
    properties: BTreeMap<String, String>,
    rect: CapturedFrame,
}

#[derive(Debug, Deserialize)]
struct SourceSize {
    width: String,
    height: String,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
struct CapturedFrame {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

fn reference() -> ReferenceFile {
    serde_json::from_str(REFERENCE).expect("C10.2 Chromium reference JSON을 읽어야 합니다")
}

fn css_property<'a>(node: &'a CapturedNode, name: &str) -> &'a str {
    node.properties
        .get(name)
        .map(String::as_str)
        .unwrap_or_else(|| panic!("{} computed {name} 값이 없습니다", node.id))
}

fn css_number(node: &CapturedNode, property: &str) -> f32 {
    css_property(node, property)
        .strip_suffix("px")
        .unwrap_or_else(|| panic!("{} {property}는 CSS px여야 합니다", node.id))
        .parse()
        .unwrap_or_else(|_| panic!("{} {property} 숫자를 읽지 못했습니다", node.id))
}

fn dimension(value: &str) -> LayoutDimension {
    if value == "auto" || value == "none" {
        return LayoutDimension::Auto;
    }
    if let Some(percentage) = value.strip_suffix('%') {
        return LayoutDimension::Percent(
            percentage
                .parse::<f32>()
                .expect("CSS percentage 숫자를 읽어야 합니다")
                / 100.0,
        );
    }
    LayoutDimension::Fixed(
        value
            .strip_suffix("px")
            .unwrap_or_else(|| panic!("지원하지 않는 CSS dimension: {value}"))
            .parse()
            .expect("CSS px 숫자를 읽어야 합니다"),
    )
}

fn length_percentage(value: &str) -> LayoutLengthPercentage {
    if value == "normal" {
        return LayoutLengthPercentage::ZERO;
    }
    if value == "auto" {
        return LayoutLengthPercentage::Auto;
    }
    if let Some(percentage) = value.strip_suffix('%') {
        return LayoutLengthPercentage::percent(
            percentage
                .parse::<f32>()
                .expect("CSS percentage 숫자를 읽어야 합니다")
                / 100.0,
        );
    }
    LayoutLengthPercentage::length(
        value
            .strip_suffix("px")
            .unwrap_or_else(|| panic!("지원하지 않는 CSS length: {value}"))
            .parse()
            .expect("CSS px 숫자를 읽어야 합니다"),
    )
}

fn style(node: &CapturedNode) -> LayoutStyle {
    let display = match css_property(node, "display") {
        "flex" => LayoutDisplay::Flex,
        "block" => LayoutDisplay::Block,
        "flow-root" => LayoutDisplay::FlowRoot,
        "none" => LayoutDisplay::None,
        other => panic!("{} display가 지원 범위를 벗어났습니다: {other}", node.id),
    };
    let box_sizing = match css_property(node, "box-sizing") {
        "border-box" => LayoutBoxSizing::BorderBox,
        "content-box" => LayoutBoxSizing::ContentBox,
        other => panic!("{} box-sizing을 읽지 못했습니다: {other}", node.id),
    };
    let flex_direction = match css_property(node, "flex-direction") {
        "row" => FlexDirection::Row,
        "column" => FlexDirection::Column,
        other => panic!(
            "{} flex-direction이 지원 범위를 벗어났습니다: {other}",
            node.id
        ),
    };
    let flex_wrap = match css_property(node, "flex-wrap") {
        "nowrap" => FlexWrap::NoWrap,
        "wrap" => FlexWrap::Wrap,
        other => panic!("{} flex-wrap이 지원 범위를 벗어났습니다: {other}", node.id),
    };
    let direction = match css_property(node, "direction") {
        "ltr" => TextDirection::Ltr,
        "rtl" => TextDirection::Rtl,
        other => panic!("{} direction을 읽지 못했습니다: {other}", node.id),
    };
    let align_items = match css_property(node, "align-items") {
        "stretch" => LayoutAlignItems::Stretch,
        "flex-start" => LayoutAlignItems::FlexStart,
        "flex-end" => LayoutAlignItems::FlexEnd,
        "center" => LayoutAlignItems::Center,
        "normal" => LayoutAlignItems::Stretch,
        other => panic!(
            "{} align-items가 지원 범위를 벗어났습니다: {other}",
            node.id
        ),
    };
    let justify_content = match css_property(node, "justify-content") {
        "flex-start" | "normal" => LayoutJustifyContent::FlexStart,
        "flex-end" => LayoutJustifyContent::FlexEnd,
        "center" => LayoutJustifyContent::Center,
        "space-between" => LayoutJustifyContent::SpaceBetween,
        "space-around" => LayoutJustifyContent::SpaceAround,
        "space-evenly" => LayoutJustifyContent::SpaceEvenly,
        other => panic!(
            "{} justify-content가 지원 범위를 벗어났습니다: {other}",
            node.id
        ),
    };
    LayoutStyle {
        display,
        box_sizing,
        width: dimension(&node.source_size.width),
        height: dimension(&node.source_size.height),
        min_width: dimension(css_property(node, "min-width")),
        max_width: dimension(css_property(node, "max-width")),
        min_height: dimension(css_property(node, "min-height")),
        max_height: dimension(css_property(node, "max-height")),
        flex_basis: dimension(css_property(node, "flex-basis")),
        flex_direction,
        flex_wrap,
        direction,
        align_items,
        justify_content,
        margin: LayoutEdges {
            top: length_percentage(css_property(node, "margin-top")),
            right: length_percentage(css_property(node, "margin-right")),
            bottom: length_percentage(css_property(node, "margin-bottom")),
            left: length_percentage(css_property(node, "margin-left")),
        },
        padding: LayoutEdges {
            top: length_percentage(css_property(node, "padding-top")),
            right: length_percentage(css_property(node, "padding-right")),
            bottom: length_percentage(css_property(node, "padding-bottom")),
            left: length_percentage(css_property(node, "padding-left")),
        },
        border: LayoutBorder {
            top: css_number(node, "border-top-width"),
            right: css_number(node, "border-right-width"),
            bottom: css_number(node, "border-bottom-width"),
            left: css_number(node, "border-left-width"),
        },
        gap: LayoutGap {
            row: length_percentage(css_property(node, "row-gap")),
            column: length_percentage(css_property(node, "column-gap")),
        },
        flex_grow: css_property(node, "flex-grow")
            .parse()
            .expect("computed flex-grow 숫자를 읽어야 합니다"),
        flex_shrink: css_property(node, "flex-shrink")
            .parse()
            .expect("computed flex-shrink 숫자를 읽어야 합니다"),
        ..LayoutStyle::default()
    }
}

fn node_id_map(nodes: &[&CapturedNode]) -> BTreeMap<String, NodeId> {
    nodes
        .iter()
        .enumerate()
        .map(|(index, node)| {
            (
                node.id.clone(),
                NodeId::new(
                    u64::try_from(index + 1).expect("fixture node 수는 u64 범위여야 합니다"),
                )
                .expect("fixture node ID를 만들어야 합니다"),
            )
        })
        .collect()
}

fn descendants<'a>(
    root: &'a CapturedNode,
    by_id: &BTreeMap<&str, &'a CapturedNode>,
) -> Vec<&'a CapturedNode> {
    let mut output = Vec::new();
    let mut pending = vec![root.id.as_str()];
    while let Some(id) = pending.pop() {
        let node = by_id
            .get(id)
            .unwrap_or_else(|| panic!("reference child {id}가 case에 없습니다"));
        output.push(*node);
        pending.extend(node.children.iter().rev().map(String::as_str));
    }
    output
}

fn root_viewport(root: &CapturedNode) -> Viewport {
    let width = dimension(&root.source_size.width);
    let height = dimension(&root.source_size.height);
    match (width, height) {
        (LayoutDimension::Fixed(width), LayoutDimension::Fixed(height)) => {
            Viewport { width, height }
        }
        _ => panic!("{} root size는 definite px여야 합니다", root.id),
    }
}

fn layout_input_for_root(
    root: &CapturedNode,
    subtree: &[&CapturedNode],
) -> (LayoutInput, BTreeMap<String, NodeId>) {
    let ids = node_id_map(subtree);
    let nodes = subtree
        .iter()
        .map(|node| crate::LayoutNode {
            id: ids[&node.id],
            children: node.children.iter().map(|child| ids[child]).collect(),
            style: style(node),
        })
        .collect::<Vec<_>>();
    let root_id = ids[&root.id];
    (
        LayoutInput {
            root: root_id,
            revision: LayoutInputRevision::new(
                LayoutSourceRevision::Tree(Revision::default()),
                StyleRevision::default(),
                EnvironmentRevision::default(),
            ),
            viewport: root_viewport(root),
            root_sizing: RootSizingPolicy::Match,
            nodes,
            css_math: vec![],
        },
        ids,
    )
}

fn assert_case_matches_chromium(case: &CapturedCase) {
    let by_id = case
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect::<BTreeMap<_, _>>();
    let child_ids = case
        .nodes
        .iter()
        .flat_map(|node| node.children.iter().map(String::as_str))
        .collect::<BTreeSet<_>>();
    let roots = case
        .nodes
        .iter()
        .filter(|node| !child_ids.contains(node.id.as_str()))
        .collect::<Vec<_>>();
    assert!(
        !roots.is_empty(),
        "{} case에는 root가 있어야 합니다",
        case.id
    );

    for root in roots {
        let subtree = descendants(root, &by_id);
        let (input, ids) = layout_input_for_root(root, &subtree);
        let actual = TaffyLayoutEngine
            .compute(&input)
            .unwrap_or_else(|error| panic!("{} Taffy layout 실패: {error}", case.id));
        let root_reference = root.rect;
        for node in subtree {
            let actual_frame = actual.frames[&ids[&node.id]];
            let expected_frame = node.rect;
            for (field, actual_value, expected_value) in [
                ("x", actual_frame.x, expected_frame.x - root_reference.x),
                ("y", actual_frame.y, expected_frame.y - root_reference.y),
                ("width", actual_frame.width, expected_frame.width),
                ("height", actual_frame.height, expected_frame.height),
            ] {
                assert!(
                    (actual_value - expected_value).abs() <= MAX_ERROR_CSS_PX,
                    "{} {}.{field}: Taffy={actual_value}, Chromium={expected_value}, 허용치={MAX_ERROR_CSS_PX}",
                    case.id,
                    node.id
                );
            }
        }
    }
}

#[test]
fn invalid_flex_distribution_factors_fail_with_node_and_property_context() {
    let reference = reference();
    let case = reference.observations[0]
        .cases
        .iter()
        .find(|case| case.id == "grow-equal")
        .unwrap();
    let by_id = case
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect::<BTreeMap<_, _>>();
    let root = by_id["c102-grow-equal-root"];
    let subtree = descendants(root, &by_id);
    let (valid, ids) = layout_input_for_root(root, &subtree);
    let invalid_node = ids["c102-grow-equal-first"];

    for (field, value) in [
        ("flex_grow", -1.0),
        ("flex_grow", f32::NAN),
        ("flex_grow", f32::INFINITY),
        ("flex_shrink", -1.0),
        ("flex_shrink", f32::NAN),
        ("flex_shrink", f32::INFINITY),
    ] {
        let mut invalid = valid.clone();
        match field {
            "flex_grow" => invalid.nodes[1].style.flex_grow = value,
            "flex_shrink" => invalid.nodes[1].style.flex_shrink = value,
            _ => unreachable!("known flex factor"),
        }
        assert_eq!(
            TaffyLayoutEngine.compute(&invalid),
            Err(LayoutError::InvalidStyle {
                node: invalid_node,
                field,
            })
        );
    }
}

#[test]
fn taffy_matches_all_c102_chromium_flex_distribution_cases() {
    let reference = reference();
    assert_eq!(reference.observations.len(), 2);
    let dpr_one = &reference.observations[0];
    let dpr_two = &reference.observations[1];
    assert_eq!(
        dpr_one
            .cases
            .iter()
            .map(|case| &case.id)
            .collect::<Vec<_>>(),
        dpr_two
            .cases
            .iter()
            .map(|case| &case.id)
            .collect::<Vec<_>>(),
        "DPR별 case order가 같아야 합니다"
    );
    for (one, two) in dpr_one.cases.iter().zip(&dpr_two.cases) {
        assert_eq!(one.nodes.len(), two.nodes.len(), "{} node count", one.id);
        for (node_one, node_two) in one.nodes.iter().zip(&two.nodes) {
            assert_eq!(node_one.id, node_two.id, "{} node order", one.id);
            assert_eq!(node_one.rect, node_two.rect, "{} DPR geometry", one.id);
        }
        assert_case_matches_chromium(one);
    }
}
