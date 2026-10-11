use spinon_core::{NodeId, StyleRevision};
use std::{error::Error, fmt};
use style::properties::LonghandId;

#[cfg(test)]
use crate::UA_STYLESHEET;
use crate::{StylesheetRegistryError, StylesheetSource};

use super::StyloDocumentView;
mod device;
mod incremental;
mod margin;
mod runtime_block_display;
mod runtime_layout;
mod runtime_paint;
mod s04;
mod snapshot;
mod ua_baseline;

pub use incremental::{
    RuntimeCascadeReuseStats, compute_runtime_incremental_cascade_with_stylesheets,
};
pub use runtime_block_display::{
    first_unsupported_runtime_block_display_inline_value,
    first_unsupported_runtime_block_display_stylesheet_value,
};
pub use runtime_layout::{
    compute_runtime_flex_custom_properties_cascade,
    compute_runtime_flex_custom_properties_cascade_with_stylesheets,
    compute_runtime_flex_layout_cascade,
    compute_runtime_flex_registered_properties_cascade_with_stylesheets,
    first_unsupported_runtime_custom_properties_inline_property,
    first_unsupported_runtime_custom_properties_paint_inline_property,
    first_unsupported_runtime_layout_inline_property,
};
pub use runtime_paint::{
    compute_runtime_block_formatting_cascade_with_stylesheets, compute_runtime_block_paint_cascade,
    compute_runtime_block_paint_cascade_with_stylesheets,
    compute_runtime_block_positioning_cascade_with_stylesheets,
    compute_runtime_flex_custom_properties_paint_cascade,
    compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets,
    compute_runtime_flex_paint_cascade,
    compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets,
    first_unsupported_runtime_block_formatting_inline_property,
    first_unsupported_runtime_block_paint_inline_property,
    first_unsupported_runtime_block_positioning_inline_property,
    first_unsupported_runtime_flex_paint_inline_property,
};

#[cfg(test)]
#[path = "cascade/ua_baseline_tests.rs"]
mod ua_baseline_tests;

#[cfg(test)]
#[path = "cascade/ua_incremental_profile_tests.rs"]
mod ua_incremental_profile_tests;

#[cfg(test)]
#[path = "cascade/font_relative_units_tests.rs"]
mod font_relative_units_tests;

#[cfg(test)]
#[path = "cascade/container_relative_units_tests.rs"]
mod container_relative_units_tests;

#[cfg(test)]
#[path = "cascade/typed_css_math_tests.rs"]
mod typed_css_math_tests;

pub use margin::compute_flex_margin_cascade;
pub use margin::compute_flex_media_environment_cascade;
pub use snapshot::{
    CascadeDiagnostic, ComputedCssDimension, ComputedCssEdges, ComputedCssMath, ComputedCssMaxSize,
    ComputedCssPosition, ComputedCssSpacingValue, ComputedElementStyle, ComputedLayoutBorder,
    ComputedLayoutDimensions, ComputedLayoutInsets, ComputedLayoutSpacing, ComputedStyleProfile,
    ComputedStyleSnapshot, CssColorScheme, CssMediaEnvironment, CssPointerCapabilities,
    CssPrimaryPointer, CssViewport, FixedContainingBlockEffect,
};
pub use ua_baseline::compute_supported_elements_ua_cascade;

const UA_STYLESHEET_ID: &str = "spinon-ua-supported-elements-v0";
const UA_STYLESHEET_URL: &str = "https://spinon.invalid/ua/supported-elements-v0.css";
// C04.1 전용 계산 profile은 fixture가 소비하며 제품 runtime 연결은 아직 없습니다.
#[allow(dead_code)]
const BASIC_CASCADE_PROPERTIES: &[(&str, LonghandId)] = &[
    ("display", LonghandId::Display),
    ("color", LonghandId::Color),
    ("font-size", LonghandId::FontSize),
    ("font-weight", LonghandId::FontWeight),
    ("margin-top", LonghandId::MarginTop),
];

const FLEX_LAYOUT_PROPERTIES: &[(&str, LonghandId)] = &[
    ("display", LonghandId::Display),
    ("box-sizing", LonghandId::BoxSizing),
    ("width", LonghandId::Width),
    ("height", LonghandId::Height),
    ("flex-direction", LonghandId::FlexDirection),
    ("flex-grow", LonghandId::FlexGrow),
    ("flex-shrink", LonghandId::FlexShrink),
    ("flex-basis", LonghandId::FlexBasis),
    ("direction", LonghandId::Direction),
    ("row-gap", LonghandId::RowGap),
    ("column-gap", LonghandId::ColumnGap),
];

const FLEX_LAYOUT_AUTHOR_PROPERTIES: &[&str] = &[
    "display",
    "box-sizing",
    "width",
    "height",
    "flex-direction",
    "flex-grow",
    "flex-shrink",
    "flex-basis",
    "direction",
    "row-gap",
    "column-gap",
];

