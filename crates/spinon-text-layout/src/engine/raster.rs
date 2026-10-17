use std::collections::BTreeMap;

use swash::{
    FontRef, GlyphId,
    scale::{Render, ScaleContext, Source, StrikeWith},
    zeno::{Format, Vector},
};

use crate::{
    GlyphBitmap, GlyphBitmapContent, TextGlyphRaster, TextGlyphRasterSnapshot, TextLayoutError,
    TextLayoutSnapshot,
};

use super::TextLayoutEngine;

const MAX_DEVICE_SCALE_FACTOR: f32 = 64.0;
const MAX_GLYPH_SIZE_PX: f32 = 512.0;
const MAX_GLYPH_BITMAP_BYTES: usize = 16 * 1024 * 1024;
const MAX_GLYPH_SNAPSHOT_BYTES: usize = 64 * 1024 * 1024;
const RASTER_SOURCES: &[Source] = &[
    Source::ColorOutline(0),
    Source::ColorBitmap(StrikeWith::BestFit),
    Source::Outline,
];

impl TextLayoutEngine {
    /// 현재 등록된 고정 글꼴로 shaped glyph를 device-pixel bitmap으로 만듭니다.
    pub fn rasterize_glyphs(
        &self,
        layout: &TextLayoutSnapshot,
        device_scale_factor: f32,
    ) -> Result<TextGlyphRasterSnapshot, TextLayoutError> {
        if layout.font_registry_revision != self.font_registry_revision {
            return Err(TextLayoutError::StaleFontRegistryRevision {
                expected: self.font_registry_revision,
                actual: layout.font_registry_revision,
            });
        }
        if !layout
            .font_registry_token
            .matches(&self.font_registry_token)
        {
            return Err(TextLayoutError::ForeignFontRegistrySnapshot);
        }
        if !device_scale_factor.is_finite()
            || device_scale_factor <= 0.0
            || device_scale_factor > MAX_DEVICE_SCALE_FACTOR
        {
            return Err(TextLayoutError::InvalidRasterScale);
        }

        let glyph_count = layout
            .runs
            .iter()
            .try_fold(0_usize, |count, run| count.checked_add(run.glyphs.len()));
        let Some(glyph_count) = glyph_count else {
            return Err(TextLayoutError::InvalidGlyphLayout(layout.paragraph));
        };
        let mut glyphs = Vec::new();
        glyphs
            .try_reserve(glyph_count)
            .map_err(|_| TextLayoutError::RasterAllocationFailed(layout.paragraph))?;

        let mut scale_context = ScaleContext::new();
        let mut font_cache = BTreeMap::new();
        let mut raster_bytes = 0;
        for (run_index, run) in layout.runs.iter().enumerate() {
            let pixel_size = run.font_size * device_scale_factor;
            if !pixel_size.is_finite() || pixel_size <= 0.0 || pixel_size > MAX_GLYPH_SIZE_PX {
                return Err(TextLayoutError::RasterFontSizeOutOfRange {
                    paragraph: layout.paragraph,
                    font_family: run.font_family.clone(),
                });
            }
            let font_bytes = self
                .font_assets
                .get(&run.font_family)
                .ok_or_else(|| TextLayoutError::MissingRasterFont(run.font_family.clone()))?;
            let font = *font_cache
                .entry(run.font_family.as_str())
                .or_insert_with(|| FontRef::from_index(font_bytes.as_slice(), 0))
                .as_ref()
                .ok_or_else(|| TextLayoutError::InvalidRasterFont(run.font_family.clone()))?;
            if FontRef::from_index(font_bytes.as_slice(), 1).is_some() {
                return Err(TextLayoutError::InvalidRasterFont(run.font_family.clone()));
            }
            let font_glyph_count = font.glyph_metrics(&[]).glyph_count();
            let mut scaler = scale_context.builder(font).size(pixel_size).build();
            for (glyph_index, glyph) in run.glyphs.iter().enumerate() {
                if ![glyph.x, glyph.y, glyph.advance]
                    .iter()
                    .all(|value| value.is_finite())
                    || !super::valid_text_range(&glyph.cluster_text_range, &layout.text)
                    || glyph.sources.is_empty()
                    || glyph.sources.iter().any(|source| {
                        source.source_utf16.start >= source.source_utf16.end
                            || !super::valid_text_range(&source.paragraph_utf8, &layout.text)
                            || source.paragraph_utf8.start >= glyph.cluster_text_range.end
                            || glyph.cluster_text_range.start >= source.paragraph_utf8.end
                            || !source_is_from_layout(source, &layout.sources)
                    })
                {
                    return Err(TextLayoutError::InvalidGlyphLayout(layout.paragraph));
                }
                let glyph_id = GlyphId::try_from(glyph.id).map_err(|_| {
                    TextLayoutError::GlyphRasterizationFailed {
                        paragraph: layout.paragraph,
                        glyph_id: glyph.id,
                    }
                })?;
                if glyph_id == 0 || glyph_id >= font_glyph_count {
                    return Err(TextLayoutError::GlyphRasterizationFailed {
                        paragraph: layout.paragraph,
                        glyph_id: glyph.id,
                    });
                }
                let physical_x = glyph.x * device_scale_factor;
                let physical_baseline_y = glyph.y * device_scale_factor;
                let origin_x_px = pixel_anchor(physical_x, layout.paragraph)?;
                let baseline_y_px = pixel_anchor(physical_baseline_y, layout.paragraph)?;
                let offset = Vector::new(
                    physical_x - physical_x.floor(),
                    -(physical_baseline_y - physical_baseline_y.floor()),
                );
                let mut renderer = Render::new(RASTER_SOURCES);
                renderer.format(Format::Alpha).offset(offset);
                let bitmap = match renderer.render(&mut scaler, glyph_id) {
                    Some(image) => checked_bitmap(image, layout.paragraph, glyph.id)?,
                    None => None,
                };
                if let Some(bitmap) = &bitmap {
                    raster_bytes = accumulated_raster_bytes(
                        raster_bytes,
                        bitmap.pixels.len(),
                        layout.paragraph,
                    )?;
                }
                glyphs.push(TextGlyphRaster {
                    run_index,
                    glyph_index,
                    glyph_id: glyph.id,
                    x_css_px: glyph.x,
                    baseline_y_css_px: glyph.y,
                    origin_x_px,
                    baseline_y_px,
                    cluster_text_range: glyph.cluster_text_range.clone(),
                    sources: glyph.sources.clone(),
                    bitmap,
                });
            }
        }

        Ok(TextGlyphRasterSnapshot {
            paragraph: layout.paragraph,
            font_registry_revision: layout.font_registry_revision,
            device_scale_factor,
            glyphs,
        })
    }
}

