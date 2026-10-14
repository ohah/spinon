use super::*;

#[test]
fn c131_clip_radius_and_transform_fail_closed() {
    let clip_root = "display:block;position:relative;width:120px;height:80px;overflow:clip";
    let cases = [
        (
            "rounded clip ancestor",
            format!("{clip_root};border-radius:10px"),
            "width:40px;height:20px",
            false,
            "radius",
        ),
        (
            "transformed clip ancestor",
            format!("{clip_root};transform:translateX(4px)"),
            "width:40px;height:20px",
            false,
            "transform",
        ),
        (
            "transformed positioned descendant",
            clip_root.to_owned(),
            "position:absolute;left:10px;top:10px;width:40px;height:20px;transform:translateX(4px)",
            true,
            "transform",
        ),
    ];

    for (case, root_style, child_style, effect_on_child, property_marker) in cases {
        let (snapshot, root, children) =
            make_layout_snapshot(&root_style, &[child_style], false, None);
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
        let failure = calculation.layout.unwrap_err();
        let effect_node = if effect_on_child {
            children[0].id()
        } else {
            root.id()
        };
        assert_eq!(
            failure.code, "unsupported_inline_property",
            "{case}: {failure:?}"
        );
        assert_eq!(
            failure.node_id,
            Some(effect_node.get()),
            "{case}: {failure:?}"
        );
        assert!(
            failure
                .property
                .as_deref()
                .is_some_and(|property| property.contains(property_marker)),
            "{case}: expected unsupported {property_marker}, got {failure:?}"
        );
    }

    for (css, property_marker) in [
        (".painted { border-radius:10px; }", "radius"),
        (".painted { transform:translateX(4px); }", "transform"),
    ] {
        let (request, _) =
            super::super::block_paint_tests::block_request_with_author_stylesheet(css);
        let error = compute_request_for_block_flex_overflow_clip(&request).unwrap_err();
        assert!(
            error.contains(property_marker),
            "stylesheet property {property_marker} must fail closed: {error}"
        );
    }
}
