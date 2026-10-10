use crate::css_math::{LayoutCalcId, LayoutCssMathProperty};

/// 루트 기준으로 계산할 고정 화면 크기입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewport {
    pub width: f32,
    pub height: f32,
}

/// 축에 지정할 크기입니다. 길이는 CSS px이며 백분율은 부모 content size의 비율입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayoutDimension {
    Auto,
    Fixed(f32),
    Percent(f32),
    Calc(LayoutCalcId),
}

/// 상자 간격과 가장자리에 쓰는 CSS 길이 또는 비율입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayoutLengthPercentage {
    /// 해당 속성의 자동 사용값 계산을 Taffy에 맡깁니다. margin과 위치 inset에서 사용합니다.
    Auto,
    LengthPx(f32),
    Percentage(f32),
    Calc(LayoutCalcId),
}

impl LayoutLengthPercentage {
    pub const ZERO: Self = Self::LengthPx(0.0);

    pub const fn length(value: f32) -> Self {
        Self::LengthPx(value)
    }

    pub const fn percent(fraction: f32) -> Self {
        Self::Percentage(fraction)
    }

    pub(crate) fn value(self) -> f32 {
        match self {
            Self::LengthPx(value) | Self::Percentage(value) => value,
            Self::Auto | Self::Calc(_) => f32::NAN,
        }
    }

    pub(crate) fn is_calc(self) -> bool {
        matches!(self, Self::Calc(_))
    }
}

/// Flex 자식 배치의 주축입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

/// Flex 컨테이너의 항목을 주축의 다음 줄로 보낼지 정합니다.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FlexWrap {
    #[default]
    NoWrap,
    Wrap,
    WrapReverse,
}

/// Taffy가 계산하는 제한 CSS display 값입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LayoutDisplay {
    Flex,
    Block,
    /// Block layout을 사용하면서 자식에 독립적인 Block formatting context를 만듭니다.
    FlowRoot,
    None,
}

/// CSS 상자의 너비·높이를 해석하는 기준입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LayoutBoxSizing {
    BorderBox,
    ContentBox,
}

/// 가로 방향의 순서와 시작점을 정합니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextDirection {
    Ltr,
    Rtl,
}

/// CSS overflow alignment modifier입니다. 생략된 modifier와 명시적 `unsafe`를 구분합니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AlignmentSafety {
    Safe,
    Unsafe,
}

/// `align-items`·`align-self`에서 사용하는 비-baseline positional 값입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ItemAlignmentPosition {
    Start,
    End,
    FlexStart,
    FlexEnd,
    SelfStart,
    SelfEnd,
    Center,
}

/// `align-content`에서 사용하는 비-baseline positional 값입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContentAlignmentPosition {
    Start,
    End,
    FlexStart,
    FlexEnd,
    Center,
}

/// `justify-content`에서 사용하는 positional 값입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JustifyContentPosition {
    Start,
    End,
    FlexStart,
    FlexEnd,
    Center,
    Left,
    Right,
}

/// Flex item의 교차축 정렬입니다. `Normal`은 computed keyword를 유지합니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LayoutAlignItems {
    Normal,
    Stretch,
    FlexStart,
    FlexEnd,
    Center,
    FirstBaseline,
    LastBaseline,
    Position {
        position: ItemAlignmentPosition,
        safety: Option<AlignmentSafety>,
    },
}

impl LayoutAlignItems {
    pub(crate) const fn uses_stretch_behavior(self) -> bool {
        matches!(self, Self::Normal | Self::Stretch)
    }
}

/// 개별 Flex item의 교차축 정렬입니다. `Auto`는 부모 `align-items`를 사용합니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LayoutAlignSelf {
    Auto,
    Normal,
    Stretch,
    FlexStart,
    FlexEnd,
    Center,
    FirstBaseline,
    LastBaseline,
    Position {
        position: ItemAlignmentPosition,
        safety: Option<AlignmentSafety>,
    },
}

/// Flex line 묶음의 교차축 정렬입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LayoutAlignContent {
    Normal,
    Stretch,
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    FirstBaseline,
    Position {
        position: ContentAlignmentPosition,
        safety: Option<AlignmentSafety>,
    },
}

