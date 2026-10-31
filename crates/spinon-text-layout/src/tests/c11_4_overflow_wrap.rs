use crate::{
    OverflowWrapMode, ParagraphInput, ParagraphStyle, TextDirection, TextLayoutEngine, TextSource,
    WhiteSpaceMode, WordBreakMode,
};

const AHEM: &[u8] = include_bytes!("../../../../tests/fixtures/resources/c15/Ahem.ttf");

fn engine() -> TextLayoutEngine {
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).unwrap();
    engine
}

fn paragraph(mode: OverflowWrapMode) -> ParagraphInput {
    ParagraphInput {
        node: spinon_core::NodeId::new(1).unwrap(),
        sources: vec![TextSource {
            node: spinon_core::NodeId::new(2).unwrap(),
            inline_ancestors: Vec::new(),
            text: "XXXX".to_owned(),
        }],
        style: ParagraphStyle {
            font_family: "Ahem".to_owned(),
            font_size: 10.0,
            line_height: 10.0,
            line_height_is_normal: false,
            font_weight: 400.0,
            font_style: crate::FontStyleMode::Normal,
            white_space: WhiteSpaceMode::Normal,
            word_break: WordBreakMode::Normal,
            overflow_wrap: mode,
            direction: TextDirection::Ltr,
            line_break: crate::LineBreakMode::Normal,
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
fn c114_overflow_wrap_separates_min_content_from_emergency_wrap() {
    let mut engine = engine();
    for (mode, min_inline, min_block, fixed_line_count) in [
        (OverflowWrapMode::Normal, 40.0, 10.0, 1),
        (OverflowWrapMode::Anywhere, 10.0, 40.0, 4),
        (OverflowWrapMode::BreakWord, 40.0, 10.0, 4),
    ] {
        let input = paragraph(mode);
        let sizes = engine.intrinsic_inline_sizes(&input).unwrap();
        let label = format!("overflow-wrap={mode:?}");
        close_to(
            sizes.min_content(),
            min_inline,
            &format!("{label} min inline"),
        );
        close_to(
            sizes.min_content_height(),
            min_block,
            &format!("{label} min block"),
        );
        close_to(sizes.max_content(), 40.0, &format!("{label} max inline"));
        close_to(
            sizes.max_content_height(),
            10.0,
            &format!("{label} max block"),
        );

        let snapshot = engine.layout_paragraph(&input, 10.0).unwrap();
        assert_eq!(snapshot.overflow_wrap, mode, "{label} snapshot style");
        assert_eq!(
            snapshot.lines.len(),
            fixed_line_count,
            "{label} fixed-width lines"
        );
        close_to(
            snapshot.height,
            fixed_line_count as f32 * 10.0,
            &format!("{label} fixed-width block size"),
        );
    }
}

#[test]
fn empty_snapshot_keeps_computed_overflow_wrap_mode() {
    let mut input = paragraph(OverflowWrapMode::BreakWord);
    input.sources[0].text.clear();
    let snapshot = engine().layout_paragraph(&input, 10.0).unwrap();
    assert_eq!(snapshot.overflow_wrap, OverflowWrapMode::BreakWord);
    assert!(snapshot.lines.is_empty());
}
