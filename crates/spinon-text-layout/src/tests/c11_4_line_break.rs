use crate::{
    FontStyleMode, LigatureMode, LineBreakMode, ParagraphInput, ParagraphStyle, TextDirection,
    TextLayoutEngine, TextSource, WhiteSpaceMode, WordBreakMode,
};

const NOTO_KR: &[u8] =
    include_bytes!("../../../../tests/fixtures/resources/c15/c15-3/NotoSansKR-VF.ttf");

fn paragraph(language: &str, line_break: LineBreakMode) -> ParagraphInput {
    paragraph_with_text(language, line_break, "文〜文")
}

fn paragraph_with_text(language: &str, line_break: LineBreakMode, text: &str) -> ParagraphInput {
    ParagraphInput {
        node: spinon_core::NodeId::new(1).unwrap(),
        sources: vec![TextSource {
            node: spinon_core::NodeId::new(2).unwrap(),
            inline_ancestors: Vec::new(),
            text: text.to_owned(),
        }],
        style: ParagraphStyle {
            overflow_wrap: Default::default(),
            font_family: "Noto Sans KR".to_owned(),
            font_size: 10.0,
            line_height: 10.0,
            line_height_is_normal: false,
            font_weight: 400.0,
            font_style: FontStyleMode::Normal,
            white_space: WhiteSpaceMode::Normal,
            word_break: WordBreakMode::Normal,
            locale: Some(language.to_owned()),
            direction: TextDirection::Ltr,
            line_break,
            ligatures: LigatureMode::Normal,
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
fn c114_line_break_uses_language_and_mode_for_intrinsic_inline_and_block_sizes() {
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Noto Sans KR", NOTO_KR).unwrap();
    for (language, mode, min_inline, max_inline, min_block, max_block) in [
        ("ja", LineBreakMode::Normal, 10.0, 30.0, 30.0, 10.0),
        ("ja", LineBreakMode::Strict, 20.0, 30.0, 20.0, 10.0),
        ("en", LineBreakMode::Normal, 20.0, 30.0, 20.0, 10.0),
    ] {
        let input = paragraph(language, mode);
        let sizes = engine.intrinsic_inline_sizes(&input).unwrap();
        let label = format!("lang={language}; line-break={mode:?}");
        close_to(
            sizes.min_content(),
            min_inline,
            &format!("{label} min inline"),
        );
        close_to(
            sizes.max_content(),
            max_inline,
            &format!("{label} max inline"),
        );
        close_to(
            sizes.min_content_height(),
            min_block,
            &format!("{label} min block"),
        );
        close_to(
            sizes.max_content_height(),
            max_block,
            &format!("{label} max block"),
        );
    }
}

#[test]
fn c114_line_break_rejects_malformed_language_metadata() {
    let mut input = paragraph("ja", LineBreakMode::Normal);
    input.style.locale = None;
    assert!(input.style.validate().is_ok());
    for invalid in ["", " ja", "ja ", "ja JP"] {
        input.style.locale = Some(invalid.to_owned());
        assert_eq!(input.style.validate(), Err("locale"), "{invalid:?}");
    }
}

#[test]
fn c114_line_break_loose_allows_japanese_prefix_break_for_intrinsic_sizes() {
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Noto Sans KR", NOTO_KR).unwrap();
    for (language, mode, min_inline, min_block) in [
        ("ja", LineBreakMode::Loose, 10.0, 30.0),
        ("en", LineBreakMode::Loose, 20.0, 20.0),
        ("ja", LineBreakMode::Normal, 20.0, 20.0),
        ("ja", LineBreakMode::Strict, 20.0, 20.0),
    ] {
        let input = paragraph_with_text(language, mode, "文＄文");
        let sizes = engine.intrinsic_inline_sizes(&input).unwrap();
        let label = format!("lang={language}; line-break={mode:?}; text=文＄文");
        close_to(
            sizes.min_content(),
            min_inline,
            &format!("{label} min inline"),
        );
        close_to(sizes.max_content(), 30.0, &format!("{label} max inline"));
        close_to(
            sizes.min_content_height(),
            min_block,
            &format!("{label} min block"),
        );
        close_to(
            sizes.max_content_height(),
            10.0,
            &format!("{label} max block"),
        );
    }
}
