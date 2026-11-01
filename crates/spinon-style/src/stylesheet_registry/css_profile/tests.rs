use super::{
    is_invalid_fixed_px_gap_declaration_diagnostic,
    is_invalid_root_html_named_grid_declaration_diagnostic, supported_media_query_surface,
    supported_root_html_media_query_surface,
};
use crate::stylesheet_registry::{CssOrigin, StylesheetRegistry, StylesheetSource};

#[test]
fn viewport_width_and_height_media_use_css_pixel_lengths() {
    assert!(supported_root_html_media_query_surface(
        "screen and (max-width: 390px) and (min-height: 844px)"
    ));
    assert!(supported_root_html_media_query_surface(
        "(width: 390px), (height: 844px)"
    ));
    assert!(supported_root_html_media_query_surface("(max-width: 0)"));
}

#[test]
fn viewport_media_rejects_unsupported_units_and_invalid_lengths() {
    assert!(!supported_root_html_media_query_surface(
        "(max-width: 24rem)"
    ));
    assert!(!supported_root_html_media_query_surface(
        "(max-width: -1px)"
    ));
    assert!(!supported_root_html_media_query_surface(
        "(orientation: portrait)"
    ));
    for malformed in [
        "width",
        "max-width",
        "screen and",
        "(max-width: 390px) and",
        "(max-width: 390px),",
        "only (max-width: 390px)",
    ] {
        assert!(
            !supported_root_html_media_query_surface(malformed),
            "malformed media query must fail closed: {malformed}"
        );
    }
}

#[test]
fn root_html_gap_recovery_ignores_only_invalid_negative_fixed_px_declarations() {
    for diagnostic in [
        "Unsupported property declaration: 'gap: -1px 10px;', OtherInvalidValue",
        "Unsupported property declaration: 'row-gap: -0.5px;', OtherInvalidValue",
        "Unsupported property declaration: 'column-gap: 10PX -2PX !important;', OtherInvalidValue",
        "Unsupported property declaration: 'GAP: -1PX 10px !IMPORTANT;', OtherInvalidValue",
        "Unsupported property declaration: 'gap: -1px/**/ 10px;', OtherInvalidValue",
        "Unsupported property declaration: 'gap: -1px 10px !/**/IMPORTANT;', OtherInvalidValue",
    ] {
        assert!(
            is_invalid_fixed_px_gap_declaration_diagnostic(diagnostic),
            "negative fixed-px gap declaration should be treated as a discarded CSS declaration: {diagnostic}"
        );
    }

    for diagnostic in [
        "Unsupported property declaration: 'gap: 1px 10px;', OtherInvalidValue",
        "Unsupported property declaration: 'gap: -1em 10px;', OtherInvalidValue",
        "Unsupported property declaration: 'gap: calc(-1px);', OtherInvalidValue",
        "Unsupported property declaration: 'color: bad;', OtherInvalidValue",
        "Unsupported property declaration: 'display: grid;', UnexpectedToken",
    ] {
        assert!(
            !is_invalid_fixed_px_gap_declaration_diagnostic(diagnostic),
            "non-matching CSS parser diagnostic must remain fail-closed: {diagnostic}"
        );
    }
}

#[test]
fn root_html_named_grid_recovery_ignores_invalid_values_only_for_grid_placement_properties() {
    for diagnostic in [
        "Unsupported property declaration: 'grid-template-areas: 1px;', OtherInvalidValue",
        "Unsupported property declaration: 'GRID-AREA: bad;', OtherInvalidValue",
        "Unsupported property declaration: 'grid-row-start: bad;', OtherInvalidValue",
        "Unsupported property declaration: 'grid-column: bad;', OtherInvalidValue",
    ] {
        assert!(
            is_invalid_root_html_named_grid_declaration_diagnostic(diagnostic),
            "CSS-invalid named-grid declaration should be dropped by cascade: {diagnostic}"
        );
    }

    for diagnostic in [
        "Unsupported property declaration: 'grid-template-columns: bad;', OtherInvalidValue",
        "Unsupported property declaration: 'grid-auto-flow: sideways;', OtherInvalidValue",
        "Unsupported property declaration: 'color: bad;', OtherInvalidValue",
        "Unsupported property declaration: 'grid-area: bad;', UnexpectedToken",
    ] {
        assert!(
            !is_invalid_root_html_named_grid_declaration_diagnostic(diagnostic),
            "non-matching diagnostic should remain fail-closed: {diagnostic}"
        );
    }
}

