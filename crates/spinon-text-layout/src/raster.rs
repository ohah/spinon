use std::{ops::Range, sync::Arc};

use crate::{FontRegistryRevision, TextSourceRange};

/// Swash가 만든 glyph bitmap의 pixel content입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GlyphBitmapContent {
    /// 픽셀마다 8-bit coverage 값 하나를 보존합니다.
    AlphaMask,
    /// 픽셀마다 8-bit RGBA subpixel coverage를 보존합니다.
    SubpixelMaskRgba8,
    /// 픽셀마다 8-bit RGBA color를 보존합니다.
    ColorRgba8,
}

/// Glyph baseline 원점에 상대적인 고정 크기 bitmap입니다.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlyphBitmap {
    /// Physical pixel 폭입니다.
    pub width_px: u32,
    /// Physical pixel 높이입니다.
    pub height_px: u32,
    /// baseline 원점부터의 왼쪽 offset입니다.
    pub left_px: i32,
    /// baseline 원점부터의 top offset입니다. 화면 좌표처럼 아래 방향이 양수입니다.
    pub top_px: i32,
    pub content: GlyphBitmapContent,
    pub pixels: Arc<[u8]>,
}

/// Shaped glyph 한 개의 raster와 원본 text mapping입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct TextGlyphRaster {
    pub run_index: usize,
    pub glyph_index: usize,
    pub glyph_id: u32,
    /// Paragraph 좌표계의 glyph origin입니다.
    pub x_css_px: f32,
    /// Paragraph 좌표계의 baseline입니다.
    pub baseline_y_css_px: f32,
    /// Fractional raster phase를 적용하기 전의 baseline pixel anchor입니다.
    pub origin_x_px: i32,
    /// Fractional raster phase를 적용하기 전의 baseline pixel anchor입니다.
    pub baseline_y_px: i32,
    pub cluster_text_range: Range<usize>,
    pub sources: Vec<TextSourceRange>,
    /// 공백 또는 font의 유효한 빈 glyph는 bitmap이 없습니다.
    pub bitmap: Option<GlyphBitmap>,
}

/// Font registry와 DPR에 묶인 immutable raster 결과입니다.
#[derive(Clone, Debug)]
pub struct TextGlyphRasterSnapshot {
    pub paragraph: spinon_core::NodeId,
    pub font_registry_revision: FontRegistryRevision,
    pub device_scale_factor: f32,
    pub glyphs: Vec<TextGlyphRaster>,
}

impl PartialEq for TextGlyphRasterSnapshot {
    fn eq(&self, other: &Self) -> bool {
        self.paragraph == other.paragraph
            && self.font_registry_revision == other.font_registry_revision
            && self.device_scale_factor == other.device_scale_factor
            && self.glyphs == other.glyphs
    }
}
