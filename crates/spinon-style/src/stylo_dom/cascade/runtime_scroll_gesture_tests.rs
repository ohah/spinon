use std::{sync::Arc, thread};

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId,
};

use super::{
    ComputedCssOverflow, ComputedCssOverscrollBehavior, ComputedCssTouchAction,
    ComputedStyleProfile, CssMediaEnvironment, CssViewport, StyloDocumentView,
    compute_runtime_block_flex_overflow_clip_cascade_with_stylesheets,
    compute_runtime_block_flex_scroll_gesture_cascade_with_stylesheets,
    compute_runtime_block_positioning_cascade_with_stylesheets,
    first_unsupported_runtime_block_flex_overflow_clip_inline_property,
    first_unsupported_runtime_block_flex_scroll_gesture_inline_property,
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
        DocumentChangeBatch::new(OwnerId::new(6070).unwrap(), document.document_revision());
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
        id: "c133-scroll-gesture".to_owned(),
        base_url: "https://spinon.invalid/c13-3-scroll-gesture.css".to_owned(),
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
fn scroll_gesture_profile_extracts_typed_physical_axes_and_existing_overflow() {
    let fixture = fixture(
        "display:block;width:240px;height:120px;overflow-x:scroll;overflow-y:auto;overflow-clip-margin:border-box 4px;overscroll-behavior:contain none;touch-action:pan-x",
        "display:block;width:40px;height:30px;overflow:clip;overscroll-behavior-y:contain;touch-action:pan-y",
    );
    let snapshot = compute_runtime_block_flex_scroll_gesture_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();

    assert_eq!(
        snapshot.profile,
        ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
    );
    let root = style_for(&snapshot, fixture.root);
    assert_eq!(root.layout_overflow_x, Some(ComputedCssOverflow::Scroll));
    assert_eq!(root.layout_overflow_y, Some(ComputedCssOverflow::Auto));
    assert_eq!(root.overflow_clip_margin.unwrap().offset_css_px, 4.0);
    assert_eq!(
        root.input_overscroll_behavior_x,
        Some(ComputedCssOverscrollBehavior::Contain)
    );
    assert_eq!(
        root.input_overscroll_behavior_y,
        Some(ComputedCssOverscrollBehavior::None)
    );
    assert_eq!(root.input_touch_action, Some(ComputedCssTouchAction::PanX));
    assert_eq!(
        root.properties
            .get("overscroll-behavior-x")
            .map(String::as_str),
        Some("contain")
    );
    assert_eq!(
        root.properties.get("touch-action").map(String::as_str),
        Some("pan-x")
    );

    let child = style_for(&snapshot, fixture.child);
    assert_eq!(child.layout_overflow_x, Some(ComputedCssOverflow::Clip));
    assert_eq!(
        child.input_overscroll_behavior_x,
        Some(ComputedCssOverscrollBehavior::Auto)
    );
    assert_eq!(
        child.input_overscroll_behavior_y,
        Some(ComputedCssOverscrollBehavior::Contain)
    );
    assert_eq!(child.input_touch_action, Some(ComputedCssTouchAction::PanY));
}

#[test]
fn scroll_gesture_values_use_css_non_inherited_initial_values() {
    let fixture = fixture(
        "display:block;overscroll-behavior:contain none;touch-action:none",
        "display:block",
    );
    let snapshot = compute_runtime_block_flex_scroll_gesture_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();

    let root = style_for(&snapshot, fixture.root);
    assert_eq!(root.input_touch_action, Some(ComputedCssTouchAction::None));
    let child = style_for(&snapshot, fixture.child);
    assert_eq!(
        child.input_overscroll_behavior_x,
        Some(ComputedCssOverscrollBehavior::Auto)
    );
    assert_eq!(
        child.input_overscroll_behavior_y,
        Some(ComputedCssOverscrollBehavior::Auto)
    );
    assert_eq!(child.input_touch_action, Some(ComputedCssTouchAction::Auto));
}

#[test]
fn scroll_gesture_profile_applies_single_value_shorthand_then_axis_override() {
    let fixture = fixture(
        "display:block;overscroll-behavior:contain;overscroll-behavior-x:none",
        "display:block",
    );
    let snapshot = compute_runtime_block_flex_scroll_gesture_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let root = style_for(&snapshot, fixture.root);

    assert_eq!(
        root.input_overscroll_behavior_x,
        Some(ComputedCssOverscrollBehavior::None)
    );
    assert_eq!(
        root.input_overscroll_behavior_y,
        Some(ComputedCssOverscrollBehavior::Contain)
    );
}

#[test]
fn scroll_gesture_profile_marks_valid_but_unsupported_touch_action_values() {
    let fixture = fixture(
        "display:block;touch-action:manipulation",
        "display:block;touch-action:pan-x pan-y",
    );
    let snapshot = compute_runtime_block_flex_scroll_gesture_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();

    assert_eq!(
        style_for(&snapshot, fixture.root).input_touch_action,
        Some(ComputedCssTouchAction::Unsupported)
    );
    assert_eq!(
        style_for(&snapshot, fixture.child).input_touch_action,
        Some(ComputedCssTouchAction::Unsupported)
    );
}

#[test]
fn scroll_gesture_profile_rejects_directional_touch_action_that_stylo_drops() {
    for value in ["pan-left", "pan-right", "pan-up", "pan-down"] {
        let inline = fixture(&format!("display:block;touch-action:{value}"), "");
        assert!(matches!(
            compute_runtime_block_flex_scroll_gesture_cascade_with_stylesheets(
                &inline.view,
                &[],
                viewport(),
                Default::default(),
            ),
            Err(super::CssCascadeError::UnsupportedTouchActionValue {
                node: Some(node),
                value: rejected,
                ..
            }) if node == inline.root && rejected == value
        ));
    }

    let sheet = fixture("display:block", "");
    let error = compute_runtime_block_flex_scroll_gesture_cascade_with_stylesheets(
        &sheet.view,
        &[stylesheet(
            "@media (pointer: coarse) { #root { touch-action: pan-right; } }",
        )],
        viewport(),
        Default::default(),
    )
    .unwrap_err();
    assert!(matches!(
        error,
        super::CssCascadeError::UnsupportedTouchActionValue {
            node: None,
            value,
            ..
        } if value == "pan-right"
    ));

    let dynamic = fixture("display:block;touch-action:var(--pan)", "");
    assert!(matches!(
        compute_runtime_block_flex_scroll_gesture_cascade_with_stylesheets(
            &dynamic.view,
            &[],
            viewport(),
            Default::default(),
        ),
        Err(super::CssCascadeError::UnsupportedTouchActionValue { value, .. })
            if value == "var()"
    ));
}

#[test]
fn scroll_gesture_author_stylesheets_can_set_shorthand_and_longhands() {
    let fixture = fixture("display:block", "display:block");
    let snapshot = compute_runtime_block_flex_scroll_gesture_cascade_with_stylesheets(
        &fixture.view,
        &[stylesheet(
            "#root { overscroll-behavior: none contain; touch-action: none; } #child { overscroll-behavior-x: contain; touch-action: pan-y; }",
        )],
        viewport(),
        Default::default(),
    )
    .unwrap();

    let root = style_for(&snapshot, fixture.root);
    assert_eq!(
        root.input_overscroll_behavior_x,
        Some(ComputedCssOverscrollBehavior::None)
    );
    assert_eq!(
        root.input_overscroll_behavior_y,
        Some(ComputedCssOverscrollBehavior::Contain)
    );
    assert_eq!(root.input_touch_action, Some(ComputedCssTouchAction::None));
    let child = style_for(&snapshot, fixture.child);
    assert_eq!(
        child.input_overscroll_behavior_x,
        Some(ComputedCssOverscrollBehavior::Contain)
    );
    assert_eq!(child.input_touch_action, Some(ComputedCssTouchAction::PanY));
}

#[test]
fn scroll_gesture_profile_stays_physical_and_c131_allowlist_stays_unchanged() {
    let logical = fixture("display:block;overscroll-behavior-inline:contain", "");
    assert_eq!(
        first_unsupported_runtime_block_flex_scroll_gesture_inline_property(&logical.view),
        Some((logical.root, "overscroll-behavior-inline".to_owned()))
    );
    assert!(
        compute_runtime_block_flex_scroll_gesture_cascade_with_stylesheets(
            &logical.view,
            &[stylesheet("#root { overscroll-behavior-inline: contain; }")],
            viewport(),
            Default::default(),
        )
        .is_err()
    );

    let c131 = fixture("display:block;touch-action:pan-x", "");
    assert_eq!(
        first_unsupported_runtime_block_flex_overflow_clip_inline_property(&c131.view),
        Some((c131.root, "touch-action".to_owned()))
    );
    assert!(
        compute_runtime_block_flex_overflow_clip_cascade_with_stylesheets(
            &c131.view,
            &[stylesheet("#root { touch-action: pan-x; }")],
            viewport(),
            Default::default(),
        )
        .is_err()
    );
}

#[test]
fn scroll_gesture_preference_is_isolated_from_concurrent_legacy_style_parsing() {
    let scroll_profile = thread::spawn(|| {
        for _ in 0..8 {
            let fixture = fixture("display:block;overscroll-behavior:contain", "");
            let snapshot = compute_runtime_block_flex_scroll_gesture_cascade_with_stylesheets(
                &fixture.view,
                &[],
                viewport(),
                Default::default(),
            )
            .unwrap();
            assert_eq!(
                style_for(&snapshot, fixture.root).input_overscroll_behavior_x,
                Some(ComputedCssOverscrollBehavior::Contain)
            );
        }
    });
    let legacy_profile = thread::spawn(|| {
        for _ in 0..8 {
            let fixture = fixture(
                "display:block;position:fixed;left:0;top:0;width:10px;height:10px;offset-path:circle(20px)",
                "",
            );
            let snapshot = compute_runtime_block_positioning_cascade_with_stylesheets(
                &fixture.view,
                &[],
                CssViewport::C04_FIXTURE,
                Default::default(),
            )
            .unwrap();
            assert!(snapshot.diagnostics.iter().any(|diagnostic| {
                diagnostic.node_id == Some(fixture.root)
                    && diagnostic.diagnostic.message.contains("offset-path")
            }));
        }
    });

    scroll_profile.join().unwrap();
    legacy_profile.join().unwrap();
}
