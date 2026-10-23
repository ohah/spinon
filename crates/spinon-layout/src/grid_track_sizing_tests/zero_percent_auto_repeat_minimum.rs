use super::{grid_input, id};
use crate::{
    GridAutoRepeat, GridAutoRepeatMode, GridPlacementMode, GridTrackMaxSizingFunction,
    GridTrackMinSizingFunction, GridTrackSize, LayoutBorder, LayoutBoxSizing, LayoutDimension,
    LayoutEngine, LayoutGap, LayoutGridTrackSizingAxis, LayoutGridTrackSizingStageKind,
    LayoutInput, LayoutLengthPercentage, RootSizingPolicy, TaffyLayoutEngine, Viewport,
};

fn repeat_input(
    axis: LayoutGridTrackSizingAxis,
    mode: GridAutoRepeatMode,
    minimum: f32,
    gap: f32,
    item_count: usize,
) -> LayoutInput {
    let repeated_track = GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::Percentage(minimum),
        max: GridTrackMaxSizingFunction::Fraction(1.0),
    };
    let (columns, rows) = match axis {
        LayoutGridTrackSizingAxis::Columns => {
            (vec![repeated_track], vec![GridTrackSize::LengthPx(120.0)])
        }
        LayoutGridTrackSizingAxis::Rows => {
            (vec![GridTrackSize::LengthPx(120.0)], vec![repeated_track])
        }
    };
    let gap = LayoutGap {
        column: LayoutLengthPercentage::length(gap),
        row: LayoutLengthPercentage::length(gap),
    };
    let mut input = grid_input(columns, rows, item_count, gap);
    input.nodes[0].style.box_sizing = LayoutBoxSizing::ContentBox;
    input.nodes[0].style.width = LayoutDimension::Fixed(120.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(120.0);
    input.nodes[0].style.min_width = LayoutDimension::Fixed(0.0);
    input.nodes[0].style.min_height = LayoutDimension::Fixed(0.0);
    input.nodes[0].style.border = LayoutBorder {
        top: 2.0,
        right: 2.0,
        bottom: 2.0,
        left: 2.0,
    };
    input.viewport = Viewport {
        width: 120.0,
        height: 120.0,
    };
    input.root_sizing = RootSizingPolicy::Match;
    if item_count > 0 {
        match axis {
            LayoutGridTrackSizingAxis::Columns => {
                input.nodes[1].style.width = LayoutDimension::Auto;
                input.nodes[1].style.height = LayoutDimension::Fixed(10.0);
            }
            LayoutGridTrackSizingAxis::Rows => {
                input.nodes[1].style.width = LayoutDimension::Fixed(20.0);
                input.nodes[1].style.height = LayoutDimension::Auto;
            }
        }
    }
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    let repeat = GridAutoRepeat {
        mode,
        start: 0,
        end: 1,
        line_names: Vec::new(),
    };
    match axis {
        LayoutGridTrackSizingAxis::Columns => container.columns_auto_repeat = Some(repeat),
        LayoutGridTrackSizingAxis::Rows => container.rows_auto_repeat = Some(repeat),
    }
    input
}

fn tracks_at_stage(
    input: &LayoutInput,
    axis: LayoutGridTrackSizingAxis,
    target_stage: LayoutGridTrackSizingStageKind,
) -> Vec<(bool, f32)> {
    let output = TaffyLayoutEngine.compute(input).unwrap();
    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == id(1))
        .unwrap();
    let pass = trace
        .passes
        .iter()
        .rev()
        .find(|pass| pass.axis == axis)
        .unwrap();
    pass.stages
        .iter()
        .find(|stage| stage.stage == target_stage)
        .unwrap()
        .tracks
        .iter()
        .filter(|track| !track.is_gutter)
        .map(|track| (track.is_collapsed, track.base_size))
        .collect()
}

fn is_invalid_track_error<T>(
    result: &Result<T, crate::LayoutError>,
    expected_node: crate::NodeId,
    expected_property: &str,
) -> bool {
    matches!(
        result,
        Err(crate::LayoutError::InvalidGridTrack {
            node,
            property: actual_property,
        }) | Err(crate::LayoutError::InvalidGridTrackList {
            node,
            property: actual_property,
        }) if *node == expected_node && *actual_property == expected_property
    )
}

