use super::author_stylesheets::collect_runtime_author_stylesheets;
use super::*;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";

fn element(
    batch: &mut DocumentChangeBatch,
    node: spinon_core::HostNodeHandle,
    name: &str,
    attributes: &[(&str, &str)],
) {
    batch.push(DocumentOperation::CreateElement {
        node,
        namespace: HTML.to_owned(),
        local_name: name.to_owned(),
    });
    for (name, value) in attributes {
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, *name).unwrap(),
            value: (*value).into(),
        });
    }
}

fn append(batch: &mut DocumentChangeBatch, parent: HostParent, node: spinon_core::HostNodeHandle) {
    batch.push(DocumentOperation::InsertBefore {
        parent,
        node,
        before: None,
    });
}

fn request(document: &HostDocument) -> WorkRequest {
    let snapshot = Arc::new(document.snapshot());
    WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport {
            width_css_px: 301.0,
            height_css_px: 100.0,
            device_scale_factor: 1.0,
            environment_revision: EnvironmentRevision::INITIAL,
            media_environment: CssMediaEnvironment::MOBILE,
        },
        previous_snapshot: None,
        force_full: true,
        invalidation: None,
        previous_styles: None,
    }
}

#[test]
fn author_style_in_one_host_root_is_available_to_every_root_cascade() {
    let mut document = HostDocument::new().unwrap();
    let first_root = document.reserve_node_handle().unwrap();
    let sheet = document.reserve_node_handle().unwrap();
    let sheet_text = document.reserve_node_handle().unwrap();
    let second_root = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(9051).unwrap(), document.document_revision());
    element(&mut batch, first_root, "main", &[]);
    element(&mut batch, sheet, "style", &[]);
    element(&mut batch, second_root, "div", &[("id", "shared-target")]);
    batch.push(DocumentOperation::CreateText {
        node: sheet_text,
        data: "#shared-target { width: 71px; }".into(),
    });
    append(&mut batch, HostParent::Root, first_root);
    append(&mut batch, HostParent::Node(first_root), sheet);
    append(&mut batch, HostParent::Node(sheet), sheet_text);
    append(&mut batch, HostParent::Root, second_root);
    document.commit(batch).unwrap();

    let calculation = compute_request(&request(&document)).unwrap();
    assert_eq!(calculation.roots.len(), 2);
    assert_eq!(
        calculation.roots[1]
            .styles
            .elements
            .iter()
            .find(|style| style.node_id == second_root.id())
            .unwrap()
            .properties["width"],
        "71px"
    );
    assert_eq!(calculation.layout.unwrap_err().code, "multiple_host_roots");
}

#[test]
fn stylesheet_text_uses_connected_descendant_order_without_added_separators() {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let sheet = document.reserve_node_handle().unwrap();
    let opening = document.reserve_node_handle().unwrap();
    let declaration = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(9054).unwrap(), document.document_revision());
    element(&mut batch, root, "div", &[]);
    element(&mut batch, sheet, "style", &[]);
    batch.push(DocumentOperation::CreateText {
        node: opening,
        data: "div{".into(),
    });
    batch.push(DocumentOperation::CreateText {
        node: declaration,
        data: "width:71px;}".into(),
    });
    append(&mut batch, HostParent::Root, root);
    append(&mut batch, HostParent::Node(root), sheet);
    append(&mut batch, HostParent::Node(sheet), opening);
    append(&mut batch, HostParent::Node(sheet), declaration);
    document.commit(batch).unwrap();

    let sources = collect_runtime_author_stylesheets(&document.snapshot()).unwrap();
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0].css, "div{width:71px;}");
}

#[test]
fn import_diagnostic_fails_layout_and_gpu_profiles_without_fetching() {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let sheet = document.reserve_node_handle().unwrap();
    let text = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(9052).unwrap(), document.document_revision());
    element(&mut batch, root, "div", &[]);
    element(&mut batch, sheet, "style", &[]);
    batch.push(DocumentOperation::CreateText {
        node: text,
        data: "@import url(\"https://spinon.invalid/remote.css\");".into(),
    });
    append(&mut batch, HostParent::Root, root);
    append(&mut batch, HostParent::Node(root), sheet);
    append(&mut batch, HostParent::Node(sheet), text);
    document.commit(batch).unwrap();

    let request = request(&document);
    let layout_error = compute_request(&request).unwrap_err();
    let gpu_error = compute_request_for_runtime_gpu(&request).unwrap_err();
    assert!(layout_error.contains("host-style:"));
    assert!(layout_error.contains("파싱 진단"));
    assert_eq!(layout_error, gpu_error);
}

#[test]
fn author_rule_revealing_style_text_remains_a_layout_error() {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let sheet = document.reserve_node_handle().unwrap();
    let text = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(9053).unwrap(), document.document_revision());
    element(&mut batch, root, "div", &[]);
    element(&mut batch, sheet, "style", &[("id", "visible-sheet")]);
    batch.push(DocumentOperation::CreateText {
        node: text,
        data: "#visible-sheet { display: flex; }".into(),
    });
    append(&mut batch, HostParent::Root, root);
    append(&mut batch, HostParent::Node(root), sheet);
    append(&mut batch, HostParent::Node(sheet), text);
    document.commit(batch).unwrap();

    for calculation in [
        compute_request(&request(&document)).unwrap(),
        compute_request_for_runtime_gpu(&request(&document)).unwrap(),
    ] {
        let failure = calculation.layout.unwrap_err();
        assert_eq!(failure.code, "unsupported_text_node", "{failure:?}");
    }
}
