use crate::{GlyphBitmapContent, TextLayoutError, WhiteSpaceMode, WordBreakMode};

use super::{close_to, paragraph};

const NOTO_KR: &[u8] =
    include_bytes!("../../../../tests/fixtures/resources/c15/c15-3/NotoSansKR-VF.ttf");
const NOTO_ARABIC: &[u8] =
    include_bytes!("../../../../tests/fixtures/resources/c15/c15-3/NotoSansArabic-VF.ttf");
const NOTO_EMOJI: &[u8] =
    include_bytes!("../../../../tests/fixtures/resources/c15/c15-3/NotoEmoji-VF.ttf");
const ROBOTO: &[u8] =
    include_bytes!("../../../../tests/fixtures/resources/c15/Roboto-Variable.ttf");
const AHEM: &[u8] = include_bytes!("../../../../tests/fixtures/resources/c15/Ahem.ttf");

fn font_engine(family: &str, bytes: &[u8]) -> crate::TextLayoutEngine {
    let mut engine = crate::TextLayoutEngine::new_without_system_fonts();
    engine.register_font(family, bytes).unwrap();
    engine
}

#[test]
fn raster_snapshot_preserves_visible_glyphs_and_sources_across_c15_3_scripts() {
    let cases = [
        ("Noto Sans KR", NOTO_KR, "한글 테스트", 20.0),
        ("Noto Sans KR", NOTO_KR, "甲乙、丙丁", 20.0),
        ("Noto Sans Arabic", NOTO_ARABIC, "السَّلَامُ 123", 20.0),
        ("Noto Emoji", NOTO_EMOJI, "👩‍💻", 28.0),
        ("Roboto", ROBOTO, "x\u{0323}", 20.0),
        ("Roboto", ROBOTO, "office", 20.0),
    ];

    for (family, bytes, text, font_size) in cases {
        let mut engine = font_engine(family, bytes);
        let input = paragraph(
            text,
            family,
            font_size,
            font_size + 4.0,
            WhiteSpaceMode::Normal,
            WordBreakMode::Normal,
        );
        let available_width = if text == "甲乙、丙丁" {
            40.0
        } else {
            320.0
        };
        let layout = engine.layout_paragraph(&input, available_width).unwrap();
        let raster = engine
            .rasterize_glyphs(&layout, 1.0)
            .unwrap_or_else(|error| panic!("{family} {text:?}: {error:?}"));

        if text == "甲乙、丙丁" {
            assert_eq!(
                layout
                    .lines
                    .iter()
                    .map(|line| layout.text[line.text_range.clone()].to_owned())
                    .collect::<Vec<_>>(),
                ["甲", "乙、", "丙丁"]
            );
        }

        assert_eq!(raster.paragraph, layout.paragraph);
        assert_eq!(raster.font_registry_revision, layout.font_registry_revision);
        assert_eq!(raster.device_scale_factor, 1.0);
        assert_eq!(
            raster.glyphs.len(),
            layout.runs.iter().map(|run| run.glyphs.len()).sum()
        );
        for (raster_glyph, (run_index, glyph_index, glyph)) in
            raster
                .glyphs
                .iter()
                .zip(layout.runs.iter().enumerate().flat_map(|(run_index, run)| {
                    run.glyphs
                        .iter()
                        .enumerate()
                        .map(move |(glyph_index, glyph)| (run_index, glyph_index, glyph))
                }))
        {
            assert_eq!(raster_glyph.run_index, run_index, "{family}: {text}");
            assert_eq!(raster_glyph.glyph_index, glyph_index, "{family}: {text}");
            assert_eq!(raster_glyph.glyph_id, glyph.id, "{family}: {text}");
            assert_eq!(raster_glyph.cluster_text_range, glyph.cluster_text_range);
            assert_eq!(raster_glyph.sources, glyph.sources);
            close_to(raster_glyph.x_css_px, glyph.x);
            close_to(raster_glyph.baseline_y_css_px, glyph.y);
            let cluster = &layout.text[glyph.cluster_text_range.clone()];
            if cluster.chars().all(char::is_whitespace) {
                assert!(raster_glyph.bitmap.is_none(), "{family}: {text}");
            } else {
                let bitmap = raster_glyph
                    .bitmap
                    .as_ref()
                    .unwrap_or_else(|| panic!("{family} glyph {cluster:?} has no bitmap"));
                let channels = match bitmap.content {
                    GlyphBitmapContent::AlphaMask => 1,
                    GlyphBitmapContent::SubpixelMaskRgba8 | GlyphBitmapContent::ColorRgba8 => 4,
                };
                assert_eq!(
                    bitmap.pixels.len(),
                    bitmap.width_px as usize * bitmap.height_px as usize * channels,
                    "{family}: {cluster:?}"
                );
                assert!(bitmap.width_px > 0 && bitmap.height_px > 0);
            }
        }
    }
}

