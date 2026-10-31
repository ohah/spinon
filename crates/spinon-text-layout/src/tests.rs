use spinon_core::NodeId;

use crate::{
    InlineBoxStyle, InlineEdges, ParagraphInput, ParagraphStyle, TextLayoutEngine, TextSource,
    WhiteSpaceMode, WordBreakMode,
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
            overflow_wrap: Default::default(),
            writing_mode: spinon_core::WritingMode::HorizontalTb,
            text_orientation: spinon_core::TextOrientation::Mixed,
            font_family: font_family.to_owned(),
            font_size,
            line_height,
            white_space,
            word_break,
            direction: crate::TextDirection::Auto,
            line_break: crate::LineBreakMode::Strict,
            ligatures: crate::LigatureMode::Normal,
            ..ParagraphStyle::default()
        },
        inline_styles: Default::default(),
        inline_run_styles: Default::default(),
    }
}

fn vertical_paragraph(text: &str) -> ParagraphInput {
    let mut input = paragraph(
        text,
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    input.style.writing_mode = spinon_core::WritingMode::VerticalRl;
    input.style.text_orientation = spinon_core::TextOrientation::Mixed;
    input.style.direction = crate::TextDirection::Ltr;
    input.style.line_break = crate::LineBreakMode::Normal;
    input
}

fn assert_vertical_intrinsic_rejected(engine: &mut TextLayoutEngine, input: &ParagraphInput) {
    assert!(matches!(
        engine.intrinsic_logical_sizes(input),
        Err(crate::TextLayoutError::UnsupportedVerticalIntrinsicProfile(node))
            if node == input.node
    ));
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
fn ahem_intrinsic_inline_sizes_follow_normal_break_all_and_nowrap_oracles() {
    let mut engine = engine();
    let normal = paragraph(
        "XX XXXXXXX",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    let normal_sizes = engine.intrinsic_inline_sizes(&normal).unwrap();
    assert_eq!(normal_sizes.paragraph(), normal.node);
    assert_eq!(
        normal_sizes.font_registry_revision(),
        engine.font_registry_revision()
    );
    let mut cloned_engine = engine.clone();
    let cloned_sizes = cloned_engine.intrinsic_inline_sizes(&normal).unwrap();
    assert!(normal_sizes.uses_same_font_registry_revision(&cloned_sizes));
    let mut independent_engine = TextLayoutEngine::new_without_system_fonts();
    independent_engine.register_font("Ahem", AHEM).unwrap();
    independent_engine.register_font("Roboto", ROBOTO).unwrap();
    let independent_sizes = independent_engine.intrinsic_inline_sizes(&normal).unwrap();
    assert_eq!(
        normal_sizes.font_registry_revision(),
        independent_sizes.font_registry_revision()
    );
    assert!(!normal_sizes.uses_same_font_registry_revision(&independent_sizes));
    assert_ne!(normal_sizes, independent_sizes);
    close_to(normal_sizes.min_content(), 70.0);
    close_to(normal_sizes.max_content(), 100.0);

    let break_all = paragraph(
        "XXXXXXXX",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::BreakAll,
    );
    let break_all_sizes = engine.intrinsic_inline_sizes(&break_all).unwrap();
    close_to(break_all_sizes.min_content(), 10.0);
    close_to(break_all_sizes.max_content(), 80.0);

    let nowrap = paragraph(
        "XX XXXXXXX",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::NoWrap,
        WordBreakMode::Normal,
    );
    let nowrap_sizes = engine.intrinsic_inline_sizes(&nowrap).unwrap();
    close_to(nowrap_sizes.min_content(), 100.0);
    close_to(nowrap_sizes.max_content(), 100.0);
}

#[test]
fn ahem_letter_spacing_uses_adjacent_units_and_suppresses_line_edges() {
    let mut engine = engine();
    let cases = [
        ("normal", 0.0, 20.0, 20.0, 50.0),
        ("negative", -1.0, 19.0, 19.0, 46.0),
        ("fractional", 0.5, 20.5, 20.5, 52.0),
        ("positive", 1.0, 21.0, 21.0, 54.0),
        ("wide", 2.0, 22.0, 22.0, 58.0),
    ];

    for (name, spacing, word_width, min_content, max_content) in cases {
        let mut word = paragraph(
            "XX",
            "Ahem",
            10.0,
            10.0,
            WhiteSpaceMode::Normal,
            WordBreakMode::Normal,
        );
        word.style.letter_spacing = spacing;
        let word_sizes = engine.intrinsic_inline_sizes(&word).unwrap();
        close_to(word_sizes.min_content(), word_width);
        close_to(word_sizes.max_content(), word_width);
        let word_layout = engine.layout_paragraph(&word, word_width).unwrap();
        assert_eq!(word_layout.lines.len(), 1, "{name} single-word line count");
        close_to(word_layout.lines[0].advance, word_width);
        close_to(word_layout.width, word_width);

        let mut two_words = paragraph(
            "XX XX",
            "Ahem",
            10.0,
            10.0,
            WhiteSpaceMode::Normal,
            WordBreakMode::Normal,
        );
        two_words.style.letter_spacing = spacing;
        let sizes = engine.intrinsic_inline_sizes(&two_words).unwrap();
        close_to(sizes.min_content(), min_content);
        close_to(sizes.max_content(), max_content);

        let min_layout = engine.layout_paragraph(&two_words, min_content).unwrap();
        assert_eq!(min_layout.lines.len(), 2, "{name} min-content line count");
        assert!(
            min_layout
                .lines
                .iter()
                .all(|line| (line.advance - word_width).abs() < 0.02)
        );
        let max_layout = engine.layout_paragraph(&two_words, max_content).unwrap();
        assert_eq!(max_layout.lines.len(), 1, "{name} max-content line count");
        close_to(max_layout.width, max_content);
    }
}

#[test]
fn ahem_word_spacing_changes_collapsed_separators_but_not_min_content_words() {
    let mut engine = engine();
    let cases = [(-1.0, 49.0, 78.0), (0.0, 50.0, 80.0), (1.0, 51.0, 82.0)];

    for (spacing, two_word_max, three_word_max) in cases {
        let mut two_words = paragraph(
            "XX XX",
            "Ahem",
            10.0,
            10.0,
            WhiteSpaceMode::Normal,
            WordBreakMode::Normal,
        );
        two_words.style.word_spacing = spacing;
        let sizes = engine.intrinsic_inline_sizes(&two_words).unwrap();
        close_to(sizes.min_content(), 20.0);
        close_to(sizes.max_content(), two_word_max);
        close_to(sizes.min_content_height(), 20.0);
        close_to(sizes.max_content_height(), 10.0);

        let min_layout = engine.layout_paragraph(&two_words, 20.0).unwrap();
        assert_eq!(min_layout.lines.len(), 2, "word-spacing {spacing}");
        assert!(
            min_layout
                .lines
                .iter()
                .all(|line| (line.advance - 20.0).abs() < 0.02)
        );
        let max_layout = engine.layout_paragraph(&two_words, two_word_max).unwrap();
        assert_eq!(max_layout.lines.len(), 1, "word-spacing {spacing}");
        close_to(max_layout.width, two_word_max);

        let mut repeated_spaces = paragraph(
            "XX   XX XX  ",
            "Ahem",
            10.0,
            10.0,
            WhiteSpaceMode::Normal,
            WordBreakMode::Normal,
        );
        repeated_spaces.style.word_spacing = spacing;
        let repeated_sizes = engine.intrinsic_inline_sizes(&repeated_spaces).unwrap();
        close_to(repeated_sizes.min_content(), 20.0);
        close_to(repeated_sizes.max_content(), three_word_max);
        close_to(repeated_sizes.min_content_height(), 30.0);
        close_to(repeated_sizes.max_content_height(), 10.0);

        let mut single_word = paragraph(
            "XX",
            "Ahem",
            10.0,
            10.0,
            WhiteSpaceMode::Normal,
            WordBreakMode::Normal,
        );
        single_word.style.word_spacing = spacing;
        let single_sizes = engine.intrinsic_inline_sizes(&single_word).unwrap();
        close_to(single_sizes.min_content(), 20.0);
        close_to(single_sizes.max_content(), 20.0);
    }
}

#[test]
fn ahem_text_indent_changes_first_line_min_and_max_content_advance() {
    let mut engine = engine();
    let mut input = paragraph(
        "XX XX",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    input.style.text_indent = 5.0;

    let sizes = engine.intrinsic_inline_sizes(&input).unwrap();
    close_to(sizes.min_content(), 25.0);
    close_to(sizes.max_content(), 55.0);
    close_to(sizes.min_content_height(), 20.0);
    close_to(sizes.max_content_height(), 10.0);

    let min_layout = engine.layout_paragraph(&input, 25.0).unwrap();
    assert_eq!(min_layout.lines.len(), 2);
    close_to(min_layout.lines[0].advance, 20.0);
    close_to(min_layout.lines[1].advance, 20.0);
    close_to(min_layout.runs[0].glyphs[0].x, 5.0);

    let max_layout = engine.layout_paragraph(&input, 55.0).unwrap();
    assert_eq!(max_layout.lines.len(), 1);
    close_to(max_layout.width, 50.0);
    close_to(max_layout.lines[0].advance, 50.0);
    close_to(max_layout.runs[0].glyphs[0].x, 5.0);
}

#[test]
fn ahem_text_indent_min_content_uses_only_first_unbreakable_line() {
    let mut engine = engine();
    for (text, expected_min, expected_first_line_advance) in
        [("X XXXXX", 50.0, 10.0), ("XXXXX X", 55.0, 50.0)]
    {
        let mut input = paragraph(
            text,
            "Ahem",
            10.0,
            10.0,
            WhiteSpaceMode::Normal,
            WordBreakMode::Normal,
        );
        input.style.text_indent = 5.0;

        let sizes = engine.intrinsic_inline_sizes(&input).unwrap();
        close_to(sizes.min_content(), expected_min);
        close_to(sizes.max_content(), 75.0);

        let min_layout = engine.layout_paragraph(&input, expected_min).unwrap();
        assert_eq!(min_layout.lines.len(), 2);
        close_to(min_layout.lines[0].advance, expected_first_line_advance);
        close_to(min_layout.runs[0].glyphs[0].x, 5.0);
    }
}

#[test]
fn ahem_vertical_mixed_logical_intrinsics_preserve_shaped_inline_advances() {
    let mut engine = engine();
    let cases = [
        ("ABC", "ABC", 30.0, 10.0, 30.0, 10.0, vec!["ABC"]),
        ("A BC", "A BC", 20.0, 20.0, 40.0, 10.0, vec!["A", "BC"]),
        (
            "AB CDE F",
            "AB CDE F",
            30.0,
            30.0,
            80.0,
            10.0,
            vec!["AB", "CDE", "F"],
        ),
        (
            "  A   9Z  ",
            "A 9Z",
            20.0,
            20.0,
            40.0,
            10.0,
            vec!["A", "9Z"],
        ),
        ("   ", "", 0.0, 0.0, 0.0, 0.0, vec![]),
    ];

    for (text, normalized, min_inline, min_block, max_inline, max_block, min_lines) in cases {
        let input = vertical_paragraph(text);
        let sizes = engine.intrinsic_logical_sizes(&input).unwrap();
        assert_eq!(sizes.paragraph(), input.node, "source {text:?}");
        assert_eq!(
            sizes.font_registry_revision(),
            engine.font_registry_revision(),
            "source {text:?}"
        );
        assert_eq!(sizes.writing_mode(), spinon_core::WritingMode::VerticalRl);
        assert_eq!(
            sizes.text_orientation(),
            spinon_core::TextOrientation::Mixed
        );
        close_to(sizes.min_content().inline_px, min_inline);
        close_to(sizes.min_content().block_px, min_block);
        close_to(sizes.max_content().inline_px, max_inline);
        close_to(sizes.max_content().block_px, max_block);

        let at_min_inline = engine.layout_paragraph(&input, min_inline).unwrap();
        let repeated_at_min_inline = engine.layout_paragraph(&input, min_inline).unwrap();
        assert_eq!(at_min_inline, repeated_at_min_inline, "source {text:?}");
        assert_eq!(at_min_inline.line_break, crate::LineBreakMode::Normal);
        assert_eq!(at_min_inline.text, normalized);
        let actual_min_lines = at_min_inline
            .lines
            .iter()
            .map(|line| at_min_inline.text[line.text_range.clone()].trim_matches(' '))
            .collect::<Vec<_>>();
        assert_eq!(actual_min_lines, min_lines, "source {text:?}");

        let at_max_inline = engine.layout_paragraph(&input, max_inline).unwrap();
        let repeated_at_max_inline = engine.layout_paragraph(&input, max_inline).unwrap();
        assert_eq!(at_max_inline, repeated_at_max_inline, "source {text:?}");
        assert_eq!(at_max_inline.line_break, crate::LineBreakMode::Normal);
        assert_eq!(at_max_inline.text, normalized);
        assert_eq!(
            at_max_inline.lines.len(),
            usize::from(!normalized.is_empty())
        );
    }

    let input = vertical_paragraph("ABC");
    let sizes = engine.intrinsic_logical_sizes(&input).unwrap();
    let mut cloned_engine = engine.clone();
    let cloned_sizes = cloned_engine.intrinsic_logical_sizes(&input).unwrap();
    assert!(sizes.uses_same_font_registry_revision(&cloned_sizes));
    assert_eq!(sizes, cloned_sizes);

    let mut independent_engine = TextLayoutEngine::new_without_system_fonts();
    independent_engine.register_font("Ahem", AHEM).unwrap();
    independent_engine.register_font("Roboto", ROBOTO).unwrap();
    let independent_sizes = independent_engine.intrinsic_logical_sizes(&input).unwrap();
    assert_eq!(
        sizes.font_registry_revision(),
        independent_sizes.font_registry_revision()
    );
    assert!(!sizes.uses_same_font_registry_revision(&independent_sizes));
    assert_ne!(sizes, independent_sizes);

    let shaped_at_min_inline = engine.layout_paragraph(&input, 30.0).unwrap();
    assert_eq!(shaped_at_min_inline.lines.len(), 1);
    assert!(
        shaped_at_min_inline
            .runs
            .iter()
            .flat_map(|run| &run.glyphs)
            .all(|glyph| glyph.advance == 10.0)
    );
}

#[test]
fn ahem_vertical_nowrap_intrinsics_have_no_soft_wrap_opportunities() {
    let mut engine = engine();
    let mut input = vertical_paragraph("A BC");
    input.style.white_space = WhiteSpaceMode::NoWrap;

    let sizes = engine.intrinsic_logical_sizes(&input).unwrap();
    close_to(sizes.min_content().inline_px, 40.0);
    close_to(sizes.max_content().inline_px, 40.0);
}

#[test]
fn horizontal_logical_intrinsics_preserve_legacy_measurement_axes() {
    let mut engine = engine();
    let input = paragraph(
        "XX XXXXX",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );

    let legacy = engine.intrinsic_inline_sizes(&input).unwrap();
    let logical = engine.intrinsic_logical_sizes(&input).unwrap();
    assert_eq!(
        logical.writing_mode(),
        spinon_core::WritingMode::HorizontalTb
    );
    assert_eq!(logical.min_content().inline_px, legacy.min_content());
    assert_eq!(logical.min_content().block_px, legacy.min_content_height());
    assert_eq!(logical.max_content().inline_px, legacy.max_content());
    assert_eq!(logical.max_content().block_px, legacy.max_content_height());
}

#[test]
fn vertical_logical_intrinsics_reject_out_of_profile_text_style_and_topology() {
    let mut engine = engine();
    for text in ["", "ABC中", "A-BC", "A.BC", "A\tBC", "A\nBC"] {
        assert_vertical_intrinsic_rejected(&mut engine, &vertical_paragraph(text));
    }

    let mut other_font = vertical_paragraph("ABC");
    other_font.style.font_family = "Roboto".to_owned();
    assert_vertical_intrinsic_rejected(&mut engine, &other_font);

    let mut wrong_font_size = vertical_paragraph("ABC");
    wrong_font_size.style.font_size = 11.0;
    assert_vertical_intrinsic_rejected(&mut engine, &wrong_font_size);

    let mut wrong_line_height = vertical_paragraph("ABC");
    wrong_line_height.style.line_height = 11.0;
    assert_vertical_intrinsic_rejected(&mut engine, &wrong_line_height);

    let mut indented = vertical_paragraph("ABC");
    indented.style.text_indent = 1.0;
    assert_vertical_intrinsic_rejected(&mut engine, &indented);

    let mut normal_line_height = vertical_paragraph("ABC");
    normal_line_height.style.line_height_is_normal = true;
    assert_vertical_intrinsic_rejected(&mut engine, &normal_line_height);

    let mut wrong_weight = vertical_paragraph("ABC");
    wrong_weight.style.font_weight = 500.0;
    assert_vertical_intrinsic_rejected(&mut engine, &wrong_weight);

    let mut wrong_font_style = vertical_paragraph("ABC");
    wrong_font_style.style.font_style = crate::FontStyleMode::Italic;
    assert_vertical_intrinsic_rejected(&mut engine, &wrong_font_style);

    let mut break_all = vertical_paragraph("ABC");
    break_all.style.word_break = WordBreakMode::BreakAll;
    assert_vertical_intrinsic_rejected(&mut engine, &break_all);

    let mut rtl = vertical_paragraph("ABC");
    rtl.style.direction = crate::TextDirection::Rtl;
    assert_vertical_intrinsic_rejected(&mut engine, &rtl);

    let mut strict_line_break = vertical_paragraph("ABC");
    strict_line_break.style.line_break = crate::LineBreakMode::Strict;
    assert_vertical_intrinsic_rejected(&mut engine, &strict_line_break);

    let mut no_ligatures = vertical_paragraph("ABC");
    no_ligatures.style.ligatures = crate::LigatureMode::None;
    assert_vertical_intrinsic_rejected(&mut engine, &no_ligatures);

    let mut with_inline_ancestor = vertical_paragraph("ABC");
    with_inline_ancestor.sources[0]
        .inline_ancestors
        .push(node(3));
    assert_vertical_intrinsic_rejected(&mut engine, &with_inline_ancestor);

    let mut with_inline_style = vertical_paragraph("ABC");
    with_inline_style.inline_styles.insert(
        node(3),
        InlineBoxStyle {
            border: InlineEdges::default(),
            padding: InlineEdges::default(),
        },
    );
    assert_vertical_intrinsic_rejected(&mut engine, &with_inline_style);

    let mut with_inline_run_style = vertical_paragraph("ABC");
    with_inline_run_style.inline_run_styles.insert(
        node(3),
        crate::InlineRunStyle {
            font_family: "Ahem".to_owned(),
            font_size: 10.0,
            line_height: 10.0,
            line_height_is_normal: false,
            font_weight: 400.0,
            font_style: crate::FontStyleMode::Normal,
        },
    );
    assert_vertical_intrinsic_rejected(&mut engine, &with_inline_run_style);

    let mut multiple_sources = vertical_paragraph("A");
    multiple_sources.sources.push(TextSource {
        node: node(4),
        inline_ancestors: Vec::new(),
        text: "BC".to_owned(),
    });
    assert_vertical_intrinsic_rejected(&mut engine, &multiple_sources);
}

#[test]
fn empty_intrinsic_paragraph_returns_zero_sizes_with_current_font_revision() {
    let mut engine = engine();
    let input = paragraph(
        "",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );

    let sizes = engine.intrinsic_inline_sizes(&input).unwrap();

    assert_eq!(sizes.paragraph(), input.node);
    assert_eq!(
        sizes.font_registry_revision(),
        engine.font_registry_revision()
    );
    let mut cloned_engine = engine.clone();
    let cloned_sizes = cloned_engine.intrinsic_inline_sizes(&input).unwrap();
    assert!(sizes.uses_same_font_registry_revision(&cloned_sizes));
    assert_eq!(sizes.min_content(), 0.0);
    assert_eq!(sizes.max_content(), 0.0);
    assert_eq!(sizes.min_content_height(), 0.0);
    assert_eq!(sizes.max_content_height(), 0.0);
}

#[test]
fn intrinsic_inline_sizes_include_inline_edges_and_pre_line_breaks() {
    let mut engine = engine();
    let decorated = |text, white_space, word_break| {
        let mut input = paragraph(text, "Ahem", 10.0, 10.0, white_space, word_break);
        let span = node(3);
        input.sources[0].inline_ancestors = vec![span];
        input.inline_styles.insert(
            span,
            InlineBoxStyle {
                border: InlineEdges {
                    left: 1.0,
                    right: 1.0,
                    ..InlineEdges::default()
                },
                padding: InlineEdges {
                    left: 4.0,
                    right: 4.0,
                    ..InlineEdges::default()
                },
            },
        );
        input
    };
    let normal_decorated = decorated("XX XXXXXXX", WhiteSpaceMode::Normal, WordBreakMode::Normal);
    let normal_sizes = engine.intrinsic_inline_sizes(&normal_decorated).unwrap();
    close_to(normal_sizes.min_content(), 75.0);
    assert_eq!(normal_sizes.min_content_height(), 20.0);
    let nowrap_decorated = decorated("XX XXXXXXX", WhiteSpaceMode::NoWrap, WordBreakMode::Normal);
    let nowrap_sizes = engine.intrinsic_inline_sizes(&nowrap_decorated).unwrap();
    close_to(nowrap_sizes.min_content(), 110.0);
    assert_eq!(nowrap_sizes.min_content_height(), 10.0);
    let break_all_decorated =
        decorated("XXXXXXXX", WhiteSpaceMode::Normal, WordBreakMode::BreakAll);
    let break_all_sizes = engine.intrinsic_inline_sizes(&break_all_decorated).unwrap();
    close_to(break_all_sizes.min_content(), 15.0);
    assert_eq!(break_all_sizes.min_content_height(), 80.0);
    let pre_line_decorated = decorated("XX\nXXXX", WhiteSpaceMode::PreLine, WordBreakMode::Normal);
    let pre_line_decorated_sizes = engine.intrinsic_inline_sizes(&pre_line_decorated).unwrap();
    close_to(pre_line_decorated_sizes.min_content(), 45.0);
    assert_eq!(pre_line_decorated_sizes.min_content_height(), 20.0);

    let pre_line = paragraph(
        "XX\nXXXX",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::PreLine,
        WordBreakMode::Normal,
    );
    let pre_line_sizes = engine.intrinsic_inline_sizes(&pre_line).unwrap();
    close_to(pre_line_sizes.min_content(), 40.0);
    close_to(pre_line_sizes.max_content(), 40.0);
    assert_eq!(pre_line_sizes.min_content_height(), 20.0);
    assert_eq!(pre_line_sizes.max_content_height(), 20.0);

    let pre_line_empty = paragraph(
        "XX\n\nXXXX",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::PreLine,
        WordBreakMode::Normal,
    );
    let pre_line_empty_sizes = engine.intrinsic_inline_sizes(&pre_line_empty).unwrap();
    close_to(pre_line_empty_sizes.min_content(), 40.0);
    assert_eq!(pre_line_empty_sizes.min_content_height(), 30.0);
    assert_eq!(pre_line_empty_sizes.max_content_height(), 30.0);
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
    let normalized = crate::normalize::normalize(&input).unwrap();

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

#[path = "tests/c15_3.rs"]
mod c15_3;

#[path = "tests/c15_3_raster.rs"]
mod c15_3_raster;

#[path = "tests/paragraph_style.rs"]
mod paragraph_style;

#[path = "tests/c11_4_break_spaces.rs"]
mod c11_4_break_spaces;
#[path = "tests/c11_4_line_break.rs"]
mod c11_4_line_break;

#[path = "tests/c11_4_overflow_wrap.rs"]
mod c11_4_overflow_wrap;
#[path = "tests/c11_4_pre.rs"]
mod c11_4_pre;
#[path = "tests/c11_4_pre_wrap.rs"]
mod c11_4_pre_wrap;
#[path = "tests/c11_4_text_transform.rs"]
mod c11_4_text_transform;
#[path = "tests/c11_4_word_break.rs"]
mod c11_4_word_break;
