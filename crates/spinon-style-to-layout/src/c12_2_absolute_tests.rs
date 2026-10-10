use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId,
    StyleRevision,
};
use spinon_style::{
    ComputedCssPosition, CssViewport, StyloDocumentView,
    compute_runtime_block_positioning_cascade_with_stylesheets,
};

use crate::compute_runtime_style_layout;

const HTML: &str = "http://www.w3.org/1999/xhtml";

#[test]
fn block_absolute_layout_uses_nearest_positioned_owner_and_preserves_flow_siblings() {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let owner = document.reserve_node_handle().unwrap();
    let wrapper = document.reserve_node_handle().unwrap();
    let before = document.reserve_node_handle().unwrap();
    let target = document.reserve_node_handle().unwrap();
    let after = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(12_201).unwrap(), document.document_revision());
    for (name, node, parent, style) in [
        (
            "root",
            root,
            HostParent::Root,
            "display:block;position:static;box-sizing:border-box;width:100vw;height:100vh;margin:0;padding:0",
        ),
        (
            "owner",
            owner,
            HostParent::Node(root),
            "display:block;position:relative;box-sizing:border-box;width:80px;height:60px;padding:10px;border:4px solid;margin:0",
        ),
        (
            "wrapper",
            wrapper,
            HostParent::Node(owner),
            "display:block;position:static;box-sizing:border-box;width:50px;height:40px;margin:0;padding:0",
        ),
        (
            "before",
            before,
            HostParent::Node(wrapper),
            "display:block;width:12px;height:10px;margin:0;padding:0",
        ),
        (
            "target",
            target,
            HostParent::Node(wrapper),
            "display:block;position:absolute;left:5px;top:7px;width:20px;height:10px;margin:0;padding:0",
        ),
        (
            "after",
            after,
            HostParent::Node(wrapper),
            "display:block;width:30px;height:12px;margin:0;padding:0",
        ),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "id").unwrap(),
            value: name.to_owned().into(),
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

    let snapshot = Arc::new(document.snapshot());
    let view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::clone(&snapshot), root).unwrap();
    let viewport = CssViewport::C04_FIXTURE;
    let computed = compute_runtime_block_positioning_cascade_with_stylesheets(
        &view,
        &[],
        viewport,
        StyleRevision::INITIAL,
    )
    .unwrap();
    let target_style = computed
        .elements
        .iter()
        .find(|style| style.node_id == target.id())
        .unwrap();
    assert_eq!(target_style.layout_position, ComputedCssPosition::Absolute);

    let output = compute_runtime_style_layout(&snapshot, root, computed, viewport).unwrap();
    assert_eq!(
        output.layout.positioned_owners[&target.id()],
        spinon_layout::PositionedContainingBlockOwner::Node(owner.id())
    );
    assert_eq!(
        (
            output.layout.frames[&target.id()].x,
            output.layout.frames[&target.id()].y
        ),
        (9.0, 11.0)
    );
    assert_eq!(output.layout.frames[&after.id()].y, 24.0);
    assert_eq!(output.layout.flow_frames[&after.id()].y, 34.0);
    assert_eq!(output.layout.flow_frames[&target.id()].y, 24.0);
}
