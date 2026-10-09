use std::sync::Arc;

use spinon_core::{AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, OwnerId};

use super::{CssViewport, StyloDocumentView, first_unsupported_runtime_layout_inline_property};

const HTML: &str = "http://www.w3.org/1999/xhtml";

fn view_for_inline_style(style: &str) -> (StyloDocumentView, spinon_core::NodeId) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(31).unwrap(), document.document_revision());
    batch.push(DocumentOperation::CreateElement {
        node: root,
        namespace: HTML.to_owned(),
        local_name: "div".to_owned(),
    });
    batch.push(DocumentOperation::SetAttribute {
        node: root,
        name: AttributeName::new(None, "style").unwrap(),
        value: style.into(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: spinon_core::HostParent::Root,
        node: root,
        before: None,
    });
    document.commit(batch).unwrap();
    let view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::new(document.snapshot()), root)
            .unwrap();
    (view, root.id())
}

#[test]
fn runtime_inline_allowlist_accepts_supported_shorthand_expansions() {
    let (view, _) = view_for_inline_style(
        "display:flex;flex:0 1 auto;gap:2px;margin-inline:3px;padding-block:4px",
    );
    assert_eq!(
        first_unsupported_runtime_layout_inline_property(&view),
        None
    );
}

#[test]
fn runtime_inline_allowlist_reports_valid_unsupported_properties_without_values() {
    let (view, root) = view_for_inline_style("display:block;color:rgb(1,2,3)");
    assert_eq!(
        first_unsupported_runtime_layout_inline_property(&view),
        Some((root, "color".to_owned()))
    );
}

#[test]
fn runtime_inline_allowlist_rejects_unknown_custom_properties() {
    let (view, root) = view_for_inline_style("display:block;--theme-color:red");
    assert_eq!(
        first_unsupported_runtime_layout_inline_property(&view),
        Some((root, "--theme-color".to_owned()))
    );
}

#[test]
fn runtime_inline_allowlist_ignores_invalid_declarations_after_css_recovery() {
    let (view, _) = view_for_inline_style("display:block;color:nope");
    assert_eq!(
        first_unsupported_runtime_layout_inline_property(&view),
        None
    );
    let snapshot = super::compute_runtime_flex_layout_cascade(
        &view,
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    assert_eq!(snapshot.diagnostics.len(), 1);
}
