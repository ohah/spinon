use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_style::{
    ComputedStyleProfile, CssOrigin, CssViewport, StylesheetSource, StyloDocumentView,
    compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets,
    compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets,
    compute_runtime_incremental_cascade_with_stylesheets,
};

use crate::{StyleLayoutOutput, compute_runtime_style_layout};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const OWNER: u64 = 10_133;

fn document(root_style: &str, child_style: &str) -> (HostDocument, HostNodeHandle, HostNodeHandle) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let child = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(OWNER).unwrap(), document.document_revision());
    for (node, id, style, parent) in [
        (root, "c1033-cascade-root", root_style, HostParent::Root),
        (
            child,
            "c1033-cascade-item",
            child_style,
            HostParent::Node(root),
        ),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        for (name, value) in [("id", id), ("style", style)] {
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, name).unwrap(),
                value: value.to_owned().into(),
            });
        }
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    (document, root, child)
}

fn view(document: &HostDocument, root: HostNodeHandle) -> StyloDocumentView {
    let snapshot = Arc::new(document.snapshot());
    StyloDocumentView::new_html_fragment_child_shared(snapshot, root).unwrap()
}

fn stylesheet(id: &str, css: &str) -> StylesheetSource {
    StylesheetSource {
        id: id.to_owned(),
        base_url: format!("https://spinon.invalid/c10/{id}.css"),
        origin: CssOrigin::Author,
        css: css.to_owned(),
    }
}

fn compute_layout(
    document: &HostDocument,
    root: HostNodeHandle,
    child: HostNodeHandle,
    computed: spinon_style::ComputedStyleSnapshot,
) -> (StyleLayoutOutput, String, String) {
    let snapshot = document.snapshot();
    let output =
        compute_runtime_style_layout(&snapshot, root, computed, CssViewport::C04_FIXTURE).unwrap();
    let property = |node: HostNodeHandle| {
        output
            .computed_styles
            .elements
            .iter()
            .find(|element| element.node_id == node.id())
            .unwrap()
            .properties["align-items"]
            .clone()
    };
    let root_alignment = property(root);
    let child_alignment = property(child);
    (output, root_alignment, child_alignment)
}

#[test]
fn stylesheet_custom_property_and_inline_cascade_drive_alignment_used_values() {
    let (document, root, child) = document(
        "display:flex;width:100px;height:100px;flex-direction:row",
        "display:block;width:20px;height:20px;align-self:auto",
    );
    let author = stylesheet(
        "c1033-alignment-cascade",
        "#c1033-cascade-root { align-items:center; --unused: 1 } #c1033-cascade-item { align-self:flex-end }",
    );
    let computed = compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets(
        &view(&document, root),
        std::slice::from_ref(&author),
        CssViewport::C04_FIXTURE,
        StyleRevision::INITIAL,
    )
    .unwrap();
    let (output, root_alignment, _) = compute_layout(&document, root, child, computed);
    let child_style = output
        .computed_styles
        .elements
        .iter()
        .find(|element| element.node_id == child.id())
        .unwrap();
    assert_eq!(root_alignment, "center");
    assert_eq!(child_style.properties["align-self"], "auto");
    assert_eq!(output.layout.frames[&child.id()].y, 40.0);
}

#[test]
fn stylesheet_custom_property_alignment_value_reaches_taffy() {
    let (document, root, child) = document(
        "display:flex;width:100px;height:100px;flex-direction:row",
        "display:block;width:20px;height:20px",
    );
    let author = stylesheet(
        "c1033-alignment-var",
        "#c1033-cascade-root { --cross-alignment:safe flex-end; align-items:var(--cross-alignment); } #c1033-cascade-item { --self-alignment:safe center; align-self:var(--self-alignment); }",
    );
    let computed = compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets(
        &view(&document, root),
        &[author],
        CssViewport::C04_FIXTURE,
        StyleRevision::INITIAL,
    )
    .unwrap();
    let output = compute_runtime_style_layout(
        &document.snapshot(),
        root,
        computed,
        CssViewport::C04_FIXTURE,
    )
    .unwrap();
    let child_style = output
        .computed_styles
        .elements
        .iter()
        .find(|element| element.node_id == child.id())
        .unwrap();
    assert_eq!(child_style.properties["align-self"], "safe center");
    assert_eq!(output.layout.frames[&child.id()].y, 40.0);
}

#[test]
fn registered_custom_property_keyword_is_projected_from_author_stylesheet() {
    let (document, root, child) = document(
        "display:flex;width:100px;height:100px;flex-direction:row",
        "display:block;width:20px;height:20px;align-self:auto",
    );
    let author = stylesheet(
        "c1033-registered-alignment",
        "@property --cross-alignment { syntax: \"<custom-ident>\"; inherits: true; initial-value: center; } #c1033-cascade-root { --cross-alignment:flex-end; align-items:var(--cross-alignment); }",
    );
    let computed = compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets(
        &view(&document, root),
        &[author],
        CssViewport::C04_FIXTURE,
        StyleRevision::INITIAL,
    )
    .unwrap();
    let (output, root_alignment, _) = compute_layout(&document, root, child, computed);
    assert_eq!(root_alignment, "flex-end");
    assert_eq!(output.layout.frames[&child.id()].y, 80.0);
}

#[test]
fn parent_alignment_change_recomputes_auto_child_layout_incrementally() {
    let (mut document, root, child) = document(
        "display:flex;width:100px;height:100px;flex-direction:row;align-items:center",
        "display:block;width:20px;height:20px;align-self:auto",
    );
    let viewport = CssViewport::C04_FIXTURE;
    let previous_view = view(&document, root);
    let previous = compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets(
        &previous_view,
        &[],
        viewport,
        StyleRevision::INITIAL,
    )
    .unwrap();

    let mut change =
        DocumentChangeBatch::new(OwnerId::new(OWNER).unwrap(), document.document_revision());
    change.push(DocumentOperation::SetAttribute {
        node: root,
        name: AttributeName::new(None, "style").unwrap(),
        value: "display:flex;width:100px;height:100px;flex-direction:row;align-items:flex-end"
            .to_owned()
            .into(),
    });
    document.commit(change).unwrap();

    let current_view = view(&document, root);
    let full = compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets(
        &current_view,
        &[],
        viewport,
        StyleRevision::INITIAL,
    )
    .unwrap();
    let (incremental, _) = compute_runtime_incremental_cascade_with_stylesheets(
        &current_view,
        &[],
        viewport,
        StyleRevision::INITIAL,
        ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1,
        &previous,
        &[root.id()],
    )
    .unwrap()
    .expect("부모 정렬 변경은 검증된 incremental cascade 경로를 사용해야 합니다");
    assert_eq!(incremental.elements, full.elements);
    assert_eq!(incremental.diagnostics, full.diagnostics);

    let output =
        compute_runtime_style_layout(&document.snapshot(), root, incremental, viewport).unwrap();
    let child_style = output
        .computed_styles
        .elements
        .iter()
        .find(|element| element.node_id == child.id())
        .unwrap();
    assert_eq!(child_style.properties["align-self"], "auto");
    assert_eq!(output.layout.frames[&child.id()].y, 80.0);
}
