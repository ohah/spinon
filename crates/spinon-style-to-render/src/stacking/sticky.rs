use std::collections::BTreeMap;

use spinon_core::NodeId;
use spinon_render::{RuntimeScrollContainer, RuntimeStickyConstraint};
use spinon_style::{
    ComputedCssOverflow, ComputedCssPosition, ComputedCssSpacingValue, ComputedElementStyle,
};
use spinon_style_to_layout::StyleLayoutOutput;

use super::reserve::FallibleReserve;
use super::scroll::scroll_ancestors_for_node;
use crate::StyleRenderError;

pub(super) struct StickyLayoutContext<'a> {
    pub parent_by_node: &'a BTreeMap<NodeId, NodeId>,
    pub styles: &'a BTreeMap<NodeId, &'a ComputedElementStyle>,
    pub output: &'a StyleLayoutOutput,
    pub scroll_containers: &'a [RuntimeScrollContainer],
    pub scroll_offsets: &'a BTreeMap<NodeId, (f32, f32)>,
    pub scroll_ancestors: &'a [NodeId],
}

pub(super) fn sticky_constraint_for_node<R: FallibleReserve>(
    node: NodeId,
    style: &ComputedElementStyle,
    context: &StickyLayoutContext<'_>,
    reserve: &mut R,
) -> Result<RuntimeStickyConstraint, StyleRenderError> {
    if context.output.computed_styles.profile
        != spinon_style::ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
    {
        return Err(unsupported(
            node,
            "position",
            "sticky outside C12.5 fixture profile",
        ));
    }
    if style.layout_position != ComputedCssPosition::Sticky {
        return Err(unsupported(node, "position", "expected sticky"));
    }
    let top = match style.layout_insets.top {
        ComputedCssSpacingValue::LengthPx(top)
            if top.is_finite() && top >= 0.0 && !style.layout_math_values.contains_key("top") =>
        {
            top
        }
        _ => return Err(unsupported(node, "top", computed(style, "top"))),
    };
    for (property, value) in [
        ("right", style.layout_insets.right),
        ("bottom", style.layout_insets.bottom),
        ("left", style.layout_insets.left),
    ] {
        if value != ComputedCssSpacingValue::Auto || style.layout_math_values.contains_key(property)
        {
            return Err(unsupported(node, property, computed(style, property)));
        }
    }

    let margin = style.layout_spacing.margin;
    let padding = style.layout_spacing.padding;
    if !is_zero(margin.top)
        || !is_zero(margin.right)
        || !is_zero(margin.bottom)
        || !is_zero(margin.left)
        || !is_zero(padding.top)
        || !is_zero(padding.right)
        || !is_zero(padding.bottom)
        || !is_zero(padding.left)
        || style.layout_border.top != 0.0
        || style.layout_border.right != 0.0
        || style.layout_border.bottom != 0.0
        || style.layout_border.left != 0.0
    {
        return Err(unsupported(
            node,
            "margin/padding/border",
            "nonzero sticky box edges",
        ));
    }
    if style
        .properties
        .get("direction")
        .is_some_and(|value| value != "ltr")
        || style
            .properties
            .get("writing-mode")
            .is_some_and(|value| value != "horizontal-tb")
    {
        return Err(unsupported(
            node,
            "writing-mode/direction",
            "non-horizontal-tb or rtl",
        ));
    }

    let parent = context
        .parent_by_node
        .get(&node)
        .copied()
        .ok_or(StyleRenderError::InvalidRoot)?;
    let parent_style = context
        .styles
        .get(&parent)
        .copied()
        .ok_or(StyleRenderError::MissingComputedStyle(parent))?;
    if !matches!(
        parent_style.layout_position,
        ComputedCssPosition::Static | ComputedCssPosition::Relative
    ) || !matches!(
        parent_style.properties.get("display").map(String::as_str),
        Some("block" | "flow-root")
    ) {
        return Err(unsupported(
            node,
            "containing block",
            "parent must be static/relative Block",
        ));
    }

    let mut cursor = Some(parent);
    let mut nearest_owner = None;
    let mut visited = 0_usize;
    while let Some(ancestor) = cursor {
        visited = visited
            .checked_add(1)
            .ok_or(StyleRenderError::InvalidRoot)?;
        if visited > context.parent_by_node.len().saturating_add(1) {
            return Err(StyleRenderError::InvalidRoot);
        }
        let ancestor_style = context
            .styles
            .get(&ancestor)
            .copied()
            .ok_or(StyleRenderError::MissingComputedStyle(ancestor))?;
        if !ancestor_style.fixed_containing_block_effects.is_empty() {
            return Err(unsupported(
                node,
                "ancestor containing-block effect",
                format!("ancestor {ancestor}"),
            ));
        }
        let overflow_y =
            ancestor_style
                .layout_overflow_y
                .ok_or(StyleRenderError::MissingComputedProperty {
                    node: ancestor,
                    property: "overflow-y",
                })?;
        match overflow_y {
            ComputedCssOverflow::Visible => {}
            ComputedCssOverflow::Auto | ComputedCssOverflow::Scroll => {
                nearest_owner = Some(ancestor);
                break;
            }
            ComputedCssOverflow::Hidden | ComputedCssOverflow::Clip => {
                return Err(unsupported(
                    node,
                    "nearest scroll ancestor overflow-y",
                    overflow_name(overflow_y),
                ));
            }
        }
        cursor = context.parent_by_node.get(&ancestor).copied();
    }
    let nearest_owner = nearest_owner
        .ok_or_else(|| unsupported(node, "nearest scroll owner", "no auto/scroll ancestor"))?;
    if context.scroll_ancestors.first().copied() != Some(nearest_owner) {
        return Err(unsupported(
            node,
            "nearest scroll owner",
            "scroll ancestry identity mismatch",
        ));
    }
    let owner = context
        .scroll_containers
        .iter()
        .find(|container| container.owner() == nearest_owner)
        .ok_or_else(|| unsupported(node, "nearest scroll owner", "snapshot owner missing"))?;
    if owner.max_offset_y_css_px() <= 0.0 {
        return Err(unsupported(
            node,
            "nearest scroll owner range",
            owner.max_offset_y_css_px().to_string(),
        ));
    }
    let used_edges = context
        .output
        .layout
        .used_box_edges
        .get(&nearest_owner)
        .copied()
        .ok_or(StyleRenderError::MissingComputedProperty {
            node: nearest_owner,
            property: "used padding and border edges",
        })?;
    let padding_top = used_edges.padding.top;
    if !padding_top.is_finite() || padding_top < 0.0 {
        return Err(unsupported(
            node,
            "scrollport padding-top",
            padding_top.to_string(),
        ));
    }

    let sticky_frame = context
        .output
        .layout
        .frames
        .get(&node)
        .copied()
        .ok_or(StyleRenderError::MissingLayoutFrame(node))?;
    let containing_block_frame = context
        .output
        .layout
        .frames
        .get(&parent)
        .copied()
        .ok_or(StyleRenderError::MissingLayoutFrame(parent))?;
    let containing_block_edges = context
        .output
        .layout
        .used_box_edges
        .get(&parent)
        .copied()
        .ok_or(StyleRenderError::MissingComputedProperty {
            node: parent,
            property: "used padding and border edges",
        })?;
    let content_top_inset = content_edge_inset(
        node,
        "containing block top content edge",
        containing_block_edges.border.top,
        containing_block_edges.padding.top,
    )?;
    let content_bottom_inset = content_edge_inset(
        node,
        "containing block bottom content edge",
        containing_block_edges.border.bottom,
        containing_block_edges.padding.bottom,
    )?;
    let content_height = containing_block_frame.height - content_top_inset - content_bottom_inset;
    if !content_height.is_finite() || content_height <= 0.0 {
        return Err(unsupported(
            node,
            "containing block content height",
            content_height.to_string(),
        ));
    }
    if sticky_frame.height > content_height
        || sticky_frame.height > owner.scrollport_css_px().height()
    {
        return Err(unsupported(
            node,
            "sticky box size",
            "box does not fit scrollport and containing block",
        ));
    }
    let containing_block_scroll_ancestors = scroll_ancestors_for_node(
        parent,
        parent_style,
        context.parent_by_node,
        context.scroll_offsets,
        reserve,
    )?;
    RuntimeStickyConstraint::new(
        nearest_owner,
        spinon_render::CssRect::new(
            containing_block_frame.x,
            containing_block_frame.y,
            containing_block_frame.width,
            containing_block_frame.height,
        )
        .map_err(|_| StyleRenderError::InvalidFrame(parent))?,
        containing_block_scroll_ancestors,
        content_top_inset,
        content_bottom_inset,
        padding_top,
        top,
    )
    .map_err(Into::into)
}