pub(super) fn accumulated_raster_bytes(
    current: usize,
    additional: usize,
    paragraph: spinon_core::NodeId,
) -> Result<usize, TextLayoutError> {
    current
        .checked_add(additional)
        .filter(|total| *total <= MAX_GLYPH_SNAPSHOT_BYTES)
        .ok_or(TextLayoutError::RasterSnapshotTooLarge {
            paragraph,
            max_bytes: MAX_GLYPH_SNAPSHOT_BYTES,
        })
}

fn source_is_from_layout(
    source: &crate::TextSourceRange,
    layout_sources: &[crate::TextSourceRange],
) -> bool {
    let mut source_utf16 = source.source_utf16.start;
    let mut paragraph_utf8 = source.paragraph_utf8.start;
    for candidate in layout_sources.iter().filter(|candidate| {
        candidate.node == source.node && candidate.inline_ancestors == source.inline_ancestors
    }) {
        if candidate.source_utf16.start < source_utf16
            || candidate.paragraph_utf8.start < paragraph_utf8
        {
            continue;
        }
        if candidate.source_utf16.start != source_utf16
            || candidate.paragraph_utf8.start != paragraph_utf8
        {
            return false;
        }
        source_utf16 = candidate.source_utf16.end;
        paragraph_utf8 = candidate.paragraph_utf8.end;
        if source_utf16 == source.source_utf16.end && paragraph_utf8 == source.paragraph_utf8.end {
            return true;
        }
        if source_utf16 > source.source_utf16.end || paragraph_utf8 > source.paragraph_utf8.end {
            return false;
        }
    }
    false
}

fn pixel_anchor(value: f32, paragraph: spinon_core::NodeId) -> Result<i32, TextLayoutError> {
    let value = f64::from(value).floor();
    if !value.is_finite() || value < f64::from(i32::MIN) || value > f64::from(i32::MAX) {
        return Err(TextLayoutError::InvalidGlyphLayout(paragraph));
    }
    Ok(value as i32)
}

pub(super) fn checked_bitmap(
    image: swash::scale::image::Image,
    paragraph: spinon_core::NodeId,
    glyph_id: u32,
) -> Result<Option<GlyphBitmap>, TextLayoutError> {
    let content = match image.content {
        swash::scale::image::Content::Mask => (GlyphBitmapContent::AlphaMask, 1_usize),
        swash::scale::image::Content::SubpixelMask => {
            (GlyphBitmapContent::SubpixelMaskRgba8, 4_usize)
        }
        swash::scale::image::Content::Color => (GlyphBitmapContent::ColorRgba8, 4_usize),
    };
    let expected_bytes = usize::try_from(image.placement.width)
        .ok()
        .and_then(|width| {
            usize::try_from(image.placement.height)
                .ok()
                .and_then(|height| width.checked_mul(height))
        })
        .and_then(|pixels| pixels.checked_mul(content.1))
        .filter(|bytes| *bytes <= MAX_GLYPH_BITMAP_BYTES)
        .ok_or(TextLayoutError::InvalidGlyphBitmap {
            paragraph,
            glyph_id,
        })?;
    if expected_bytes != image.data.len() {
        return Err(TextLayoutError::InvalidGlyphBitmap {
            paragraph,
            glyph_id,
        });
    }
    if image.placement.width == 0 || image.placement.height == 0 {
        if image.data.is_empty() {
            return Ok(None);
        }
        return Err(TextLayoutError::InvalidGlyphBitmap {
            paragraph,
            glyph_id,
        });
    }
    let top_px = image
        .placement
        .top
        .checked_neg()
        .ok_or(TextLayoutError::InvalidGlyphBitmap {
            paragraph,
            glyph_id,
        })?;
    Ok(Some(GlyphBitmap {
        width_px: image.placement.width,
        height_px: image.placement.height,
        left_px: image.placement.left,
        top_px,
        content: content.0,
        pixels: image.data.into(),
    }))
}
