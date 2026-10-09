use std::collections::BTreeMap;

use spinon_core::NodeId;
use spinon_layout::{
    FlexDirection, LayoutAlignItems, LayoutBoxSizing, LayoutDimension, LayoutDisplay, LayoutEdges,
    LayoutGap, LayoutJustifyContent, LayoutStyle, TextDirection,
};
use spinon_style::{ComputedStyleProfile, ComputedStyleSnapshot};

use crate::StyleLayoutError;

pub(super) fn project_styles(
    snapshot: &ComputedStyleSnapshot,
) -> Result<BTreeMap<NodeId, LayoutStyle>, StyleLayoutError> {
    let mut output = BTreeMap::new();
    for element in &snapshot.elements {
        let node = element.node_id;
        let style = LayoutStyle {
            display: parse_display(node, required(element, "display")?)?,
            box_sizing: parse_box_sizing(node, required(element, "box-sizing")?)?,
            width: parse_dimension(node, "width", required(element, "width")?)?,
            height: parse_dimension(node, "height", required(element, "height")?)?,
            flex_basis: parse_dimension(node, "flex-basis", required(element, "flex-basis")?)?,
            flex_direction: parse_flex_direction(node, required(element, "flex-direction")?)?,
            direction: parse_direction(node, required(element, "direction")?)?,
            align_items: match snapshot.profile {
                ComputedStyleProfile::FlexAlignmentV1
                | ComputedStyleProfile::FlexAlignmentCascadeLayersV1
                | ComputedStyleProfile::RuntimeFlexLayoutV1 => {
                    parse_align_items(node, required(element, "align-items")?)?
                }
                _ => LayoutStyle::default().align_items,
            },
            justify_content: match snapshot.profile {
                ComputedStyleProfile::FlexAlignmentV1
                | ComputedStyleProfile::FlexAlignmentCascadeLayersV1
                | ComputedStyleProfile::RuntimeFlexLayoutV1 => {
                    parse_justify_content(node, required(element, "justify-content")?)?
                }
                _ => LayoutStyle::default().justify_content,
            },
            flex_grow: parse_number(node, "flex-grow", required(element, "flex-grow")?)?,
            flex_shrink: parse_number(node, "flex-shrink", required(element, "flex-shrink")?)?,
            gap: LayoutGap {
                row: parse_gap(node, "row-gap", required(element, "row-gap")?)?,
                column: parse_gap(node, "column-gap", required(element, "column-gap")?)?,
            },
            margin: match snapshot.profile {
                ComputedStyleProfile::FlexMarginV1 | ComputedStyleProfile::RuntimeFlexLayoutV1 => {
                    LayoutEdges {
                        top: parse_css_margin(
                            node,
                            "margin-top",
                            required(element, "margin-top")?,
                        )?,
                        right: parse_css_margin(
                            node,
                            "margin-right",
                            required(element, "margin-right")?,
                        )?,
                        bottom: parse_css_margin(
                            node,
                            "margin-bottom",
                            required(element, "margin-bottom")?,
                        )?,
                        left: parse_css_margin(
                            node,
                            "margin-left",
                            required(element, "margin-left")?,
                        )?,
                    }
                }
                _ => LayoutEdges::default(),
            },
            padding: match snapshot.profile {
                ComputedStyleProfile::RuntimeFlexLayoutV1 => LayoutEdges {
                    top: parse_css_px(node, "padding-top", required(element, "padding-top")?)?,
                    right: parse_css_px(
                        node,
                        "padding-right",
                        required(element, "padding-right")?,
                    )?,
                    bottom: parse_css_px(
                        node,
                        "padding-bottom",
                        required(element, "padding-bottom")?,
                    )?,
                    left: parse_css_px(node, "padding-left", required(element, "padding-left")?)?,
                },
                _ => LayoutEdges::default(),
            },
        };
        if output.insert(node, style).is_some() {
            return Err(StyleLayoutError::DuplicateComputedElement(node));
        }
    }
    Ok(output)
}

fn parse_align_items(node: NodeId, value: &str) -> Result<LayoutAlignItems, StyleLayoutError> {
    match value {
        "normal" | "stretch" => Ok(LayoutAlignItems::Stretch),
        "flex-start" => Ok(LayoutAlignItems::FlexStart),
        "flex-end" => Ok(LayoutAlignItems::FlexEnd),
        "center" => Ok(LayoutAlignItems::Center),
        value => unsupported(node, "align-items", value),
    }
}

