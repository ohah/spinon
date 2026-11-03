use std::collections::BTreeMap;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_style::{
    CssMediaEnvironment, CssViewport, StyloDocumentView,
    compute_runtime_root_html_text_paint_cascade_with_stylesheets,
};
use style::context::QuirksMode;

use crate::{StyleLayoutError, compute_runtime_style_layout_with_root_html_grid_text_profiles};

const HTML: &str = "http://www.w3.org/1999/xhtml";

struct GridFixture {
    document: HostDocument,
    root: HostNodeHandle,
    items: Vec<HostNodeHandle>,
}

impl GridFixture {
    fn new(grid_style: &str, item_styles: &[&str], text_after_item: Option<usize>) -> Self {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let grid = document.reserve_node_handle().unwrap();
        let items = (0..item_styles.len())
            .map(|_| document.reserve_node_handle().unwrap())
            .collect::<Vec<_>>();
        let text = text_after_item.map(|_| document.reserve_node_handle().unwrap());
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_681).unwrap(), document.document_revision());

        for (node, parent, style) in [
            (
                root,
                HostParent::Root,
                "display:block;box-sizing:border-box;width:400px;height:300px;margin:0;padding:0",
            ),
            (grid, HostParent::Node(root), grid_style),
        ] {
            create_element(&mut batch, node, parent, style);
        }
        for (index, (item, style)) in items.iter().copied().zip(item_styles).enumerate() {
            create_element(&mut batch, item, HostParent::Node(grid), style);
            if text_after_item == Some(index) {
                let text = text.expect("selected text child was allocated");
                batch.push(DocumentOperation::CreateText {
                    node: text,
                    data: "TEXT".into(),
                });
                batch.push(DocumentOperation::InsertBefore {
                    parent: HostParent::Node(grid),
                    node: text,
                    before: None,
                });
            }
        }
        document.commit(batch).unwrap();

        Self {
            document,
            root,
            items,
        }
    }

    fn compute(&self) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
        compute_document(&self.document, self.root)
    }

    fn set_item_style(&mut self, index: usize, style: &str) {
        let mut batch = DocumentChangeBatch::new(
            OwnerId::new(11_681).unwrap(),
            self.document.document_revision(),
        );
        batch.push(DocumentOperation::SetAttribute {
            node: self.items[index],
            name: AttributeName::new(None, "style").unwrap(),
            value: style.to_owned().into(),
        });
        self.document.commit(batch).unwrap();
    }

    fn item_frame(&self, output: &crate::StyleLayoutOutput, index: usize) -> [f32; 4] {
        frame(output, self.items[index])
    }
}

fn create_element(
    batch: &mut DocumentChangeBatch,
    node: HostNodeHandle,
    parent: HostParent,
    style: &str,
) {
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

fn compute_document(
    document: &HostDocument,
    root: HostNodeHandle,
) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
    let snapshot = document.snapshot();
    let viewport = CssViewport {
        width_css_px: 400.0,
        height_css_px: 300.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    };
    let view = StyloDocumentView::new(snapshot.clone(), root, true, QuirksMode::NoQuirks).unwrap();
    let computed_styles = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
        &view,
        &[],
        viewport,
        StyleRevision::INITIAL,
    )
    .map_err(StyleLayoutError::Cascade)?;
    compute_runtime_style_layout_with_root_html_grid_text_profiles(
        &snapshot,
        root,
        computed_styles,
        viewport,
        &BTreeMap::new(),
        &BTreeMap::new(),
        &spinon_layout::TextLayoutEngine::default(),
    )
}

fn grid_style(extra: &str) -> String {
    format!(
        "display:grid;box-sizing:border-box;width:120px;height:50px;grid-template-columns:40px 80px;grid-template-rows:20px 30px;gap:0;margin:0;padding:0;border:0;overflow:visible;{extra}"
    )
}

fn item_style(extra: &str) -> String {
    format!(
        "display:block;box-sizing:border-box;width:auto;height:auto;margin:0;padding:0;border:0;overflow:visible;{extra}"
    )
}

fn frame(output: &crate::StyleLayoutOutput, node: HostNodeHandle) -> [f32; 4] {
    let frame = output.layout.frames[&node.id()];
    [frame.x, frame.y, frame.width, frame.height]
}

