use std::{collections::BTreeMap, sync::Arc};

use spinon_core::{DocumentChangeBatch, DocumentOperation, HostParent, OwnerId, StyleRevision};
use spinon_layout::{LayoutGridTrackSizingAxis, ParagraphStyle, TextLayoutEngine, WhiteSpaceMode};

use super::{Fixture, compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets};
use crate::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs;

const AHEM: &[u8] = include_bytes!("../../../../tests/fixtures/resources/c15/Ahem.ttf");

struct Case {
    id: &'static str,
    root_style: &'static str,
    columns: &'static [f32],
    rows: &'static [f32],
}

const CASES: [Case; 4] = [
    Case {
        id: "cyclic-percent-two-axis-multitrack",
        root_style: "display:grid;box-sizing:border-box;width:max-content;height:max-content;grid-template-columns:50% 25% 20px;grid-template-rows:50% 25% 10px;column-gap:0;row-gap:0;align-content:start;justify-content:start;margin:0;padding:0;border:0",
        columns: &[20.0, 10.0, 20.0],
        rows: &[10.0, 5.0, 10.0],
    },
    Case {
        id: "cyclic-percent-columns-multitrack",
        root_style: "display:grid;box-sizing:border-box;width:max-content;height:20px;grid-template-columns:50% 25% 20px;grid-template-rows:10px 5px 5px;column-gap:0;row-gap:0;align-content:start;justify-content:start;margin:0;padding:0;border:0",
        columns: &[20.0, 10.0, 20.0],
        rows: &[10.0, 5.0, 5.0],
    },
    Case {
        id: "cyclic-percent-rows-multitrack",
        root_style: "display:grid;box-sizing:border-box;width:40px;height:max-content;grid-template-columns:20px 10px 10px;grid-template-rows:50% 25% 10px;column-gap:0;row-gap:0;align-content:start;justify-content:start;margin:0;padding:0;border:0",
        columns: &[20.0, 10.0, 10.0],
        rows: &[10.0, 5.0, 10.0],
    },
    Case {
        id: "definite-percent-two-axis-multitrack",
        root_style: "display:grid;box-sizing:border-box;width:40px;height:20px;grid-template-columns:50% 25% 20px;grid-template-rows:50% 25% 10px;column-gap:0;row-gap:0;align-content:start;justify-content:start;margin:0;padding:0;border:0",
        columns: &[20.0, 10.0, 20.0],
        rows: &[10.0, 5.0, 10.0],
    },
];

fn fixture(root_style: &str) -> Fixture {
    let mut fixture = Fixture::new(
        root_style,
        &[
            (
                "text",
                "box-sizing:border-box;display:block;min-width:0;min-height:0;margin:0;padding:0;border:0;grid-column:1;grid-row:1;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#e5484d",
            ),
            (
                "overflow",
                "box-sizing:border-box;display:block;width:20px;height:10px;margin:0;padding:0;border:0;grid-column:3;grid-row:3;background-color:#20bf6b",
            ),
        ],
    );
    let text = fixture.document.reserve_node_handle().unwrap();
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(11_234).unwrap(),
        fixture.document.document_revision(),
    );
    batch
        .push(DocumentOperation::CreateText {
            node: text,
            data: "XX".to_owned().into(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(fixture.nodes["text"]),
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
        height_css_px: 80.0,
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
        fixture.nodes["text"].id(),
        ParagraphStyle {
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
        .unwrap_or_else(|| panic!("{scene}: missing grid track sizing trace"));
    let pass = trace
        .passes
        .iter()
        .rfind(|pass| pass.axis == axis)
        .unwrap_or_else(|| panic!("{scene}: missing final {axis:?} pass"));
    let stage = pass
        .stages
        .last()
        .unwrap_or_else(|| panic!("{scene}: missing {axis:?} final stage"));
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
        assert!(
            (track.base_size - expected).abs() <= 0.05,
            "{scene}: {axis:?} track {} expected {expected}, got {}",
            index + 1,
            track.base_size,
        );
    }
}

#[test]
fn cyclic_percentages_size_both_three_track_axes_and_preserve_cell_overflow() {
    for case in CASES {
        let fixture = fixture(case.root_style);
        let output = compute(&fixture);
        let root = output.layout.frames[&fixture.root.id()];
        let text = output.layout.frames[&fixture.nodes["text"].id()];
        let overflow = output.layout.frames[&fixture.nodes["overflow"].id()];
        assert_eq!(
            (root.x, root.y, root.width, root.height),
            (0.0, 0.0, 40.0, 20.0),
            "{} root",
            case.id
        );
        assert_eq!(
            (text.x, text.y, text.width, text.height),
            (0.0, 0.0, 20.0, 10.0),
            "{} text",
            case.id
        );
        assert_eq!(
            (overflow.x, overflow.y, overflow.width, overflow.height),
            (30.0, 15.0, 20.0, 10.0),
            "{} overflow",
            case.id
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
    }
}
