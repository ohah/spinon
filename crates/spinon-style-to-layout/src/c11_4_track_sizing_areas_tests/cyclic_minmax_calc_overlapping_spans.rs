use std::sync::Arc;

use spinon_core::{DocumentChangeBatch, DocumentOperation, HostParent, OwnerId, StyleRevision};
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, LayoutGridTrackSizingStageKind, ParagraphStyle,
    TextLayoutEngine, WhiteSpaceMode,
};

use super::{Fixture, compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets};
use crate::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs;

const AHEM: &[u8] = include_bytes!("../../../../tests/fixtures/resources/c15/Ahem.ttf");
const TRACK: &str = "minmax(calc(10% + 2px),max-content)";

#[derive(Clone, Copy)]
struct Item {
    id: &'static str,
    text: &'static str,
    placement: &'static str,
    span: u16,
    width: f32,
}

const LEFT: Item = Item {
    id: "span2-left",
    text: "aaaaaaa",
    placement: "grid-column:1 / 3",
    span: 2,
    width: 70.0,
};
const RIGHT: Item = Item {
    id: "span2-right",
    text: "aaaaaaaa",
    placement: "grid-column:2 / 4",
    span: 2,
    width: 80.0,
};
const SPAN3: Item = Item {
    id: "span3",
    text: "aaaaaaaaaaaaaa",
    placement: "grid-column:1 / 4",
    span: 3,
    width: 140.0,
};