fn is_zero(value: ComputedCssSpacingValue) -> bool {
    matches!(value, ComputedCssSpacingValue::LengthPx(value) if value == 0.0)
}

fn content_edge_inset(
    node: NodeId,
    property: &'static str,
    border: f32,
    padding: f32,
) -> Result<f32, StyleRenderError> {
    if !border.is_finite() || border < 0.0 || !padding.is_finite() || padding < 0.0 {
        return Err(unsupported(
            node,
            property,
            format!("border={border}, padding={padding}"),
        ));
    }
    let inset = border + padding;
    if !inset.is_finite() {
        return Err(unsupported(node, property, inset.to_string()));
    }
    Ok(inset)
}

fn computed(style: &ComputedElementStyle, property: &'static str) -> String {
    style
        .properties
        .get(property)
        .cloned()
        .unwrap_or_else(|| "<missing computed value>".to_owned())
}

fn overflow_name(value: ComputedCssOverflow) -> &'static str {
    match value {
        ComputedCssOverflow::Visible => "visible",
        ComputedCssOverflow::Clip => "clip",
        ComputedCssOverflow::Hidden => "hidden",
        ComputedCssOverflow::Scroll => "scroll",
        ComputedCssOverflow::Auto => "auto",
    }
}

fn unsupported(node: NodeId, property: &'static str, value: impl Into<String>) -> StyleRenderError {
    StyleRenderError::UnsupportedComputedValue {
        node,
        property,
        value: value.into(),
    }
}
