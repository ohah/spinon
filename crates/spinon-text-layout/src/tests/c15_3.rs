use crate::{
    LigatureMode, LineBreakMode, TextDirection, TextLayoutError, TextSource, WhiteSpaceMode,
    WordBreakMode,
};

use super::{close_to, node, paragraph};

const NOTO_KR: &[u8] =
    include_bytes!("../../../../tests/fixtures/resources/c15/c15-3/NotoSansKR-VF.ttf");
const NOTO_ARABIC: &[u8] =
    include_bytes!("../../../../tests/fixtures/resources/c15/c15-3/NotoSansArabic-VF.ttf");
const NOTO_EMOJI: &[u8] =
    include_bytes!("../../../../tests/fixtures/resources/c15/c15-3/NotoEmoji-VF.ttf");
const ROBOTO: &[u8] =
    include_bytes!("../../../../tests/fixtures/resources/c15/Roboto-Variable.ttf");
const AHEM: &[u8] = include_bytes!("../../../../tests/fixtures/resources/c15/Ahem.ttf");

fn font_engine(family: &str, bytes: &[u8]) -> crate::TextLayoutEngine {
    let mut engine = crate::TextLayoutEngine::new_without_system_fonts();
    engine.register_font(family, bytes).unwrap();
    engine
}