fn assert_frame(actual: [f32; 4], expected: [f32; 4]) {
    for (actual, expected) in actual.into_iter().zip(expected) {
        assert!(
            (actual - expected).abs() <= 1.0 / 64.0,
            "expected {expected}px ± 1/64px, got {actual}px"
        );
    }
}

#[test]
fn root_html_explicit_numeric_lines_spans_shorthands_and_reversed_endpoints_match_expected_frames()
{
    let grid_style = grid_style("");
    let positive_styles = [
        item_style("grid-row:1 / 2;grid-column:2 / 3"),
        item_style("grid-row:2 / 3;grid-column:1 / 2"),
        item_style("grid-row:1 / 2;grid-column:1 / 2"),
        item_style("grid-row:2 / 3;grid-column:2 / 3"),
    ];
    let positive_refs = positive_styles
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let positive = GridFixture::new(&grid_style, &positive_refs, None);
    let output = positive.compute().unwrap();
    for (index, expected) in [
        [40.0, 0.0, 80.0, 20.0],
        [0.0, 20.0, 40.0, 30.0],
        [0.0, 0.0, 40.0, 20.0],
        [40.0, 20.0, 80.0, 30.0],
    ]
    .into_iter()
    .enumerate()
    {
        assert_frame(positive.item_frame(&output, index), expected);
    }

    let negative_styles = [
        item_style("grid-row:-2 / -1;grid-column:-2 / -1"),
        item_style("grid-row:2 / 1;grid-column:2 / 1"),
    ];
    let negative_refs = negative_styles
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let negative = GridFixture::new(&grid_style, &negative_refs, None);
    let output = negative.compute().unwrap();
    assert_frame(negative.item_frame(&output, 0), [40.0, 20.0, 80.0, 30.0]);
    assert_frame(negative.item_frame(&output, 1), [0.0, 0.0, 40.0, 20.0]);

    let shorthand_styles = [
        item_style("grid-area:1 / 2 / 2 / 3"),
        item_style("grid-row:2 / span 1;grid-column:1 / span 1"),
        item_style("grid-area:2 / 2 / 3 / 3;grid-row-start:1"),
    ];
    let shorthand_refs = shorthand_styles
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let shorthand = GridFixture::new(&grid_style, &shorthand_refs, None);
    let output = shorthand.compute().unwrap();
    assert_frame(shorthand.item_frame(&output, 0), [40.0, 0.0, 80.0, 20.0]);
    assert_frame(shorthand.item_frame(&output, 1), [0.0, 20.0, 40.0, 30.0]);
    assert_frame(shorthand.item_frame(&output, 2), [40.0, 0.0, 80.0, 50.0]);
}

#[test]
fn root_html_explicit_overlap_and_hidden_items_preserve_item_ownership() {
    let grid_style = grid_style("");
    let overlap_styles = [
        item_style("grid-row:1 / 2;grid-column:1 / 2"),
        item_style("grid-row:1 / 2;grid-column:1 / 2"),
    ];
    let overlap_refs = overlap_styles
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let overlap = GridFixture::new(&grid_style, &overlap_refs, None);
    let output = overlap.compute().unwrap();
    assert_frame(overlap.item_frame(&output, 0), [0.0, 0.0, 40.0, 20.0]);
    assert_frame(overlap.item_frame(&output, 1), [0.0, 0.0, 40.0, 20.0]);

    let hidden_styles = [
        item_style("grid-row:1 / 2;grid-column:2 / 3"),
        item_style("display:none"),
    ];
    let hidden_refs = hidden_styles.iter().map(String::as_str).collect::<Vec<_>>();
    let hidden = GridFixture::new(&grid_style, &hidden_refs, None);
    let output = hidden.compute().unwrap();
    assert_frame(hidden.item_frame(&output, 0), [40.0, 0.0, 80.0, 20.0]);
    let hidden_frame = hidden.item_frame(&output, 1);
    assert_eq!([hidden_frame[2], hidden_frame[3]], [0.0, 0.0]);
}