const FLEX_ALIGNMENT_PROPERTIES: &[(&str, LonghandId)] = &[
    ("display", LonghandId::Display),
    ("box-sizing", LonghandId::BoxSizing),
    ("width", LonghandId::Width),
    ("height", LonghandId::Height),
    ("flex-direction", LonghandId::FlexDirection),
    ("flex-grow", LonghandId::FlexGrow),
    ("flex-shrink", LonghandId::FlexShrink),
    ("flex-basis", LonghandId::FlexBasis),
    ("direction", LonghandId::Direction),
    ("row-gap", LonghandId::RowGap),
    ("column-gap", LonghandId::ColumnGap),
    ("align-items", LonghandId::AlignItems),
    ("justify-content", LonghandId::JustifyContent),
];

const FLEX_ALIGNMENT_AUTHOR_PROPERTIES: &[&str] = &[
    "display",
    "box-sizing",
    "width",
    "height",
    "flex-direction",
    "flex-grow",
    "flex-shrink",
    "flex-basis",
    "direction",
    "row-gap",
    "column-gap",
    "align-items",
    "justify-content",
];

#[derive(Debug)]
pub enum CssCascadeError {
    InvalidViewport,
    InvalidMediaEnvironment,
    /// computed font-size를 유한한 음이 아닌 CSS px로 바꿀 수 없습니다.
    InvalidComputedFontSize {
        node: Option<NodeId>,
    },
    /// 값이 실제 플랫폼 font metrics에 의존하지만 현재 metric provider가 고정 placeholder입니다.
    UnsupportedFontMetricUnit {
        source_id: String,
        node: Option<NodeId>,
        unit: String,
    },
    /// C21 container query ownership가 아직 없어서 container-relative length를 계산할 수 없습니다.
    UnsupportedContainerRelativeUnit {
        source_id: String,
        node: Option<NodeId>,
        unit: String,
    },
    /// 합성 cascade 전용 html/body 상자의 미지원 레이아웃 변경을 조용히 버리지 않습니다.
    UnsupportedSyntheticDocumentStyle {
        element: &'static str,
        property: &'static str,
        value: String,
    },
    /// font metric 검사기가 CSS token stream을 안전하게 끝까지 읽지 못했습니다.
    InvalidFontMetricInput {
        source_id: String,
        node: Option<NodeId>,
    },
    /// container-relative unit 검사기가 CSS token stream을 안전하게 끝까지 읽지 못했습니다.
    InvalidContainerRelativeUnitInput {
        source_id: String,
        node: Option<NodeId>,
    },
    InvalidStylesheetOrigin {
        id: String,
    },
    UnsupportedAuthorCss {
        stylesheet_id: String,
        feature: String,
    },
    IncrementalReuseUnavailable,
    AuthorStylesheetDiagnostic {
        stylesheet_id: String,
        line: u32,
        column: u32,
        message: String,
    },
    UnsupportedComputedBackgroundColor {
        node: NodeId,
        reason: String,
    },
    /// 현재 layout bridge가 의미를 보존할 수 없는 computed `aspect-ratio`입니다.
    UnsupportedComputedAspectRatio {
        node: NodeId,
        reason: String,
    },
    StylesheetRegistry(StylesheetRegistryError),
}

impl fmt::Display for CssCascadeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidViewport => {
                formatter.write_str("CSS viewport 크기와 배율은 유한한 양수여야 합니다")
            }
            Self::InvalidMediaEnvironment => {
                formatter.write_str("CSS media 환경의 primary·전체 포인터 기능이 모순됩니다")
            }
            Self::InvalidComputedFontSize { node } => {
                write!(
                    formatter,
                    "computed font-size가 유한한 CSS px가 아닙니다: {node:?}"
                )
            }
            Self::UnsupportedFontMetricUnit {
                source_id,
                node,
                unit,
            } => write!(
                formatter,
                "{source_id}의 font metric 단위 {unit}은 현재 지원하지 않습니다: {node:?}"
            ),
            Self::UnsupportedContainerRelativeUnit {
                source_id,
                node,
                unit,
            } => write!(
                formatter,
                "{source_id}의 container 상대 단위 {unit}은 C21 container query 지원 전까지 사용할 수 없습니다: {node:?}"
            ),
            Self::UnsupportedSyntheticDocumentStyle {
                element,
                property,
                value,
            } => write!(
                formatter,
                "합성 {element}은 cascade 전용이라 {property}: {value} 레이아웃을 지원하지 않습니다"
            ),
            Self::InvalidFontMetricInput { source_id, node } => write!(
                formatter,
                "{source_id}의 CSS 단위 입력을 안전하게 검사할 수 없습니다: {node:?}"
            ),
            Self::InvalidContainerRelativeUnitInput { source_id, node } => write!(
                formatter,
                "{source_id}의 container 단위 입력을 안전하게 검사할 수 없습니다: {node:?}"
            ),
            Self::InvalidStylesheetOrigin { id } => {
                write!(
                    formatter,
                    "author stylesheet의 origin이 잘못되었습니다: {id}"
                )
            }
            Self::UnsupportedAuthorCss {
                stylesheet_id,
                feature,
            } => write!(
                formatter,
                "stylesheet {stylesheet_id}가 지원 CSS 입력 profile 밖 기능을 사용합니다: {feature}"
            ),
            Self::IncrementalReuseUnavailable => {
                formatter.write_str("증분 cascade 재사용 조건이 맞지 않습니다")
            }
            Self::AuthorStylesheetDiagnostic {
                stylesheet_id,
                line,
                column,
                message,
            } => write!(
                formatter,
                "stylesheet {stylesheet_id}:{line}:{column} 파싱 진단: {message}"
            ),
            Self::UnsupportedComputedBackgroundColor { node, reason } => write!(
                formatter,
                "노드 {node}의 계산 background-color를 S04 불투명 sRGB로 변환할 수 없습니다: {reason}"
            ),
            Self::UnsupportedComputedAspectRatio { node, reason } => write!(
                formatter,
                "노드 {node}의 computed aspect-ratio를 현재 layout 계약으로 표현할 수 없습니다: {reason}"
            ),
            Self::StylesheetRegistry(error) => error.fmt(formatter),
        }
    }
}

