use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId,
    StyleRevision,
};
use spinon_style::{
    ComputedCssPosition, CssOrigin, CssViewport, FixedContainingBlockEffect, StylesheetSource,
    StyloDocumentView, compute_runtime_block_positioning_cascade_with_stylesheets,
};

use crate::{StyleLayoutError, compute_runtime_style_layout};

const HTML: &str = "http://www.w3.org/1999/xhtml";

fn fixture() -> (HostDocument, Vec<spinon_core::HostNodeHandle>) {
    let mut document = HostDocument::new().unwrap();
    let handles = (1..=5)
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let [root, wrapper, fixed_parent, absolute_child, fixed_child] = handles.as_slice() else {
        unreachable!()
    };
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(12_301).unwrap(), document.document_revision());
    for (handle, id, parent, style) in [
        (
            *root,
            "c123-root",
            HostParent::Root,
            "display:block;position:static;box-sizing:border-box;width:100vw;height:100vh;margin:0;padding:0",
        ),
        (
            *wrapper,
            "c123-wrapper",
            HostParent::Node(*root),
            "display:block;position:relative;left:28px;top:19px;box-sizing:border-box;width:40px;height:30px;margin:0;padding:0",
        ),
        (
            *fixed_parent,
            "c123-fixed-parent",
            HostParent::Node(*wrapper),
            "display:block;position:fixed;left:20px;top:10px;box-sizing:border-box;width:40px;height:30px;margin:0;padding:0",
        ),
        (
            *absolute_child,
            "c123-absolute-child",
            HostParent::Node(*fixed_parent),
            "display:block;position:absolute;left:3px;top:4px;box-sizing:border-box;width:8px;height:6px;margin:0;padding:0",
        ),
        (
            *fixed_child,
            "c123-fixed-child",
            HostParent::Node(*fixed_parent),
            "display:block;position:fixed;left:60px;top:50px;box-sizing:border-box;width:7px;height:5px;margin:0;padding:0",
        ),
    ] {
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
        batch.push(DocumentOperation::SetAttribute {
            node: handle,
            name: AttributeName::new(None, "style").unwrap(),
            value: style.to_owned().into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node: handle,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    (document, handles)
}

fn viewport() -> CssViewport {
    CssViewport {
        width_css_px: 100.0,
        height_css_px: 100.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssViewport::C04_FIXTURE.media_environment,
    }
}

#[test]
fn fixed_layout_uses_viewport_and_keeps_absolute_and_fixed_owners_separate() {
    let (document, handles) = fixture();
    let [root, _, fixed_parent, absolute_child, fixed_child] = handles.as_slice() else {
        unreachable!()
    };
    let snapshot = Arc::new(document.snapshot());
    let view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::clone(&snapshot), *root).unwrap();
    let viewport = viewport();
    let computed = compute_runtime_block_positioning_cascade_with_stylesheets(
        &view,
        &[],
        viewport,
        StyleRevision::INITIAL,
    )
    .unwrap();
    assert_eq!(
        computed
            .elements
            .iter()
            .find(|element| element.node_id == fixed_parent.id())
            .unwrap()
            .layout_position,
        ComputedCssPosition::Fixed
    );

    let output = compute_runtime_style_layout(&snapshot, *root, computed, viewport).unwrap();
    assert_eq!(
        (
            output.layout.frames[&fixed_parent.id()].x,
            output.layout.frames[&fixed_parent.id()].y,
        ),
        (20.0, 10.0)
    );
    assert_eq!(
        (
            output.layout.frames[&absolute_child.id()].x,
            output.layout.frames[&absolute_child.id()].y,
        ),
        (23.0, 14.0)
    );
    assert_eq!(
        (
            output.layout.frames[&fixed_child.id()].x,
            output.layout.frames[&fixed_child.id()].y,
        ),
        (60.0, 50.0)
    );
    assert_eq!(
        output.layout.positioned_owners[&absolute_child.id()],
        spinon_layout::PositionedContainingBlockOwner::Node(fixed_parent.id())
    );
    assert_eq!(
        output.layout.fixed_owners[&fixed_parent.id()],
        spinon_layout::FixedContainingBlockOwner::Viewport
    );
    assert_eq!(
        output.layout.fixed_owners[&fixed_child.id()],
        spinon_layout::FixedContainingBlockOwner::Viewport
    );
}

#[test]
fn fixed_auto_height_uses_the_c07_3_aspect_ratio_in_the_viewport_profile() {
    let (mut document, handles) = fixture();
    let [root, _, fixed_parent, _, _] = handles.as_slice() else {
        unreachable!()
    };
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(12_301).unwrap(), document.document_revision());
    batch.push(DocumentOperation::SetAttribute {
        node: *fixed_parent,
        name: AttributeName::new(None, "style").unwrap(),
        value: "display:block;position:fixed;left:50px;top:60px;box-sizing:border-box;width:40px;height:auto;aspect-ratio:2 / 1;margin:0;padding:0".into(),
    });
    document.commit(batch).unwrap();
    let snapshot = Arc::new(document.snapshot());
    let view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::clone(&snapshot), *root).unwrap();
    let viewport = viewport();
    let computed = compute_runtime_block_positioning_cascade_with_stylesheets(
        &view,
        &[],
        viewport,
        StyleRevision::INITIAL,
    )
    .unwrap();
    let output = compute_runtime_style_layout(&snapshot, *root, computed, viewport).unwrap();
    let frame = output.layout.frames[&fixed_parent.id()];
    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        (50.0, 60.0, 40.0, 20.0)
    );
}