#[test]
fn root_html_explicit_placement_rejects_mixed_partial_implicit_and_anonymous_auto_items() {
    let base_grid_style = grid_style("");
    let mixed_styles = [
        item_style("grid-row:1 / 2;grid-column:2 / 3"),
        item_style(""),
    ];
    let mixed_refs = mixed_styles.iter().map(String::as_str).collect::<Vec<_>>();
    let mut mixed = GridFixture::new(&base_grid_style, &mixed_refs, None);
    assert!(mixed.compute().is_err());
    mixed.set_item_style(1, &item_style("grid-row:2 / 3;grid-column:1 / 2"));
    let recovered = mixed.compute().unwrap();
    assert_frame(mixed.item_frame(&recovered, 0), [40.0, 0.0, 80.0, 20.0]);
    assert_frame(mixed.item_frame(&recovered, 1), [0.0, 20.0, 40.0, 30.0]);

    let partial_styles = [
        item_style("grid-column:1 / 2"),
        item_style("grid-row:2 / 3;grid-column:2 / 3"),
    ];
    let partial_refs = partial_styles
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    assert!(
        GridFixture::new(&base_grid_style, &partial_refs, None)
            .compute()
            .is_err()
    );

    let implicit_style = grid_style("grid-auto-rows:8px");
    let implicit_item = item_style("grid-row:3 / 4;grid-column:1 / 2");
    assert!(
        GridFixture::new(&implicit_style, &[&implicit_item], None)
            .compute()
            .is_err()
    );

    let text_styles = [
        item_style("grid-row:2 / 3;grid-column:2 / 3"),
        item_style("grid-row:1 / 2;grid-column:1 / 2"),
    ];
    let text_refs = text_styles.iter().map(String::as_str).collect::<Vec<_>>();
    assert!(
        GridFixture::new(&base_grid_style, &text_refs, Some(0))
            .compute()
            .is_err()
    );
}

#[test]
fn root_html_explicit_placement_composes_with_c11_5_self_alignment() {
    let grid_style = grid_style("");
    let item = item_style(
        "width:16px;height:10px;grid-row:1 / 2;grid-column:1 / 2;align-self:end;justify-self:end",
    );
    let fixture = GridFixture::new(&grid_style, &[&item], None);
    let output = fixture.compute().unwrap();
    assert_frame(fixture.item_frame(&output, 0), [24.0, 10.0, 16.0, 10.0]);
}

#[test]
fn root_html_explicit_item_can_paint_outside_its_grid_when_overflow_is_visible() {
    let grid_style = grid_style("");
    let item = item_style(
        "box-sizing:border-box;width:100px;height:30px;grid-row:1 / 2;grid-column:2 / 3",
    );
    let fixture = GridFixture::new(&grid_style, &[&item], None);
    let output = fixture.compute().unwrap();
    assert_frame(fixture.item_frame(&output, 0), [40.0, 0.0, 100.0, 30.0]);
}

#[test]
fn root_html_explicit_route_isolated_from_all_auto_sibling_grid() {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let explicit_grid = document.reserve_node_handle().unwrap();
    let explicit_item = document.reserve_node_handle().unwrap();
    let auto_grid = document.reserve_node_handle().unwrap();
    let auto_item = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(11_682).unwrap(), document.document_revision());
    create_element(
        &mut batch,
        root,
        HostParent::Root,
        "display:block;box-sizing:border-box;width:400px;height:300px;margin:0;padding:0",
    );
    let explicit_grid_style = grid_style("");
    let explicit_grid_style = explicit_grid_style.as_str();
    create_element(
        &mut batch,
        explicit_grid,
        HostParent::Node(root),
        explicit_grid_style,
    );
    let explicit_item_style = item_style("width:80px;height:20px;grid-row:2 / 3;grid-column:2 / 3");
    create_element(
        &mut batch,
        explicit_item,
        HostParent::Node(explicit_grid),
        &explicit_item_style,
    );
    create_element(
        &mut batch,
        auto_grid,
        HostParent::Node(root),
        explicit_grid_style,
    );
    let auto_item_style = item_style("width:40px;height:20px");
    create_element(
        &mut batch,
        auto_item,
        HostParent::Node(auto_grid),
        &auto_item_style,
    );
    document.commit(batch).unwrap();

    let output = compute_document(&document, root).unwrap();
    assert_frame(frame(&output, explicit_item), [40.0, 20.0, 80.0, 20.0]);
    assert_frame(frame(&output, auto_item), [0.0, 50.0, 40.0, 20.0]);
}
