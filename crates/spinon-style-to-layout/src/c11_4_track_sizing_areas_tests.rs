use std::{collections::BTreeMap, sync::Arc};

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_style::{ComputedStyleProfile, CssMediaEnvironment, CssViewport, StyloDocumentView};

use crate::{StyleLayoutOutput, compute_runtime_block_flex_grid_track_sizing_style_layout};

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
            DocumentChangeBatch::new(OwnerId::new(11_234).unwrap(), document.document_revision());
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

    fn compute(&self) -> StyleLayoutOutput {
        let snapshot = self.document.snapshot();
        let view =
            StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), self.root)
                .unwrap();
        compute_runtime_block_flex_grid_track_sizing_style_layout(
            &snapshot,
            &view,
            self.root,
            &[],
            CssViewport {
                width_css_px: 240.0,
                height_css_px: 80.0,
                device_scale_factor: 1.0,
                environment_revision: Default::default(),
                media_environment: CssMediaEnvironment::MOBILE,
            },
            StyleRevision::INITIAL,
        )
        .unwrap()
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
fn track_sizing_profile_keeps_named_area_placement() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:240px;height:80px;grid-template-columns:80px minmax(20px,1fr) 80px;grid-template-rows:30px 50px;grid-template-areas:'head head .' 'nav main main';column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[
            (
                "head",
                "box-sizing:border-box;width:160px;height:30px;grid-area:head;margin:0;padding:0;border:0",
            ),
            (
                "nav",
                "box-sizing:border-box;width:80px;height:50px;grid-area:nav;margin:0;padding:0;border:0",
            ),
            (
                "main",
                "box-sizing:border-box;width:160px;height:50px;grid-area:main;margin:0;padding:0;border:0",
            ),
        ],
    );
    let output = fixture.compute();

    assert_eq!(
        output.computed_styles.profile,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
    );
    for (id, expected) in [
        ("head", (0.0, 0.0, 160.0, 30.0)),
        ("nav", (0.0, 30.0, 80.0, 50.0)),
        ("main", (80.0, 30.0, 160.0, 50.0)),
    ] {
        let frame = output.layout.frames[&fixture.nodes[id].id()];
        assert_eq!(
            (frame.x, frame.y, frame.width, frame.height),
            expected,
            "area {id}"
        );
    }
}

#[test]
fn track_sizing_profile_lays_out_css_auto_repeat_modes() {
    for (keyword, expected_width) in [("auto-fill", 21.25), ("auto-fit", 100.0)] {
        let fixture = Fixture::new(
            &format!(
                "display:grid;box-sizing:border-box;width:100px;height:30px;grid-template-columns:repeat({keyword},minmax(20px,1fr));grid-template-rows:30px;column-gap:5px;row-gap:0;margin:0;padding:0;border:0"
            ),
            &[(
                "item",
                "box-sizing:border-box;width:100%;height:20px;margin:0;padding:0;border:0",
            )],
        );
        let output = fixture.compute();
        let frame = output.layout.frames[&fixture.nodes["item"].id()];

        assert_eq!(frame.width, expected_width, "{keyword}");
        assert_eq!(frame.height, 20.0, "{keyword}");
    }
}
