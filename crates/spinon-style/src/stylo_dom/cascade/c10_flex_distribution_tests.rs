use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};

use super::{
    ComputedCssDimension, ComputedCssSpacingValue, ComputedStyleProfile, CssViewport,
    StyloDocumentView, compute_runtime_flex_custom_properties_cascade,
    compute_runtime_flex_custom_properties_paint_cascade,
    compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets,
    compute_runtime_flex_layout_cascade, compute_runtime_flex_paint_cascade,
    compute_runtime_flex_registered_properties_cascade_with_stylesheets,
    compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets,
    compute_runtime_incremental_cascade_with_stylesheets,
};
use crate::{CssOrigin, StylesheetSource};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const ROOT_STYLE: &str = "display:flex;box-sizing:border-box;width:300px;height:40px;flex-direction:row;flex-wrap:nowrap;align-items:flex-start;justify-content:flex-start;column-gap:20px;min-width:0;min-height:0";
const FIRST_STYLE: &str = "width:220px;height:10px;flex:1 0 100px;min-width:120px;max-width:130px;box-sizing:border-box;padding-left:2px;border-left:2px solid red";
const SECOND_STYLE: &str = "width:80px;height:10px;flex:3 0 100px";

fn create_element(batch: &mut DocumentChangeBatch, handle: HostNodeHandle, id: &str, style: &str) {
    batch.push(DocumentOperation::CreateElement {
        node: handle,
        namespace: HTML.to_owned(),
        local_name: "div".to_owned(),
    });
    for (name, value) in [("id", id), ("style", style)] {
        batch.push(DocumentOperation::SetAttribute {
            node: handle,
            name: AttributeName::new(None, name).unwrap(),
            value: value.to_owned().into(),
        });
    }
}

fn document_with_styles(
    root_style: &str,
    first_style: &str,
    second_style: &str,
) -> (HostDocument, [HostNodeHandle; 3]) {
    let mut document = HostDocument::new().unwrap();
    let handles = [
        document.reserve_node_handle().unwrap(),
        document.reserve_node_handle().unwrap(),
        document.reserve_node_handle().unwrap(),
    ];
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(60302).unwrap(), document.document_revision());
    create_element(&mut batch, handles[0], "c10-flex-root", root_style);
    create_element(&mut batch, handles[1], "c10-flex-first", first_style);
    create_element(&mut batch, handles[2], "c10-flex-second", second_style);
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Root,
        node: handles[0],
        before: None,
    });
    for child in &handles[1..] {
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(handles[0]),
            node: *child,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    (document, handles)
}

fn view(document: &HostDocument, root: HostNodeHandle) -> StyloDocumentView {
    StyloDocumentView::new_html_fragment_child_shared(Arc::new(document.snapshot()), root).unwrap()
}

fn element(
    snapshot: &super::ComputedStyleSnapshot,
    node: spinon_core::NodeId,
) -> &super::ComputedElementStyle {
    snapshot
        .elements
        .iter()
        .find(|element| element.node_id == node)
        .unwrap_or_else(|| panic!("computed style에 node {node}가 없습니다"))
}

