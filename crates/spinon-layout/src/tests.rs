use std::collections::BTreeMap;

use serde::Deserialize;
use spinon_core::{ChangeBatch, EnvironmentRevision, NodeId, Revision, StyleRevision, Tree};

use crate::{
    FlexDirection, LayoutDimension, LayoutEdges, LayoutEngine, LayoutError, LayoutGap, LayoutInput,
    LayoutInputRevision, LayoutLengthPercentage, LayoutNode, LayoutSourceRevision, LayoutStyle,
    RootSizingPolicy, TaffyLayoutEngine, TextDirection, Viewport,
};

#[path = "tests/invalid_inputs.rs"]
mod invalid_inputs;
#[path = "tests/legacy_oracle.rs"]
mod legacy_oracle;
#[path = "tests/margin.rs"]
mod margin;
#[path = "tests/percentage_spacing.rs"]
mod percentage_spacing;
#[path = "tests/percentages.rs"]
mod percentages;
#[path = "tests/typed_math.rs"]
mod typed_math;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Fixture {
    version: u32,
    viewport: FixtureViewport,
    root: u64,
    nodes: Vec<FixtureNode>,
    #[serde(default)]
    expected: BTreeMap<String, FixtureFrame>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FixtureViewport {
    width: f32,
    height: f32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FixtureNode {
    id: u64,
    tag: String,
    children: Vec<u64>,
    style: FixtureStyle,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct FixtureStyle {
    width: FixtureDimension,
    height: FixtureDimension,
    flex_direction: String,
    direction: String,
    padding: FixtureEdges,
    row_gap: f32,
    column_gap: f32,
    flex_grow: f32,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum FixtureDimension {
    Number(f32),
    Keyword(String),
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FixtureEdges {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct FixtureFrame {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

const HTML_FIXTURE: &str = include_str!("../tests/fixtures/s02-basic-flex.html");
const FIXTURE_START: &str = "<script id=\"fixture-data\" type=\"application/json\">";
const RTL_FIXTURE_START: &str = "<script id=\"rtl-fixture-data\" type=\"application/json\">";
const FRACTIONAL_FIXTURE_START: &str =
    "<script id=\"fractional-fixture-data\" type=\"application/json\">";
const FIXTURE_END: &str = "</script>";
const TOLERANCE: f32 = 0.5;
const FRACTIONAL_TOLERANCE: f32 = 0.01;

fn fixture() -> Fixture {
    parse_fixture(FIXTURE_START)
}

fn rtl_fixture() -> Fixture {
    parse_fixture(RTL_FIXTURE_START)
}

fn fractional_fixture() -> Fixture {
    parse_fixture(FRACTIONAL_FIXTURE_START)
}

fn parse_fixture(marker: &str) -> Fixture {
    let start = HTML_FIXTURE.find(marker).unwrap() + marker.len();
    let end = start + HTML_FIXTURE[start..].find(FIXTURE_END).unwrap();
    let fixture: Fixture = serde_json::from_str(&HTML_FIXTURE[start..end]).unwrap();
    assert_eq!(
        fixture.version, 1,
        "테스트 코드가 fixture 버전을 처리해야 합니다"
    );
    fixture
}

fn to_input(fixture: &Fixture) -> LayoutInput {
    LayoutInput {
        root: node_id(fixture.root),
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: fixture.viewport.width,
            height: fixture.viewport.height,
        },
        root_sizing: RootSizingPolicy::MatchViewport,
        css_math: vec![],
        nodes: fixture
            .nodes
            .iter()
            .map(|node| LayoutNode {
                id: node_id(node.id),
                children: node.children.iter().copied().map(node_id).collect(),
                style: LayoutStyle {
                    width: dimension(&node.style.width),
                    height: dimension(&node.style.height),
                    flex_direction: match node.style.flex_direction.as_str() {
                        "row" => FlexDirection::Row,
                        "column" => FlexDirection::Column,
                        other => panic!("알 수 없는 fixture flexDirection: {other}"),
                    },
                    direction: match node.style.direction.as_str() {
                        "ltr" => TextDirection::Ltr,
                        "rtl" => TextDirection::Rtl,
                        other => panic!("알 수 없는 fixture direction: {other}"),
                    },
                    padding: LayoutEdges {
                        top: LayoutLengthPercentage::length(node.style.padding.top),
                        right: LayoutLengthPercentage::length(node.style.padding.right),
                        bottom: LayoutLengthPercentage::length(node.style.padding.bottom),
                        left: LayoutLengthPercentage::length(node.style.padding.left),
                    },
                    gap: LayoutGap {
                        row: LayoutLengthPercentage::length(node.style.row_gap),
                        column: LayoutLengthPercentage::length(node.style.column_gap),
                    },
                    flex_grow: node.style.flex_grow,
                    ..LayoutStyle::default()
                },
            })
            .collect(),
    }
}

fn node_id(value: u64) -> NodeId {
    NodeId::new(value).unwrap()
}

fn dimension(value: &FixtureDimension) -> LayoutDimension {
    match value {
        FixtureDimension::Number(value) => LayoutDimension::Fixed(*value),
        FixtureDimension::Keyword(keyword) if keyword == "auto" => LayoutDimension::Auto,
        FixtureDimension::Keyword(keyword) => panic!("알 수 없는 fixture dimension: {keyword}"),
    }
}

fn assert_close(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() <= TOLERANCE,
        "실제 {actual}과 기준 {expected}의 차이가 허용치 {TOLERANCE}보다 큽니다"
    );
}

fn assert_browser_frames(
    actual: &BTreeMap<NodeId, crate::LayoutFrame>,
    expected: &BTreeMap<String, FixtureFrame>,
) {
    assert_browser_frames_with_tolerance(actual, expected, TOLERANCE);
}

fn assert_browser_frames_with_tolerance(
    actual: &BTreeMap<NodeId, crate::LayoutFrame>,
    expected: &BTreeMap<String, FixtureFrame>,
    tolerance: f32,
) {
    assert_eq!(
        actual.len(),
        expected.len(),
        "fixture와 출력 노드 수가 다릅니다"
    );
    for (id, expected) in expected {
        let id = node_id(id.parse().unwrap());
        let frame = actual
            .get(&id)
            .unwrap_or_else(|| panic!("노드 {id} 프레임이 없습니다"));
        for (field, actual, expected) in [
            ("x", frame.x, expected.x),
            ("y", frame.y, expected.y),
            ("width", frame.width, expected.width),
            ("height", frame.height, expected.height),
        ] {
            assert!(
                (actual - expected).abs() <= tolerance,
                "노드 {id}의 {field}: 실제 {actual}, 기준 {expected}, 허용치 {tolerance}"
            );
        }
    }
}

#[test]
fn taffy_matches_the_browser_reference_fixture() {
    let fixture = fixture();
    assert_eq!(
        fixture.expected.len(),
        fixture.nodes.len(),
        "브라우저 기준 프레임을 fixture에 기록해야 합니다"
    );
    let output = TaffyLayoutEngine.compute(&to_input(&fixture)).unwrap();
    assert_browser_frames(&output.frames, &fixture.expected);
}

#[test]
fn taffy_matches_the_preserved_small_engine_on_the_shared_fixture() {
    let fixture = fixture();
    let input = to_input(&fixture);
    let taffy = TaffyLayoutEngine.compute(&input).unwrap();
    let legacy = legacy_oracle::legacy_frames(&fixture);
    assert_eq!(legacy.len(), taffy.frames.len());
    for (id, expected) in &legacy {
        let actual = taffy.frames.get(id).unwrap();
        assert_close(actual.x, expected.x);
        assert_close(actual.y, expected.y);
        assert_close(actual.width, expected.width);
        assert_close(actual.height, expected.height);
    }
    assert_browser_frames(&legacy, &fixture.expected);
}

#[test]
fn rtl_row_preserves_dom_child_order_while_starting_at_the_right() {
    let fixture = rtl_fixture();
    let output = TaffyLayoutEngine.compute(&to_input(&fixture)).unwrap();
    assert_browser_frames(&output.frames, &fixture.expected);
}

#[test]
fn fractional_flex_distribution_matches_chromium_without_integer_rounding() {
    let fixture = fractional_fixture();
    assert_eq!(
        fixture.expected.len(),
        fixture.nodes.len(),
        "소수 Flex fixture의 브라우저 기준 프레임을 기록해야 합니다"
    );

    let output = TaffyLayoutEngine.compute(&to_input(&fixture)).unwrap();
    assert_browser_frames_with_tolerance(&output.frames, &fixture.expected, FRACTIONAL_TOLERANCE);
}

#[test]
fn fractional_dimensions_are_not_rounded_by_the_layout_engine() {
    let root = node_id(1);
    let child = node_id(2);
    let mut child_node = fixed_node(child, 50.25, 25.5);
    child_node.style.width = LayoutDimension::Fixed(50.25);
    let input = LayoutInput {
        root,
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: 100.5,
            height: 80.5,
        },
        root_sizing: RootSizingPolicy::MatchViewport,
        css_math: vec![],
        nodes: vec![
            LayoutNode {
                id: root,
                children: vec![child],
                style: LayoutStyle {
                    width: LayoutDimension::Fixed(100.5),
                    height: LayoutDimension::Fixed(80.5),
                    ..LayoutStyle::default()
                },
            },
            child_node,
        ],
    };

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    assert_eq!(output.frames[&root].width, 100.5);
    assert_eq!(output.frames[&child].width, 50.25);
    assert_eq!(output.frames[&child].height, 25.5);
}

#[test]
fn duplicate_ids_and_invalid_styles_are_rejected() {
    let valid = to_input(&fixture());

    let mut invalid = valid.clone();
    invalid.nodes.push(invalid.nodes[1].clone());
    assert_eq!(
        TaffyLayoutEngine.compute(&invalid),
        Err(LayoutError::DuplicateNode(node_id(2)))
    );

    let mut invalid = valid.clone();
    invalid.nodes[1].style.gap.row = LayoutLengthPercentage::length(-1.0);
    assert_eq!(
        TaffyLayoutEngine.compute(&invalid),
        Err(LayoutError::InvalidStyle {
            node: node_id(2),
            field: "gap.row"
        })
    );

    let mut invalid = valid.clone();
    invalid.nodes[1].style.flex_grow = f32::INFINITY;
    assert_eq!(
        TaffyLayoutEngine.compute(&invalid),
        Err(LayoutError::InvalidStyle {
            node: node_id(2),
            field: "flex_grow"
        })
    );

    let mut invalid = valid.clone();
    invalid.nodes[1].style.width = LayoutDimension::Fixed(f32::NAN);
    assert_eq!(
        TaffyLayoutEngine.compute(&invalid),
        Err(LayoutError::InvalidStyle {
            node: node_id(2),
            field: "width"
        })
    );

    let mut invalid = valid.clone();
    invalid.nodes[1].style.padding.left = LayoutLengthPercentage::length(-1.0);
    assert_eq!(
        TaffyLayoutEngine.compute(&invalid),
        Err(LayoutError::InvalidStyle {
            node: node_id(2),
            field: "padding.left"
        })
    );

    let mut invalid = valid;
    invalid.nodes[0].style.width = LayoutDimension::Fixed(319.0);
    assert_eq!(
        TaffyLayoutEngine.compute(&invalid),
        Err(LayoutError::RootSizeMismatch { axis: "width" })
    );
}

#[test]
fn input_snapshot_reads_the_core_tree_and_keeps_child_order() {
    let root = node_id(1);
    let first_child = node_id(3);
    let second_child = node_id(2);
    let mut tree = Tree::new();
    let mut batch = ChangeBatch::new(tree.revision());
    batch
        .create(root, "div")
        .insert(root, None, 0)
        .create(first_child, "div")
        .insert(first_child, Some(root), 0)
        .create(second_child, "div")
        .insert(second_child, Some(root), 1);
    tree.commit(batch).unwrap();

    let mut styles = BTreeMap::from([
        (
            root,
            LayoutStyle {
                width: LayoutDimension::Fixed(100.0),
                height: LayoutDimension::Fixed(80.0),
                flex_direction: FlexDirection::Row,
                ..LayoutStyle::default()
            },
        ),
        (
            first_child,
            LayoutStyle {
                width: LayoutDimension::Fixed(20.0),
                height: LayoutDimension::Fixed(20.0),
                ..LayoutStyle::default()
            },
        ),
        (
            second_child,
            LayoutStyle {
                width: LayoutDimension::Fixed(20.0),
                height: LayoutDimension::Fixed(20.0),
                ..LayoutStyle::default()
            },
        ),
    ]);
    let viewport = Viewport {
        width: 100.0,
        height: 80.0,
    };
    let input = LayoutInput::from_tree(
        &tree,
        viewport,
        &styles,
        StyleRevision::default(),
        EnvironmentRevision::default(),
    )
    .unwrap();
    assert_eq!(input.nodes[0].id, root);
    assert_eq!(input.nodes[1].id, second_child);
    assert_eq!(input.nodes[0].children, vec![first_child, second_child]);
    let output = TaffyLayoutEngine.compute(&input).unwrap();
    assert!(matches!(
        input.source_revision(),
        LayoutSourceRevision::Tree(revision) if revision.get() == 1
    ));
    assert_eq!(output.revision.source(), input.source_revision());
    assert_eq!(output.frames[&first_child].x, 0.0);
    assert_eq!(output.frames[&second_child].x, 20.0);

    styles.remove(&first_child);
    assert_eq!(
        LayoutInput::from_tree(
            &tree,
            viewport,
            &styles,
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        Err(LayoutError::MissingStyle(first_child))
    );

    styles.insert(first_child, LayoutStyle::default());
    styles.insert(node_id(99), LayoutStyle::default());
    assert_eq!(
        LayoutInput::from_tree(
            &tree,
            viewport,
            &styles,
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        Err(LayoutError::UnknownStyleNode(node_id(99)))
    );

    assert_eq!(
        LayoutInput::from_tree(
            &Tree::new(),
            viewport,
            &BTreeMap::new(),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        Err(LayoutError::EmptyTree)
    );
}

fn fixed_node(id: NodeId, width: f32, height: f32) -> LayoutNode {
    LayoutNode {
        id,
        children: Vec::new(),
        style: LayoutStyle {
            width: LayoutDimension::Fixed(width),
            height: LayoutDimension::Fixed(height),
            ..LayoutStyle::default()
        },
    }
}
