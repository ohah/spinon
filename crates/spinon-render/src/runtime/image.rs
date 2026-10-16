use std::sync::Arc;

use spinon_core::{
    DocumentGeneration, DocumentRevision, IntrinsicSizeRevision, NodeId, RenderTreeRevision,
    StyleRevision,
};

use super::{CssRect, RuntimeRenderError, RuntimeRenderKey};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RuntimeImageObjectFit {
    Fill,
    Contain,
    Cover,
    None,
    ScaleDown,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RuntimeImageObjectPosition {
    x_percent: f32,
    y_percent: f32,
}

impl RuntimeImageObjectPosition {
    pub fn new(x_percent: f32, y_percent: f32) -> Result<Self, String> {
        if !x_percent.is_finite() || !y_percent.is_finite() {
            return Err("object-position percentage는 유한한 수여야 합니다".to_owned());
        }
        Ok(Self {
            x_percent,
            y_percent,
        })
    }

    pub const fn x_percent(self) -> f32 {
        self.x_percent
    }

    pub const fn y_percent(self) -> f32 {
        self.y_percent
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RuntimeImageSampling {
    Nearest,
    Linear,
    /// CSS `image-rendering: auto` sampling used by the Chrome 155 profile.
    LinearEncoded,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RuntimeImageObjectStyle {
    fit: RuntimeImageObjectFit,
    position: RuntimeImageObjectPosition,
    sampling: RuntimeImageSampling,
}

impl RuntimeImageObjectStyle {
    pub fn parse(fit: &str, position: &str, rendering: &str) -> Result<Self, String> {
        let fit = match fit {
            "fill" => RuntimeImageObjectFit::Fill,
            "contain" => RuntimeImageObjectFit::Contain,
            "cover" => RuntimeImageObjectFit::Cover,
            "none" => RuntimeImageObjectFit::None,
            "scale-down" => RuntimeImageObjectFit::ScaleDown,
            _ => return Err(format!("지원하지 않는 object-fit computed value: {fit}")),
        };
        let mut components = position.split_whitespace();
        let x_percent = parse_percentage(components.next(), position)?;
        let y_percent = parse_percentage(components.next(), position)?;
        if components.next().is_some() {
            return Err(format!(
                "object-position은 물리 축 percentage 두 개여야 합니다: {position}"
            ));
        }
        if rendering != "auto" {
            return Err(format!(
                "지원하지 않는 image-rendering computed value: {rendering}"
            ));
        }
        let position = RuntimeImageObjectPosition::new(x_percent, y_percent)?;
        Ok(Self {
            fit,
            position,
            sampling: RuntimeImageSampling::LinearEncoded,
        })
    }

    pub const fn fit(self) -> RuntimeImageObjectFit {
        self.fit
    }

    pub const fn position(self) -> RuntimeImageObjectPosition {
        self.position
    }

    pub const fn sampling(self) -> RuntimeImageSampling {
        self.sampling
    }
}

fn parse_percentage(component: Option<&str>, value: &str) -> Result<f32, String> {
    let component = component
        .ok_or_else(|| format!("object-position은 물리 축 percentage 두 개여야 합니다: {value}"))?;
    let number = component
        .strip_suffix('%')
        .ok_or_else(|| format!("object-position은 percentage만 지원합니다: {value}"))?;
    let parsed = number
        .parse::<f32>()
        .map_err(|_| format!("object-position percentage가 유효하지 않습니다: {value}"))?;
    if !parsed.is_finite() {
        return Err(format!(
            "object-position percentage가 유한하지 않습니다: {value}"
        ));
    }
    Ok(parsed)
}

/// 디코드 이미지 자원을 요청한 문서·연결 트리·intrinsic-size revision입니다.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RuntimeImageResourceKey {
    generation: DocumentGeneration,
    document_revision: DocumentRevision,
    render_tree_revision: RenderTreeRevision,
    intrinsic_size_revision: IntrinsicSizeRevision,
}

impl RuntimeImageResourceKey {
    pub const fn new(
        generation: DocumentGeneration,
        document_revision: DocumentRevision,
        render_tree_revision: RenderTreeRevision,
        intrinsic_size_revision: IntrinsicSizeRevision,
    ) -> Self {
        Self {
            generation,
            document_revision,
            render_tree_revision,
            intrinsic_size_revision,
        }
    }

    pub const fn matches(self, key: RuntimeRenderKey) -> bool {
        self.generation.get() == key.generation().get()
            && self.document_revision.get() == key.document_revision().get()
            && self.render_tree_revision.get() == key.render_tree_revision().get()
            && self.intrinsic_size_revision.get() == key.intrinsic_size_revision().get()
    }

    pub const fn generation(self) -> DocumentGeneration {
        self.generation
    }

    pub const fn document_revision(self) -> DocumentRevision {
        self.document_revision
    }

    pub const fn render_tree_revision(self) -> RenderTreeRevision {
        self.render_tree_revision
    }

    pub const fn intrinsic_size_revision(self) -> IntrinsicSizeRevision {
        self.intrinsic_size_revision
    }
}

/// 자연 크기 RGBA8 픽셀을 source revision에 묶어 전달합니다.
#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeRenderImage {
    node_id: NodeId,
    source: RuntimeImageResourceKey,
    width: u32,
    height: u32,
    rgba8: Arc<[u8]>,
    object_style: Option<RuntimeImageObjectStyle>,
    object_style_revision: Option<StyleRevision>,
}

impl RuntimeRenderImage {
    pub fn new(
        node_id: NodeId,
        source: RuntimeImageResourceKey,
        width: u32,
        height: u32,
        rgba8: Arc<[u8]>,
    ) -> Result<Self, RuntimeRenderError> {
        Self::new_with_optional_object_style(node_id, source, width, height, rgba8, None, None)
    }

    pub fn new_with_object_style(
        node_id: NodeId,
        source: RuntimeImageResourceKey,
        width: u32,
        height: u32,
        rgba8: Arc<[u8]>,
        object_style: RuntimeImageObjectStyle,
        style_revision: StyleRevision,
    ) -> Result<Self, RuntimeRenderError> {
        Self::new_with_optional_object_style(
            node_id,
            source,
            width,
            height,
            rgba8,
            Some(object_style),
            Some(style_revision),
        )
    }

    fn new_with_optional_object_style(
        node_id: NodeId,
        source: RuntimeImageResourceKey,
        width: u32,
        height: u32,
        rgba8: Arc<[u8]>,
        object_style: Option<RuntimeImageObjectStyle>,
        object_style_revision: Option<StyleRevision>,
    ) -> Result<Self, RuntimeRenderError> {
        let expected_bytes = width
            .checked_mul(height)
            .and_then(|pixels| pixels.checked_mul(4))
            .and_then(|bytes| usize::try_from(bytes).ok());
        if width == 0 || height == 0 || expected_bytes != Some(rgba8.len()) {
            return Err(RuntimeRenderError::InvalidImageBytes(node_id));
        }
        Ok(Self {
            node_id,
            source,
            width,
            height,
            rgba8,
            object_style,
            object_style_revision,
        })
    }

    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub const fn source(&self) -> RuntimeImageResourceKey {
        self.source
    }

    pub const fn width(&self) -> u32 {
        self.width
    }

    pub const fn height(&self) -> u32 {
        self.height
    }

    pub fn rgba8(&self) -> &[u8] {
        &self.rgba8
    }

    pub const fn object_style(&self) -> Option<RuntimeImageObjectStyle> {
        self.object_style
    }

    pub const fn object_style_revision(&self) -> Option<StyleRevision> {
        self.object_style_revision
    }

    pub const fn sampling(&self) -> RuntimeImageSampling {
        match self.object_style {
            Some(style) => style.sampling(),
            None => RuntimeImageSampling::Nearest,
        }
    }

    pub fn object_rect_css_px(&self, frame: CssRect) -> Result<CssRect, RuntimeRenderError> {
        let fw = f64::from(frame.width());
        let fh = f64::from(frame.height());
        let fx = f64::from(frame.x());
        let fy = f64::from(frame.y());
        if !fw.is_finite() || fw <= 0.0 || !fh.is_finite() || fh <= 0.0 {
            return Err(RuntimeRenderError::InvalidImageGeometry(self.node_id));
        }
        let Some(style) = self.object_style else {
            if fw != f64::from(self.width) || fh != f64::from(self.height) {
                return Err(RuntimeRenderError::InvalidImageFrame(self.node_id));
            }
            return Ok(frame);
        };
        let nw = f64::from(self.width);
        let nh = f64::from(self.height);
        let contain_scale = (fw / nw).min(fh / nh);
        let cover_scale = (fw / nw).max(fh / nh);
        let (width, height) = match style.fit() {
            RuntimeImageObjectFit::Fill => (fw, fh),
            RuntimeImageObjectFit::Contain => (nw * contain_scale, nh * contain_scale),
            RuntimeImageObjectFit::Cover => (nw * cover_scale, nh * cover_scale),
            RuntimeImageObjectFit::None => (nw, nh),
            RuntimeImageObjectFit::ScaleDown if contain_scale < 1.0 => {
                (nw * contain_scale, nh * contain_scale)
            }
            RuntimeImageObjectFit::ScaleDown => (nw, nh),
        };
        let position = style.position();
        let x = fx + (fw - width) * f64::from(position.x_percent()) / 100.0;
        let y = fy + (fh - height) * f64::from(position.y_percent()) / 100.0;
        let rect = CssRect::new(x as f32, y as f32, width as f32, height as f32)
            .map_err(|_| RuntimeRenderError::InvalidImageGeometry(self.node_id))?;
        if ![rect.x(), rect.y(), rect.width(), rect.height()]
            .into_iter()
            .all(f32::is_finite)
            || rect.width() <= 0.0
            || rect.height() <= 0.0
            || !(rect.x() + rect.width()).is_finite()
            || !(rect.y() + rect.height()).is_finite()
        {
            return Err(RuntimeRenderError::InvalidImageGeometry(self.node_id));
        }
        Ok(rect)
    }
}