fn assert_flex_projection(snapshot: &super::ComputedStyleSnapshot, handles: &[HostNodeHandle; 3]) {
    let root = element(snapshot, handles[0].id());
    assert_eq!(root.properties["display"], "flex");
    assert_eq!(root.properties["flex-direction"], "row");
    assert_eq!(
        root.layout_spacing.column_gap,
        ComputedCssSpacingValue::LengthPx(20.0),
        "profile {:?}, computed gap {:?}",
        snapshot.profile,
        root.properties.get("column-gap")
    );
    let first = element(snapshot, handles[1].id());
    assert_eq!(first.properties["flex-grow"], "1");
    assert_eq!(first.properties["flex-shrink"], "0");
    assert_eq!(first.properties["flex-basis"], "100px");
    assert_eq!(
        first.layout_dimensions.width,
        ComputedCssDimension::LengthPx(220.0)
    );
    assert_eq!(
        first.layout_dimensions.flex_basis,
        ComputedCssDimension::LengthPx(100.0)
    );
    assert_eq!(
        first.layout_dimensions.min_width,
        ComputedCssDimension::LengthPx(120.0)
    );
    assert_eq!(
        first.layout_dimensions.max_width,
        super::ComputedCssMaxSize::LengthPx(130.0)
    );
    assert_eq!(
        first.layout_spacing.padding.left,
        ComputedCssSpacingValue::LengthPx(2.0)
    );
    assert_eq!(first.layout_border.left, 2.0);

    let second = element(snapshot, handles[2].id());
    assert_eq!(second.properties["flex-grow"], "3");
    assert_eq!(second.properties["flex-basis"], "100px");
    assert_eq!(
        second.layout_dimensions.flex_basis,
        ComputedCssDimension::LengthPx(100.0)
    );
}

#[test]
fn all_six_runtime_flex_profiles_keep_the_same_typed_distribution_inputs() {
    let (document, handles) = document_with_styles(ROOT_STYLE, FIRST_STYLE, SECOND_STYLE);
    let view = view(&document, handles[0]);
    let snapshots = [
        compute_runtime_flex_layout_cascade(
            &view,
            CssViewport::C04_FIXTURE,
            StyleRevision::INITIAL,
        )
        .unwrap(),
        compute_runtime_flex_paint_cascade(&view, CssViewport::C04_FIXTURE, StyleRevision::INITIAL)
            .unwrap(),
        compute_runtime_flex_custom_properties_cascade(
            &view,
            CssViewport::C04_FIXTURE,
            StyleRevision::INITIAL,
        )
        .unwrap(),
        compute_runtime_flex_custom_properties_paint_cascade(
            &view,
            CssViewport::C04_FIXTURE,
            StyleRevision::INITIAL,
        )
        .unwrap(),
        compute_runtime_flex_registered_properties_cascade_with_stylesheets(
            &view,
            &[],
            CssViewport::C04_FIXTURE,
            StyleRevision::INITIAL,
        )
        .unwrap(),
        compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets(
            &view,
            &[],
            CssViewport::C04_FIXTURE,
            StyleRevision::INITIAL,
        )
        .unwrap(),
    ];
    let expected_profiles = [
        ComputedStyleProfile::RuntimeFlexLayoutV1,
        ComputedStyleProfile::RuntimeFlexPaintV1,
        ComputedStyleProfile::RuntimeFlexCustomPropertiesV1,
        ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1,
        ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1,
        ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1,
    ];
    for (snapshot, expected_profile) in snapshots.iter().zip(expected_profiles) {
        assert_eq!(snapshot.profile, expected_profile);
        assert_flex_projection(snapshot, &handles);
    }
}

fn stylesheet(id: &str, css: &str) -> StylesheetSource {
    StylesheetSource {
        id: id.to_owned(),
        base_url: "https://spinon.invalid/c10-flex-distribution.css".to_owned(),
        origin: CssOrigin::Author,
        css: css.to_owned(),
    }
}

#[test]
fn author_stylesheet_custom_property_reaches_layout_and_paint_profiles() {
    let root_style = format!("{ROOT_STYLE};--c10-tile-color:#123456");
    let (document, handles) = document_with_styles(
        &root_style,
        "height:10px;--c10-grow:1",
        "height:10px;--c10-grow:3",
    );
    let view = view(&document, handles[0]);
    let author = stylesheet(
        "c10-flex-distribution-author",
        "#c10-flex-first { flex-grow:var(--c10-grow); flex-shrink:0; flex-basis:100px; background-color:var(--c10-tile-color); } #c10-flex-second { flex-grow:var(--c10-grow); flex-shrink:0; flex-basis:100px; }",
    );
    let snapshot = compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets(
        &view,
        std::slice::from_ref(&author),
        CssViewport::C04_FIXTURE,
        StyleRevision::INITIAL,
    )
    .unwrap();
    assert_eq!(
        element(&snapshot, handles[1].id()).properties["flex-grow"],
        "1"
    );
    assert_eq!(
        element(&snapshot, handles[2].id()).properties["flex-grow"],
        "3"
    );
    assert_eq!(
        element(&snapshot, handles[1].id())
            .layout_dimensions
            .flex_basis,
        ComputedCssDimension::LengthPx(100.0)
    );
    assert!(
        element(&snapshot, handles[1].id())
            .background_paint
            .is_some()
    );
}

