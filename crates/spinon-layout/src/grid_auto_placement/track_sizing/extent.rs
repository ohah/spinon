use spinon_core::NodeId;

use crate::{LayoutError, grid::GRID_MAX_TRACKS};

#[derive(Clone, Copy)]
pub(super) struct Extent {
    pub(super) start: i64,
    pub(super) end: i64,
}

impl Extent {
    pub(super) fn explicit(
        node: NodeId,
        property: &'static str,
        track_count: usize,
    ) -> Result<Self, LayoutError> {
        validate_count(node, property, track_count)?;
        let end = i64::try_from(track_count)
            .map_err(|_| LayoutError::GridTrackExpansionOverflow(node))?;
        Ok(Self { start: 0, end })
    }

    pub(super) fn include(&mut self, start: i64, end: i64) {
        self.start = self.start.min(start);
        self.end = self.end.max(end);
    }

    pub(super) fn include_end(
        &mut self,
        node: NodeId,
        property: &'static str,
        end: i64,
    ) -> Result<(), LayoutError> {
        self.end = self.end.max(end);
        self.count(node, property).map(|_| ())
    }

    pub(super) fn count(self, node: NodeId, property: &'static str) -> Result<usize, LayoutError> {
        let count = self
            .end
            .checked_sub(self.start)
            .and_then(|count| usize::try_from(count).ok())
            .ok_or(LayoutError::GridTrackExpansionOverflow(node))?;
        validate_count(node, property, count)?;
        Ok(count)
    }

    pub(super) fn ensure_count(
        &mut self,
        node: NodeId,
        property: &'static str,
        minimum: usize,
    ) -> Result<(), LayoutError> {
        let minimum =
            i64::try_from(minimum).map_err(|_| LayoutError::GridTrackExpansionOverflow(node))?;
        let minimum_end = self
            .start
            .checked_add(minimum)
            .ok_or(LayoutError::GridTrackExpansionOverflow(node))?;
        self.end = self.end.max(minimum_end);
        self.count(node, property).map(|_| ())
    }

    pub(super) fn leading_count(self, node: NodeId) -> Result<usize, LayoutError> {
        let count = self
            .start
            .checked_neg()
            .ok_or(LayoutError::GridTrackExpansionOverflow(node))?;
        usize::try_from(count).map_err(|_| LayoutError::GridTrackExpansionOverflow(node))
    }

    pub(super) fn trailing_count(
        self,
        node: NodeId,
        explicit_count: usize,
    ) -> Result<usize, LayoutError> {
        let explicit_end = i64::try_from(explicit_count)
            .map_err(|_| LayoutError::GridTrackExpansionOverflow(node))?;
        let count = self
            .end
            .checked_sub(explicit_end)
            .ok_or(LayoutError::GridTrackExpansionOverflow(node))?;
        usize::try_from(count).map_err(|_| LayoutError::GridTrackExpansionOverflow(node))
    }

    pub(super) fn origin_shift(self, node: NodeId) -> Result<i64, LayoutError> {
        self.start
            .checked_neg()
            .ok_or(LayoutError::GridTrackExpansionOverflow(node))
    }
}

pub(super) fn raw_track_end(node: NodeId, start: i64, span: usize) -> Result<i64, LayoutError> {
    let span = i64::try_from(span).map_err(|_| LayoutError::GridTrackExpansionOverflow(node))?;
    start
        .checked_add(span)
        .ok_or(LayoutError::GridTrackExpansionOverflow(node))
}

pub(super) fn track_end(
    node: NodeId,
    property: &'static str,
    start: i64,
    span: usize,
    origin: i64,
) -> Result<i64, LayoutError> {
    if start < origin {
        return Err(LayoutError::GridTrackExpansionOverflow(node));
    }
    let end = raw_track_end(node, start, span)?;
    let count = end
        .checked_sub(origin)
        .and_then(|count| usize::try_from(count).ok())
        .ok_or(LayoutError::GridTrackExpansionOverflow(node))?;
    validate_count(node, property, count)?;
    Ok(end)
}

pub(super) fn coordinate_index(
    node: NodeId,
    coordinate: i64,
    origin: i64,
) -> Result<usize, LayoutError> {
    coordinate
        .checked_sub(origin)
        .and_then(|offset| usize::try_from(offset).ok())
        .ok_or(LayoutError::GridTrackExpansionOverflow(node))
}

pub(super) fn next_coordinate(node: NodeId, coordinate: i64) -> Result<i64, LayoutError> {
    coordinate
        .checked_add(1)
        .ok_or(LayoutError::GridTrackExpansionOverflow(node))
}

pub(super) fn validate_count(
    node: NodeId,
    property: &'static str,
    count: usize,
) -> Result<(), LayoutError> {
    if count > GRID_MAX_TRACKS {
        return Err(LayoutError::GridTrackLimitExceeded {
            node,
            property,
            count,
            maximum: GRID_MAX_TRACKS,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_signed_extent_accepts_9999_and_10000_tracks_and_rejects_10001() {
        let node = NodeId::new(1).unwrap();
        for expected_count in [9_999_usize, 10_000] {
            let mut extent = Extent::explicit(node, "grid-template-columns", 1).unwrap();
            let start = 1_i64 - i64::try_from(expected_count).unwrap();
            extent.include(start, 1);
            assert_eq!(
                extent.count(node, "grid-template-columns").unwrap(),
                expected_count
            );
        }

        let mut over_limit = Extent::explicit(node, "grid-template-columns", 1).unwrap();
        over_limit.include(1_i64 - 10_001, 1);
        assert!(matches!(
            over_limit.count(node, "grid-template-columns"),
            Err(LayoutError::GridTrackLimitExceeded {
                node: observed_node,
                property: "grid-template-columns",
                count: 10_001,
                maximum: 10_000,
            }) if observed_node == node
        ));
    }
}
