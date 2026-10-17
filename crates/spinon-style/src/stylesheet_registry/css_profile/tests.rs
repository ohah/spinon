use super::{supported_media_query_surface, supported_root_html_media_query_surface};
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