#[test]
fn registered_shorthand_author_stylesheet_reaches_full_cascade() {
    let root_style = format!("{ROOT_STYLE};--c10-tile-color:#123456");
    let (document, handles) = document_with_styles(
        &root_style,
        "height:10px;--c10-grow:1",
        "height:10px;--c10-grow:3",
    );
    let view = view(&document, handles[0]);
    let author = stylesheet(
        "c10-flex-distribution-registered-author",
        "@property --c10-grow { syntax: \"<number>\"; inherits: false; initial-value: 1; } #c10-flex-first { flex:var(--c10-grow) 0 100px; } #c10-flex-second { flex:var(--c10-grow) 0 100px; }",
    );
    for snapshot in [
        super::compute_runtime_flex_registered_properties_cascade_with_stylesheets(
            &view,
            std::slice::from_ref(&author),
            CssViewport::C04_FIXTURE,
            StyleRevision::INITIAL,
        )
        .unwrap(),
        compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets(
            &view,
            std::slice::from_ref(&author),
            CssViewport::C04_FIXTURE,
            StyleRevision::INITIAL,
        )
        .unwrap(),
    ] {
        assert_eq!(
            element(&snapshot, handles[1].id()).properties["flex-grow"],
            "1"
        );
        assert_eq!(
            element(&snapshot, handles[2].id()).properties["flex-grow"],
            "3"
        );
        assert_eq!(
            element(&snapshot, handles[1].id())
                .layout_dimensions
                .flex_basis,
            ComputedCssDimension::LengthPx(100.0)
        );
    }
}

#[test]
fn inline_flex_factor_full_and_incremental_cascade_match_after_change() {
    let (mut document, handles) = document_with_styles(
        ROOT_STYLE,
        "height:10px;flex:1 0 100px",
        "height:10px;flex:3 0 100px",
    );
    let previous_view = view(&document, handles[0]);
    let previous = compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets(
        &previous_view,
        &[],
        CssViewport::C04_FIXTURE,
        StyleRevision::INITIAL,
    )
    .unwrap();

    let mut change =
        DocumentChangeBatch::new(OwnerId::new(60302).unwrap(), document.document_revision());
    change.push(DocumentOperation::SetAttribute {
        node: handles[1],
        name: AttributeName::new(None, "style").unwrap(),
        value: "height:10px;flex:2 0 100px".to_owned().into(),
    });
    document.commit(change).unwrap();
    let current_view = view(&document, handles[0]);
    let full = compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets(
        &current_view,
        &[],
        CssViewport::C04_FIXTURE,
        StyleRevision::INITIAL,
    )
    .unwrap();
    let (incremental, _) = compute_runtime_incremental_cascade_with_stylesheets(
        &current_view,
        &[],
        CssViewport::C04_FIXTURE,
        StyleRevision::INITIAL,
        ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1,
        &previous,
        &[handles[1].id()],
    )
    .unwrap()
    .expect("inline factor 변경은 안전한 incremental restyle이어야 합니다");
    assert_eq!(incremental.elements, full.elements);
    assert_eq!(incremental.diagnostics, full.diagnostics);
    assert_eq!(
        element(&incremental, handles[1].id()).properties["flex-grow"],
        "2"
    );
    assert_eq!(
        element(&incremental, handles[1].id())
            .layout_dimensions
            .flex_basis,
        ComputedCssDimension::LengthPx(100.0)
    );
}
