use crate::{
    LigatureMode, LineBreakMode, ParagraphInput, ParagraphStyle, TextDirection, TextLayoutEngine,
    TextSource, TextTransformMode, WhiteSpaceMode, WordBreakMode,
};

const NOTO_KR: &[u8] =
    include_bytes!("../../../../tests/fixtures/resources/c15/c15-3/NotoSansKR-VF.ttf");

fn paragraph(text: &str, text_transform: TextTransformMode) -> ParagraphInput {
    ParagraphInput {
        node: spinon_core::NodeId::new(1).unwrap(),
        sources: vec![TextSource {
            node: spinon_core::NodeId::new(2).unwrap(),
            inline_ancestors: Vec::new(),
            text: text.to_owned(),
        }],
        style: ParagraphStyle {
            font_family: "Noto Sans KR".to_owned(),
            font_size: 10.0,
            line_height: 10.0,
            line_height_is_normal: false,
            font_weight: 400.0,
            font_style: crate::FontStyleMode::Normal,
            white_space: WhiteSpaceMode::Normal,
            word_break: WordBreakMode::Normal,
            direction: TextDirection::Ltr,
            line_break: LineBreakMode::Normal,
            ligatures: LigatureMode::Normal,
            text_transform,
            ..ParagraphStyle::default()
        },
        inline_styles: Default::default(),
        inline_run_styles: Default::default(),
    }
}

fn engine() -> TextLayoutEngine {
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Noto Sans KR", NOTO_KR).unwrap();
    engine
}

fn close_to(actual: f32, expected: f32, label: &str) {
    assert!(
        (actual - expected).abs() <= 0.03125,
        "{label}: expected {expected}, got {actual} CSS px"
    );
}

#[test]
fn c114_text_transform_changes_layout_only_ascii_and_preserves_source_ranges() {
    for (mode, expected_text, min_inline, max_inline) in [
        (TextTransformMode::None, "IIII ii", 11.734375, 19.46875),
        (TextTransformMode::Uppercase, "IIII II", 11.734375, 19.84375),
        (TextTransformMode::Lowercase, "iiii ii", 11.0, 18.71875),
    ] {
        let mut engine = engine();
        let input = paragraph("IIII ii", mode);
        let original_source = input.sources[0].text.clone();
        let intrinsic = engine.intrinsic_inline_sizes(&input).unwrap();
        close_to(intrinsic.min_content(), min_inline, "min-content");
        close_to(intrinsic.max_content(), max_inline, "max-content");
        close_to(intrinsic.min_content_height(), 20.0, "min-content height");
        close_to(intrinsic.max_content_height(), 10.0, "max-content height");

        let layout = engine.layout_paragraph(&input, 120.0).unwrap();
        assert_eq!(layout.text_transform, mode);
        assert_eq!(layout.text, expected_text);
        assert_eq!(input.sources[0].text, original_source);
        assert_eq!(input.sources[0].text, "IIII ii");
        assert_eq!(layout.sources.len(), 7);
        for (index, source) in layout.sources.iter().enumerate() {
            assert_eq!(source.node, spinon_core::NodeId::new(2).unwrap());
            assert_eq!(source.source_utf16, index..index + 1);
            assert_eq!(source.paragraph_utf8, index..index + 1);
        }
        assert_eq!(layout.lines.len(), 1);
        close_to(
            layout.lines[0].advance,
            max_inline,
            "max-content line advance",
        );
    }
}

#[test]
fn c114_text_transform_keeps_collapsed_space_source_mapping_one_to_one() {
    let mut engine = engine();
    let input = paragraph("II   ii", TextTransformMode::Lowercase);
    let layout = engine.layout_paragraph(&input, 120.0).unwrap();

    assert_eq!(layout.text, "ii ii");
    assert_eq!(input.sources[0].text, "II   ii");
    assert_eq!(layout.sources.len(), 7);
    for (index, source) in layout.sources[2..5].iter().enumerate() {
        assert_eq!(source.source_utf16, index + 2..index + 3);
        assert_eq!(source.paragraph_utf8, 2..3);
    }
    assert_eq!(layout.sources[5].source_utf16, 5..6);
    assert_eq!(layout.sources[5].paragraph_utf8, 3..4);
}

#[test]
fn c114_text_transform_fails_closed_for_non_ascii_and_inline_sources() {
    let mut engine = engine();
    let mut non_ascii = paragraph("İi", TextTransformMode::Lowercase);
    assert_eq!(
        engine.layout_paragraph(&non_ascii, 120.0),
        Err(crate::TextLayoutError::UnsupportedTextTransformProfile(
            non_ascii.node
        ))
    );

    let mut inline = paragraph("II", TextTransformMode::Uppercase);
    inline.sources[0]
        .inline_ancestors
        .push(spinon_core::NodeId::new(3).unwrap());
    assert_eq!(
        engine.intrinsic_inline_sizes(&inline),
        Err(crate::TextLayoutError::UnsupportedTextTransformProfile(
            inline.node
        ))
    );

    non_ascii.style.text_transform = TextTransformMode::None;
    let layout = engine.layout_paragraph(&non_ascii, 120.0).unwrap();
    assert_eq!(layout.text, "İi");
}
