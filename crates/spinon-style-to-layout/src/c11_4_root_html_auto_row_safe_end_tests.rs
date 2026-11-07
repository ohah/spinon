use std::collections::BTreeMap;

use serde_json::Value;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_layout::{ParagraphStyle, TextLayoutEngine, TextParagraphDirection, WhiteSpaceMode};
use spinon_style::{
    CssMediaEnvironment, CssViewport, StyloDocumentView,
    compute_runtime_root_html_text_paint_cascade_with_stylesheets,
};
use style::context::QuirksMode;

use crate::{StyleLayoutError, compute_runtime_style_layout_with_root_html_grid_text_profiles};

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
const EXPECTED: &str = include_str!(
    "../../../spec/internal/evidence/c11-root-html-auto-row-safe-end-precomparison-2026-11-07/expected-values.json"
);
const FLEX_END_EXPECTED: &str = include_str!(
    "../../../spec/internal/evidence/c11-root-html-auto-row-safe-flex-end-precomparison-2026-11-07/expected.json"
);
const TOLERANCE: f32 = 1.0 / 64.0;

struct AutoRowGridFixture {
    document: HostDocument,
    root: HostNodeHandle,
    grid: HostNodeHandle,
    items: Vec<HostNodeHandle>,
    viewport: CssViewport,
}

impl AutoRowGridFixture {
    fn new(content_alignment: &str, row_gap: &str) -> Self {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let grid = document.reserve_node_handle().unwrap();
        let items = (0..3)
            .map(|_| document.reserve_node_handle().unwrap())
            .collect::<Vec<_>>();
        let text_sources = (0..2)
            .map(|_| document.reserve_node_handle().unwrap())
            .collect::<Vec<_>>();
        let grid_style = format!(
            "display:grid;box-sizing:border-box;width:40px;height:40px;grid-template-columns:40px;grid-template-rows:10px;grid-auto-rows:auto;grid-auto-flow:row;align-content:{content_alignment};align-items:normal;row-gap:{row_gap};column-gap:0;overflow:visible;margin:0;padding:0;font-family:monospace;font-size:10px;font-weight:400;font-style:normal;line-height:10px"
        );
        let text_style = "display:block;box-sizing:border-box;width:auto;height:auto;min-height:auto;max-height:none;margin:0;padding:0;border:0;position:static;font-family:monospace;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space:normal";
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_304).unwrap(), document.document_revision());
        for (node, parent, style) in [
            (
                root,
                HostParent::Root,
                "display:block;box-sizing:border-box;width:300px;height:200px;margin:0;padding:0",
            ),
            (grid, HostParent::Node(root), grid_style.as_str()),
            (
                items[0],
                HostParent::Node(grid),
                "display:block;box-sizing:border-box;width:40px;height:10px;margin:0;padding:0",
            ),
            (items[1], HostParent::Node(grid), text_style),
            (items[2], HostParent::Node(grid), text_style),
        ]
        .into_iter()
        {
            batch.push(DocumentOperation::CreateElement {
                node,
                namespace: HTML_NAMESPACE.to_owned(),
                local_name: "div".to_owned(),
            });
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "style").unwrap(),
                value: style.to_owned().into(),
            });
            batch.push(DocumentOperation::InsertBefore {
                parent,
                node,
                before: None,
            });
        }
        for (text, item) in text_sources.iter().copied().zip([items[1], items[2]]) {
            batch.push(DocumentOperation::CreateText {
                node: text,
                data: if item == items[1] { "A" } else { "B" }.into(),
            });
            batch.push(DocumentOperation::InsertBefore {
                parent: HostParent::Node(item),
                node: text,
                before: None,
            });
        }
        document.commit(batch).unwrap();
        Self {
            document,
            root,
            grid,
            items,
            viewport: CssViewport {
                width_css_px: 390.0,
                height_css_px: 844.0,
                device_scale_factor: 1.0,
                environment_revision: Default::default(),
                media_environment: CssMediaEnvironment::MOBILE,
            },
        }
    }

    fn compute(&self) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
        let snapshot = self.document.snapshot();
        let view = StyloDocumentView::new(snapshot.clone(), self.root, true, QuirksMode::NoQuirks)
            .unwrap();
        let computed_styles = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
            &view,
            &[],
            self.viewport,
            StyleRevision::INITIAL,
        )
        .map_err(StyleLayoutError::Cascade)?;
        let mut paragraphs = BTreeMap::new();
        for item in self.items.iter().copied().skip(1) {
            let text_style = computed_styles
                .elements
                .iter()
                .find(|style| style.node_id == item.id())
                .expect("each direct-text item should have a computed style");
            paragraphs.insert(
                item.id(),
                ParagraphStyle {
                    font_family: text_style.properties["font-family"].clone(),
                    font_size: text_style.font_size_css_px,
                    line_height: 10.0,
                    white_space: WhiteSpaceMode::Normal,
                    direction: TextParagraphDirection::Ltr,
                    ..ParagraphStyle::default()
                },
            );
        }
        let text_engine = TextLayoutEngine::new_for_runtime();
        compute_runtime_style_layout_with_root_html_grid_text_profiles(
            &snapshot,
            self.root,
            computed_styles,
            self.viewport,
            &paragraphs,
            &BTreeMap::new(),
            &text_engine,
        )
    }
}