/// Flex item 묶음의 주축 정렬입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LayoutJustifyContent {
    Normal,
    Stretch,
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    Position {
        position: JustifyContentPosition,
        safety: Option<AlignmentSafety>,
    },
}

/// 위·오른쪽·아래·왼쪽 상자 가장자리 값입니다.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LayoutEdges {
    pub top: LayoutLengthPercentage,
    pub right: LayoutLengthPercentage,
    pub bottom: LayoutLengthPercentage,
    pub left: LayoutLengthPercentage,
}

impl LayoutEdges {
    pub const fn auto() -> Self {
        Self {
            top: LayoutLengthPercentage::Auto,
            right: LayoutLengthPercentage::Auto,
            bottom: LayoutLengthPercentage::Auto,
            left: LayoutLengthPercentage::Auto,
        }
    }
}

/// 현재 제한 CSS profile에서 사용할 수 있는 계산 위치 값입니다.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LayoutPosition {
    #[default]
    Static,
    Relative,
    Absolute,
}

/// `LayoutStyle`의 기존 크기·Flex 입력과 분리한 CSS positioning 입력입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutPositioning {
    pub position: LayoutPosition,
    pub inset: LayoutEdges,
}

impl LayoutPositioning {
    pub(crate) fn calc_values(self) -> impl Iterator<Item = (LayoutCssMathProperty, LayoutCalcId)> {
        let values = if self.position == LayoutPosition::Static {
            Vec::new()
        } else {
            [
                (
                    LayoutCssMathProperty::Top,
                    calc_length_percentage(self.inset.top),
                ),
                (
                    LayoutCssMathProperty::Right,
                    calc_length_percentage(self.inset.right),
                ),
                (
                    LayoutCssMathProperty::Bottom,
                    calc_length_percentage(self.inset.bottom),
                ),
                (
                    LayoutCssMathProperty::Left,
                    calc_length_percentage(self.inset.left),
                ),
            ]
            .into_iter()
            .filter_map(|(property, id)| id.map(|id| (property, id)))
            .collect()
        };
        values.into_iter()
    }
}

impl Default for LayoutPositioning {
    fn default() -> Self {
        Self {
            position: LayoutPosition::Static,
            inset: LayoutEdges {
                top: LayoutLengthPercentage::Auto,
                right: LayoutLengthPercentage::Auto,
                bottom: LayoutLengthPercentage::Auto,
                left: LayoutLengthPercentage::Auto,
            },
        }
    }
}

/// Taffy에 전달하는 면별 CSS border used width입니다. 모든 값은 CSS px입니다.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LayoutBorder {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

/// Flex 행·열 사이의 간격입니다.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LayoutGap {
    pub row: LayoutLengthPercentage,
    pub column: LayoutLengthPercentage,
}

/// 이 초기 내부 계약이 표현하는 제한된 Flex 스타일입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutStyle {
    pub display: LayoutDisplay,
    pub box_sizing: LayoutBoxSizing,
    pub width: LayoutDimension,
    pub height: LayoutDimension,
    /// `auto`는 최소 자동 크기 계산을 사용합니다.
    pub min_width: LayoutDimension,
    /// `auto`는 CSS `none`처럼 제한이 없음을 뜻합니다.
    pub max_width: LayoutDimension,
    /// `auto`는 최소 자동 크기 계산을 사용합니다.
    pub min_height: LayoutDimension,
    /// `auto`는 CSS `none`처럼 제한이 없음을 뜻합니다.
    pub max_height: LayoutDimension,
    /// 선호 비율 `width / height`; 값은 유한한 양수여야 합니다.
    pub aspect_ratio: Option<f32>,
    /// Flex item의 order-modified document order 값입니다. CSS 기본값은 0입니다.
    pub order: i32,
    pub flex_basis: LayoutDimension,
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub direction: TextDirection,
    pub align_items: LayoutAlignItems,
    pub align_self: LayoutAlignSelf,
    /// `None`은 프로파일이 `align-content`를 지원하지 않음을 뜻합니다.
    pub align_content: Option<LayoutAlignContent>,
    pub justify_content: LayoutJustifyContent,
    /// 음수 값도 허용하는 외부 여백입니다.
    pub margin: LayoutEdges,
    /// 음수 값을 허용하지 않는 내부 여백입니다.
    pub padding: LayoutEdges,
    /// 면별 CSS border used width입니다. paint 속성은 포함하지 않습니다.
    pub border: LayoutBorder,
    pub gap: LayoutGap,
    pub flex_grow: f32,
    pub flex_shrink: f32,
}

