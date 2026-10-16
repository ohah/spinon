use spinon_core::NodeId;

use crate::{
    ParagraphInput, ParagraphStyle, TextLayoutEngine, TextSource, WhiteSpaceMode, WordBreakMode,
};

const AHEM: &[u8] = include_bytes!("../../../tests/fixtures/resources/c15/Ahem.ttf");
const ROBOTO: &[u8] = include_bytes!("../../../tests/fixtures/resources/c15/Roboto-Variable.ttf");

fn node(value: u64) -> NodeId {
    NodeId::new(value).unwrap()
}

fn engine() -> TextLayoutEngine {
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).unwrap();
    engine.register_font("Roboto", ROBOTO).unwrap();
    engine
}

fn paragraph(
    text: &str,
    font_family: &str,
    font_size: f32,
    line_height: f32,
    white_space: WhiteSpaceMode,
    word_break: WordBreakMode,
) -> ParagraphInput {
    ParagraphInput {
        node: node(1),
        sources: vec![TextSource {
            node: node(2),
            inline_ancestors: Vec::new(),
            text: text.to_owned(),
        }],
        style: ParagraphStyle {
            font_family: font_family.to_owned(),
            font_size,
            line_height,
            white_space,
            word_break,
        },
        inline_styles: Default::default(),
        inline_run_styles: Default::default(),
    }
}

fn close_to(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() <= 0.05,
        "actual {actual} differs from expected {expected} CSS px"
    );
}

