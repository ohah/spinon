use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostDocumentSnapshot,
    HostNodeHandle, HostNodeKind, HostParent, OwnerId, StyleRevision,
};
use spinon_layout::LayoutGridTrackSizingAxis;
use spinon_style::{
    CssMediaEnvironment, CssOrigin, CssViewport, StylesheetSource, StyloDocumentView,
    compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets,
};

use super::Axis;

const IMAGE_URL: &str = "https://spinon.invalid/c14-1-png-basic.png";
const VIEWPORT: CssViewport = CssViewport {
    width_css_px: 1280.0,
    height_css_px: 633.0,
    device_scale_factor: 1.0,
    environment_revision: spinon_core::EnvironmentRevision::INITIAL,
    media_environment: CssMediaEnvironment::DESKTOP,
};
const STYLESHEET: &str = r#"
.root {
  box-sizing: border-box;
  display: grid;
  margin: 0;
  padding: 0;
  align-items: normal;
  align-content: start;
  justify-items: normal;
  justify-content: start;
  direction: ltr;
  writing-mode: horizontal-tb;
}
.root.columns {
  width: 300px;
  height: 120px;
  column-gap: 10px;
  row-gap: 0;
  grid-template-columns: minmax(min-content, 1fr) 40px minmax(0px, 2fr);
  grid-template-rows: 120px;
}
.root.rows {
  width: 120px;
  height: 300px;
  column-gap: 0;
  row-gap: 10px;
  grid-template-columns: 120px;
  grid-template-rows: minmax(min-content, 1fr) 40px minmax(0px, 2fr);
}
.root.columns.border-overflow {
  width: 100px;
  height: 120px;
  border-width: 2px;
  border-style: solid;
  border-color: #f0c674;
  grid-template-rows: 116px;
}
.root.rows.border-overflow {
  width: 120px;
  height: 100px;
  border-width: 2px;
  border-style: solid;
  border-color: #f0c674;
  grid-template-columns: 116px;
}
.item {
  box-sizing: border-box;
  display: block;
  position: static;
  width: auto;
  height: auto;
  min-width: auto;
  min-height: auto;
  margin: 0;
  padding: 0;
}
.text {
  font-family: Ahem;
  font-size: 10px;
  font-weight: 400;
  font-style: normal;
  line-height: 10px;
  white-space-collapse: collapse;
  text-wrap-mode: nowrap;
  word-break: normal;
  direction: ltr;
  line-break: auto;
  text-align: start;
  overflow-wrap: normal;
  text-transform: none;
  letter-spacing: normal;
  word-spacing: 0px;
  text-indent: 0px;
}
.rows .text { writing-mode: vertical-rl; }
.a { grid-column: 1 / 2; grid-row: 1 / 2; }
.b.columns { grid-column: 2 / 3; grid-row: 1 / 2; }
.b.rows { grid-column: 1 / 2; grid-row: 2 / 3; }
.c.columns { grid-column: 3 / 4; grid-row: 1 / 2; }
.c.rows { grid-column: 1 / 2; grid-row: 3 / 4; }
.a, .c { background-color: #2864dc; color: #2864dc; }
.visibility-hidden, .visibility-visible-control {
  grid-column: 1 / 2;
  grid-row: 1 / 2;
  background-color: #8e44ad;
  color: #8e44ad;
}
.visibility-hidden { visibility: hidden; }
.visibility-visible-control { visibility: visible; }
.hidden {
  display: none;
  grid-column: 1 / 2;
  grid-row: 1 / 2;
}
"#;

struct Fixture {
    _document: HostDocument,
    snapshot: HostDocumentSnapshot,
    root: HostNodeHandle,
    a: HostNodeHandle,
    b: HostNodeHandle,
    c: HostNodeHandle,
    view: StyloDocumentView,
}

enum NodeSpec {
    Element {
        tag: &'static str,
        id: String,
        class: String,
        attributes: Vec<(&'static str, String)>,
        children: Vec<NodeSpec>,
    },
    Text(String),
}

fn make_fixture(axis: Axis, a_length: usize, reversed: bool) -> Fixture {
    make_fixture_with_image_source(axis, a_length, reversed, IMAGE_URL)
}

fn make_fixture_with_image_source(
    axis: Axis,
    a_length: usize,
    reversed: bool,
    image_source: &str,
) -> Fixture {
    let a = NodeSpec::Element {
        tag: "div",
        id: "novel-a".to_owned(),
        class: "item text a".to_owned(),
        attributes: Vec::new(),
        children: vec![NodeSpec::Text("X".repeat(a_length))],
    };
    let b = NodeSpec::Element {
        tag: "img",
        id: "novel-b".to_owned(),
        class: format!("item image b {}", axis.class_name()),
        attributes: vec![("alt", String::new()), ("src", image_source.to_owned())],
        children: Vec::new(),
    };
    let c = NodeSpec::Element {
        tag: "div",
        id: "novel-c".to_owned(),
        class: format!("item text c {}", axis.class_name()),
        attributes: Vec::new(),
        children: vec![NodeSpec::Text("XXXX".to_owned())],
    };
    let children = if reversed {
        vec![c, b, a]
    } else {
        vec![a, b, c]
    };
    let root = NodeSpec::Element {
        tag: "div",
        id: "novel-root".to_owned(),
        class: format!("root {}", axis.class_name()),
        attributes: Vec::new(),
        children,
    };

    let mut document = HostDocument::new().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(11_489).unwrap(), document.document_revision());
    let root_handle = append_spec(&mut document, &mut batch, HostParent::Root, root);
    document.commit(batch).unwrap();
    let snapshot = document.snapshot();
    let view =
        StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), root_handle)
            .unwrap();
    let mut a_handle = None;
    let mut b_handle = None;
    let mut c_handle = None;
    for child in snapshot.children(root_handle).unwrap() {
        match snapshot.node(child).unwrap().kind() {
            HostNodeKind::Element(element) => match element.local_name() {
                "img" => b_handle = Some(child),
                "div" => {
                    let (_, _, id) = super::super::element_identity(&snapshot, child).unwrap();
                    match id.as_str() {
                        "novel-a" => a_handle = Some(child),
                        "novel-c" => c_handle = Some(child),
                        _ => unreachable!("fixture child ID"),
                    }
                }
                _ => unreachable!("fixture child tag"),
            },
            HostNodeKind::Text(_) => unreachable!("fixture root only contains elements"),
        }
    }
    Fixture {
        _document: document,
        snapshot,
        root: root_handle,
        a: a_handle.unwrap(),
        b: b_handle.unwrap(),
        c: c_handle.unwrap(),
        view,
    }
}

