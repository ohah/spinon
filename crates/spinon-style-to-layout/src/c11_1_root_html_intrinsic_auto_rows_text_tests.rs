use std::collections::BTreeMap;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_layout::{
    LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
    LayoutGridTrackSizingStageKind, ParagraphStyle, TextLayoutEngine, TextParagraphDirection,
    WhiteSpaceMode,
};
use spinon_style::{
    ComputedCssGridAutoRows, ComputedCssGridTrackSize, CssMediaEnvironment, CssViewport,
    StyloDocumentView, compute_runtime_root_html_text_paint_cascade_with_stylesheets,
};
use style::context::QuirksMode;

use crate::{StyleLayoutError, compute_runtime_style_layout_with_root_html_grid_text_profiles};

#[path = "c11_1_root_html_intrinsic_auto_rows_text_tests/combined_multiple_text.rs"]
mod combined_multiple_text;
#[path = "c11_1_root_html_intrinsic_auto_rows_text_tests/letter_words.rs"]
mod letter_words;
#[path = "c11_1_root_html_intrinsic_auto_rows_text_tests/normal_whitespace.rs"]
mod normal_whitespace;
#[path = "c11_1_root_html_intrinsic_auto_rows_text_tests/track_limit.rs"]
mod track_limit;

const HTML: &str = "http://www.w3.org/1999/xhtml";
const TEXT: &str = "A A A A A A";

struct Fixture {
    document: HostDocument,
    root: HostNodeHandle,
    grid: HostNodeHandle,
    first: HostNodeHandle,
    wrapped_text: HostNodeHandle,
    text_source: HostNodeHandle,
    last: HostNodeHandle,
    additional_text_items: Vec<(HostNodeHandle, HostNodeHandle)>,
}

impl Fixture {
    fn new() -> Self {
        Self::with_tracks("40px", 40.0, "0")
    }

