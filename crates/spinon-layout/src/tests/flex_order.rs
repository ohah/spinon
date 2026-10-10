use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;
use spinon_core::{EnvironmentRevision, NodeId, Revision, StyleRevision};

use crate::{
    FlexDirection, FlexWrap, LayoutAlignItems, LayoutBorder, LayoutBoxSizing, LayoutDimension,
    LayoutDisplay, LayoutEdges, LayoutEngine, LayoutGap, LayoutInput, LayoutInputRevision,
    LayoutJustifyContent, LayoutLengthPercentage, LayoutNode, LayoutSourceRevision, LayoutStyle,
    RootSizingPolicy, TaffyLayoutEngine, TextDirection, Viewport,
};

const REFERENCE: &str =
    include_str!("../../../../tests/fixtures/css/references/c10-3-flex-order-alignment-v1.json");
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
struct CapturedNode {
    id: String,
    children: Vec<String>,
    properties: BTreeMap<String, String>,
    typed: TypedProperties,
    rect: CapturedFrame,
}

#[derive(Debug, Deserialize)]
struct TypedProperties {
    order: i32,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct CapturedFrame {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

fn reference() -> ReferenceFile {
    serde_json::from_str(REFERENCE).expect("C10.3.2 Chromium reference JSON을 읽어야 합니다")
}

fn property<'a>(node: &'a CapturedNode, name: &str) -> &'a str {
    node.properties
        .get(name)
        .map(String::as_str)
        .unwrap_or_else(|| panic!("{} computed {name} 값이 없습니다", node.id))
}

fn dimension(value: &str) -> LayoutDimension {
    if matches!(value, "auto" | "none") {
        return LayoutDimension::Auto;
    }
    if let Some(value) = value.strip_suffix('%') {
        return LayoutDimension::Percent(value.parse::<f32>().unwrap() / 100.0);
    }
    LayoutDimension::Fixed(
        value
            .strip_suffix("px")
            .unwrap_or_else(|| panic!("지원하지 않는 computed dimension: {value}"))
            .parse()
            .unwrap_or_else(|_| panic!("computed dimension 숫자를 읽지 못했습니다: {value}")),
    )
}

fn spacing(value: &str) -> LayoutLengthPercentage {
    if value == "normal" {
        return LayoutLengthPercentage::ZERO;
    }
    if value == "auto" {
        return LayoutLengthPercentage::Auto;
    }
    if let Some(value) = value.strip_suffix('%') {
        return LayoutLengthPercentage::percent(value.parse::<f32>().unwrap() / 100.0);
    }
    LayoutLengthPercentage::length(
        value
            .strip_suffix("px")
            .unwrap_or_else(|| panic!("지원하지 않는 computed spacing: {value}"))
            .parse()
            .unwrap_or_else(|_| panic!("computed spacing 숫자를 읽지 못했습니다: {value}")),
    )
}

fn border(value: &str) -> f32 {
    value
        .strip_suffix("px")
        .unwrap_or_else(|| panic!("computed border width가 CSS px가 아닙니다: {value}"))
        .parse()
        .unwrap_or_else(|_| panic!("computed border width를 읽지 못했습니다: {value}"))
}

