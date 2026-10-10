use std::sync::Arc;

use super::{
    ComputedCssDimension, CssCascadeError, CssViewport, compute_basic_cascade,
    compute_runtime_flex_custom_properties_cascade_with_stylesheets,
    compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets,
};
use crate::{CssOrigin, StylesheetSource, StyloDocumentView};
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const FONT_METRIC_UNITS: &[&str] = &[
    "ex", "rex", "ch", "rch", "cap", "rcap", "ic", "ric", "lh", "rlh",
];

fn runtime_view(
    root_style: &str,
    child_style: &str,
) -> (
    Arc<spinon_core::HostDocumentSnapshot>,
    HostNodeHandle,
    HostNodeHandle,
) {
    let mut document = HostDocument::new().unwrap();
    let owner = OwnerId::new(604).unwrap();
    let root = document.reserve_node_handle().unwrap();
    let child = document.reserve_node_handle().unwrap();
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    for (handle, parent, id, style) in [
        (root, HostParent::Root, "mount", root_style),
        (child, HostParent::Node(root), "target", child_style),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node: handle,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: handle,
            name: AttributeName::new(None, "id").unwrap(),
            value: id.into(),
        });
        if !style.is_empty() {
            batch.push(DocumentOperation::SetAttribute {
                node: handle,
                name: AttributeName::new(None, "style").unwrap(),
                value: style.into(),
            });
        }
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node: handle,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    (Arc::new(document.snapshot()), root, child)
}

fn stylesheet(css: &str) -> StylesheetSource {
    StylesheetSource {
        id: "c06-4-font-relative-test".to_owned(),
        base_url: "https://spinon.invalid/c06-4.css".to_owned(),
        origin: CssOrigin::Author,
        css: css.to_owned(),
    }
}

#[test]
fn synthetic_document_root_drives_rem_and_body_inheritance_drives_em() {
    let (snapshot, root, target) = runtime_view("", "");
    let view = StyloDocumentView::new_html_runtime_mount_shared(snapshot, root).unwrap();
    let styles = [stylesheet(
        ":root { font-size: 1.25rem; } body { font-size: 2rem; } #target { font-size: 1.25em; width: 2rem; height: 1em; }",
    )];

    let computed = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        &view,
        &styles,
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();

    assert_eq!(computed.document_root_font_size_css_px, 20.0);
    let mount_style = computed
        .elements
        .iter()
        .find(|element| element.node_id == root.id())
        .unwrap();
    assert_eq!(mount_style.font_size_css_px, 40.0);
    assert_eq!(mount_style.properties["font-size"], "40px");
    let target_style = computed
        .elements
        .iter()
        .find(|element| element.node_id == target.id())
        .unwrap();
    assert_eq!(target_style.font_size_css_px, 50.0);
    assert_eq!(target_style.properties["font-size"], "50px");
    assert_eq!(target_style.properties["width"], "40px");
    assert_eq!(
        target_style.layout_dimensions.width,
        ComputedCssDimension::LengthPx(40.0)
    );
    assert_eq!(
        target_style.layout_dimensions.height,
        ComputedCssDimension::LengthPx(50.0)
    );
}

#[test]
fn synthetic_body_inherits_direction_and_custom_properties_into_the_mount() {
    let (snapshot, root, target) = runtime_view("", "");
    let view = StyloDocumentView::new_html_runtime_mount_shared(snapshot, root).unwrap();
    let computed = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        &view,
        &[stylesheet(
            ":root { font-size: 1.25rem; } body { direction: rtl; font-size: 2rem; --inherited-size: 2em; } #target { width: var(--inherited-size); }",
        )],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();

    let mount = computed
        .elements
        .iter()
        .find(|element| element.node_id == root.id())
        .unwrap();
    assert_eq!(mount.properties["direction"], "rtl");
    assert_eq!(mount.font_size_css_px, 40.0);

    let target = computed
        .elements
        .iter()
        .find(|element| element.node_id == target.id())
        .unwrap();
    assert_eq!(
        target.layout_dimensions.width,
        ComputedCssDimension::LengthPx(80.0)
    );
}

#[test]
fn zero_document_root_font_size_is_preserved_for_rem_and_em() {
    let (snapshot, root, target) = runtime_view("", "");
    let view = StyloDocumentView::new_html_runtime_mount_shared(snapshot, root).unwrap();
    let computed = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        &view,
        &[stylesheet(
            ":root { font-size: 0px; } #mount { width: 2rem; height: 1rem; font-size: 1rem; } #target { width: 2rem; height: 1em; }",
        )],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();

    assert_eq!(computed.document_root_font_size_css_px, 0.0);
    let mount = computed
        .elements
        .iter()
        .find(|element| element.node_id == root.id())
        .unwrap();
    assert_eq!(mount.font_size_css_px, 0.0);
    assert_eq!(
        mount.layout_dimensions.width,
        ComputedCssDimension::LengthPx(0.0)
    );
    let target = computed
        .elements
        .iter()
        .find(|element| element.node_id == target.id())
        .unwrap();
    assert_eq!(
        target.layout_dimensions.width,
        ComputedCssDimension::LengthPx(0.0)
    );
    assert_eq!(
        target.layout_dimensions.height,
        ComputedCssDimension::LengthPx(0.0)
    );
}

