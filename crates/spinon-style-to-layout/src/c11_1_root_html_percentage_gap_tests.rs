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
    items: Vec<HostNodeHandle>,
}

impl Fixture {
    fn new(grid_style: &str, item_styles: &[&str]) -> Self {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let grid = document.reserve_node_handle().unwrap();
        let items = item_styles
            .iter()
            .map(|_| document.reserve_node_handle().unwrap())
            .collect::<Vec<_>>();
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_196).unwrap(), document.document_revision());
        for (node, parent, style) in [
            (
                root,
                HostParent::Root,
                "display:block;box-sizing:border-box;width:300px;height:200px;margin:0;padding:0",
            ),
            (grid, HostParent::Node(root), grid_style),
        ]
        .into_iter()
        .chain(
            items
                .iter()
                .copied()
                .zip(item_styles.iter().copied())
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
        let viewport = CssViewport {
            width_css_px: 300.0,
            height_css_px: 200.0,
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

    fn item_frame(&self, output: &crate::StyleLayoutOutput, index: usize) -> [f32; 4] {
        let frame = output.layout.frames[&self.items[index].id()];
        [frame.x, frame.y, frame.width, frame.height]
    }
}

fn auto_items(count: usize) -> Vec<&'static str> {
    vec!["display:block;box-sizing:border-box;width:10px;height:10px;margin:0;padding:0"; count]
}

#[test]
fn percentage_gaps_resolve_against_their_own_axes_and_shorthand_order() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:200px;height:100px;grid-template-columns:40px 1fr;grid-template-rows:20px 1fr;column-gap:10%;row-gap:10%;margin:0;padding:0",
        &auto_items(4),
    );
    let output = fixture.compute().unwrap();
    assert_eq!(fixture.item_frame(&output, 1), [60.0, 0.0, 10.0, 10.0]);
    assert_eq!(fixture.item_frame(&output, 2), [0.0, 30.0, 10.0, 10.0]);
    assert_eq!(fixture.item_frame(&output, 3), [60.0, 30.0, 10.0, 10.0]);

    for gap_property in ["gap:5% 10%", "grid-gap:5% 10%"] {
        let style = format!(
            "display:grid;box-sizing:border-box;width:200px;height:100px;grid-template-columns:30px 1fr;grid-template-rows:10px 1fr;{gap_property};margin:0;padding:0"
        );
        let fixture = Fixture::new(&style, &auto_items(4));
        let output = fixture.compute().unwrap();
        assert_eq!(fixture.item_frame(&output, 1), [50.0, 0.0, 10.0, 10.0]);
        assert_eq!(fixture.item_frame(&output, 2), [0.0, 15.0, 10.0, 10.0]);
        assert_eq!(fixture.item_frame(&output, 3), [50.0, 15.0, 10.0, 10.0]);
    }
}

#[test]
fn zero_and_over_hundred_percent_gaps_keep_their_used_geometry() {
    let zero = Fixture::new(
        "display:grid;box-sizing:border-box;width:0px;height:20px;grid-template-columns:0px 0px;grid-template-rows:20px;column-gap:75%;row-gap:0%;margin:0;padding:0",
        &auto_items(2),
    );
    let output = zero.compute().unwrap();
    assert_eq!(zero.item_frame(&output, 0), [0.0, 0.0, 10.0, 10.0]);
    assert_eq!(zero.item_frame(&output, 1), [0.0, 0.0, 10.0, 10.0]);

    let overflow = Fixture::new(
        "display:grid;box-sizing:border-box;width:100px;height:20px;grid-template-columns:10px 10px;grid-template-rows:20px;column-gap:120%;row-gap:0;margin:0;padding:0",
        &auto_items(2),
    );
    let output = overflow.compute().unwrap();
    assert_eq!(overflow.item_frame(&output, 1), [130.0, 0.0, 10.0, 10.0]);
    assert_eq!(output.layout.frames[&overflow.grid.id()].width, 100.0);
}

#[test]
fn simulator_fixture_keeps_percentage_gap_geometry_and_visible_overflow() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:200px;height:200px;grid-template-columns:20px 20px;grid-template-rows:20px 20px;column-gap:120%;row-gap:10%;margin:0;padding:0;overflow:visible",
        &[
            "display:block;box-sizing:border-box;width:20px;height:20px;margin:0;padding:0",
            "display:block;box-sizing:border-box;width:20px;height:20px;margin:0;padding:0",
            "display:block;box-sizing:border-box;width:20px;height:20px;margin:0;padding:0",
            "display:block;box-sizing:border-box;width:20px;height:20px;margin:0;padding:0",
        ],
    );
    let output = fixture.compute().unwrap();
    assert_eq!(output.layout.frames[&fixture.grid.id()].width, 200.0);
    assert_eq!(output.layout.frames[&fixture.grid.id()].height, 200.0);
    assert_eq!(
        (0..4)
            .map(|index| fixture.item_frame(&output, index))
            .collect::<Vec<_>>(),
        [
            [0.0, 0.0, 20.0, 20.0],
            [260.0, 0.0, 20.0, 20.0],
            [0.0, 40.0, 20.0, 20.0],
            [260.0, 40.0, 20.0, 20.0],
        ]
    );
}