struct Case {
    id: &'static str,
    width: &'static str,
    expected_root: [f32; 2],
    expected_columns: [f32; 3],
    expected_items: [(&'static str, [f32; 4]); 3],
    order: [Item; 3],
    cyclic: bool,
}

const CYCLIC_FRAMES: [(&str, [f32; 4]); 3] = [
    ("span2-left", [2.0, 2.0, 91.0, 10.0]),
    ("span2-right", [46.0, 2.0, 96.0, 10.0]),
    ("span3", [2.0, 2.0, 140.0, 10.0]),
];
const SMALL_FRAMES: [(&str, [f32; 4]); 3] = [
    ("span2-left", [2.0, 2.0, 66.0, 10.0]),
    ("span2-right", [36.0, 2.0, 66.0, 10.0]),
    ("span3", [2.0, 2.0, 100.0, 10.0]),
];

const CASES: [Case; 6] = [
    Case {
        id: "cyclic-minmax-calc-overlap-forward",
        width: "max-content",
        expected_root: [144.0, 14.0],
        expected_columns: [42.0, 47.0, 47.0],
        expected_items: CYCLIC_FRAMES,
        order: [LEFT, RIGHT, SPAN3],
        cyclic: true,
    },
    Case {
        id: "cyclic-minmax-calc-overlap-reverse",
        width: "max-content",
        expected_root: [144.0, 14.0],
        expected_columns: [42.0, 47.0, 47.0],
        expected_items: CYCLIC_FRAMES,
        order: [SPAN3, RIGHT, LEFT],
        cyclic: true,
    },
    Case {
        id: "definite-minmax-calc-overlap-forward",
        width: "140px",
        expected_root: [144.0, 14.0],
        expected_columns: [42.0, 47.0, 47.0],
        expected_items: CYCLIC_FRAMES,
        order: [LEFT, RIGHT, SPAN3],
        cyclic: false,
    },
    Case {
        id: "definite-minmax-calc-overlap-reverse",
        width: "140px",
        expected_root: [144.0, 14.0],
        expected_columns: [42.0, 47.0, 47.0],
        expected_items: CYCLIC_FRAMES,
        order: [SPAN3, RIGHT, LEFT],
        cyclic: false,
    },
    Case {
        id: "definite-minmax-calc-overlap-small-forward",
        width: "100px",
        expected_root: [104.0, 14.0],
        expected_columns: [32.0, 32.0, 32.0],
        expected_items: SMALL_FRAMES,
        order: [LEFT, RIGHT, SPAN3],
        cyclic: false,
    },
    Case {
        id: "definite-minmax-calc-overlap-small-reverse",
        width: "100px",
        expected_root: [104.0, 14.0],
        expected_columns: [32.0, 32.0, 32.0],
        expected_items: SMALL_FRAMES,
        order: [SPAN3, RIGHT, LEFT],
        cyclic: false,
    },
];

fn fixture(case: &Case) -> Fixture {
    let columns = format!("{TRACK} {TRACK} {TRACK}");
    let root_style = format!(
        "box-sizing:content-box;display:grid;width:{};height:10px;margin:0;padding:0;border:2px solid #f0c674;column-gap:2px;row-gap:0;align-content:start;justify-content:start;grid-template-columns:{columns};grid-template-rows:10px;background-color:#101820;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;color:#e5484d",
        case.width
    );
    let item_styles = case
        .order
        .iter()
        .map(|item| {
            (
                item.id,
                format!(
                    "box-sizing:border-box;display:block;width:auto;height:auto;min-width:0;min-height:0;margin:0;padding:0;border:0;{};grid-row:1 / 2;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:nowrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#e5484d",
                    item.placement
                ),
            )
        })
        .collect::<Vec<_>>();
    let item_style_refs = item_styles
        .iter()
        .map(|(id, style)| (*id, style.as_str()))
        .collect::<Vec<_>>();
    let mut fixture = Fixture::new(&root_style, &item_style_refs);
    let text_nodes = case
        .order
        .iter()
        .map(|item| {
            (
                fixture.nodes[item.id],
                fixture.document.reserve_node_handle().unwrap(),
                item.text,
            )
        })
        .collect::<Vec<_>>();
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(11_234).unwrap(),
        fixture.document.document_revision(),
    );
    for (item, text_node, text) in text_nodes {
        batch
            .push(DocumentOperation::CreateText {
                node: text_node,
                data: text.to_owned().into(),
            })
            .push(DocumentOperation::InsertBefore {
                parent: HostParent::Node(item),
                node: text_node,
                before: None,
            });
    }
    fixture.document.commit(batch).unwrap();
    fixture
}

fn compute(fixture: &Fixture) -> crate::StyleLayoutOutput {
    let snapshot = fixture.document.snapshot();
    let view = spinon_style::StyloDocumentView::new_html_runtime_mount_shared(
        Arc::new(snapshot.clone()),
        fixture.root,
    )
    .unwrap();
    let viewport = spinon_style::CssViewport {
        width_css_px: 240.0,
        height_css_px: 160.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: spinon_style::CssMediaEnvironment::MOBILE,
    };
    let computed_styles = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &view,
        &[],
        viewport,
        StyleRevision::INITIAL,
    )
    .unwrap();
    let paragraphs = fixture
        .nodes
        .values()
        .map(|node| {
            (
                node.id(),
                ParagraphStyle {
                    font_family: "Ahem".to_owned(),
                    font_size: 10.0,
                    line_height: 10.0,
                    white_space: WhiteSpaceMode::NoWrap,
                    ..ParagraphStyle::default()
                },
            )
        })
        .collect();
    let mut text_engine = TextLayoutEngine::new_without_system_fonts();
    text_engine.register_font("Ahem", AHEM).unwrap();
    compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs(
        &snapshot,
        fixture.root,
        computed_styles,
        viewport,
        &paragraphs,
        &text_engine,
    )
    .unwrap()
}

