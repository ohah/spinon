use spinon_core::NodeId;

use crate::{
    ParagraphInput, ParagraphStyle, TextLayoutEngine, TextLayoutError, TextSource, WhiteSpaceMode,
    WordBreakMode,
};

#[test]
fn paragraph_style_rejects_invalid_family_and_non_positive_or_non_finite_metrics() {
    let node = NodeId::new(1).unwrap();
    let source = NodeId::new(2).unwrap();
    let cases = [
        (
            ParagraphStyle {
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
