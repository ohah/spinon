use std::{collections::BTreeMap, fs, sync::Arc};

use serde_json::Value;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument,
    HostNodeHandle, HostParent, IntrinsicSizeRevision, OwnerId, StyleRevision,
};
use spinon_layout::{LayoutIntrinsicSize, LayoutIntrinsicSizeSnapshot, LayoutSourceRevision};
use spinon_style::{
    CssMediaEnvironment, CssOrigin, CssViewport, StylesheetSource, StyloDocumentView,
    compute_runtime_block_flex_grid_template_cascade_with_stylesheets,
};

use crate::{StyleLayoutError, compute_runtime_style_layout_with_intrinsic_size_snapshot};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const REFERENCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/references/c14-3-image-intrinsic-contributions-v1.json"
);

struct Fixture {
    document: HostDocument,
    root: HostNodeHandle,
    basis: HostNodeHandle,
    image: HostNodeHandle,
}

fn fixture(nested_basis: bool) -> Fixture {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let basis = if nested_basis {
        document.reserve_node_handle().unwrap()
    } else {
        root
    };
    let image = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(1433).unwrap(), document.document_revision());
    batch
        .push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML.to_owned(),
            local_name: "main".to_owned(),
        })
        .push(DocumentOperation::SetAttribute {
            node: root,
            name: AttributeName::new(None, "id").unwrap(),
            value: "root".to_owned().into(),
        });
    if nested_basis {
        batch
            .push(DocumentOperation::CreateElement {
                node: basis,
                namespace: HTML.to_owned(),
                local_name: "div".to_owned(),
            })
            .push(DocumentOperation::SetAttribute {
                node: basis,
                name: AttributeName::new(None, "id").unwrap(),
                value: "basis".to_owned().into(),
            })
            .push(DocumentOperation::InsertBefore {
                parent: HostParent::Node(root),
                node: basis,
                before: None,
            });
    }
    batch
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
            parent: HostParent::Node(basis),
            node: image,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        });
    document.commit(batch).unwrap();
    Fixture {
        document,
        root,
        basis,
        image,
    }
}

fn add_element_child(fixture: &mut Fixture) {
    let child = fixture.document.reserve_node_handle().unwrap();
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(1433).unwrap(),
        fixture.document.document_revision(),
    );
    batch
        .push(DocumentOperation::CreateElement {
            node: child,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(fixture.image),
            node: child,
            before: None,
        });
    fixture.document.commit(batch).unwrap();
}

fn viewport() -> CssViewport {
    CssViewport {
        width_css_px: 1280.0,
        height_css_px: 633.0,
        device_scale_factor: 1.0,
        environment_revision: EnvironmentRevision::default(),
        media_environment: CssMediaEnvironment::DESKTOP,
    }
}

fn run(
    fixture: &Fixture,
    root_css: &str,
    basis_css: &str,
    image_css: &str,
) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
    let snapshot = fixture.document.snapshot();
    let view =
        StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), fixture.root)
            .unwrap();
    let stylesheet = StylesheetSource {
        id: "c14-3-intrinsic-contributions".to_owned(),
        base_url: "https://spinon.invalid/c14-3.css".to_owned(),
        origin: CssOrigin::Author,
        css: format!(
            "* {{ box-sizing: content-box; margin: 0; padding: 0; }}\
             #root {{ display: block; {root_css} }}\
             #basis {{ display: block; {basis_css} }}\
             #image {{ display: block; {image_css} }}"
        ),
    };
    let computed = compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
        &view,
        &[stylesheet],
        viewport(),
        StyleRevision::INITIAL,
    )
    .unwrap();
    let intrinsic = LayoutIntrinsicSizeSnapshot::new(
        LayoutSourceRevision::HostDocument {
            generation: snapshot.generation(),
            document: snapshot.document_revision(),
            render_tree: snapshot.render_tree_revision(),
        },
        IntrinsicSizeRevision::INITIAL.checked_next().unwrap(),
        BTreeMap::from([(fixture.image.id(), LayoutIntrinsicSize::new(24.0, 16.0))]),
    )
    .unwrap();
    compute_runtime_style_layout_with_intrinsic_size_snapshot(
        &snapshot,
        fixture.root,
        computed,
        viewport(),
        intrinsic,
    )
}

