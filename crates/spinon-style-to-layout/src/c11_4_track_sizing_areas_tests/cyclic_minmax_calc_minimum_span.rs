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

struct Case {
    id: &'static str,
    root_style: &'static str,
    item_style: &'static str,
    text: &'static str,
    root: [f32; 2],
    item: [f32; 4],
    columns: &'static [f32],
    rows: &'static [f32],
    span_axis: LayoutGridTrackSizingAxis,
    max_content_contribution: f32,
    text_sizes: [f32; 4],
    cyclic: bool,
    intrinsic_sizing_tracks_with_gutters: &'static [f32],
}

const COLUMNS: &[f32] = &[29.0, 29.0];
const ROWS: &[f32] = &[12.0, 12.0];

const CASES: [Case; 4] = [
    Case {
        id: "cyclic-minmax-calc-min-span-columns",
        root_style: "box-sizing:content-box;display:grid;width:max-content;height:10px;margin:0;padding:0;border:2px solid #f0c674;column-gap:4px;row-gap:0;align-content:start;justify-content:start;grid-template-columns:minmax(calc(50% + 4px),max-content) minmax(calc(50% + 4px),max-content);grid-template-rows:10px;background-color:#101820;font-family:Ahem;font-size:10px;line-height:10px;color:#e5484d",
        item_style: "box-sizing:border-box;min-width:0;min-height:0;margin:0;padding:0;border:0;grid-column:1 / span 2;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#e5484d",
        text: "aa aa",
        root: [54.0, 14.0],
        item: [2.0, 2.0, 62.0, 10.0],
        columns: COLUMNS,
        rows: &[10.0],
        span_axis: LayoutGridTrackSizingAxis::Columns,
        max_content_contribution: 50.0,
        text_sizes: [20.0, 50.0, 20.0, 10.0],
        cyclic: true,
        intrinsic_sizing_tracks_with_gutters: &[0.0, 23.0, 4.0, 23.0, 0.0],
    },
    Case {
        id: "definite-minmax-calc-min-span-columns",
        root_style: "box-sizing:content-box;display:grid;width:50px;height:10px;margin:0;padding:0;border:2px solid #f0c674;column-gap:4px;row-gap:0;align-content:start;justify-content:start;grid-template-columns:minmax(calc(50% + 4px),max-content) minmax(calc(50% + 4px),max-content);grid-template-rows:10px;background-color:#101820;font-family:Ahem;font-size:10px;line-height:10px;color:#e5484d",
        item_style: "box-sizing:border-box;min-width:0;min-height:0;margin:0;padding:0;border:0;grid-column:1 / span 2;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#e5484d",
        text: "aa aa",
        root: [54.0, 14.0],
        item: [2.0, 2.0, 62.0, 10.0],
        columns: COLUMNS,
        rows: &[10.0],
        span_axis: LayoutGridTrackSizingAxis::Columns,
        max_content_contribution: 50.0,
        text_sizes: [20.0, 50.0, 20.0, 10.0],
        cyclic: false,
        intrinsic_sizing_tracks_with_gutters: &[],
    },
    Case {
        id: "cyclic-minmax-calc-min-span-rows",
        root_style: "box-sizing:content-box;display:grid;width:20px;height:max-content;margin:0;padding:0;border:2px solid #f0c674;column-gap:0;row-gap:4px;align-content:start;justify-content:start;grid-template-columns:20px;grid-template-rows:minmax(calc(50% + 2px),max-content) minmax(calc(50% + 2px),max-content);background-color:#101820;font-family:Ahem;font-size:10px;line-height:10px;color:#e5484d",
        item_style: "box-sizing:border-box;min-width:0;min-height:0;margin:0;padding:0;border:0;grid-row:1 / span 2;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#e5484d",
        text: "a a",
        root: [24.0, 24.0],
        item: [2.0, 2.0, 20.0, 28.0],
        columns: &[20.0],
        rows: ROWS,
        span_axis: LayoutGridTrackSizingAxis::Rows,
        max_content_contribution: 20.0,
        text_sizes: [10.0, 30.0, 20.0, 10.0],
        cyclic: true,
        intrinsic_sizing_tracks_with_gutters: &[0.0, 8.0, 4.0, 8.0, 0.0],
    },
    Case {
        id: "definite-minmax-calc-min-span-rows",
        root_style: "box-sizing:content-box;display:grid;width:20px;height:20px;margin:0;padding:0;border:2px solid #f0c674;column-gap:0;row-gap:4px;align-content:start;justify-content:start;grid-template-columns:20px;grid-template-rows:minmax(calc(50% + 2px),max-content) minmax(calc(50% + 2px),max-content);background-color:#101820;font-family:Ahem;font-size:10px;line-height:10px;color:#e5484d",
        item_style: "box-sizing:border-box;min-width:0;min-height:0;margin:0;padding:0;border:0;grid-row:1 / span 2;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#e5484d",
        text: "a a",
        root: [24.0, 24.0],
        item: [2.0, 2.0, 20.0, 28.0],
        columns: &[20.0],
        rows: ROWS,
        span_axis: LayoutGridTrackSizingAxis::Rows,
        max_content_contribution: 20.0,
        text_sizes: [10.0, 30.0, 20.0, 10.0],
        cyclic: false,
        intrinsic_sizing_tracks_with_gutters: &[],
    },
];

