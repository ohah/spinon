use super::*;

#[test]
fn c131_float_formatting_comparison_cases_fail_closed() {
    let cases = [
        (
            "hidden establishes a formatting context",
            "display:block;width:120px;overflow:hidden",
            "block",
            "hidden",
        ),
        (
            "clip does not establish a formatting context",
            "display:block;width:120px;overflow:clip",
            "block",
            "clip",
        ),
        (
            "flow-root establishes a formatting context with clip",
            "display:flow-root;width:120px;overflow:clip",
            "flow-root",
            "clip",
        ),
    ];

    for (case, root_style, expected_display, expected_overflow) in cases {
        let (snapshot, root, children) = make_layout_snapshot(
            root_style,
            &["float:left;width:48px;height:36px"],
            false,
            None,
        );
        let request = WorkRequest {
            key: key_for(&snapshot, EnvironmentRevision::INITIAL),
            snapshot,
            viewport: CssViewport::C04_FIXTURE,
            previous_snapshot: None,
            force_full: true,
            invalidation: None,
            previous_styles: None,
        };

        let calculation = compute_request_for_block_flex_overflow_clip(&request)
            .unwrap_or_else(|error| panic!("{case}: cascade failed unexpectedly: {error}"));
        assert_eq!(calculation.roots.len(), 1, "{case}: cascade snapshot");
        let computed_root = calculation.roots[0]
            .styles
            .elements
            .iter()
            .find(|style| style.node_id == root.id())
            .unwrap_or_else(|| panic!("{case}: root style is missing"));
        assert_eq!(
            computed_root.properties.get("display").map(String::as_str),
            Some(expected_display),
            "{case}: display must retain its computed value"
        );
        for axis in ["overflow-x", "overflow-y"] {
            assert_eq!(
                computed_root.properties.get(axis).map(String::as_str),
                Some(expected_overflow),
                "{case}: {axis} must retain its computed value"
            );
        }
        assert_eq!(
            calculation.layout.unwrap_err(),
            layout_failure(
                "unsupported_inline_property",
                Some(children[0].id().get()),
                Some("float".to_owned()),
            ),
            "{case}: float must stay unsupported until the C26 owner implements it"
        );
    }

    let (request, _) = super::super::block_paint_tests::block_request_with_author_stylesheet(
        ".painted { float:left; }",
    );
    let stylesheet_error = compute_request_for_block_flex_overflow_clip(&request).unwrap_err();
    assert!(
        stylesheet_error.contains("float"),
        "C13.1 stylesheet float must fail closed: {stylesheet_error}"
    );
}