    fn with_tracks(columns: &str, item_width: f32, row_gap: &str) -> Self {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let grid = document.reserve_node_handle().unwrap();
        let first = document.reserve_node_handle().unwrap();
        let wrapped_text = document.reserve_node_handle().unwrap();
        let text_source = document.reserve_node_handle().unwrap();
        let last = document.reserve_node_handle().unwrap();
        let grid_style = format!(
            "display:grid;box-sizing:border-box;width:40px;height:40px;grid-template-columns:{columns};grid-template-rows:10px;grid-auto-flow:row;grid-auto-rows:min-content;row-gap:{row_gap};column-gap:0;align-content:start;justify-content:start;margin:0;padding:0;border:0;overflow:visible"
        );
        let first_style = format!(
            "display:block;box-sizing:border-box;width:{item_width}px;height:10px;margin:0;padding:0;border:0;position:static"
        );
        let last_style = format!(
            "display:block;box-sizing:border-box;width:{item_width}px;height:34px;margin:0;padding:0;border:0;position:static"
        );
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_914).unwrap(), document.document_revision());
        for (node, parent, style) in [
            (
                root,
                HostParent::Root,
                "display:block;box-sizing:border-box;width:120px;height:90px;margin:0;padding:0",
            ),
            (grid, HostParent::Node(root), grid_style.as_str()),
            (first, HostParent::Node(grid), first_style.as_str()),
            (
                wrapped_text,
                HostParent::Node(grid),
                "display:block;box-sizing:border-box;width:auto;height:auto;min-height:auto;max-height:none;margin:0;padding:0;border:0;position:static;font-family:monospace;font-size:10px;line-height:10px;white-space:normal",
            ),
            (last, HostParent::Node(grid), last_style.as_str()),
        ] {
            batch.push(DocumentOperation::CreateElement {
                node,
                namespace: HTML.to_owned(),
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
        batch.push(DocumentOperation::CreateText {
            node: text_source,
            data: TEXT.into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(wrapped_text),
            node: text_source,
            before: None,
        });
        document.commit(batch).unwrap();
        Self {
            document,
            root,
            grid,
            first,
            wrapped_text,
            text_source,
            last,
            additional_text_items: Vec::new(),
        }
    }

    fn add_text_item_before(
        &mut self,
        text: &str,
        before: Option<HostNodeHandle>,
    ) -> (HostNodeHandle, HostNodeHandle) {
        let item = self.document.reserve_node_handle().unwrap();
        let text_source = self.document.reserve_node_handle().unwrap();
        let mut batch = DocumentChangeBatch::new(
            OwnerId::new(11_914).unwrap(),
            self.document.document_revision(),
        );
        batch.push(DocumentOperation::CreateElement {
            node: item,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: item,
            name: AttributeName::new(None, "style").unwrap(),
            value: "display:block;box-sizing:border-box;width:auto;height:auto;min-height:auto;max-height:none;margin:0;padding:0;border:0;position:static;font-family:monospace;font-size:10px;line-height:10px;white-space:normal".into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(self.grid),
            node: item,
            before,
        });
        batch.push(DocumentOperation::CreateText {
            node: text_source,
            data: text.into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(item),
            node: text_source,
            before: None,
        });
        self.document.commit(batch).unwrap();
        self.additional_text_items.push((item, text_source));
        (item, text_source)
    }

    fn set_text_source(&mut self, text: &str) {
        let mut batch = DocumentChangeBatch::new(
            OwnerId::new(11_914).unwrap(),
            self.document.document_revision(),
        );
        batch.push(DocumentOperation::SetTextData {
            node: self.text_source,
            data: text.into(),
        });
        self.document.commit(batch).unwrap();
    }

    fn add_empty_items(&mut self, count: usize) -> Option<HostNodeHandle> {
        let mut batch = DocumentChangeBatch::new(
            OwnerId::new(11_914).unwrap(),
            self.document.document_revision(),
        );
        let mut last_item = None;
        for _ in 0..count {
            let item = self.document.reserve_node_handle().unwrap();
            last_item = Some(item);
            batch.push(DocumentOperation::CreateElement {
                node: item,
                namespace: HTML.to_owned(),
                local_name: "div".to_owned(),
            });
            batch.push(DocumentOperation::SetAttribute {
                node: item,
                name: AttributeName::new(None, "style").unwrap(),
                value: "display:block;box-sizing:border-box;width:1px;height:1px;margin:0;padding:0;border:0;position:static".into(),
            });
            batch.push(DocumentOperation::InsertBefore {
                parent: HostParent::Node(self.grid),
                node: item,
                before: None,
            });
        }
        self.document.commit(batch).unwrap();
        last_item
    }

    fn compute(&self) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
        let snapshot = self.document.snapshot();
        let viewport = CssViewport {
            width_css_px: 120.0,
            height_css_px: 90.0,
            device_scale_factor: 1.0,
            environment_revision: Default::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        };
        let view = StyloDocumentView::new(snapshot.clone(), self.root, true, QuirksMode::NoQuirks)
            .unwrap();
        let computed_styles = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
            &view,
            &[],
            viewport,
            StyleRevision::INITIAL,
        )
        .map_err(StyleLayoutError::Cascade)?;
        let mut text_items = vec![self.wrapped_text];
        text_items.extend(self.additional_text_items.iter().map(|(item, _)| *item));
        let mut paragraphs = BTreeMap::new();
        for item in text_items {
            let text_style = computed_styles
                .elements
                .iter()
                .find(|style| style.node_id == item.id())
                .expect("each text owner should have a computed style in the same snapshot");
            assert_eq!(text_style.properties["font-family"], "monospace");
            assert_eq!(text_style.properties["line-height"], "10px");
            assert_eq!(text_style.font_size_css_px, 10.0);
            assert_eq!(
                text_style.layout_dimensions.width,
                spinon_style::ComputedCssDimension::Auto
            );
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
            viewport,
            &paragraphs,
            &BTreeMap::new(),
            &text_engine,
        )
    }
}