#[test]
fn fixed_ancestor_effect_snapshot_fails_closed_after_author_preflight_boundary() {
    let (document, handles) = fixture();
    let [root, wrapper, fixed_parent, _, _] = handles.as_slice() else {
        unreachable!()
    };
    let snapshot = Arc::new(document.snapshot());
    let view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::clone(&snapshot), *root).unwrap();
    let viewport = viewport();
    let mut computed = compute_runtime_block_positioning_cascade_with_stylesheets(
        &view,
        &[],
        viewport,
        StyleRevision::INITIAL,
    )
    .unwrap();
    Arc::make_mut(&mut computed.elements)
        .iter_mut()
        .find(|element| element.node_id == wrapper.id())
        .unwrap()
        .fixed_containing_block_effects = vec![FixedContainingBlockEffect::Transform];

    assert!(matches!(
        compute_runtime_style_layout(&snapshot, *root, computed, viewport),
        Err(StyleLayoutError::UnsupportedFixedContainingBlockEffect {
            node,
            ancestor,
            effect: FixedContainingBlockEffect::Transform,
        }) if node == fixed_parent.id() && ancestor == wrapper.id()
    ));
}

#[test]
fn stylo_unknown_fixed_effect_diagnostics_fail_before_layout() {
    let viewport = viewport();
    for (index, declaration) in [
        "offset-path:circle(20px)",
        "content-visibility:auto",
        "backdrop-filter:blur(1px)",
        "contain:layout",
        "contain:paint",
        "contain:content",
    ]
    .into_iter()
    .enumerate()
    {
        let property = declaration.split(':').next().unwrap();
        let source_id = format!("c12-3-unknown-fixed-effect-{index}");
        let (document, handles) = fixture();
        let [root, _, _, _, _] = handles.as_slice() else {
            unreachable!()
        };
        let snapshot = Arc::new(document.snapshot());
        let view = StyloDocumentView::new_html_fragment_child_shared(Arc::clone(&snapshot), *root)
            .unwrap();
        let stylesheet = StylesheetSource {
            id: source_id.clone(),
            base_url: "https://spinon.invalid/c12-3-unknown-effect.css".to_owned(),
            origin: CssOrigin::Author,
            css: format!(".c123-wrapper {{{declaration}}}"),
        };
        let computed = compute_runtime_block_positioning_cascade_with_stylesheets(
            &view,
            &[stylesheet],
            viewport,
            StyleRevision::INITIAL,
        )
        .unwrap();
        assert!(computed.diagnostics.iter().any(|diagnostic| {
            diagnostic.source_id == source_id && diagnostic.diagnostic.message.contains(property)
        }));
        let layout_result = compute_runtime_style_layout(&snapshot, *root, computed, viewport);
        assert!(
            matches!(layout_result, Err(StyleLayoutError::CascadeDiagnostic(_))),
            "stylesheet diagnostic escaped the layout boundary for {declaration}: {layout_result:?}"
        );

        let (mut document, handles) = fixture();
        let [root, wrapper, _, _, _] = handles.as_slice() else {
            unreachable!()
        };
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(12_301).unwrap(), document.document_revision());
        batch.push(DocumentOperation::SetAttribute {
            node: *wrapper,
            name: AttributeName::new(None, "style").unwrap(),
            value: format!("display:block;position:relative;left:28px;top:19px;box-sizing:border-box;width:40px;height:30px;margin:0;padding:0;{declaration}").into(),
        });
        document.commit(batch).unwrap();
        let snapshot = Arc::new(document.snapshot());
        let view = StyloDocumentView::new_html_fragment_child_shared(Arc::clone(&snapshot), *root)
            .unwrap();
        let computed = compute_runtime_block_positioning_cascade_with_stylesheets(
            &view,
            &[],
            viewport,
            StyleRevision::INITIAL,
        )
        .unwrap();
        assert!(computed.diagnostics.iter().any(|diagnostic| {
            diagnostic.node_id == Some(wrapper.id())
                && diagnostic.diagnostic.message.contains(property)
        }));
        assert!(matches!(
            compute_runtime_style_layout(&snapshot, *root, computed, viewport),
            Err(StyleLayoutError::CascadeDiagnostic(_))
        ));
    }
}
