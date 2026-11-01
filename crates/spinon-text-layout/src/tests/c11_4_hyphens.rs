use crate::{
    FontStyleMode, HyphensMode, LigatureMode, LineBreakMode, OverflowWrapMode, ParagraphInput,
    ParagraphStyle, TextDirection, TextLayoutEngine, TextLayoutError, TextSource,
    TextTransformMode, WhiteSpaceMode, WordBreakMode,
};

const AHEM: &[u8] = include_bytes!("../../../../tests/fixtures/resources/c15/Ahem.ttf");

fn engine() -> TextLayoutEngine {
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).unwrap();
    engine
}

fn paragraph(text: &str, hyphens: HyphensMode) -> ParagraphInput {
    ParagraphInput {
        node: spinon_core::NodeId::new(1).unwrap(),
        sources: vec![TextSource {
            node: spinon_core::NodeId::new(2).unwrap(),
            inline_ancestors: Vec::new(),
            text: text.to_owned(),
        }],
        style: ParagraphStyle {
            font_family: "Ahem".to_owned(),
            font_size: 10.0,
            line_height: 10.0,
            line_height_is_normal: false,
            font_weight: 400.0,
            font_style: FontStyleMode::Normal,
            white_space: WhiteSpaceMode::Normal,
            word_break: WordBreakMode::Normal,
            hyphens: Some(hyphens),
            overflow_wrap: OverflowWrapMode::Normal,
            direction: TextDirection::Ltr,
            line_break: LineBreakMode::Normal,
            ligatures: LigatureMode::Normal,
            text_transform: TextTransformMode::None,
            locale: Some("en".to_owned()),
            ..ParagraphStyle::default()
        },
        inline_styles: Default::default(),
        inline_run_styles: Default::default(),
    }
}

fn close_to(actual: f32, expected: f32, label: &str) {
    assert!(
        (actual - expected).abs() <= 0.05,
        "{label}: expected {expected}, got {actual} CSS px"
    );
}

#[test]
fn c114_hyphens_none_suppresses_soft_hyphen_but_keeps_source_span() {
    let mut engine = engine();
    let input = paragraph("AB\u{00ad}CD", HyphensMode::None);
    let intrinsic = engine.intrinsic_inline_sizes(&input).unwrap();
    close_to(intrinsic.min_content(), 40.0, "none min-content");
    close_to(intrinsic.max_content(), 40.0, "none max-content");
    close_to(
        intrinsic.min_content_height(),
        10.0,
        "none min-content height",
    );

    let layout = engine.layout_paragraph(&input, 30.0).unwrap();
    assert_eq!(layout.text, "AB\u{2060}CD");
    assert_eq!(layout.lines.len(), 1);
    close_to(layout.lines[0].advance, 40.0, "none unbroken line advance");
    let shy_source = layout
        .sources
        .iter()
        .find(|source| source.source_utf16 == (2..3))
        .expect("soft-hyphen source mapping");
    assert_eq!(shy_source.paragraph_utf8, 2..5);
}

#[test]
fn c114_hyphens_manual_includes_discretionary_glyph_in_min_and_line_advance() {
    let mut engine = engine();
    let input = paragraph("AB\u{00ad}CD", HyphensMode::Manual);
    let intrinsic = engine.intrinsic_inline_sizes(&input).unwrap();
    close_to(intrinsic.min_content(), 30.0, "manual min-content");
    close_to(intrinsic.max_content(), 40.0, "manual max-content");
    close_to(
        intrinsic.min_content_height(),
        20.0,
        "manual min-content height",
    );
    close_to(
        intrinsic.max_content_height(),
        10.0,
        "manual max-content height",
    );

    let layout = engine.layout_paragraph(&input, 30.0).unwrap();
    assert_eq!(layout.text, "AB\u{00ad}CD");
    assert_eq!(layout.lines.len(), 2);
    assert_eq!(layout.lines[0].text_range, 0..4);
    close_to(
        layout.lines[0].advance,
        30.0,
        "manual hyphenated line advance",
    );
    close_to(layout.lines[1].advance, 20.0, "manual continuation advance");
    close_to(layout.width, 30.0, "manual paragraph width");
    close_to(layout.height, 20.0, "manual paragraph height");
    let shy_source = layout
        .sources
        .iter()
        .find(|source| source.source_utf16 == (2..3))
        .expect("soft-hyphen source mapping");
    assert_eq!(shy_source.paragraph_utf8, 2..4);
}

#[test]
fn c114_hyphens_manual_measures_multiple_opportunities_and_visible_hyphen() {
    let mut engine = engine();
    for (text, expected_min, expected_max) in [
        ("A\u{00ad}B\u{00ad}CD", 20.0, 40.0),
        ("AB-CD", 30.0, 50.0),
        ("ABCD", 40.0, 40.0),
    ] {
        let input = paragraph(text, HyphensMode::Manual);
        let sizes = engine.intrinsic_inline_sizes(&input).unwrap();
        close_to(sizes.min_content(), expected_min, &format!("{text:?} min"));
        close_to(sizes.max_content(), expected_max, &format!("{text:?} max"));
    }

    let none_visible_hyphen = engine
        .intrinsic_inline_sizes(&paragraph("AB-CD", HyphensMode::None))
        .unwrap();
    close_to(
        none_visible_hyphen.min_content(),
        30.0,
        "none visible-hyphen min",
    );
}