#[test]
fn zero_percent_minimum_admits_only_definite_positive_content_axes() {
    for (axis, property) in [
        (LayoutGridTrackSizingAxis::Columns, "grid-template-columns"),
        (LayoutGridTrackSizingAxis::Rows, "grid-template-rows"),
    ] {
        for (label, invalid_minimum) in [
            ("negative", -0.1),
            ("NaN", f32::NAN),
            ("infinite", f32::INFINITY),
        ] {
            let input = repeat_input(axis, GridAutoRepeatMode::AutoFill, invalid_minimum, 0.0, 1);
            let result = TaffyLayoutEngine.compute(&input);
            assert!(
                is_invalid_track_error(&result, id(1), property),
                "{axis:?}/{label} minimum must fail closed: {result:?}"
            );
        }

        let mut indefinite = repeat_input(axis, GridAutoRepeatMode::AutoFill, 0.0, 0.0, 1);
        match axis {
            LayoutGridTrackSizingAxis::Columns => {
                indefinite.nodes[0].style.width = LayoutDimension::Auto;
            }
            LayoutGridTrackSizingAxis::Rows => {
                indefinite.nodes[0].style.height = LayoutDimension::Auto;
            }
        }
        indefinite.root_sizing = RootSizingPolicy::ResolveWithin;
        let result = TaffyLayoutEngine.compute(&indefinite);
        assert!(
            is_invalid_track_error(&result, id(1), property),
            "{axis:?} indefinite content basis must fail closed"
        );

        let mut zero_basis = repeat_input(axis, GridAutoRepeatMode::AutoFill, 0.0, 0.0, 1);
        match axis {
            LayoutGridTrackSizingAxis::Columns => {
                zero_basis.nodes[0].style.width = LayoutDimension::Fixed(0.0);
            }
            LayoutGridTrackSizingAxis::Rows => {
                zero_basis.nodes[0].style.height = LayoutDimension::Fixed(0.0);
            }
        }
        zero_basis.root_sizing = RootSizingPolicy::ResolveWithin;
        let result = TaffyLayoutEngine.compute(&zero_basis);
        assert!(
            is_invalid_track_error(&result, id(1), property),
            "{axis:?} zero content basis must fail closed"
        );

        let mut unsupported_maximum = repeat_input(axis, GridAutoRepeatMode::AutoFill, 0.0, 0.0, 1);
        let container = unsupported_maximum.grid_containers.get_mut(&id(1)).unwrap();
        let track = GridTrackSize::MinMax {
            min: GridTrackMinSizingFunction::Percentage(0.0),
            max: GridTrackMaxSizingFunction::Auto,
        };
        match axis {
            LayoutGridTrackSizingAxis::Columns => container.columns[0] = track,
            LayoutGridTrackSizingAxis::Rows => container.rows[0] = track,
        }
        let result = TaffyLayoutEngine.compute(&unsupported_maximum);
        assert!(
            is_invalid_track_error(&result, id(1), property),
            "{axis:?} unsupported maximum must fail closed"
        );
    }
}

#[test]
fn zero_percent_repeat_uses_count_floor_without_changing_auto_fit_sizing() {
    for axis in [
        LayoutGridTrackSizingAxis::Columns,
        LayoutGridTrackSizingAxis::Rows,
    ] {
        let fill = repeat_input(axis, GridAutoRepeatMode::AutoFill, 0.0, 0.0, 1);
        let fill_tracks = tracks_at_stage(&fill, axis, LayoutGridTrackSizingStageKind::Initialize);
        assert_eq!(fill_tracks.len(), 120, "{axis:?} auto-fill count");
        assert!(fill_tracks.iter().all(|(collapsed, _)| !collapsed));

        let fit_empty = repeat_input(axis, GridAutoRepeatMode::AutoFit, 0.0, 0.0, 0);
        let empty_tracks =
            tracks_at_stage(&fit_empty, axis, LayoutGridTrackSizingStageKind::Initialize);
        assert_eq!(empty_tracks.len(), 120, "{axis:?} pre-placement count");
        assert!(
            empty_tracks
                .iter()
                .all(|(collapsed, size)| *collapsed && *size == 0.0)
        );

        let fit_occupied = repeat_input(axis, GridAutoRepeatMode::AutoFit, 0.0, 0.0, 1);
        let occupied_tracks = tracks_at_stage(
            &fit_occupied,
            axis,
            LayoutGridTrackSizingStageKind::Initialize,
        );
        assert_eq!(occupied_tracks.len(), 120, "{axis:?} pre-placement count");
        assert_eq!(
            occupied_tracks
                .iter()
                .filter(|(collapsed, _)| *collapsed)
                .count(),
            119,
            "{axis:?} post-placement collapse"
        );
        let expanded_tracks = tracks_at_stage(
            &fit_occupied,
            axis,
            LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
        );
        assert_eq!(
            expanded_tracks[0],
            (false, 120.0),
            "{axis:?} occupied track"
        );
        assert!(
            expanded_tracks[1..]
                .iter()
                .all(|(collapsed, size)| *collapsed && *size == 0.0)
        );
    }
}

#[test]
fn zero_percent_repeat_count_keeps_gaps_and_border_box_content_basis() {
    let with_gap = repeat_input(
        LayoutGridTrackSizingAxis::Columns,
        GridAutoRepeatMode::AutoFill,
        0.0,
        4.0,
        1,
    );
    assert_eq!(
        tracks_at_stage(
            &with_gap,
            LayoutGridTrackSizingAxis::Columns,
            LayoutGridTrackSizingStageKind::Initialize,
        )
        .len(),
        24
    );

    let mut border_box = repeat_input(
        LayoutGridTrackSizingAxis::Columns,
        GridAutoRepeatMode::AutoFill,
        0.0,
        4.0,
        1,
    );
    border_box.nodes[0].style.box_sizing = LayoutBoxSizing::BorderBox;
    border_box.nodes[0].style.width = LayoutDimension::Fixed(136.0);
    border_box.nodes[0].style.height = LayoutDimension::Fixed(136.0);
    border_box.nodes[0].style.padding = crate::LayoutEdges {
        top: LayoutLengthPercentage::length(8.0),
        right: LayoutLengthPercentage::length(8.0),
        bottom: LayoutLengthPercentage::length(8.0),
        left: LayoutLengthPercentage::length(8.0),
    };
    border_box.nodes[0].style.border = LayoutBorder {
        top: 2.0,
        right: 2.0,
        bottom: 2.0,
        left: 2.0,
    };
    border_box.viewport = Viewport {
        width: 136.0,
        height: 136.0,
    };
    assert_eq!(
        tracks_at_stage(
            &border_box,
            LayoutGridTrackSizingAxis::Columns,
            LayoutGridTrackSizingStageKind::Initialize,
        )
        .len(),
        24
    );
}