#[test]
fn percentage_gaps_compose_with_implicit_rows_and_column_flow_columns() {
    let rows = Fixture::new(
        "display:grid;box-sizing:border-box;width:100px;height:100px;grid-template-columns:40px 55px;grid-template-rows:20px;grid-auto-rows:25%;column-gap:5px;row-gap:10%;margin:0;padding:0",
        &auto_items(3),
    );
    let output = rows.compute().unwrap();
    assert_eq!(rows.item_frame(&output, 2), [0.0, 30.0, 10.0, 10.0]);

    let columns = Fixture::new(
        "display:grid;box-sizing:border-box;width:200px;height:100px;grid-template-columns:40px;grid-template-rows:20px 20px;grid-auto-columns:50%;grid-auto-flow:column;column-gap:10%;row-gap:0;margin:0;padding:0",
        &[
            "display:block;box-sizing:border-box;width:10px;height:10px;margin:0;padding:0",
            "display:block;box-sizing:border-box;width:10px;height:10px;margin:0;padding:0",
            "display:block;box-sizing:border-box;width:100px;height:10px;margin:0;padding:0",
        ],
    );
    let output = columns.compute().unwrap();
    assert_eq!(columns.item_frame(&output, 2), [60.0, 0.0, 100.0, 10.0]);
}

#[test]
fn fixed_named_area_accepts_percentage_and_existing_px_gaps() {
    for (gaps, expected_header_width, expected_lower_y, expected_right_x) in [
        ("column-gap:10%;row-gap:10%", 132.0, 27.0, 52.0),
        ("column-gap:6px;row-gap:4px", 126.0, 24.0, 46.0),
    ] {
        let header_style = format!(
            "display:block;box-sizing:border-box;width:{expected_header_width}px;height:20px;grid-area:header;margin:0;padding:0"
        );
        let items = [
            header_style.as_str(),
            "display:block;box-sizing:border-box;width:40px;height:50px;grid-area:side;margin:0;padding:0",
            "display:block;box-sizing:border-box;width:80px;height:50px;grid-area:content;margin:0;padding:0",
        ];
        let fixture = Fixture::new(
            &format!(
                "display:grid;box-sizing:border-box;width:120px;height:70px;grid-template-columns:40px 80px;grid-template-rows:20px 50px;grid-template-areas:'header header' 'side content';{gaps};margin:0;padding:0"
            ),
            &items,
        );
        let output = fixture.compute().unwrap();
        assert_eq!(
            fixture.item_frame(&output, 0),
            [0.0, 0.0, expected_header_width, 20.0]
        );
        assert_eq!(fixture.item_frame(&output, 1)[1], expected_lower_y);
        assert_eq!(fixture.item_frame(&output, 2)[0], expected_right_x);
    }
}

#[test]
fn invalid_negative_gap_cascades_but_math_and_indefinite_basis_fail_closed() {
    let invalid = Fixture::new(
        "display:grid;box-sizing:border-box;width:100px;height:20px;grid-template-columns:10px 10px;grid-template-rows:20px;column-gap:5px;column-gap:-10px;row-gap:0;margin:0;padding:0",
        &auto_items(2),
    );
    let output = invalid.compute().unwrap();
    assert_eq!(invalid.item_frame(&output, 1)[0], 15.0);

    for (style, property) in [
        (
            "display:grid;box-sizing:border-box;width:200px;height:20px;grid-template-columns:10px 10px;grid-template-rows:20px;column-gap:calc(5% + 2px);margin:0;padding:0",
            "column-gap",
        ),
        (
            "display:grid;box-sizing:border-box;height:20px;grid-template-columns:10px 10px;grid-template-rows:20px;column-gap:10%;margin:0;padding:0",
            "column-gap",
        ),
        (
            "display:grid;box-sizing:border-box;width:100px;min-width:1px;height:20px;grid-template-columns:10px 10px;grid-template-rows:20px;column-gap:10%;margin:0;padding:0",
            "min-width",
        ),
    ] {
        let fixture = Fixture::new(style, &auto_items(2));
        let result = fixture.compute();
        assert!(
            matches!(
                &result,
                Err(StyleLayoutError::UnsupportedComputedValue {
                    node,
                    property: actual,
                    ..
                }) if *node == fixture.grid.id() && *actual == property
            ),
            "{style} produced {result:?}, expected {property}"
        );
    }
}