#[test]
fn fourteen_image_intrinsic_contribution_cases_match_chrome_155_css_pixels() {
    let reference: Value = serde_json::from_slice(&fs::read(REFERENCE).unwrap()).unwrap();
    assert_eq!(
        reference["schema"],
        "spinon.c14.3.intrinsic-contributions.v1"
    );
    assert_eq!(reference["browser"]["version"], "155.0.8059.85");
    let cases = [
        (
            "min-content",
            false,
            "width: 210px; height: 150px;",
            "",
            "width: min-content; height: auto;",
        ),
        (
            "border-box-min-content",
            false,
            "width: 200px; height: 120px;",
            "",
            "box-sizing: border-box; width: min-content; height: auto; padding: 4px; border-width: 1px; border-style: solid;",
        ),
        (
            "max-content",
            false,
            "width: 210px; height: 150px;",
            "",
            "width: max-content; height: auto;",
        ),
        (
            "fit-content-small-basis",
            false,
            "width: 12px; height: 150px; padding: 4px; border-width: 1px; border-style: solid;",
            "",
            "width: fit-content; height: auto;",
        ),
        (
            "percent-width",
            false,
            "width: 200px; height: 120px;",
            "",
            "width: 50%; height: auto;",
        ),
        (
            "percent-height",
            false,
            "width: 200px; height: 120px;",
            "",
            "width: auto; height: 50%;",
        ),
        (
            "fit-content-max-clamped",
            false,
            "width: 200px; height: 120px;",
            "",
            "width: fit-content; height: auto; max-width: 12px;",
        ),
        (
            "percent-cyclic-width",
            true,
            "width: 210px; height: 150px;",
            "width: min-content; height: auto;",
            "width: 50%; height: auto;",
        ),
        (
            "flex-min-content",
            false,
            "display: flex; flex-flow: row nowrap; align-items: flex-start; width: 200px; height: 120px;",
            "",
            "flex: 0 0 auto; width: min-content; height: auto;",
        ),
        (
            "flex-percent-width",
            false,
            "display: flex; flex-flow: row nowrap; align-items: flex-start; width: 200px; height: 120px;",
            "",
            "flex: 0 0 auto; width: 50%; height: auto;",
        ),
        (
            "flex-percent-height",
            false,
            "display: flex; flex-flow: row nowrap; align-items: flex-start; width: 200px; height: 120px;",
            "",
            "flex: 0 0 auto; width: auto; height: 50%;",
        ),
        (
            "grid-min-content",
            false,
            "display: grid; grid-template-columns: 200px; grid-template-rows: 120px; align-items: start; width: 200px; height: 120px;",
            "",
            "width: min-content; height: auto;",
        ),
        (
            "grid-percent-width",
            false,
            "display: grid; grid-template-columns: 200px; grid-template-rows: 120px; align-items: start; width: 200px; height: 120px;",
            "",
            "width: 50%; height: auto;",
        ),
        (
            "grid-percent-height",
            false,
            "display: grid; grid-template-columns: 200px; grid-template-rows: 120px; align-items: start; width: 200px; height: 120px;",
            "",
            "width: auto; height: 50%;",
        ),
    ];

    for (id, nested, root_css, basis_css, image_css) in cases {
        let case = reference["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|case| case["id"] == id)
            .unwrap_or_else(|| panic!("missing oracle case {id}"));
        let fixture = fixture(nested);
        let output = run(&fixture, root_css, basis_css, image_css)
            .unwrap_or_else(|error| panic!("{id} rejected: {error}"));
        let image = output.layout.frames[&fixture.image.id()];
        let basis = output.layout.frames[&fixture.basis.id()];
        let expected = &case["imageUsedSize"];
        let tolerance = reference["toleranceCssPx"].as_f64().unwrap() as f32;
        let expected_basis = &case["containingBlockContentSize"];
        let edge_sum = if id == "fit-content-small-basis" {
            10.0
        } else {
            0.0
        };
        for (axis, actual, expected) in [
            (
                "basis.width",
                basis.width - edge_sum,
                expected_basis["width"].as_f64().unwrap() as f32,
            ),
            (
                "basis.height",
                basis.height - edge_sum,
                expected_basis["height"].as_f64().unwrap() as f32,
            ),
        ] {
            assert!(
                (actual - expected).abs() <= tolerance,
                "{id}.{axis}: actual {actual} != Chrome {expected} (tolerance {tolerance})"
            );
        }
        for (axis, actual, expected) in [
            (
                "width",
                image.width,
                expected["width"].as_f64().unwrap() as f32,
            ),
            (
                "height",
                image.height,
                expected["height"].as_f64().unwrap() as f32,
            ),
        ] {
            assert!(
                (actual - expected).abs() <= tolerance,
                "{id}.{axis}: actual {actual} != Chrome {expected} (tolerance {tolerance})"
            );
        }
        if id == "percent-cyclic-width" {
            assert_eq!((basis.width, basis.height), (0.0, 0.0));
        }
        if id == "fit-content-small-basis" {
            let root = output.layout.frames[&fixture.root.id()];
            let right_overrun = image.x + image.width - (root.x + root.width);
            assert_eq!(right_overrun, 7.0);
        }
    }
}

#[test]
fn fit_content_function_is_rejected_for_the_image_node() {
    let fixture = fixture(false);
    let error = run(
        &fixture,
        "width: 200px; height: 120px;",
        "",
        "width: fit-content(12px); height: auto;",
    )
    .unwrap_err();
    assert!(matches!(
        error,
        StyleLayoutError::Layout(spinon_layout::LayoutError::UnsupportedIntrinsicSizing {
            node,
            ..
        }) if node == fixture.image.id()
    ));
}

#[test]
fn intrinsic_keyword_on_unapproved_container_is_rejected() {
    let fixture = fixture(false);
    let error = run(
        &fixture,
        "width: min-content; height: auto;",
        "",
        "width: auto; height: auto;",
    )
    .unwrap_err();
    assert!(matches!(
        error,
        StyleLayoutError::Layout(spinon_layout::LayoutError::UnsupportedIntrinsicSizing {
            node,
            ..
        }) if node == fixture.root.id()
    ));
}

#[test]
fn cyclic_min_content_parent_in_flex_context_is_rejected() {
    let fixture = fixture(true);
    let error = run(
        &fixture,
        "display: flex; flex-flow: row nowrap; width: 210px; height: 150px;",
        "width: min-content; height: auto;",
        "width: 50%; height: auto;",
    )
    .unwrap_err();
    assert!(matches!(
        error,
        StyleLayoutError::Layout(spinon_layout::LayoutError::UnsupportedIntrinsicSizing {
            node,
            ..
        }) if node == fixture.basis.id()
    ));
}

#[test]
fn cyclic_min_content_parent_with_min_width_is_rejected() {
    let fixture = fixture(true);
    let error = run(
        &fixture,
        "width: 210px; height: 150px;",
        "width: min-content; min-width: 40px; height: auto;",
        "width: 50%; height: auto;",
    )
    .unwrap_err();
    assert!(matches!(
        error,
        StyleLayoutError::Layout(spinon_layout::LayoutError::UnsupportedIntrinsicSizing {
            node,
            ..
        }) if node == fixture.basis.id()
    ));
}

#[test]
fn intrinsic_sizing_on_non_leaf_image_is_rejected() {
    let mut fixture = fixture(false);
    add_element_child(&mut fixture);
    let error = run(
        &fixture,
        "width: 200px; height: 120px;",
        "",
        "width: min-content; height: auto;",
    )
    .unwrap_err();
    assert!(matches!(
        error,
        StyleLayoutError::Layout(spinon_layout::LayoutError::UnsupportedIntrinsicSizing {
            node,
            ..
        }) if node == fixture.image.id()
    ));
}