fn fixture(case: &Case) -> Fixture {
    let mut fixture = Fixture::new(case.root_style, &[("item", case.item_style)]);
    let text = fixture.document.reserve_node_handle().unwrap();
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(11_234).unwrap(),
        fixture.document.document_revision(),
    );
    batch
        .push(DocumentOperation::CreateText {
            node: text,
            data: case.text.to_owned().into(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(fixture.nodes["item"]),
            node: text,
            before: None,
        });
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
    let paragraphs = std::collections::BTreeMap::from([(
        fixture.nodes["item"].id(),
        ParagraphStyle {
            overflow_wrap: Default::default(),
            writing_mode: spinon_core::WritingMode::HorizontalTb,
            text_orientation: spinon_core::TextOrientation::Mixed,
            font_family: "Ahem".to_owned(),
            font_size: 10.0,
            line_height: 10.0,
            white_space: WhiteSpaceMode::Normal,
            ..ParagraphStyle::default()
        },
    )]);
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
fn cyclic_minmax_calc_minimum_distributes_span_then_resolves_two_tracks_and_overflow() {
    for case in &CASES {
        let fixture = fixture(case);
        let output = compute(&fixture);
        let root = output.layout.frames[&fixture.root.id()];
        let item = output.layout.frames[&fixture.nodes["item"].id()];

        assert_frame(
            [root.width, root.height],
            case.root,
            case.id,
            "root border box",
        );
        assert_frame(
            [item.x, item.y, item.width, item.height],
            case.item,
            case.id,
            "span item frame",
        );
        assert_axis_tracks(
            &output,
            fixture.root.id(),
            LayoutGridTrackSizingAxis::Columns,
            case.columns,
            case.id,
        );
        assert_axis_tracks(
            &output,
            fixture.root.id(),
            LayoutGridTrackSizingAxis::Rows,
            case.rows,
            case.id,
        );
        assert_text_intrinsic_sizes(&output, fixture.nodes["item"].id(), case);
        assert_span_contribution_and_intrinsic_distribution(&output, fixture, case);
    }
}

fn assert_text_intrinsic_sizes(
    output: &crate::StyleLayoutOutput,
    item: spinon_core::NodeId,
    case: &Case,
) {
    let input = output
        .layout
        .grid_item_intrinsic_inputs
        .iter()
        .find(|input| input.node == item)
        .unwrap_or_else(|| panic!("{}: missing Grid item intrinsic input", case.id));
    let LayoutGridItemIntrinsicSource::Text { sizes } = &input.source else {
        panic!("{}: expected direct Ahem text input", case.id);
    };
    for (label, actual, expected) in [
        (
            "min-content inline",
            sizes.min_content(),
            case.text_sizes[0],
        ),
        (
            "max-content inline",
            sizes.max_content(),
            case.text_sizes[1],
        ),
        (
            "min-content height",
            sizes.min_content_height(),
            case.text_sizes[2],
        ),
        (
            "max-content height",
            sizes.max_content_height(),
            case.text_sizes[3],
        ),
    ] {
        assert_close(actual, expected, case.id, label);
    }
}

fn assert_span_contribution_and_intrinsic_distribution(
    output: &crate::StyleLayoutOutput,
    fixture: Fixture,
    case: &Case,
) {
    let trace = output
        .layout
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == fixture.root.id())
        .unwrap_or_else(|| panic!("{}: missing Grid track sizing trace", case.id));
    let pass = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == case.span_axis)
        .find(|_| case.cyclic)
        .or_else(|| {
            trace
                .passes
                .iter()
                .rfind(|pass| pass.axis == case.span_axis)
        })
        .unwrap_or_else(|| panic!("{}: missing {:?} pass", case.id, case.span_axis));
    let contribution = pass
        .contributions
        .iter()
        .find(|contribution| {
            contribution.item == fixture.nodes["item"].id()
                && contribution.kind == LayoutGridTrackSizingContributionKind::MaxContent
                && contribution.span == 2
        })
        .unwrap_or_else(|| panic!("{}: missing span-2 max-content contribution", case.id));
    assert_close(
        contribution.value,
        case.max_content_contribution,
        case.id,
        "span max-content contribution",
    );
    if case.cyclic {
        let maximized = pass
            .stages
            .iter()
            .find(|stage| stage.stage == LayoutGridTrackSizingStageKind::MaximizeTracks)
            .unwrap_or_else(|| panic!("{}: missing intrinsic max-content sizing stage", case.id));
        let sizes = maximized
            .tracks
            .iter()
            .map(|track| track.base_size)
            .collect::<Vec<_>>();
        assert_eq!(
            sizes, case.intrinsic_sizing_tracks_with_gutters,
            "{}: cyclic intrinsic span distribution before final percentage resolution",
            case.id
        );
    }
}

fn assert_axis_tracks(
    output: &crate::StyleLayoutOutput,
    grid: spinon_core::NodeId,
    axis: LayoutGridTrackSizingAxis,
    expected: &[f32],
    scene: &str,
) {
    let trace = output
        .layout
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == grid)
        .unwrap_or_else(|| panic!("{scene}: missing Grid track sizing trace"));
    let pass = trace
        .passes
        .iter()
        .rfind(|pass| pass.axis == axis)
        .unwrap_or_else(|| panic!("{scene}: missing final {axis:?} pass"));
    let stage = pass
        .stages
        .last()
        .unwrap_or_else(|| panic!("{scene}: missing final {axis:?} stage"));
    let tracks = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .collect::<Vec<_>>();
    assert_eq!(
        tracks.len(),
        expected.len(),
        "{scene}: {axis:?} track count"
    );
    for (index, (track, expected)) in tracks.iter().zip(expected).enumerate() {
        assert_close(
            track.base_size,
            *expected,
            scene,
            &format!("{axis:?} track {}", index + 1),
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
