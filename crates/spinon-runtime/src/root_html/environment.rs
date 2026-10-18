use super::{HTML_NAMESPACE, ParsedNodeKind, ParsedRootHtml};
use spinon_core::EnvironmentRevision;
use spinon_style::{
    ComputedBackgroundPaint, CssColorScheme, CssMediaEnvironment, CssViewport, OpaqueCssSrgb,
};

#[cfg(test)]
mod tests;

const DEFAULT_MOBILE_LAYOUT_WIDTH_CSS_PX: f32 = 980.0;
const MAX_VIEWPORT_WIDTH_CSS_PX: f32 = 100_000.0;
const MIN_INITIAL_SCALE: f32 = 0.01;
const MAX_INITIAL_SCALE: f32 = 10.0;

#[derive(Clone, Copy, Debug, PartialEq)]
enum ViewportWidth {
    Device,
    CssPx(f32),
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct ViewportMetadata {
    width: Option<ViewportWidth>,
    initial_scale: Option<f32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ColorSchemeMetadata {
    light: bool,
    dark: bool,
    only: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct RootHtmlEnvironmentMetadata {
    viewport: Option<ViewportMetadata>,
    color_scheme: Option<ColorSchemeMetadata>,
    duplicate_color_scheme_metadata: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ResolvedRootHtmlEnvironment {
    pub(crate) viewport_width_css_px: f32,
    pub(crate) viewport_height_css_px: f32,
    pub(crate) visual_viewport_width_css_px: f32,
    pub(crate) visual_viewport_height_css_px: f32,
    pub(crate) device_scale_factor: f32,
    pub(crate) page_scale: f32,
    pub(crate) prefers_color_scheme: CssColorScheme,
    pub(crate) document_color_scheme: CssColorScheme,
    pub(crate) media_environment: CssMediaEnvironment,
}

/// Root HTML metadata로 해석한 cascade viewport와 별도 초기 document scheme입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RootHtmlEnvironmentSnapshot {
    /// `meta viewport`를 적용한 CSS layout viewport 너비입니다.
    pub viewport_width_css_px: f32,
    /// `meta viewport`를 적용한 CSS layout viewport 높이입니다.
    pub viewport_height_css_px: f32,
    /// 초기 visual viewport 너비입니다. layout viewport보다 좁을 수 있습니다.
    pub visual_viewport_width_css_px: f32,
    /// 초기 visual viewport 높이입니다. layout viewport보다 낮을 수 있습니다.
    pub visual_viewport_height_css_px: f32,
    /// native surface에서 전달한 device scale입니다. Page scale과 곱해 저장하지 않습니다.
    pub device_scale_factor: f32,
    /// HTML `initial-scale` 또는 mobile overview mode에서 계산한 별도 page scale입니다.
    pub page_scale: f32,
    /// CSS `prefers-color-scheme`가 읽는 native OS/user preference입니다.
    pub prefers_color_scheme: CssColorScheme,
    /// metadata가 지정한 UA 초기 document scheme입니다. Canvas paint에는 아직 적용하지 않습니다.
    pub document_color_scheme: CssColorScheme,
    pub(crate) media_environment: CssMediaEnvironment,
    /// cascade 입력에 할당된 환경 revision입니다.
    pub environment_revision: EnvironmentRevision,
}

impl RootHtmlEnvironmentMetadata {
    pub(crate) fn from_document(document: &ParsedRootHtml) -> Self {
        let Some(head) = document.html_element().and_then(|html| {
            document.nodes[html]
                .children
                .iter()
                .copied()
                .find(|child| is_html_element(document, *child, "head"))
        }) else {
            return Self {
                viewport: None,
                color_scheme: None,
                duplicate_color_scheme_metadata: false,
            };
        };

        let mut viewport = None;
        let mut color_scheme = None;
        let mut color_scheme_metadata_count = 0_u8;
        for child in &document.nodes[head].children {
            if !is_html_element(document, *child, "meta") {
                continue;
            }
            let ParsedNodeKind::Element { attributes, .. } = &document.nodes[*child].kind else {
                continue;
            };
            let Some(name) = attribute(attributes, "name") else {
                continue;
            };
            match name.trim().to_ascii_lowercase().as_str() {
                "viewport" => {
                    viewport = Some(parse_viewport_metadata(attribute(attributes, "content")));
                }
                "color-scheme" => {
                    color_scheme_metadata_count = color_scheme_metadata_count.saturating_add(1);
                    if color_scheme_metadata_count == 1 {
                        color_scheme =
                            attribute(attributes, "content").and_then(parse_color_scheme_metadata);
                    }
                }
                _ => {}
            }
        }

        Self {
            viewport,
            color_scheme,
            duplicate_color_scheme_metadata: color_scheme_metadata_count > 1,
        }
    }

    pub(crate) fn has_duplicate_color_scheme_metadata(self) -> bool {
        self.duplicate_color_scheme_metadata
    }

    pub(crate) fn resolve_canvas_color_scheme(
        self,
        computed_root_value: &str,
        preference: CssColorScheme,
    ) -> Option<CssColorScheme> {
        resolve_root_html_canvas_color_scheme(self.color_scheme, computed_root_value, preference)
    }

    fn resolve(
        self,
        surface_width_px: f32,
        surface_height_px: f32,
        device_scale_factor: f32,
        media_environment: CssMediaEnvironment,
    ) -> Result<ResolvedRootHtmlEnvironment, String> {
        let input = CssViewport {
            width_css_px: surface_width_px,
            height_css_px: surface_height_px,
            device_scale_factor,
            environment_revision: EnvironmentRevision::INITIAL,
            media_environment,
        };
        if input.validate().is_err() {
            return Err("root HTML surface 환경 값이 유효하지 않습니다".to_owned());
        }
        let screen_width_css_px = surface_width_px / device_scale_factor;
        let screen_height_css_px = surface_height_px / device_scale_factor;
        let requested_width = self
            .viewport
            .and_then(|metadata| metadata.width)
            .map(|width| match width {
                ViewportWidth::Device => screen_width_css_px,
                ViewportWidth::CssPx(width) => width,
            })
            .unwrap_or(DEFAULT_MOBILE_LAYOUT_WIDTH_CSS_PX);
        let explicit_scale = self.viewport.and_then(|metadata| metadata.initial_scale);
        let page_scale =
            explicit_scale.unwrap_or_else(|| (screen_width_css_px / requested_width).min(1.0));
        let viewport_width_css_px = requested_width.max(screen_width_css_px / page_scale);
        let viewport_height_css_px = viewport_width_css_px * (surface_height_px / surface_width_px);
        let visual_viewport_width_css_px = screen_width_css_px / page_scale;
        let visual_viewport_height_css_px = screen_height_css_px / page_scale;
        let document_color_scheme = self
            .color_scheme
            .map(|metadata| metadata.resolve(media_environment.color_scheme))
            .unwrap_or(CssColorScheme::Light);

        if !viewport_width_css_px.is_finite()
            || viewport_width_css_px <= 0.0
            || viewport_width_css_px > MAX_VIEWPORT_WIDTH_CSS_PX
            || !viewport_height_css_px.is_finite()
            || viewport_height_css_px <= 0.0
            || !visual_viewport_width_css_px.is_finite()
            || visual_viewport_width_css_px <= 0.0
            || !visual_viewport_height_css_px.is_finite()
            || visual_viewport_height_css_px <= 0.0
            || !page_scale.is_finite()
            || !(MIN_INITIAL_SCALE..=MAX_INITIAL_SCALE).contains(&page_scale)
        {
            return Err("root HTML metadata가 허용된 CSS viewport 범위를 벗어났습니다".to_owned());
        }

        Ok(ResolvedRootHtmlEnvironment {
            viewport_width_css_px,
            viewport_height_css_px,
            visual_viewport_width_css_px,
            visual_viewport_height_css_px,
            device_scale_factor,
            page_scale,
            prefers_color_scheme: media_environment.color_scheme,
            document_color_scheme,
            media_environment,
        })
    }
}

impl ColorSchemeMetadata {
    fn resolve(self, preference: CssColorScheme) -> CssColorScheme {
        if self.only {
            return if self.dark {
                CssColorScheme::Dark
            } else {
                CssColorScheme::Light
            };
        }
        match preference {
            CssColorScheme::Light if self.light => CssColorScheme::Light,
            CssColorScheme::Dark if self.dark => CssColorScheme::Dark,
            _ if self.light => CssColorScheme::Light,
            _ => CssColorScheme::Dark,
        }
    }
}

pub(crate) fn resolve_root_html_environment(
    metadata: RootHtmlEnvironmentMetadata,
    surface_width_px: f32,
    surface_height_px: f32,
    device_scale_factor: f32,
    media_environment: CssMediaEnvironment,
    environment_revision: EnvironmentRevision,
) -> Result<RootHtmlEnvironmentSnapshot, String> {
    let resolved = metadata.resolve(
        surface_width_px,
        surface_height_px,
        device_scale_factor,
        media_environment,
    )?;
    Ok(RootHtmlEnvironmentSnapshot {
        viewport_width_css_px: resolved.viewport_width_css_px,
        viewport_height_css_px: resolved.viewport_height_css_px,
        visual_viewport_width_css_px: resolved.visual_viewport_width_css_px,
        visual_viewport_height_css_px: resolved.visual_viewport_height_css_px,
        device_scale_factor: resolved.device_scale_factor,
        page_scale: resolved.page_scale,
        prefers_color_scheme: resolved.prefers_color_scheme,
        document_color_scheme: resolved.document_color_scheme,
        media_environment: resolved.media_environment,
        environment_revision,
    })
}

fn is_html_element(document: &ParsedRootHtml, node: usize, local_name: &str) -> bool {
    matches!(
        &document.nodes[node].kind,
        ParsedNodeKind::Element {
            namespace,
            local_name: name,
            ..
        } if namespace == HTML_NAMESPACE && name == local_name
    )
}

fn attribute<'a>(attributes: &'a [(String, String)], name: &str) -> Option<&'a str> {
    attributes
        .iter()
        .find(|(attribute_name, _)| attribute_name.eq_ignore_ascii_case(name))
        .map(|(_, value)| value.as_str())
}

fn parse_viewport_metadata(content: Option<&str>) -> ViewportMetadata {
    let mut metadata = ViewportMetadata::default();
    for descriptor in content
        .into_iter()
        .flat_map(|content| content.split([',', ';']))
    {
        let Some((name, value)) = descriptor.split_once('=') else {
            continue;
        };
        let name = name.trim();
        let value = value.trim();
        match name.to_ascii_lowercase().as_str() {
            "width" => {
                let parsed = if value.eq_ignore_ascii_case("device-width") {
                    Some(ViewportWidth::Device)
                } else {
                    parse_positive_number(value, MAX_VIEWPORT_WIDTH_CSS_PX)
                        .map(ViewportWidth::CssPx)
                };
                if let Some(width) = parsed {
                    metadata.width = Some(width);
                }
            }
            "initial-scale" => {
                if let Some(scale) = parse_positive_number(value, MAX_INITIAL_SCALE)
                    .filter(|scale| *scale >= MIN_INITIAL_SCALE)
                {
                    metadata.initial_scale = Some(scale);
                }
            }
            _ => {}
        }
    }
    metadata
}

fn parse_positive_number(value: &str, maximum: f32) -> Option<f32> {
    let number = value.parse::<f32>().ok()?;
    (number.is_finite() && number > 0.0 && number <= maximum).then_some(number)
}

fn parse_color_scheme_metadata(content: &str) -> Option<ColorSchemeMetadata> {
    let mut tokens = content.split_ascii_whitespace();
    let first = tokens.next()?.to_ascii_lowercase();
    let (only, first_scheme) = if first == "only" {
        (true, tokens.next()?.to_ascii_lowercase())
    } else {
        (false, first)
    };
    let mut metadata = ColorSchemeMetadata {
        light: false,
        dark: false,
        only,
    };
    for token in std::iter::once(first_scheme).chain(tokens.map(str::to_ascii_lowercase)) {
        match token.as_str() {
            "light" if !metadata.only || !metadata.light && !metadata.dark => {
                metadata.light = true;
            }
            "dark" if !metadata.only || !metadata.light && !metadata.dark => {
                metadata.dark = true;
            }
            _ => return None,
        }
    }
    (metadata.light || metadata.dark).then_some(metadata)
}

pub(crate) fn resolve_root_html_canvas_background(
    html_background: ComputedBackgroundPaint,
    body_background: Option<ComputedBackgroundPaint>,
    fallback_scheme: CssColorScheme,
) -> Option<[u8; 4]> {
    let to_rgba = |color: OpaqueCssSrgb| [color.red, color.green, color.blue, 255];
    match html_background {
        ComputedBackgroundPaint::Opaque(color) => Some(to_rgba(color)),
        ComputedBackgroundPaint::Translucent(_) => None,
        ComputedBackgroundPaint::Transparent => match body_background {
            Some(ComputedBackgroundPaint::Opaque(color)) => Some(to_rgba(color)),
            Some(ComputedBackgroundPaint::Translucent(_)) => None,
            Some(ComputedBackgroundPaint::Transparent) | None => Some(match fallback_scheme {
                CssColorScheme::Light => [255, 255, 255, 255],
                CssColorScheme::Dark => [18, 18, 18, 255],
            }),
        },
    }
}

fn resolve_root_html_canvas_color_scheme(
    metadata: Option<ColorSchemeMetadata>,
    computed_root_value: &str,
    preference: CssColorScheme,
) -> Option<CssColorScheme> {
    if computed_root_value.trim().eq_ignore_ascii_case("normal") {
        return Some(
            metadata
                .map(|schemes| schemes.resolve(preference))
                .unwrap_or(CssColorScheme::Light),
        );
    }
    parse_color_scheme_metadata(computed_root_value).map(|schemes| schemes.resolve(preference))
}
