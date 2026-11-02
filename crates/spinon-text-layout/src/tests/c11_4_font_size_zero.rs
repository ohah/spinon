use crate::{
    LigatureMode, LineBreakMode, ParagraphInput, ParagraphStyle, TextDirection, TextLayoutEngine,
    TextSource, WhiteSpaceMode, WordBreakMode,
};

const AHEM: &[u8] = include_bytes!("../../../../tests/fixtures/resources/c15/Ahem.ttf");

fn paragraph(text: &str, font_size: f32) -> ParagraphInput {
    ParagraphInput {
        node: spinon_core::NodeId::new(1).unwrap(),
        sources: vec![TextSource {
            node: spinon_core::NodeId::new(2).unwrap(),
            inline_ancestors: Vec::new(),
            text: text.to_owned(),
        }],
        style: ParagraphStyle {
            font_family: "Ahem".to_owned(),
            font_size,
            line_height: 10.0,
            line_height_is_normal: false,
            font_weight: 400.0,
            font_style: crate::FontStyleMode::Normal,
            white_space: WhiteSpaceMode::Normal,
            word_break: WordBreakMode::Normal,
            direction: TextDirection::Ltr,
            line_break: LineBreakMode::Strict,
            ligatures: LigatureMode::Normal,
            ..ParagraphStyle::default()
        },
        inline_styles: Default::default(),
        inline_run_styles: Default::default(),
    }
}

#[test]
fn paragraph_style_accepts_finite_zero_but_rejects_negative_and_non_finite_font_sizes() {
    for font_size in [0.0, -0.0] {
        assert_eq!(paragraph("X", font_size).style.validate(), Ok(()));
    }
    for font_size in [-f32::EPSILON, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert_eq!(paragraph("X", font_size).style.validate(), Err("font-size"));
    }
}

#[test]
fn c114_zero_font_size_keeps_zero_inline_contribution_and_absolute_line_height() {
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).unwrap();
    let input = paragraph("XX XX", 0.0);

    let intrinsic = engine.intrinsic_inline_sizes(&input).unwrap();
    assert_eq!(intrinsic.min_content(), 0.0);
    assert_eq!(intrinsic.max_content(), 0.0);
    assert_eq!(intrinsic.min_content_height(), 10.0);
    assert_eq!(intrinsic.max_content_height(), 10.0);

    let layout = engine.layout_paragraph(&input, 100.0).unwrap();
    assert_eq!(layout.text, "XX XX");
    assert_eq!(layout.lines.len(), 1);
    assert_eq!(layout.width, 0.0);
    assert_eq!(layout.height, 10.0);
    assert_eq!(layout.lines[0].advance, 0.0);
    assert_eq!(layout.lines[0].line_height, 10.0);
    assert!(layout.lines[0].baseline.is_finite());
}
