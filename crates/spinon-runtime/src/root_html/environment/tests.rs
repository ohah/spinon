use super::{
    ColorSchemeMetadata, RootHtmlEnvironmentMetadata, parse_color_scheme_metadata,
    resolve_root_html_canvas_color_scheme,
};
use crate::root_html::ParsedRootHtml;
use spinon_style::{CssColorScheme, CssMediaEnvironment};

const MOBILE_DEFAULT: &str = include_str!(
    "../../../../../spec/internal/evidence/c04-root-html-document-environment-2026-10-17/mobile-default.html"
);
const MOBILE_DEVICE_WIDTH: &str = include_str!(
    "../../../../../spec/internal/evidence/c04-root-html-document-environment-2026-10-17/mobile-width-device.html"
);
const MOBILE_FIXED_WIDTH: &str = include_str!(
    "../../../../../spec/internal/evidence/c04-root-html-document-environment-2026-10-17/mobile-width-320.html"
);
const MOBILE_HALF_SCALE: &str = include_str!(
    "../../../../../spec/internal/evidence/c04-root-html-document-environment-2026-10-17/mobile-scale-half.html"
);
const MOBILE_DUPLICATE_WIDTH: &str = include_str!(
    "../../../../../spec/internal/evidence/c04-root-html-document-environment-2026-10-17/mobile-duplicate-width.html"
);
const SCHEME_INVALID: &str = include_str!(
    "../../../../../spec/internal/evidence/c04-root-html-document-environment-2026-10-17/scheme-invalid.html"
);
const SCHEME_META_DARK: &str = include_str!(
    "../../../../../spec/internal/evidence/c04-root-html-document-environment-2026-10-17/scheme-meta-dark.html"
);
const SCHEME_META_LIGHT_DARK: &str = include_str!(
    "../../../../../spec/internal/evidence/c04-root-html-document-environment-2026-10-17/scheme-meta-light-dark.html"
);
const SCHEME_ONLY_DARK: &str = include_str!(
    "../../../../../spec/internal/evidence/c04-root-html-document-environment-2026-10-17/scheme-only-dark.html"
);
const SCHEME_DUPLICATE: &str = include_str!(
    "../../../../../spec/internal/evidence/c04-root-html-document-environment-2026-10-17/scheme-duplicate.html"
);
const CANVAS_INVALID_THEN_VALID: &str = include_str!(
    "../../../../../spec/internal/evidence/c04-12-root-canvas-color-scheme-2026-10-17/meta-invalid-then-valid.html"
);

fn resolve(source: &str, color_scheme: CssColorScheme) -> super::ResolvedRootHtmlEnvironment {
    let parsed = ParsedRootHtml::parse(source).unwrap();
    let metadata = RootHtmlEnvironmentMetadata::from_document(&parsed);
    let mut media_environment = CssMediaEnvironment::MOBILE;
    media_environment.color_scheme = color_scheme;
    metadata
        .resolve(1170.0, 2532.0, 3.0, media_environment)
        .unwrap()
}

fn close_to(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() <= 0.001,
        "expected {expected}, got {actual}"
    );
}

#[test]
fn mobile_default_matches_chrome_overview_viewport_and_keeps_page_scale_separate() {
    let resolved = resolve(MOBILE_DEFAULT, CssColorScheme::Light);

    assert_eq!(resolved.viewport_width_css_px, 980.0);
    close_to(resolved.viewport_height_css_px, 2120.8206);
    close_to(resolved.page_scale, 0.39795917);
    assert_eq!(resolved.device_scale_factor, 3.0);
}

#[test]
fn device_width_metadata_matches_chrome_css_viewport() {
    let resolved = resolve(MOBILE_DEVICE_WIDTH, CssColorScheme::Light);

    assert_eq!(resolved.viewport_width_css_px, 390.0);
    assert_eq!(resolved.viewport_height_css_px, 844.0);
    assert_eq!(resolved.page_scale, 1.0);
}

#[test]
fn narrow_fixed_width_expands_to_the_mobile_screen_width() {
    let resolved = resolve(MOBILE_FIXED_WIDTH, CssColorScheme::Light);

    assert_eq!(resolved.viewport_width_css_px, 390.0);
    assert_eq!(resolved.viewport_height_css_px, 844.0);
    assert_eq!(resolved.page_scale, 1.0);
}

#[test]
fn initial_scale_changes_layout_viewport_but_not_device_scale() {
    let resolved = resolve(MOBILE_HALF_SCALE, CssColorScheme::Light);

    assert_eq!(resolved.viewport_width_css_px, 780.0);
    assert_eq!(resolved.viewport_height_css_px, 1688.0);
    assert_eq!(resolved.page_scale, 0.5);
    assert_eq!(resolved.device_scale_factor, 3.0);
}

#[test]
fn last_viewport_element_matches_pinned_chrome_duplicate_fixture() {
    let resolved = resolve(MOBILE_DUPLICATE_WIDTH, CssColorScheme::Light);

    assert_eq!(resolved.viewport_width_css_px, 500.0);
    assert_eq!(resolved.viewport_height_css_px, 844.0);
    assert_eq!(resolved.page_scale, 1.0);
}

