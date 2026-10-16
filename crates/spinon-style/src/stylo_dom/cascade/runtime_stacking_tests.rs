use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId,
};

use super::{
    ComputedCssVisibility, ComputedCssZIndex, ComputedStyleProfile, CssMediaEnvironment,
    CssViewport, StyloDocumentView, compute_runtime_block_flex_stacking_cascade_with_stylesheets,
    first_unsupported_runtime_block_flex_stacking_inline_property,
};
use crate::{CssOrigin, StylesheetSource};

const HTML: &str = "http://www.w3.org/1999/xhtml";

struct Fixture {
    view: StyloDocumentView,
    root: spinon_core::NodeId,
    child: spinon_core::NodeId,
}

fn fixture(root_style: &str, child_style: &str) -> Fixture {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let child = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(6058).unwrap(), document.document_revision());
    for (node, id, style) in [(root, "root", root_style), (child, "child", child_style)] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "id").unwrap(),
            value: id.to_owned().into(),
        });
        if !style.is_empty() {
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "style").unwrap(),
                value: style.to_owned().into(),
            });
        }
        batch.push(DocumentOperation::InsertBefore {
            parent: if node == root {
                HostParent::Root
            } else {
                HostParent::Node(root)
            },
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    let view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::new(document.snapshot()), root)
            .unwrap();
    Fixture {
        view,
        root: root.id(),
        child: child.id(),
    }
}

fn viewport() -> CssViewport {
    CssViewport {
        width_css_px: 300.0,
        height_css_px: 200.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    }
}

fn stylesheet(css: &str) -> StylesheetSource {
    StylesheetSource {
        id: "c124-stacking".to_owned(),
        base_url: "https://spinon.invalid/c124-stacking.css".to_owned(),
        origin: CssOrigin::Author,
        css: css.to_owned(),
    }
}

fn style_for(
    snapshot: &super::ComputedStyleSnapshot,
    node: spinon_core::NodeId,
) -> &super::ComputedElementStyle {
    snapshot
        .elements
        .iter()
        .find(|style| style.node_id == node)
        .unwrap()
}

#[test]
fn stacking_profile_cascades_layer_registered_z_index_and_visibility_as_typed_values() {
    let fixture = fixture(
        "display:flex;box-sizing:border-box;width:240px;height:120px",
        "display:block;box-sizing:border-box;width:40px;height:30px;visibility:visible;background-color:#3366ff",
    );
    let css = stylesheet(
        r#"
        @property --stack-level { syntax: "<integer>"; inherits: false; initial-value: 2; }
        @layer base, theme;
        @layer base {
          #child { --stack-level: 7; }
        }
        @layer theme {
          #child { z-index: var(--stack-level); }
        }
        "#,
    );
    let snapshot = compute_runtime_block_flex_stacking_cascade_with_stylesheets(
        &fixture.view,
        &[css],
        viewport(),
        Default::default(),
    )
    .unwrap();

    assert_eq!(
        snapshot.profile,
        ComputedStyleProfile::RuntimeBlockFlexStackingV1
    );
    assert_eq!(
        style_for(&snapshot, fixture.child).stacking_z_index,
        Some(ComputedCssZIndex::Integer(7))
    );
    assert_eq!(
        style_for(&snapshot, fixture.child).stacking_visibility,
        Some(ComputedCssVisibility::Visible)
    );
    assert_eq!(
        style_for(&snapshot, fixture.child)
            .stacking_opacity
            .map(|opacity| opacity.value()),
        Some(1.0)
    );
}

#[test]
fn stacking_profile_preserves_computed_opacity_as_typed_values() {
    let fixture = fixture(
        "display:block;box-sizing:border-box;width:240px;height:120px;opacity:0.5",
        "display:block;box-sizing:border-box;width:40px;height:30px;opacity:0.25",
    );
    let snapshot = compute_runtime_block_flex_stacking_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();

    assert_eq!(
        style_for(&snapshot, fixture.root)
            .stacking_opacity
            .map(|opacity| opacity.value()),
        Some(0.5)
    );
    assert_eq!(
        style_for(&snapshot, fixture.child)
            .stacking_opacity
            .map(|opacity| opacity.value()),
        Some(0.25)
    );
    assert_eq!(
        first_unsupported_runtime_block_flex_stacking_inline_property(&fixture.view),
        None
    );
}

#[test]
fn stacking_profile_preserves_inherited_hidden_visibility_for_renderer_gate() {
    let fixture = fixture(
        "display:block;box-sizing:border-box;width:240px;height:120px;visibility:hidden",
        "display:block;box-sizing:border-box;width:40px;height:30px",
    );
    let snapshot = compute_runtime_block_flex_stacking_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();

    assert_eq!(
        style_for(&snapshot, fixture.root).stacking_visibility,
        Some(ComputedCssVisibility::Hidden)
    );
    assert_eq!(
        style_for(&snapshot, fixture.child).stacking_visibility,
        Some(ComputedCssVisibility::Hidden)
    );
    assert_eq!(
        style_for(&snapshot, fixture.child).stacking_z_index,
        Some(ComputedCssZIndex::Auto)
    );
}

#[test]
fn stacking_profile_accepts_only_declared_inline_values_and_rejects_other_at_rules() {
    let supported = fixture(
        "display:flex;box-sizing:border-box;width:240px;height:120px;--layer:1",
        "display:block;box-sizing:border-box;width:40px;height:30px;z-index:var(--layer);visibility:visible",
    );
    assert_eq!(
        first_unsupported_runtime_block_flex_stacking_inline_property(&supported.view),
        None
    );

    let unsupported_inline = fixture("display:block;transform:translateX(1px)", "");
    assert_eq!(
        first_unsupported_runtime_block_flex_stacking_inline_property(&unsupported_inline.view),
        Some((unsupported_inline.root, "transform".to_owned()))
    );

    let no_styles = fixture("display:block", "");
    for css in [
        "@media screen { #root { z-index: 1; } }",
        "@supports (display: grid) { #root { display: block; } }",
        "#root { & .child { z-index: 1; } }",
        "@unknown-rule { #root { z-index: 1; } }",
    ] {
        assert!(
            compute_runtime_block_flex_stacking_cascade_with_stylesheets(
                &no_styles.view,
                &[stylesheet(css)],
                viewport(),
                Default::default(),
            )
            .is_err(),
            "CSS must fail closed: {css}"
        );
    }
}
