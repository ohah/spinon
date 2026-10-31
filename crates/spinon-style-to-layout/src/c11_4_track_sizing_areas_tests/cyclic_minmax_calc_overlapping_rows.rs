use std::{collections::BTreeMap, sync::Arc};

use spinon_core::{DocumentChangeBatch, DocumentOperation, HostParent, OwnerId, StyleRevision};
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, LayoutGridTrackSizingStageKind, ParagraphStyle,
    TextLayoutEngine, WhiteSpaceMode, WordBreakMode,
};

use super::{Fixture, compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets};
use crate::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs;

const AHEM: &[u8] = include_bytes!("../../../../tests/fixtures/resources/c15/Ahem.ttf");
const TRACKS: &str = "minmax(calc(10% + 2px),max-content) minmax(calc(10% + 2px),max-content) minmax(calc(10% + 2px),max-content)";

#[derive(Clone, Copy)]
struct Item {
    id: &'static str,
    text: &'static str,
    placement: &'static str,
    span: u16,
    contribution: f32,
}

const LEFT: Item = Item {
    id: "span2-left",
    text: "aaaaaaa",
    placement: "grid-row:1 / 3",
    span: 2,
    contribution: 70.0,
};
const RIGHT: Item = Item {
    id: "span2-right",
    text: "aaaaaaaa",
    placement: "grid-row:2 / 4",
    span: 2,
    contribution: 80.0,
};
const SPAN3: Item = Item {
    id: "span3",
    text: "aaaaaaaaaaaaaa",
    placement: "grid-row:1 / 4",
    span: 3,
    contribution: 140.0,
};

