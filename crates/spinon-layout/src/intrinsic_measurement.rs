use spinon_core::{NodeId, TextOrientation, WritingMode};
use spinon_text_layout::{FontRegistryRevision, LogicalIntrinsicSize, TextIntrinsicInlineSizes};

use crate::{
    LayoutDimension, LayoutInputRevision, LayoutIntrinsicSize, LayoutLengthPercentage, LayoutStyle,
};

/// Intrinsic measure callback이 각 축에 전달한 available space입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayoutIntrinsicMeasureSpace {
    MinContent,
    MaxContent,
    Definite(f32),
}

/// Inline·block 축의 intrinsic measure available space입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutIntrinsicMeasureAvailableSpace {
    pub width: LayoutIntrinsicMeasureSpace,
    pub height: LayoutIntrinsicMeasureSpace,
}

/// Taffy measure callback에 이미 고정되어 있던 축 크기입니다.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LayoutIntrinsicMeasureKnownDimensions {
    pub width: Option<f32>,
    pub height: Option<f32>,
}

/// Intrinsic measure 당시의 parent content-box 크기입니다.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LayoutIntrinsicMeasureParentSize {
    pub width: Option<f32>,
    pub height: Option<f32>,
}

/// Measure callback이 반환한 content-box 크기입니다. Padding과 border는 포함하지 않습니다.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LayoutIntrinsicMeasureSize {
    pub width: f32,
    pub height: f32,
}

/// Intrinsic measurement의 원본 입력입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayoutIntrinsicMeasurementSource {
    Text {
        font_registry_revision: FontRegistryRevision,
    },
    OrthogonalText {
        min_content: LogicalIntrinsicSize,
        max_content: LogicalIntrinsicSize,
        child_writing_mode: WritingMode,
        parent_writing_mode: WritingMode,
        text_orientation: TextOrientation,
        font_registry_revision: FontRegistryRevision,
    },
    Replaced {
        natural: LayoutIntrinsicSize,
    },
}

/// Grid item leaf에서 계산한 원본 intrinsic content-size 입력입니다.
#[derive(Clone, Debug, PartialEq)]
pub enum LayoutGridItemIntrinsicSource {
    Text {
        sizes: TextIntrinsicInlineSizes,
    },
    OrthogonalText {
        min_content: LogicalIntrinsicSize,
        max_content: LogicalIntrinsicSize,
        child_writing_mode: WritingMode,
        parent_writing_mode: WritingMode,
        text_orientation: TextOrientation,
        font_registry_revision: FontRegistryRevision,
    },
    Replaced {
        natural: LayoutIntrinsicSize,
    },
}

/// Anonymous Grid text item에 들어간 원본 HostDocument text node와 정확한 문자열입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutGridItemTextSource {
    pub node: NodeId,
    pub text: String,
}

/// 직접 Grid leaf의 intrinsic sizing 후속 계산에 사용할 원본 입력입니다.
///
/// `Text` source는 `TextIntrinsicInlineSizes`를 보존하므로 font registry identity/revision도 유지합니다.
/// `OrthogonalText` source는 logical min/max 값과 숫자 revision·writing metadata를 보존하지만
/// font registry token identity는 이 전달 값에 포함하지 않습니다. 이 값은 한 layout tree의
/// text engine에서 계산되고 원본 node/text와 layout revision이 별도로 전달됩니다.
/// Replaced source의 natural size는 content-box CSS px이며, 지정 크기·aspect ratio·외곽 edge
/// 적용은 `item_style`과 함께 다음 contribution 단계에서 결정합니다.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutGridItemIntrinsicInput {
    pub grid_container: NodeId,
    pub node: NodeId,
    /// 원본 child list에서의 순서입니다. 이후 `order` 정렬의 안정 순서로 사용합니다.
    pub source_order: usize,
    pub revision: LayoutInputRevision,
    pub source: LayoutGridItemIntrinsicSource,
    /// Anonymous Grid item의 정확한 원본 text node 목록입니다. 보통 Grid item에는 비어 있습니다.
    pub text_sources: Vec<LayoutGridItemTextSource>,
    pub item_style: LayoutStyle,
}

