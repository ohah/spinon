use std::collections::BTreeMap;

use spinon_core::NodeId;
use spinon_layout::{
    FlexDirection, LayoutBorder, LayoutBoxSizing, LayoutDimension, LayoutDisplay, LayoutEdges,
    LayoutGap, LayoutStyle, TextDirection,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssMath, ComputedCssMaxSize, ComputedElementStyle,
    ComputedStyleProfile, ComputedStyleSnapshot,
};

mod flex_values;
mod spacing_values;

use super::typed_math::CssMathProjector;
use crate::StyleLayoutError;
use flex_values::{
    parse_align_items, parse_direction, parse_flex_direction, parse_flex_wrap,
    parse_justify_content, parse_number, parse_order,
};
use spacing_values::{layout_math_property, parse_gap, parse_margin, parse_nonnegative_spacing};

pub(super) struct ProjectedStyles {
    pub styles: BTreeMap<NodeId, LayoutStyle>,
    pub css_math: Vec<spinon_layout::LayoutCssMathValue>,
}

pub(super) fn project_styles(
    snapshot: &ComputedStyleSnapshot,
) -> Result<ProjectedStyles, StyleLayoutError> {
    let mut output = BTreeMap::new();
    let mut math = CssMathProjector::default();
    for element in snapshot.elements.iter() {
        let node = element.node_id;
        if let Some(property) = unsupported_aspect_ratio_constraint(element) {
            return Err(StyleLayoutError::UnsupportedAspectRatioConstraint { node, property });
        }
        let supports_size_constraints = matches!(
            snapshot.profile,
            ComputedStyleProfile::RuntimeFlexLayoutV1
                | ComputedStyleProfile::RuntimeFlexPaintV1
                | ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
                | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
                | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
                | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
                | ComputedStyleProfile::RuntimeBlockPaintV1
                | ComputedStyleProfile::RuntimeBlockFormattingV1
        );
        let supports_border_layout = matches!(
            snapshot.profile,
            ComputedStyleProfile::RuntimeFlexLayoutV1
                | ComputedStyleProfile::RuntimeFlexPaintV1
                | ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
                | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
                | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
                | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
                | ComputedStyleProfile::RuntimeBlockPaintV1
                | ComputedStyleProfile::RuntimeBlockFormattingV1
        );
        let supports_runtime_flex = matches!(
            snapshot.profile,
            ComputedStyleProfile::RuntimeFlexLayoutV1
                | ComputedStyleProfile::RuntimeFlexPaintV1
                | ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
                | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
                | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
                | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
        );
        let flex_direction = parse_flex_direction(
            node,
            required(element, "flex-direction")?,
            supports_runtime_flex,
        )?;
        let direction = parse_direction(node, required(element, "direction")?)?;
        if direction == TextDirection::Rtl
            && matches!(
                flex_direction,
                FlexDirection::RowReverse | FlexDirection::ColumnReverse
            )
        {
            let value = match flex_direction {
                FlexDirection::RowReverse => "row-reverse",
                FlexDirection::ColumnReverse => "column-reverse",
                FlexDirection::Row | FlexDirection::Column => unreachable!(),
            };
            return Err(unsupported_value(
                node,
                "flex-direction",
                &format!("{value} with direction:rtl"),
            ));
        }
        let style = LayoutStyle {
            display: parse_display(
                node,
                required(element, "display")?,
                snapshot.profile == ComputedStyleProfile::RuntimeBlockFormattingV1,
            )?,
            box_sizing: parse_box_sizing(node, required(element, "box-sizing")?)?,
            width: parse_dimension(
                node,
                "width",
                required(element, "width")?,
                element.layout_dimensions.width,
                element.layout_math_values.get("width"),
                &mut math,
            )?,
            height: parse_dimension(
                node,
                "height",
                required(element, "height")?,
                element.layout_dimensions.height,
                element.layout_math_values.get("height"),
                &mut math,
            )?,
            min_width: if supports_size_constraints {
                parse_dimension(
                    node,
                    "min-width",
                    required(element, "min-width")?,
                    element.layout_dimensions.min_width,
                    element.layout_math_values.get("min-width"),
                    &mut math,
                )?
            } else {
                LayoutStyle::default().min_width
            },
            max_width: if supports_size_constraints {
                parse_dimension(
                    node,
                    "max-width",
                    required(element, "max-width")?,
                    max_size_as_dimension(element.layout_dimensions.max_width),
                    element.layout_math_values.get("max-width"),
                    &mut math,
                )?
            } else {
                LayoutStyle::default().max_width
            },
            min_height: if supports_size_constraints {
                parse_dimension(
                    node,
                    "min-height",
                    required(element, "min-height")?,
                    element.layout_dimensions.min_height,
                    element.layout_math_values.get("min-height"),
                    &mut math,
                )?
            } else {
                LayoutStyle::default().min_height
            },
            max_height: if supports_size_constraints {
                parse_dimension(
                    node,
                    "max-height",
                    required(element, "max-height")?,
                    max_size_as_dimension(element.layout_dimensions.max_height),
                    element.layout_math_values.get("max-height"),
                    &mut math,
                )?
            } else {
                LayoutStyle::default().max_height
            },
            aspect_ratio: if supports_size_constraints {
                element.layout_aspect_ratio
            } else {
                None
            },
            order: if supports_runtime_flex {
                parse_order(node, required(element, "order")?)?
            } else {
                0
            },
            flex_basis: parse_dimension(
                node,
                "flex-basis",
                required(element, "flex-basis")?,
                element.layout_dimensions.flex_basis,
                element.layout_math_values.get("flex-basis"),
                &mut math,
            )?,
            flex_direction,
            flex_wrap: if supports_runtime_flex {
                parse_flex_wrap(node, required(element, "flex-wrap")?)?
            } else {
                LayoutStyle::default().flex_wrap
            },
            direction,
            align_items: match snapshot.profile {
                ComputedStyleProfile::FlexAlignmentV1
                | ComputedStyleProfile::FlexAlignmentCascadeLayersV1
                | ComputedStyleProfile::RuntimeFlexLayoutV1
                | ComputedStyleProfile::RuntimeFlexPaintV1
                | ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
                | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
                | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
                | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
                | ComputedStyleProfile::RuntimeBlockPaintV1
                | ComputedStyleProfile::RuntimeBlockFormattingV1 => {
                    parse_align_items(node, required(element, "align-items")?)?
                }
                _ => LayoutStyle::default().align_items,
            },
            justify_content: match snapshot.profile {
                ComputedStyleProfile::FlexAlignmentV1
                | ComputedStyleProfile::FlexAlignmentCascadeLayersV1
                | ComputedStyleProfile::RuntimeFlexLayoutV1
                | ComputedStyleProfile::RuntimeFlexPaintV1
                | ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
                | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
                | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
                | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
                | ComputedStyleProfile::RuntimeBlockPaintV1
                | ComputedStyleProfile::RuntimeBlockFormattingV1 => {
                    parse_justify_content(node, required(element, "justify-content")?)?
                }
                _ => LayoutStyle::default().justify_content,
            },
            flex_grow: parse_number(node, "flex-grow", required(element, "flex-grow")?)?,
            flex_shrink: parse_number(node, "flex-shrink", required(element, "flex-shrink")?)?,
            gap: LayoutGap {
                row: parse_gap(
                    node,
                    "row-gap",
                    required(element, "row-gap")?,
                    element.layout_spacing.row_gap,
                    element.layout_math_values.get("row-gap"),
                    &mut math,
                )?,
                column: parse_gap(
                    node,
                    "column-gap",
                    required(element, "column-gap")?,
                    element.layout_spacing.column_gap,
                    element.layout_math_values.get("column-gap"),
                    &mut math,
                )?,
            },
            margin: match snapshot.profile {
                ComputedStyleProfile::FlexMarginV1
                | ComputedStyleProfile::RuntimeFlexLayoutV1
                | ComputedStyleProfile::RuntimeFlexPaintV1
                | ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
                | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
                | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
                | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
                | ComputedStyleProfile::RuntimeBlockPaintV1
                | ComputedStyleProfile::RuntimeBlockFormattingV1 => LayoutEdges {
                    top: parse_margin(
                        node,
                        "margin-top",
                        required(element, "margin-top")?,
                        element.layout_spacing.margin.top,
                        element.layout_math_values.get("margin-top"),
                        &mut math,
                        snapshot.profile == ComputedStyleProfile::RuntimeBlockFormattingV1,
                    )?,
                    right: parse_margin(
                        node,
                        "margin-right",
                        required(element, "margin-right")?,
                        element.layout_spacing.margin.right,
                        element.layout_math_values.get("margin-right"),
                        &mut math,
                        snapshot.profile == ComputedStyleProfile::RuntimeBlockFormattingV1,
                    )?,
                    bottom: parse_margin(
                        node,
                        "margin-bottom",
                        required(element, "margin-bottom")?,
                        element.layout_spacing.margin.bottom,
                        element.layout_math_values.get("margin-bottom"),
                        &mut math,
                        snapshot.profile == ComputedStyleProfile::RuntimeBlockFormattingV1,
                    )?,
                    left: parse_margin(
                        node,
                        "margin-left",
                        required(element, "margin-left")?,
                        element.layout_spacing.margin.left,
                        element.layout_math_values.get("margin-left"),
                        &mut math,
                        snapshot.profile == ComputedStyleProfile::RuntimeBlockFormattingV1,
                    )?,
                },
                _ => LayoutEdges::default(),
            },
            padding: match snapshot.profile {
                ComputedStyleProfile::RuntimeFlexLayoutV1
                | ComputedStyleProfile::RuntimeFlexPaintV1
                | ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
                | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
                | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
                | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
                | ComputedStyleProfile::RuntimeBlockPaintV1
                | ComputedStyleProfile::RuntimeBlockFormattingV1 => LayoutEdges {
                    top: parse_nonnegative_spacing(
                        node,
                        "padding-top",
                        required(element, "padding-top")?,
                        element.layout_spacing.padding.top,
                        element.layout_math_values.get("padding-top"),
                        &mut math,
                    )?,
                    right: parse_nonnegative_spacing(
                        node,
                        "padding-right",
                        required(element, "padding-right")?,
                        element.layout_spacing.padding.right,
                        element.layout_math_values.get("padding-right"),
                        &mut math,
                    )?,
                    bottom: parse_nonnegative_spacing(
                        node,
                        "padding-bottom",
                        required(element, "padding-bottom")?,
                        element.layout_spacing.padding.bottom,
                        element.layout_math_values.get("padding-bottom"),
                        &mut math,
                    )?,
                    left: parse_nonnegative_spacing(
                        node,
                        "padding-left",
                        required(element, "padding-left")?,
                        element.layout_spacing.padding.left,
                        element.layout_math_values.get("padding-left"),
                        &mut math,
                    )?,
                },
                _ => LayoutEdges::default(),
            },
            border: if supports_border_layout {
                LayoutBorder {
                    top: element.layout_border.top,
                    right: element.layout_border.right,
                    bottom: element.layout_border.bottom,
                    left: element.layout_border.left,
                }
            } else {
                LayoutBorder::default()
            },
        };
        if output.insert(node, style).is_some() {
            return Err(StyleLayoutError::DuplicateComputedElement(node));
        }
    }
    Ok(ProjectedStyles {
        styles: output,
        css_math: math.into_values(),
    })
}

