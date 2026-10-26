use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId,
    StyleRevision,
};

use crate::{CssOrigin, StylesheetSource};

use super::{
    ComputedStyleProfile, CssViewport, StyloDocumentView,
    compute_c19_computed_color_cascade_with_stylesheets,
    compute_runtime_flex_custom_properties_paint_cascade,
    compute_runtime_incremental_cascade_with_stylesheets,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";

fn create_element(batch: &mut DocumentChangeBatch, node: spinon_core::HostNodeHandle, style: &str) {
    batch.push(DocumentOperation::CreateElement {
        node,
        namespace: HTML.to_owned(),
        local_name: "div".to_owned(),
    });
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, "style").unwrap(),
        value: style.to_owned().into(),
    });
}

fn node_style(snapshot: &super::ComputedStyleSnapshot, id: spinon_core::NodeId) -> &str {
    snapshot
        .elements
        .iter()
        .find(|element| element.node_id == id)
        .unwrap()
        .properties
        .get("width")
        .unwrap()
}

#[test]
fn inline_style_subtree_reuse_matches_full_cascade_and_keeps_sibling_output() {
    let mut document = HostDocument::new().unwrap();
    let handles = (0..6)
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let mut initial =
        DocumentChangeBatch::new(OwnerId::new(5401).unwrap(), document.document_revision());
    create_element(
        &mut initial,
        handles[0],
        "display:flex;width:320px;height:180px;--tile-size:41px",
    );
    create_element(
        &mut initial,
        handles[1],
        "display:flex;flex-wrap:nowrap;width:140px;height:80px;--tile-size:32px;background-color:#101827",
    );
    create_element(
        &mut initial,
        handles[2],
        "width:var(--tile-size);height:10px",
    );
    create_element(
        &mut initial,
        handles[3],
        "display:flex;width:140px;height:80px;--tile-size:41px",
    );
    create_element(
        &mut initial,
        handles[4],
        "width:var(--tile-size);height:10px",
    );
    create_element(&mut initial, handles[5], "width:50%;height:10px");
    for (node, parent) in [
        (handles[0], HostParent::Root),
        (handles[1], HostParent::Node(handles[0])),
        (handles[2], HostParent::Node(handles[1])),
        (handles[3], HostParent::Node(handles[0])),
        (handles[4], HostParent::Node(handles[3])),
        (handles[5], HostParent::Node(handles[0])),
    ] {
        initial.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(initial).unwrap();
    let previous_view = StyloDocumentView::new_html_fragment_child_shared(
        Arc::new(document.snapshot()),
        handles[0],
    )
    .unwrap();
    let viewport = CssViewport {
        width_css_px: 320.0,
        height_css_px: 180.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: super::CssMediaEnvironment::MOBILE,
    };
    let previous = compute_runtime_flex_custom_properties_paint_cascade(
        &previous_view,
        viewport,
        Default::default(),
    )
    .unwrap();

    let mut change =
        DocumentChangeBatch::new(OwnerId::new(5401).unwrap(), document.document_revision());
    change.push(DocumentOperation::SetAttribute {
        node: handles[1],
        name: AttributeName::new(None, "style").unwrap(),
        value: "display:flex;flex-wrap:wrap;width:140px;height:80px;--tile-size:46px;background-color:#243047"
            .to_owned()
            .into(),
    });
    document.commit(change).unwrap();
    let current_view = StyloDocumentView::new_html_fragment_child_shared(
        Arc::new(document.snapshot()),
        handles[0],
    )
    .unwrap();
    let full = compute_runtime_flex_custom_properties_paint_cascade(
        &current_view,
        viewport,
        Default::default(),
    )
    .unwrap();
    let (partial, stats) = compute_runtime_incremental_cascade_with_stylesheets(
        &current_view,
        &[],
        viewport,
        Default::default(),
        ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1,
        &previous,
        &[handles[1].id()],
    )
    .unwrap()
    .expect("안전한 inline style 변경은 부분 cascade 경로를 허용해야 합니다");

    assert_eq!(partial.elements, full.elements);
    assert_eq!(partial.diagnostics, full.diagnostics);
    assert_eq!(
        partial
            .elements
            .iter()
            .find(|element| element.node_id == handles[1].id())
            .unwrap()
            .properties["flex-wrap"],
        "wrap"
    );
    assert_eq!(
        partial
            .elements
            .iter()
            .find(|element| element.node_id == handles[1].id())
            .unwrap()
            .properties["background-color"],
        "rgb(36, 48, 71)"
    );
    assert_eq!(node_style(&partial, handles[2].id()), "46px");
    assert_eq!(node_style(&partial, handles[4].id()), "41px");
    assert_eq!(
        partial
            .elements
            .iter()
            .find(|element| element.node_id == handles[5].id())
            .unwrap()
            .layout_dimensions
            .width,
        super::ComputedCssDimension::Percentage(0.5)
    );
    assert_eq!(stats.recomputed_style_elements, 2);
    assert_eq!(stats.reused_style_elements, 4);
    assert_eq!(stats.context_style_elements, 1);
}

#[test]
fn c19_computed_color_incremental_reuse_matches_full_cascade() {
    let mut document = HostDocument::new().unwrap();
    let handles = (0..4)
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let mut initial =
        DocumentChangeBatch::new(OwnerId::new(1901).unwrap(), document.document_revision());
    create_element(
        &mut initial,
        handles[0],
        "color:color(display-p3 0.8 0.3 0.1);background-color:oklch(60% 0.15 230deg)",
    );
    create_element(
        &mut initial,
        handles[1],
        "color:oklch(60% 0.15 230deg);background-color:currentColor",
    );
    create_element(
        &mut initial,
        handles[2],
        "color:inherit;background-color:currentColor",
    );
    create_element(
        &mut initial,
        handles[3],
        "color:inherit;background-color:currentColor",
    );
    for (node, parent) in [
        (handles[0], HostParent::Root),
        (handles[1], HostParent::Node(handles[0])),
        (handles[2], HostParent::Node(handles[1])),
        (handles[3], HostParent::Node(handles[0])),
    ] {
        initial.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(initial).unwrap();

    let previous_view = StyloDocumentView::new_html_fragment_child_shared(
        Arc::new(document.snapshot()),
        handles[0],
    )
    .unwrap();
    let viewport = CssViewport {
        width_css_px: 320.0,
        height_css_px: 180.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: super::CssMediaEnvironment::MOBILE,
    };
    let previous_style_revision = StyleRevision::default();
    let previous = compute_c19_computed_color_cascade_with_stylesheets(
        &previous_view,
        &[],
        viewport,
        previous_style_revision,
    )
    .unwrap();
    let previous_root = previous
        .elements
        .iter()
        .find(|element| element.node_id == handles[0].id())
        .unwrap()
        .clone();
    let previous_target = previous
        .elements
        .iter()
        .find(|element| element.node_id == handles[2].id())
        .unwrap();
    let previous_target_color = previous_target.computed_color.unwrap();
    let previous_target_background = previous_target.computed_background_color.unwrap();
    let previous_sibling = previous
        .elements
        .iter()
        .find(|element| element.node_id == handles[3].id())
        .unwrap()
        .clone();

    let mut change =
        DocumentChangeBatch::new(OwnerId::new(1901).unwrap(), document.document_revision());
    change.push(DocumentOperation::SetAttribute {
        node: handles[1],
        name: AttributeName::new(None, "style").unwrap(),
        value: "color:color(display-p3 0.9 0.2 0.1);background-color:currentColor"
            .to_owned()
            .into(),
    });
    document.commit(change).unwrap();

    let current_view = StyloDocumentView::new_html_fragment_child_shared(
        Arc::new(document.snapshot()),
        handles[0],
    )
    .unwrap();
    let current_style_revision = previous_style_revision.checked_next().unwrap();
    let full = compute_c19_computed_color_cascade_with_stylesheets(
        &current_view,
        &[],
        viewport,
        current_style_revision,
    )
    .unwrap();
    let (incremental, stats) = compute_runtime_incremental_cascade_with_stylesheets(
        &current_view,
        &[],
        viewport,
        current_style_revision,
        ComputedStyleProfile::C19ComputedColorV1,
        &previous,
        &[handles[1].id()],
    )
    .unwrap()
    .expect("C19.1의 안전한 inline subtree 변경은 재사용을 허용해야 합니다");

    assert_eq!(incremental.elements, full.elements);
    assert_eq!(incremental.diagnostics, full.diagnostics);
    assert_eq!(incremental.style_revision, current_style_revision);
    for element in incremental.elements.iter() {
        assert_eq!(element.properties.len(), 2);
        assert!(element.properties.contains_key("background-color"));
        assert!(element.properties.contains_key("color"));
        assert!(element.background_color.is_none());
        assert!(element.background_paint.is_none());
    }
    assert_eq!(
        incremental
            .elements
            .iter()
            .find(|element| element.node_id == handles[0].id())
            .unwrap(),
        &previous_root,
        "context root output is reused after its style is recalculated for inheritance"
    );
    let updated_target = incremental
        .elements
        .iter()
        .find(|element| element.node_id == handles[2].id())
        .unwrap();
    assert_ne!(
        updated_target.computed_color.unwrap(),
        previous_target_color
    );
    assert_ne!(
        updated_target.computed_background_color.unwrap(),
        previous_target_background
    );
    assert_eq!(
        updated_target.computed_color, updated_target.computed_background_color,
        "상속된 color와 currentColor background carrier는 같아야 합니다"
    );
    assert_eq!(
        incremental
            .elements
            .iter()
            .find(|element| element.node_id == handles[3].id())
            .unwrap(),
        &previous_sibling,
        "변경 없는 sibling output은 그대로 재사용해야 합니다"
    );
    assert_eq!(stats.recomputed_style_elements, 2);
    assert_eq!(stats.reused_style_elements, 2);
    assert_eq!(stats.context_style_elements, 1);

    let author_stylesheet = StylesheetSource {
        id: "c19-incremental-unsupported-author-sheet".to_owned(),
        base_url: "https://spinon.invalid/c19-incremental.css".to_owned(),
        origin: CssOrigin::Author,
        css: "div { color: red }".to_owned(),
    };
    assert!(
        compute_runtime_incremental_cascade_with_stylesheets(
            &current_view,
            &[author_stylesheet],
            viewport,
            current_style_revision,
            ComputedStyleProfile::C19ComputedColorV1,
            &previous,
            &[handles[1].id()],
        )
        .unwrap()
        .is_none()
    );
    assert!(
        compute_runtime_incremental_cascade_with_stylesheets(
            &current_view,
            &[],
            viewport,
            current_style_revision,
            ComputedStyleProfile::RuntimeBlockPaintV1,
            &previous,
            &[handles[1].id()],
        )
        .unwrap()
        .is_none()
    );
    let mut changed_viewport = viewport;
    changed_viewport.width_css_px += 1.0;
    assert!(
        compute_runtime_incremental_cascade_with_stylesheets(
            &current_view,
            &[],
            changed_viewport,
            current_style_revision,
            ComputedStyleProfile::C19ComputedColorV1,
            &previous,
            &[handles[1].id()],
        )
        .unwrap()
        .is_none()
    );
    let skipped_style_revision = current_style_revision.checked_next().unwrap();
    assert!(
        compute_runtime_incremental_cascade_with_stylesheets(
            &current_view,
            &[],
            viewport,
            skipped_style_revision,
            ComputedStyleProfile::C19ComputedColorV1,
            &previous,
            &[handles[1].id()],
        )
        .unwrap()
        .is_none()
    );
    assert!(
        compute_runtime_incremental_cascade_with_stylesheets(
            &previous_view,
            &[],
            viewport,
            previous_style_revision,
            ComputedStyleProfile::C19ComputedColorV1,
            &previous,
            &[handles[1].id()],
        )
        .unwrap()
        .is_none()
    );
}
