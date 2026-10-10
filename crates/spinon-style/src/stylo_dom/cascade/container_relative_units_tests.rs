use std::sync::Arc;

use super::{
    CssCascadeError, CssViewport, compute_runtime_flex_custom_properties_cascade_with_stylesheets,
};
use crate::{CssOrigin, StylesheetSource, StyloDocumentView};
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const CONTAINER_UNITS: &[&str] = &["cqw", "cqh", "cqi", "cqb", "cqmin", "cqmax"];

fn view(style: Option<&str>) -> (Arc<spinon_core::HostDocumentSnapshot>, HostNodeHandle) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(6066).unwrap(), document.document_revision());
    batch
        .push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        })
        .push(DocumentOperation::SetAttribute {
            node: root,
            name: AttributeName::new(None, "id").unwrap(),
            value: "mount".into(),
        });
    if let Some(style) = style {
        batch.push(DocumentOperation::SetAttribute {
            node: root,
            name: AttributeName::new(None, "style").unwrap(),
            value: style.into(),
        });
    }
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Root,
        node: root,
        before: None,
    });
    document.commit(batch).unwrap();
    (Arc::new(document.snapshot()), root)
}

fn stylesheet(css: &str) -> StylesheetSource {
    StylesheetSource {
        id: "c06-6-container-unit-gate".to_owned(),
        base_url: "https://spinon.invalid/c06/container-units.css".to_owned(),
        origin: CssOrigin::Author,
        css: css.to_owned(),
    }
}

#[test]
fn every_container_relative_unit_fails_closed_in_stylesheets() {
    for unit in CONTAINER_UNITS {
        let (snapshot, root) = view(None);
        let stylo = StyloDocumentView::new_html_runtime_mount_shared(snapshot, root).unwrap();
        let error = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
            &stylo,
            &[stylesheet(&format!("#mount {{ width: 1{unit}; }}"))],
            CssViewport::C04_FIXTURE,
            Default::default(),
        )
        .unwrap_err();
        assert!(
            matches!(&error, CssCascadeError::UnsupportedContainerRelativeUnit { unit: found, node: None, .. } if found.as_str() == *unit),
            "{unit} must be rejected before Stylo's viewport fallback: {error}"
        );
    }
}

#[test]
fn container_units_in_inline_custom_properties_and_fallbacks_fail_closed() {
    for style in [
        "--size: 2cqw; width: var(--size)",
        "width: var(--missing, 2cqh)",
        "--size: 2C\\71W; width: var(--size)",
    ] {
        let (snapshot, root) = view(Some(style));
        let stylo = StyloDocumentView::new_html_runtime_mount_shared(snapshot, root).unwrap();
        let error = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
            &stylo,
            &[],
            CssViewport::C04_FIXTURE,
            Default::default(),
        )
        .unwrap_err();
        assert!(
            matches!(
                &error,
                CssCascadeError::UnsupportedContainerRelativeUnit { node: Some(_), .. }
            ),
            "inline CSS must reject container unit `{style}`: {error}"
        );
    }
}

#[test]
fn container_unit_lookalikes_in_comments_and_strings_do_not_fail_closed() {
    let (snapshot, root) = view(Some(r#"--label: "1cqw"; width: 10px"#));
    let stylo = StyloDocumentView::new_html_runtime_mount_shared(snapshot, root).unwrap();
    let result = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        &stylo,
        &[stylesheet(
            "/* 2cqh */ #mount { height: 20px; --label: '3cqi'; }",
        )],
        CssViewport::C04_FIXTURE,
        Default::default(),
    );
    assert!(
        result.is_ok(),
        "comments/strings are not unit tokens: {result:?}"
    );
}
