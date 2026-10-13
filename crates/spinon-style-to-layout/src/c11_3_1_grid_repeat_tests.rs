use std::{collections::BTreeMap, sync::Arc};

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_style::{CssMediaEnvironment, CssViewport, StyloDocumentView};

use crate::{StyleLayoutOutput, compute_runtime_block_flex_grid_template_style_layout};

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
        let mut nodes = BTreeMap::new();
        for (id, _) in item_styles {
            nodes.insert((*id).to_owned(), document.reserve_node_handle().unwrap());
        }
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_231).unwrap(), document.document_revision());
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

    fn compute(&self) -> Result<StyleLayoutOutput, crate::StyleLayoutError> {
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
                width_css_px: 320.0,
                height_css_px: 240.0,
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

#[test]
fn repeated_tracks_reach_taffy_and_match_pinned_chrome_frames() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:320px;height:240px;grid-template-columns:20px 80px repeat(2,40px 1fr) 20px;grid-template-rows:repeat(2,50px);column-gap:8px;row-gap:6px;margin:0;padding:0;border:0",
        &[
            (
                "item-a",
                "width:30px;height:20px;margin:0;padding:0;border:0",
            ),
            (
                "item-b",
                "width:24px;height:22px;margin:0;padding:0;border:0",
            ),
        ],
    );

    let output = fixture.compute().unwrap();
    let grid_style = output
        .computed_styles
        .elements
        .iter()
        .find(|style| style.node_id == fixture.root.id())
        .unwrap()
        .grid_layout
        .as_ref()
        .unwrap();
    assert_eq!(grid_style.columns.len(), 7);
    assert_eq!(grid_style.rows.len(), 2);
    assert_eq!(grid_style.columns_line_names.len(), 8);
    assert_eq!(grid_style.rows_line_names.len(), 3);

    let item_a = output.layout.frames[&fixture.nodes["item-a"].id()];
    let item_b = output.layout.frames[&fixture.nodes["item-b"].id()];
    assert_eq!(
        (item_a.x, item_a.y, item_a.width, item_a.height),
        (0.0, 0.0, 30.0, 20.0)
    );
    assert_eq!(
        (item_b.x, item_b.y, item_b.width, item_b.height),
        (28.0, 0.0, 24.0, 22.0)
    );
}

#[test]
fn unsupported_repeat_templates_fail_before_any_layout_output() {
    for tracks in [
        "repeat(auto-fill,40px)",
        "repeat(2,minmax(20px,1fr))",
        "repeat(10001,1px)",
    ] {
        let fixture = Fixture::new(
            &format!(
                "display:grid;box-sizing:border-box;width:320px;height:240px;grid-template-columns:{tracks};grid-template-rows:40px;margin:0;padding:0"
            ),
            &[("item", "width:20px;height:20px;margin:0;padding:0")],
        );
        assert!(fixture.compute().is_err(), "{tracks}");
    }
}
