use std::collections::BTreeMap;
use std::sync::Arc;

use spinon_core::{DocumentChangeBatch, DocumentOperation, HostParent, OwnerId, StyleRevision};
use spinon_layout::{LayoutGridTrackSizingAxis, ParagraphStyle, TextLayoutEngine, WhiteSpaceMode};

use super::{
    Fixture, append_element, compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets,
};
use crate::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs;

const AHEM: &[u8] = include_bytes!("../../../../tests/fixtures/resources/c15/Ahem.ttf");

fn fixture(root_style: &str, item_text: &str) -> Fixture {
    let mut document = spinon_core::HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let item = document.reserve_node_handle().unwrap();
    let text = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(11_236).unwrap(), document.document_revision());
    append_element(&mut batch, root, HostParent::Root, "grid", root_style);
    append_element(
        &mut batch,
        item,
        HostParent::Node(root),
        "item",
        "box-sizing:border-box;min-width:0;min-height:0;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;line-height:10px",
    );
    batch
        .push(DocumentOperation::CreateText {
            node: text,
            data: item_text.to_owned().into(),
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

fn paragraph_style() -> ParagraphStyle {
    ParagraphStyle {
        font_family: "Ahem".to_owned(),
        font_size: 10.0,
        line_height: 10.0,
        white_space: WhiteSpaceMode::Normal,
        ..ParagraphStyle::default()
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
    let paragraphs = BTreeMap::from([(fixture.nodes["item"].id(), paragraph_style())]);
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
    expected: f32,
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
    assert_eq!(tracks.len(), 1, "{scene}: expected one {axis:?} track");
    assert!(
        (tracks[0].base_size - expected).abs() <= 0.05,
        "{scene}: {axis:?} track expected {expected}, got {}",
        tracks[0].base_size
    );
}

#[test]
fn cyclic_percentage_fit_content_text_scenes_match_chrome_tracks_and_frames() {
    let cases = [
        (
            "cyclic-fit-content-column",
            "display:grid;box-sizing:border-box;width:max-content;height:auto;grid-template-columns:fit-content(50%);grid-template-rows:auto;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
            "aa aa",
            (50.0, 20.0),
            (25.0, 20.0),
            25.0,
            20.0,
        ),
        (
            "cyclic-fit-content-row",
            "display:grid;box-sizing:border-box;width:20px;height:max-content;grid-template-columns:20px;grid-template-rows:fit-content(50%);column-gap:0;row-gap:0;margin:0;padding:0;border:0",
            "a a a a",
            (20.0, 40.0),
            (20.0, 20.0),
            20.0,
            20.0,
        ),
        (
            "definite-fit-content-column",
            "display:grid;box-sizing:border-box;width:40px;height:auto;grid-template-columns:fit-content(50%);grid-template-rows:auto;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
            "aa aa",
            (40.0, 20.0),
            (20.0, 20.0),
            20.0,
            20.0,
        ),
    ];

    for (scene, root_style, text, expected_root, expected_item, expected_columns, expected_rows) in
        cases
    {
        let fixture = fixture(root_style, text);
        let output = compute(&fixture);
        let root = output.layout.frames[&fixture.root.id()];
        let item = output.layout.frames[&fixture.nodes["item"].id()];
        assert!(
            (root.width - expected_root.0).abs() <= 0.05
                && (root.height - expected_root.1).abs() <= 0.05,
            "{scene}: expected root {:?}, got ({}, {})",
            expected_root,
            root.width,
            root.height
        );
        assert!(
            (item.width - expected_item.0).abs() <= 0.05
                && (item.height - expected_item.1).abs() <= 0.05,
            "{scene}: expected item {:?}, got ({}, {})",
            expected_item,
            item.width,
            item.height
        );
        assert_axis_tracks(
            &output,
            fixture.root.id(),
            LayoutGridTrackSizingAxis::Columns,
            expected_columns,
            scene,
        );
        assert_axis_tracks(
            &output,
            fixture.root.id(),
            LayoutGridTrackSizingAxis::Rows,
            expected_rows,
            scene,
        );
    }
}

#[test]
fn cyclic_fit_content_calc_text_scenes_match_tracks_and_frames() {
    let cases = [
        (
            "cyclic-fit-content-calc-column",
            "display:grid;box-sizing:border-box;width:max-content;height:auto;grid-template-columns:fit-content(calc(50% + 4px));grid-template-rows:auto;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
            "aa aa",
            (50.0, 20.0),
            (29.0, 20.0),
            29.0,
            20.0,
        ),
        (
            "cyclic-fit-content-calc-row",
            "display:grid;box-sizing:border-box;width:20px;height:max-content;grid-template-columns:20px;grid-template-rows:fit-content(calc(50% + 2px));column-gap:0;row-gap:0;margin:0;padding:0;border:0",
            "a a a a",
            (20.0, 40.0),
            (20.0, 22.0),
            20.0,
            22.0,
        ),
        (
            "definite-fit-content-calc-column",
            "display:grid;box-sizing:border-box;width:40px;height:auto;grid-template-columns:fit-content(calc(50% + 4px));grid-template-rows:auto;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
            "aa aa",
            (40.0, 20.0),
            (24.0, 20.0),
            24.0,
            20.0,
        ),
        (
            "definite-fit-content-calc-small-limit",
            "display:grid;box-sizing:border-box;width:10px;height:auto;grid-template-columns:fit-content(calc(50% + 4px));grid-template-rows:auto;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
            "aa aa",
            (10.0, 20.0),
            (9.0, 20.0),
            9.0,
            20.0,
        ),
    ];

    for (scene, root_style, text, expected_root, expected_item, expected_columns, expected_rows) in
        cases
    {
        let fixture = fixture(root_style, text);
        let output = compute(&fixture);
        let root = output.layout.frames[&fixture.root.id()];
        let item = output.layout.frames[&fixture.nodes["item"].id()];
        assert!(
            (root.width - expected_root.0).abs() <= 0.05
                && (root.height - expected_root.1).abs() <= 0.05,
            "{scene}: expected root {:?}, got ({}, {})",
            expected_root,
            root.width,
            root.height
        );
        assert!(
            item.x.abs() <= 0.05
                && item.y.abs() <= 0.05
                && (item.width - expected_item.0).abs() <= 0.05
                && (item.height - expected_item.1).abs() <= 0.05,
            "{scene}: expected item at origin with size {:?}, got ({}, {}, {}, {})",
            expected_item,
            item.x,
            item.y,
            item.width,
            item.height
        );
        assert_axis_tracks(
            &output,
            fixture.root.id(),
            LayoutGridTrackSizingAxis::Columns,
            expected_columns,
            scene,
        );
        assert_axis_tracks(
            &output,
            fixture.root.id(),
            LayoutGridTrackSizingAxis::Rows,
            expected_rows,
            scene,
        );
    }
}
