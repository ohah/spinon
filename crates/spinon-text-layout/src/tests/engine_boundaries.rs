use super::{AHEM, ROBOTO, close_to, engine, node, paragraph};
use crate::{
    LigatureMode, LineBreakMode, TextDirection, TextLayoutEngine, TextLayoutError, WhiteSpaceMode,
    WordBreakMode,
};

#[test]
fn empty_text_does_not_create_a_line_and_missing_fonts_fail_closed() {
    let mut engine = engine();
    let mut empty = paragraph(
        "   ",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    empty.style.direction = TextDirection::Rtl;
    empty.style.line_break = LineBreakMode::Normal;
    empty.style.ligatures = LigatureMode::None;
    let empty_layout = engine.layout_paragraph(&empty, 30.0).unwrap();
    assert!(empty_layout.lines.is_empty());
    assert_eq!(empty_layout.height, 0.0);
    assert_eq!(empty_layout.direction, TextDirection::Rtl);
    assert_eq!(empty_layout.line_break, LineBreakMode::Normal);
    assert_eq!(empty_layout.ligatures, LigatureMode::None);
    assert_eq!(empty_layout.sources.len(), 3);
    assert!(
        empty_layout
            .sources
            .iter()
            .all(|source| source.paragraph_utf8 == (0..0))
    );

    let missing = paragraph(
        "X",
        "System Sans",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    assert!(matches!(
        engine.layout_paragraph(&missing, 30.0),
        Err(TextLayoutError::MissingFontFamily(_))
    ));
}
#[test]
fn zero_width_is_definite_and_preserves_unbreakable_text_advance() {
    let mut engine = engine();
    let input = paragraph(
        "X",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    let layout = engine.layout_paragraph(&input, 0.0).unwrap();

    assert_eq!(layout.available_width, 0.0);
    let negative_zero = engine.layout_paragraph(&input, -0.0).unwrap();
    assert_eq!(negative_zero.available_width.to_bits(), 0.0_f32.to_bits());
    assert_eq!(layout.lines.len(), 1);
    close_to(layout.width, 10.0);
    let glyph_fragment = layout
        .inline_fragments
        .iter()
        .find(|fragment| fragment.node == node(2))
        .unwrap();
    close_to(glyph_fragment.rect.width, 10.0);
    assert!(matches!(
        engine.layout_paragraph(&input, -1.0),
        Err(TextLayoutError::InvalidWidth)
    ));
    assert!(matches!(
        engine.layout_paragraph(&input, f32::NAN),
        Err(TextLayoutError::InvalidWidth)
    ));
    assert!(matches!(
        engine.layout_paragraph(&input, f32::INFINITY),
        Err(TextLayoutError::InvalidWidth)
    ));
}

#[test]
fn font_registration_is_transactional_and_checks_the_new_font_family() {
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    let first_revision = engine.register_font("Ahem", AHEM).unwrap();
    assert_eq!(first_revision.0, 1);

    assert!(matches!(
        engine.register_font("Invalid", [0_u8, 1, 2]),
        Err(TextLayoutError::InvalidFontData(family)) if family == "Invalid"
    ));
    assert_eq!(engine.font_registry_revision(), first_revision);
    assert!(
        engine
            .font_context
            .collection
            .family_id("Invalid")
            .is_none()
    );

    assert!(matches!(
        engine.register_font("Ahem", ROBOTO),
        Err(TextLayoutError::InvalidFontData(family)) if family == "Ahem"
    ));
    assert_eq!(engine.font_registry_revision(), first_revision);

    let missing = paragraph(
        "X",
        "Roboto",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    assert!(matches!(
        engine.layout_paragraph(&missing, 30.0),
        Err(TextLayoutError::MissingFontFamily(family)) if family == "Roboto"
    ));

    let second_revision = engine.register_font("Roboto", ROBOTO).unwrap();
    assert_eq!(second_revision.0, 2);

    let mut exhausted = TextLayoutEngine::new_without_system_fonts();
    exhausted.font_registry_revision = super::super::FontRegistryRevision(u64::MAX);
    assert!(matches!(
        exhausted.register_font("Ahem", AHEM),
        Err(TextLayoutError::FontRevisionOverflow)
    ));
    assert!(
        exhausted
            .font_context
            .collection
            .family_id("Ahem")
            .is_none()
    );
}