#[test]
fn metric_units_fail_closed_in_inline_and_stylesheet_custom_property_fallbacks() {
    let (snapshot, root, _) = runtime_view("--size: var(--missing, 1ch); width: var(--size)", "");
    let view = StyloDocumentView::new_html_runtime_mount_shared(snapshot, root).unwrap();
    let error = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap_err();
    assert!(matches!(
        error,
        CssCascadeError::UnsupportedFontMetricUnit {
            node: Some(_),
            unit,
            ..
        } if unit == "ch"
    ));

    let (snapshot, root, _) = runtime_view("", "");
    let view = StyloDocumentView::new_html_runtime_mount_shared(snapshot, root).unwrap();
    let error = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        &view,
        &[stylesheet(
            "#target { --size: var(--missing, 1rlh); width: var(--size); }",
        )],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap_err();
    assert!(matches!(
        error,
        CssCascadeError::UnsupportedFontMetricUnit {
            node: None,
            unit,
            ..
        } if unit == "rlh"
    ));
}

#[test]
fn every_font_metric_unit_fails_closed_across_runtime_style_inputs() {
    for unit in FONT_METRIC_UNITS {
        for (inline_style, author_css, case) in [
            (format!("width: 1{unit}"), String::new(), "inline length"),
            (
                format!("font-size: 1{unit}"),
                String::new(),
                "inline font-size",
            ),
            (
                String::new(),
                format!("#mount {{ width: 1{unit}; }}"),
                "stylesheet length",
            ),
            (
                format!("--size: 1{unit}; width: var(--size)"),
                String::new(),
                "inline custom property",
            ),
            (
                format!("width: var(--missing, 1{unit})"),
                String::new(),
                "inline fallback",
            ),
            (
                String::new(),
                format!("#mount {{ --size: 1{unit}; width: var(--size); }}"),
                "stylesheet custom-property substitution",
            ),
            (
                String::new(),
                format!("#mount {{ width: var(--missing, 1{unit}); }}"),
                "stylesheet fallback",
            ),
            (
                String::new(),
                format!("#mount {{ width: calc(1{unit}); }}"),
                "nested stylesheet function",
            ),
        ] {
            let (snapshot, root, _) = runtime_view(&inline_style, "");
            let view = StyloDocumentView::new_html_runtime_mount_shared(snapshot, root).unwrap();
            let stylesheets = (!author_css.is_empty())
                .then(|| stylesheet(&author_css))
                .into_iter()
                .collect::<Vec<_>>();
            let error = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
                &view,
                &stylesheets,
                CssViewport::C04_FIXTURE,
                Default::default(),
            )
            .unwrap_err();

            assert!(
                matches!(
                    &error,
                    CssCascadeError::UnsupportedFontMetricUnit {
                        unit: rejected,
                        ..
                    } if rejected == unit
                ),
                "{case} must reject {unit}, got {error}"
            );
        }
    }
}

#[test]
fn metric_unit_scanning_is_limited_to_runtime_layout_profiles() {
    let (snapshot, root, _) = runtime_view("width: 1ch", "");
    let view = StyloDocumentView::new_html_runtime_mount_shared(snapshot, root).unwrap();
    assert!(compute_basic_cascade(&view, &[], CssViewport::C04_FIXTURE).is_ok());
}

#[test]
fn synthetic_html_and_body_layout_changes_fail_instead_of_being_discarded() {
    for (css, expected_element, expected_property) in [
        ("html { width: 200px; }", "html", "width"),
        ("html { padding: 2px; }", "html", "padding-top"),
        ("body { margin: 8px; }", "body", "margin-top"),
        ("body { display: none; }", "body", "display"),
    ] {
        let (snapshot, root, _) = runtime_view("", "");
        let view = StyloDocumentView::new_html_runtime_mount_shared(snapshot, root).unwrap();
        let error = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
            &view,
            &[stylesheet(css)],
            CssViewport::C04_FIXTURE,
            Default::default(),
        )
        .unwrap_err();

        assert!(
            matches!(error,
                CssCascadeError::UnsupportedSyntheticDocumentStyle {
                    element,
                    property,
                    ..
                } if element == expected_element && property == expected_property
            ),
            "{css} must fail at {expected_element}.{expected_property}, got {error}"
        );
    }
}

#[test]
fn synthetic_document_background_paint_fails_instead_of_disappearing() {
    let (snapshot, root, _) = runtime_view("", "");
    let view = StyloDocumentView::new_html_runtime_mount_shared(snapshot, root).unwrap();
    let error = compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets(
        &view,
        &[stylesheet("body { background-color: #ff0000; }")],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap_err();

    assert!(matches!(
        error,
        CssCascadeError::UnsupportedSyntheticDocumentStyle {
            element: "body",
            property: "background-color",
            ..
        }
    ));
}