#[test]
fn hangul_uses_the_registered_korean_font_and_emits_real_glyphs() {
    let mut engine = font_engine("Noto Sans KR", NOTO_KR);
    let input = paragraph(
        "한글 테스트",
        "Noto Sans KR",
        20.0,
        24.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    let layout = engine.layout_paragraph(&input, 320.0).unwrap();

    assert_eq!(layout.text, "한글 테스트");
    assert_eq!(layout.lines.len(), 1);
    // 이 값은 shaper advance입니다. Chrome의 97.609375px inline-block rect에는
    // inline formatting geometry가 포함되므로 후속 layout 소비처에서 확인합니다.
    close_to(layout.width, 96.470_17);
    close_to(layout.height, 24.0);
    let glyphs = layout
        .runs
        .iter()
        .flat_map(|run| &run.glyphs)
        .collect::<Vec<_>>();
    assert_eq!(glyphs.len(), 6);
    assert!(glyphs.iter().all(|glyph| glyph.id != 0));
    assert!(glyphs.iter().all(|glyph| !glyph.sources.is_empty()));
}

#[test]
fn strict_cjk_break_keeps_closing_punctuation_with_the_previous_cluster() {
    let mut engine = font_engine("Noto Sans KR", NOTO_KR);
    let mut input = paragraph(
        "甲乙、丙丁",
        "Noto Sans KR",
        20.0,
        24.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    input.style.line_break = LineBreakMode::Strict;
    let layout = engine.layout_paragraph(&input, 40.0).unwrap();

    assert_eq!(layout.direction, TextDirection::Auto);
    assert_eq!(layout.line_break, LineBreakMode::Strict);
    assert_eq!(
        layout
            .lines
            .iter()
            .map(|line| layout.text[line.text_range.clone()].to_owned())
            .collect::<Vec<_>>(),
        ["甲", "乙、", "丙丁"]
    );
    close_to(layout.width, 40.0);
    close_to(layout.height, 72.0);
}

#[test]
fn combining_sequence_glyphs_retain_one_grapheme_and_the_full_utf16_source() {
    let mut engine = font_engine("Roboto", ROBOTO);
    let input = paragraph(
        "x\u{0323}",
        "Roboto",
        20.0,
        24.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    let layout = engine.layout_paragraph(&input, 100.0).unwrap();
    let glyphs = layout
        .runs
        .iter()
        .flat_map(|run| &run.glyphs)
        .collect::<Vec<_>>();

    assert!(!glyphs.is_empty());
    close_to(layout.width, 9.921875);
    for glyph in glyphs {
        assert_eq!(glyph.cluster_text_range, 0..3);
        assert_eq!(glyph.sources.len(), 1);
        assert_eq!(glyph.sources[0].node, node(2));
        assert_eq!(glyph.sources[0].source_utf16, 0..2);
        assert_eq!(glyph.sources[0].paragraph_utf8, 0..3);
    }
}

#[test]
fn ligature_mode_changes_shaping_and_keeps_the_full_ligature_source_range() {
    let mut engine = font_engine("Roboto", ROBOTO);
    let input = paragraph(
        "office",
        "Roboto",
        20.0,
        24.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    let ligatures = engine.layout_paragraph(&input, 200.0).unwrap();
    let mut no_ligatures_input = input.clone();
    no_ligatures_input.style.ligatures = LigatureMode::None;
    let no_ligatures = engine.layout_paragraph(&no_ligatures_input, 200.0).unwrap();

    assert_eq!(ligatures.ligatures, LigatureMode::Normal);
    assert_eq!(no_ligatures.ligatures, LigatureMode::None);
    close_to(ligatures.width, 49.5625);
    close_to(no_ligatures.width, 51.25);
    assert!(
        ligatures
            .runs
            .iter()
            .flat_map(|run| &run.glyphs)
            .any(|glyph| {
                glyph.cluster_text_range == (1..4)
                    && glyph.sources.len() == 1
                    && glyph.sources[0].source_utf16 == (1..4)
            })
    );
    assert!(
        no_ligatures
            .runs
            .iter()
            .flat_map(|run| &run.glyphs)
            .all(|glyph| glyph.cluster_text_range != (1..4))
    );
}

#[test]
fn arabic_rtl_run_keeps_european_digits_in_ltr_order() {
    let mut engine = font_engine("Noto Sans Arabic", NOTO_ARABIC);
    let mut input = paragraph(
        "السَّلَامُ 123",
        "Noto Sans Arabic",
        20.0,
        28.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    input.style.direction = TextDirection::Rtl;
    let layout = engine.layout_paragraph(&input, 140.0).unwrap();

    assert_eq!(layout.direction, TextDirection::Rtl);
    assert_eq!(layout.lines.len(), 1);
    close_to(layout.height, 28.0);
    close_to(layout.width, 88.139_92);
    close_to(layout.lines[0].baseline, 20.0);
    assert!(
        layout
            .runs
            .iter()
            .any(|run| run.is_rtl && &layout.text[run.text_range.clone()] != "123")
    );
    let digit_run = layout
        .runs
        .iter()
        .find(|run| &layout.text[run.text_range.clone()] == "123")
        .expect("European digits form a separate LTR bidi run");
    assert!(!digit_run.is_rtl);
    let digit_glyphs = digit_run.glyphs.iter().collect::<Vec<_>>();
    assert_eq!(digit_glyphs.len(), 3);
    assert!(digit_glyphs.windows(2).all(|pair| pair[0].x < pair[1].x));
    for (index, glyph) in digit_glyphs.into_iter().enumerate() {
        assert_eq!(glyph.sources.len(), 1);
        assert_eq!(glyph.sources[0].source_utf16, (11 + index)..(12 + index));
    }
    let arabic_run = layout
        .runs
        .iter()
        .find(|run| run.is_rtl)
        .expect("Arabic letters use an RTL shaping run");
    assert!(
        arabic_run
            .glyphs
            .windows(2)
            .all(|pair| pair[0].cluster_text_range.start >= pair[1].cluster_text_range.start)
    );
    assert!(
        arabic_run
            .glyphs
            .iter()
            .all(|glyph| !glyph.sources.is_empty())
    );
    assert!(arabic_run.glyphs.iter().any(|glyph| {
        glyph
            .sources
            .iter()
            .any(|source| source.source_utf16.end - source.source_utf16.start > 1)
    }));
}

#[test]
fn auto_rtl_and_ltr_base_directions_place_an_arabic_paragraph_correctly() {
    let mut engine = font_engine("Noto Sans Arabic", NOTO_ARABIC);
    let mut input = paragraph(
        "السَّلَامُ",
        "Noto Sans Arabic",
        20.0,
        28.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    input.style.line_break = LineBreakMode::Normal;

    let auto = engine.layout_paragraph(&input, 140.0).unwrap();
    assert_eq!(auto.direction, TextDirection::Auto);
    assert_eq!(auto.line_break, LineBreakMode::Normal);
    let auto_left = auto
        .runs
        .iter()
        .flat_map(|run| &run.glyphs)
        .map(|glyph| glyph.x)
        .fold(f32::INFINITY, f32::min);
    assert!(auto.runs.iter().all(|run| run.is_rtl));

    input.style.direction = TextDirection::Rtl;
    let rtl = engine.layout_paragraph(&input, 140.0).unwrap();
    let rtl_left = rtl
        .runs
        .iter()
        .flat_map(|run| &run.glyphs)
        .map(|glyph| glyph.x)
        .fold(f32::INFINITY, f32::min);
    close_to(auto_left, rtl_left);

    input.style.direction = TextDirection::Ltr;
    let ltr = engine.layout_paragraph(&input, 140.0).unwrap();
    let ltr_left = ltr
        .runs
        .iter()
        .flat_map(|run| &run.glyphs)
        .map(|glyph| glyph.x)
        .fold(f32::INFINITY, f32::min);
    assert_eq!(ltr.direction, TextDirection::Ltr);
    assert!(ltr.runs.iter().all(|run| run.is_rtl));
    close_to(ltr_left, 0.0);
    assert!(ltr_left < auto_left);
}

#[test]
fn emoji_zwj_sequence_is_one_shaped_cluster_with_a_full_utf16_range() {
    let mut engine = font_engine("Noto Emoji", NOTO_EMOJI);
    let input = paragraph(
        "👩‍💻",
        "Noto Emoji",
        28.0,
        36.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    let layout = engine.layout_paragraph(&input, 100.0).unwrap();
    let glyphs = layout
        .runs
        .iter()
        .flat_map(|run| &run.glyphs)
        .collect::<Vec<_>>();

    assert_eq!(glyphs.len(), 1);
    assert_ne!(glyphs[0].id, 0);
    assert_eq!(glyphs[0].cluster_text_range, 0..11);
    assert_eq!(glyphs[0].sources.len(), 1);
    assert_eq!(glyphs[0].sources[0].source_utf16, 0..5);
    assert_eq!(glyphs[0].sources[0].paragraph_utf8, 0..11);
    close_to(layout.width, 35.546875);
    close_to(layout.height, 36.0);
}

#[test]
fn one_combining_cluster_can_preserve_sources_from_adjacent_dom_text_nodes() {
    let mut engine = font_engine("Roboto", ROBOTO);
    let mut input = paragraph(
        "",
        "Roboto",
        20.0,
        24.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    input.sources = vec![
        TextSource {
            node: node(2),
            inline_ancestors: Vec::new(),
            text: "x".to_owned(),
        },
        TextSource {
            node: node(3),
            inline_ancestors: Vec::new(),
            text: "\u{0323}".to_owned(),
        },
    ];
    let layout = engine.layout_paragraph(&input, 100.0).unwrap();
    let glyph = layout.runs[0]
        .glyphs
        .first()
        .expect("combining cluster has a shaped glyph");

    assert_eq!(glyph.cluster_text_range, 0..3);
    assert_eq!(glyph.sources.len(), 2);
    assert_eq!(glyph.sources[0].node, node(2));
    assert_eq!(glyph.sources[0].source_utf16, 0..1);
    assert_eq!(glyph.sources[0].paragraph_utf8, 0..1);
    assert_eq!(glyph.sources[1].node, node(3));
    assert_eq!(glyph.sources[1].source_utf16, 0..1);
    assert_eq!(glyph.sources[1].paragraph_utf8, 1..3);
}

#[test]
fn missing_fixture_glyph_does_not_fall_back_to_a_system_font_or_notdef_glyph() {
    let mut engine = crate::TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).unwrap();
    let input = paragraph(
        "🦦",
        "Ahem",
        20.0,
        24.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );

    match engine.layout_paragraph(&input, 100.0) {
        Err(TextLayoutError::MissingGlyphCoverage {
            paragraph,
            text_range,
        }) => {
            assert_eq!(paragraph, node(1));
            assert_eq!(text_range, 0..4);
        }
        result => panic!("expected missing glyph coverage error, got {result:?}"),
    }
}
