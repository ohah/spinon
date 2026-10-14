use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument,
    HostNodeHandle, HostParent, IntrinsicSizeRevision, OwnerId, StyleRevision,
};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_layout::{LayoutIntrinsicSize, LayoutSourceRevision};
use spinon_style::{
    CssMediaEnvironment, CssOrigin, CssViewport, StylesheetSource, StyloDocumentView,
    compute_runtime_flex_custom_properties_cascade_with_stylesheets,
};

use crate::compute_runtime_style_layout_with_intrinsic_size_snapshot;

const HTML: &str = "http://www.w3.org/1999/xhtml";
const PNG: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/resources/c14-1-png-basic.png"
));
const STYLESHEET: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c14/replaced-auto-size.html"
);
const REFERENCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/resources/c14-1-png-basic-reference.json"
);

fn document() -> (HostDocument, HostNodeHandle, HostNodeHandle) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let image = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(1415).unwrap(), document.document_revision());
    batch
        .push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        })
        .push(DocumentOperation::SetAttribute {
            node: root,
            name: AttributeName::new(None, "id").unwrap(),
            value: "root".to_owned().into(),
        })
        .push(DocumentOperation::CreateElement {
            node: image,
            namespace: HTML.to_owned(),
            local_name: "img".to_owned(),
        })
        .push(DocumentOperation::SetAttribute {
            node: image,
            name: AttributeName::new(None, "id").unwrap(),
            value: "image".to_owned().into(),
        })
        .push(DocumentOperation::SetAttribute {
            node: image,
            name: AttributeName::new(None, "src").unwrap(),
            value: "https://spinon.invalid/c14-1-png-basic.png"
                .to_owned()
                .into(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: image,
            before: None,
        });
    document.commit(batch).unwrap();
    (document, root, image)
}

fn stylesheet() -> StylesheetSource {
    let html = std::fs::read_to_string(STYLESHEET).unwrap();
    let (_, rest) = html.split_once("<style>").unwrap();
    let (css, _) = rest.split_once("</style>").unwrap();
    StylesheetSource {
        id: "c14-1-png-resource".to_owned(),
        base_url: "https://spinon.invalid/c14-1-png-resource.css".to_owned(),
        origin: CssOrigin::Author,
        css: css.to_owned(),
    }
}

fn viewport() -> CssViewport {
    CssViewport {
        width_css_px: 120.0,
        height_css_px: 80.0,
        device_scale_factor: 1.0,
        environment_revision: EnvironmentRevision::default(),
        media_environment: CssMediaEnvironment::DESKTOP,
    }
}

fn resource_snapshot(
    document: &spinon_core::HostDocumentSnapshot,
    image: HostNodeHandle,
) -> ImageResourceSnapshot {
    let request = ImageRequest::from_document(document, image).unwrap();
    ImageResourceSnapshot::new(
        document,
        IntrinsicSizeRevision::INITIAL.checked_next().unwrap(),
        [request.decode(PNG).unwrap()],
    )
    .unwrap()
}

#[test]
fn decoded_png_flows_through_resource_and_c14_layout_snapshots() {
    let reference: serde_json::Value =
        serde_json::from_slice(&std::fs::read(REFERENCE).unwrap()).unwrap();
    assert_eq!(reference["fixture"]["id"], "U05.1-png-natural-size-v1");
    assert_eq!(reference["oracle"]["product"], "HeadlessChrome/155");

    let (document, root, image) = document();
    let snapshot = document.snapshot();
    let resources = resource_snapshot(&snapshot, image);
    let intrinsic = resources.to_layout_intrinsic_size_snapshot().unwrap();
    let expected_width = reference["observation"]["natural_width"].as_f64().unwrap() as f32;
    let expected_height = reference["observation"]["natural_height"].as_f64().unwrap() as f32;
    assert_eq!(
        intrinsic.sizes()[&image.id()],
        LayoutIntrinsicSize::new(expected_width, expected_height)
    );

    let view =
        StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), root).unwrap();
    let computed = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        &view,
        &[stylesheet()],
        viewport(),
        StyleRevision::INITIAL,
    )
    .unwrap();
    let output = compute_runtime_style_layout_with_intrinsic_size_snapshot(
        &snapshot,
        root,
        computed,
        viewport(),
        intrinsic,
    )
    .unwrap();
    let frame = output.layout.frames[&image.id()];
    let expected = &reference["observation"]["rect"];
    for (name, actual, key) in [
        ("x", frame.x, "x"),
        ("y", frame.y, "y"),
        ("width", frame.width, "width"),
        ("height", frame.height, "height"),
    ] {
        let expected = expected[key].as_f64().unwrap() as f32;
        assert!(
            (actual - expected).abs() <= 0.5,
            "image.{name}: actual {actual} != Chrome {expected}"
        );
    }
}

