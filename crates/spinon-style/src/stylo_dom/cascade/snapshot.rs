use std::collections::BTreeMap;
use std::sync::Arc;

use spinon_core::{
    DocumentGeneration, DocumentRevision, EnvironmentRevision, NodeId, RenderTreeRevision,
    StyleRevision,
};

use crate::{ComputedBackgroundPaint, CssParseDiagnostic, OpaqueCssSrgb};

/// CSS 계산에 적용할 viewport와 플랫폼 환경 snapshot 출처입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssViewport {
    pub width_css_px: f32,
    pub height_css_px: f32,
    pub device_scale_factor: f32,
    /// 이 viewport 값이 속한 플랫폼 환경 snapshot의 revision입니다.
    pub environment_revision: EnvironmentRevision,
    /// CSS media query 계산에 사용할 색상·포인터 환경입니다.
    pub media_environment: CssMediaEnvironment,
}

impl CssViewport {
    pub const C04_FIXTURE: Self = Self {
        width_css_px: 800.0,
        height_css_px: 600.0,
        device_scale_factor: 1.0,
        environment_revision: EnvironmentRevision::INITIAL,
        media_environment: CssMediaEnvironment::DESKTOP,
    };

    pub(crate) fn is_valid(self) -> bool {
        let device_width = self.width_css_px * self.device_scale_factor;
        let device_height = self.height_css_px * self.device_scale_factor;
        self.width_css_px.is_finite()
            && self.width_css_px > 0.0
            && self.height_css_px.is_finite()
            && self.height_css_px > 0.0
            && self.device_scale_factor.is_finite()
            && self.device_scale_factor > 0.0
            && device_width.is_finite()
            && device_width > 0.0
            && device_height.is_finite()
            && device_height > 0.0
    }

    /// 런타임 환경 입력이 CSS cascade에 전달 가능한지 검증합니다.
    pub fn validate(self) -> Result<(), super::CssCascadeError> {
        if !self.is_valid() {
            return Err(super::CssCascadeError::InvalidViewport);
        }
        if !self.media_environment.is_valid() {
            return Err(super::CssCascadeError::InvalidMediaEnvironment);
        }
        Ok(())
    }
}

/// CSS `prefers-color-scheme` media feature 입력입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssColorScheme {
    Light,
    Dark,
}

/// CSS `pointer` media feature의 primary input 종류입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssPrimaryPointer {
    None,
    Coarse,
    Fine,
}

/// CSS `any-pointer`·`any-hover`에 전달할 전체 포인터 장치 기능입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CssPointerCapabilities {
    pub coarse: bool,
    pub fine: bool,
    pub hover: bool,
}

/// 한 cascade 요청에서 고정해 사용할 CSS media 환경 snapshot입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CssMediaEnvironment {
    pub color_scheme: CssColorScheme,
    pub primary_pointer: CssPrimaryPointer,
    pub primary_hover: bool,
    pub all_pointers: CssPointerCapabilities,
}

impl CssMediaEnvironment {
    /// 데스크톱 Chromium fixture의 기본 입력입니다.
    pub const DESKTOP: Self = Self {
        color_scheme: CssColorScheme::Light,
        primary_pointer: CssPrimaryPointer::Fine,
        primary_hover: true,
        all_pointers: CssPointerCapabilities {
            coarse: false,
            fine: true,
            hover: true,
        },
    };

    /// 터치 중심 모바일 Chromium fixture의 기본 입력입니다.
    pub const MOBILE: Self = Self {
        color_scheme: CssColorScheme::Light,
        primary_pointer: CssPrimaryPointer::Coarse,
        primary_hover: false,
        all_pointers: CssPointerCapabilities {
            coarse: true,
            fine: false,
            hover: false,
        },
    };

    pub(crate) fn is_valid(self) -> bool {
        let primary_kind_is_available = match self.primary_pointer {
            CssPrimaryPointer::None => !self.primary_hover,
            CssPrimaryPointer::Coarse => self.all_pointers.coarse,
            CssPrimaryPointer::Fine => self.all_pointers.fine,
        };
        primary_kind_is_available
            && (!self.primary_hover || self.all_pointers.hover)
            && (!self.all_pointers.hover || self.all_pointers.coarse || self.all_pointers.fine)
    }
}

/// 레이아웃 adapter에 보존하는 Stylo의 typed CSS 크기 값입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ComputedCssDimension {
    Auto,
    LengthPx(f32),
    Percentage(f32),
    Unsupported,
}

/// Stylo에서 추출한 `max-width`·`max-height`의 typed 값입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ComputedCssMaxSize {
    None,
    LengthPx(f32),
    Percentage(f32),
    Unsupported,
}

/// 하나의 Stylo computed-style revision에서 추출한 CSS 크기 값입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputedLayoutDimensions {
    pub width: ComputedCssDimension,
    pub height: ComputedCssDimension,
    pub min_width: ComputedCssDimension,
    pub max_width: ComputedCssMaxSize,
    pub min_height: ComputedCssDimension,
    pub max_height: ComputedCssMaxSize,
    pub flex_basis: ComputedCssDimension,
}

