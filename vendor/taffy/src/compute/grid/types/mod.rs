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
