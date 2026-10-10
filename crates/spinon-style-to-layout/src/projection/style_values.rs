use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, NodeId};
use spinon_layout::{
    FlexDirection, LayoutBorder, LayoutEdges, LayoutGap, LayoutPositioning, LayoutStyle,
    TextDirection,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssMaxSize, ComputedElementStyle, ComputedStyleProfile,
    ComputedStyleSnapshot,
};

mod flex_values;
mod positioning_values;
mod primitive_values;
mod spacing_values;

use super::typed_math::CssMathProjector;
use crate::StyleLayoutError;
use flex_values::{
    parse_align_items, parse_direction, parse_flex_direction, parse_flex_wrap,
    parse_justify_content, parse_number, parse_order, parse_runtime_align_content,
    parse_runtime_align_items, parse_runtime_align_self, parse_runtime_justify_content,
};
use positioning_values::parse_positioning;
use primitive_values::{
    max_size_as_dimension, parse_box_sizing, parse_dimension, parse_display, required,
    unsupported_value,
};
use spacing_values::{parse_gap, parse_margin, parse_nonnegative_spacing};

pub(super) struct ProjectedStyles {
    pub styles: BTreeMap<NodeId, LayoutStyle>,
    pub positioning: BTreeMap<NodeId, LayoutPositioning>,
    pub css_math: Vec<spinon_layout::LayoutCssMathValue>,
}

