use std::{collections::BTreeMap, fs, sync::Arc};

use serde_json::Value;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument,
    HostNodeHandle, HostParent, IntrinsicSizeRevision, OwnerId, StyleRevision,
};
use spinon_layout::{LayoutIntrinsicSize, LayoutIntrinsicSizeSnapshot, LayoutSourceRevision};
use spinon_style::{
    CssMediaEnvironment, CssOrigin, CssViewport, StylesheetSource, StyloDocumentView,
    compute_runtime_flex_custom_properties_cascade_with_stylesheets,
};

use crate::{
    StyleLayoutError, compute_runtime_style_layout,
    compute_runtime_style_layout_with_intrinsic_size_snapshot,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c14/replaced-auto-size.html"
);
const REFERENCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/references/c14-replaced-auto-size-v1.json"
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
    let html = fs::read_to_string(FIXTURE).unwrap();
    let (_, rest) = html.split_once("<style>").unwrap();
    let (css, _) = rest.split_once("</style>").unwrap();
    StylesheetSource {
        id: "c14-replaced-auto-size".to_owned(),
        base_url: "https://spinon.invalid/c14/replaced-auto-size.css".to_owned(),
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

fn layout(
    document: &HostDocument,
    root: HostNodeHandle,
    image: HostNodeHandle,
) -> crate::StyleLayoutOutput {
    let snapshot = document.snapshot();
    let intrinsic = image_snapshot(&snapshot, image, LayoutIntrinsicSize::new(24.0, 16.0));
    layout_with_inputs(document, root, stylesheet(), Some(intrinsic)).unwrap()
}

fn image_snapshot(
    snapshot: &spinon_core::HostDocumentSnapshot,
    image: HostNodeHandle,
    size: LayoutIntrinsicSize,
) -> LayoutIntrinsicSizeSnapshot {
    LayoutIntrinsicSizeSnapshot::new(
        LayoutSourceRevision::HostDocument {
            generation: snapshot.generation(),
            document: snapshot.document_revision(),
            render_tree: snapshot.render_tree_revision(),
        },
        IntrinsicSizeRevision::INITIAL.checked_next().unwrap(),
        BTreeMap::from([(image.id(), size)]),
    )
    .unwrap()
}

fn layout_with_inputs(
    document: &HostDocument,
    root: HostNodeHandle,
    author_stylesheet: StylesheetSource,
    intrinsic_sizes: Option<LayoutIntrinsicSizeSnapshot>,
) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
    let snapshot = document.snapshot();
    let view =
        StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), root).unwrap();
    let computed = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        &view,
        &[author_stylesheet],
        viewport(),
        StyleRevision::INITIAL,
    )
    .unwrap();
    match intrinsic_sizes {
        Some(intrinsic_sizes) => compute_runtime_style_layout_with_intrinsic_size_snapshot(
            &snapshot,
            root,
            computed,
            viewport(),
            intrinsic_sizes,
        ),
        None => compute_runtime_style_layout(&snapshot, root, computed, viewport()),
    }
}

#[test]
fn host_document_stylo_and_taffy_match_agent_browser_replaced_auto_size() {
    let reference: Value = serde_json::from_slice(&fs::read(REFERENCE).unwrap()).unwrap();
    assert_eq!(reference["fixture"]["id"], "C14.1-replaced-auto-size-v1");
    assert_eq!(reference["oracle"]["product"], "Chrome/155.0.8059.40");
    let expected = reference["observations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|node| node["id"] == "image")
        .unwrap();

    let (document, root, image) = document();
    let output = layout(&document, root, image);
    let frame = output.layout.frames[&image.id()];
    for (field, actual, key) in [
        ("x", frame.x, "x"),
        ("y", frame.y, "y"),
        ("width", frame.width, "width"),
        ("height", frame.height, "height"),
    ] {
        let expected = expected["rect"][key].as_f64().unwrap() as f32;
        assert!(
            (actual - expected).abs() <= 0.5,
            "image.{field}: actual {actual} != Chrome {expected}"
        );
    }
    assert_eq!(
        output.layout.revision.intrinsic_size(),
        IntrinsicSizeRevision::INITIAL.checked_next().unwrap()
    );
}

#[test]
fn legacy_runtime_projection_rejects_visible_auto_image_without_decoder_measurement() {
    let (document, root, _) = document();
    let snapshot = document.snapshot();
    let view =
        StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), root).unwrap();
    let computed = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        &view,
        &[stylesheet()],
        viewport(),
        StyleRevision::INITIAL,
    )
    .unwrap();
    assert!(matches!(
        compute_runtime_style_layout(&snapshot, root, computed, viewport()),
        Err(StyleLayoutError::Layout(
            spinon_layout::LayoutError::MissingIntrinsicSize(_)
        ))
    ));
}