#[test]
fn variable_font_raster_uses_the_same_normalized_weight_as_parley_shaping() {
    let mut engine = font_engine("Roboto", ROBOTO);
    let regular = paragraph(
        "A",
        "Roboto",
        32.0,
        40.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    let mut bold = regular.clone();
    bold.style.font_weight = 700.0;

    let regular_layout = engine.layout_paragraph(&regular, 100.0).unwrap();
    let bold_layout = engine.layout_paragraph(&bold, 100.0).unwrap();
    assert_eq!(regular_layout.runs.len(), 1);
    assert_eq!(bold_layout.runs.len(), 1);
    assert!(!bold_layout.runs[0].synthesis_embolden);
    assert_ne!(
        regular_layout.runs[0].normalized_coords, bold_layout.runs[0].normalized_coords,
        "the variable font's weight axis must match the coordinates used by shaping"
    );

    let regular_raster = engine.rasterize_glyphs(&regular_layout, 1.0).unwrap();
    let bold_raster = engine.rasterize_glyphs(&bold_layout, 1.0).unwrap();
    assert_ne!(
        regular_raster.glyphs[0].bitmap.as_ref().unwrap().pixels,
        bold_raster.glyphs[0].bitmap.as_ref().unwrap().pixels,
        "raster output must use the selected weight axis"
    );
}

#[test]
fn static_font_raster_applies_parley_synthetic_bold_and_italic() {
    let mut engine = font_engine("Ahem", AHEM);
    let regular = paragraph(
        "X",
        "Ahem",
        20.0,
        24.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    let mut emphasized = regular.clone();
    emphasized.style.font_weight = 700.0;
    emphasized.style.font_style = crate::FontStyleMode::Italic;

    let regular_layout = engine.layout_paragraph(&regular, 40.0).unwrap();
    let emphasized_layout = engine.layout_paragraph(&emphasized, 40.0).unwrap();
    assert_eq!(emphasized_layout.runs.len(), 1);
    assert!(emphasized_layout.runs[0].synthesis_embolden);
    assert_eq!(emphasized_layout.runs[0].synthesis_skew_degrees, Some(14.0));

    let regular_raster = engine.rasterize_glyphs(&regular_layout, 1.0).unwrap();
    let emphasized_raster = engine.rasterize_glyphs(&emphasized_layout, 1.0).unwrap();
    assert_ne!(
        regular_raster.glyphs[0].bitmap, emphasized_raster.glyphs[0].bitmap,
        "synthetic weight/style must change the glyph outline"
    );
}

#[test]
fn ahem_raster_dimensions_scale_in_physical_pixels_but_keep_css_positions() {
    let mut engine = font_engine("Ahem", AHEM);
    let input = paragraph(
        "XX",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    let layout = engine.layout_paragraph(&input, 30.0).unwrap();
    let one = engine.rasterize_glyphs(&layout, 1.0).unwrap();
    let two = engine.rasterize_glyphs(&layout, 2.0).unwrap();
    let three = engine.rasterize_glyphs(&layout, 3.0).unwrap();

    assert_eq!(one.glyphs.len(), 2);
    assert_eq!(two.glyphs.len(), 2);
    assert_eq!(three.glyphs.len(), 2);
    for index in 0..2 {
        let one_bitmap = one.glyphs[index].bitmap.as_ref().unwrap();
        let two_bitmap = two.glyphs[index].bitmap.as_ref().unwrap();
        let three_bitmap = three.glyphs[index].bitmap.as_ref().unwrap();
        assert_eq!((one_bitmap.width_px, one_bitmap.height_px), (10, 10));
        assert_eq!((two_bitmap.width_px, two_bitmap.height_px), (20, 20));
        assert_eq!((three_bitmap.width_px, three_bitmap.height_px), (30, 30));
        close_to(one.glyphs[index].x_css_px, two.glyphs[index].x_css_px);
        close_to(two.glyphs[index].x_css_px, three.glyphs[index].x_css_px);
        close_to(
            one.glyphs[index].baseline_y_css_px,
            three.glyphs[index].baseline_y_css_px,
        );
        assert_eq!(
            one.glyphs[index].origin_x_px,
            (one.glyphs[index].x_css_px * one.device_scale_factor).floor() as i32
        );
        assert_eq!(
            two.glyphs[index].origin_x_px,
            (two.glyphs[index].x_css_px * two.device_scale_factor).floor() as i32
        );
        assert_eq!(
            three.glyphs[index].baseline_y_px,
            (three.glyphs[index].baseline_y_css_px * three.device_scale_factor).floor() as i32
        );
        assert!(one_bitmap.pixels.iter().all(|coverage| *coverage == 255));
        assert!(two_bitmap.pixels.iter().all(|coverage| *coverage == 255));
        assert!(three_bitmap.pixels.iter().all(|coverage| *coverage == 255));
    }

    let mut fractional_layout = layout.clone();
    fractional_layout.runs[0].glyphs[0].x = 0.25;
    fractional_layout.runs[0].glyphs[0].y += 0.25;
    let fractional = engine.rasterize_glyphs(&fractional_layout, 1.0).unwrap();
    let phased_glyph = &fractional.glyphs[0];
    let phased_bitmap = phased_glyph.bitmap.as_ref().unwrap();
    assert_eq!(phased_glyph.origin_x_px, 0);
    assert_eq!(
        phased_glyph.baseline_y_px,
        (phased_glyph.baseline_y_css_px.floor()) as i32
    );
    assert_eq!((phased_bitmap.width_px, phased_bitmap.height_px), (11, 11));
    assert!(phased_bitmap.pixels.iter().any(|coverage| *coverage < 255));
}

#[test]
fn raster_uses_owned_font_bytes_after_registration_input_is_dropped() {
    let mut engine = crate::TextLayoutEngine::new_without_system_fonts();
    let font_bytes = AHEM.to_vec();
    engine.register_font("Ahem", &font_bytes).unwrap();
    drop(font_bytes);

    let input = paragraph(
        "X",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    let layout = engine.layout_paragraph(&input, 30.0).unwrap();
    let raster = engine.rasterize_glyphs(&layout, 1.0).unwrap();
    assert!(raster.glyphs[0].bitmap.is_some());
}

#[test]
fn rasterizer_rejects_invalid_scale_stale_registry_and_corrupt_glyph_ids() {
    let mut engine = font_engine("Roboto", ROBOTO);
    let input = paragraph(
        "office",
        "Roboto",
        20.0,
        24.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    let layout = engine.layout_paragraph(&input, 100.0).unwrap();
    let mut equivalent_engine = font_engine("Roboto", ROBOTO);
    let equivalent_layout = equivalent_engine.layout_paragraph(&input, 100.0).unwrap();
    assert_eq!(layout, equivalent_layout);
    assert_eq!(
        engine.rasterize_glyphs(&layout, 1.0).unwrap(),
        equivalent_engine
            .rasterize_glyphs(&equivalent_layout, 1.0)
            .unwrap()
    );
    let engine_clone = engine.clone();
    assert_eq!(
        engine_clone.rasterize_glyphs(&layout, 1.0),
        engine.rasterize_glyphs(&layout, 1.0)
    );

    let mut left_registry = engine.clone();
    let mut right_registry = engine.clone();
    left_registry.register_font("Ahem", AHEM).unwrap();
    right_registry
        .register_font("Noto Sans KR", NOTO_KR)
        .unwrap();
    let branched_layout = left_registry.layout_paragraph(&input, 100.0).unwrap();
    assert!(matches!(
        right_registry.rasterize_glyphs(&branched_layout, 1.0),
        Err(TextLayoutError::ForeignFontRegistrySnapshot)
    ));

    for scale in [0.0, -1.0, f32::NAN, f32::INFINITY, 64.1] {
        assert!(matches!(
            engine.rasterize_glyphs(&layout, scale),
            Err(TextLayoutError::InvalidRasterScale)
        ));
    }

    for font_size in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        let mut invalid_size = layout.clone();
        invalid_size.runs[0].font_size = font_size;
        assert!(matches!(
            engine.rasterize_glyphs(&invalid_size, 1.0),
            Err(TextLayoutError::RasterFontSizeOutOfRange { .. })
        ));
    }

    let mut corrupt = layout.clone();
    corrupt.runs[0].glyphs[0].id = u32::MAX;
    assert!(matches!(
        engine.rasterize_glyphs(&corrupt, 1.0),
        Err(TextLayoutError::GlyphRasterizationFailed {
            paragraph,
            glyph_id: u32::MAX,
        }) if paragraph == layout.paragraph
    ));

    let mut notdef = layout.clone();
    notdef.runs[0].glyphs[0].id = 0;
    assert!(matches!(
        engine.rasterize_glyphs(&notdef, 1.0),
        Err(TextLayoutError::GlyphRasterizationFailed { glyph_id: 0, .. })
    ));

    let mut out_of_font_range = layout.clone();
    out_of_font_range.runs[0].glyphs[0].id = u16::MAX.into();
    assert!(matches!(
        engine.rasterize_glyphs(&out_of_font_range, 1.0),
        Err(TextLayoutError::GlyphRasterizationFailed {
            glyph_id,
            ..
        }) if glyph_id == u16::MAX.into()
    ));

    let mut invalid_source = layout.clone();
    invalid_source.runs[0].glyphs[0].cluster_text_range = 0..usize::MAX;
    assert!(matches!(
        engine.rasterize_glyphs(&invalid_source, 1.0),
        Err(TextLayoutError::InvalidGlyphLayout(paragraph)) if paragraph == layout.paragraph
    ));

    let mut missing_mapping = layout.clone();
    missing_mapping.runs[0].glyphs[0].sources.clear();
    assert!(matches!(
        engine.rasterize_glyphs(&missing_mapping, 1.0),
        Err(TextLayoutError::InvalidGlyphLayout(paragraph)) if paragraph == layout.paragraph
    ));

    let mut fabricated_mapping = layout.clone();
    fabricated_mapping.runs[0].glyphs[0].sources[0].paragraph_utf8 = 0..usize::MAX;
    assert!(matches!(
        engine.rasterize_glyphs(&fabricated_mapping, 1.0),
        Err(TextLayoutError::InvalidGlyphLayout(paragraph)) if paragraph == layout.paragraph
    ));

    let mut position_overflow = layout.clone();
    position_overflow.runs[0].glyphs[0].x = f32::MAX;
    assert!(matches!(
        engine.rasterize_glyphs(&position_overflow, 1.0),
        Err(TextLayoutError::InvalidGlyphLayout(paragraph)) if paragraph == layout.paragraph
    ));

    let mut missing_font = layout.clone();
    missing_font.runs[0].font_data.index = u32::MAX;
    assert!(matches!(
        engine.rasterize_glyphs(&missing_font, 1.0),
        Err(TextLayoutError::InvalidRasterFont(family)) if family == "Roboto"
    ));

    let mut invalid_font_engine = font_engine("Roboto", ROBOTO);
    let mut invalid_font_layout = invalid_font_engine.layout_paragraph(&input, 100.0).unwrap();
    invalid_font_layout.runs[0].font_data.index = u32::MAX;
    assert!(matches!(
        invalid_font_engine.rasterize_glyphs(&invalid_font_layout, 1.0),
        Err(TextLayoutError::InvalidRasterFont(family)) if family == "Roboto"
    ));

    engine.register_font("Ahem", AHEM).unwrap();
    assert!(matches!(
        engine.rasterize_glyphs(&layout, 1.0),
        Err(TextLayoutError::StaleFontRegistryRevision { expected, actual })
            if expected == engine.font_registry_revision() && actual == layout.font_registry_revision
    ));

    let other_engine = font_engine("Ahem", AHEM);
    assert!(matches!(
        other_engine.rasterize_glyphs(&layout, 1.0),
        Err(TextLayoutError::ForeignFontRegistrySnapshot)
    ));
}

#[test]
fn rasterizer_handles_empty_text_and_rejects_oversized_physical_font_size() {
    let mut engine = font_engine("Ahem", AHEM);
    let empty_input = paragraph(
        "",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    let empty_layout = engine.layout_paragraph(&empty_input, 30.0).unwrap();
    let empty_raster = engine.rasterize_glyphs(&empty_layout, 1.0).unwrap();
    assert!(empty_raster.glyphs.is_empty());

    let input = paragraph(
        "X",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    let mut layout = engine.layout_paragraph(&input, 30.0).unwrap();
    layout.runs[0].font_size = 256.0;
    assert!(matches!(
        engine.rasterize_glyphs(&layout, 3.0),
        Err(TextLayoutError::RasterFontSizeOutOfRange {
            paragraph,
            font_family,
        }) if paragraph == layout.paragraph && font_family == "Ahem"
    ));
}
