use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId,
};

use super::{
    ComputedStyleProfile, CssMediaEnvironment, CssViewport, StyloDocumentView,
    compute_runtime_flex_custom_properties_cascade_with_stylesheets,
    compute_runtime_flex_registered_properties_cascade_with_stylesheets,
    compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets,
};
use crate::{ComputedBackgroundPaint, CssOrigin, OpaqueCssSrgb, StylesheetSource};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const CSS_ONE: &str = r#"
@property --tile-width { syntax: "<length>"; inherits: false; initial-value: 23px; }
@property --tone { syntax: "<length>"; inherits: true; initial-value: 7px; }
@property --unknown-descriptor {
  syntax: "<length>"; inherits: false; initial-value: 13px; vendor-extension: ignored;
}
#app { display:flex; box-sizing:border-box; width:301px; height:100px; align-items:flex-start; gap:5px; }
.tile { display:block; box-sizing:border-box; height:14px; }
#declared { --tile-width:41px; width:var(--tile-width); }
#default { width:var(--tile-width); }
#invalid { --tile-width:red; width:var(--tile-width); }
#inherit-parent { display:flex; flex-direction:column; width:60px; height:30px; gap:2px; --tile-width:71px; --tone:19px; }
#not-inherited { width:var(--tile-width); }
#inherited { width:var(--tone); }
#late { width:var(--late-width); }
@property --late-width { syntax: "<length>"; inherits: false; initial-value: 31px; }
#unknown { width:var(--unknown-descriptor); }
#important { --tile-width:39px; width:var(--tile-width); }
"#;
const CSS_TWO: &str = r#"
@property --duplicate { syntax:"<number>"; inherits:false; initial-value:3; }
#duplicate { --duplicate:29px; width:var(--duplicate); }
"#;

struct Fixture {
    view: StyloDocumentView,
    ids: Vec<spinon_core::NodeId>,
}

fn fixture() -> Fixture {
    let nodes = [
        (None, "app", "", ""),
        (Some(0), "declared", "tile", ""),
        (Some(0), "default", "tile", ""),
        (Some(0), "invalid", "tile", ""),
        (Some(0), "inherit-parent", "", ""),
        (Some(4), "not-inherited", "tile", ""),
        (Some(4), "inherited", "tile", ""),
        (Some(0), "late", "tile", ""),
        (Some(0), "unknown", "tile", ""),
        (Some(0), "important", "tile", "--tile-width:47px !important"),
        (Some(0), "duplicate", "tile", ""),
    ];
    let mut document = HostDocument::new().unwrap();
    let handles = nodes
        .iter()
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(3054).unwrap(), document.document_revision());
    for (index, (parent, id, class, inline_style)) in nodes.iter().enumerate() {
        batch.push(DocumentOperation::CreateElement {
            node: handles[index],
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: handles[index],
            name: AttributeName::new(None, "id").unwrap(),
            value: (*id).to_owned().into(),
        });
        if !class.is_empty() {
            batch.push(DocumentOperation::SetAttribute {
                node: handles[index],
                name: AttributeName::new(None, "class").unwrap(),
                value: (*class).to_owned().into(),
            });
        }
        if !inline_style.is_empty() {
            batch.push(DocumentOperation::SetAttribute {
                node: handles[index],
                name: AttributeName::new(None, "style").unwrap(),
                value: (*inline_style).to_owned().into(),
            });
        }
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
        ids: handles.iter().map(|handle| handle.id()).collect(),
    }
}

fn stylesheet(id: &str, css: &str) -> StylesheetSource {
    StylesheetSource {
        id: id.to_owned(),
        base_url: format!("https://spinon.invalid/{id}.css"),
        origin: CssOrigin::Author,
        css: css.to_owned(),
    }
}

fn property<'a>(
    snapshot: &'a super::ComputedStyleSnapshot,
    node: spinon_core::NodeId,
    name: &str,
) -> &'a str {
    snapshot
        .elements
        .iter()
        .find(|element| element.node_id == node)
        .unwrap()
        .properties
        .get(name)
        .unwrap()
}

fn viewport() -> CssViewport {
    CssViewport {
        width_css_px: 301.0,
        height_css_px: 100.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    }
}

