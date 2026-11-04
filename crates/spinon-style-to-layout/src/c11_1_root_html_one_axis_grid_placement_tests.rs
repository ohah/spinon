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

const HTML: &str = "http://www.w3.org/1999/xhtml";

struct GridFixture {
    document: HostDocument,
    root: HostNodeHandle,
    items_by_grid: Vec<Vec<HostNodeHandle>>,
}

impl GridFixture {
    fn new(grid_specs: &[(&str, &[&str])]) -> Self {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let grids = grid_specs
            .iter()
            .map(|_| document.reserve_node_handle().unwrap())
            .collect::<Vec<_>>();
        let items_by_grid = grid_specs
            .iter()
            .map(|(_, items)| {
                items
                    .iter()
                    .map(|_| document.reserve_node_handle().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_681).unwrap(), document.document_revision());

        create_element(
            &mut batch,
            root,
            HostParent::Root,
            "display:block;box-sizing:border-box;width:400px;height:300px;margin:0;padding:0",
        );
        for ((grid_style, item_styles), (grid, items)) in grid_specs
            .iter()
            .zip(grids.iter().copied().zip(&items_by_grid))
        {
            create_element(&mut batch, grid, HostParent::Node(root), grid_style);
            for (item, item_style) in items.iter().copied().zip(*item_styles) {
                create_element(&mut batch, item, HostParent::Node(grid), item_style);
            }
        }
        document.commit(batch).unwrap();
        Self {
            document,
            root,
            items_by_grid,
        }
    }

    fn compute(&self) -> Result<crate::StyleLayoutOutput, crate::StyleLayoutError> {
        compute_document(&self.document, self.root)
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
) -> Result<crate::StyleLayoutOutput, crate::StyleLayoutError> {
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
    .map_err(crate::StyleLayoutError::Cascade)?;
    crate::compute_runtime_style_layout_with_root_html_grid_text_profiles(
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

#[test]
fn root_html_one_axis_placement_isolated_between_sibling_grid_owners() {
    let row_a = item_style("grid-row:1 / 2;");
    let row_b = item_style("grid-row:1 / 2;");
    let column_a = item_style("grid-column:2 / 3;");
    let column_b = item_style("grid-column:2 / 3;");
    let fixture = GridFixture::new(&[
        (&grid_style(""), &[row_a.as_str(), row_b.as_str()]),
        (&grid_style(""), &[column_a.as_str(), column_b.as_str()]),
    ]);
    let output = fixture.compute().unwrap();
    let first_grid = &fixture.items_by_grid[0];
    let second_grid = &fixture.items_by_grid[1];
    assert_eq!(frame(&output, first_grid[0]), [0.0, 0.0, 40.0, 20.0]);
    assert_eq!(frame(&output, first_grid[1]), [40.0, 0.0, 80.0, 20.0]);
    assert_eq!(frame(&output, second_grid[0]), [40.0, 50.0, 80.0, 20.0]);
    assert_eq!(frame(&output, second_grid[1]), [40.0, 70.0, 80.0, 30.0]);
}

#[test]
fn root_html_one_axis_placement_ignores_display_none_and_counts_visibility_hidden() {
    let display_none = item_style("display:none;grid-column:1 / 2;");
    let visible_first = item_style("grid-column:1 / 2;");
    let visibility_hidden = item_style("visibility:hidden;grid-column:1 / 2;");
    let visible_last = item_style("grid-column:1 / 2;");
    let grid_style = "display:grid;box-sizing:border-box;width:120px;height:90px;grid-template-columns:40px 80px;grid-template-rows:20px 30px 40px;gap:0;margin:0;padding:0;border:0;overflow:visible;grid-auto-rows:9px;grid-auto-columns:7px";
    let fixture = GridFixture::new(&[(
        grid_style,
        &[
            display_none.as_str(),
            visible_first.as_str(),
            visibility_hidden.as_str(),
            visible_last.as_str(),
        ],
    )]);
    let output = fixture.compute().unwrap();
    let items = &fixture.items_by_grid[0];
    assert_eq!(frame(&output, items[1]), [0.0, 0.0, 40.0, 20.0]);
    assert_eq!(frame(&output, items[2]), [0.0, 20.0, 40.0, 30.0]);
    assert_eq!(frame(&output, items[3]), [0.0, 50.0, 40.0, 40.0]);
}

#[test]
fn root_html_one_axis_placement_keeps_all_auto_items_on_the_existing_route() {
    let first = item_style("");
    let second = item_style("");
    let fixture = GridFixture::new(&[(&grid_style(""), &[first.as_str(), second.as_str()])]);
    let output = fixture.compute().unwrap();
    let items = &fixture.items_by_grid[0];
    assert_eq!(frame(&output, items[0]), [0.0, 0.0, 40.0, 20.0]);
    assert_eq!(frame(&output, items[1]), [40.0, 0.0, 80.0, 20.0]);
}

#[test]
fn root_html_one_axis_placement_rejects_mixed_axes_and_nested_grid_items() {
    let row_locked = item_style("grid-row:1 / 2;");
    let column_locked = item_style("grid-column:1 / 2;");
    let mixed = GridFixture::new(&[(
        &grid_style(""),
        &[row_locked.as_str(), column_locked.as_str()],
    )]);
    assert!(mixed.compute().is_err());

    let nested = item_style(
        "display:grid;grid-template-columns:40px 80px;grid-template-rows:20px 30px;grid-column:1 / 2;",
    );
    let nested_fixture = GridFixture::new(&[(&grid_style(""), &[nested.as_str()])]);
    assert!(nested_fixture.compute().is_err());
}
