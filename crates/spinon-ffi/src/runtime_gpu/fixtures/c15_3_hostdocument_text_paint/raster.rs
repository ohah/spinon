use std::sync::Arc;

use spinon_layout::LayoutTextSnapshot;
use spinon_render::RuntimeGlyphBitmap;
use spinon_text_layout::{GlyphBitmapContent, TextGlyphRasterSnapshot};

use super::{DEVICE_SCALE_FACTOR, VIEWPORT_HEIGHT, VIEWPORT_WIDTH};

pub(super) fn validate_raster_snapshot(
    text_snapshot: &LayoutTextSnapshot,
    raster: &TextGlyphRasterSnapshot,
) -> Result<(), String> {
    let shaped_glyphs = text_snapshot
        .layout
        .runs
        .iter()
        .flat_map(|run| run.glyphs.iter())
        .collect::<Vec<_>>();
    if raster.paragraph != text_snapshot.layout.paragraph
        || raster.font_registry_revision != text_snapshot.layout.font_registry_revision
        || (raster.device_scale_factor - DEVICE_SCALE_FACTOR).abs() > f32::EPSILON
        || raster.glyphs.len() != shaped_glyphs.len()
    {
        return Err(
            "C15.3 fixture raster revision·paragraph·glyph 개수가 layout과 다릅니다".to_owned(),
        );
    }
    for (index, glyph) in raster.glyphs.iter().enumerate() {
        let shaped = shaped_glyphs[index];
        let bitmap = glyph
            .bitmap
            .as_ref()
            .ok_or_else(|| format!("C15.3 fixture glyph {index}에 ink bitmap이 없습니다"))?;
        if glyph.glyph_id != shaped.id
            || glyph.sources.len() != 1
            || glyph.sources[0].source_utf16 != (index..index + 1)
            || bitmap.content != GlyphBitmapContent::AlphaMask
        {
            return Err(format!(
                "C15.3 fixture raster glyph {index}가 같은 layout glyph/source가 아닙니다"
            ));
        }
    }
    Ok(())
}

pub(super) fn runtime_glyphs(
    raster: &TextGlyphRasterSnapshot,
) -> Result<Vec<RuntimeGlyphBitmap>, String> {
    let mut bitmaps = Vec::new();
    bitmaps
        .try_reserve_exact(raster.glyphs.len())
        .map_err(|_| "C15.3 fixture runtime glyph 목록 할당 실패".to_owned())?;
    for (index, glyph) in raster.glyphs.iter().enumerate() {
        let bitmap = glyph
            .bitmap
            .as_ref()
            .ok_or_else(|| format!("C15.3 fixture glyph {index}에 bitmap이 없습니다"))?;
        if bitmap.content != GlyphBitmapContent::AlphaMask {
            return Err(format!(
                "C15.3 fixture glyph {index} bitmap이 alpha mask가 아닙니다"
            ));
        }
        bitmaps.push(
            RuntimeGlyphBitmap::new(
                bitmap.width_px,
                bitmap.height_px,
                glyph.origin_x_px,
                glyph.baseline_y_px,
                bitmap.left_px,
                bitmap.top_px,
                Arc::clone(&bitmap.pixels),
            )
            .map_err(|error| format!("C15.3 fixture RuntimeGlyphBitmap 생성 실패: {error}"))?,
        );
    }
    Ok(bitmaps)
}

pub(super) fn overflow_ink_samples(
    raster: &TextGlyphRasterSnapshot,
    paragraph_frame: spinon_layout::LayoutFrame,
) -> Result<[(u32, u32); 6], String> {
    let mut first_glyph_samples = Vec::new();
    let mut overflow_glyph_samples = Vec::new();
    for (glyph_index, glyph) in raster.glyphs.iter().enumerate() {
        let bitmap = glyph
            .bitmap
            .as_ref()
            .ok_or_else(|| format!("C15.3 fixture glyph {glyph_index}에 bitmap이 없습니다"))?;
        let left = paragraph_frame.x + (glyph.origin_x_px + bitmap.left_px) as f32;
        let top = paragraph_frame.y + (glyph.baseline_y_px + bitmap.top_px) as f32;
        for row in 0..bitmap.height_px {
            for column in 0..bitmap.width_px {
                let index = usize::try_from(
                    u64::from(row) * u64::from(bitmap.width_px) + u64::from(column),
                )
                .map_err(|_| "C15.3 fixture bitmap index 변환 실패".to_owned())?;
                let coverage = bitmap.pixels.get(index).copied().ok_or_else(|| {
                    format!(
                        "C15.3 fixture glyph {glyph_index} bitmap pixel buffer가 크기보다 짧습니다"
                    )
                })?;
                if coverage != u8::MAX {
                    continue;
                }
                let x = left + column as f32;
                let y = top + row as f32;
                let point = (x.floor() as u32, y.floor() as u32);
                let center_x = point.0 as f32 + 0.5;
                let center_y = point.1 as f32 + 0.5;
                if center_x >= VIEWPORT_WIDTH || center_y >= VIEWPORT_HEIGHT {
                    continue;
                }
                let samples = if glyph_index == 0 {
                    if center_x < paragraph_frame.x + paragraph_frame.width {
                        &mut first_glyph_samples
                    } else {
                        continue;
                    }
                } else if glyph_index == 1 {
                    if center_x >= paragraph_frame.x + paragraph_frame.width {
                        &mut overflow_glyph_samples
                    } else {
                        continue;
                    }
                } else {
                    return Err("C15.3 fixture raster에 두 개보다 많은 glyph가 있습니다".to_owned());
                };
                if !samples.contains(&point) {
                    samples.push(point);
                }
            }
        }
    }
    if first_glyph_samples.len() < 2 || overflow_glyph_samples.len() < 4 {
        return Err(format!(
            "C15.3 fixture coverage 표본이 부족합니다: first={} overflow={}",
            first_glyph_samples.len(),
            overflow_glyph_samples.len()
        ));
    }
    Ok([
        first_glyph_samples[0],
        first_glyph_samples[1],
        overflow_glyph_samples[0],
        overflow_glyph_samples[1],
        overflow_glyph_samples[2],
        overflow_glyph_samples[3],
    ])
}

pub(super) fn encode_sample_points(points: &[(u32, u32); 6]) -> String {
    let names = [
        "first_ink_1",
        "first_ink_2",
        "overflow_ink_1",
        "overflow_ink_2",
        "overflow_ink_3",
        "overflow_ink_4",
    ];
    names
        .into_iter()
        .zip(points)
        .map(|(name, (x, y))| format!("{name}:{x},{y}"))
        .collect::<Vec<_>>()
        .join(";")
}
