use std::collections::BTreeMap;

use spinon_core::NodeId;
use spinon_layout::PositionedContainingBlockOwner;
use spinon_render::{CssAxisClip, CssClip};
use spinon_style::{
    ComputedCssOverflow, ComputedCssOverflowClipMargin, ComputedCssOverflowVisualBox,
    ComputedCssPosition,
};
use spinon_style_to_layout::StyleLayoutOutput;

use super::reserve::FallibleReserve;
use crate::StyleRenderError;

pub(super) fn validate_computed_overflow_values(
    styles: &BTreeMap<NodeId, &spinon_style::ComputedElementStyle>,
) -> Result<(), StyleRenderError> {
    for (node, style) in styles {
        if style.layout_overflow_x.is_none() {
            return Err(StyleRenderError::MissingComputedProperty {
                node: *node,
                property: "overflow-x",
            });
        }
        if style.layout_overflow_y.is_none() {
            return Err(StyleRenderError::MissingComputedProperty {
                node: *node,
                property: "overflow-y",
            });
        }
        let margin =
            style
                .overflow_clip_margin
                .ok_or(StyleRenderError::MissingComputedProperty {
                    node: *node,
                    property: "overflow-clip-margin",
                })?;
        if !margin.offset_css_px.is_finite() || margin.offset_css_px < 0.0 {
            return Err(StyleRenderError::UnsupportedComputedValue {
                node: *node,
                property: "overflow-clip-margin",
                value: margin.offset_css_px.to_string(),
            });
        }
    }
    Ok(())
}

pub(super) fn runtime_clips_for_node<R: FallibleReserve>(
    node: NodeId,
    parent_by_node: &BTreeMap<NodeId, NodeId>,
    styles: &BTreeMap<NodeId, &spinon_style::ComputedElementStyle>,
    output: &StyleLayoutOutput,
    reserve: &mut R,
) -> Result<(CssClip, CssClip), StyleRenderError> {
    let mut lineage = Vec::new();
    let mut cursor = Some(node);
    while let Some(current) = cursor {
        reserve.reserve(&mut lineage, 1, "overflow clip ancestry")?;
        lineage.push(current);
        cursor = parent_by_node.get(&current).copied();
    }

    let mut paint_clip = CssClip::unconstrained();
    let mut hit_test_clip = CssClip::unconstrained();
    for (clip_index, clip_node) in lineage.iter().copied().enumerate().skip(1) {
        let mut excluded_by_positioned_descendant = false;
        for descendant in lineage[..clip_index].iter().copied() {
            let style = styles
                .get(&descendant)
                .copied()
                .ok_or(StyleRenderError::MissingComputedStyle(descendant))?;
            match style.layout_position {
                ComputedCssPosition::Absolute => {
                    let owner = output.layout.positioned_owners.get(&descendant);
                    let owner_inside_clip = match owner {
                        Some(PositionedContainingBlockOwner::Node(owner)) => {
                            lineage[..=clip_index].contains(owner)
                        }
                        Some(
                            PositionedContainingBlockOwner::Viewport
                            | PositionedContainingBlockOwner::NoBox,
                        )
                        | None => false,
                    };
                    if !owner_inside_clip {
                        excluded_by_positioned_descendant = true;
                        break;
                    }
                }
                ComputedCssPosition::Fixed => {
                    // C12 fixed containing blocks are viewport-only, so no element clip owns it.
                    excluded_by_positioned_descendant = true;
                    break;
                }
                _ => {}
            }
        }
        if excluded_by_positioned_descendant {
            continue;
        }
        let clip_style = styles
            .get(&clip_node)
            .copied()
            .ok_or(StyleRenderError::MissingComputedStyle(clip_node))?;
        let overflow_x =
            clip_style
                .layout_overflow_x
                .ok_or(StyleRenderError::MissingComputedProperty {
                    node: clip_node,
                    property: "overflow-x",
                })?;
        let overflow_y =
            clip_style
                .layout_overflow_y
                .ok_or(StyleRenderError::MissingComputedProperty {
                    node: clip_node,
                    property: "overflow-y",
                })?;
        if overflow_x == ComputedCssOverflow::Visible && overflow_y == ComputedCssOverflow::Visible
        {
            continue;
        }
        let frame = output
            .layout
            .frames
            .get(&clip_node)
            .copied()
            .ok_or(StyleRenderError::MissingLayoutFrame(clip_node))?;
        let edges = output
            .layout
            .used_box_edges
            .get(&clip_node)
            .copied()
            .ok_or(StyleRenderError::MissingComputedProperty {
                node: clip_node,
                property: "used padding and border edges",
            })?;
        let margin =
            clip_style
                .overflow_clip_margin
                .ok_or(StyleRenderError::MissingComputedProperty {
                    node: clip_node,
                    property: "overflow-clip-margin",
                })?;
        let border_box = [
            frame.x,
            frame.y,
            frame.x + frame.width,
            frame.y + frame.height,
        ];
        let padding_box = [
            frame.x + edges.border.left,
            frame.y + edges.border.top,
            frame.x + frame.width - edges.border.right,
            frame.y + frame.height - edges.border.bottom,
        ];
        let content_box = [
            padding_box[0] + edges.padding.left,
            padding_box[1] + edges.padding.top,
            padding_box[2] - edges.padding.right,
            padding_box[3] - edges.padding.bottom,
        ];
        for value in border_box.into_iter().chain(padding_box).chain(content_box) {
            if !value.is_finite() {
                return Err(StyleRenderError::InvalidFrame(clip_node));
            }
        }
        if overflow_x != ComputedCssOverflow::Visible {
            let (paint_min, paint_max) = clip_axis_bounds(
                overflow_x,
                margin,
                frame.x,
                frame.x + frame.width,
                padding_box[0],
                padding_box[2],
                content_box[0],
                content_box[2],
            );
            let (hit_min, hit_max) = hit_test_clip_axis_bounds(
                overflow_x,
                clip_style.layout_position,
                border_box[0],
                border_box[2],
                paint_min,
                paint_max,
            );
            paint_clip = paint_clip.with_x(CssAxisClip::new(paint_min, paint_max)?);
            hit_test_clip = hit_test_clip.with_x(CssAxisClip::new(hit_min, hit_max)?);
        }
        if overflow_y != ComputedCssOverflow::Visible {
            let (paint_min, paint_max) = clip_axis_bounds(
                overflow_y,
                margin,
                frame.y,
                frame.y + frame.height,
                padding_box[1],
                padding_box[3],
                content_box[1],
                content_box[3],
            );
            let (hit_min, hit_max) = hit_test_clip_axis_bounds(
                overflow_y,
                clip_style.layout_position,
                border_box[1],
                border_box[3],
                paint_min,
                paint_max,
            );
            paint_clip = paint_clip.with_y(CssAxisClip::new(paint_min, paint_max)?);
            hit_test_clip = hit_test_clip.with_y(CssAxisClip::new(hit_min, hit_max)?);
        }
    }
    Ok((paint_clip, hit_test_clip))
}

