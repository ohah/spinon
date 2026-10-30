use super::{close_to, engine, node, paragraph};
use crate::{TextLayoutError, TextSource, WhiteSpaceMode, WordBreakMode};

fn pre_wrap(text: &str) -> crate::ParagraphInput {
    let mut input = paragraph(
        text,
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::PreWrap,
        WordBreakMode::Normal,
    );
    input.style.direction = crate::TextDirection::Ltr;
    input
}

#[test]
fn pre_wrap_preserves_repeated_leading_and_trailing_spaces() {
    let mut engine = engine();
    let repeated = pre_wrap("XX  XX");
    let sizes = engine.intrinsic_inline_sizes(&repeated).unwrap();
    close_to(sizes.min_content(), 20.0);
    close_to(sizes.max_content(), 60.0);
    assert_eq!(sizes.min_content_height(), 20.0);
    assert_eq!(sizes.max_content_height(), 10.0);

    for (text, expected_width) in [("  XX", 40.0), ("XX  ", 40.0)] {
        let input = pre_wrap(text);
        let layout = engine.layout_paragraph(&input, 100.0).unwrap();
        assert_eq!(layout.text, text);
        close_to(layout.width, expected_width);
        assert_eq!(layout.height, 10.0);
    }
}

#[test]
fn pre_wrap_soft_wraps_and_keeps_the_preserved_separator_in_the_source() {
    let mut engine = engine();
    let input = pre_wrap("XX XX");
    let layout = engine.layout_paragraph(&input, 25.0).unwrap();

    assert_eq!(layout.text, "XX XX");
    assert_eq!(layout.lines.len(), 2);
    close_to(layout.lines[0].advance, 20.0);
    close_to(layout.lines[0].hanging_advance, 10.0);
    close_to(layout.lines[1].advance, 20.0);
    close_to(layout.lines[1].hanging_advance, 0.0);
    close_to(layout.width, 20.0);
    assert_eq!(layout.height, 20.0);
    assert!(
        layout
            .sources
            .iter()
            .any(|source| { source.source_utf16 == (2..3) && source.paragraph_utf8 == (2..3) })
    );
}

#[test]
fn pre_wrap_normalizes_segment_breaks_and_keeps_crlf_source_provenance() {
    let mut engine = engine();
    let crlf = pre_wrap("X\r\nX");
    let layout = engine.layout_paragraph(&crlf, 100.0).unwrap();

    assert_eq!(layout.text, "X\nX");
    assert_eq!(layout.lines.len(), 2);
    assert_eq!(layout.sources[1].source_utf16, 1..2);
    assert_eq!(layout.sources[2].source_utf16, 2..3);
    assert_eq!(layout.sources[1].paragraph_utf8, 1..2);
    assert_eq!(layout.sources[2].paragraph_utf8, 1..2);
    assert_eq!(layout.height, 20.0);

    let mut split = pre_wrap("");
    split.sources = vec![
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
    let split_layout = engine.layout_paragraph(&split, 100.0).unwrap();
    assert_eq!(split_layout.text, "X\nX");
    assert_eq!(split_layout.lines.len(), 2);
    assert_eq!(split_layout.sources[1].node, node(2));
    assert_eq!(split_layout.sources[1].source_utf16, 1..2);
    assert_eq!(split_layout.sources[2].node, node(4));
    assert_eq!(split_layout.sources[2].source_utf16, 0..1);
    assert_eq!(split_layout.sources[1].paragraph_utf8, 1..2);
    assert_eq!(split_layout.sources[2].paragraph_utf8, 1..2);

    for source_break in ['\r', '\u{000c}'] {
        let text = format!("X{source_break}X");
        let layout = engine.layout_paragraph(&pre_wrap(&text), 100.0).unwrap();
        assert_eq!(layout.text, "X\nX");
        assert_eq!(layout.lines.len(), 2);
    }
}

#[test]
fn pre_wrap_expands_default_ahem_tab_stops_and_maps_the_source_tab() {
    let mut engine = engine();
    let input = pre_wrap("XX\tXX");
    let layout = engine.layout_paragraph(&input, 100.0).unwrap();

    assert_eq!(layout.text, "XX      XX");
    close_to(layout.width, 100.0);
    assert_eq!(layout.height, 10.0);
    let tab_source = layout
        .sources
        .iter()
        .find(|source| source.source_utf16 == (2..3))
        .expect("the source tab maps to its expanded tab-stop range");
    assert_eq!(tab_source.paragraph_utf8, 2..8);

    let sizes = engine.intrinsic_inline_sizes(&input).unwrap();
    close_to(sizes.min_content(), 20.0);
    close_to(sizes.max_content(), 100.0);
}

#[test]
fn pre_wrap_tab_outside_the_fixed_ahem_profile_fails_closed() {
    let mut engine = engine();
    let mut input = paragraph(
        "XX\tXX",
        "Roboto",
        10.0,
        10.0,
        WhiteSpaceMode::PreWrap,
        WordBreakMode::Normal,
    );
    input.style.direction = crate::TextDirection::Ltr;

    assert!(matches!(
        engine.layout_paragraph(&input, 200.0),
        Err(TextLayoutError::UnsupportedPreWrapTabProfile(node)) if node == input.node
    ));

    for text in ["X\tXX", "XX\tXX XX"] {
        let input = pre_wrap(text);
        assert!(matches!(
            engine.layout_paragraph(&input, 200.0),
            Err(TextLayoutError::UnsupportedPreWrapTabProfile(node)) if node == input.node
        ));
    }

    let mut nested = pre_wrap("XX\tXX");
    nested.sources[0].inline_ancestors.push(node(3));
    assert!(matches!(
        engine.layout_paragraph(&nested, 200.0),
        Err(TextLayoutError::UnsupportedPreWrapTabProfile(node)) if node == nested.node
    ));
}