#[test]
fn ahem_normalizes_collapsed_spaces_and_preserves_source_range() {
    let mut engine = engine();
    let input = paragraph(
        "X   X",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    let layout = engine.layout_paragraph(&input, 30.0).unwrap();

    assert_eq!(layout.text, "X X");
    assert_eq!(layout.lines.len(), 1);
    assert_eq!(layout.sources.len(), 5);
    close_to(layout.width, 30.0);
    close_to(layout.height, 10.0);
    close_to(layout.lines[0].baseline - layout.lines[0].line_top, 8.0);
    assert!(
        !layout.runs.is_empty(),
        "Parley must return a shaped font run"
    );
    assert!(
        !layout.runs[0].glyphs.is_empty(),
        "the snapshot must retain glyph IDs"
    );
    let glyphs = layout
        .runs
        .iter()
        .flat_map(|run| &run.glyphs)
        .collect::<Vec<_>>();
    assert_eq!(glyphs.len(), 3);
    for (glyph, expected_x) in glyphs.iter().zip([0.0, 10.0, 20.0]) {
        close_to(glyph.x, expected_x);
        close_to(glyph.advance, 10.0);
    }
    assert_eq!(
        layout.sources[1..4]
            .iter()
            .map(|source| source.source_utf16.clone())
            .collect::<Vec<_>>(),
        vec![1..2, 2..3, 3..4]
    );
    assert!(
        layout.sources[1..4]
            .iter()
            .all(|source| source.paragraph_utf8 == (1..2))
    );
}

#[test]
fn collapsed_space_and_pre_line_crlf_keep_all_source_code_units() {
    let mut engine = engine();
    let mut collapsed = paragraph(
        "",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    collapsed.sources = vec![
        TextSource {
            node: node(2),
            inline_ancestors: Vec::new(),
            text: "X ".to_owned(),
        },
        TextSource {
            node: node(4),
            inline_ancestors: vec![node(3)],
            text: "  X".to_owned(),
        },
    ];
    let collapsed_layout = engine.layout_paragraph(&collapsed, 30.0).unwrap();
    assert_eq!(collapsed_layout.text, "X X");
    let mapped_spaces = collapsed_layout
        .sources
        .iter()
        .filter(|source| source.paragraph_utf8 == (1..2))
        .collect::<Vec<_>>();
    assert_eq!(mapped_spaces.len(), 3);
    assert_eq!(mapped_spaces[0].node, node(2));
    assert_eq!(mapped_spaces[0].source_utf16, 1..2);
    assert_eq!(mapped_spaces[1].node, node(4));
    assert_eq!(mapped_spaces[1].source_utf16, 0..1);
    assert_eq!(mapped_spaces[2].node, node(4));
    assert_eq!(mapped_spaces[2].source_utf16, 1..2);
    assert!(
        mapped_spaces
            .iter()
            .skip(1)
            .all(|source| { source.inline_ancestors == vec![node(3)] })
    );
    let second_node_fragment = collapsed_layout
        .inline_fragments
        .iter()
        .find(|fragment| fragment.node == node(4))
        .unwrap();
    close_to(second_node_fragment.rect.x, 20.0);
    close_to(second_node_fragment.rect.width, 10.0);

    let crlf = paragraph(
        "X\r\nX",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::PreLine,
        WordBreakMode::Normal,
    );
    let crlf_layout = engine.layout_paragraph(&crlf, 30.0).unwrap();
    assert_eq!(crlf_layout.text, "X\nX");
    assert_eq!(crlf_layout.sources[1].source_utf16, 1..2);
    assert_eq!(crlf_layout.sources[2].source_utf16, 2..3);
    assert_eq!(crlf_layout.sources[1].paragraph_utf8, 1..2);
    assert_eq!(crlf_layout.sources[2].paragraph_utf8, 1..2);
    assert_eq!(crlf_layout.lines.len(), 2);
}

#[test]
fn ahem_wraps_words_and_keeps_long_unbreakable_words_overflowing() {
    let mut engine = engine();
    let wrap = paragraph(
        "XX XX",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    let wrapped = engine.layout_paragraph(&wrap, 30.0).unwrap();
    assert_eq!(wrapped.lines.len(), 2);
    close_to(wrapped.lines[0].advance, 20.0);
    close_to(wrapped.lines[1].advance, 20.0);
    close_to(wrapped.height, 20.0);

    let long_word = paragraph(
        "XX",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    let overflow = engine.layout_paragraph(&long_word, 15.0).unwrap();
    assert_eq!(overflow.lines.len(), 1);
    close_to(overflow.lines[0].advance, 20.0);
}

#[test]
fn ahem_supports_break_all_pre_line_and_no_wrap() {
    let mut engine = engine();
    let break_all = paragraph(
        "XXXXX",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::BreakAll,
    );
    let broken = engine.layout_paragraph(&break_all, 20.0).unwrap();
    assert_eq!(broken.lines.len(), 3);
    for (line, expected) in broken.lines.iter().zip([20.0, 20.0, 10.0]) {
        close_to(line.advance, expected);
    }

    let pre_line = paragraph(
        "X\nX",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::PreLine,
        WordBreakMode::Normal,
    );
    let preserved = engine.layout_paragraph(&pre_line, 30.0).unwrap();
    assert_eq!(preserved.lines.len(), 2);
    close_to(preserved.height, 20.0);

    let nowrap = paragraph(
        "XX XX",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::NoWrap,
        WordBreakMode::Normal,
    );
    let unwrapped = engine.layout_paragraph(&nowrap, 30.0).unwrap();
    assert_eq!(unwrapped.lines.len(), 1);
    close_to(unwrapped.width, 50.0);
}

#[test]
fn roboto_uses_real_shaped_glyph_advances_and_metrics() {
    let mut engine = engine();
    let iiii = paragraph(
        "iiii",
        "Roboto",
        20.0,
        24.0,
        WhiteSpaceMode::NoWrap,
        WordBreakMode::Normal,
    );
    let wide = paragraph(
        "WWWW",
        "Roboto",
        20.0,
        24.0,
        WhiteSpaceMode::NoWrap,
        WordBreakMode::Normal,
    );
    let office = paragraph(
        "office",
        "Roboto",
        20.0,
        24.0,
        WhiteSpaceMode::NoWrap,
        WordBreakMode::Normal,
    );

    let narrow_layout = engine.layout_paragraph(&iiii, 200.0).unwrap();
    let wide_layout = engine.layout_paragraph(&wide, 200.0).unwrap();
    let office_layout = engine.layout_paragraph(&office, 200.0).unwrap();
    close_to(narrow_layout.width, 19.4531);
    close_to(wide_layout.width, 70.9844);
    close_to(office_layout.width, 49.5625);
    assert!(
        wide_layout.width > narrow_layout.width * 3.0,
        "same code-unit count must not imply the same width"
    );
    assert!(
        narrow_layout.runs[0].font_metrics.ascent > 0.0,
        "the shaped run must preserve its real font ascent"
    );
    assert!(
        narrow_layout.runs[0]
            .glyphs
            .iter()
            .all(|glyph| glyph.id > 0),
        "Roboto fixture characters must have non-missing glyph IDs"
    );
}

#[test]
fn adjacent_text_nodes_keep_source_order_and_inline_fragments() {
    let mut engine = engine();
    let mut input = paragraph(
        "",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    input.sources = vec![
        TextSource {
            node: node(2),
            inline_ancestors: Vec::new(),
            text: "X".to_owned(),
        },
        TextSource {
            node: node(4),
            inline_ancestors: vec![node(3)],
            text: "X".to_owned(),
        },
        TextSource {
            node: node(5),
            inline_ancestors: Vec::new(),
            text: "X".to_owned(),
        },
    ];
    let layout = engine.layout_paragraph(&input, 30.0).unwrap();

    assert_eq!(layout.text, "XXX");
    assert_eq!(layout.sources.len(), 3);
    assert_eq!(
        layout
            .sources
            .iter()
            .map(|source| source.node)
            .collect::<Vec<_>>(),
        vec![node(2), node(4), node(5)]
    );
    let span_fragment = layout
        .inline_fragments
        .iter()
        .find(|fragment| fragment.node == node(3))
        .expect("the span owner must have one fragment");
    close_to(span_fragment.rect.x, 10.0);
    close_to(span_fragment.rect.width, 10.0);

    let mut wrapped_span = paragraph(
        "",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    wrapped_span.sources = vec![TextSource {
        node: node(12),
        inline_ancestors: vec![node(11)],
        text: "XX XX".to_owned(),
    }];
    let wrapped_layout = engine.layout_paragraph(&wrapped_span, 30.0).unwrap();
    let mut span_fragments = wrapped_layout
        .inline_fragments
        .iter()
        .filter(|fragment| fragment.node == node(11))
        .collect::<Vec<_>>();
    span_fragments.sort_by_key(|fragment| fragment.line_index);
    assert_eq!(span_fragments.len(), 2);
    for fragment in span_fragments {
        close_to(fragment.rect.x, 0.0);
        close_to(fragment.rect.width, 20.0);
        close_to(fragment.rect.height, 10.0);
    }
}

#[test]
fn supplementary_scalar_preserves_utf16_and_utf8_source_ranges() {
    let input = paragraph(
        "😀",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    let normalized = crate::normalize::normalize(&input);

    assert_eq!(normalized.text, "😀");
    assert_eq!(normalized.sources.len(), 1);
    assert_eq!(normalized.sources[0].node, node(2));
    assert_eq!(normalized.sources[0].source_utf16, 0..2);
    assert_eq!(normalized.sources[0].paragraph_utf8, 0..4);
}

#[path = "tests/engine_boundaries.rs"]
mod engine_boundaries;

#[path = "tests/c15_2.rs"]
mod c15_2;

#[path = "tests/paragraph_style.rs"]
mod paragraph_style;