fn append_spec(
    document: &mut HostDocument,
    batch: &mut DocumentChangeBatch,
    parent: HostParent,
    spec: NodeSpec,
) -> HostNodeHandle {
    let handle = document.reserve_node_handle().unwrap();
    let children = match spec {
        NodeSpec::Text(value) => {
            batch.push(DocumentOperation::CreateText {
                node: handle,
                data: value.into(),
            });
            Vec::new()
        }
        NodeSpec::Element {
            tag,
            id,
            class,
            attributes,
            children,
        } => {
            batch.push(DocumentOperation::CreateElement {
                node: handle,
                namespace: "http://www.w3.org/1999/xhtml".to_owned(),
                local_name: tag.to_owned(),
            });
            for (name, value) in [("id", id), ("class", class)].into_iter().chain(attributes) {
                batch.push(DocumentOperation::SetAttribute {
                    node: handle,
                    name: AttributeName::new(None, name).unwrap(),
                    value: value.into(),
                });
            }
            children
        }
    };
    batch.push(DocumentOperation::InsertBefore {
        parent,
        node: handle,
        before: None,
    });
    for child in children {
        append_spec(document, batch, HostParent::Node(handle), child);
    }
    handle
}

fn cascade(fixture: &Fixture) -> spinon_style::ComputedStyleSnapshot {
    compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
        &[StylesheetSource {
            id: "c11-4-generic-mixed-intrinsic-flex-fixed".to_owned(),
            base_url: "https://spinon.invalid/c11-4/mixed-flex.css".to_owned(),
            origin: CssOrigin::Author,
            css: STYLESHEET.to_owned(),
        }],
        VIEWPORT,
        StyleRevision::INITIAL,
    )
    .unwrap()
}

