use std::collections::BTreeMap;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_layout::TextLayoutEngine;
use spinon_style::{
    CssMediaEnvironment, CssViewport, StyloDocumentView,
    compute_runtime_root_html_text_paint_cascade_with_stylesheets,
};
use style::context::QuirksMode;

use crate::{StyleLayoutError, compute_runtime_style_layout_with_root_html_grid_text_profiles};

const HTML: &str = "http://www.w3.org/1999/xhtml";

struct RootHtmlGridFixture {
    document: HostDocument,
    root: HostNodeHandle,
    grid: HostNodeHandle,
    items: Vec<HostNodeHandle>,
}

impl RootHtmlGridFixture {
    fn new(grid_style: &str, item_styles: &[String]) -> Self {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let grid = document.reserve_node_handle().unwrap();
        let items = item_styles
            .iter()
            .map(|_| document.reserve_node_handle().unwrap())
            .collect::<Vec<_>>();
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_195).unwrap(), document.document_revision());
        for (node, parent, style) in [
            (
                root,
                HostParent::Root,
                "display:block;box-sizing:border-box;width:120px;height:120px;margin:0;padding:0",
            ),
            (grid, HostParent::Node(root), grid_style),
        ]
        .into_iter()
        .chain(
            items
                .iter()
                .copied()
                .zip(item_styles.iter().map(String::as_str))
                .map(|(node, style)| (node, HostParent::Node(grid), style)),
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
        document.commit(batch).unwrap();
        Self {
            document,
            root,
            grid,
            items,
        }
    }

    fn compute(&self) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
        let snapshot = self.document.snapshot();
        let view = StyloDocumentView::new(snapshot.clone(), self.root, true, QuirksMode::NoQuirks)
            .unwrap();
        let viewport = viewport();
        let computed_styles = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
            &view,
            &[],
            viewport,
            StyleRevision::INITIAL,
        )
        .map_err(StyleLayoutError::Cascade)?;
        compute_runtime_style_layout_with_root_html_grid_text_profiles(
            &snapshot,
            self.root,
            computed_styles,
            viewport,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &TextLayoutEngine::default(),
        )
    }
}

fn viewport() -> CssViewport {
    CssViewport {
        width_css_px: 120.0,
        height_css_px: 120.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    }
}

fn item_styles(heights: &[f32]) -> Vec<String> {
    heights
        .iter()
        .copied()
        .enumerate()
        .map(|(index, height)| {
            let width = if index % 2 == 0 { 40 } else { 55 };
            format!(
                "display:block;box-sizing:border-box;width:{width}px;height:{height}px;margin:0;padding:0"
            )
        })
        .collect()
}

#[test]
fn parsed_root_html_percentage_auto_row_list_reaches_taffy_used_geometry() {
    let fixture = RootHtmlGridFixture::new(
        "display:grid;box-sizing:border-box;width:100px;height:100px;grid-template-columns:40px 55px;grid-template-rows:20px;grid-auto-rows:10% 20%;column-gap:5px;row-gap:5px;margin:0;padding:0",
        &item_styles(&[20.0, 20.0, 10.0, 10.0, 20.0, 20.0, 10.0, 10.0, 20.0, 20.0]),
    );
    let output = fixture
        .compute()
        .expect("parsed Root HTML percentages should project through the runtime path");
    let grid = output
        .computed_styles
        .elements
        .iter()
        .find(|style| style.node_id == fixture.grid.id())
        .and_then(|style| style.grid_layout.as_ref())
        .expect("the parsed Grid style should be present");

    assert_eq!(
        grid.auto_rows,
        spinon_style::ComputedCssGridAutoRows::TrackList(vec![
            spinon_style::ComputedCssGridTrackSize::Percentage(0.1),
            spinon_style::ComputedCssGridTrackSize::Percentage(0.2),
        ])
    );
    let frames = fixture
        .items
        .iter()
        .map(|item| {
            let frame = output.layout.frames[&item.id()];
            [frame.x, frame.y, frame.width, frame.height]
        })
        .collect::<Vec<_>>();
    assert_eq!(
        frames,
        vec![
            [0.0, 0.0, 40.0, 20.0],
            [45.0, 0.0, 55.0, 20.0],
            [0.0, 25.0, 40.0, 10.0],
            [45.0, 25.0, 55.0, 10.0],
            [0.0, 40.0, 40.0, 20.0],
            [45.0, 40.0, 55.0, 20.0],
            [0.0, 65.0, 40.0, 10.0],
            [45.0, 65.0, 55.0, 10.0],
            [0.0, 80.0, 40.0, 20.0],
            [45.0, 80.0, 55.0, 20.0],
        ]
    );
}

