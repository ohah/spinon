use std::collections::BTreeMap;

use spinon_core::NodeId;
use taffy::prelude::{
    FromFr, FromLength, FromPercent, GridAutoFlow as TaffyGridAutoFlow,
    GridTemplateComponent as TaffyGridTemplateComponent, LengthPercentage, MaxTrackSizingFunction,
    MinTrackSizingFunction, TrackSizingFunction,
};
use taffy::style::{GridTemplateRepetition, RepetitionCount};
use taffy::style_helpers::{
    TaffyAuto, TaffyFitContent, TaffyGridLine, TaffyMaxContent, TaffyMinContent,
};

use crate::{
    GridAutoFlow, GridAutoRepeatMode, GridContainerInput, GridPlacementInput,
    GridTrackLengthPercentage, GridTrackMaxSizingFunction, GridTrackMinSizingFunction,
    GridTrackSize, LayoutCalcId, LayoutCssMathProperty, LayoutError, LayoutLengthPercentage,
};

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
    calc_handles: &BTreeMap<LayoutCalcId, *const ()>,
) -> Result<Vec<TaffyGridTemplateComponent<String>>, LayoutError> {
    to_taffy_grid_tracks_with_auto_repeat(node, property, tracks, None, calc_handles)
}

pub(super) fn to_taffy_grid_tracks_with_auto_repeat(
    node: NodeId,
    property: &'static str,
    tracks: &[GridTrackSize],
    auto_repeat: Option<GridAutoRepeatMode>,
    calc_handles: &BTreeMap<LayoutCalcId, *const ()>,
) -> Result<Vec<TaffyGridTemplateComponent<String>>, LayoutError> {
    let mut output = Vec::new();
    if let Some(auto_repeat) = auto_repeat {
        if tracks.is_empty() {
            return Err(LayoutError::InvalidGridTrackList { node, property });
        }
        let mut repeated_tracks = Vec::new();
        repeated_tracks
            .try_reserve_exact(tracks.len())
            .map_err(|_| LayoutError::GridAllocationFailure { node, property })?;
        for track in tracks {
            repeated_tracks.push(to_taffy_track_sizing_function(
                node,
                property,
                *track,
                calc_handles,
            )?);
        }
        output
            .try_reserve_exact(1)
            .map_err(|_| LayoutError::GridAllocationFailure { node, property })?;
        output.push(TaffyGridTemplateComponent::Repeat(GridTemplateRepetition {
            count: match auto_repeat {
                GridAutoRepeatMode::AutoFill => RepetitionCount::AutoFill,
                GridAutoRepeatMode::AutoFit => RepetitionCount::AutoFit,
            },
            tracks: repeated_tracks,
            line_names: Vec::new(),
        }));
    } else {
        output
            .try_reserve(tracks.len())
            .map_err(|_| LayoutError::GridAllocationFailure { node, property })?;
        for track in tracks {
            let sizing = to_taffy_track_sizing_function(node, property, *track, calc_handles)?;
            output.push(TaffyGridTemplateComponent::Single(sizing));
        }
    }
    Ok(output)
}

fn to_taffy_track_sizing_function(
    node: NodeId,
    property: &'static str,
    track: GridTrackSize,
    calc_handles: &BTreeMap<LayoutCalcId, *const ()>,
) -> Result<TrackSizingFunction, LayoutError> {
    let math_property = grid_math_property(property);
    let sizing = match track {
        GridTrackSize::LengthPx(value) => TrackSizingFunction::from_length(value),
        GridTrackSize::Percentage(value) => TrackSizingFunction::from_percent(value),
        GridTrackSize::Fraction(value) => TrackSizingFunction::from_fr(value),
        GridTrackSize::Calc(id) => {
            let value =
                LengthPercentage::calc(super::calc_handle(node, math_property, id, calc_handles)?);
            value.into()
        }
        GridTrackSize::Auto => TrackSizingFunction::AUTO,
        GridTrackSize::MinContent => TrackSizingFunction::MIN_CONTENT,
        GridTrackSize::MaxContent => TrackSizingFunction::MAX_CONTENT,
        GridTrackSize::MinMax { min, max } => TrackSizingFunction {
            min: to_taffy_min_sizing_function(node, math_property, min, calc_handles)?,
            max: to_taffy_max_sizing_function(node, math_property, max, calc_handles)?,
        },
        GridTrackSize::FitContent(value) => TrackSizingFunction::fit_content(
            to_taffy_length_percentage(node, math_property, value, calc_handles)?,
        ),
    };
    Ok(sizing)
}