struct Case {
    id: &'static str,
    height: &'static str,
    expected_root: [f32; 2],
    expected_rows: [f32; 3],
    expected_frames: [(&'static str, [f32; 4]); 3],
    order: [Item; 3],
    cyclic: bool,
}

const CYCLIC_FRAMES: [(&str, [f32; 4]); 3] = [
    ("span2-left", [2.0, 2.0, 10.0, 91.0]),
    ("span2-right", [2.0, 46.0, 10.0, 96.0]),
    ("span3", [2.0, 2.0, 10.0, 140.0]),
];
const SMALL_FRAMES: [(&str, [f32; 4]); 3] = [
    ("span2-left", [2.0, 2.0, 10.0, 66.0]),
    ("span2-right", [2.0, 36.0, 10.0, 66.0]),
    ("span3", [2.0, 2.0, 10.0, 100.0]),
];

const CASES: [Case; 6] = [
    Case {
        id: "cyclic-minmax-calc-overlap-rows-forward",
        height: "max-content",
        expected_root: [14.0, 144.0],
        expected_rows: [42.0, 47.0, 47.0],
        expected_frames: CYCLIC_FRAMES,
        order: [LEFT, RIGHT, SPAN3],
        cyclic: true,
    },
    Case {
        id: "cyclic-minmax-calc-overlap-rows-reverse",
        height: "max-content",
        expected_root: [14.0, 144.0],
        expected_rows: [42.0, 47.0, 47.0],
        expected_frames: CYCLIC_FRAMES,
        order: [SPAN3, RIGHT, LEFT],
        cyclic: true,
    },
    Case {
        id: "definite-minmax-calc-overlap-rows-forward",
        height: "140px",
        expected_root: [14.0, 144.0],
        expected_rows: [42.0, 47.0, 47.0],
        expected_frames: CYCLIC_FRAMES,
        order: [LEFT, RIGHT, SPAN3],
        cyclic: false,
    },
    Case {
        id: "definite-minmax-calc-overlap-rows-reverse",
        height: "140px",
        expected_root: [14.0, 144.0],
        expected_rows: [42.0, 47.0, 47.0],
        expected_frames: CYCLIC_FRAMES,
        order: [SPAN3, RIGHT, LEFT],
        cyclic: false,
    },
    Case {
        id: "definite-minmax-calc-overlap-rows-small-forward",
        height: "100px",
        expected_root: [14.0, 104.0],
        expected_rows: [32.0, 32.0, 32.0],
        expected_frames: SMALL_FRAMES,
        order: [LEFT, RIGHT, SPAN3],
        cyclic: false,
    },
    Case {
        id: "definite-minmax-calc-overlap-rows-small-reverse",
        height: "100px",
        expected_root: [14.0, 104.0],
        expected_rows: [32.0, 32.0, 32.0],
        expected_frames: SMALL_FRAMES,
        order: [SPAN3, RIGHT, LEFT],
        cyclic: false,
    },
];

fn fixture(case: &Case) -> Fixture {
    let root_style = format!(
        "box-sizing:content-box;display:grid;width:10px;height:{};margin:0;padding:0;border:2px solid #f0c674;column-gap:0;row-gap:2px;align-content:start;justify-content:start;grid-template-columns:10px;grid-template-rows:{};background-color:#101820;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;color:#e5484d",
        case.height, TRACKS
    );
    let item_styles = case
        .order
        .iter()
        .map(|item| {
            (
                item.id,
                format!(
                    "box-sizing:border-box;display:block;width:auto;height:auto;min-width:0;min-height:0;margin:0;padding:0;border:0;grid-column:1 / 2;{};font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:break-all;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#e5484d",
                    item.placement
                ),
            )
        })
        .collect::<Vec<_>>();
    let item_style_refs = item_styles
        .iter()
        .map(|(id, style)| (*id, style.as_str()))
        .collect::<Vec<_>>();
    // Build a root and item tree in source order so reversed cases exercise the actual input order.
    let mut document = spinon_core::HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let nodes = item_style_refs
        .iter()
        .map(|(id, _)| ((*id).to_owned(), document.reserve_node_handle().unwrap()))
        .collect::<BTreeMap<_, _>>();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(11_244).unwrap(), document.document_revision());
    super::append_element(&mut batch, root, HostParent::Root, "grid", &root_style);
    for (id, style) in &item_style_refs {
        super::append_element(&mut batch, nodes[*id], HostParent::Node(root), "div", style);
    }
    document.commit(batch).unwrap();
    let mut text_batch =
        DocumentChangeBatch::new(OwnerId::new(11_244).unwrap(), document.document_revision());
    for item in case.order {
        let text = document.reserve_node_handle().unwrap();
        text_batch
            .push(DocumentOperation::CreateText {
                node: text,
                data: item.text.to_owned().into(),
            })
            .push(DocumentOperation::InsertBefore {
                parent: HostParent::Node(nodes[item.id]),
                node: text,
                before: None,
            });
    }
    document.commit(text_batch).unwrap();
    Fixture {
        document,
        root,
        nodes,
    }
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
        height_css_px: 240.0,
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
                    overflow_wrap: Default::default(),
                    writing_mode: spinon_core::WritingMode::HorizontalTb,
                    text_orientation: spinon_core::TextOrientation::Mixed,
                    font_family: "Ahem".to_owned(),
                    font_size: 10.0,
                    line_height: 10.0,
                    white_space: WhiteSpaceMode::Normal,
                    word_break: WordBreakMode::BreakAll,
                    ..ParagraphStyle::default()
                },
            )
        })
        .collect::<BTreeMap<_, _>>();
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
fn cyclic_minmax_calc_overlapping_row_span_groups_are_source_order_independent() {
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
        for (item_id, expected) in case.expected_frames {
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
                .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        } else {
            trace
                .passes
                .iter()
                .rfind(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        }
        .unwrap_or_else(|| panic!("{}: missing Rows sizing pass", case.id));
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
        let expected_intrinsic = if case.expected_rows == [32.0, 32.0, 32.0] {
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
        assert_final_rows(&output, &fixture, case);
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
            10.0,
            case.id,
            "min-content inline source",
        );
        assert_close(
            sizes.max_content(),
            item.contribution,
            case.id,
            "max-content inline source",
        );
        assert_close(
            sizes.min_content_height(),
            item.contribution,
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
            assert_close(
                contribution.value,
                item.contribution,
                case.id,
                "row span contribution",
            );
        }
    }
}

fn assert_final_rows(output: &crate::StyleLayoutOutput, fixture: &Fixture, case: &Case) {
    let trace = output
        .layout
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == fixture.root.id())
        .unwrap();
    let pass = trace
        .passes
        .iter()
        .rfind(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .unwrap();
    let stage = pass.stages.last().unwrap();
    let actual = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(actual.len(), 3, "{}: final row count", case.id);
    for (index, (actual, expected)) in actual.iter().zip(case.expected_rows).enumerate() {
        assert_close(
            *actual,
            expected,
            case.id,
            &format!("final row {}", index + 1),
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
