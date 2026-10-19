//! Structs and enums that are used within the grid module
mod cell_occupancy;
mod coordinates;
mod grid_item;
mod grid_track;
mod grid_track_counts;
mod named;

use crate::geometry::Size;

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