#[allow(clippy::too_many_arguments)]
fn clip_axis_bounds(
    overflow: ComputedCssOverflow,
    margin: ComputedCssOverflowClipMargin,
    border_min: f32,
    border_max: f32,
    padding_min: f32,
    padding_max: f32,
    content_min: f32,
    content_max: f32,
) -> (f32, f32) {
    if overflow != ComputedCssOverflow::Clip {
        return (padding_min, padding_max);
    }
    let (mut min, mut max) = match margin.visual_box {
        ComputedCssOverflowVisualBox::BorderBox => (border_min, border_max),
        ComputedCssOverflowVisualBox::PaddingBox => (padding_min, padding_max),
        ComputedCssOverflowVisualBox::ContentBox => (content_min, content_max),
    };
    min -= margin.offset_css_px;
    max += margin.offset_css_px;
    (min, max)
}

fn hit_test_clip_axis_bounds(
    overflow: ComputedCssOverflow,
    position: ComputedCssPosition,
    border_min: f32,
    border_max: f32,
    paint_min: f32,
    paint_max: f32,
) -> (f32, f32) {
    if overflow == ComputedCssOverflow::Clip {
        if position == ComputedCssPosition::Static {
            // Chrome clips static descendants at the visual edge while the
            // clip element's own border remains independently hit-testable.
            (paint_min, paint_max)
        } else {
            // Positioned clip elements retain the border box for descendant
            // hit testing and extend it when overflow-clip-margin reaches out.
            (border_min.min(paint_min), border_max.max(paint_max))
        }
    } else {
        (border_min, border_max)
    }
}
