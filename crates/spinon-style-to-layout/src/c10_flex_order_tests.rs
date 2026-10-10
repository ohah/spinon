use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_style::{
    ComputedStyleProfile, CssMediaEnvironment, CssViewport, StyloDocumentView,
    compute_runtime_flex_custom_properties_cascade,
    compute_runtime_flex_custom_properties_paint_cascade,
    compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets,
    compute_runtime_flex_layout_cascade, compute_runtime_flex_paint_cascade,
    compute_runtime_flex_registered_properties_cascade_with_stylesheets,
    compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets,
    compute_runtime_incremental_cascade_with_stylesheets,
};

use crate::{StyleLayoutError, compute_runtime_style_layout};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const ROOT_STYLE: &str = "display:flex;box-sizing:border-box;width:160px;height:20px;flex-direction:row;flex-wrap:nowrap;align-items:flex-start;justify-content:flex-start;order:-7";
const CHILD_STYLES: [&str; 3] = [
    "display:block;box-sizing:border-box;width:40px;height:20px;flex:0 0 auto;order:2",
    "display:block;box-sizing:border-box;width:40px;height:20px;flex:0 0 auto;order:-1",
    "display:block;box-sizing:border-box;width:40px;height:20px;flex:0 0 auto",
];

fn make_document(
    root_style: &str,
    child_styles: &[&str],
) -> (HostDocument, HostNodeHandle, Vec<HostNodeHandle>) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let children = child_styles
        .iter()
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(10_132).unwrap(), document.document_revision());
    add_element(&mut batch, root, "c1032-root", root_style, HostParent::Root);
    for (index, (child, style)) in children.iter().zip(child_styles).enumerate() {
        add_element(
            &mut batch,
            *child,
            &format!("c1032-item-{index}"),
            style,
            HostParent::Node(root),
        );
    }
    document.commit(batch).unwrap();
    (document, root, children)
}

fn add_element(
    batch: &mut DocumentChangeBatch,
    node: HostNodeHandle,
    id: &str,
    style: &str,
    parent: HostParent,
) {
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

fn viewport() -> CssViewport {
    CssViewport {
        width_css_px: 320.0,
        height_css_px: 240.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    }
}

#[test]
fn all_six_runtime_flex_profiles_project_order_and_reorder_only_layout_items() {
    let (document, root, children) = make_document(ROOT_STYLE, &CHILD_STYLES);
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), root)
        .unwrap();
    let viewport = viewport();
    let profiles = [
        compute_runtime_flex_layout_cascade(&view, viewport, StyleRevision::INITIAL).unwrap(),
        compute_runtime_flex_paint_cascade(&view, viewport, StyleRevision::INITIAL).unwrap(),
        compute_runtime_flex_custom_properties_cascade(&view, viewport, StyleRevision::INITIAL)
            .unwrap(),
        compute_runtime_flex_custom_properties_paint_cascade(
            &view,
            viewport,
            StyleRevision::INITIAL,
        )
        .unwrap(),
        compute_runtime_flex_registered_properties_cascade_with_stylesheets(
            &view,
            &[],
            viewport,
            StyleRevision::INITIAL,
        )
        .unwrap(),
        compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets(
            &view,
            &[],
            viewport,
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

    for (computed, expected_profile) in profiles.into_iter().zip(expected_profiles) {
        assert_eq!(computed.profile, expected_profile);
        let output = compute_runtime_style_layout(&snapshot, root, computed, viewport).unwrap();
        let root_style = output
            .computed_styles
            .elements
            .iter()
            .find(|style| style.node_id == root.id())
            .unwrap();
        assert_eq!(root_style.properties["order"], "-7", "{expected_profile:?}");
        assert_eq!(
            output.layout.frames[&children[0].id()].x,
            80.0,
            "{expected_profile:?}"
        );
        assert_eq!(
            output.layout.frames[&children[1].id()].x,
            0.0,
            "{expected_profile:?}"
        );
        assert_eq!(
            output.layout.frames[&children[2].id()].x,
            40.0,
            "{expected_profile:?}"
        );
        assert_eq!(
            snapshot.children(root).unwrap().collect::<Vec<_>>(),
            children,
            "CSS order cannot mutate HostDocument source order"
        );
    }
}

#[test]
fn custom_property_order_is_cascaded_and_invalid_adapter_values_fail_closed() {
    let custom_children = [
        "display:block;box-sizing:border-box;width:40px;height:20px;flex:0 0 auto;--item-order:2;order:var(--item-order)",
        "display:block;box-sizing:border-box;width:40px;height:20px;flex:0 0 auto;--item-order:-1;order:var(--item-order)",
        "display:block;box-sizing:border-box;width:40px;height:20px;flex:0 0 auto",
    ];
    let (document, root, children) = make_document(
        "display:flex;box-sizing:border-box;width:120px;height:20px;flex-direction:row;align-items:flex-start;justify-content:flex-start",
        &custom_children,
    );
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), root)
        .unwrap();
    let styles = compute_runtime_flex_custom_properties_paint_cascade(
        &view,
        viewport(),
        StyleRevision::INITIAL,
    )
    .unwrap();
    let output = compute_runtime_style_layout(&snapshot, root, styles, viewport()).unwrap();
    assert_eq!(output.layout.frames[&children[0].id()].x, 80.0);
    assert_eq!(output.layout.frames[&children[1].id()].x, 0.0);
    assert_eq!(output.layout.frames[&children[2].id()].x, 40.0);

    let (document, root, children) =
        make_document("display:flex;width:120px;height:20px", &CHILD_STYLES);
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), root)
        .unwrap();
    let mut computed =
        compute_runtime_flex_layout_cascade(&view, viewport(), StyleRevision::INITIAL).unwrap();
    let mut elements = computed.elements.to_vec();
    let item = elements
        .iter_mut()
        .find(|style| style.node_id == children[0].id())
        .unwrap();
    item.properties.insert("order".to_owned(), "1.5".to_owned());
    computed.elements = elements.into();
    assert!(matches!(
        compute_runtime_style_layout(&snapshot, root, computed, viewport()),
        Err(StyleLayoutError::UnsupportedComputedValue {
            node,
            property: "order",
            ..
        }) if node == children[0].id()
    ));
}

