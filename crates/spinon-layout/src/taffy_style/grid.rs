use spinon_core::NodeId;
use taffy::prelude::{
    FromFr, FromLength, FromPercent, GridAutoFlow as TaffyGridAutoFlow,
    GridTemplateComponent as TaffyGridTemplateComponent, TrackSizingFunction,
};
use taffy::style_helpers::TaffyGridLine;

use crate::{GridAutoFlow, GridContainerInput, GridPlacementInput, GridTrackSize, LayoutError};

pub(super) fn to_taffy_grid_auto_flow(container: Option<&GridContainerInput>) -> TaffyGridAutoFlow {
    match container.map(|container| container.auto_flow) {
        Some(GridAutoFlow::Row) | None => TaffyGridAutoFlow::Row,
        Some(GridAutoFlow::Column) => TaffyGridAutoFlow::Column,
        Some(GridAutoFlow::RowDense) => TaffyGridAutoFlow::RowDense,
        Some(GridAutoFlow::ColumnDense) => TaffyGridAutoFlow::ColumnDense,
    }
}

pub(super) fn taffy_grid_placement_line(
    node: NodeId,
    property: &'static str,
    placement: GridPlacementInput,
) -> Result<taffy::style::GridPlacement<String>, LayoutError> {
    match placement {
        GridPlacementInput::Auto => Ok(taffy::style::GridPlacement::Auto),
        GridPlacementInput::Line(line) if line > 0 => {
            let line = i16::try_from(line).map_err(|_| LayoutError::InvalidGridPlacement {
                node,
                property,
                reason: "정규화한 line이 Taffy 범위를 넘었습니다",
            })?;
            Ok(taffy::style::GridPlacement::from_line_index(line))
        }
        GridPlacementInput::Line(_) | GridPlacementInput::Span(_) => {
            Err(LayoutError::InvalidGridPlacement {
                node,
                property,
                reason: "Taffy 입력은 양수 explicit line으로 정규화되어야 합니다",
            })
        }
    }
}

pub(super) fn to_taffy_grid_tracks(
    node: NodeId,
    property: &'static str,
    tracks: &[GridTrackSize],
) -> Result<Vec<TaffyGridTemplateComponent<String>>, LayoutError> {
    let mut output = Vec::new();
    output
        .try_reserve(tracks.len())
        .map_err(|_| LayoutError::GridAllocationFailure { node, property })?;
    for track in tracks {
        let sizing = match track {
            GridTrackSize::LengthPx(value) => TrackSizingFunction::from_length(*value),
            GridTrackSize::Percentage(value) => TrackSizingFunction::from_percent(*value),
            GridTrackSize::Fraction(value) => TrackSizingFunction::from_fr(*value),
        };
        output.push(TaffyGridTemplateComponent::Single(sizing));
    }
    Ok(output)
}