#[test]
fn eight_column_and_row_scenes_use_the_generic_grid_sizing_path() {
    for axis in [Axis::Columns, Axis::Rows] {
        for (a_length, reversed, first, last) in [
            (10, false, 100.0, 140.0),
            (4, false, 80.0, 160.0),
            (8, false, 80.0, 160.0),
            (10, true, 100.0, 140.0),
        ] {
            let fixture = make_fixture(axis, a_length, reversed);
            let styles = cascade(&fixture);
            let input = super::super::prepare(&fixture.snapshot, fixture.root, &styles)
                .unwrap_or_else(|error| panic!("unexpected generic adapter error: {error:?}"))
                .expect("novel direct text/image/text scene routes generically");
            assert!(!input.use_extended_fixture_projection);
            assert_eq!(input.paragraph_styles.len(), 2);
            let paragraph = &input.paragraph_styles[&fixture.a.id()];
            assert_eq!(paragraph.white_space, spinon_layout::WhiteSpaceMode::NoWrap);
            assert_eq!(
                paragraph.writing_mode,
                if axis == Axis::Rows {
                    spinon_core::WritingMode::VerticalRl
                } else {
                    spinon_core::WritingMode::HorizontalTb
                }
            );

            let output = spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_and_intrinsic_sizes(
                &fixture.snapshot,
                fixture.root,
                styles,
                VIEWPORT,
                &input.paragraph_styles,
                &input.engine,
                input.intrinsic_sizes,
            )
            .unwrap_or_else(|error| panic!("generic Grid layout failed: {error:?}"));
            let root = output.layout.frames[&fixture.root.id()];
            let first_item = output.layout.frames[&fixture.a.id()];
            let image = output.layout.frames[&fixture.b.id()];
            let last_item = output.layout.frames[&fixture.c.id()];
            assert_eq!((root.x, root.y), (0.0, 0.0));
            match axis {
                Axis::Columns => {
                    close(root.width, 300.0, "root width");
                    close(root.height, 120.0, "root height");
                    close(first_item.width, first, "first intrinsic track");
                    close(first_item.height, 120.0, "first item cross size");
                    close(image.x, first + 10.0, "image x");
                    close(image.y, 0.0, "image y");
                    close(last_item.x, first + 60.0, "last track x");
                    close(last_item.width, last, "last flexible track");
                    close(last_item.height, 120.0, "last item cross size");
                }
                Axis::Rows => {
                    close(root.width, 120.0, "root width");
                    close(root.height, 300.0, "root height");
                    close(first_item.width, 120.0, "first item cross size");
                    close(first_item.height, first, "first intrinsic track");
                    close(image.x, 0.0, "image x");
                    close(image.y, first + 10.0, "image y");
                    close(last_item.y, first + 60.0, "last track y");
                    close(last_item.width, 120.0, "last item cross size");
                    close(last_item.height, last, "last flexible track");
                }
            }
            close(image.width, 24.0, "image natural width");
            close(image.height, 16.0, "image natural height");

            let trace = output
                .layout
                .grid_track_sizing_traces
                .iter()
                .find(|trace| trace.node == fixture.root.id())
                .expect("generic root track-sizing trace");
            let target_axis = if axis == Axis::Rows {
                LayoutGridTrackSizingAxis::Rows
            } else {
                LayoutGridTrackSizingAxis::Columns
            };
            let pass = trace
                .passes
                .iter()
                .filter(|pass| pass.axis == target_axis)
                .max_by_key(|pass| pass.sequence)
                .expect("target track-sizing pass");
            let bases = pass
                .stages
                .last()
                .expect("final track-sizing stage")
                .tracks
                .iter()
                .filter(|track| !track.is_gutter && !track.is_collapsed)
                .map(|track| track.base_size)
                .collect::<Vec<_>>();
            assert_eq!(bases.len(), 3);
            close(bases[0], first, "first trace base");
            close(bases[1], 40.0, "fixed trace base");
            close(bases[2], last, "last trace base");

            let source_order = |node| {
                output
                    .layout
                    .grid_item_intrinsic_inputs
                    .iter()
                    .find(|input| input.node == node)
                    .expect("intrinsic contribution input")
                    .source_order
            };
            assert_eq!(source_order(fixture.a.id()), if reversed { 2 } else { 0 });
            assert_eq!(source_order(fixture.b.id()), 1);
            assert_eq!(source_order(fixture.c.id()), if reversed { 0 } else { 2 });
        }
    }
}