/// Stylo에서 추출한 간격 값이며 레이아웃 어댑터에서 px 문자열로 재해석하지 않습니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ComputedCssSpacingValue {
    LengthPx(f32),
    Percentage(f32),
    Auto,
    Normal,
    Unsupported,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputedCssEdges {
    pub top: ComputedCssSpacingValue,
    pub right: ComputedCssSpacingValue,
    pub bottom: ComputedCssSpacingValue,
    pub left: ComputedCssSpacingValue,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputedLayoutSpacing {
    pub margin: ComputedCssEdges,
    pub padding: ComputedCssEdges,
    pub row_gap: ComputedCssSpacingValue,
    pub column_gap: ComputedCssSpacingValue,
}

/// Stylo border style gate를 적용한 면별 used width이며 단위는 CSS px입니다.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ComputedLayoutBorder {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

/// Stylo Typed OM에서 소유형식으로 복사한 `<length-percentage>` 계산식입니다.
#[derive(Clone, Debug, PartialEq)]
pub enum ComputedCssMath {
    Number(f32),
    LengthPx(f32),
    Percentage(f32),
    Sum(Vec<Self>),
    Product(Vec<Self>),
    Negate(Box<Self>),
    Invert(Box<Self>),
    Min(Vec<Self>),
    Max(Vec<Self>),
    Clamp {
        min: Box<Self>,
        value: Box<Self>,
        max: Box<Self>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct ComputedElementStyle {
    pub node_id: NodeId,
    pub properties: BTreeMap<String, String>,
    /// 같은 cascade 결과에서 얻은 computed `font-size` CSS px입니다.
    pub font_size_css_px: f32,
    /// `properties`와 같은 계산 결과에서 추출한 typed width·height·flex-basis입니다.
    pub layout_dimensions: ComputedLayoutDimensions,
    /// 같은 Stylo computed-style revision에서 추출한 margin·padding·gap 값입니다.
    pub layout_spacing: ComputedLayoutSpacing,
    /// 같은 cascade 결과에서 `none`·`hidden` gate를 적용한 면별 border 폭입니다.
    pub layout_border: ComputedLayoutBorder,
    /// Stylo가 reify한 layout 계산식입니다. 문자열이 아니라 Typed OM 트리에서 복사했습니다.
    pub layout_math_values: BTreeMap<String, ComputedCssMath>,
    /// S04 paint profile에서만 설정하는 Stylo 계산 배경색입니다.
    pub background_color: Option<OpaqueCssSrgb>,
    /// S04 또는 런타임 paint profile에서만 설정하는 투명/불투명 paint입니다.
    pub background_paint: Option<ComputedBackgroundPaint>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CascadeDiagnostic {
    pub source_id: String,
    pub node_id: Option<NodeId>,
    pub diagnostic: CssParseDiagnostic,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ComputedStyleSnapshot {
    pub profile: ComputedStyleProfile,
    pub viewport: CssViewport,
    pub style_revision: StyleRevision,
    pub generation: DocumentGeneration,
    pub document_revision: DocumentRevision,
    pub render_tree_revision: RenderTreeRevision,
    /// Stylo document root의 computed font-size CSS px입니다. synthetic HTML root도 포함합니다.
    pub document_root_font_size_css_px: f32,
    pub elements: Arc<[ComputedElementStyle]>,
    pub diagnostics: Vec<CascadeDiagnostic>,
}

/// computed-style snapshot을 만든 whitelist profile입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComputedStyleProfile {
    /// 컴파일 시 포함한 지원 HTML 요소 UA 규칙의 기본 computed-style snapshot입니다.
    SupportedElementsUaV1,
    /// C04.1의 5개 cascade 비교 속성입니다.
    BasicCascadeV1,
    /// C04.2의 제한 Taffy Flex 입력 속성입니다.
    FlexLayoutV1,
    /// C04.6의 Flex 입력과 네 방향 CSS margin입니다.
    FlexMarginV1,
    /// C04.7의 Flex margin 입력과 제한 scheme/pointer media query입니다.
    FlexMediaEnvironmentV1,
    /// C04.3의 Flex 입력과 제한 정렬 속성입니다.
    FlexAlignmentV1,
    /// C04.4의 Flex 정렬 입력과 제한 Cascade Layers입니다.
    FlexAlignmentCascadeLayersV1,
    /// S04의 Flex layout 속성과 불투명 `#RRGGBB` 배경 페인트입니다.
    S04FlexPaintV1,
    /// C04 runtime UA snapshot과 제한 CSS layout 입력을 한 cascade에서 계산합니다.
    RuntimeFlexLayoutV1,
    /// C04.10 runtime layout 입력과 완전 투명 또는 불투명 단색 배경 paint입니다.
    RuntimeFlexPaintV1,
    /// C05.1 사용자 지정 속성을 계산한 제한 runtime layout profile입니다.
    RuntimeFlexCustomPropertiesV1,
    /// C05.1 사용자 지정 속성과 단색 배경 paint를 계산한 제한 runtime profile입니다.
    RuntimeFlexCustomPropertiesPaintV1,
    /// C05.2 등록 사용자 지정 속성을 계산한 제한 runtime layout profile입니다.
    RuntimeFlexRegisteredPropertiesV1,
    /// C05.2 등록 사용자 지정 속성과 단색 배경 paint를 계산한 제한 runtime profile입니다.
    RuntimeFlexRegisteredPropertiesPaintV1,
}