#[test]
fn image_request_completion_is_rejected_after_src_changes() {
    let (mut document, _root, image) = document();
    let old_snapshot = document.snapshot();
    let old_request = ImageRequest::from_document(&old_snapshot, image).unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(1415).unwrap(), document.document_revision());
    batch.push(DocumentOperation::SetAttribute {
        node: image,
        name: AttributeName::new(None, "src").unwrap(),
        value: "https://spinon.invalid/replaced.png".to_owned().into(),
    });
    document.commit(batch).unwrap();

    let result = ImageResourceSnapshot::new(
        &document.snapshot(),
        IntrinsicSizeRevision::INITIAL.checked_next().unwrap(),
        [old_request.decode(PNG).unwrap()],
    );
    assert!(matches!(
        result,
        Err(spinon_image::ImageSnapshotError::SourceChanged(node)) if node == image.id()
    ));
}

#[test]
fn resource_snapshot_rejects_duplicate_nodes_and_document_revisions() {
    let (mut document, _root, image) = document();
    let old_snapshot = document.snapshot();
    let request = ImageRequest::from_document(&old_snapshot, image).unwrap();
    let duplicate = ImageResourceSnapshot::new(
        &old_snapshot,
        IntrinsicSizeRevision::INITIAL,
        [
            request.clone().decode(PNG).unwrap(),
            request.clone().decode(PNG).unwrap(),
        ],
    );
    assert!(matches!(
        duplicate,
        Err(spinon_image::ImageSnapshotError::DuplicateNode(node)) if node == request.element().id()
    ));

    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(1415).unwrap(), document.document_revision());
    batch.push(DocumentOperation::SetAttribute {
        node: request.element(),
        name: AttributeName::new(None, "alt").unwrap(),
        value: "changed".to_owned().into(),
    });
    document.commit(batch).unwrap();
    let stale = ImageResourceSnapshot::new(
        &document.snapshot(),
        IntrinsicSizeRevision::INITIAL.checked_next().unwrap(),
        [request.decode(PNG).unwrap()],
    );
    assert!(matches!(
        stale,
        Err(spinon_image::ImageSnapshotError::StaleRequest(_))
    ));
}

#[test]
fn resource_snapshot_rejects_a_different_document_generation() {
    let (first_document, _first_root, image) = document();
    let first_snapshot = first_document.snapshot();
    let request = ImageRequest::from_document(&first_snapshot, image).unwrap();
    let (second_document, _second_root, _second_image) = document();

    let stale = ImageResourceSnapshot::new(
        &second_document.snapshot(),
        IntrinsicSizeRevision::INITIAL.checked_next().unwrap(),
        [request.decode(PNG).unwrap()],
    );
    assert!(matches!(
        stale,
        Err(spinon_image::ImageSnapshotError::StaleRequest(_))
    ));
}

#[test]
fn resource_snapshot_preserves_host_document_source_and_measurement_revision() {
    let (document, _root, image) = document();
    let snapshot = document.snapshot();
    let resources = resource_snapshot(&snapshot, image);
    let intrinsic = resources.to_layout_intrinsic_size_snapshot().unwrap();
    assert_eq!(
        intrinsic.source(),
        LayoutSourceRevision::HostDocument {
            generation: snapshot.generation(),
            document: snapshot.document_revision(),
            render_tree: snapshot.render_tree_revision(),
        }
    );
    assert_eq!(
        intrinsic.revision(),
        IntrinsicSizeRevision::INITIAL.checked_next().unwrap()
    );
}