fn unsupported_aspect_ratio_constraint(element: &ComputedElementStyle) -> Option<&'static str> {
    element.layout_aspect_ratio?;
    let dimensions = element.layout_dimensions;
    if dimensions.min_width != ComputedCssDimension::Auto {
        Some("min-width")
    } else if dimensions.max_width != ComputedCssMaxSize::None {
        Some("max-width")
    } else if dimensions.min_height != ComputedCssDimension::Auto {
        Some("min-height")
    } else if dimensions.max_height != ComputedCssMaxSize::None {
        Some("max-height")
    } else {
        None
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

fn parse_display(
    node: NodeId,
    value: &str,
    allow_flow_root: bool,
) -> Result<LayoutDisplay, StyleLayoutError> {
    match value {
        "flex" => Ok(LayoutDisplay::Flex),
        "block" => Ok(LayoutDisplay::Block),
        "flow-root" if allow_flow_root => Ok(LayoutDisplay::FlowRoot),
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

fn max_size_as_dimension(value: ComputedCssMaxSize) -> ComputedCssDimension {
    match value {
        ComputedCssMaxSize::None => ComputedCssDimension::Auto,
        ComputedCssMaxSize::LengthPx(value) => ComputedCssDimension::LengthPx(value),
        ComputedCssMaxSize::Percentage(value) => ComputedCssDimension::Percentage(value),
        ComputedCssMaxSize::Unsupported => ComputedCssDimension::Unsupported,
    }
}

fn parse_dimension(
    node: NodeId,
    property: &'static str,
    serialized_value: &str,
    value: ComputedCssDimension,
    math: Option<&ComputedCssMath>,
    projector: &mut CssMathProjector,
) -> Result<LayoutDimension, StyleLayoutError> {
    if let Some(math) = math {
        let id = projector.project(node, layout_math_property(node, property)?, math)?;
        return Ok(LayoutDimension::Calc(id));
    }
    match value {
        ComputedCssDimension::Auto => Ok(LayoutDimension::Auto),
        ComputedCssDimension::LengthPx(value) if value.is_finite() && value >= 0.0 => {
            Ok(LayoutDimension::Fixed(value))
        }
        ComputedCssDimension::Percentage(value) if value.is_finite() && value >= 0.0 => {
            Ok(LayoutDimension::Percent(value))
        }
        ComputedCssDimension::LengthPx(_)
        | ComputedCssDimension::Percentage(_)
        | ComputedCssDimension::Unsupported => unsupported(node, property, serialized_value),
    }
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
