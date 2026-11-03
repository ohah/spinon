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

struct Fixture {
    document: HostDocument,
    root: HostNodeHandle,
    grid: HostNodeHandle,
    items: [HostNodeHandle; 2],
}

impl Fixture {
    fn new(grid_style: &str, item_styles: [&str; 2]) -> Self {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let grid = document.reserve_node_handle().unwrap();
        let items = [
            document.reserve_node_handle().unwrap(),
            document.reserve_node_handle().unwrap(),
        ];
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_260).unwrap(), document.document_revision());
        for (node, parent, style) in [
            (
                root,
                HostParent::Root,
                "display:block;box-sizing:border-box;width:400px;height:400px;margin:0;padding:0;border:0",
            ),
            (grid, HostParent::Node(root), grid_style),
            (items[0], HostParent::Node(grid), item_styles[0]),
            (items[1], HostParent::Node(grid), item_styles[1]),
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
        let viewport = CssViewport {
            width_css_px: 400.0,
            height_css_px: 400.0,
            device_scale_factor: 1.0,
            environment_revision: Default::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        };
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

fn grid_style(width: &str, height: &str, columns: &str, rows: &str, extra: &str) -> String {
    format!(
        "display:grid;box-sizing:content-box;width:{width};height:{height};grid-template-columns:{columns};grid-template-rows:{rows};gap:0;margin:0;padding:0;border:2px solid;overflow:visible;{extra}"
    )
}

fn item_style(width: u32, height: u32, extra: &str) -> String {
    format!(
        "display:block;box-sizing:content-box;width:{width}px;height:{height}px;min-width:auto;min-height:auto;margin:0;padding:0;border:0;overflow:visible;{extra}"
    )
}

fn assert_frame(actual: spinon_layout::LayoutFrame, expected: [f32; 4], name: &str) {
    for (component, (actual, expected)) in [actual.x, actual.y, actual.width, actual.height]
        .into_iter()
        .zip(expected)
        .enumerate()
    {
        assert!(
            actual.is_finite() && (actual - expected).abs() <= 0.02,
            "{name} frame component {component}: actual={actual:?}, expected={expected:?} ± 0.02"
        );
    }
}

#[test]
fn parsed_root_html_auto_minimum_flex_tracks_reach_the_default_profile_on_both_axes() {
    for (
        name,
        width,
        height,
        columns,
        rows,
        first,
        second,
        expected_root,
        expected_first,
        expected_second,
    ) in [
        (
            "columns-auto-minimum",
            "100px",
            "40px",
            "minmax(auto,1fr) 40px",
            "40px",
            (80, 40),
            (40, 40),
            [0.0, 0.0, 104.0, 44.0],
            [2.0, 2.0, 80.0, 40.0],
            [82.0, 2.0, 40.0, 40.0],
        ),
        (
            "columns-zero-track-control",
            "100px",
            "40px",
            "minmax(0px,1fr) 40px",
            "40px",
            (80, 40),
            (40, 40),
            [0.0, 0.0, 104.0, 44.0],
            [2.0, 2.0, 80.0, 40.0],
            [62.0, 2.0, 40.0, 40.0],
        ),
        (
            "columns-zero-fraction",
            "100px",
            "40px",
            "minmax(auto,0fr) 40px",
            "40px",
            (80, 40),
            (40, 40),
            [0.0, 0.0, 104.0, 44.0],
            [2.0, 2.0, 80.0, 40.0],
            [82.0, 2.0, 40.0, 40.0],
        ),
        (
            "rows-auto-minimum",
            "40px",
            "100px",
            "40px",
            "minmax(auto,1fr) 40px",
            (40, 80),
            (40, 40),
            [0.0, 0.0, 44.0, 104.0],
            [2.0, 2.0, 40.0, 80.0],
            [2.0, 82.0, 40.0, 40.0],
        ),
        (
            "rows-zero-track-control",
            "40px",
            "100px",
            "40px",
            "minmax(0px,1fr) 40px",
            (40, 80),
            (40, 40),
            [0.0, 0.0, 44.0, 104.0],
            [2.0, 2.0, 40.0, 80.0],
            [2.0, 62.0, 40.0, 40.0],
        ),
        (
            "rows-zero-fraction",
            "40px",
            "100px",
            "40px",
            "minmax(auto,0fr) 40px",
            (40, 80),
            (40, 40),
            [0.0, 0.0, 44.0, 104.0],
            [2.0, 2.0, 40.0, 80.0],
            [2.0, 82.0, 40.0, 40.0],
        ),
    ] {
        let grid_style = grid_style(width, height, columns, rows, "");
        let first_style = item_style(first.0, first.1, "");
        let second_style = item_style(second.0, second.1, "");
        let fixture = Fixture::new(&grid_style, [&first_style, &second_style]);
        let output = fixture.compute().unwrap_or_else(|error| {
            panic!("{name} should reach default Root HTML profile: {error:?}")
        });

        assert_frame(
            output.layout.frames[&fixture.grid.id()],
            expected_root,
            name,
        );
        assert_frame(
            output.layout.frames[&fixture.items[0].id()],
            expected_first,
            name,
        );
        assert_frame(
            output.layout.frames[&fixture.items[1].id()],
            expected_second,
            name,
        );
    }
}

#[test]
fn parsed_root_html_auto_minimum_rejects_indefinite_or_overridden_grid_axis() {
    for (name, width, height, columns, rows, extra, expected_property) in [
        (
            "zero-column-axis",
            "0px",
            "40px",
            "minmax(auto,1fr) 40px",
            "40px",
            "",
            "width",
        ),
        (
            "auto-column-axis",
            "auto",
            "40px",
            "minmax(auto,1fr) 40px",
            "40px",
            "",
            "width",
        ),
        (
            "column-minimum-override",
            "100px",
            "40px",
            "minmax(auto,1fr) 40px",
            "40px",
            "min-width:1px",
            "min-width",
        ),
        (
            "column-maximum-override",
            "100px",
            "40px",
            "minmax(auto,1fr) 40px",
            "40px",
            "max-width:120px",
            "max-width",
        ),
        (
            "column-aspect-ratio",
            "100px",
            "40px",
            "minmax(auto,1fr) 40px",
            "40px",
            "aspect-ratio:1",
            "aspect-ratio",
        ),
        (
            "zero-row-axis",
            "40px",
            "0px",
            "40px",
            "minmax(auto,1fr) 40px",
            "",
            "height",
        ),
    ] {
        let grid_style = grid_style(width, height, columns, rows, extra);
        let first_style = item_style(80, 40, "");
        let second_style = item_style(40, 40, "");
        let fixture = Fixture::new(&grid_style, [&first_style, &second_style]);
        let error = fixture
            .compute()
            .expect_err("unsupported grid-axis constraints must fail before layout publication");
        assert!(
            matches!(
                error,
                StyleLayoutError::UnsupportedComputedValue { node, property, .. }
                    if node == fixture.grid.id() && property == expected_property
            ),
            "{name}: expected {expected_property}, got {error:?}"
        );
    }
}

#[test]
fn parsed_root_html_auto_minimum_rejects_scrollable_grid_item_overflow() {
    for overflow in ["hidden", "auto", "scroll", "clip"] {
        let grid_style = grid_style("100px", "40px", "minmax(auto,1fr) 40px", "40px", "");
        let first_style = item_style(80, 40, &format!("overflow:{overflow}"));
        let second_style = item_style(40, 40, "");
        let fixture = Fixture::new(&grid_style, [&first_style, &second_style]);
        let error = fixture
            .compute()
            .expect_err("non-visible item overflow is outside this bounded profile");
        assert!(matches!(
            error,
            StyleLayoutError::UnsupportedComputedValue { node, property, .. }
                if node == fixture.items[0].id() && property == "overflow-x"
        ));
    }
}
