use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId,
    StyleRevision,
};

use super::{
    ComputedCssGridAutoFlow, ComputedCssGridAutoRows, ComputedCssGridPlacement,
    ComputedCssGridTrackSize, ComputedStyleProfile, CssMediaEnvironment, CssViewport,
    StyloDocumentView, compute_runtime_block_flex_grid_auto_placement_cascade_with_stylesheets,
    compute_runtime_block_flex_grid_cascade_with_stylesheets,
    compute_runtime_block_flex_grid_placement_cascade_with_stylesheets,
    compute_runtime_incremental_cascade_with_stylesheets,
    first_unsupported_runtime_block_flex_grid_auto_placement_inline_property,
    first_unsupported_runtime_block_flex_grid_inline_property,
    first_unsupported_runtime_block_flex_grid_placement_inline_property,
};
use crate::{CssOrigin, StylesheetSource};

const HTML: &str = "http://www.w3.org/1999/xhtml";

struct Fixture {
    view: StyloDocumentView,
    root: spinon_core::NodeId,
}

fn fixture(style: &str) -> Fixture {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let child = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(6059).unwrap(), document.document_revision());
    for (node, name, parent, node_style) in [
        (root, "root", HostParent::Root, style),
        (
            child,
            "child",
            HostParent::Node(root),
            "display:block;box-sizing:border-box;width:20px;height:10px;margin:0;padding:0",
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
            value: node_style.to_owned().into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    let view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::new(document.snapshot()), root)
            .unwrap();
    Fixture {
        view,
        root: root.id(),
    }
}

fn viewport() -> CssViewport {
    CssViewport {
        width_css_px: 300.0,
        height_css_px: 200.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    }
}

fn placement_fixture(
    children: &[&str],
) -> (
    StyloDocumentView,
    spinon_core::NodeId,
    Vec<spinon_core::NodeId>,
) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let child_handles = (0..children.len())
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(11_221).unwrap(), document.document_revision());
    let mut nodes = vec![(
        root,
        HostParent::Root,
        "root".to_owned(),
        "display:grid;box-sizing:border-box;width:250px;height:85px;grid-template-columns:80px 80px 80px;grid-template-rows:40px 40px;column-gap:5px;row-gap:5px;margin:0;padding:0".to_owned(),
    )];
    nodes.extend(
        child_handles
            .iter()
            .zip(children)
            .enumerate()
            .map(|(index, (handle, style))| {
                (
                    *handle,
                    HostParent::Node(root),
                    format!("item-{index}"),
                    (*style).to_owned(),
                )
            }),
    );
    for (node, parent, name, node_style) in nodes {
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
            value: node_style.to_owned().into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    let root_id = root.id();
    let child_ids = child_handles
        .into_iter()
        .map(|handle| handle.id())
        .collect();
    let view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::new(document.snapshot()), root)
            .unwrap();
    (view, root_id, child_ids)
}

#[path = "runtime_grid_tests/auto_placement.rs"]
mod auto_placement;
#[path = "runtime_grid_tests/hidden_context.rs"]
mod hidden_context;
#[path = "runtime_grid_tests/placement.rs"]
mod placement;
#[path = "runtime_grid_tests/tracks.rs"]
mod tracks;
