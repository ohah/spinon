use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};
use spinon_style::{
    ComputedCssOverflow, CssMediaEnvironment, CssViewport, StyloDocumentView,
    compute_runtime_block_flex_overflow_clip_cascade_with_stylesheets,
    first_unsupported_runtime_block_flex_overflow_clip_inline_property,
};

use crate::compute_runtime_style_layout;

const HTML: &str = "http://www.w3.org/1999/xhtml";

struct Fixture {
    document: HostDocument,
    root: HostNodeHandle,
    child: HostNodeHandle,
}

fn fixture(root_style: &str, child_style: &str) -> Fixture {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let child = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(13_101).unwrap(), document.document_revision());
    for (handle, id, parent, style) in [
        (root, "root", HostParent::Root, root_style),
        (child, "child", HostParent::Node(root), child_style),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node: handle,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: handle,
            name: AttributeName::new(None, "id").unwrap(),
            value: id.to_owned().into(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: handle,
            name: AttributeName::new(None, "style").unwrap(),
            value: style.to_owned().into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node: handle,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    Fixture {
        document,
        root,
        child,
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

fn layout(fixture: &Fixture) -> crate::StyleLayoutOutput {
    let snapshot = fixture.document.snapshot();
    let view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), fixture.root)
            .unwrap();
    let styles = compute_runtime_block_flex_overflow_clip_cascade_with_stylesheets(
        &view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();
    compute_runtime_style_layout(&snapshot, fixture.root, styles, viewport()).unwrap()
}

#[test]
fn clip_axis_values_project_to_taffy_and_used_box_edges_survive_layout() {
    let fixture = fixture(
        "display:block;position:relative;box-sizing:border-box;width:240px;height:120px;border:4px solid;padding:10px;overflow-x:clip;overflow-y:visible;overflow-clip-margin:content-box 8px",
        "position:absolute;left:80px;top:40px;width:60px;height:50px;overflow:visible",
    );
    let output = layout(&fixture);
    let root = output
        .computed_styles
        .elements
        .iter()
        .find(|style| style.node_id == fixture.root.id())
        .unwrap();
    assert_eq!(root.layout_overflow_x, Some(ComputedCssOverflow::Clip));
    assert_eq!(root.layout_overflow_y, Some(ComputedCssOverflow::Visible));
    assert_eq!(
        output
            .layout
            .used_box_edges
            .get(&fixture.root.id())
            .unwrap()
            .border
            .left,
        4.0
    );
    assert_eq!(
        output
            .layout
            .used_box_edges
            .get(&fixture.root.id())
            .unwrap()
            .padding
            .left,
        10.0
    );
    let root_layout_style = output
        .computed_styles
        .elements
        .iter()
        .find(|style| style.node_id == fixture.root.id())
        .unwrap();
    assert_eq!(
        root_layout_style.layout_overflow_x,
        Some(ComputedCssOverflow::Clip)
    );
    assert_eq!(
        output.layout.frames.get(&fixture.child.id()).unwrap().width,
        60.0
    );
}

#[test]
fn mixed_scrollable_computed_axis_values_use_scrollable_layout_semantics() {
    let fixture = fixture(
        "display:block;position:relative;width:240px;height:120px;overflow-x:visible;overflow-y:hidden",
        "position:absolute;left:10px;top:10px;width:40px;height:30px;overflow-x:clip;overflow-y:auto",
    );
    let output = layout(&fixture);
    assert_eq!(
        output
            .computed_styles
            .elements
            .iter()
            .find(|style| style.node_id == fixture.root.id())
            .unwrap()
            .layout_overflow_x,
        Some(ComputedCssOverflow::Auto)
    );
    assert_eq!(
        output.layout.positioned_owners[&fixture.child.id()],
        spinon_layout::PositionedContainingBlockOwner::Node(fixture.root.id())
    );
}

#[test]
fn unsupported_float_is_rejected_in_c131_profile() {
    let fixture = fixture(
        "display:block;position:relative;width:240px;height:120px;overflow:clip",
        "float:left;width:30px;height:20px",
    );
    let snapshot = fixture.document.snapshot();
    let view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), fixture.root)
            .unwrap();
    assert_eq!(
        first_unsupported_runtime_block_flex_overflow_clip_inline_property(&view),
        Some((fixture.child.id(), "float".to_owned()))
    );
}
