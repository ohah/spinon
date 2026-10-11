use crate::{CssOrigin, StylesheetSource};

use super::super::{
    ComputedCssPosition, CssCascadeError, CssViewport,
    compute_runtime_block_positioning_cascade_with_stylesheets,
    first_unsupported_runtime_block_formatting_inline_property,
    first_unsupported_runtime_block_positioning_inline_property,
};

#[test]
fn block_positioning_profile_preserves_fixed_position_and_empty_effects() {
    let (view, node) = super::view_for_inline_style(
        "display:block;position:fixed;left:10px;top:20px;width:30px;height:12px",
    );
    let snapshot = compute_runtime_block_positioning_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    let style = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == node)
        .unwrap();
    assert_eq!(style.layout_position, ComputedCssPosition::Fixed);
    assert!(style.fixed_containing_block_effects.is_empty());
}

#[test]
fn block_positioning_inline_allowlist_includes_the_ratio_already_in_its_cascade_profile() {
    let (view, _) = super::view_for_inline_style(
        "display:block;position:fixed;left:50px;top:60px;width:40px;height:auto;aspect-ratio:2 / 1",
    );
    assert_eq!(
        first_unsupported_runtime_block_positioning_inline_property(&view),
        None
    );
    assert_eq!(
        first_unsupported_runtime_block_formatting_inline_property(&view),
        Some((view.root_handle().id(), "aspect-ratio".to_owned()))
    );
}

#[test]
fn fixed_containing_block_effects_are_rejected_in_stylesheets_and_inline_style() {
    let declarations = [
        "transform:translate(1px, 2px)",
        "translate:1px",
        "rotate:1deg",
        "scale:2",
        "transform-style:preserve-3d",
        "perspective:500px",
        "filter:blur(1px)",
        "will-change:transform",
        "will-change:perspective",
        "will-change:filter",
        "will-change:backdrop-filter",
        "will-change:contain",
    ];

    for (index, declaration) in declarations.into_iter().enumerate() {
        let (view, _) = super::view_for_inline_style(
            "display:block;position:fixed;left:0;top:0;width:10px;height:10px",
        );
        let stylesheet = StylesheetSource {
            id: format!("c12-3-effect-{index}"),
            base_url: "https://spinon.invalid/c12-3-effect.css".to_owned(),
            origin: CssOrigin::Author,
            css: format!(".ancestor {{{declaration}}}"),
        };
        let stylesheet_result = compute_runtime_block_positioning_cascade_with_stylesheets(
            &view,
            &[stylesheet],
            CssViewport::C04_FIXTURE,
            Default::default(),
        );
        assert!(
            matches!(
                stylesheet_result,
                Err(CssCascadeError::UnsupportedAuthorCss { .. })
            ),
            "stylesheet declaration was not rejected: {declaration}; result={stylesheet_result:?}"
        );

        let (inline_view, inline_node) = super::view_for_inline_style(&format!(
            "display:block;position:fixed;left:0;top:0;width:10px;height:10px;{declaration}"
        ));
        let inline_result =
            first_unsupported_runtime_block_positioning_inline_property(&inline_view);
        assert!(
            matches!(inline_result, Some((node, _)) if node == inline_node),
            "inline declaration was not rejected: {declaration}; result={inline_result:?}"
        );
    }
}

#[test]
fn stylo_unsupported_fixed_containing_block_properties_remain_diagnostics() {
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
        let (view, _) = super::view_for_inline_style(&format!(
            "display:block;position:fixed;left:0;top:0;width:10px;height:10px;{declaration}"
        ));
        let snapshot = compute_runtime_block_positioning_cascade_with_stylesheets(
            &view,
            &[],
            CssViewport::C04_FIXTURE,
            Default::default(),
        )
        .unwrap();
        assert!(
            snapshot.diagnostics.iter().any(|diagnostic| {
                diagnostic.node_id.is_some()
                    && diagnostic
                        .diagnostic
                        .message
                        .contains(declaration.split(':').next().unwrap())
            }),
            "unknown inline property must remain visible as a diagnostic: {declaration}; {:?}",
            snapshot.diagnostics
        );

        let stylesheet = StylesheetSource {
            id: format!("c12-3-unsupported-effect-{index}"),
            base_url: "https://spinon.invalid/c12-3-effect.css".to_owned(),
            origin: CssOrigin::Author,
            css: format!(".ancestor {{{declaration}}}"),
        };
        let (stylesheet_view, _) = super::view_for_inline_style(
            "display:block;position:fixed;left:0;top:0;width:10px;height:10px",
        );
        let stylesheet_snapshot = compute_runtime_block_positioning_cascade_with_stylesheets(
            &stylesheet_view,
            &[stylesheet],
            CssViewport::C04_FIXTURE,
            Default::default(),
        )
        .unwrap();
        assert!(
            stylesheet_snapshot.diagnostics.iter().any(|diagnostic| {
                diagnostic.source_id == format!("c12-3-unsupported-effect-{index}")
                    && diagnostic
                        .diagnostic
                        .message
                        .contains(declaration.split(':').next().unwrap())
            }),
            "unknown stylesheet property must remain visible as a diagnostic: {declaration}; {:?}",
            stylesheet_snapshot.diagnostics
        );
    }
}
