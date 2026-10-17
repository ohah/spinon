use std::sync::Arc;

use spinon_core::NodeId;

use super::{CssClip, RuntimeRenderError, RuntimeRenderKey, RuntimeSrgbColor};

/// WGPU가 한 글리프의 coverage mask로 사용할 고정 raster bitmap입니다.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeGlyphBitmap {
    width_px: u32,
    height_px: u32,
    origin_x_px: i32,
    baseline_y_px: i32,
    left_px: i32,
    top_px: i32,
    pixels: Arc<[u8]>,
}

impl RuntimeGlyphBitmap {
    pub fn new(
        width_px: u32,
        height_px: u32,
        origin_x_px: i32,
        baseline_y_px: i32,
        left_px: i32,
        top_px: i32,
        pixels: Arc<[u8]>,
    ) -> Result<Self, RuntimeRenderError> {
        let expected_bytes = u64::from(width_px)
            .checked_mul(u64::from(height_px))
            .and_then(|pixels| usize::try_from(pixels).ok());
        if width_px == 0
            || height_px == 0
            || expected_bytes != Some(pixels.len())
            || pixels.len() > MAX_RUNTIME_GLYPH_BITMAP_BYTES
        {
            return Err(RuntimeRenderError::InvalidTextBitmap);
        }
        Ok(Self {
            width_px,
            height_px,
            origin_x_px,
            baseline_y_px,
            left_px,
            top_px,
            pixels,
        })
    }

    pub const fn width_px(&self) -> u32 {
        self.width_px
    }

    pub const fn height_px(&self) -> u32 {
        self.height_px
    }

    pub const fn origin_x_px(&self) -> i32 {
        self.origin_x_px
    }

    pub const fn baseline_y_px(&self) -> i32 {
        self.baseline_y_px
    }

    pub const fn left_px(&self) -> i32 {
        self.left_px
    }

    pub const fn top_px(&self) -> i32 {
        self.top_px
    }

    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }
}

/// 하나의 paragraph raster 결과를 revision·색·clip과 함께 paint snapshot에 보관합니다.
#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeRenderText {
    node_id: NodeId,
    source: RuntimeRenderKey,
    base_origin_x_css_px: f32,
    base_origin_y_css_px: f32,
    origin_x_css_px: f32,
    origin_y_css_px: f32,
    device_scale_factor: f32,
    color: RuntimeSrgbColor,
    base_clip_css_px: CssClip,
    clip_css_px: CssClip,
    glyphs: Arc<[RuntimeGlyphBitmap]>,
}

impl RuntimeRenderText {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        node_id: NodeId,
        source: RuntimeRenderKey,
        origin_x_css_px: f32,
        origin_y_css_px: f32,
        device_scale_factor: f32,
        color: RuntimeSrgbColor,
        clip_css_px: CssClip,
        glyphs: Vec<RuntimeGlyphBitmap>,
    ) -> Result<Self, RuntimeRenderError> {
        if !origin_x_css_px.is_finite()
            || !origin_y_css_px.is_finite()
            || !device_scale_factor.is_finite()
            || device_scale_factor <= 0.0
        {
            return Err(RuntimeRenderError::InvalidTextGeometry);
        }
        let total_bytes = glyphs.iter().try_fold(0_usize, |total, glyph| {
            total.checked_add(glyph.pixels.len())
        });
        if total_bytes.is_none_or(|bytes| bytes > MAX_RUNTIME_TEXT_BYTES)
            || glyphs.len() > MAX_RUNTIME_GLYPHS
        {
            return Err(RuntimeRenderError::InvalidTextBitmap);
        }
        Ok(Self {
            node_id,
            source,
            base_origin_x_css_px: origin_x_css_px,
            base_origin_y_css_px: origin_y_css_px,
            origin_x_css_px,
            origin_y_css_px,
            device_scale_factor,
            color,
            base_clip_css_px: clip_css_px,
            clip_css_px,
            glyphs: glyphs.into(),
        })
    }

    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub const fn source(&self) -> RuntimeRenderKey {
        self.source
    }

    pub const fn origin_x_css_px(&self) -> f32 {
        self.origin_x_css_px
    }

    pub const fn origin_y_css_px(&self) -> f32 {
        self.origin_y_css_px
    }

    pub const fn device_scale_factor(&self) -> f32 {
        self.device_scale_factor
    }

    pub const fn color(&self) -> RuntimeSrgbColor {
        self.color
    }

    pub const fn clip_css_px(&self) -> CssClip {
        self.clip_css_px
    }

    pub(super) fn apply_scroll_transform(
        &mut self,
        scroll_x_css_px: f32,
        scroll_y_css_px: f32,
        owner_clip_css_px: CssClip,
    ) -> Result<(), RuntimeRenderError> {
        let origin_x_css_px = self.base_origin_x_css_px - scroll_x_css_px;
        let origin_y_css_px = self.base_origin_y_css_px - scroll_y_css_px;
        if !origin_x_css_px.is_finite() || !origin_y_css_px.is_finite() {
            return Err(RuntimeRenderError::InvalidTextGeometry);
        }
        self.origin_x_css_px = origin_x_css_px;
        self.origin_y_css_px = origin_y_css_px;
        self.clip_css_px = self.base_clip_css_px.intersect(owner_clip_css_px);
        Ok(())
    }

    pub fn glyphs(&self) -> &[RuntimeGlyphBitmap] {
        &self.glyphs
    }
}

const MAX_RUNTIME_GLYPH_BITMAP_BYTES: usize = 16 * 1024 * 1024;
const MAX_RUNTIME_TEXT_BYTES: usize = 64 * 1024 * 1024;
const MAX_RUNTIME_GLYPHS: usize = 1_000_000;
