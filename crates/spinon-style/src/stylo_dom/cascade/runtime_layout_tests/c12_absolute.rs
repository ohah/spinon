use crate::{CssOrigin, StylesheetSource};

use super::super::{
    ComputedCssPosition, ComputedStyleProfile, CssCascadeError, CssViewport,
    compute_runtime_block_positioning_cascade_with_stylesheets,
    first_unsupported_runtime_block_formatting_inline_property,
};

#[test]
fn block_positioning_profile_keeps_absolute_and_blockified_inline_values_typed() {
    let (view, node) = super::view_for_inline_style(
        "display:inline;position:absolute;left:calc(10% + 4px);top:7px;width:30px;height:12px",
    );
    assert_eq!(
        first_unsupported_runtime_block_formatting_inline_property(&view),
        None
    );

    let snapshot = compute_runtime_block_positioning_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();

    assert_eq!(
        snapshot.profile,
        ComputedStyleProfile::RuntimeBlockPositioningV1
    );
    let style = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == node)
        .unwrap();
    assert_eq!(style.layout_position, ComputedCssPosition::Absolute);
    assert_eq!(
        style.properties.get("display").map(String::as_str),
        Some("block")
    );
    assert_eq!(
        style.properties.get("position").map(String::as_str),
        Some("absolute")
    );
    assert!(style.layout_math_values.contains_key("left"));
}

#[test]
fn block_positioning_author_profile_accepts_layers_vars_and_border_resets_only() {
    let (view, node) = super::view_for_inline_style(
        "display:inline;width:30px;height:12px;background-color:#3366ff",
    );
    let stylesheet = StylesheetSource {
        id: "c12-positioning-layers".to_owned(),
        base_url: "https://spinon.invalid/c12-positioning.css".to_owned(),
        origin: CssOrigin::Author,
        css: "@layer base, positioned; @layer base { div { border: 2px solid red; --offset: 12px; aspect-ratio: 2 / 1; } } @layer positioned { div { position: absolute; left: var(--offset); top: 7px; } }".to_owned(),
    };
    let snapshot = compute_runtime_block_positioning_cascade_with_stylesheets(
        &view,
        &[stylesheet],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    let style = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == node)
        .unwrap();
    assert_eq!(style.properties["position"], "absolute");
    assert_eq!(style.properties["left"], "12px");
    assert_eq!(style.layout_aspect_ratio, Some(2.0));

    let explicit_border_image = StylesheetSource {
        id: "c12-unsupported-border-image".to_owned(),
        base_url: "https://spinon.invalid/c12-unsupported.css".to_owned(),
        origin: CssOrigin::Author,
        css: "div { border-image-outset: 2px; }".to_owned(),
    };
    assert!(matches!(
        compute_runtime_block_positioning_cascade_with_stylesheets(
            &view,
            &[explicit_border_image],
            CssViewport::C04_FIXTURE,
            Default::default(),
        ),
        Err(CssCascadeError::UnsupportedAuthorCss { .. })
    ));
}