fn style(node: &CapturedNode) -> LayoutStyle {
    let display = match property(node, "display") {
        "flex" => LayoutDisplay::Flex,
        "block" => LayoutDisplay::Block,
        "none" => LayoutDisplay::None,
        value => panic!("{} display가 범위를 벗어났습니다: {value}", node.id),
    };
    let box_sizing = match property(node, "box-sizing") {
        "border-box" => LayoutBoxSizing::BorderBox,
        "content-box" => LayoutBoxSizing::ContentBox,
        value => panic!("{} box-sizing을 읽지 못했습니다: {value}", node.id),
    };
    let flex_direction = match property(node, "flex-direction") {
        "row" => FlexDirection::Row,
        "column" => FlexDirection::Column,
        "row-reverse" => FlexDirection::RowReverse,
        "column-reverse" => FlexDirection::ColumnReverse,
        value => panic!("{} flex-direction을 읽지 못했습니다: {value}", node.id),
    };
    let flex_wrap = match property(node, "flex-wrap") {
        "nowrap" => FlexWrap::NoWrap,
        "wrap" => FlexWrap::Wrap,
        "wrap-reverse" => FlexWrap::WrapReverse,
        value => panic!("{} flex-wrap을 읽지 못했습니다: {value}", node.id),
    };
    let direction = match property(node, "direction") {
        "ltr" => TextDirection::Ltr,
        "rtl" => TextDirection::Rtl,
        value => panic!("{} direction을 읽지 못했습니다: {value}", node.id),
    };
    let align_items = match property(node, "align-items") {
        "normal" | "stretch" => LayoutAlignItems::Stretch,
        "flex-start" => LayoutAlignItems::FlexStart,
        "flex-end" => LayoutAlignItems::FlexEnd,
        "center" => LayoutAlignItems::Center,
        value => panic!("{} align-items을 읽지 못했습니다: {value}", node.id),
    };
    let justify_content = match property(node, "justify-content") {
        "normal" | "flex-start" => LayoutJustifyContent::FlexStart,
        "flex-end" => LayoutJustifyContent::FlexEnd,
        "center" => LayoutJustifyContent::Center,
        "space-between" => LayoutJustifyContent::SpaceBetween,
        "space-around" => LayoutJustifyContent::SpaceAround,
        "space-evenly" => LayoutJustifyContent::SpaceEvenly,
        value => panic!("{} justify-content을 읽지 못했습니다: {value}", node.id),
    };
    let computed_number = |name| {
        property(node, name)
            .parse::<f32>()
            .unwrap_or_else(|_| panic!("{} {name} 숫자를 읽지 못했습니다", node.id))
    };

    LayoutStyle {
        display,
        box_sizing,
        width: dimension(property(node, "width")),
        height: dimension(property(node, "height")),
        min_width: dimension(property(node, "min-width")),
        max_width: dimension(property(node, "max-width")),
        min_height: dimension(property(node, "min-height")),
        max_height: dimension(property(node, "max-height")),
        order: node.typed.order,
        flex_basis: dimension(property(node, "flex-basis")),
        flex_direction,
        flex_wrap,
        direction,
        align_items,
        justify_content,
        margin: LayoutEdges {
            top: spacing(property(node, "margin-top")),
            right: spacing(property(node, "margin-right")),
            bottom: spacing(property(node, "margin-bottom")),
            left: spacing(property(node, "margin-left")),
        },
        padding: LayoutEdges {
            top: spacing(property(node, "padding-top")),
            right: spacing(property(node, "padding-right")),
            bottom: spacing(property(node, "padding-bottom")),
            left: spacing(property(node, "padding-left")),
        },
        border: LayoutBorder {
            top: border(property(node, "border-top-width")),
            right: border(property(node, "border-right-width")),
            bottom: border(property(node, "border-bottom-width")),
            left: border(property(node, "border-left-width")),
        },
        gap: LayoutGap {
            row: spacing(property(node, "row-gap")),
            column: spacing(property(node, "column-gap")),
        },
        flex_grow: computed_number("flex-grow"),
        flex_shrink: computed_number("flex-shrink"),
        ..LayoutStyle::default()
    }
}

fn layout_input(case: &CapturedCase) -> LayoutInput {
    let ids = case
        .nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (node.id.as_str(), NodeId::new(index as u64 + 1).unwrap()))
        .collect::<BTreeMap<_, _>>();
    let nodes = case
        .nodes
        .iter()
        .map(|node| LayoutNode {
            id: ids[node.id.as_str()],
            children: node
                .children
                .iter()
                .map(|child| ids[child.as_str()])
                .collect(),
            style: style(node),
        })
        .collect();
    LayoutInput {
        root: NodeId::new(1).unwrap(),
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: 320.0,
            height: 240.0,
        },
        root_sizing: RootSizingPolicy::ResolveWithin,
        nodes,
        css_math: Vec::new(),
        positioning: Default::default(),
    }
}

#[test]
fn order_modified_layout_matches_every_chromium_node_and_keeps_source_children() {
    let reference = reference();
    assert_eq!(reference.observations.len(), 2);
    let observation = &reference.observations[0];
    let mut seen = BTreeSet::new();
    for case in &observation.cases {
        assert!(
            seen.insert(case.id.as_str()),
            "{} fixture case 중복",
            case.id
        );
        let input = layout_input(case);
        for (input_node, captured) in input.nodes().iter().zip(&case.nodes) {
            assert_eq!(
                input_node.children,
                captured
                    .children
                    .iter()
                    .map(|id| {
                        NodeId::new(
                            case.nodes.iter().position(|node| node.id == *id).unwrap() as u64 + 1,
                        )
                        .unwrap()
                    })
                    .collect::<Vec<_>>(),
                "{}의 계산용 입력도 source child order를 유지합니다",
                captured.id,
            );
        }
        let actual = TaffyLayoutEngine.compute(&input).unwrap();
        for (index, captured) in case.nodes.iter().enumerate() {
            let id = NodeId::new(index as u64 + 1).unwrap();
            let frame = actual
                .frames
                .get(&id)
                .unwrap_or_else(|| panic!("{}:{}의 Taffy frame이 없습니다", case.id, captured.id));
            for (field, actual) in [
                ("x", frame.x),
                ("y", frame.y),
                ("width", frame.width),
                ("height", frame.height),
            ] {
                let expected = match field {
                    "x" => captured.rect.x,
                    "y" => captured.rect.y,
                    "width" => captured.rect.width,
                    "height" => captured.rect.height,
                    _ => unreachable!(),
                };
                assert!(
                    (actual - expected).abs() <= MAX_ERROR_CSS_PX,
                    "{}:{}.{field}: Taffy {actual}, Chrome {expected}, 허용치 {MAX_ERROR_CSS_PX} CSS px",
                    case.id,
                    captured.id,
                );
            }
        }
    }
    assert_eq!(seen.len(), 11);
}
