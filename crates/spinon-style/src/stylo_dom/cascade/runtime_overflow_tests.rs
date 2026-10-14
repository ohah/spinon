use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId,
};

use super::{
    ComputedCssOverflow, ComputedCssOverflowVisualBox, ComputedStyleProfile, CssMediaEnvironment,
    CssViewport, StyloDocumentView,
    compute_runtime_block_flex_overflow_clip_cascade_with_stylesheets,
    first_unsupported_runtime_block_flex_overflow_clip_inline_property,
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
        DocumentChangeBatch::new(OwnerId::new(6060).unwrap(), document.document_revision());
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
        id: "c131-overflow".to_owned(),
        base_url: "https://spinon.invalid/c13-1-overflow.css".to_owned(),
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
fn overflow_profile_keeps_computed_axes_and_clip_margin_typed() {
    let fixture = fixture(
        "display:block;width:240px;height:120px;overflow-x:clip;overflow-y:visible;overflow-clip-margin:content-box 8px",
        "display:block;width:40px;height:30px;overflow:clip;overflow-clip-margin:border-box 3px",
    );
    let snapshot = compute_runtime_block_flex_overflow_clip_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();

    assert_eq!(
        snapshot.profile,
        ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
    );
    let root = style_for(&snapshot, fixture.root);
    assert_eq!(root.layout_overflow_x, Some(ComputedCssOverflow::Clip));
    assert_eq!(root.layout_overflow_y, Some(ComputedCssOverflow::Visible));
    assert_eq!(
        root.overflow_clip_margin.unwrap().visual_box,
        ComputedCssOverflowVisualBox::ContentBox
    );
    assert_eq!(root.overflow_clip_margin.unwrap().offset_css_px, 8.0);

    let child = style_for(&snapshot, fixture.child);
    assert_eq!(child.layout_overflow_x, Some(ComputedCssOverflow::Clip));
    assert_eq!(child.layout_overflow_y, Some(ComputedCssOverflow::Clip));
    assert_eq!(
        child.overflow_clip_margin.unwrap().visual_box,
        ComputedCssOverflowVisualBox::BorderBox
    );
    assert_eq!(child.overflow_clip_margin.unwrap().offset_css_px, 3.0);
}

#[test]
fn overflow_profile_records_mixed_axis_computed_normalization() {
    let fixture = fixture(
        "display:block;width:240px;height:120px;overflow-x:visible;overflow-y:hidden",
        "display:block;width:40px;height:30px;overflow-x:clip;overflow-y:auto",
    );
    let snapshot = compute_runtime_block_flex_overflow_clip_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();

    assert_eq!(
        style_for(&snapshot, fixture.root).layout_overflow_x,
        Some(ComputedCssOverflow::Auto)
    );
    assert_eq!(
        style_for(&snapshot, fixture.root).layout_overflow_y,
        Some(ComputedCssOverflow::Hidden)
    );
    assert_eq!(
        style_for(&snapshot, fixture.child).layout_overflow_x,
        Some(ComputedCssOverflow::Hidden)
    );
    assert_eq!(
        style_for(&snapshot, fixture.child).layout_overflow_y,
        Some(ComputedCssOverflow::Auto)
    );
}

#[test]
fn overflow_profile_limits_inline_css_and_author_stylesheet_features() {
    let supported = fixture(
        "display:block;overflow:clip;overflow-clip-margin:padding-box 4px;--theme:#123456",
        "display:block;overflow-x:visible;overflow-y:clip",
    );
    assert_eq!(
        first_unsupported_runtime_block_flex_overflow_clip_inline_property(&supported.view),
        None
    );

    let logical = fixture("display:block;overflow-block:clip", "");
    assert_eq!(
        first_unsupported_runtime_block_flex_overflow_clip_inline_property(&logical.view),
        Some((logical.root, "overflow-block".to_owned()))
    );

    let no_styles = fixture("display:block", "");
    for css in [
        "#root { overflow: clip; transform: translateX(1px); }",
        "@media screen { #root { overflow: clip; } }",
        "#root { overflow-block: clip; }",
    ] {
        assert!(
            compute_runtime_block_flex_overflow_clip_cascade_with_stylesheets(
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