impl Error for CssCascadeError {}

impl From<StylesheetRegistryError> for CssCascadeError {
    fn from(error: StylesheetRegistryError) -> Self {
        Self::StylesheetRegistry(error)
    }
}

/// 고정 viewport에서 한 문서 snapshot 전체의 제한된 computed style을 계산합니다.
// C04.1 전용 계산 entrypoint는 fixture 검증에서 호출합니다.
#[allow(dead_code)]
pub(crate) fn compute_basic_cascade(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
) -> Result<ComputedStyleSnapshot, CssCascadeError> {
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        StyleRevision::default(),
        BASIC_CASCADE_PROPERTIES,
        ComputedStyleProfile::BasicCascadeV1,
    )
}

/// C04.2 Flex adapter가 사용하는 제한 computed-style snapshot을 계산합니다.
pub fn compute_flex_layout_cascade(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<ComputedStyleSnapshot, CssCascadeError> {
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        FLEX_LAYOUT_PROPERTIES,
        ComputedStyleProfile::FlexLayoutV1,
    )
}

/// C04.3 Flex alignment adapter가 사용하는 제한 computed-style snapshot을 계산합니다.
pub fn compute_flex_alignment_cascade(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<ComputedStyleSnapshot, CssCascadeError> {
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        FLEX_ALIGNMENT_PROPERTIES,
        ComputedStyleProfile::FlexAlignmentV1,
    )
}

/// C04.4의 Cascade Layers를 허용하는 제한 Flex alignment snapshot을 계산합니다.
pub fn compute_flex_alignment_layers_cascade(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<ComputedStyleSnapshot, CssCascadeError> {
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        FLEX_ALIGNMENT_PROPERTIES,
        ComputedStyleProfile::FlexAlignmentCascadeLayersV1,
    )
}

/// S04 고정 fixture용 Flex layout 및 불투명 배경색 계산 style을 계산합니다.
pub fn compute_s04_flex_paint_cascade(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<ComputedStyleSnapshot, CssCascadeError> {
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        s04::S04_FLEX_PAINT_PROPERTIES,
        ComputedStyleProfile::S04FlexPaintV1,
    )
}

fn compute_cascade(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
    properties: &[(&str, LonghandId)],
    profile: ComputedStyleProfile,
) -> Result<ComputedStyleSnapshot, CssCascadeError> {
    incremental::compute_cascade_with_reuse(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        properties,
        profile,
        None,
    )
    .and_then(|result| match result {
        Some((snapshot, _)) => Ok(snapshot),
        None => Err(CssCascadeError::IncrementalReuseUnavailable),
    })
}

#[cfg(test)]
#[path = "cascade/tests.rs"]
mod tests;

#[cfg(test)]
#[path = "cascade/media_environment_tests.rs"]
mod media_environment_tests;

#[cfg(test)]
#[path = "cascade/runtime_layout_tests.rs"]
mod runtime_layout_tests;

#[cfg(test)]
#[path = "cascade/c10_flex_distribution_tests.rs"]
mod c10_flex_distribution_tests;

#[cfg(test)]
#[path = "cascade/custom_properties_tests.rs"]
mod custom_properties_tests;

#[cfg(test)]
#[path = "cascade/incremental_tests.rs"]
mod incremental_tests;

#[cfg(test)]
#[path = "cascade/registered_properties_tests.rs"]
mod registered_properties_tests;