#[test]
fn cyclic_minmax_calc_overlapping_span_groups_are_source_order_independent() {
    for case in &CASES {
        let fixture = fixture(case);
        let output = compute(&fixture);
        let root = output.layout.frames[&fixture.root.id()];
        assert_frame(
            [root.width, root.height],
            case.expected_root,
            case.id,
            "root border box",
        );
        for (item_id, expected) in case.expected_items {
            let frame = output.layout.frames[&fixture.nodes[item_id].id()];
            assert_frame(
                [frame.x, frame.y, frame.width, frame.height],
                expected,
                case.id,
                item_id,
            );
        }

        let trace = output
            .layout
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == fixture.root.id())
            .unwrap_or_else(|| panic!("{}: missing Grid track sizing trace", case.id));
        let pass = if case.cyclic {
            trace
                .passes
                .iter()
                .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        } else {
            trace
                .passes
                .iter()
                .rfind(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        }
        .unwrap_or_else(|| panic!("{}: missing Columns sizing pass", case.id));
        let batches = pass
            .stages
            .iter()
            .filter_map(|stage| match stage.stage {
                LayoutGridTrackSizingStageKind::IntrinsicItemBatch { span, .. } => Some(span),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(batches, [2, 3], "{}: intrinsic span group order", case.id);

        let maximize = pass
            .stages
            .iter()
            .find(|stage| stage.stage == LayoutGridTrackSizingStageKind::MaximizeTracks)
            .unwrap_or_else(|| panic!("{}: missing MaximizeTracks stage", case.id));
        let expected_intrinsic = if case.expected_columns == [32.0, 32.0, 32.0] {
            [0.0, 32.0, 2.0, 32.0, 2.0, 32.0, 0.0]
        } else {
            [0.0, 42.0, 2.0, 47.0, 2.0, 47.0, 0.0]
        };
        let actual_intrinsic = maximize
            .tracks
            .iter()
            .map(|track| track.base_size)
            .collect::<Vec<_>>();
        assert_frame(
            actual_intrinsic
                .try_into()
                .unwrap_or_else(|tracks: Vec<f32>| {
                    panic!("{}: intrinsic track state count {}", case.id, tracks.len())
                }),
            expected_intrinsic,
            case.id,
            "track sizing after max-content distribution",
        );

        assert_intrinsic_inputs(&output, &fixture, case);
        assert_contribution_trace(&fixture, case, pass);
        assert_final_columns(&output, &fixture, case);
    }
}

fn assert_intrinsic_inputs(output: &crate::StyleLayoutOutput, fixture: &Fixture, case: &Case) {
    for item in case.order {
        let intrinsic = output
            .layout
            .grid_item_intrinsic_inputs
            .iter()
            .find(|input| input.node == fixture.nodes[item.id].id())
            .unwrap_or_else(|| panic!("{}: missing {} intrinsic source", case.id, item.id));
        let LayoutGridItemIntrinsicSource::Text { sizes } = &intrinsic.source else {
            panic!("{}: {} is not a text intrinsic source", case.id, item.id);
        };
        assert_close(
            sizes.min_content(),
            item.width,
            case.id,
            "min-content source",
        );
        assert_close(
            sizes.max_content(),
            item.width,
            case.id,
            "max-content source",
        );
        assert_close(
            sizes.min_content_height(),
            10.0,
            case.id,
            "min-content block source",
        );
        assert_close(
            sizes.max_content_height(),
            10.0,
            case.id,
            "max-content block source",
        );
    }
}

fn assert_contribution_trace(
    fixture: &Fixture,
    case: &Case,
    pass: &spinon_layout::LayoutGridTrackSizingPassTrace,
) {
    for item in case.order {
        for kind in [
            LayoutGridTrackSizingContributionKind::MinContent,
            LayoutGridTrackSizingContributionKind::MaxContent,
        ] {
            let contribution = pass
                .contributions
                .iter()
                .find(|contribution| {
                    contribution.item == fixture.nodes[item.id].id()
                        && contribution.kind == kind
                        && contribution.span == item.span
                })
                .unwrap_or_else(|| {
                    panic!(
                        "{}: missing {:?} contribution for {}",
                        case.id, kind, item.id
                    )
                });
            assert_close(contribution.value, item.width, case.id, "span contribution");
        }
    }
}

fn assert_final_columns(output: &crate::StyleLayoutOutput, fixture: &Fixture, case: &Case) {
    let trace = output
        .layout
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == fixture.root.id())
        .unwrap();
    let pass = trace
        .passes
        .iter()
        .rfind(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .unwrap();
    let stage = pass.stages.last().unwrap();
    let actual = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(actual.len(), 3, "{}: final column count", case.id);
    for (index, (actual, expected)) in actual.iter().zip(case.expected_columns).enumerate() {
        assert_close(
            *actual,
            expected,
            case.id,
            &format!("final column {}", index + 1),
        );
    }
}

fn assert_frame<const N: usize>(actual: [f32; N], expected: [f32; N], scene: &str, label: &str) {
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert_close(
            *actual,
            expected,
            scene,
            &format!("{label} component {index}"),
        );
    }
}

fn assert_close(actual: f32, expected: f32, scene: &str, label: &str) {
    assert!(
        (actual - expected).abs() <= 0.05,
        "{scene}: {label} expected {expected}, got {actual}"
    );
}
