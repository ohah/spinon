use crate::{
    LigatureMode, LineBreakMode, ParagraphInput, ParagraphStyle, TextDirection, TextLayoutEngine,
    TextSource, WhiteSpaceMode, WordBreakMode,
};

const AHEM: &[u8] = include_bytes!("../../../../tests/fixtures/resources/c15/Ahem.ttf");
const NOTO_KR: &[u8] =
    include_bytes!("../../../../tests/fixtures/resources/c15/c15-3/NotoSansKR-VF.ttf");

fn engine() -> TextLayoutEngine {
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).unwrap();
    engine.register_font("Noto Sans KR", NOTO_KR).unwrap();
    engine
}

fn paragraph(text: &str, font_family: &str, word_break: WordBreakMode) -> ParagraphInput {
    paragraph_with_locale(text, font_family, word_break, None)
}

fn paragraph_with_locale(
    text: &str,
    font_family: &str,
    word_break: WordBreakMode,
    locale: Option<&str>,
) -> ParagraphInput {
    ParagraphInput {
        node: spinon_core::NodeId::new(1).unwrap(),
        sources: vec![TextSource {
            node: spinon_core::NodeId::new(2).unwrap(),
            inline_ancestors: Vec::new(),
            text: text.to_owned(),
        }],
        style: ParagraphStyle {
            font_family: font_family.to_owned(),
            font_size: 10.0,
            line_height: 10.0,
            line_height_is_normal: false,
            font_weight: 400.0,
            font_style: crate::FontStyleMode::Normal,
            white_space: WhiteSpaceMode::Normal,
            word_break,
            locale: locale.map(str::to_owned),
            direction: TextDirection::Ltr,
            line_break: LineBreakMode::Strict,
            ligatures: LigatureMode::Normal,
            ..ParagraphStyle::default()
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
fn ahem_intrinsic_sizes_distinguish_normal_and_break_all() {
    let mut engine = engine();
    for (text, normal_min, normal_min_height, break_all_min, max) in [
        ("XXXX", 40.0, 10.0, 10.0, 40.0),
        ("XX XX", 20.0, 20.0, 10.0, 50.0),
    ] {
        let normal = engine
            .intrinsic_inline_sizes(&paragraph(text, "Ahem", WordBreakMode::Normal))
            .unwrap();
        let break_all = engine
            .intrinsic_inline_sizes(&paragraph(text, "Ahem", WordBreakMode::BreakAll))
            .unwrap();

        close_to(normal.min_content(), normal_min);
        close_to(normal.min_content_height(), normal_min_height);
        close_to(normal.max_content(), max);
        close_to(normal.max_content_height(), 10.0);
        close_to(break_all.min_content(), break_all_min);
        close_to(break_all.min_content_height(), 40.0);
        close_to(break_all.max_content(), max);
        close_to(break_all.max_content_height(), 10.0);
    }
}

#[test]
fn korean_word_break_changes_min_content_and_locale_is_metadata() {
    let mut engine = engine();
    let normal = engine
        .intrinsic_inline_sizes(&paragraph_with_locale(
            "가나다라",
            "Noto Sans KR",
            WordBreakMode::Normal,
            Some("ko"),
        ))
        .unwrap();
    let normal_without_locale = engine
        .intrinsic_inline_sizes(&paragraph(
            "가나다라",
            "Noto Sans KR",
            WordBreakMode::Normal,
        ))
        .unwrap();
    let keep_all = engine
        .intrinsic_inline_sizes(&paragraph_with_locale(
            "가나다라",
            "Noto Sans KR",
            WordBreakMode::KeepAll,
            Some("ko"),
        ))
        .unwrap();

    close_to(normal.min_content(), 9.2);
    close_to(normal.min_content_height(), 40.0);
    close_to(normal.max_content(), 36.8);
    close_to(normal.max_content_height(), 10.0);
    close_to(normal_without_locale.min_content(), normal.min_content());
    close_to(
        normal_without_locale.min_content_height(),
        normal.min_content_height(),
    );
    close_to(normal_without_locale.max_content(), normal.max_content());
    close_to(
        normal_without_locale.max_content_height(),
        normal.max_content_height(),
    );
    close_to(keep_all.min_content(), 36.8);
    close_to(keep_all.min_content_height(), 10.0);
    close_to(keep_all.max_content(), 36.8);
    close_to(keep_all.max_content_height(), 10.0);
}

#[test]
fn missing_locale_keeps_default_and_invalid_locale_is_rejected() {
    let mut input = paragraph("가나다라", "Noto Sans KR", WordBreakMode::Normal);
    assert_eq!(input.style.locale, None);
    input.style.font_size = 10.0;
    assert!(input.style.validate().is_ok());

    for invalid in ["", " ko", "ko ", "ko KR"] {
        input.style.locale = Some(invalid.to_owned());
        assert_eq!(input.style.validate(), Err("locale"), "{invalid:?}");
    }
    for valid in ["ko", "ko-KR", "en-US"] {
        input.style.locale = Some(valid.to_owned());
        assert!(input.style.validate().is_ok(), "{valid:?}");
    }
}