fn to_taffy_min_sizing_function(
    node: NodeId,
    property: LayoutCssMathProperty,
    value: GridTrackMinSizingFunction,
    calc_handles: &BTreeMap<LayoutCalcId, *const ()>,
) -> Result<MinTrackSizingFunction, LayoutError> {
    Ok(match value {
        GridTrackMinSizingFunction::LengthPx(value) => MinTrackSizingFunction::length(value),
        GridTrackMinSizingFunction::Percentage(value) => MinTrackSizingFunction::percent(value),
        GridTrackMinSizingFunction::Calc(id) => {
            MinTrackSizingFunction::calc(super::calc_handle(node, property, id, calc_handles)?)
        }
        GridTrackMinSizingFunction::Auto => MinTrackSizingFunction::AUTO,
        GridTrackMinSizingFunction::MinContent => MinTrackSizingFunction::MIN_CONTENT,
        GridTrackMinSizingFunction::MaxContent => MinTrackSizingFunction::MAX_CONTENT,
    })
}

fn to_taffy_max_sizing_function(
    node: NodeId,
    property: LayoutCssMathProperty,
    value: GridTrackMaxSizingFunction,
    calc_handles: &BTreeMap<LayoutCalcId, *const ()>,
) -> Result<MaxTrackSizingFunction, LayoutError> {
    Ok(match value {
        GridTrackMaxSizingFunction::LengthPx(value) => MaxTrackSizingFunction::length(value),
        GridTrackMaxSizingFunction::Percentage(value) => MaxTrackSizingFunction::percent(value),
        GridTrackMaxSizingFunction::Calc(id) => {
            MaxTrackSizingFunction::calc(super::calc_handle(node, property, id, calc_handles)?)
        }
        GridTrackMaxSizingFunction::Fraction(value) => MaxTrackSizingFunction::from_fr(value),
        GridTrackMaxSizingFunction::Auto => MaxTrackSizingFunction::AUTO,
        GridTrackMaxSizingFunction::MinContent => MaxTrackSizingFunction::MIN_CONTENT,
        GridTrackMaxSizingFunction::MaxContent => MaxTrackSizingFunction::MAX_CONTENT,
    })
}

fn to_taffy_length_percentage(
    node: NodeId,
    property: LayoutCssMathProperty,
    value: GridTrackLengthPercentage,
    calc_handles: &BTreeMap<LayoutCalcId, *const ()>,
) -> Result<LengthPercentage, LayoutError> {
    let value = match value {
        GridTrackLengthPercentage::LengthPx(value) => LayoutLengthPercentage::LengthPx(value),
        GridTrackLengthPercentage::Percentage(value) => LayoutLengthPercentage::Percentage(value),
        GridTrackLengthPercentage::Calc(id) => LayoutLengthPercentage::Calc(id),
    };
    super::to_taffy_length_percentage(node, property, value, calc_handles)
}

fn grid_math_property(property: &'static str) -> LayoutCssMathProperty {
    match property {
        "grid-template-columns" => LayoutCssMathProperty::GridTemplateColumns,
        "grid-template-rows" => LayoutCssMathProperty::GridTemplateRows,
        _ => unreachable!("Taffy Grid tracks have a fixed columns/rows property name"),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use spinon_core::NodeId;
    use taffy::prelude::FromFr;
    use taffy::style_helpers::{TaffyAuto, TaffyFitContent, TaffyMaxContent, TaffyMinContent};

    use super::{TaffyGridTemplateComponent, TrackSizingFunction, to_taffy_grid_tracks};
    use crate::{
        GridTrackLengthPercentage, GridTrackMaxSizingFunction, GridTrackMinSizingFunction,
        GridTrackSize,
    };

    #[test]
    fn intrinsic_track_functions_preserve_typed_min_and_max_functions() {
        let node = NodeId::new(11_404).unwrap();
        let tracks = [
            GridTrackSize::Auto,
            GridTrackSize::MinContent,
            GridTrackSize::MaxContent,
            GridTrackSize::MinMax {
                min: GridTrackMinSizingFunction::LengthPx(30.0),
                max: GridTrackMaxSizingFunction::Fraction(2.0),
            },
            GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(80.0)),
        ];

        let converted =
            to_taffy_grid_tracks(node, "grid-template-columns", &tracks, &BTreeMap::new()).unwrap();
        let converted = converted
            .into_iter()
            .map(|component| match component {
                TaffyGridTemplateComponent::Single(sizing) => sizing,
                TaffyGridTemplateComponent::Repeat(_) => {
                    panic!("a single input track must not introduce repetition")
                }
            })
            .collect::<Vec<_>>();

        assert_eq!(converted[0], TrackSizingFunction::AUTO);
        assert_eq!(converted[1], TrackSizingFunction::MIN_CONTENT);
        assert_eq!(converted[2], TrackSizingFunction::MAX_CONTENT);
        assert_eq!(
            converted[3],
            TrackSizingFunction {
                min: taffy::style::MinTrackSizingFunction::length(30.0),
                max: taffy::style::MaxTrackSizingFunction::from_fr(2.0),
            }
        );
        assert_eq!(
            converted[4],
            TrackSizingFunction::fit_content(taffy::prelude::LengthPercentage::length(80.0))
        );
    }
}