/// Grid item 한 축의 CSS contribution 계산에 필요한 후보와 box 입력입니다.
///
/// `min_content`·`max_content`는 padding/border 바깥의 content-box CSS px입니다.
/// 지정·최소·최대 크기와 margin/padding은 계산 전 typed 값으로 보존하고, border는 used px입니다.
/// 이 입력은 최종 CSS contribution이나 track span·automatic minimum을 뜻하지 않습니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutGridItemIntrinsicAxisContributionInput {
    pub min_content: Option<f32>,
    pub max_content: Option<f32>,
    pub preferred_size: LayoutDimension,
    pub min_size: LayoutDimension,
    pub max_size: LayoutDimension,
    pub margin_start: LayoutLengthPercentage,
    pub margin_end: LayoutLengthPercentage,
    pub padding_start: LayoutLengthPercentage,
    pub padding_end: LayoutLengthPercentage,
    pub border_start: f32,
    pub border_end: f32,
}

impl LayoutGridItemIntrinsicAxisContributionInput {
    /// 고정 edge profile에서 content-box 후보를 margin-box 크기로 바꿉니다.
    ///
    /// 백분율·calc edge는 basis가 연결되는 다음 단계 전까지 명시적으로 거부합니다.
    pub fn outer_size(
        self,
        content_size: Option<f32>,
        node: NodeId,
        axis: &'static str,
    ) -> Result<Option<f32>, crate::LayoutError> {
        let Some(content_size) = content_size else {
            return Ok(None);
        };
        if !content_size.is_finite() || content_size < 0.0 {
            return Err(crate::LayoutError::InvalidIntrinsicSize { node, axis });
        }

        let margin_start = fixed_margin(self.margin_start, node)?;
        let margin_end = fixed_margin(self.margin_end, node)?;
        let padding_start = fixed_padding(self.padding_start, node)?;
        let padding_end = fixed_padding(self.padding_end, node)?;
        if !self.border_start.is_finite()
            || self.border_start < 0.0
            || !self.border_end.is_finite()
            || self.border_end < 0.0
        {
            return Err(crate::LayoutError::InvalidIntrinsicSize { node, axis });
        }

        let outer_size = content_size
            + margin_start
            + margin_end
            + padding_start
            + padding_end
            + self.border_start
            + self.border_end;
        if !outer_size.is_finite() {
            return Err(crate::LayoutError::InvalidIntrinsicSize { node, axis });
        }
        Ok(Some(outer_size))
    }
}

fn fixed_margin(value: LayoutLengthPercentage, node: NodeId) -> Result<f32, crate::LayoutError> {
    match value {
        LayoutLengthPercentage::Auto => Ok(0.0),
        LayoutLengthPercentage::LengthPx(value) if value.is_finite() => Ok(value),
        _ => Err(crate::LayoutError::UnsupportedIntrinsicSizing {
            node,
            reason: "Grid item 외곽 크기 후보에는 고정 여백 값이 필요합니다",
        }),
    }
}

fn fixed_padding(value: LayoutLengthPercentage, node: NodeId) -> Result<f32, crate::LayoutError> {
    match value {
        LayoutLengthPercentage::LengthPx(value) if value.is_finite() && value >= 0.0 => Ok(value),
        _ => Err(crate::LayoutError::UnsupportedIntrinsicSizing {
            node,
            reason: "Grid item 외곽 크기 후보에는 0 이상의 고정 padding 값이 필요합니다",
        }),
    }
}

/// CSS Grid contribution 계산용 direct-leaf 입력을 같은 layout source에 결속합니다.
///
/// `width`와 `height`는 현재 제한 profile의 물리 축입니다. `source`는 text font registry
/// revision 또는 replaced natural metadata를 보존합니다. 전체 `item_style`도 함께 보존해
/// aspect ratio, overflow, order처럼 축 후보와 함께 해석할 값을 잃지 않습니다.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutGridItemIntrinsicContributionInput {
    pub grid_container: NodeId,
    pub node: NodeId,
    pub source_order: usize,
    pub revision: LayoutInputRevision,
    pub source: LayoutGridItemIntrinsicSource,
    pub text_sources: Vec<LayoutGridItemTextSource>,
    pub item_style: LayoutStyle,
    pub width: LayoutGridItemIntrinsicAxisContributionInput,
    pub height: LayoutGridItemIntrinsicAxisContributionInput,
}

