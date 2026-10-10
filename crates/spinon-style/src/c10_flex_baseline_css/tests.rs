use super::invalidate_chrome_unsupported_last_baseline;

#[test]
fn inline_invalid_values_keep_the_previous_declaration_and_important_priority() {
    let input = concat!(
        "align-content: center;",
        "align-content: LAST/**/baseline;",
        "place-content: flex-end;",
        "place-content: center last baseline !important;",
    );
    let output = invalidate_chrome_unsupported_last_baseline(input, false);

    assert_eq!(
        output,
        concat!(
            "align-content: center;",
            "align-content: xAST/**/baseline;",
            "place-content: flex-end;",
            "place-content: center xast baseline !important;",
        )
    );
}

#[test]
fn stylesheet_filter_walks_grouping_rules_without_touching_strings_or_comments() {
    let input = concat!(
        "/* align-content:last baseline */",
        ".a { content: \"align-content:last baseline\";",
        " --value: var(--fallback, last baseline);",
        " align-content: center; align-content: last baseline; }",
        "@media screen { .b { place-content: center;",
        " place-content: center last baseline; } }",
    );
    let output = invalidate_chrome_unsupported_last_baseline(input, true);

    assert_eq!(
        output,
        concat!(
            "/* align-content:last baseline */",
            ".a { content: \"align-content:last baseline\";",
            " --value: var(--fallback, last baseline);",
            " align-content: center; align-content: xast baseline; }",
            "@media screen { .b { place-content: center;",
            " place-content: center xast baseline; } }",
        )
    );
}

#[test]
fn escaped_identifiers_are_invalidated_without_changing_source_offsets() {
    let input = "align-content: l\\61 st baseline; width: 10px";
    let output = invalidate_chrome_unsupported_last_baseline(input, false);

    assert_eq!(output, "align-content: x\\61 st baseline; width: 10px");
    assert_eq!(input.len(), output.len());
    assert_eq!(input.lines().count(), output.lines().count());
}

#[test]
fn unrelated_properties_and_values_are_byte_for_byte_stable() {
    for input in [
        "align-content: first baseline",
        "align-content: center",
        "place-content: space-between center",
        "--alignment: last baseline",
        "align-content: var(--alignment, last baseline)",
        "content: \"last baseline\"",
    ] {
        assert_eq!(
            invalidate_chrome_unsupported_last_baseline(input, false),
            input
        );
    }
}

#[test]
fn excessive_nesting_does_not_partially_rewrite_the_source() {
    let mut input = String::new();
    for _ in 0..66 {
        input.push_str("@media screen {");
    }
    input.push_str(".a { align-content: center; align-content: last baseline; }");
    for _ in 0..66 {
        input.push('}');
    }

    assert_eq!(
        invalidate_chrome_unsupported_last_baseline(&input, true),
        input
    );
}