#[test]
fn changing_only_order_recomputes_layout_without_changing_document_child_order() {
    let (mut document, root, children) = make_document(ROOT_STYLE, &CHILD_STYLES);
    let viewport = viewport();
    let previous_snapshot = document.snapshot();
    let previous_view = StyloDocumentView::new_html_fragment_child_shared(
        Arc::new(previous_snapshot.clone()),
        root,
    )
    .unwrap();
    let previous =
        compute_runtime_flex_paint_cascade(&previous_view, viewport, StyleRevision::INITIAL)
            .unwrap();

    let mut change =
        DocumentChangeBatch::new(OwnerId::new(10_132).unwrap(), document.document_revision());
    change.push(DocumentOperation::SetAttribute {
        node: children[1],
        name: AttributeName::new(None, "style").unwrap(),
        value: "display:block;box-sizing:border-box;width:40px;height:20px;flex:0 0 auto;order:3"
            .to_owned()
            .into(),
    });
    document.commit(change).unwrap();

    let current_snapshot = document.snapshot();
    let current_view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::new(current_snapshot.clone()), root)
            .unwrap();
    let full = compute_runtime_flex_paint_cascade(&current_view, viewport, StyleRevision::INITIAL)
        .unwrap();
    let (incremental, _) = compute_runtime_incremental_cascade_with_stylesheets(
        &current_view,
        &[],
        viewport,
        StyleRevision::INITIAL,
        ComputedStyleProfile::RuntimeFlexPaintV1,
        &previous,
        &[children[1].id()],
    )
    .unwrap()
    .expect("inline order 변경은 안전한 incremental cascade를 허용해야 합니다");

    assert_eq!(incremental.elements, full.elements);
    let incremental_layout =
        compute_runtime_style_layout(&current_snapshot, root, incremental, viewport).unwrap();
    let full_layout =
        compute_runtime_style_layout(&current_snapshot, root, full, viewport).unwrap();
    assert_eq!(incremental_layout.layout.frames, full_layout.layout.frames);
    assert_eq!(incremental_layout.layout.frames[&children[0].id()].x, 40.0);
    assert_eq!(incremental_layout.layout.frames[&children[1].id()].x, 80.0);
    assert_eq!(incremental_layout.layout.frames[&children[2].id()].x, 0.0);
    assert_eq!(
        current_snapshot.children(root).unwrap().collect::<Vec<_>>(),
        children,
        "order 변경 뒤에도 HostDocument는 원래 자식 순서를 보존합니다"
    );
}

