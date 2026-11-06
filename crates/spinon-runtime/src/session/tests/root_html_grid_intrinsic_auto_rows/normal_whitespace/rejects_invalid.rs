use super::*;

#[test]
fn parsed_root_html_normal_whitespace_rejects_invalid_sources_atomically() {
    for (label, invalid_html) in [
        ("whitespace-only", " \t\n "),
        ("form-feed", "A&#x0c;A"),
        ("vertical-tab", "A&#x0b;A"),
        ("nbsp", "A&nbsp;A"),
        ("nel", "A&#x85;A"),
        ("unicode-space", "A&#x2003;A"),
        ("digit-token", "A1"),
        ("punctuation-token", "A-B"),
    ] {
        let source = ROOT_HTML_SOURCE.replacen(
            SHORT_ITEM,
            &format!("<div id=\"short-text\" class=\"item wrapped-text\">{invalid_html}</div>"),
            1,
        );
        let (session, layout) = run(&source);
        let cascade = session.ua_cascade_snapshot();
        assert_eq!(
            cascade.state,
            crate::session::RuntimeUaCascadeState::Ready,
            "case={label}: valid CSS should complete cascade before text-source rejection"
        );
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "case={label}: layout error={:?}; cascade state={:?}; cascade error={:?}",
            layout.error,
            cascade.state,
            cascade.error
        );
        assert!(
            layout.completed.is_none(),
            "case={label} must not publish frames or a paint snapshot"
        );
        let failure = layout
            .error
            .as_ref()
            .unwrap_or_else(|| panic!("case={label} should report a contextual failure"));
        assert_eq!(failure.code, "unsupported_computed_value", "case={label}");
        assert_eq!(
            failure.property.as_deref(),
            Some("grid-auto-rows"),
            "case={label}: bounded source rejection should name the row property"
        );
    }
}
