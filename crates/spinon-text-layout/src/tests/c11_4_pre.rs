use super::{close_to, engine, paragraph};
use crate::{TextLayoutError, WhiteSpaceMode, WordBreakMode};

fn pre(text: &str) -> crate::ParagraphInput {
    let mut input = paragraph(
        text,
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Pre,
        WordBreakMode::Normal,
    );
    input.style.direction = crate::TextDirection::Ltr;
    input
}

#[test]
fn pre_preserves_spaces_and_disables_soft_wrapping() {
    let mut engine = engine();
    let input = pre("XX  XX");
    let sizes = engine.intrinsic_inline_sizes(&input).unwrap();
    close_to(sizes.min_content(), 60.0);
    close_to(sizes.max_content(), 60.0);
    assert_eq!(sizes.min_content_height(), 10.0);
    assert_eq!(sizes.max_content_height(), 10.0);

    let layout = engine.layout_paragraph(&input, 25.0).unwrap();
    assert_eq!(layout.text, "XX  XX");
    assert_eq!(layout.lines.len(), 1);
    close_to(layout.lines[0].advance, 60.0);
    close_to(layout.width, 60.0);
    assert_eq!(layout.height, 10.0);
}

#[test]
fn pre_keeps_space_boundaries_unbreakable_for_min_content() {
    let mut engine = engine();
    let input = pre("XX XX");
    let sizes = engine.intrinsic_inline_sizes(&input).unwrap();
    close_to(sizes.min_content(), 50.0);
    close_to(sizes.max_content(), 50.0);
    assert_eq!(sizes.min_content_height(), 10.0);
    assert_eq!(sizes.max_content_height(), 10.0);
}

#[test]
fn pre_preserves_forced_segment_breaks_and_source_provenance() {
    let mut engine = engine();
    let crlf = pre("XX\r\nXX");
    let layout = engine.layout_paragraph(&crlf, 25.0).unwrap();

    assert_eq!(layout.text, "XX\nXX");
    assert_eq!(layout.lines.len(), 2);
    close_to(layout.lines[0].advance, 20.0);
    close_to(layout.lines[1].advance, 20.0);
    close_to(layout.width, 20.0);
    assert_eq!(layout.height, 20.0);
    assert_eq!(layout.sources[2].source_utf16, 2..3);
    assert_eq!(layout.sources[3].source_utf16, 3..4);
    assert_eq!(layout.sources[2].paragraph_utf8, 2..3);
    assert_eq!(layout.sources[3].paragraph_utf8, 2..3);

    let form_feed = engine.layout_paragraph(&pre("X\u{000c}X"), 25.0).unwrap();
    assert_eq!(form_feed.text, "X\nX");
    assert_eq!(form_feed.lines.len(), 2);
}

#[test]
fn pre_tabs_fail_closed_for_intrinsic_and_final_layout() {
    let mut engine = engine();
    let input = pre("XX\tXX");

    assert!(matches!(
        engine.intrinsic_inline_sizes(&input),
        Err(TextLayoutError::UnsupportedPreTabProfile(node)) if node == input.node
    ));
    assert!(matches!(
        engine.layout_paragraph(&input, 100.0),
        Err(TextLayoutError::UnsupportedPreTabProfile(node)) if node == input.node
    ));
}
