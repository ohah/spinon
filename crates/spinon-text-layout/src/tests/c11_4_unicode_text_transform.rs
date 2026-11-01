#![cfg(feature = "c11-4-text-transform-unicode-fixture-test")]

use crate::{
    LigatureMode, LineBreakMode, ParagraphInput, ParagraphStyle, TextDirection, TextLayoutEngine,
    TextSource, TextTransformMode, WhiteSpaceMode, WordBreakMode,
};

const NOTO_KR: &[u8] =
    include_bytes!("../../../../tests/fixtures/resources/c15/c15-3/NotoSansKR-VF.ttf");

fn paragraph(text: &str) -> ParagraphInput {
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
            text_transform: TextTransformMode::Capitalize,
            white_space: WhiteSpaceMode::Normal,
            word_break: WordBreakMode::Normal,
            locale: Some("en".to_owned()),
            direction: TextDirection::Ltr,
            line_break: LineBreakMode::Normal,
            ligatures: LigatureMode::Normal,
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

#[test]
fn c114_unicode_capitalize_uses_uax29_boundaries_and_full_titlecase() {
    for (source, expected) in [
        ("ßeta", "Sseta"),
        ("éclair", "Éclair"),
        ("e\u{0301}lan", "E\u{0301}lan"),
        ("well-known", "Well-Known"),
        ("o'neill", "O'neill"),
        ("«élan»", "«Élan»"),
        ("49ers", "49ers"),
        ("ßeta   éclair", "Sseta Éclair"),
    ] {
        let mut engine = engine();
        let input = paragraph(source);
        let original = input.sources[0].text.clone();
        let layout = engine.layout_paragraph(&input, 200.0).unwrap();
        assert_eq!(layout.text, expected, "{source:?}");
        assert_eq!(input.sources[0].text, original, "{source:?}");
    }
}

#[test]
fn c114_unicode_capitalize_maps_each_expansion_scalar_to_the_source_scalar() {
    let mut engine = engine();
    let input = paragraph("ßeta");
    let layout = engine.layout_paragraph(&input, 200.0).unwrap();

    assert_eq!(layout.text, "Sseta");
    assert_eq!(layout.sources.len(), 5);
    assert_eq!(layout.sources[0].source_utf16, 0..1);
    assert_eq!(layout.sources[0].paragraph_utf8, 0..1);
    assert_eq!(layout.sources[1].source_utf16, 0..1);
    assert_eq!(layout.sources[1].paragraph_utf8, 1..2);
    for (index, source) in layout.sources[2..].iter().enumerate() {
        assert_eq!(source.source_utf16, index + 1..index + 2);
        assert_eq!(source.paragraph_utf8, index + 2..index + 3);
    }
}

#[test]
fn c114_unicode_capitalize_collapses_whitespace_before_mapping_and_preserves_all_sources() {
    let mut engine = engine();
    let input = paragraph("ßeta   éclair");
    let layout = engine.layout_paragraph(&input, 200.0).unwrap();

    assert_eq!(layout.text, "Sseta Éclair");
    let sharp_s = &layout.sources[..2];
    assert!(sharp_s.iter().all(|source| source.source_utf16 == (0..1)));
    assert_eq!(sharp_s[0].paragraph_utf8, 0..1);
    assert_eq!(sharp_s[1].paragraph_utf8, 1..2);
    let collapsed_spaces = layout
        .sources
        .iter()
        .filter(|source| source.paragraph_utf8 == (5..6))
        .collect::<Vec<_>>();
    assert_eq!(collapsed_spaces.len(), 3);
    assert_eq!(
        collapsed_spaces
            .iter()
            .map(|source| source.source_utf16.clone())
            .collect::<Vec<_>>(),
        [4..5, 5..6, 6..7]
    );
    let accented = layout
        .sources
        .iter()
        .find(|source| source.source_utf16 == (7..8))
        .unwrap();
    assert_eq!(accented.paragraph_utf8, 6..8);
}

#[test]
fn c114_unicode_capitalize_does_not_reset_across_adjacent_direct_text_nodes() {
    let mut input = paragraph("ßeta");
    input.sources = vec![
        TextSource {
            node: spinon_core::NodeId::new(2).unwrap(),
            inline_ancestors: Vec::new(),
            text: "ß".to_owned(),
        },
        TextSource {
            node: spinon_core::NodeId::new(3).unwrap(),
            inline_ancestors: Vec::new(),
            text: "eta".to_owned(),
        },
    ];
    let mut engine = engine();
    let layout = engine.layout_paragraph(&input, 200.0).unwrap();

    assert_eq!(layout.text, "Sseta");
    assert_eq!(layout.sources[0].node, spinon_core::NodeId::new(2).unwrap());
    assert_eq!(layout.sources[1].node, spinon_core::NodeId::new(2).unwrap());
    assert!(
        layout.sources[0..2]
            .iter()
            .all(|source| source.source_utf16 == (0..1))
    );
    assert!(
        layout.sources[2..]
            .iter()
            .all(|source| source.node == spinon_core::NodeId::new(3).unwrap())
    );
    assert_eq!(layout.sources[2].source_utf16, 0..1);
}

#[test]
fn c114_unicode_capitalize_rejects_non_english_inline_and_unlisted_sources() {
    let mut engine = engine();
    let mut other_language = paragraph("ßeta");
    other_language.style.locale = Some("de".to_owned());
    assert!(matches!(
        engine.layout_paragraph(&other_language, 200.0),
        Err(crate::TextLayoutError::UnsupportedTextTransformProfile(node)) if node == other_language.node
    ));

    let mut preserved_white_space = paragraph("ßeta");
    preserved_white_space.style.white_space = WhiteSpaceMode::Pre;
    assert!(matches!(
        engine.layout_paragraph(&preserved_white_space, 200.0),
        Err(crate::TextLayoutError::UnsupportedTextTransformProfile(node))
            if node == preserved_white_space.node
    ));

    let mut inline = paragraph("ßeta");
    inline.sources[0]
        .inline_ancestors
        .push(spinon_core::NodeId::new(3).unwrap());
    assert!(matches!(
        engine.intrinsic_inline_sizes(&inline),
        Err(crate::TextLayoutError::UnsupportedTextTransformProfile(node)) if node == inline.node
    ));

    let mut non_ascii_uppercase = paragraph("éclair");
    non_ascii_uppercase.style.text_transform = TextTransformMode::Uppercase;
    assert!(matches!(
        engine.layout_paragraph(&non_ascii_uppercase, 200.0),
        Err(crate::TextLayoutError::UnsupportedTextTransformProfile(node))
            if node == non_ascii_uppercase.node
    ));

    let unlisted = paragraph("ßeta!");
    assert!(matches!(
        engine.layout_paragraph(&unlisted, 200.0),
        Err(crate::TextLayoutError::UnsupportedTextTransformProfile(node)) if node == unlisted.node
    ));
}
