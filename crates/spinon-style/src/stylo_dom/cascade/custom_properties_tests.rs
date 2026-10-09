use std::{fs, sync::Arc};

use serde_json::Value;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId,
};

use super::{
    CssViewport, StyloDocumentView, compute_runtime_flex_custom_properties_cascade,
    compute_runtime_flex_custom_properties_paint_cascade,
    first_unsupported_runtime_custom_properties_inline_property,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";

struct Fixture {
    view: StyloDocumentView,
    node_ids: Vec<spinon_core::NodeId>,
}

fn tree_fixture(nodes: &[(Option<usize>, &str)]) -> Fixture {
    let mut document = HostDocument::new().unwrap();
    let handles = nodes
        .iter()
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(3053).unwrap(), document.document_revision());
    for (index, (parent, style)) in nodes.iter().enumerate() {
        add_element(&mut batch, handles[index], style);
        batch.push(DocumentOperation::InsertBefore {
            parent: parent.map_or(HostParent::Root, |parent| HostParent::Node(handles[parent])),
            node: handles[index],
            before: None,
        });
    }
    document.commit(batch).unwrap();
    let view = StyloDocumentView::new_html_fragment_child_shared(
        Arc::new(document.snapshot()),
        handles[0],
    )
    .unwrap();
    Fixture {
        view,
        node_ids: handles.iter().map(|handle| handle.id()).collect(),
    }
}

fn fixture(root_style: &str, child_styles: &[&str]) -> Fixture {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let children = child_styles
        .iter()
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(3051).unwrap(), document.document_revision());
    add_element(&mut batch, root, root_style);
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Root,
        node: root,
        before: None,
    });
    for (handle, style) in children.iter().zip(child_styles) {
        add_element(&mut batch, *handle, style);
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: *handle,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    let view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::new(document.snapshot()), root)
            .unwrap();
    Fixture {
        view,
        node_ids: std::iter::once(root.id())
            .chain(children.iter().map(|child| child.id()))
            .collect(),
    }
}

fn add_element(batch: &mut DocumentChangeBatch, node: spinon_core::HostNodeHandle, style: &str) {
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

fn property<'a>(
    snapshot: &'a super::ComputedStyleSnapshot,
    node: spinon_core::NodeId,
    name: &str,
) -> &'a str {
    snapshot
        .elements
        .iter()
        .find(|style| style.node_id == node)
        .unwrap()
        .properties
        .get(name)
        .unwrap()
}

fn chromium_value<'a>(reference: &'a Value, node: &str, property: &str) -> &'a str {
    reference["observations"]["initial"][node][property]
        .as_str()
        .unwrap_or_else(|| panic!("Chromium reference에 {node}.{property} 값이 없습니다"))
}

#[test]
fn runtime_custom_property_values_match_pinned_chromium_reference() {
    let reference_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/fixtures/css/references/c05-runtime-custom-properties-v1.json"
    );
    let reference: Value =
        serde_json::from_str(&fs::read_to_string(reference_path).unwrap()).unwrap();
    assert_eq!(
        reference["schema"],
        "spinon-css-c05-runtime-custom-properties-reference/v1"
    );
    assert_eq!(reference["oracle"]["product"], "Chrome/154.0.8037.98");

    let fixture = tree_fixture(&[
        (None, "display:block;width:390px;height:844px"),
        (Some(0), "--measure:41px"),
        (Some(1), "width:var(--measure);height:8px"),
        (Some(0), "--Measure:17px"),
        (Some(3), "width:var(--measure,23px);height:8px"),
        (Some(0), "--space:13px"),
        (Some(5), "width:var(--missing,var(--space,29px));height:8px"),
        (Some(0), "--base:41px"),
        (Some(7), "--base:inherit;width:var(--base,23px);height:8px"),
        (Some(7), "--base:unset;width:var(--base,23px);height:8px"),
        (Some(7), "--base:initial;width:var(--base,23px);height:8px"),
        (Some(7), "--base:revert;width:var(--base,23px);height:8px"),
        (
            Some(0),
            "--self:var(--self);--left:var(--right);--right:var(--left);width:var(--self,37px);margin-left:var(--left,7px);height:8px",
        ),
        (Some(0), "--bad:red;margin-left:var(--bad);height:8px"),
        (Some(0), "--empty: ;margin-left:var(--empty,7px);height:8px"),
        (
            Some(0),
            "--rank:19px!important;--rank:43px;width:var(--rank);height:8px",
        ),
        (
            Some(0),
            "display:flex;box-sizing:border-box;width:301px;height:100px;--panel:#123456;background-color:var(--panel);--space:11px;gap:var(--space)",
        ),
        (
            Some(16),
            "width:20px;height:10px;--surface:#3366ff;background-color:var(--surface)",
        ),
        (
            Some(16),
            "width:30px;height:10px;--detail:#ffcc33;background-color:var(--detail)",
        ),
        (Some(0), "--size:41px"),
        (Some(19), "width:var(--size);height:8px"),
        (Some(0), "--size:91px"),
    ]);
    let snapshot = compute_runtime_flex_custom_properties_paint_cascade(
        &fixture.view,
        CssViewport {
            width_css_px: 390.0,
            height_css_px: 844.0,
            device_scale_factor: 1.0,
            environment_revision: Default::default(),
            media_environment: super::CssMediaEnvironment::MOBILE,
        },
        Default::default(),
    )
    .unwrap();

    for (node, index, name, css_property) in [
        ("inherit-child", 2, "width", "width"),
        ("case-child", 4, "width", "width"),
        ("fallback-child", 6, "width", "width"),
        ("keyword-inherit", 8, "width", "width"),
        ("keyword-unset", 9, "width", "width"),
        ("keyword-initial", 10, "width", "width"),
        ("keyword-revert", 11, "width", "width"),
        ("cycle", 12, "width", "width"),
        ("cycle", 12, "marginLeft", "margin-left"),
        ("invalid", 13, "marginLeft", "margin-left"),
        ("empty", 14, "marginLeft", "margin-left"),
        ("important", 15, "width", "width"),
        ("gap-parent", 16, "rowGap", "row-gap"),
        ("gap-parent", 16, "columnGap", "column-gap"),
        ("gap-parent", 16, "backgroundColor", "background-color"),
        ("gap-first", 17, "width", "width"),
        ("gap-first", 17, "backgroundColor", "background-color"),
        ("gap-second", 18, "width", "width"),
        ("gap-second", 18, "backgroundColor", "background-color"),
        ("moving-target", 20, "width", "width"),
    ] {
        assert_eq!(
            property(&snapshot, fixture.node_ids[index], css_property),
            chromium_value(&reference, node, name),
            "{node}.{css_property}"
        );
    }
}

