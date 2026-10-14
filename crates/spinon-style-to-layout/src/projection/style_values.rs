use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, NodeId};
use spinon_layout::{
    FlexDirection, GridContainerInput, GridItemInput, LayoutBorder, LayoutEdges, LayoutGap,
    LayoutOverflow, LayoutPositioning, LayoutStyle, TextDirection,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridLayout, ComputedCssMaxSize, ComputedCssOverflow,
    ComputedCssPosition, ComputedElementStyle, ComputedStyleProfile, ComputedStyleSnapshot,
};

mod flex_values;
mod grid_placement_values;
mod grid_values;
mod positioning_values;
mod primitive_values;
mod spacing_values;

use super::fixed_containing_block::validate_fixed_ancestor_effects;
use super::typed_math::CssMathProjector;
use crate::StyleLayoutError;
use flex_values::{
    parse_align_items, parse_direction, parse_flex_direction, parse_flex_wrap,
    parse_justify_content, parse_number, parse_order, parse_runtime_align_content,
    parse_runtime_align_items, parse_runtime_align_self, parse_runtime_justify_content,
};
use grid_placement_values::project_grid_item_placement;
use grid_values::project_grid_layout;
use positioning_values::parse_positioning;
use primitive_values::{
    max_size_as_dimension, parse_box_sizing, parse_dimension, parse_display, required,
    unsupported_value,
};
use spacing_values::{parse_gap, parse_margin, parse_nonnegative_spacing};

pub(super) struct ProjectedStyles {
    pub styles: BTreeMap<NodeId, LayoutStyle>,
    pub positioning: BTreeMap<NodeId, LayoutPositioning>,
    pub grid_containers: BTreeMap<NodeId, GridContainerInput>,
    pub grid_items: BTreeMap<NodeId, GridItemInput>,
    pub css_math: Vec<spinon_layout::LayoutCssMathValue>,
}