#[test]
fn c114_hyphens_rejects_inputs_outside_the_fixed_profile() {
    let mut engine = engine();
    let mut input = paragraph("AB\u{00ad}CD", HyphensMode::Manual);
    input.style.font_size = 12.0;
    assert!(matches!(
        engine.intrinsic_inline_sizes(&input),
        Err(TextLayoutError::UnsupportedHyphensProfile(node)) if node == input.node
    ));

    let mut input = paragraph("AB\u{00ad}CD", HyphensMode::None);
    input.style.text_transform = TextTransformMode::Uppercase;
    assert!(matches!(
        engine.layout_paragraph(&input, 30.0),
        Err(TextLayoutError::UnsupportedHyphensProfile(node)) if node == input.node
    ));
}

#[test]
fn c114_hyphens_auto_is_rejected_without_its_fixture_feature() {
    if cfg!(feature = "c11-4-hyphens-auto-fixture-test") {
        return;
    }
    let mut engine = engine();
    let input = paragraph("hyphenation", HyphensMode::Auto);
    assert!(matches!(
        engine.intrinsic_inline_sizes(&input),
        Err(TextLayoutError::UnsupportedHyphensProfile(node)) if node == input.node
    ));
}

#[cfg(feature = "c11-4-hyphens-auto-fixture-test")]
#[test]
fn c114_hyphens_auto_matches_fixed_english_intrinsic_and_line_oracles() {
    let mut engine = engine();
    for (text, expected_min, expected_max, expected_advances) in [
        ("hyphenation", 50.0, 110.0, &[30.0, 50.0, 50.0][..]),
        (
            "internationalization",
            50.0,
            200.0,
            &[30.0, 40.0, 30.0, 50.0, 30.0, 40.0, 40.0][..],
        ),
        (
            "representation",
            40.0,
            140.0,
            &[40.0, 30.0, 40.0, 30.0, 40.0][..],
        ),
    ] {
        let input = paragraph(text, HyphensMode::Auto);
        let intrinsic = engine.intrinsic_inline_sizes(&input).unwrap();
        close_to(intrinsic.min_content(), expected_min, "auto min-content");
        close_to(intrinsic.max_content(), expected_max, "auto max-content");

        let layout = engine.layout_paragraph(&input, 50.0).unwrap();
        let expected_text = match text {
            "hyphenation" => "hy\u{00ad}phen\u{00ad}ation",
            "internationalization" => {
                "in\u{00ad}ter\u{00ad}na\u{00ad}tion\u{00ad}al\u{00ad}iza\u{00ad}tion"
            }
            "representation" => "rep\u{00ad}re\u{00ad}sen\u{00ad}ta\u{00ad}tion",
            _ => unreachable!(),
        };
        assert_eq!(layout.text, expected_text);
        assert_eq!(
            layout.lines.len(),
            expected_advances.len(),
            "{text}: lines={:?}, advances={:?}",
            layout
                .lines
                .iter()
                .map(|line| &layout.text[line.text_range.clone()])
                .collect::<Vec<_>>(),
            layout
                .lines
                .iter()
                .map(|line| line.advance)
                .collect::<Vec<_>>()
        );
        for (line, expected) in layout.lines.iter().zip(expected_advances) {
            close_to(line.advance, *expected, "auto visible line advance");
        }

        let source_utf16_len = text.encode_utf16().count();
        assert!(
            layout
                .sources
                .iter()
                .all(|source| source.source_utf16.end <= source_utf16_len)
        );
        for marker_start in layout
            .text
            .match_indices('\u{00ad}')
            .map(|(index, _)| index)
        {
            let marker_end = marker_start + '\u{00ad}'.len_utf8();
            let marker_sources = layout
                .sources
                .iter()
                .filter(|source| source.paragraph_utf8 == (marker_start..marker_end))
                .collect::<Vec<_>>();
            assert_eq!(marker_sources.len(), 1);
            assert_eq!(
                marker_sources[0].source_utf16.start,
                marker_sources[0].source_utf16.end
            );
        }
    }
}

#[cfg(feature = "c11-4-hyphens-auto-fixture-test")]
#[test]
fn c114_hyphens_auto_fails_closed_for_language_word_and_soft_hyphen_outside_profile() {
    let mut engine = engine();
    for (text, locale) in [
        ("hyphenation", None),
        ("hyphenation", Some("en-US")),
        ("hyphenation", Some("fr")),
        ("unsupported", Some("en")),
        ("hy\u{00ad}phenation", Some("en")),
    ] {
        let mut input = paragraph(text, HyphensMode::Auto);
        input.style.locale = locale.map(str::to_owned);
        assert!(
            matches!(
                engine.intrinsic_inline_sizes(&input),
                Err(TextLayoutError::UnsupportedHyphensProfile(node)) if node == input.node
            ),
            "unexpectedly accepted {text:?} with {locale:?}"
        );
    }
}
