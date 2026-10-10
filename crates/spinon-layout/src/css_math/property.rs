/// 한 번의 레이아웃 입력 안에서 typed CSS 계산식을 가리킵니다.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct LayoutCalcId(pub u32);

/// 계산식이 적용되는 속성입니다. 부호 범위와 percentage 기준을 식별하는 데 씁니다.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum LayoutCssMathProperty {
    Width,
    Height,
    MinWidth,
    MaxWidth,
    MinHeight,
    MaxHeight,
    FlexBasis,
    Top,
    Right,
    Bottom,
    Left,
    MarginTop,
    MarginRight,
    MarginBottom,
    MarginLeft,
    PaddingTop,
    PaddingRight,
    PaddingBottom,
    PaddingLeft,
    RowGap,
    ColumnGap,
}

impl LayoutCssMathProperty {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Width => "width",
            Self::Height => "height",
            Self::MinWidth => "min-width",
            Self::MaxWidth => "max-width",
            Self::MinHeight => "min-height",
            Self::MaxHeight => "max-height",
            Self::FlexBasis => "flex-basis",
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::MarginTop => "margin-top",
            Self::MarginRight => "margin-right",
            Self::MarginBottom => "margin-bottom",
            Self::MarginLeft => "margin-left",
            Self::PaddingTop => "padding-top",
            Self::PaddingRight => "padding-right",
            Self::PaddingBottom => "padding-bottom",
            Self::PaddingLeft => "padding-left",
            Self::RowGap => "row-gap",
            Self::ColumnGap => "column-gap",
        }
    }

    pub const fn is_nonnegative(self) -> bool {
        !matches!(
            self,
            Self::Top
                | Self::Right
                | Self::Bottom
                | Self::Left
                | Self::MarginTop
                | Self::MarginRight
                | Self::MarginBottom
                | Self::MarginLeft
        )
    }
}