#[test]
fn stylo_integer_rounding_clamping_invalid_declarations_and_author_stylesheets_reach_taffy() {
    let calc_children = [
        "display:block;box-sizing:border-box;width:30px;height:20px;flex:0 0 auto;order:4;order:1.5",
        "display:block;box-sizing:border-box;width:30px;height:20px;flex:0 0 auto;order:1.5",
        "display:block;box-sizing:border-box;width:30px;height:20px;flex:0 0 auto;order:calc(1.5)",
        "display:block;box-sizing:border-box;width:30px;height:20px;flex:0 0 auto;order:calc(-1.5)",
    ];
    let (document, root, children) = make_document(
        "display:flex;box-sizing:border-box;width:160px;height:20px;flex-direction:row;align-items:flex-start;justify-content:flex-start",
        &calc_children,
    );
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), root)
        .unwrap();
    let styles =
        compute_runtime_flex_layout_cascade(&view, viewport(), StyleRevision::INITIAL).unwrap();
    let output = compute_runtime_style_layout(&snapshot, root, styles, viewport()).unwrap();
    let values = output
        .computed_styles
        .elements
        .iter()
        .filter(|element| element.node_id != root.id())
        .map(|element| element.properties["order"].as_str())
        .collect::<Vec<_>>();
    assert_eq!(values, ["4", "0", "2", "-1"]);
    assert_eq!(output.layout.frames[&children[0].id()].x, 90.0);
    assert_eq!(output.layout.frames[&children[1].id()].x, 30.0);
    assert_eq!(output.layout.frames[&children[2].id()].x, 60.0);
    assert_eq!(output.layout.frames[&children[3].id()].x, 0.0);

    let clamped_children = [
        "display:block;width:50px;height:20px;flex:0 0 auto;order:2147483649",
        "display:block;width:50px;height:20px;flex:0 0 auto;order:2147483648",
        "display:block;width:50px;height:20px;flex:0 0 auto;order:2147483647",
    ];
    let (document, root, children) = make_document(
        "display:flex;width:150px;height:20px;flex-direction:row;align-items:flex-start;justify-content:flex-start",
        &clamped_children,
    );
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), root)
        .unwrap();
    let styles =
        compute_runtime_flex_layout_cascade(&view, viewport(), StyleRevision::INITIAL).unwrap();
    let output = compute_runtime_style_layout(&snapshot, root, styles, viewport()).unwrap();
    let values = output
        .computed_styles
        .elements
        .iter()
        .filter(|element| element.node_id != root.id())
        .map(|element| element.properties["order"].as_str())
        .collect::<Vec<_>>();
    assert_eq!(values, ["2147483647", "2147483647", "2147483647"]);
    assert_eq!(output.layout.frames[&children[0].id()].x, 0.0);
    assert_eq!(output.layout.frames[&children[1].id()].x, 50.0);
    assert_eq!(output.layout.frames[&children[2].id()].x, 100.0);

    let stylesheet_children = [
        "display:block;width:40px;height:20px;flex:0 0 auto",
        "display:block;width:40px;height:20px;flex:0 0 auto",
        "display:block;width:40px;height:20px;flex:0 0 auto",
    ];
    let (document, root, children) = make_document(
        "display:flex;width:120px;height:20px;flex-direction:row;align-items:flex-start;justify-content:flex-start",
        &stylesheet_children,
    );
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), root)
        .unwrap();
    let stylesheet = spinon_style::StylesheetSource {
        id: "c1032-order-author".to_owned(),
        base_url: "https://spinon.invalid/c10/order.css".to_owned(),
        origin: spinon_style::CssOrigin::Author,
        css: "#c1032-item-0 { order: 2 } #c1032-item-1 { order: -1 } #c1032-item-2 { order: 0 }"
            .to_owned(),
    };
    let styles = compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets(
        &view,
        &[stylesheet],
        viewport(),
        StyleRevision::INITIAL,
    )
    .unwrap();
    let output = compute_runtime_style_layout(&snapshot, root, styles, viewport()).unwrap();
    assert_eq!(output.layout.frames[&children[0].id()].x, 80.0);
    assert_eq!(output.layout.frames[&children[1].id()].x, 0.0);
    assert_eq!(output.layout.frames[&children[2].id()].x, 40.0);
}