#[test]
fn root_html_auto_row_safe_end_style_projection_matches_expected_frames() {
    let expected: Value =
        serde_json::from_str(EXPECTED).expect("safe-end expected values should be valid JSON");
    for (group_name, alignment, row_gap) in [
        ("positiveSpace", "safe end", "0px"),
        ("positiveSpace", "unsafe end", "0px"),
        ("positiveSpace", "end", "0px"),
        ("overflow", "safe end", "calc(120% - 2px)"),
        ("overflow", "unsafe end", "calc(120% - 2px)"),
        ("overflow", "end", "calc(120% - 2px)"),
    ] {
        let fixture = AutoRowGridFixture::new(alignment, row_gap);
        let output = fixture
            .compute()
            .unwrap_or_else(|error| panic!("{group_name} / {alignment}: {error:?}"));
        let grid = output.layout.frames[&fixture.grid.id()];
        let expected_y = expected[group_name]["expectedYRelativeCssPx"][alignment]
            .as_array()
            .expect("expected scene should list item y offsets");
        for (index, expected) in expected_y.iter().enumerate() {
            let expected_y = expected.as_f64().expect("y offset should be numeric") as f32;
            let actual = output.layout.frames[&fixture.items[index].id()];
            let actual_y = actual.y - grid.y;
            assert!(
                (actual_y - expected_y).abs() <= TOLERANCE,
                "{group_name} / {alignment} item {index}: expected y={expected_y}, got {actual_y}"
            );
            assert_eq!(actual.height, 10.0, "item row height should remain fixed");
        }
    }
}

#[test]
fn root_html_auto_row_safe_flex_end_style_projection_matches_chrome_matrix() {
    let expected: Value = serde_json::from_str(FLEX_END_EXPECTED)
        .expect("safe flex-end expected values should be valid JSON");
    let alignments = expected["alignmentValues"]
        .as_array()
        .expect("expected values should list alignments");

    for (gap_case, row_gap) in [("positive", "0px"), ("overflow", "calc(120% - 2px)")] {
        for alignment in alignments {
            let alignment = alignment.as_str().expect("alignment should be a string");
            let expected_frames =
                &expected["expectedFramesByAlignmentAndGapCase"][alignment][gap_case];
            let fixture = AutoRowGridFixture::new(alignment, row_gap);
            let output = fixture
                .compute()
                .unwrap_or_else(|error| panic!("{gap_case} / {alignment}: {error:?}"));
            let grid_style = output
                .computed_styles
                .elements
                .iter()
                .find(|style| style.node_id == fixture.grid.id())
                .expect("Root Grid should keep a computed-style snapshot");
            assert_eq!(
                grid_style
                    .properties
                    .get("align-content")
                    .map(String::as_str),
                Some(alignment),
                "{gap_case} should preserve the exact computed alignment"
            );
            let grid = output.layout.frames[&fixture.grid.id()];
            for (index, expected_frame) in expected_frames
                .as_array()
                .expect("expected scene should list item frames")
                .iter()
                .enumerate()
            {
                let frame = output.layout.frames[&fixture.items[index].id()];
                let expected_x = expected_frame["x"].as_f64().unwrap() as f32;
                let expected_y = expected_frame["y"].as_f64().unwrap() as f32;
                assert!(
                    ((frame.x - grid.x) - expected_x).abs() <= TOLERANCE,
                    "{gap_case} / {alignment} / item {index}: expected x={expected_x}, got {}",
                    frame.x - grid.x
                );
                assert!(
                    ((frame.y - grid.y) - expected_y).abs() <= TOLERANCE,
                    "{gap_case} / {alignment} / item {index}: expected y={expected_y}, got {}",
                    frame.y - grid.y
                );
                assert_eq!(
                    frame.width,
                    expected_frame["width"].as_f64().unwrap() as f32
                );
                assert_eq!(
                    frame.height,
                    expected_frame["height"].as_f64().unwrap() as f32
                );
            }
        }
    }
}
