//! Structs and enums that are used within the grid module
mod cell_occupancy;
mod coordinates;
mod grid_item;
mod grid_track;
mod grid_track_counts;
mod named;

use crate::geometry::Size;
use crate::BoxSizing;

/// Applies Grid's preferred ratio in the box selected by `box-sizing`.
/// The input and output dimensions are border-box dimensions, while a content-box ratio must
/// exclude padding and border before conversion and restore them afterwards.
pub(super) fn maybe_apply_aspect_ratio_with_box_sizing(
    dimensions: Size<Option<f32>>,
    aspect_ratio: Option<f32>,
    box_sizing: BoxSizing,
    padding_border_size: Size<f32>,
) -> Size<Option<f32>> {
    if box_sizing != BoxSizing::ContentBox || aspect_ratio.is_none() {
        return dimensions.maybe_apply_aspect_ratio(aspect_ratio);
    }

    let content_dimensions = Size {
        width: dimensions
            .width
            .map(|width| (width - padding_border_size.width).max(0.0)),
        height: dimensions
            .height
            .map(|height| (height - padding_border_size.height).max(0.0)),
    }
    .maybe_apply_aspect_ratio(aspect_ratio);

    Size {
        width: content_dimensions
            .width
            .map(|width| width + padding_border_size.width),
        height: content_dimensions
            .height
            .map(|height| height + padding_border_size.height),
    }
}

/// Transfers a min/max constraint through an aspect ratio only when its opposite preferred axis is auto.
pub(super) fn maybe_apply_aspect_ratio_to_constraint_size(
    constraints: Size<Option<f32>>,
    preferred_size: Size<Option<f32>>,
    aspect_ratio: Option<f32>,
) -> Size<Option<f32>> {
    match (preferred_size.width, preferred_size.height) {
        (Some(_), Some(_)) => constraints,
        (Some(_), None) if constraints.width.is_some() && constraints.height.is_none() => {
            constraints.maybe_apply_aspect_ratio(aspect_ratio)
        }
        (None, Some(_)) if constraints.width.is_none() && constraints.height.is_some() => {
            constraints.maybe_apply_aspect_ratio(aspect_ratio)
        }
        (None, None) => constraints.maybe_apply_aspect_ratio(aspect_ratio),
        _ => constraints,
    }
}

/// Transfers Grid item's min/max constraints through a preferred ratio and applies the
/// cross-axis precedence rules from CSS Sizing: transferred minimums are capped by a direct
/// maximum in the destination axis, and transferred maximums are floored by the effective
/// minimum there. Direct min/max conflicts remain resolved by the usual min-wins clamp.
pub(super) fn maybe_apply_aspect_ratio_to_min_max_constraints(
    minimum: Size<Option<f32>>,
    maximum: Size<Option<f32>>,
    preferred_size: Size<Option<f32>>,
    aspect_ratio: Option<f32>,
) -> (Size<Option<f32>>, Size<Option<f32>>) {
    let minimum_from_width = minimum.width.is_some() && minimum.height.is_none();
    let minimum_from_height = minimum.width.is_none() && minimum.height.is_some();
    let maximum_from_width = maximum.width.is_some() && maximum.height.is_none();
    let maximum_from_height = maximum.width.is_none() && maximum.height.is_some();
    let direct_maximum = maximum;
    let mut minimum =
        maybe_apply_aspect_ratio_to_constraint_size(minimum, preferred_size, aspect_ratio);
    let mut maximum =
        maybe_apply_aspect_ratio_to_constraint_size(maximum, preferred_size, aspect_ratio);

    if preferred_size.width.is_none() && preferred_size.height.is_none() {
        // Transfer a minimum only into an automatic opposite-axis minimum. It cannot exceed a
        // direct maximum in that destination axis.
        if minimum_from_width {
            if let Some(destination_max) = direct_maximum.height {
                minimum.height = minimum.height.map(|value| value.min(destination_max));
            }
        }
        if minimum_from_height {
            if let Some(destination_max) = direct_maximum.width {
                minimum.width = minimum.width.map(|value| value.min(destination_max));
            }
        }

        // A transferred maximum cannot undercut an explicit or transferred minimum in its
        // destination axis. Keep the source-axis min/max conflict for the ordinary min-wins
        // clamp below.
        if maximum_from_width {
            if let (Some(transferred_maximum), Some(minimum)) = (maximum.height, minimum.height) {
                maximum.height = Some(transferred_maximum.max(minimum));
            }
        }
        if maximum_from_height {
            if let (Some(transferred_maximum), Some(minimum)) = (maximum.width, minimum.width) {
                maximum.width = Some(transferred_maximum.max(minimum));
            }
        }
    }

    (minimum, maximum)
}

// Publish only locally in the grid module
pub(super) use cell_occupancy::{CellOccupancyMatrix, CellOccupancyState};
pub(crate) use coordinates::{
    GridCoordinate, GridLine, OriginZeroLine, MAX_GRID_TRACKS, MAX_OZ_LINE, MIN_OZ_LINE,
};
pub(super) use grid_item::GridItem;
pub(super) use grid_track::GridTrack;
pub(super) use grid_track_counts::TrackCounts;
pub(super) use named::{GridAreasMap, NamedLineResolver};
pub use named::{GridLineNames, GridLineNamesIter};

#[allow(unused_imports)]
pub(super) use grid_track::GridTrackKind;