#[test]
fn viewport_and_os_scheme_media_can_be_combined() {
    assert!(supported_root_html_media_query_surface(
        "screen and (max-width: 390px) and (prefers-color-scheme: dark)"
    ));
}

#[test]
fn runtime_media_policy_allows_only_viewport_and_os_scheme_queries() {
    assert!(supported_root_html_media_query_surface(
        "screen and (max-width: 390px) and (prefers-color-scheme: dark)"
    ));
    assert!(!supported_media_query_surface("(orientation: portrait)"));
    assert!(!supported_media_query_surface("(min-resolution: 2dppx)"));
    assert!(!supported_media_query_surface("(max-width: 390px)"));
}

#[test]
fn root_html_media_allowlist_does_not_change_default_runtime_profile() {
    let mut registry = StylesheetRegistry::new();
    registry
        .append(StylesheetSource {
            id: "root-html-media".to_owned(),
            base_url: "https://spinon.invalid/index.html".to_owned(),
            origin: CssOrigin::Author,
            css: "@media (max-width: 390px) { div { width: 123px; } }".to_owned(),
        })
        .unwrap();

    assert!(
        registry
            .first_unsupported_runtime_author_feature(&["width"], false, true, false)
            .is_some()
    );
    assert!(
        registry
            .first_unsupported_runtime_author_feature_with_media(&["width"])
            .is_none()
    );
}

#[test]
fn root_html_media_allowlist_rejects_unknown_features_and_properties() {
    for css in [
        "@media (orientation: portrait) { div { width: 123px; } }",
        "@media (max-width: 390px) { div { color: red; } }",
    ] {
        let mut registry = StylesheetRegistry::new();
        registry
            .append(StylesheetSource {
                id: "root-html-media-negative".to_owned(),
                base_url: "https://spinon.invalid/index.html".to_owned(),
                origin: CssOrigin::Author,
                css: css.to_owned(),
            })
            .unwrap();

        assert!(
            registry
                .first_unsupported_runtime_author_feature_with_media(&["width"])
                .is_some()
        );
    }
}

#[test]
fn color_scheme_is_admitted_only_by_the_root_html_runtime_profile() {
    let mut registry = StylesheetRegistry::new();
    registry
        .append(StylesheetSource {
            id: "root-html-color-scheme".to_owned(),
            base_url: "https://spinon.invalid/index.html".to_owned(),
            origin: CssOrigin::Author,
            css: ":root { color-scheme: light dark; }".to_owned(),
        })
        .unwrap();

    assert!(
        registry
            .first_unsupported_runtime_author_feature(&["display"], false, true, false)
            .is_some()
    );
    assert!(
        registry
            .first_unsupported_runtime_author_feature_with_media(&["display", "color-scheme"])
            .is_none()
    );
}

#[test]
fn root_html_canvas_profile_rejects_system_colors_and_their_custom_properties() {
    for css in [
        "body { background-color: Canvas; }",
        "@media (prefers-color-scheme: dark) { body { background-color: CanvasText; } }",
        ":root { --canvas-background: Canvas; } body { background-color: var(--canvas-background); }",
    ] {
        let mut registry = StylesheetRegistry::new();
        registry
            .append(StylesheetSource {
                id: "root-html-system-color".to_owned(),
                base_url: "https://spinon.invalid/index.html".to_owned(),
                origin: CssOrigin::Author,
                css: css.to_owned(),
            })
            .unwrap();

        assert!(
            registry
                .first_unsupported_runtime_root_html_system_color()
                .is_some(),
            "system color must fail closed: {css}"
        );
    }
}

#[test]
fn root_html_canvas_system_color_scanner_ignores_string_contents() {
    let mut registry = StylesheetRegistry::new();
    registry
        .append(StylesheetSource {
            id: "root-html-system-color-string".to_owned(),
            base_url: "https://spinon.invalid/index.html".to_owned(),
            origin: CssOrigin::Author,
            css: ":root { --label: \"Canvas\"; } body { background-color: #123456; }".to_owned(),
        })
        .unwrap();

    assert_eq!(
        registry.first_unsupported_runtime_root_html_system_color(),
        None
    );
}