fn parse_justify_content(
    node: NodeId,
    value: &str,
) -> Result<LayoutJustifyContent, StyleLayoutError> {
    match value {
        "normal" | "flex-start" => Ok(LayoutJustifyContent::FlexStart),
        "flex-end" => Ok(LayoutJustifyContent::FlexEnd),
        "center" => Ok(LayoutJustifyContent::Center),
        "space-between" => Ok(LayoutJustifyContent::SpaceBetween),
        "space-around" => Ok(LayoutJustifyContent::SpaceAround),
        "space-evenly" => Ok(LayoutJustifyContent::SpaceEvenly),
        value => unsupported(node, "justify-content", value),
    }
}

fn required<'a>(
    element: &'a spinon_style::ComputedElementStyle,
    property: &'static str,
) -> Result<&'a str, StyleLayoutError> {
    element.properties.get(property).map(String::as_str).ok_or(
        StyleLayoutError::MissingComputedProperty {
            node: element.node_id,
            property,
        },
    )
}

fn parse_display(node: NodeId, value: &str) -> Result<LayoutDisplay, StyleLayoutError> {
    match value {
        "flex" => Ok(LayoutDisplay::Flex),
        "block" => Ok(LayoutDisplay::Block),
        "none" => Ok(LayoutDisplay::None),
        value => unsupported(node, "display", value),
    }
}

fn parse_box_sizing(node: NodeId, value: &str) -> Result<LayoutBoxSizing, StyleLayoutError> {
    match value {
        "border-box" => Ok(LayoutBoxSizing::BorderBox),
        "content-box" => Ok(LayoutBoxSizing::ContentBox),
        value => unsupported(node, "box-sizing", value),
    }
}

fn parse_dimension(
    node: NodeId,
    property: &'static str,
    value: &str,
) -> Result<LayoutDimension, StyleLayoutError> {
    if value == "auto" {
        return Ok(LayoutDimension::Auto);
    }
    parse_css_px(node, property, value).map(LayoutDimension::Fixed)
}

fn parse_flex_direction(node: NodeId, value: &str) -> Result<FlexDirection, StyleLayoutError> {
    match value {
        "row" => Ok(FlexDirection::Row),
        "column" => Ok(FlexDirection::Column),
        value => unsupported(node, "flex-direction", value),
    }
}

fn parse_direction(node: NodeId, value: &str) -> Result<TextDirection, StyleLayoutError> {
    match value {
        "ltr" => Ok(TextDirection::Ltr),
        "rtl" => Ok(TextDirection::Rtl),
        value => unsupported(node, "direction", value),
    }
}

fn parse_number(
    node: NodeId,
    property: &'static str,
    value: &str,
) -> Result<f32, StyleLayoutError> {
    let parsed = value
        .parse::<f32>()
        .ok()
        .filter(|number| number.is_finite() && *number >= 0.0);
    parsed.ok_or_else(|| unsupported_value(node, property, value))
}

fn parse_gap(node: NodeId, property: &'static str, value: &str) -> Result<f32, StyleLayoutError> {
    if value == "normal" {
        return Ok(0.0);
    }
    parse_css_px(node, property, value)
}

fn parse_css_px(
    node: NodeId,
    property: &'static str,
    value: &str,
) -> Result<f32, StyleLayoutError> {
    let Some(number) = value.strip_suffix("px") else {
        return unsupported(node, property, value);
    };
    let parsed = number
        .parse::<f32>()
        .ok()
        .filter(|number| number.is_finite() && *number >= 0.0);
    parsed.ok_or_else(|| unsupported_value(node, property, value))
}

fn parse_css_margin(
    node: NodeId,
    property: &'static str,
    value: &str,
) -> Result<f32, StyleLayoutError> {
    let Some(number) = value.strip_suffix("px") else {
        return unsupported(node, property, value);
    };
    let parsed = number
        .parse::<f32>()
        .ok()
        .filter(|number| number.is_finite());
    parsed.ok_or_else(|| unsupported_value(node, property, value))
}

fn unsupported<T>(
    node: NodeId,
    property: &'static str,
    value: &str,
) -> Result<T, StyleLayoutError> {
    Err(unsupported_value(node, property, value))
}

fn unsupported_value(node: NodeId, property: &'static str, value: &str) -> StyleLayoutError {
    StyleLayoutError::UnsupportedComputedValue {
        node,
        property,
        value: value.to_owned(),
    }
}