#[test]
fn parsed_root_html_intrinsic_rows_measure_multiple_text_items_by_identity_and_order() {
    let mut fixture = Fixture::new();
    let last = fixture.last;
    let (short_text, short_source) = fixture.add_text_item_before("A", Some(last));
    let output = fixture
        .compute()
        .expect("multiple direct text items should each contribute to a min-content row");
    let frame = |node: HostNodeHandle| {
        let frame = output.layout.frames[&node.id()];
        [frame.x, frame.y, frame.width, frame.height]
    };
    assert_eq!(frame(fixture.grid), [0.0, 0.0, 40.0, 40.0]);
    assert_eq!(frame(fixture.first), [0.0, 0.0, 40.0, 10.0]);
    assert_eq!(frame(fixture.wrapped_text), [0.0, 10.0, 40.0, 20.0]);
    assert_eq!(frame(short_text), [0.0, 30.0, 40.0, 10.0]);
    assert_eq!(frame(fixture.last), [0.0, 40.0, 40.0, 34.0]);
    assert_eq!(frame(fixture.last)[1] + frame(fixture.last)[3], 74.0);

    let long_layout = &output.layout.text_layouts[&fixture.wrapped_text.id()];
    let short_layout = &output.layout.text_layouts[&short_text.id()];
    assert_eq!(long_layout.input.sources[0].text, TEXT);
    assert_eq!(long_layout.layout.lines.len(), 2);
    assert_eq!(long_layout.layout.height, 20.0);
    assert_eq!(long_layout.revision, output.layout.revision);
    assert_eq!(short_layout.input.sources[0].node, short_source.id());
    assert_eq!(short_layout.input.sources[0].text, "A");
    assert_eq!(short_layout.layout.lines.len(), 1);
    assert_eq!(short_layout.layout.height, 10.0);
    assert_eq!(short_layout.revision, output.layout.revision);

    let trace = output
        .layout
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == fixture.grid.id())
        .expect("the multi-text Grid should retain its sizing trace");
    assert_eq!(trace.used_rows.used_track_list, "10px 20px 10px 34px");
    let row_pass = trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .expect("the row-axis sizing pass should be recorded");
    for (item, value) in [(fixture.wrapped_text, 20.0), (short_text, 10.0)] {
        let contribution = row_pass
            .contributions
            .iter()
            .find(|contribution| contribution.item == item.id())
            .unwrap_or_else(|| panic!("missing row contribution for {:?}", item.id()));
        assert_eq!(
            contribution.kind,
            LayoutGridTrackSizingContributionKind::MinContent
        );
        assert_eq!(contribution.value, value);
    }

    let mut reversed = Fixture::new();
    let long_text = reversed.wrapped_text;
    let (reversed_short, _) = reversed.add_text_item_before("A", Some(long_text));
    let reversed_output = reversed
        .compute()
        .expect("reversing direct text source order should preserve both contributions");
    let reversed_frame = |node: HostNodeHandle| {
        let frame = reversed_output.layout.frames[&node.id()];
        [frame.x, frame.y, frame.width, frame.height]
    };
    assert_eq!(reversed_frame(reversed.first), [0.0, 0.0, 40.0, 10.0]);
    assert_eq!(reversed_frame(reversed_short), [0.0, 10.0, 40.0, 10.0]);
    assert_eq!(reversed_frame(long_text), [0.0, 20.0, 40.0, 20.0]);
    assert_eq!(reversed_frame(reversed.last), [0.0, 40.0, 40.0, 34.0]);
    let reversed_trace = reversed_output
        .layout
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == reversed.grid.id())
        .expect("the reversed Grid should retain its sizing trace");
    assert_eq!(
        reversed_trace.used_rows.used_track_list,
        "10px 10px 20px 34px"
    );
}

#[test]
fn parsed_root_html_min_content_implicit_rows_use_the_direct_item_text_measure() {
    let fixture = Fixture::new();
    let output = fixture
        .compute()
        .expect("one direct text child should contribute through the default Root HTML route");
    let computed_grid = output
        .computed_styles
        .elements
        .iter()
        .find(|style| style.node_id == fixture.grid.id())
        .and_then(|style| style.grid_layout.as_ref())
        .expect("computed Grid tracks should be present");
    assert_eq!(
        computed_grid.auto_rows,
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::MinContent])
    );

    let frame = |node: HostNodeHandle| {
        let frame = output.layout.frames[&node.id()];
        [frame.x, frame.y, frame.width, frame.height]
    };
    assert_eq!(frame(fixture.grid), [0.0, 0.0, 40.0, 40.0]);
    assert_eq!(frame(fixture.first), [0.0, 0.0, 40.0, 10.0]);
    assert_eq!(frame(fixture.wrapped_text), [0.0, 10.0, 40.0, 20.0]);
    assert_eq!(frame(fixture.last), [0.0, 30.0, 40.0, 34.0]);
    assert!(frame(fixture.last)[1] + frame(fixture.last)[3] > frame(fixture.grid)[3]);

    let text_layout = &output.layout.text_layouts[&fixture.wrapped_text.id()];
    assert_eq!(text_layout.revision, output.layout.revision);
    assert_eq!(text_layout.input.sources.len(), 1);
    assert_eq!(text_layout.input.sources[0].node, fixture.text_source.id());
    assert_eq!(text_layout.input.sources[0].text, TEXT);
    assert_eq!(text_layout.layout.lines.len(), 2);
    assert_eq!(text_layout.layout.height, 20.0);

    let trace = output
        .layout
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == fixture.grid.id())
        .expect("the bounded min-content implicit-row route should record its track trace");
    assert_eq!(trace.used_rows.used_track_list, "10px 20px 34px");
    let row_pass = trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .expect("row sizing should be present in the Grid trace");
    let text_contribution = row_pass
        .contributions
        .iter()
        .find(|contribution| contribution.item == fixture.wrapped_text.id())
        .expect("the row trace should retain direct text item identity");
    assert_eq!(
        text_contribution.kind,
        LayoutGridTrackSizingContributionKind::MinContent
    );
    assert_eq!(text_contribution.value, 20.0);
    let final_stage = row_pass
        .stages
        .iter()
        .rev()
        .find(|stage| stage.stage == LayoutGridTrackSizingStageKind::IntrinsicSizingComplete)
        .expect("the min-content row sizing stage should be recorded");
    assert_eq!(
        final_stage
            .tracks
            .iter()
            .filter(|track| !track.is_gutter && !track.is_collapsed)
            .map(|track| track.base_size)
            .collect::<Vec<_>>(),
        vec![10.0, 20.0, 34.0]
    );
}