pub(super) fn project_styles(
    tree: &HostDocumentSnapshot,
    root: HostNodeHandle,
    snapshot: &ComputedStyleSnapshot,
) -> Result<ProjectedStyles, StyleLayoutError> {
    let mut output = BTreeMap::new();
    let mut positioning = BTreeMap::new();
    let mut math = CssMathProjector::default();
    let runtime_flex_items = if supports_runtime_flex(snapshot.profile) {
        runtime_flex_item_nodes(tree, root, snapshot)
    } else {
        BTreeSet::new()
    };
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
                | ComputedStyleProfile::RuntimeBlockPositioningV1
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
                | ComputedStyleProfile::RuntimeBlockPositioningV1
        );
        let supports_runtime_flex = supports_runtime_flex(snapshot.profile);
        let supports_auto_margin = runtime_flex_items.contains(&node)
            || matches!(
                snapshot.profile,
                ComputedStyleProfile::RuntimeBlockFormattingV1
                    | ComputedStyleProfile::RuntimeBlockPositioningV1
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
                matches!(
                    snapshot.profile,
                    ComputedStyleProfile::RuntimeBlockFormattingV1
                        | ComputedStyleProfile::RuntimeBlockPositioningV1
                ),
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
                ComputedStyleProfile::RuntimeFlexLayoutV1
                | ComputedStyleProfile::RuntimeFlexPaintV1
                | ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
                | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
                | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
                | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1 => {
                    parse_runtime_align_items(node, required(element, "align-items")?)?
                }
                ComputedStyleProfile::FlexAlignmentV1
                | ComputedStyleProfile::FlexAlignmentCascadeLayersV1
                | ComputedStyleProfile::RuntimeBlockPaintV1
                | ComputedStyleProfile::RuntimeBlockFormattingV1
                | ComputedStyleProfile::RuntimeBlockPositioningV1 => {
                    parse_align_items(node, required(element, "align-items")?)?
                }
                _ => LayoutStyle::default().align_items,
            },
            align_self: if supports_runtime_flex {
                parse_runtime_align_self(node, required(element, "align-self")?)?
            } else {
                LayoutStyle::default().align_self
            },
            align_content: if supports_runtime_flex {
                Some(parse_runtime_align_content(
                    node,
                    required(element, "align-content")?,
                )?)
            } else {
                None
            },
            justify_content: match snapshot.profile {
                ComputedStyleProfile::FlexAlignmentV1
                | ComputedStyleProfile::FlexAlignmentCascadeLayersV1
                | ComputedStyleProfile::RuntimeBlockPaintV1
                | ComputedStyleProfile::RuntimeBlockFormattingV1
                | ComputedStyleProfile::RuntimeBlockPositioningV1 => {
                    parse_justify_content(node, required(element, "justify-content")?)?
                }
                ComputedStyleProfile::RuntimeFlexLayoutV1
                | ComputedStyleProfile::RuntimeFlexPaintV1
                | ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
                | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
                | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
                | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1 => {
                    parse_runtime_justify_content(node, required(element, "justify-content")?)?
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
                | ComputedStyleProfile::RuntimeBlockFormattingV1
                | ComputedStyleProfile::RuntimeBlockPositioningV1 => LayoutEdges {
                    top: parse_margin(
                        node,
                        "margin-top",
                        required(element, "margin-top")?,
                        element.layout_spacing.margin.top,
                        element.layout_math_values.get("margin-top"),
                        &mut math,
                        supports_auto_margin,
                    )?,
                    right: parse_margin(
                        node,
                        "margin-right",
                        required(element, "margin-right")?,
                        element.layout_spacing.margin.right,
                        element.layout_math_values.get("margin-right"),
                        &mut math,
                        supports_auto_margin,
                    )?,
                    bottom: parse_margin(
                        node,
                        "margin-bottom",
                        required(element, "margin-bottom")?,
                        element.layout_spacing.margin.bottom,
                        element.layout_math_values.get("margin-bottom"),
                        &mut math,
                        supports_auto_margin,
                    )?,
                    left: parse_margin(
                        node,
                        "margin-left",
                        required(element, "margin-left")?,
                        element.layout_spacing.margin.left,
                        element.layout_math_values.get("margin-left"),
                        &mut math,
                        supports_auto_margin,
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
                | ComputedStyleProfile::RuntimeBlockFormattingV1
                | ComputedStyleProfile::RuntimeBlockPositioningV1 => LayoutEdges {
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
        if supports_positioning(snapshot.profile) {
            positioning.insert(
                node,
                parse_positioning(
                    element,
                    &mut math,
                    snapshot.profile == ComputedStyleProfile::RuntimeBlockPositioningV1,
                )?,
            );
        }
        if output.insert(node, style).is_some() {
            return Err(StyleLayoutError::DuplicateComputedElement(node));
        }
    }
    Ok(ProjectedStyles {
        styles: output,
        positioning,
        css_math: math.into_values(),
    })
}

fn supports_positioning(profile: ComputedStyleProfile) -> bool {
    matches!(
        profile,
        ComputedStyleProfile::RuntimeFlexLayoutV1
            | ComputedStyleProfile::RuntimeFlexPaintV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
            | ComputedStyleProfile::RuntimeBlockPaintV1
            | ComputedStyleProfile::RuntimeBlockFormattingV1
            | ComputedStyleProfile::RuntimeBlockPositioningV1
    )
}

fn supports_runtime_flex(profile: ComputedStyleProfile) -> bool {
    matches!(
        profile,
        ComputedStyleProfile::RuntimeFlexLayoutV1
            | ComputedStyleProfile::RuntimeFlexPaintV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
    )
}

fn runtime_flex_item_nodes(
    tree: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> BTreeSet<NodeId> {
    let display_by_node = styles
        .elements
        .iter()
        .filter_map(|element| {
            element
                .properties
                .get("display")
                .map(|display| (element.node_id, display.as_str()))
        })
        .collect::<BTreeMap<_, _>>();
    let mut flex_items = BTreeSet::new();
    let mut pending = vec![root];
    while let Some(parent) = pending.pop() {
        let is_flex_container = display_by_node
            .get(&parent.id())
            .is_some_and(|display| *display == "flex");
        let Some(children) = tree.children(parent) else {
            continue;
        };
        for child in children {
            if is_flex_container {
                flex_items.insert(child.id());
            }
            pending.push(child);
        }
    }
    flex_items
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