#[test]
fn runtime_custom_properties_inherit_use_nested_fallbacks_and_preserve_name_case() {
    let fixture = fixture(
        "display:flex;width:300px;height:100px;--measure:41px;--space:13px;--Case:17px",
        &[
            "display:block;width:var(--measure);height:8px",
            "display:block;width:var(--missing,var(--space,29px));height:8px",
            "display:block;width:var(--case,23px);height:8px",
        ],
    );
    let snapshot = compute_runtime_flex_custom_properties_cascade(
        &fixture.view,
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    assert_eq!(property(&snapshot, fixture.node_ids[1], "width"), "41px");
    assert_eq!(property(&snapshot, fixture.node_ids[2], "width"), "13px");
    assert_eq!(property(&snapshot, fixture.node_ids[3], "width"), "23px");
    assert_eq!(
        snapshot.profile,
        super::ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
    );
}

#[test]
fn runtime_custom_properties_follow_cycle_empty_invalid_and_important_rules() {
    let fixture = fixture(
        "display:flex;width:300px;height:100px",
        &[
            "display:block;--self:var(--self);--left:var(--right);--right:var(--left);width:var(--self,37px);height:8px;margin-left:var(--left,7px)",
            "display:block;--bad:red;margin-left:var(--bad);height:8px",
            "display:block;--empty: ;margin-left:var(--empty,7px);height:8px",
            "display:block;--rank:19px!important;--rank:43px;width:var(--rank);height:8px",
        ],
    );
    let snapshot = compute_runtime_flex_custom_properties_cascade(
        &fixture.view,
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    assert_eq!(property(&snapshot, fixture.node_ids[1], "width"), "37px");
    assert_eq!(
        property(&snapshot, fixture.node_ids[1], "margin-left"),
        "7px"
    );
    assert_eq!(
        property(&snapshot, fixture.node_ids[2], "margin-left"),
        "0px"
    );
    assert_eq!(
        property(&snapshot, fixture.node_ids[3], "margin-left"),
        "0px"
    );
    assert_eq!(property(&snapshot, fixture.node_ids[4], "width"), "19px");
}

#[test]
fn runtime_custom_properties_feed_existing_shorthand_and_paint_profiles() {
    let fixture = fixture(
        "display:flex;width:300px;height:40px;--space:11px;gap:var(--space);background-color:#123456",
        &[
            "display:block;width:20px;height:10px",
            "display:block;width:30px;height:10px",
        ],
    );
    let snapshot = compute_runtime_flex_custom_properties_paint_cascade(
        &fixture.view,
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    assert_eq!(property(&snapshot, fixture.node_ids[0], "row-gap"), "11px");
    assert_eq!(
        property(&snapshot, fixture.node_ids[0], "column-gap"),
        "11px"
    );
    assert_eq!(
        property(&snapshot, fixture.node_ids[0], "background-color"),
        "rgb(18, 52, 86)"
    );
    assert_eq!(property(&snapshot, fixture.node_ids[2], "width"), "30px");
    assert_eq!(
        snapshot.profile,
        super::ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
    );
}

#[test]
fn runtime_custom_property_allowlist_does_not_allow_unrelated_css_properties() {
    let unsupported = fixture("display:block;--supported:12px;color:red", &[]);
    assert_eq!(
        first_unsupported_runtime_custom_properties_inline_property(&unsupported.view),
        Some((unsupported.node_ids[0], "color".to_owned()))
    );

    let supported = fixture("display:block;--supported:12px;width:var(--supported)", &[]);
    assert_eq!(
        first_unsupported_runtime_custom_properties_inline_property(&supported.view),
        None
    );

    let substituted_unsupported =
        fixture("display:block;--supported:red;color:var(--supported)", &[]);
    assert_eq!(
        first_unsupported_runtime_custom_properties_inline_property(&substituted_unsupported.view),
        Some((substituted_unsupported.node_ids[0], "color".to_owned()))
    );

    let layout_background = fixture(
        "display:block;--surface:#123456;background-color:var(--surface)",
        &[],
    );
    assert_eq!(
        first_unsupported_runtime_custom_properties_inline_property(&layout_background.view),
        Some((layout_background.node_ids[0], "background-color".to_owned()))
    );

    let paint_color = fixture(
        "display:block;--foreground:red;color:var(--foreground)",
        &[],
    );
    assert_eq!(
        super::first_unsupported_runtime_custom_properties_paint_inline_property(&paint_color.view),
        Some((paint_color.node_ids[0], "color".to_owned()))
    );
}