impl LayoutGridItemIntrinsicInput {
    /// 원본 source/style에서 CSS Grid 후보 입력을 필요할 때 생성합니다.
    ///
    /// output에 별도 벡터를 만들지 않으므로 layout 계산의 할당·실패 경로는 바뀌지 않습니다.
    pub fn contribution_input(
        &self,
    ) -> Result<LayoutGridItemIntrinsicContributionInput, crate::LayoutError> {
        let (min_width, max_width, min_height, max_height) = match &self.source {
            LayoutGridItemIntrinsicSource::Text { sizes } => (
                Some(sizes.min_content()),
                Some(sizes.max_content()),
                Some(sizes.min_content_height()),
                Some(sizes.max_content_height()),
            ),
            LayoutGridItemIntrinsicSource::OrthogonalText {
                min_content,
                max_content,
                child_writing_mode,
                parent_writing_mode,
                text_orientation,
                ..
            } => {
                if *parent_writing_mode != WritingMode::HorizontalTb
                    || *child_writing_mode != WritingMode::VerticalRl
                    || *text_orientation != TextOrientation::Mixed
                {
                    return Err(crate::LayoutError::UnsupportedIntrinsicSizing {
                        node: self.node,
                        reason: "orthogonal intrinsic contribution requires horizontal-tb parent and vertical-rl/mixed text",
                    });
                }
                if !valid_logical_intrinsic_size(*min_content)
                    || !valid_logical_intrinsic_size(*max_content)
                {
                    return Err(crate::LayoutError::InvalidIntrinsicSize {
                        node: self.node,
                        axis: "orthogonal logical",
                    });
                }
                (
                    Some(min_content.block_px),
                    Some(max_content.block_px),
                    Some(min_content.inline_px),
                    Some(max_content.inline_px),
                )
            }
            LayoutGridItemIntrinsicSource::Replaced { natural } => {
                (natural.width, natural.width, natural.height, natural.height)
            }
        };
        let style = self.item_style;

        Ok(LayoutGridItemIntrinsicContributionInput {
            grid_container: self.grid_container,
            node: self.node,
            source_order: self.source_order,
            revision: self.revision,
            source: self.source.clone(),
            text_sources: self.text_sources.clone(),
            item_style: style,
            width: LayoutGridItemIntrinsicAxisContributionInput {
                min_content: min_width,
                max_content: max_width,
                preferred_size: style.width,
                min_size: style.min_width,
                max_size: style.max_width,
                margin_start: style.margin.left,
                margin_end: style.margin.right,
                padding_start: style.padding.left,
                padding_end: style.padding.right,
                border_start: style.border.left,
                border_end: style.border.right,
            },
            height: LayoutGridItemIntrinsicAxisContributionInput {
                min_content: min_height,
                max_content: max_height,
                preferred_size: style.height,
                min_size: style.min_height,
                max_size: style.max_height,
                margin_start: style.margin.top,
                margin_end: style.margin.bottom,
                padding_start: style.padding.top,
                padding_end: style.padding.bottom,
                border_start: style.border.top,
                border_end: style.border.bottom,
            },
        })
    }
}

fn valid_logical_intrinsic_size(size: LogicalIntrinsicSize) -> bool {
    size.inline_px.is_finite()
        && size.inline_px >= 0.0
        && size.block_px.is_finite()
        && size.block_px >= 0.0
}

/// Grid item의 measure callback 한 번에 대한 source-bound 관찰값입니다.
///
/// `revision`은 paragraph/style/document/environment와 replaced-resource 입력을 함께 묶습니다.
/// Text source는 추가로 font registry revision을 보존합니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutIntrinsicMeasurement {
    pub node: NodeId,
    pub revision: LayoutInputRevision,
    pub source: LayoutIntrinsicMeasurementSource,
    pub item_style: LayoutStyle,
    pub available_space: LayoutIntrinsicMeasureAvailableSpace,
    pub known_dimensions: LayoutIntrinsicMeasureKnownDimensions,
    pub parent_size: LayoutIntrinsicMeasureParentSize,
    pub content_size: LayoutIntrinsicMeasureSize,
}
