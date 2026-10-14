use std::{collections::BTreeMap, sync::Arc};

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_style::{CssMediaEnvironment, CssViewport, StyloDocumentView};

use crate::{
    StyleLayoutError, StyleLayoutOutput, compute_runtime_block_flex_grid_template_style_layout,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";

struct Fixture {
    document: HostDocument,
    root: HostNodeHandle,
    nodes: BTreeMap<String, HostNodeHandle>,
}

impl Fixture {
    fn new(root_style: &str, item_styles: &[(&str, &str)]) -> Self {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let nodes = item_styles
            .iter()
            .map(|(id, _)| ((*id).to_owned(), document.reserve_node_handle().unwrap()))
            .collect::<BTreeMap<_, _>>();
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_233).unwrap(), document.document_revision());
        append_element(&mut batch, root, HostParent::Root, "grid", root_style);
        for (id, style) in item_styles {
            append_element(&mut batch, nodes[*id], HostParent::Node(root), id, style);
        }
        document.commit(batch).unwrap();
        Self {
            document,
            root,
            nodes,
        }
    }

    fn compute(&self) -> Result<StyleLayoutOutput, StyleLayoutError> {
        let snapshot = self.document.snapshot();
        let view =
            StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), self.root)
                .unwrap();
        compute_runtime_block_flex_grid_template_style_layout(
            &snapshot,
            &view,
            self.root,
            &[],
            CssViewport {
                width_css_px: 240.0,
                height_css_px: 100.0,
                device_scale_factor: 1.0,
                environment_revision: Default::default(),
                media_environment: CssMediaEnvironment::MOBILE,
            },
            StyleRevision::INITIAL,
        )
    }
}

fn append_element(
    batch: &mut DocumentChangeBatch,
    node: HostNodeHandle,
    parent: HostParent,
    id: &str,
    style: &str,
) {
    batch.push(DocumentOperation::CreateElement {
        node,
        namespace: HTML.to_owned(),
        local_name: "div".to_owned(),
    });
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, "id").unwrap(),
        value: id.to_owned().into(),
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

fn sized_area_item(area: &str, width: u32, height: u32) -> String {
    format!(
        "box-sizing:border-box;width:{width}px;height:{height}px;grid-area:{area};margin:0;padding:0;border:0"
    )
}

#[test]
fn explicit_areas_place_items_at_their_sparse_rectangles() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:240px;height:80px;grid-template-columns:80px 80px 80px;grid-template-rows:30px 50px;grid-template-areas:'head head .' 'nav main main';column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[
            ("head", &sized_area_item("head", 160, 30)),
            ("nav", &sized_area_item("nav", 80, 50)),
            ("main", &sized_area_item("main", 160, 50)),
        ],
    );
    let output = fixture.compute().unwrap();

    let frame = |id: &str| output.layout.frames[&fixture.nodes[id].id()];
    let head = frame("head");
    let nav = frame("nav");
    let main = frame("main");
    assert_eq!(
        (head.x, head.y, head.width, head.height),
        (0.0, 0.0, 160.0, 30.0)
    );
    assert_eq!(
        (nav.x, nav.y, nav.width, nav.height),
        (0.0, 30.0, 80.0, 50.0)
    );
    assert_eq!(
        (main.x, main.y, main.width, main.height),
        (80.0, 30.0, 160.0, 50.0)
    );
}

#[test]
fn auto_sized_area_item_stretches_to_its_named_area() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:160px;height:40px;grid-template-columns:40px 80px 40px;grid-template-rows:40px;grid-template-areas:'. hero .';column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[("hero", "grid-area:hero;margin:0;padding:0;border:0")],
    );
    let output = fixture.compute().unwrap();
    let frame = output.layout.frames[&fixture.nodes["hero"].id()];

    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        (40.0, 0.0, 80.0, 40.0)
    );
}

#[test]
fn bare_area_name_uses_first_area_alias_while_numeric_ident_uses_named_line() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:160px;height:40px;grid-template-columns:[hero-start] 40px [hero] 40px [hero-end] 80px;grid-template-rows:40px;grid-template-areas:'. hero .';column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[
            (
                "bare",
                "box-sizing:border-box;width:80px;height:40px;grid-row:1 / 2;grid-column-start:hero;grid-column-end:hero;margin:0;padding:0;border:0",
            ),
            (
                "numbered",
                "box-sizing:border-box;width:120px;height:40px;grid-row:1 / 2;grid-column-start:1 hero;grid-column-end:4;margin:0;padding:0;border:0",
            ),
        ],
    );
    let output = fixture.compute().unwrap();
    let bare = output.layout.frames[&fixture.nodes["bare"].id()];
    let numbered = output.layout.frames[&fixture.nodes["numbered"].id()];

    assert_eq!((bare.x, bare.width), (0.0, 80.0));
    assert_eq!((numbered.x, numbered.width), (40.0, 120.0));
}

#[test]
fn explicit_line_pairs_form_implicit_areas_on_both_axes() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:160px;height:80px;grid-template-columns:[box-start] 40px [box-end] 120px;grid-template-rows:[box-start] 30px [box-end] 50px;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[("implicit", &sized_area_item("box", 40, 30))],
    );
    let output = fixture.compute().unwrap();
    let frame = output.layout.frames[&fixture.nodes["implicit"].id()];

    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        (0.0, 0.0, 40.0, 30.0)
    );
}

#[test]
fn mismatched_area_dimensions_are_rejected_before_layout_output() {
    for areas in ["'a b c'", "'. . .'", "'a' 'b'"] {
        let fixture = Fixture::new(
            &format!(
                "display:grid;box-sizing:border-box;width:80px;height:40px;grid-template-columns:40px 40px;grid-template-rows:40px;grid-template-areas:{areas};margin:0;padding:0;border:0"
            ),
            &[("item", "width:20px;height:20px;margin:0;padding:0;border:0")],
        );
        let error = fixture.compute().unwrap_err();

        assert!(
            matches!(
                error,
                StyleLayoutError::UnsupportedComputedValue {
                    node,
                    property: "grid-template-areas",
                    ..
                } if node == fixture.root.id()
            ),
            "{areas}: {error}"
        );
    }
}