pub(super) fn project_styles(
    tree: &HostDocumentSnapshot,
    root: HostNodeHandle,
    snapshot: &ComputedStyleSnapshot,
) -> Result<ProjectedStyles, StyleLayoutError> {
    validate_fixed_ancestor_effects(tree, root, snapshot)?;
    let mut output = BTreeMap::new();
    let mut positioning = BTreeMap::new();
    let mut grid_containers = BTreeMap::new();
    let mut grid_styles = BTreeMap::<NodeId, &ComputedCssGridLayout>::new();
    let mut grid_items = BTreeMap::new();
    let mut math = CssMathProjector::default();
    let grid_nodes =
        is_grid_profile(snapshot.profile).then(|| runtime_grid_nodes(tree, root, snapshot));
    let runtime_flex_items = if supports_runtime_flex(snapshot.profile) {
        runtime_flex_item_nodes(tree, root, snapshot)
    } else {
        BTreeSet::new()
    };
    for element in snapshot.elements.iter() {
        let node = element.node_id;
        if matches!(
            snapshot.profile,
            ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
                | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
        ) {
            if grid_nodes
                .as_ref()
                .is_some_and(|nodes| nodes.grid_items.contains(&node))
            {
                let order = required(element, "order")?;
                if order != "0" {
                    return Err(StyleLayoutError::UnsupportedComputedValue {
                        node,
                        property: "order",
                        value: order.to_owned(),
                    });
                }
            }
            if grid_nodes
                .as_ref()
                .is_some_and(|nodes| nodes.grid_containers.contains(&node))
                && element.layout_position == ComputedCssPosition::Absolute
            {
                let indefinite_dimension = [
                    ("width", element.layout_dimensions.width),
                    ("height", element.layout_dimensions.height),
                ]
                .into_iter()
                .find_map(|(property, value)| {
                    (value == ComputedCssDimension::Auto).then_some(property)
                });
                if let Some(property) = indefinite_dimension {
                    return Err(StyleLayoutError::UnsupportedComputedValue {
                        node,
                        property,
                        value: "auto (absolute Grid axis has no definite size)".to_owned(),
                    });
                }
            }
        }
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
                | ComputedStyleProfile::RuntimeBlockFlexStackingV1
                | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
                | ComputedStyleProfile::RuntimeBlockFlexGridV1
                | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
                | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
                | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
                | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
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
                | ComputedStyleProfile::RuntimeBlockFlexStackingV1
                | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
                | ComputedStyleProfile::RuntimeBlockFlexGridV1
                | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
                | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
                | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
                | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
        );
        let supports_runtime_flex = supports_runtime_flex(snapshot.profile);
        let supports_auto_margin = runtime_flex_items.contains(&node)
            || matches!(
                snapshot.profile,
                ComputedStyleProfile::RuntimeBlockFormattingV1
                    | ComputedStyleProfile::RuntimeBlockPositioningV1
                    | ComputedStyleProfile::RuntimeBlockFlexStackingV1
                    | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
                    | ComputedStyleProfile::RuntimeBlockFlexGridV1
                    | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
                    | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
                    | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
                    | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
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
        let mut style = LayoutStyle {
            display: parse_display(
                node,
                required(element, "display")?,
                matches!(
                    snapshot.profile,
                    ComputedStyleProfile::RuntimeBlockFormattingV1
                        | ComputedStyleProfile::RuntimeBlockPositioningV1
                        | ComputedStyleProfile::RuntimeBlockFlexStackingV1
                        | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
                        | ComputedStyleProfile::RuntimeBlockFlexGridV1
                        | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
                        | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
                        | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
                        | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
                ),
                matches!(
                    snapshot.profile,
                    ComputedStyleProfile::RuntimeBlockFlexGridV1
                        | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
                        | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
                        | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
                        | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
                ),
            )?,
            overflow_x: if supports_overflow_clip(snapshot.profile) {
                parse_overflow(node, "overflow-x", element.layout_overflow_x)?
            } else {
                LayoutOverflow::Visible
            },
            overflow_y: if supports_overflow_clip(snapshot.profile) {
                parse_overflow(node, "overflow-y", element.layout_overflow_y)?
            } else {
                LayoutOverflow::Visible
            },
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
                | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
                | ComputedStyleProfile::RuntimeBlockFlexStackingV1
                | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
                | ComputedStyleProfile::RuntimeBlockFlexGridV1
                | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
                | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
                | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
                | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1 => {
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
                | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
                | ComputedStyleProfile::RuntimeBlockFlexStackingV1
                | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
                | ComputedStyleProfile::RuntimeBlockFlexGridV1
                | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
                | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
                | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
                | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1 => {
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
                | ComputedStyleProfile::RuntimeBlockPositioningV1
                | ComputedStyleProfile::RuntimeBlockFlexStackingV1
                | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
                | ComputedStyleProfile::RuntimeBlockFlexGridV1
                | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
                | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
                | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
                | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1 => LayoutEdges {
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
                | ComputedStyleProfile::RuntimeBlockPositioningV1
                | ComputedStyleProfile::RuntimeBlockFlexStackingV1
                | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
                | ComputedStyleProfile::RuntimeBlockFlexGridV1
                | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
                | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
                | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
                | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1 => LayoutEdges {
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
        if grid_nodes
            .as_ref()
            .is_some_and(|nodes| nodes.hidden.contains(&node))
        {
            style.display = spinon_layout::LayoutDisplay::None;
        }
        if let Some(nodes) = grid_nodes.as_ref() {
            if nodes.grid_containers.contains(&node) {
                reject_grid_alignment(element, true, false)?;
            }
            if nodes.grid_items.contains(&node) {
                reject_grid_alignment(element, false, true)?;
            }
        }
        if style.display == spinon_layout::LayoutDisplay::Grid {
            let grid = element.grid_layout.as_ref().ok_or_else(|| {
                crate::StyleLayoutError::UnsupportedComputedValue {
                    node,
                    property: "grid-template-columns",
                    value: element
                        .properties
                        .get("grid-template-columns")
                        .cloned()
                        .unwrap_or_else(|| "<계산값 없음>".to_owned()),
                }
            })?;
            grid_styles.insert(node, grid);
            grid_containers.insert(node, project_grid_layout(node, grid, snapshot.profile)?);
        }
        if supports_positioning(snapshot.profile) {
            positioning.insert(
                node,
                parse_positioning(
                    element,
                    &mut math,
                    snapshot.profile == ComputedStyleProfile::RuntimeBlockPositioningV1
                        || snapshot.profile == ComputedStyleProfile::RuntimeBlockFlexStackingV1
                        || supports_overflow_clip(snapshot.profile)
                        || snapshot.profile == ComputedStyleProfile::RuntimeBlockFlexGridV1
                        || snapshot.profile
                            == ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
                        || snapshot.profile
                            == ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
                        || snapshot.profile
                            == ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
                        || snapshot.profile == ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
                        || supports_runtime_flex,
                    snapshot.profile == ComputedStyleProfile::RuntimeBlockPositioningV1
                        || snapshot.profile == ComputedStyleProfile::RuntimeBlockFlexStackingV1
                        || supports_overflow_clip(snapshot.profile)
                        || snapshot.profile == ComputedStyleProfile::RuntimeBlockFlexGridV1
                        || snapshot.profile
                            == ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
                        || snapshot.profile
                            == ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
                        || snapshot.profile
                            == ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
                        || snapshot.profile == ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
                )?,
            );
        }
        if output.insert(node, style).is_some() {
            return Err(StyleLayoutError::DuplicateComputedElement(node));
        }
    }
    if matches!(
        snapshot.profile,
        ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
    ) {
        let grid_item_nodes = grid_nodes
            .as_ref()
            .map(|nodes| &nodes.grid_items)
            .expect("placement profile has a Grid tree context");
        for element in snapshot
            .elements
            .iter()
            .filter(|element| grid_item_nodes.contains(&element.node_id))
        {
            let placement = element.grid_item_placement.as_ref().ok_or(
                crate::StyleLayoutError::MissingComputedProperty {
                    node: element.node_id,
                    property: "grid-row-start",
                },
            )?;
            let owner = grid_nodes
                .as_ref()
                .and_then(|nodes| nodes.grid_item_owners.get(&element.node_id))
                .copied()
                .ok_or(crate::StyleLayoutError::MissingComputedElement(
                    element.node_id,
                ))?;
            let container = grid_containers
                .get(&owner)
                .ok_or(crate::StyleLayoutError::MissingComputedElement(owner))?;
            let grid_style = grid_styles
                .get(&owner)
                .ok_or(crate::StyleLayoutError::MissingComputedElement(owner))?;
            if let Some(placement) = project_grid_item_placement(
                element.node_id,
                placement,
                container,
                grid_style.template_areas.as_ref(),
            )? {
                grid_items.insert(element.node_id, placement);
            }
        }
    }
    Ok(ProjectedStyles {
        styles: output,
        positioning,
        grid_containers,
        grid_items,
        css_math: math.into_values(),
    })
}

#[derive(Default)]
struct RuntimeGridNodes {
    hidden: BTreeSet<NodeId>,
    grid_containers: BTreeSet<NodeId>,
    grid_items: BTreeSet<NodeId>,
    grid_item_owners: BTreeMap<NodeId, NodeId>,
}

fn is_grid_profile(profile: ComputedStyleProfile) -> bool {
    matches!(
        profile,
        ComputedStyleProfile::RuntimeBlockFlexGridV1
            | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
    )
}

fn runtime_grid_nodes(
    tree: &HostDocumentSnapshot,
    root: HostNodeHandle,
    snapshot: &ComputedStyleSnapshot,
) -> RuntimeGridNodes {
    let displays = snapshot
        .elements
        .iter()
        .filter_map(|element| {
            element
                .properties
                .get("display")
                .map(|display| (element.node_id, display.as_str()))
        })
        .collect::<BTreeMap<_, _>>();
    let mut nodes = RuntimeGridNodes::default();
    let mut pending = vec![(root, false)];
    while let Some((parent, ancestor_hidden)) = pending.pop() {
        let display = displays.get(&parent.id()).copied().unwrap_or("block");
        let hidden = ancestor_hidden || display == "none";
        if hidden {
            nodes.hidden.insert(parent.id());
        } else if display == "grid" {
            nodes.grid_containers.insert(parent.id());
        }
        let Some(children) = tree.children(parent) else {
            continue;
        };
        for child in children {
            if !hidden
                && display == "grid"
                && displays
                    .get(&child.id())
                    .is_none_or(|display| *display != "none")
            {
                nodes.grid_items.insert(child.id());
                nodes.grid_item_owners.insert(child.id(), parent.id());
            }
            pending.push((child, hidden));
        }
    }
    nodes
}

fn reject_grid_alignment(
    element: &ComputedElementStyle,
    is_grid_container: bool,
    is_grid_item: bool,
) -> Result<(), StyleLayoutError> {
    let mut unsupported = Vec::new();
    if is_grid_container {
        unsupported.extend([
            ("align-items", "normal"),
            ("align-content", "normal"),
            ("justify-content", "normal"),
            ("justify-items", "normal"),
        ]);
    }
    if is_grid_item {
        unsupported.extend([("align-self", "auto"), ("justify-self", "auto")]);
    }
    for (property, initial) in unsupported {
        if let Some(value) = element.properties.get(property)
            && value != initial
        {
            return Err(StyleLayoutError::UnsupportedComputedValue {
                node: element.node_id,
                property,
                value: value.clone(),
            });
        }
    }
    Ok(())
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
            | ComputedStyleProfile::RuntimeBlockFlexStackingV1
            | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
            | ComputedStyleProfile::RuntimeBlockFlexGridV1
            | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
    )
}

fn supports_overflow_clip(profile: ComputedStyleProfile) -> bool {
    profile == ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
}

fn parse_overflow(
    node: NodeId,
    property: &'static str,
    value: Option<ComputedCssOverflow>,
) -> Result<LayoutOverflow, StyleLayoutError> {
    let value = value.ok_or(StyleLayoutError::MissingComputedProperty { node, property })?;
    Ok(match value {
        ComputedCssOverflow::Visible => LayoutOverflow::Visible,
        ComputedCssOverflow::Clip => LayoutOverflow::Clip,
        ComputedCssOverflow::Hidden | ComputedCssOverflow::Scroll | ComputedCssOverflow::Auto => {
            LayoutOverflow::Hidden
        }
    })
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
            | ComputedStyleProfile::RuntimeBlockFlexStackingV1
            | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
            | ComputedStyleProfile::RuntimeBlockFlexGridV1
            | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
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
