use std::{collections::BTreeMap, sync::Arc};

use spinon_core::{
    DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId, StyleRevision,
};
use spinon_layout::{LayoutGridTrackSizingAxis, ParagraphStyle, TextLayoutEngine, WhiteSpaceMode};

use super::{
    Fixture, append_element, compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets,
};
use crate::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs;

const AHEM: &[u8] = include_bytes!("../../../../tests/fixtures/resources/c15/Ahem.ttf");

struct Case {
    id: &'static str,
    root_style: &'static str,
    columns: f32,
    rows: f32,
}

const ITEM_STYLE: &str = "box-sizing:border-box;min-width:0;min-height:0;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#e5484d";

const CASES: [Case; 4] = [
    Case {
        id: "cyclic-minmax-calc-min-column",
        root_style: "box-sizing:content-box;display:grid;width:max-content;height:10px;margin:0;padding:0;border:2px solid #f0c674;column-gap:0;row-gap:0;align-content:start;justify-content:start;grid-template-columns:minmax(calc(200% + 4px),max-content);grid-template-rows:10px;background-color:#101820;font-family:Ahem;font-size:10px;line-height:10px;color:#e5484d",
        columns: 24.0,
        rows: 10.0,
    },
    Case {
        id: "cyclic-minmax-calc-min-row",
        root_style: "box-sizing:content-box;display:grid;width:10px;height:max-content;margin:0;padding:0;border:2px solid #f0c674;column-gap:0;row-gap:0;align-content:start;justify-content:start;grid-template-columns:10px;grid-template-rows:minmax(calc(200% + 2px),max-content);background-color:#101820;font-family:Ahem;font-size:10px;line-height:10px;color:#e5484d",
        columns: 10.0,
        rows: 22.0,
    },
    Case {
        id: "definite-minmax-calc-min-column",
        root_style: "box-sizing:content-box;display:grid;width:10px;height:10px;margin:0;padding:0;border:2px solid #f0c674;column-gap:0;row-gap:0;align-content:start;justify-content:start;grid-template-columns:minmax(calc(200% + 4px),max-content);grid-template-rows:10px;background-color:#101820;font-family:Ahem;font-size:10px;line-height:10px;color:#e5484d",
        columns: 24.0,
        rows: 10.0,
    },
    Case {
        id: "definite-minmax-calc-min-row",
        root_style: "box-sizing:content-box;display:grid;width:10px;height:10px;margin:0;padding:0;border:2px solid #f0c674;column-gap:0;row-gap:0;align-content:start;justify-content:start;grid-template-columns:10px;grid-template-rows:minmax(calc(200% + 2px),max-content);background-color:#101820;font-family:Ahem;font-size:10px;line-height:10px;color:#e5484d",
        columns: 10.0,
        rows: 22.0,
    },
];

fn fixture(root_style: &str) -> Fixture {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let item = document.reserve_node_handle().unwrap();
    let text = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(11_237).unwrap(), document.document_revision());
    append_element(&mut batch, root, HostParent::Root, "grid", root_style);
    append_element(&mut batch, item, HostParent::Node(root), "item", ITEM_STYLE);
    batch
        .push(DocumentOperation::CreateText {
            node: text,
            data: "a".to_owned().into(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(item),
            node: text,
            before: None,
        });
    document.commit(batch).unwrap();
    Fixture {
        document,
        root,
        nodes: BTreeMap::from([("item".to_owned(), item)]),
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
fn cyclic_minimum_calc_resolves_against_ten_pixel_content_box_after_intrinsic_sizing() {
    for case in CASES {
        let fixture = fixture(case.root_style);
        let output = compute(&fixture);
        let root = output.layout.frames[&fixture.root.id()];
        let item = output.layout.frames[&fixture.nodes["item"].id()];

        assert_frame(
            [root.width, root.height],
            [14.0, 14.0],
            case.id,
            "root border box",
        );
        assert_frame(
            [item.x, item.y, item.width, item.height],
            [2.0, 2.0, case.columns, case.rows],
            case.id,
            "item frame",
        );
        assert_axis_track(
            &output,
            fixture.root.id(),
            LayoutGridTrackSizingAxis::Columns,
            case.columns,
            case.id,
        );
        assert_axis_track(
            &output,
            fixture.root.id(),
            LayoutGridTrackSizingAxis::Rows,
            case.rows,
            case.id,
        );
    }
}

fn assert_frame<const N: usize>(actual: [f32; N], expected: [f32; N], scene: &str, label: &str) {
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert!(
            (actual - expected).abs() <= 0.05,
            "{scene}: {label} component {index} expected {expected}, got {actual}"
        );
    }
}

fn assert_axis_track(
    output: &crate::StyleLayoutOutput,
    grid: spinon_core::NodeId,
    axis: LayoutGridTrackSizingAxis,
    expected: f32,
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
    assert_eq!(tracks.len(), 1, "{scene}: expected one {axis:?} track");
    assert!(
        (tracks[0].base_size - expected).abs() <= 0.05,
        "{scene}: {axis:?} track expected {expected}, got {}",
        tracks[0].base_size,
    );
}