#[test]
fn parsed_root_html_intrinsic_columns_combine_with_direct_text_min_content_rows() {
    for (columns, item_width, row_gap, expected_column_range, expected_lines, expected_row) in [
        ("min-content", 0.0, "0", (6.0, 7.0), 6, 60.0),
        ("max-content", 0.0, "0", (60.0, 70.0), 1, 10.0),
        ("minmax(10px,max-content)", 0.0, "0", (40.0, 40.0), 2, 20.0),
        ("auto", 0.0, "0", (40.0, 40.0), 2, 20.0),
        ("minmax(min-content,1fr)", 0.0, "0", (40.0, 40.0), 2, 20.0),
        ("min-content", 0.0, "5%", (6.0, 7.0), 6, 60.0),
    ] {
        let fixture = Fixture::with_tracks(columns, item_width, row_gap);
        let output = fixture
            .compute()
            .unwrap_or_else(|error| panic!("columns={columns}, row-gap={row_gap}: {error:?}"));
        let frame = |node: HostNodeHandle| {
            let frame = output.layout.frames[&node.id()];
            [frame.x, frame.y, frame.width, frame.height]
        };
        let text_frame = frame(fixture.wrapped_text);
        let expected_text_y = if row_gap == "5%" { 12.0 } else { 10.0 };
        assert_eq!(frame(fixture.grid), [0.0, 0.0, 40.0, 40.0]);
        assert_eq!(text_frame[1], expected_text_y, "columns={columns}");
        assert!(
            text_frame[2] >= expected_column_range.0 && text_frame[2] <= expected_column_range.1,
            "columns={columns}: expected width in {:?}, got {}",
            expected_column_range,
            text_frame[2]
        );
        assert!(
            (text_frame[3] - expected_row).abs() <= 0.02,
            "columns={columns}: expected height {expected_row}, got {}",
            text_frame[3]
        );
        let text_layout = &output.layout.text_layouts[&fixture.wrapped_text.id()];
        assert_eq!(
            text_layout.layout.lines.len(),
            expected_lines,
            "columns={columns}"
        );
        assert_eq!(text_layout.layout.height, expected_row, "columns={columns}");
        let last_frame = frame(fixture.last);
        let expected_last_y = if row_gap == "5%" {
            74.0
        } else {
            10.0 + expected_row
        };
        assert_eq!(last_frame[1], expected_last_y, "columns={columns}");
        if expected_row == 60.0 {
            assert!(last_frame[1] + last_frame[3] > frame(fixture.grid)[3]);
        }
        let trace = output
            .layout
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == fixture.grid.id())
            .expect("combined intrinsic sizing should retain its trace");
        let rows = &trace.used_rows.used_track_list;
        assert_eq!(
            rows,
            if expected_row == 60.0 {
                "10px 60px 34px"
            } else if expected_row == 20.0 {
                "10px 20px 34px"
            } else {
                "10px 10px 34px"
            },
            "columns={columns}"
        );
    }
}