#[test]
fn parsed_root_html_zero_and_over_hundred_percent_implicit_rows_keep_css_geometry() {
    let bounds = RootHtmlGridFixture::new(
        "display:grid;box-sizing:border-box;width:100px;height:100px;grid-template-columns:40px 55px;grid-template-rows:20px;grid-auto-rows:0% 120%;column-gap:5px;row-gap:5px;margin:0;padding:0",
        &item_styles(&[20.0, 20.0, 0.0, 0.0, 120.0, 120.0]),
    );
    let bounds_output = bounds.compute().unwrap();
    let last = bounds_output.layout.frames[&bounds.items[5].id()];
    assert_eq!(
        [last.x, last.y, last.width, last.height],
        [45.0, 30.0, 55.0, 120.0]
    );
    assert_eq!(last.y + last.height - 100.0, 50.0);

    let zero = RootHtmlGridFixture::new(
        "display:grid;box-sizing:border-box;width:100px;height:0px;grid-template-columns:40px 55px;grid-template-rows:0px;grid-auto-rows:50%;column-gap:5px;row-gap:5px;margin:0;padding:0",
        &item_styles(&[0.0, 0.0, 7.0]),
    );
    let zero_output = zero.compute().unwrap();
    let overflowing_item = zero_output.layout.frames[&zero.items[2].id()];
    assert_eq!(
        [
            overflowing_item.x,
            overflowing_item.y,
            overflowing_item.width,
            overflowing_item.height,
        ],
        [0.0, 5.0, 40.0, 7.0]
    );
}

#[test]
fn parsed_root_html_percentage_auto_rows_require_a_definite_unconstrained_height() {
    for (height_rules, property) in [
        ("height:auto", "height"),
        ("height:100px;min-height:1px", "min-height"),
        ("height:100px;max-height:120px", "max-height"),
        ("height:100px;aspect-ratio:1", "aspect-ratio"),
        ("height:calc(50px + 50px)", "height"),
    ] {
        let style = format!(
            "display:grid;box-sizing:border-box;width:100px;{height_rules};grid-template-columns:40px 55px;grid-template-rows:20px;grid-auto-rows:10%;column-gap:5px;row-gap:5px;margin:0;padding:0"
        );
        let fixture = RootHtmlGridFixture::new(&style, &item_styles(&[20.0, 20.0]));
        let error = fixture.compute().unwrap_err();
        assert!(
            matches!(
                &error,
                StyleLayoutError::UnsupportedComputedValue {
                    node,
                    property: actual,
                    ..
                } if *node == fixture.grid.id() && *actual == property
            ),
            "{height_rules} should reject {property}, got {error:?}"
        );
    }
}

#[test]
fn parsed_root_html_percentage_auto_rows_keep_percentage_gap_unsupported() {
    let fixture = RootHtmlGridFixture::new(
        "display:grid;box-sizing:border-box;width:100px;height:100px;grid-template-columns:40px 55px;grid-template-rows:20px;grid-auto-rows:10%;column-gap:5px;row-gap:5%;margin:0;padding:0",
        &item_styles(&[20.0, 20.0]),
    );
    let error = fixture.compute().unwrap_err();
    assert!(matches!(
        error,
        StyleLayoutError::UnsupportedComputedValue {
            node,
            property: "row-gap",
            ..
        } if node == fixture.grid.id()
    ));
}