#[test]
fn stale_measurement_snapshot_is_rejected_before_sizing_preflight() {
    let (document, root, image) = document();
    let snapshot = document.snapshot();
    let valid = image_snapshot(&snapshot, image, LayoutIntrinsicSize::new(24.0, 16.0));
    let stale = LayoutIntrinsicSizeSnapshot::new(
        LayoutSourceRevision::Tree(Default::default()),
        valid.revision(),
        valid.sizes().clone(),
    )
    .unwrap();
    assert!(matches!(
        layout_with_inputs(&document, root, stylesheet(), Some(stale)),
        Err(StyleLayoutError::Layout(
            spinon_layout::LayoutError::IntrinsicSizeSourceMismatch { .. }
        ))
    ));
}

#[test]
fn cache_entries_for_non_images_or_nodes_outside_the_subtree_are_rejected() {
    let (document, root, image) = document();
    let snapshot = document.snapshot();
    let non_image = LayoutIntrinsicSizeSnapshot::new(
        image_snapshot(&snapshot, image, LayoutIntrinsicSize::new(24.0, 16.0)).source(),
        IntrinsicSizeRevision::INITIAL,
        BTreeMap::from([(root.id(), LayoutIntrinsicSize::new(24.0, 16.0))]),
    )
    .unwrap();
    assert!(matches!(
        layout_with_inputs(&document, root, stylesheet(), Some(non_image)),
        Err(StyleLayoutError::Layout(
            spinon_layout::LayoutError::UnexpectedIntrinsicSizeNode(id)
        )) if id == root.id()
    ));

    let outside = LayoutIntrinsicSizeSnapshot::new(
        image_snapshot(&snapshot, image, LayoutIntrinsicSize::new(24.0, 16.0)).source(),
        IntrinsicSizeRevision::INITIAL,
        BTreeMap::from([(
            spinon_core::NodeId::new(9999).unwrap(),
            LayoutIntrinsicSize::new(24.0, 16.0),
        )]),
    )
    .unwrap();
    assert!(matches!(
        layout_with_inputs(&document, root, stylesheet(), Some(outside)),
        Err(StyleLayoutError::Layout(
            spinon_layout::LayoutError::UnknownIntrinsicSizeNode(id)
        )) if id == spinon_core::NodeId::new(9999).unwrap()
    ));
}

#[test]
fn hidden_image_needs_no_measurement_and_fixed_size_ignores_cached_natural_size() {
    let (document, root, image) = document();
    let mut hidden_stylesheet = stylesheet();
    hidden_stylesheet
        .css
        .push_str("\n#image { display: none; width: auto; height: auto; }\n");
    let hidden = layout_with_inputs(&document, root, hidden_stylesheet, None).unwrap();
    assert_eq!(hidden.layout.frames[&image.id()].width, 0.0);
    assert_eq!(hidden.layout.frames[&image.id()].height, 0.0);

    let snapshot = document.snapshot();
    let mut fixed_stylesheet = stylesheet();
    fixed_stylesheet
        .css
        .push_str("\n#image { width: 48px; height: 32px; }\n");
    let fixed = layout_with_inputs(
        &document,
        root,
        fixed_stylesheet,
        Some(image_snapshot(
            &snapshot,
            image,
            LayoutIntrinsicSize::new(24.0, 16.0),
        )),
    )
    .unwrap();
    let frame = fixed.layout.frames[&image.id()];
    assert_eq!((frame.width, frame.height), (48.0, 32.0));
}

#[test]
fn partial_auto_and_aspect_ratio_image_sizing_are_rejected() {
    let (document, root, image) = document();
    let snapshot = document.snapshot();
    let intrinsic = image_snapshot(&snapshot, image, LayoutIntrinsicSize::new(24.0, 16.0));

    for override_rule in [
        "#image { width: 48px; height: auto; }",
        "#image { width: auto; height: 24px; }",
        "#image { width: auto; height: auto; aspect-ratio: 3 / 2; }",
    ] {
        let mut author_stylesheet = stylesheet();
        author_stylesheet.css.push('\n');
        author_stylesheet.css.push_str(override_rule);
        author_stylesheet.css.push('\n');
        assert!(
            matches!(
                layout_with_inputs(&document, root, author_stylesheet, Some(intrinsic.clone())),
                Err(StyleLayoutError::Layout(
                    spinon_layout::LayoutError::UnsupportedIntrinsicSizing { node, .. }
                )) if node == image.id()
            ),
            "unexpected acceptance of {override_rule}"
        );
    }
}
