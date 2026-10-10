use std::collections::BTreeMap;

use serde::Deserialize;
use spinon_core::{EnvironmentRevision, NodeId, Revision, StyleRevision};

use crate::{
    FlexDirection, FlexWrap, LayoutAlignItems, LayoutDimension, LayoutDisplay, LayoutEdges,
    LayoutEngine, LayoutGap, LayoutInput, LayoutInputRevision, LayoutJustifyContent,
    LayoutLengthPercentage, LayoutNode, LayoutSourceRevision, LayoutStyle, RootSizingPolicy,
    TaffyLayoutEngine, TextDirection, Viewport,
};

const REFERENCE: &str =
    include_str!("../../../../tests/fixtures/css/references/c10-3-1-flex-reverse-v1.json");
const MAX_ERROR_CSS_PX: f32 = 0.5;

#[derive(Debug, Deserialize)]
struct ReferenceFile {
    observations: Vec<Observation>,
}

#[derive(Debug, Deserialize)]
struct Observation {
    viewport: CapturedViewport,
    cases: Vec<CapturedCase>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CapturedViewport {
    device_scale_factor: f32,
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
    serde_json::from_str(REFERENCE).expect("C10.3.1 Chromium reference JSON을 읽어야 합니다")
}

fn property<'a>(node: &'a CapturedNode, name: &str) -> &'a str {
    node.properties
        .get(name)
        .map(String::as_str)
        .unwrap_or_else(|| panic!("{} computed {name} 값이 없습니다", node.id))
}

fn px_dimension(value: &str) -> LayoutDimension {
    LayoutDimension::Fixed(
        value
            .strip_suffix("px")
            .unwrap_or_else(|| panic!("fixture dimension이 CSS px가 아닙니다: {value}"))
            .parse()
            .unwrap_or_else(|_| panic!("fixture dimension 숫자를 읽지 못했습니다: {value}")),
    )
}

fn px_gap(value: &str) -> LayoutLengthPercentage {
    match value {
        "normal" => LayoutLengthPercentage::ZERO,
        value => LayoutLengthPercentage::length(
            value
                .strip_suffix("px")
                .unwrap_or_else(|| panic!("fixture gap이 CSS px가 아닙니다: {value}"))
                .parse()
                .unwrap_or_else(|_| panic!("fixture gap 숫자를 읽지 못했습니다: {value}")),
        ),
    }
}

fn flex_direction(node: &CapturedNode) -> FlexDirection {
    match property(node, "flex-direction") {
        "row" => FlexDirection::Row,
        "column" => FlexDirection::Column,
        "row-reverse" => FlexDirection::RowReverse,
        "column-reverse" => FlexDirection::ColumnReverse,
        other => panic!("{} flex-direction이 범위를 벗어났습니다: {other}", node.id),
    }
}