#[test]
fn unsupported_text_and_item_placement_fail_closed() {
    let bad_text = make_fixture(Axis::Columns, 7, false);
    let bad_text_styles = cascade(&bad_text);
    assert!(super::super::prepare(&bad_text.snapshot, bad_text.root, &bad_text_styles).is_err());

    let fixture = make_fixture(Axis::Rows, 10, true);
    let mut styles = cascade(&fixture);
    let item = Arc::make_mut(&mut styles.elements)
        .iter_mut()
        .find(|style| style.node_id == fixture.a.id())
        .unwrap();
    item.grid_item_placement = None;
    assert!(super::super::prepare(&fixture.snapshot, fixture.root, &styles).is_err());
}

#[test]
fn wrong_image_source_fails_closed_in_both_axes() {
    for axis in [Axis::Columns, Axis::Rows] {
        let fixture = make_fixture_with_image_source(
            axis,
            10,
            false,
            "https://spinon.invalid/not-the-fixture.png",
        );
        let styles = cascade(&fixture);
        assert_image_input_rejected(&fixture, &styles, super::super::PNG);
    }
}

#[test]
fn undecodable_image_bytes_fail_closed_in_both_axes() {
    for axis in [Axis::Columns, Axis::Rows] {
        let fixture = make_fixture(axis, 10, false);
        let styles = cascade(&fixture);
        assert_image_input_rejected(&fixture, &styles, b"not a PNG");
    }
}

#[test]
fn unexpected_decoded_image_size_fails_closed_in_both_axes() {
    const WRONG_SIZE_PNG: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/fixtures/resources/c18-4-async-a-slow.png"
    ));
    let decoded = spinon_image::PngDecoder
        .decode(WRONG_SIZE_PNG)
        .expect("wrong-size control is a valid, decodable PNG");
    assert_eq!(
        (decoded.dimensions().width(), decoded.dimensions().height()),
        (2, 2)
    );

    for axis in [Axis::Columns, Axis::Rows] {
        let fixture = make_fixture(axis, 10, false);
        let styles = cascade(&fixture);
        assert_image_input_rejected(&fixture, &styles, WRONG_SIZE_PNG);
    }
}

fn assert_image_input_rejected(
    fixture: &Fixture,
    styles: &spinon_style::ComputedStyleSnapshot,
    png: &[u8],
) {
    let error = match super::prepare_with_png(&fixture.snapshot, fixture.root, styles, png) {
        Err(error) => error,
        Ok(_) => panic!("invalid mixed image input unexpectedly produced a fixture layout"),
    };
    assert_eq!(error.code, "unsupported_c114_generic_intrinsic_flex_fixed");
    assert_eq!(error.node_id, Some(fixture.b.id().get()));
}

fn close(actual: f32, expected: f32, label: &str) {
    assert!(
        (actual - expected).abs() <= 0.05,
        "{label}: expected {expected}, got {actual}"
    );
}

impl Axis {
    fn class_name(self) -> &'static str {
        match self {
            Self::Columns => "columns",
            Self::Rows => "rows",
        }
    }
}

#[path = "negative_controls.rs"]
mod negative_controls;

#[path = "tests/hidden.rs"]
mod hidden;
#[path = "tests/visibility_hidden.rs"]
mod visibility_hidden;