#[test]
fn document_scheme_does_not_replace_os_prefers_color_scheme() {
    let resolved = resolve(SCHEME_META_DARK, CssColorScheme::Light);

    assert_eq!(resolved.document_color_scheme, CssColorScheme::Dark);
    assert_eq!(resolved.prefers_color_scheme, CssColorScheme::Light);
    assert_eq!(
        resolved.media_environment.color_scheme,
        CssColorScheme::Light
    );
}

#[test]
fn supported_scheme_list_selects_the_native_preference() {
    assert_eq!(
        resolve(SCHEME_META_LIGHT_DARK, CssColorScheme::Dark).document_color_scheme,
        CssColorScheme::Dark
    );
    assert_eq!(
        resolve(SCHEME_META_LIGHT_DARK, CssColorScheme::Light).document_color_scheme,
        CssColorScheme::Light
    );
}

#[test]
fn only_scheme_overrides_the_native_preference() {
    let resolved = resolve(SCHEME_ONLY_DARK, CssColorScheme::Light);

    assert_eq!(resolved.document_color_scheme, CssColorScheme::Dark);
    assert_eq!(resolved.prefers_color_scheme, CssColorScheme::Light);
}

#[test]
fn invalid_first_color_scheme_metadata_falls_back_to_light() {
    let resolved = resolve(SCHEME_INVALID, CssColorScheme::Dark);

    assert_eq!(resolved.document_color_scheme, CssColorScheme::Light);
    assert_eq!(resolved.prefers_color_scheme, CssColorScheme::Dark);
}

#[test]
fn first_color_scheme_element_wins_for_the_duplicate_fixture() {
    let resolved = resolve(SCHEME_DUPLICATE, CssColorScheme::Light);

    assert_eq!(resolved.document_color_scheme, CssColorScheme::Light);
    assert_eq!(resolved.prefers_color_scheme, CssColorScheme::Light);
}

#[test]
fn duplicate_color_scheme_metadata_is_detected_even_when_the_first_value_is_invalid() {
    let parsed = ParsedRootHtml::parse(CANVAS_INVALID_THEN_VALID).unwrap();
    let metadata = RootHtmlEnvironmentMetadata::from_document(&parsed);

    assert!(metadata.has_duplicate_color_scheme_metadata());
    assert_eq!(metadata.color_scheme, None);
}

#[test]
fn root_canvas_scheme_uses_css_precedence_metadata_fallback_and_light_default() {
    let dark = ColorSchemeMetadata {
        light: false,
        dark: true,
        only: false,
    };
    let both = ColorSchemeMetadata {
        light: true,
        dark: true,
        only: false,
    };

    assert_eq!(
        resolve_root_html_canvas_color_scheme(Some(dark), "normal", CssColorScheme::Light),
        Some(CssColorScheme::Dark)
    );
    assert_eq!(
        resolve_root_html_canvas_color_scheme(Some(both), "normal", CssColorScheme::Dark),
        Some(CssColorScheme::Dark)
    );
    assert_eq!(
        resolve_root_html_canvas_color_scheme(None, "normal", CssColorScheme::Dark),
        Some(CssColorScheme::Light)
    );
    assert_eq!(
        resolve_root_html_canvas_color_scheme(
            Some(ColorSchemeMetadata {
                light: true,
                dark: false,
                only: false,
            }),
            "dark",
            CssColorScheme::Light
        ),
        Some(CssColorScheme::Dark)
    );
}

#[test]
fn root_canvas_scheme_rejects_unknown_computed_values() {
    assert_eq!(
        resolve_root_html_canvas_color_scheme(None, "sepia", CssColorScheme::Light),
        None
    );
}

#[test]
fn color_scheme_tokens_are_case_insensitive_and_reject_unknown_values() {
    assert!(parse_color_scheme_metadata("ONLY Dark").is_some());
    assert!(parse_color_scheme_metadata("light dark").is_some());
    assert!(parse_color_scheme_metadata("dark sepia").is_none());
    assert!(parse_color_scheme_metadata("only dark light").is_none());
}

#[test]
fn invalid_surface_inputs_fail_before_creating_a_css_viewport() {
    let parsed = ParsedRootHtml::parse(MOBILE_DEVICE_WIDTH).unwrap();
    let metadata = RootHtmlEnvironmentMetadata::from_document(&parsed);

    assert!(
        metadata
            .resolve(1170.0, 2532.0, 0.0, CssMediaEnvironment::MOBILE)
            .is_err()
    );
    assert!(
        metadata
            .resolve(f32::NAN, 2532.0, 3.0, CssMediaEnvironment::MOBILE)
            .is_err()
    );
    assert!(
        metadata
            .resolve(1170.0, f32::INFINITY, 3.0, CssMediaEnvironment::MOBILE)
            .is_err()
    );
}