fn flex_wrap(node: &CapturedNode) -> FlexWrap {
    match property(node, "flex-wrap") {
        "nowrap" => FlexWrap::NoWrap,
        "wrap" => FlexWrap::Wrap,
        "wrap-reverse" => FlexWrap::WrapReverse,
        other => panic!("{} flex-wrap이 범위를 벗어났습니다: {other}", node.id),
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
        .map(|node| {
            let display = match property(node, "display") {
                "flex" => LayoutDisplay::Flex,
                "block" => LayoutDisplay::Block,
                "none" => LayoutDisplay::None,
                other => panic!("{} display가 범위를 벗어났습니다: {other}", node.id),
            };
            let direction = match property(node, "direction") {
                "ltr" => TextDirection::Ltr,
                "rtl" => TextDirection::Rtl,
                other => panic!("{} direction을 읽지 못했습니다: {other}", node.id),
            };
            let align_items = match property(node, "align-items") {
                "flex-start" => LayoutAlignItems::FlexStart,
                "flex-end" => LayoutAlignItems::FlexEnd,
                "center" => LayoutAlignItems::Center,
                "stretch" | "normal" => LayoutAlignItems::Stretch,
                other => panic!("{} align-items가 범위를 벗어났습니다: {other}", node.id),
            };
            let justify_content = match property(node, "justify-content") {
                "flex-start" | "normal" => LayoutJustifyContent::FlexStart,
                "flex-end" => LayoutJustifyContent::FlexEnd,
                "center" => LayoutJustifyContent::Center,
                other => panic!("{} justify-content가 범위를 벗어났습니다: {other}", node.id),
            };
            LayoutNode {
                id: ids[node.id.as_str()],
                children: node
                    .children
                    .iter()
                    .map(|child| ids[child.as_str()])
                    .collect(),
                style: LayoutStyle {
                    display,
                    width: px_dimension(&node.source_size.width),
                    height: px_dimension(&node.source_size.height),
                    flex_direction: flex_direction(node),
                    flex_wrap: flex_wrap(node),
                    direction,
                    align_items,
                    justify_content,
                    margin: LayoutEdges {
                        top: px_gap(property(node, "margin-top")),
                        right: px_gap(property(node, "margin-right")),
                        bottom: px_gap(property(node, "margin-bottom")),
                        left: px_gap(property(node, "margin-left")),
                    },
                    gap: LayoutGap {
                        row: px_gap(property(node, "row-gap")),
                        column: px_gap(property(node, "column-gap")),
                    },
                    flex_grow: 0.0,
                    flex_shrink: 0.0,
                    ..LayoutStyle::default()
                },
            }
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
    }
}

fn assert_frame(actual: crate::LayoutFrame, expected: CapturedFrame, context: &str) {
    for (field, actual, expected) in [
        ("x", actual.x, expected.x),
        ("y", actual.y, expected.y),
        ("width", actual.width, expected.width),
        ("height", actual.height, expected.height),
    ] {
        assert!(
            (actual - expected).abs() <= MAX_ERROR_CSS_PX,
            "{context} {field}: 실제 {actual}, Chromium {expected}, 허용치 {MAX_ERROR_CSS_PX}"
        );
    }
}

#[test]
fn flex_reverse_layout_matches_every_chromium_node_at_both_device_scale_factors() {
    let reference = reference();
    assert_eq!(reference.observations.len(), 2);
    let expected_scale_factors = [1.0, 2.0];
    for (observation, expected_scale) in reference.observations.iter().zip(expected_scale_factors) {
        assert_eq!(observation.viewport.device_scale_factor, expected_scale);
        for case in &observation.cases {
            let output = TaffyLayoutEngine
                .compute(&layout_input(case))
                .unwrap_or_else(|error| panic!("{} layout 실패: {error}", case.id));
            assert_eq!(output.frames.len(), case.nodes.len(), "{} node 수", case.id);
            for (index, expected) in case.nodes.iter().enumerate() {
                let node_id = NodeId::new(index as u64 + 1).unwrap();
                let actual =
                    output.frames.get(&node_id).copied().unwrap_or_else(|| {
                        panic!("{} node {} frame이 없습니다", case.id, expected.id)
                    });
                assert_frame(
                    actual,
                    expected.rect,
                    &format!("{} / {}", case.id, expected.id),
                );
            }
        }
    }
}

#[test]
fn reverse_geometry_keeps_source_child_vectors_unchanged() {
    let reference = reference();
    let cases = &reference.observations[0].cases;
    for (case_id, root_id, expected_children) in [
        (
            "row-reverse-start",
            "c1031-row-start-root",
            vec![
                "c1031-row-start-first",
                "c1031-row-start-second",
                "c1031-row-start-third",
            ],
        ),
        (
            "row-reverse-wrap-reverse",
            "c1031-row-combined-root",
            vec![
                "c1031-row-combined-first",
                "c1031-row-combined-second",
                "c1031-row-combined-third",
                "c1031-row-combined-fourth",
                "c1031-row-combined-fifth",
            ],
        ),
        (
            "column-reverse-wrap-reverse",
            "c1031-column-combined-root",
            vec![
                "c1031-column-combined-first",
                "c1031-column-combined-second",
                "c1031-column-combined-third",
            ],
        ),
    ] {
        let case = cases
            .iter()
            .find(|case| case.id == case_id)
            .unwrap_or_else(|| panic!("reference에 {case_id}가 없습니다"));
        let root = case
            .nodes
            .iter()
            .find(|node| node.id == root_id)
            .unwrap_or_else(|| panic!("{case_id} root가 없습니다"));
        assert_eq!(root.children, expected_children, "{case_id} source order");
    }
}

#[test]
fn reference_covers_reverse_shorthand_override_and_wrap_boundaries() {
    let reference = reference();
    let cases = &reference.observations[0].cases;
    let case = |id: &str| {
        cases
            .iter()
            .find(|case| case.id == id)
            .unwrap_or_else(|| panic!("reference에 {id}가 없습니다"))
    };
    let root = |case_id: &str| &case(case_id).nodes[0];
    assert_eq!(
        property(root("flex-flow-direction-first"), "flex-direction"),
        "row-reverse"
    );
    assert_eq!(
        property(root("flex-flow-direction-first"), "flex-wrap"),
        "wrap"
    );
    assert_eq!(
        property(root("flex-flow-wrap-first"), "flex-direction"),
        "row-reverse"
    );
    assert_eq!(property(root("flex-flow-wrap-first"), "flex-wrap"), "wrap");
    assert_eq!(
        property(root("flex-flow-longhand-override"), "flex-direction"),
        "row"
    );
    assert_eq!(
        property(root("flex-flow-longhand-override"), "flex-wrap"),
        "wrap"
    );

    let exact = case("row-reverse-exact-fit");
    assert_eq!(exact.nodes[1].rect.x, 45.0);
    assert_eq!(exact.nodes[2].rect.x, 0.0);
    let over = case("row-reverse-one-pixel-over");
    assert_eq!(over.nodes[1].rect.y, 0.0);
    assert_eq!(over.nodes[2].rect.y, 10.0);
}
