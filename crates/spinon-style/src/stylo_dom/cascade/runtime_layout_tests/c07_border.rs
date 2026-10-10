use super::super::CssViewport;
use super::view_for_inline_style;
use crate::{CssOrigin, StylesheetSource};

#[test]
fn c07_border_floor_keeps_values_just_below_an_integer_at_high_dpr() {
    let viewport = CssViewport {
        device_scale_factor: 2.625,
        ..CssViewport::C04_FIXTURE
    };

    for (specified, expected) in [
        ("1.999px", 1.0),
        ("1.9999px", 1.0),
        ("0.3333333333vh", 2.0),
        ("calc(1.999px + 0px)", 1.0),
        ("2px", 2.0),
    ] {
        let (view, node) = view_for_inline_style(&format!(
            "display:block;border-style:solid;border-width:{specified}"
        ));
        let snapshot =
            super::super::compute_runtime_flex_layout_cascade(&view, viewport, Default::default())
                .unwrap();
        let style = snapshot
            .elements
            .iter()
            .find(|style| style.node_id == node)
            .unwrap();
        assert_eq!(
            style.layout_border.left, expected,
            "border-width {specified} at DPR 2.625"
        );
    }

    let (view, node) =
        view_for_inline_style("display:block;font-size:1px;border-style:solid;border-width:2em");
    let snapshot = super::super::compute_runtime_flex_custom_properties_cascade(
        &view,
        viewport,
        Default::default(),
    )
    .unwrap();
    let computed_style = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == node)
        .unwrap();
    assert_eq!(computed_style.layout_border.left, 2.0, "border-width 2em");

    for (css, expected) in [
        ("font-size:1px;--edge:2em;border-width:var(--edge)", 2.0),
        ("--edge:1.999px;border-width:var(--edge)", 1.0),
        ("--edge:2px;border-width:var(--edge)", 2.0),
        (
            "--base:1.999px;--edge:var(--base);border-width:var(--edge)",
            1.0,
        ),
        ("border-width:var(--missing, 1.999px)", 1.0),
        ("border-width:var(--missing, 2px)", 2.0),
        ("--edge:1.999px;border-width:calc(var(--edge) + 0px)", 1.0),
        ("--edge:calc(1.999px + 0px);border-width:var(--edge)", 1.0),
        (
            "--edge:2px;border-width:var(--edge) !important;border-width:1px",
            2.0,
        ),
        (
            "--edge:2px;border-width:var(--edge) /* retained comment */",
            2.0,
        ),
        ("border-width:1.999pt", 2.0),
        ("border-width:thin", 1.0),
        ("border-width:medium", 3.0),
        ("border-width:thick", 5.0),
    ] {
        let (view, node) =
            view_for_inline_style(&format!("display:block;border-style:solid;{css}"));
        let snapshot = super::super::compute_runtime_flex_custom_properties_cascade(
            &view,
            viewport,
            Default::default(),
        )
        .unwrap();
        let computed_style = snapshot
            .elements
            .iter()
            .find(|style| style.node_id == node)
            .unwrap();
        assert_eq!(
            computed_style.layout_border.left, expected,
            "border width from {css}"
        );
    }
}

#[test]
fn c07_border_floor_recovers_author_stylesheet_shorthands() {
    let viewport = CssViewport {
        device_scale_factor: 2.625,
        ..CssViewport::C04_FIXTURE
    };
    for (css, inline_style, expected) in [
        (
            "div { --edge: 1.999px; border-width: var(--edge); border-style: solid; }",
            "display:block",
            1.0,
        ),
        (
            "div { --edge: 2px; border-width: var(--edge); border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { font-size: 1px; border-width: 2em; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { border-width: 0.125rem; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { border-width: 0.25vw; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { border-width: 0.25svw; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { border-width: 0.25lvw; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { border-width: 0.25dvw; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { border-width: 0.25vi; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { border-width: 0.25svi; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { border-width: 0.25lvi; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { border-width: 0.25dvi; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { border-width: 0.3333333333vh; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { border-width: 0.3333333333svh; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { border-width: 0.3333333333lvh; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { border-width: 0.3333333333dvh; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { border-width: 0.3333333333vb; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { border-width: 0.3333333333svb; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { border-width: 0.3333333333lvb; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { border-width: 0.3333333333dvb; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { border-width: 0.3333333333vmin; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { border-width: 0.3333333333dvmin; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { border-width: 0.25vmax; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { border-width: 0.25dvmax; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { --edge: var(--missing, 2px); border-width: var(--edge); border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { --edge: 2px; border-width: var(--missing, var(--edge)); border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { --edge: 1.999px; border-width: calc(var(--edge) + 0px); border-style: solid; }",
            "display:block",
            1.0,
        ),
        (
            "div { --edge: 2px; border-width: calc(var(--edge) + 0px); border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { --edge: 2px; border-width: var(--edge) !important; border-width: 1px; border-style: solid; }",
            "display:block",
            2.0,
        ),
        (
            "div { --edge: 1.999px; border-width: var(--edge); border-style: solid; }",
            "display:block;border-width:2px",
            2.0,
        ),
    ] {
        let (view, node) = view_for_inline_style(inline_style);
        let stylesheet = StylesheetSource {
            id: "border-author-source".to_owned(),
            base_url: "https://spinon.invalid/border-author-source.css".to_owned(),
            origin: CssOrigin::Author,
            css: css.to_owned(),
        };
        let snapshot =
            super::super::compute_runtime_flex_custom_properties_cascade_with_stylesheets(
                &view,
                &[stylesheet],
                viewport,
                Default::default(),
            )
            .unwrap();
        let computed_style = snapshot
            .elements
            .iter()
            .find(|style| style.node_id == node)
            .unwrap();
        assert_eq!(
            computed_style.layout_border.left, expected,
            "{css} / {inline_style}"
        );
    }
}

#[test]
fn c07_border_floor_does_not_drop_a_winning_declaration_after_many_earlier_values() {
    let viewport = CssViewport {
        width_css_px: 800.0,
        height_css_px: 600.0,
        device_scale_factor: 2.625,
        ..CssViewport::C04_FIXTURE
    };
    let repeated_declarations = "border-width:1px;".repeat(256);
    let (view, node) = view_for_inline_style(&format!(
        "display:block;border-style:solid;{repeated_declarations}border-width:0.3333333333vh"
    ));
    let snapshot = super::super::compute_runtime_flex_custom_properties_cascade(
        &view,
        viewport,
        Default::default(),
    )
    .unwrap();
    let computed_style = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == node)
        .unwrap();
    assert_eq!(computed_style.layout_border.left, 2.0);
}

#[test]
fn c07_border_floor_preserves_a_small_border_declaration_in_a_large_style_source() {
    let viewport = CssViewport {
        width_css_px: 800.0,
        height_css_px: 600.0,
        device_scale_factor: 2.625,
        ..CssViewport::C04_FIXTURE
    };
    let large_comment = "x".repeat(2 * 1024 * 1024);
    let (view, node) = view_for_inline_style(&format!(
        "display:block;border-style:solid;/*{large_comment}*/border-width:0.3333333333vh"
    ));
    let snapshot = super::super::compute_runtime_flex_custom_properties_cascade(
        &view,
        viewport,
        Default::default(),
    )
    .unwrap();
    let computed_style = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == node)
        .unwrap();
    assert_eq!(computed_style.layout_border.left, 2.0);
}
