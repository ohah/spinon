use std::{collections::BTreeMap, sync::Arc};

use spinon_core::{DocumentChangeBatch, DocumentOperation, HostParent, OwnerId, StyleRevision};
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, ParagraphStyle, TextLayoutEngine, WhiteSpaceMode,
    WordBreakMode,
};

use super::{Fixture, compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets};
use crate::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs;

const AHEM: &[u8] = include_bytes!("../../../../tests/fixtures/resources/c15/Ahem.ttf");
const TRACK: &str = "minmax(calc(10% + 2px),max-content)";
const TEXT: &str = "aaaaaa";

struct Case {
    id: &'static str,
    width: &'static str,
    root: [f32; 2],
    item: [f32; 4],
    columns: &'static [f32],
    rows: &'static [f32],
    text_height: f32,
}

const SMALL_TRACKS: &[f32] = &[4.0, 4.0];
const LARGE_TRACKS: &[f32] = &[29.0, 29.0];

const CASES: [Case; 4] = [
    Case {
        id: "cyclic-minmax-calc-two-axis-span-text-min-content",
        width: "min-content",
        root: [14.0, 64.0],
        item: [2.0, 2.0, 10.0, 60.0],
        columns: SMALL_TRACKS,
        rows: LARGE_TRACKS,
        text_height: 60.0,
    },
    Case {
        id: "cyclic-minmax-calc-two-axis-span-text-max-content",
        width: "max-content",
        root: [64.0, 14.0],
        item: [2.0, 2.0, 60.0, 10.0],
        columns: LARGE_TRACKS,
        rows: SMALL_TRACKS,
        text_height: 10.0,
    },
    Case {
        id: "definite-minmax-calc-two-axis-span-text-min-content",
        width: "10px",
        root: [14.0, 64.0],
        item: [2.0, 2.0, 10.0, 60.0],
        columns: SMALL_TRACKS,
        rows: LARGE_TRACKS,
        text_height: 60.0,
    },
    Case {
        id: "definite-minmax-calc-two-axis-span-text-max-content",
        width: "60px",
        root: [64.0, 14.0],
        item: [2.0, 2.0, 60.0, 10.0],
        columns: LARGE_TRACKS,
        rows: SMALL_TRACKS,
        text_height: 10.0,
    },
];

fn fixture(case: &Case) -> Fixture {
    let root_style = format!(
        "box-sizing:content-box;display:grid;width:{};height:max-content;margin:0;padding:0;border:2px solid #f0c674;column-gap:2px;row-gap:2px;align-content:start;justify-content:start;grid-template-columns:{} {};grid-template-rows:{} {};background-color:#101820;font-family:Ahem;font-size:10px;line-height:10px;color:#e5484d",
        case.width, TRACK, TRACK, TRACK, TRACK
    );
    let item_style = "box-sizing:border-box;width:auto;height:auto;min-width:auto;min-height:auto;margin:0;padding:0;border:0;grid-column:1 / 3;grid-row:1 / 3;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:break-all;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#e5484d";
    let mut fixture = Fixture::new(&root_style, &[("item", item_style)]);
    let text = fixture.document.reserve_node_handle().unwrap();
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(11_234).unwrap(),
        fixture.document.document_revision(),
    );
    batch
        .push(DocumentOperation::CreateText {
            node: text,
            data: TEXT.to_owned().into(),
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
    let paragraphs = BTreeMap::from([(
        fixture.nodes["item"].id(),
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
fn cyclic_minmax_calc_two_axis_span_uses_column_width_for_text_row_contribution() {
    for case in &CASES {
        let fixture = fixture(case);
        let output = compute(&fixture);
        assert_text_intrinsic_sizes(&output, fixture.nodes["item"].id(), case);
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
            "item frame",
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
        assert_cross_axis_max_content_contributions(&output, fixture, case);
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
    assert_close(
        sizes.min_content(),
        10.0,
        case.id,
        "text min-content inline",
    );
    assert_close(
        sizes.max_content(),
        60.0,
        case.id,
        "text max-content inline",
    );
}

fn assert_cross_axis_max_content_contributions(
    output: &crate::StyleLayoutOutput,
    fixture: Fixture,
    case: &Case,
) {
    let trace = output
        .layout
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == fixture.root.id())
        .unwrap_or_else(|| panic!("{}: missing Grid track-sizing trace", case.id));
    let columns = trace
        .passes
        .iter()
        .rposition(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .unwrap_or_else(|| panic!("{}: missing Columns pass", case.id));
    let rows = trace
        .passes
        .iter()
        .rposition(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .unwrap_or_else(|| panic!("{}: missing Rows pass", case.id));
    assert!(columns < rows, "{}: columns must size before rows", case.id);
    for (axis, kind, expected) in [
        (
            LayoutGridTrackSizingAxis::Columns,
            LayoutGridTrackSizingContributionKind::MinContent,
            10.0,
        ),
        (
            LayoutGridTrackSizingAxis::Columns,
            LayoutGridTrackSizingContributionKind::MaxContent,
            60.0,
        ),
        (
            LayoutGridTrackSizingAxis::Rows,
            LayoutGridTrackSizingContributionKind::MinContent,
            case.text_height,
        ),
        (
            LayoutGridTrackSizingAxis::Rows,
            LayoutGridTrackSizingContributionKind::MaxContent,
            case.text_height,
        ),
    ] {
        let pass = trace
            .passes
            .iter()
            .rev()
            .find(|pass| pass.axis == axis)
            .unwrap_or_else(|| panic!("{}: missing {axis:?} pass", case.id));
        let contributions = pass
            .contributions
            .iter()
            .filter(|contribution| {
                contribution.item == fixture.nodes["item"].id()
                    && contribution.kind == kind
                    && contribution.span == 2
            })
            .collect::<Vec<_>>();
        assert!(
            contributions
                .iter()
                .any(|contribution| (contribution.value - expected).abs() <= 0.05),
            "{}: {axis:?} {kind:?} span contribution expected {expected}, got {contributions:?}",
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
        .unwrap_or_else(|| panic!("{scene}: missing Grid track-sizing trace"));
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
