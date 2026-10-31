use spinon_core::NodeId;

use crate::{
    ParagraphInput, ParagraphStyle, TextLayoutEngine, TextLayoutError, TextSource, WhiteSpaceMode,
    WordBreakMode,
};

const AHEM: &[u8] = include_bytes!("../../../../tests/fixtures/resources/c15/Ahem.ttf");

#[test]
fn paragraph_style_default_keeps_the_legacy_horizontal_text_profile() {
    let style = ParagraphStyle::default();

    assert_eq!(style.writing_mode, spinon_core::WritingMode::HorizontalTb);
    assert_eq!(style.text_orientation, spinon_core::TextOrientation::Mixed);
}

#[test]
fn paragraph_style_rejects_invalid_family_and_negative_or_non_finite_metrics() {
    let node = NodeId::new(1).unwrap();
    let source = NodeId::new(2).unwrap();
    let cases = [
        (
            ParagraphStyle {
                writing_mode: spinon_core::WritingMode::HorizontalTb,
                text_orientation: spinon_core::TextOrientation::Mixed,
                font_family: "  ".to_owned(),
                font_size: 10.0,
                line_height: 10.0,
                white_space: WhiteSpaceMode::Normal,
                word_break: WordBreakMode::Normal,
                direction: crate::TextDirection::Auto,
                line_break: crate::LineBreakMode::Strict,
                ligatures: crate::LigatureMode::Normal,
                ..ParagraphStyle::default()
            },
            "font-family",
        ),
        (
            ParagraphStyle {
                writing_mode: spinon_core::WritingMode::HorizontalTb,
                text_orientation: spinon_core::TextOrientation::Mixed,
                font_family: "Ahem".to_owned(),
                font_size: 0.0,
                line_height: 10.0,
                white_space: WhiteSpaceMode::Normal,
                word_break: WordBreakMode::Normal,
                direction: crate::TextDirection::Auto,
                line_break: crate::LineBreakMode::Strict,
                ligatures: crate::LigatureMode::Normal,
                ..ParagraphStyle::default()
            },
            "font-size",
        ),
        (
            ParagraphStyle {
                writing_mode: spinon_core::WritingMode::HorizontalTb,
                text_orientation: spinon_core::TextOrientation::Mixed,
                font_family: "Ahem".to_owned(),
                font_size: f32::INFINITY,
                line_height: 10.0,
                white_space: WhiteSpaceMode::Normal,
                word_break: WordBreakMode::Normal,
                direction: crate::TextDirection::Auto,
                line_break: crate::LineBreakMode::Strict,
                ligatures: crate::LigatureMode::Normal,
                ..ParagraphStyle::default()
            },
            "font-size",
        ),
        (
            ParagraphStyle {
                writing_mode: spinon_core::WritingMode::HorizontalTb,
                text_orientation: spinon_core::TextOrientation::Mixed,
                font_family: "Ahem".to_owned(),
                font_size: 10.0,
                line_height: f32::NAN,
                white_space: WhiteSpaceMode::Normal,
                word_break: WordBreakMode::Normal,
                direction: crate::TextDirection::Auto,
                line_break: crate::LineBreakMode::Strict,
                ligatures: crate::LigatureMode::Normal,
                ..ParagraphStyle::default()
            },
            "line-height",
        ),
        (
            ParagraphStyle {
                writing_mode: spinon_core::WritingMode::HorizontalTb,
                text_orientation: spinon_core::TextOrientation::Mixed,
                font_family: "Ahem".to_owned(),
                font_size: 10.0,
                line_height: -1.0,
                white_space: WhiteSpaceMode::Normal,
                word_break: WordBreakMode::Normal,
                direction: crate::TextDirection::Auto,
                line_break: crate::LineBreakMode::Strict,
                ligatures: crate::LigatureMode::Normal,
                ..ParagraphStyle::default()
            },
            "line-height",
        ),
    ];

    for (style, expected_field) in cases {
        let mut engine = TextLayoutEngine::new_without_system_fonts();
        let input = ParagraphInput {
            node,
            sources: vec![TextSource {
                node: source,
                inline_ancestors: Vec::new(),
                text: "X".to_owned(),
            }],
            style,
            inline_styles: Default::default(),
            inline_run_styles: Default::default(),
        };
        assert!(matches!(
            engine.layout_paragraph(&input, 10.0),
            Err(TextLayoutError::InvalidStyle(actual_field)) if actual_field == expected_field
        ));
    }
}

#[test]
fn paragraph_style_allows_zero_line_height_and_keeps_text_lines_finite() {
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).unwrap();
    let input = ParagraphInput {
        node: NodeId::new(1).unwrap(),
        sources: vec![TextSource {
            node: NodeId::new(2).unwrap(),
            inline_ancestors: Vec::new(),
            text: "X\nX".to_owned(),
        }],
        style: ParagraphStyle {
            font_family: "Ahem".to_owned(),
            font_size: 10.0,
            line_height: 0.0,
            line_height_is_normal: false,
            writing_mode: spinon_core::WritingMode::HorizontalTb,
            text_orientation: spinon_core::TextOrientation::Mixed,
            white_space: WhiteSpaceMode::Pre,
            word_break: WordBreakMode::Normal,
            direction: crate::TextDirection::Auto,
            line_break: crate::LineBreakMode::Strict,
            ligatures: crate::LigatureMode::Normal,
            ..ParagraphStyle::default()
        },
        inline_styles: Default::default(),
        inline_run_styles: Default::default(),
    };

    let layout = engine.layout_paragraph(&input, 20.0).unwrap();

    assert_eq!(layout.lines.len(), 2);
    assert_eq!(layout.height, 0.0);
    assert!(layout.width.is_finite());
    assert!(layout.lines.iter().all(|line| {
        line.advance.is_finite() && line.baseline.is_finite() && line.line_height == 0.0
    }));
}

#[test]
fn inline_run_style_still_rejects_zero_line_height() {
    let style = crate::InlineRunStyle {
        font_family: "Ahem".to_owned(),
        font_size: 10.0,
        line_height: 0.0,
        line_height_is_normal: false,
        font_weight: 400.0,
        font_style: crate::FontStyleMode::Normal,
    };

    assert_eq!(style.validate(), Err("line-height"));
}