impl LayoutStyle {
    pub(crate) fn calc_values(self) -> impl Iterator<Item = (LayoutCssMathProperty, LayoutCalcId)> {
        [
            (LayoutCssMathProperty::Width, calc_dimension(self.width)),
            (LayoutCssMathProperty::Height, calc_dimension(self.height)),
            (
                LayoutCssMathProperty::MinWidth,
                calc_dimension(self.min_width),
            ),
            (
                LayoutCssMathProperty::MaxWidth,
                calc_dimension(self.max_width),
            ),
            (
                LayoutCssMathProperty::MinHeight,
                calc_dimension(self.min_height),
            ),
            (
                LayoutCssMathProperty::MaxHeight,
                calc_dimension(self.max_height),
            ),
            (
                LayoutCssMathProperty::FlexBasis,
                calc_dimension(self.flex_basis),
            ),
            (
                LayoutCssMathProperty::MarginTop,
                calc_length_percentage(self.margin.top),
            ),
            (
                LayoutCssMathProperty::MarginRight,
                calc_length_percentage(self.margin.right),
            ),
            (
                LayoutCssMathProperty::MarginBottom,
                calc_length_percentage(self.margin.bottom),
            ),
            (
                LayoutCssMathProperty::MarginLeft,
                calc_length_percentage(self.margin.left),
            ),
            (
                LayoutCssMathProperty::PaddingTop,
                calc_length_percentage(self.padding.top),
            ),
            (
                LayoutCssMathProperty::PaddingRight,
                calc_length_percentage(self.padding.right),
            ),
            (
                LayoutCssMathProperty::PaddingBottom,
                calc_length_percentage(self.padding.bottom),
            ),
            (
                LayoutCssMathProperty::PaddingLeft,
                calc_length_percentage(self.padding.left),
            ),
            (
                LayoutCssMathProperty::RowGap,
                calc_length_percentage(self.gap.row),
            ),
            (
                LayoutCssMathProperty::ColumnGap,
                calc_length_percentage(self.gap.column),
            ),
        ]
        .into_iter()
        .filter_map(|(property, id)| id.map(|id| (property, id)))
    }
}

fn calc_dimension(value: LayoutDimension) -> Option<LayoutCalcId> {
    match value {
        LayoutDimension::Calc(id) => Some(id),
        LayoutDimension::Auto | LayoutDimension::Fixed(_) | LayoutDimension::Percent(_) => None,
    }
}

fn calc_length_percentage(value: LayoutLengthPercentage) -> Option<LayoutCalcId> {
    match value {
        LayoutLengthPercentage::Calc(id) => Some(id),
        LayoutLengthPercentage::Auto
        | LayoutLengthPercentage::LengthPx(_)
        | LayoutLengthPercentage::Percentage(_) => None,
    }
}

impl Default for LayoutLengthPercentage {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Default for LayoutStyle {
    fn default() -> Self {
        Self {
            display: LayoutDisplay::Flex,
            box_sizing: LayoutBoxSizing::BorderBox,
            width: LayoutDimension::Auto,
            height: LayoutDimension::Auto,
            min_width: LayoutDimension::Auto,
            max_width: LayoutDimension::Auto,
            min_height: LayoutDimension::Auto,
            max_height: LayoutDimension::Auto,
            aspect_ratio: None,
            order: 0,
            flex_basis: LayoutDimension::Auto,
            flex_direction: FlexDirection::Column,
            flex_wrap: FlexWrap::NoWrap,
            direction: TextDirection::Ltr,
            align_items: LayoutAlignItems::Normal,
            align_self: LayoutAlignSelf::Auto,
            align_content: None,
            justify_content: LayoutJustifyContent::Normal,
            margin: LayoutEdges::default(),
            padding: LayoutEdges::default(),
            border: LayoutBorder::default(),
            gap: LayoutGap::default(),
            flex_grow: 0.0,
            flex_shrink: 0.0,
        }
    }
}
