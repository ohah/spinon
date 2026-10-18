use spinon_core::NodeId;
use spinon_text_layout::FontRegistryRevision;

use crate::{LayoutInputRevision, LayoutIntrinsicSize, LayoutStyle};

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
    Replaced {
        natural: LayoutIntrinsicSize,
    },
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