#[test]
fn registered_properties_follow_syntax_initial_inheritance_and_source_order() {
    let fixture = fixture();
    let stylesheets = [
        stylesheet("sheet-one", CSS_ONE),
        stylesheet("sheet-two", CSS_TWO),
    ];
    let snapshot = compute_runtime_flex_registered_properties_cascade_with_stylesheets(
        &fixture.view,
        &stylesheets,
        viewport(),
        Default::default(),
    )
    .unwrap();

    assert_eq!(
        snapshot.profile,
        ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
    );
    assert_eq!(property(&snapshot, fixture.ids[1], "width"), "41px");
    assert_eq!(property(&snapshot, fixture.ids[2], "width"), "23px");
    assert_eq!(property(&snapshot, fixture.ids[3], "width"), "23px");
    assert_eq!(property(&snapshot, fixture.ids[5], "width"), "23px");
    assert_eq!(property(&snapshot, fixture.ids[6], "width"), "19px");
    assert_eq!(property(&snapshot, fixture.ids[7], "width"), "31px");
    assert_eq!(property(&snapshot, fixture.ids[8], "width"), "13px");
    assert_eq!(property(&snapshot, fixture.ids[9], "width"), "47px");
    assert!(snapshot.diagnostics.is_empty());
}

#[test]
fn registered_property_paint_profile_resolves_typed_color_initial_and_override() {
    let css = format!(
        "@property --tone-color {{ syntax: \"<color>\"; inherits: false; initial-value: rgb(51, 102, 255); }}\n{}\n.tile {{ background-color:var(--tone-color); width:11px; }}\n#color-override {{ --tone-color:#ff6600; }}",
        CSS_ONE
    );
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let children = ["blue", "orange"]
        .iter()
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(3055).unwrap(), document.document_revision());
    for (handle, id) in std::iter::once((root, "app"))
        .chain(children.iter().copied().zip(["blue", "color-override"]))
    {
        batch.push(DocumentOperation::CreateElement {
            node: handle,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: handle,
            name: AttributeName::new(None, "id").unwrap(),
            value: id.to_owned().into(),
        });
        if handle != root {
            batch.push(DocumentOperation::SetAttribute {
                node: handle,
                name: AttributeName::new(None, "class").unwrap(),
                value: "tile".into(),
            });
        }
        batch.push(DocumentOperation::InsertBefore {
            parent: if handle == root {
                HostParent::Root
            } else {
                HostParent::Node(root)
            },
            node: handle,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    let view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::new(document.snapshot()), root)
            .unwrap();
    let snapshot = compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets(
        &view,
        &[stylesheet("paint", &css)],
        viewport(),
        Default::default(),
    )
    .unwrap();

    assert_eq!(
        snapshot.profile,
        ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
    );
    assert_eq!(
        snapshot
            .elements
            .iter()
            .find(|element| element.node_id == children[0].id())
            .unwrap()
            .background_paint,
        Some(ComputedBackgroundPaint::Opaque(OpaqueCssSrgb {
            red: 51,
            green: 102,
            blue: 255,
        }))
    );
    assert_eq!(
        snapshot
            .elements
            .iter()
            .find(|element| element.node_id == children[1].id())
            .unwrap()
            .background_paint,
        Some(ComputedBackgroundPaint::Opaque(OpaqueCssSrgb {
            red: 255,
            green: 102,
            blue: 0,
        }))
    );
}

#[test]
fn registered_property_profile_keeps_legacy_and_general_css_boundaries() {
    let fixture = fixture();
    let registration = stylesheet(
        "registered",
        "@property --size { syntax: \"<length>\"; inherits: false; initial-value: 12px; } #app { width:var(--size); }",
    );
    assert!(
        compute_runtime_flex_custom_properties_cascade_with_stylesheets(
            &fixture.view,
            std::slice::from_ref(&registration),
            viewport(),
            Default::default(),
        )
        .is_err()
    );

    for css in [
        "@property --size { syntax: \"<length>\"; inherits: false; }",
        "@property --size { syntax: \"<length>\"; inherits: false; initial-value:red; }",
        "@media (min-width: 1px) { @property --size { syntax: \"<length>\"; inherits: false; initial-value: 12px; } }",
        "#app { color: red; }",
        "#app { background-color: red; }",
    ] {
        assert!(
            compute_runtime_flex_registered_properties_cascade_with_stylesheets(
                &fixture.view,
                &[stylesheet("invalid", css)],
                viewport(),
                Default::default(),
            )
            .is_err(),
            "CSS must be rejected: {css}"
        );
    }
}
