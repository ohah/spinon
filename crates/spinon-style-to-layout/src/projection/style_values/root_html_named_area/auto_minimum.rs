use spinon_style::{ComputedCssDimension, ComputedCssMaxSize, ComputedElementStyle};

use super::super::grid_values::RootHtmlNamedAreaAutoMinimumAxis;
use crate::StyleLayoutError;

pub(super) fn validate_box(
    container: &ComputedElementStyle,
    axis: RootHtmlNamedAreaAutoMinimumAxis,
    unsupported: &impl Fn(spinon_core::NodeId, &'static str, &str) -> StyleLayoutError,
) -> Result<(), StyleLayoutError> {
    let node = container.node_id;
    if container.properties.get("box-sizing").map(String::as_str) != Some("content-box") {
        return Err(unsupported(
            node,
            "box-sizing",
            "Root HTML named-area 자동 최소 크기 profile은 content-box를 요구합니다",
        ));
    }
    if [
        container.layout_border.top,
        container.layout_border.right,
        container.layout_border.bottom,
        container.layout_border.left,
    ] != [2.0; 4]
    {
        return Err(unsupported(
            node,
            "border-width",
            "Root HTML named-area 자동 최소 크기 profile은 네 변에 2px border를 요구합니다",
        ));
    }
    if container.layout_aspect_ratio.is_some()
        || container
            .properties
            .get("aspect-ratio")
            .is_some_and(|value| value != "auto")
    {
        return Err(unsupported(
            node,
            "aspect-ratio",
            "Root HTML named-area 자동 최소 크기 profile은 aspect-ratio를 허용하지 않습니다",
        ));
    }
    let (size_property, size, min_property, min_size, max_property, max_size) = match axis {
        RootHtmlNamedAreaAutoMinimumAxis::Columns => (
            "width",
            container.layout_dimensions.width,
            "min-width",
            container.layout_dimensions.min_width,
            "max-width",
            container.layout_dimensions.max_width,
        ),
        RootHtmlNamedAreaAutoMinimumAxis::Rows => (
            "height",
            container.layout_dimensions.height,
            "min-height",
            container.layout_dimensions.min_height,
            "max-height",
            container.layout_dimensions.max_height,
        ),
    };
    if !matches!(size, ComputedCssDimension::LengthPx(value) if value.is_finite() && value > 0.0) {
        return Err(unsupported(
            node,
            size_property,
            "Root HTML named-area 자동 최소 크기는 해당 축의 양수 고정 CSS px 크기를 요구합니다",
        ));
    }
    if min_size != ComputedCssDimension::Auto {
        return Err(unsupported(
            node,
            min_property,
            "Root HTML named-area 자동 최소 크기는 해당 축의 min-size:auto를 요구합니다",
        ));
    }
    if max_size != ComputedCssMaxSize::None {
        return Err(unsupported(
            node,
            max_property,
            "Root HTML named-area 자동 최소 크기는 해당 축의 max-size:none을 요구합니다",
        ));
    }
    Ok(())
}

pub(super) fn validate_items(
    owner: spinon_core::NodeId,
    axis: RootHtmlNamedAreaAutoMinimumAxis,
    area_items: &[Option<&ComputedElementStyle>; 3],
    unsupported: &impl Fn(spinon_core::NodeId, &'static str, &str) -> StyleLayoutError,
) -> Result<(), StyleLayoutError> {
    let expected_dimensions = match axis {
        RootHtmlNamedAreaAutoMinimumAxis::Columns => [
            (
                ComputedCssDimension::Auto,
                ComputedCssDimension::LengthPx(20.0),
            ),
            (
                ComputedCssDimension::LengthPx(80.0),
                ComputedCssDimension::LengthPx(20.0),
            ),
            (
                ComputedCssDimension::LengthPx(40.0),
                ComputedCssDimension::LengthPx(20.0),
            ),
        ],
        RootHtmlNamedAreaAutoMinimumAxis::Rows => [
            (
                ComputedCssDimension::LengthPx(80.0),
                ComputedCssDimension::LengthPx(80.0),
            ),
            (
                ComputedCssDimension::LengthPx(40.0),
                ComputedCssDimension::LengthPx(40.0),
            ),
            (
                ComputedCssDimension::LengthPx(40.0),
                ComputedCssDimension::LengthPx(40.0),
            ),
        ],
    };
    for (area_index, (element, (expected_width, expected_height))) in
        area_items.iter().zip(expected_dimensions).enumerate()
    {
        let element = element.ok_or_else(|| {
            unsupported(
                owner,
                "grid-area",
                "각 named area에는 직접 자식 하나가 필요합니다",
            )
        })?;
        if element.layout_dimensions.width != expected_width {
            return Err(unsupported(
                element.node_id,
                "width",
                if area_index == 0 && axis == RootHtmlNamedAreaAutoMinimumAxis::Columns {
                    "Columns 축의 header item은 width:auto여야 합니다"
                } else {
                    "Root HTML named-area 자동 최소 크기 item의 width가 고정 profile과 다릅니다"
                },
            ));
        }
        if element.layout_dimensions.height != expected_height {
            return Err(unsupported(
                element.node_id,
                "height",
                "Root HTML named-area 자동 최소 크기 item의 height가 고정 profile과 다릅니다",
            ));
        }
        if element.layout_dimensions.min_width != ComputedCssDimension::Auto
            || element.layout_dimensions.min_height != ComputedCssDimension::Auto
        {
            return Err(unsupported(
                element.node_id,
                if element.layout_dimensions.min_width != ComputedCssDimension::Auto {
                    "min-width"
                } else {
                    "min-height"
                },
                "Root HTML named-area 자동 최소 크기 item은 min-width/height:auto를 요구합니다",
            ));
        }
        if element.layout_dimensions.max_width != ComputedCssMaxSize::None
            || element.layout_dimensions.max_height != ComputedCssMaxSize::None
        {
            return Err(unsupported(
                element.node_id,
                if element.layout_dimensions.max_width != ComputedCssMaxSize::None {
                    "max-width"
                } else {
                    "max-height"
                },
                "Root HTML named-area 자동 최소 크기 item은 max-width/height:none을 요구합니다",
            ));
        }
        if element.layout_aspect_ratio.is_some()
            || element
                .properties
                .get("aspect-ratio")
                .is_some_and(|value| value != "auto")
        {
            return Err(unsupported(
                element.node_id,
                "aspect-ratio",
                "Root HTML named-area 자동 최소 크기 item은 aspect-ratio를 허용하지 않습니다",
            ));
        }
    }
    Ok(())
}
