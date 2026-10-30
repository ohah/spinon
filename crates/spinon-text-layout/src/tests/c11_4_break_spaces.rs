use super::{close_to, engine, node, paragraph};
use crate::{TextLayoutError, TextSource, WhiteSpaceMode, WordBreakMode};

fn break_spaces(text: &str) -> crate::ParagraphInput {
    let mut input = paragraph(
        text,
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::BreakSpaces,
        WordBreakMode::Normal,
    );
    input.style.direction = crate::TextDirection::Ltr;
    input.style.line_break = crate::LineBreakMode::Normal;
    input
}

#[test]
fn break_spaces_preserves_each_space_for_min_and_max_content() {
    let mut engine = engine();
    let input = break_spaces("XX  XX");
    let sizes = engine.intrinsic_inline_sizes(&input).unwrap();

    close_to(sizes.min_content(), 30.0);
    close_to(sizes.max_content(), 60.0);
    assert_eq!(sizes.min_content_height(), 20.0);
    assert_eq!(sizes.max_content_height(), 10.0);

    let layout = engine.layout_paragraph(&input, 25.0).unwrap();
    assert_eq!(layout.text, "XX  XX");
    assert_eq!(layout.lines.len(), 2);
    assert_eq!(layout.available_width, 25.0);
    close_to(layout.width, 30.0);
    close_to(layout.lines[0].advance, 30.0);
    close_to(layout.lines[0].hanging_advance, 0.0);
    close_to(layout.lines[1].advance, 30.0);
    close_to(layout.lines[1].hanging_advance, 0.0);
    assert_eq!(layout.height, 20.0);
}

#[test]
fn break_spaces_preserves_leading_and_trailing_spaces_in_max_content() {
    let mut engine = engine();
    for text in ["  XX", "XX  "] {
        let input = break_spaces(text);
        let sizes = engine.intrinsic_inline_sizes(&input).unwrap();
        close_to(sizes.max_content(), 40.0);

        let layout = engine.layout_paragraph(&input, 100.0).unwrap();
        assert_eq!(layout.text, text);
        close_to(layout.width, 40.0);
        assert_eq!(layout.height, 10.0);
    }

    let wrapped_trailing = engine
        .layout_paragraph(&break_spaces("XX  "), 30.0)
        .unwrap();
    assert_eq!(wrapped_trailing.lines.len(), 2);
    close_to(wrapped_trailing.lines[0].advance, 30.0);
    close_to(wrapped_trailing.lines[0].hanging_advance, 0.0);
    close_to(wrapped_trailing.lines[1].advance, 10.0);
    close_to(wrapped_trailing.lines[1].hanging_advance, 0.0);
    assert_eq!(wrapped_trailing.height, 20.0);
}

#[test]
fn break_spaces_forced_break_keeps_each_crlf_source_mapping() {
    let mut engine = engine();
    let input = break_spaces("XX  \r\nXX");
    let sizes = engine.intrinsic_inline_sizes(&input).unwrap();
    close_to(sizes.max_content(), 40.0);
    close_to(sizes.min_content(), 30.0);
    assert_eq!(sizes.min_content_height(), 30.0);
    assert_eq!(sizes.max_content_height(), 20.0);

    let layout = engine.layout_paragraph(&input, 100.0).unwrap();
    assert_eq!(layout.text, "XX  \nXX");
    assert_eq!(layout.lines.len(), 2);
    assert_eq!(layout.height, 20.0);
    assert_eq!(layout.sources[4].source_utf16, 4..5);
    assert_eq!(layout.sources[5].source_utf16, 5..6);
    assert_eq!(layout.sources[4].paragraph_utf8, 4..5);
    assert_eq!(layout.sources[5].paragraph_utf8, 4..5);
}

#[test]
fn break_spaces_segment_break_split_across_sources_keeps_provenance() {
    let mut engine = engine();
    let mut input = break_spaces("");
    input.sources = vec![
        TextSource {
            node: node(2),
            inline_ancestors: Vec::new(),
            text: "X\r".to_owned(),
        },
        TextSource {
            node: node(4),
            inline_ancestors: Vec::new(),
            text: "\nX".to_owned(),
        },
    ];

    let layout = engine.layout_paragraph(&input, 100.0).unwrap();
    assert_eq!(layout.text, "X\nX");
    assert_eq!(layout.lines.len(), 2);
    assert_eq!(layout.sources[1].node, node(2));
    assert_eq!(layout.sources[1].source_utf16, 1..2);
    assert_eq!(layout.sources[2].node, node(4));
    assert_eq!(layout.sources[2].source_utf16, 0..1);
    assert_eq!(layout.sources[1].paragraph_utf8, 1..2);
    assert_eq!(layout.sources[2].paragraph_utf8, 1..2);
}

#[test]
fn break_spaces_tabs_fail_closed() {
    let mut engine = engine();
    let input = break_spaces("XX\tXX");

    assert!(matches!(
        engine.intrinsic_inline_sizes(&input),
        Err(TextLayoutError::UnsupportedBreakSpacesTabProfile(node)) if node == input.node
    ));
    assert!(matches!(
        engine.layout_paragraph(&input, 100.0),
        Err(TextLayoutError::UnsupportedBreakSpacesTabProfile(node)) if node == input.node
    ));
}
