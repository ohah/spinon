use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId,
};
use spinon_style::{
    ComputedCssOverscrollBehavior, ComputedCssTouchAction, ComputedStyleProfile,
    CssMediaEnvironment, CssViewport, StyloDocumentView,
    compute_runtime_block_flex_scroll_gesture_cascade_with_stylesheets,
};

use crate::compute_runtime_style_layout;

const HTML: &str = "http://www.w3.org/1999/xhtml";

#[test]
fn c133_scroll_input_style_profile_keeps_existing_layout_projection() {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let child = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(13_301).unwrap(), document.document_revision());
    for (handle, id, parent, style) in [
        (
            root,
            "root",
            HostParent::Root,
            "display:block;width:120px;height:80px;overflow:auto;overscroll-behavior:contain none;touch-action:none",
        ),
        (
            child,
            "child",
            HostParent::Node(root),
            "display:block;width:200px;height:100px",
        ),
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

    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), root)
        .unwrap();
    let viewport = CssViewport {
        width_css_px: 300.0,
        height_css_px: 200.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    };
    let styles = compute_runtime_block_flex_scroll_gesture_cascade_with_stylesheets(
        &view,
        &[],
        viewport,
        Default::default(),
    )
    .unwrap();
    assert_eq!(
        styles.profile,
        ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
    );
    let root_style = styles
        .elements
        .iter()
        .find(|style| style.node_id == root.id())
        .unwrap();
    assert_eq!(
        root_style.input_overscroll_behavior_x,
        Some(ComputedCssOverscrollBehavior::Contain)
    );
    assert_eq!(
        root_style.input_overscroll_behavior_y,
        Some(ComputedCssOverscrollBehavior::None)
    );
    assert_eq!(
        root_style.input_touch_action,
        Some(ComputedCssTouchAction::None)
    );

    let output = compute_runtime_style_layout(&snapshot, root, styles, viewport).unwrap();
    assert_eq!(
        output.computed_styles.profile,
        ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
    );
}
