use super::{grid_input, id};
use crate::{
    GRID_MAX_TRACKS, GridAutoRepeat, GridAutoRepeatMode, GridPlacementMode,
    GridTrackMaxSizingFunction, GridTrackMinSizingFunction, GridTrackSize, LayoutDimension,
    LayoutEngine, LayoutGap, LayoutGridTrackSizingAxis, LayoutGridTrackSizingStageKind,
    TaffyLayoutEngine, Viewport,
};

fn minmax_px_flex(minimum: f32, factor: f32) -> GridTrackSize {
    GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::LengthPx(minimum),
        max: GridTrackMaxSizingFunction::Fraction(factor),
    }
}

fn set_track_sizing_profile(input: &mut crate::LayoutInput) {
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::TrackSizing;
}

fn set_auto_repeat_axis(
    input: &mut crate::LayoutInput,
    axis: LayoutGridTrackSizingAxis,
    mode: GridAutoRepeatMode,
    start: usize,
    end: usize,
) {
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    let repeat = GridAutoRepeat {
        mode,
        start,
        end,
        line_names: Vec::new(),
    };
    match axis {
        LayoutGridTrackSizingAxis::Columns => container.columns_auto_repeat = Some(repeat),
        LayoutGridTrackSizingAxis::Rows => container.rows_auto_repeat = Some(repeat),
    }
}

fn axis_property(axis: LayoutGridTrackSizingAxis) -> &'static str {
    match axis {
        LayoutGridTrackSizingAxis::Columns => "grid-template-columns",
        LayoutGridTrackSizingAxis::Rows => "grid-template-rows",
    }
}

fn tracks_at_stage(
    input: &crate::LayoutInput,
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

fn zero_and_sub_one_flex_input(axis: LayoutGridTrackSizingAxis) -> crate::LayoutInput {
    let (columns, rows) = match axis {
        LayoutGridTrackSizingAxis::Columns => (
            vec![minmax_px_flex(0.0, 0.0), minmax_px_flex(0.0, 0.5)],
            vec![GridTrackSize::LengthPx(120.0)],
        ),
        LayoutGridTrackSizingAxis::Rows => (
            vec![GridTrackSize::LengthPx(120.0)],
            vec![minmax_px_flex(0.0, 0.0), minmax_px_flex(0.0, 0.5)],
        ),
    };
    let mut input = grid_input(columns, rows, 0, LayoutGap::default());
    input.nodes[0].style.width = LayoutDimension::Fixed(200.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(200.0);
    input.viewport = Viewport {
        width: 200.0,
        height: 200.0,
    };
    set_track_sizing_profile(&mut input);
    input
}

fn flex_overflow_without_repeat_input(axis: LayoutGridTrackSizingAxis) -> crate::LayoutInput {
    let overflow_tracks = vec![minmax_px_flex(1.0, 1.0e38); 4];
    let (columns, rows) = match axis {
        LayoutGridTrackSizingAxis::Columns => {
            (overflow_tracks, vec![GridTrackSize::LengthPx(120.0)])
        }
        LayoutGridTrackSizingAxis::Rows => (vec![GridTrackSize::LengthPx(120.0)], overflow_tracks),
    };
    let mut input = grid_input(columns, rows, 0, LayoutGap::default());
    set_track_sizing_profile(&mut input);
    input
}

fn flex_overflow_at_auto_repeat_ceiling_input(
    axis: LayoutGridTrackSizingAxis,
) -> crate::LayoutInput {
    let repeated = minmax_px_flex(1.0, 1.0e38);
    let (columns, rows, repeat_start, repeat_end) = match axis {
        LayoutGridTrackSizingAxis::Columns => (
            vec![GridTrackSize::LengthPx(1.0), repeated],
            vec![GridTrackSize::LengthPx(120.0)],
            1,
            2,
        ),
        LayoutGridTrackSizingAxis::Rows => (
            vec![GridTrackSize::LengthPx(120.0)],
            vec![GridTrackSize::LengthPx(1.0), repeated],
            1,
            2,
        ),
    };
    let mut input = grid_input(columns, rows, 0, LayoutGap::default());
    match axis {
        LayoutGridTrackSizingAxis::Columns => {
            input.nodes[0].style.width = LayoutDimension::Fixed(GRID_MAX_TRACKS as f32);
            input.nodes[0].style.height = LayoutDimension::Fixed(120.0);
        }
        LayoutGridTrackSizingAxis::Rows => {
            input.nodes[0].style.width = LayoutDimension::Fixed(120.0);
            input.nodes[0].style.height = LayoutDimension::Fixed(GRID_MAX_TRACKS as f32);
        }
    }
    input.viewport = Viewport {
        width: match axis {
            LayoutGridTrackSizingAxis::Columns => GRID_MAX_TRACKS as f32,
            LayoutGridTrackSizingAxis::Rows => 120.0,
        },
        height: match axis {
            LayoutGridTrackSizingAxis::Columns => 120.0,
            LayoutGridTrackSizingAxis::Rows => GRID_MAX_TRACKS as f32,
        },
    };
    set_track_sizing_profile(&mut input);
    set_auto_repeat_axis(
        &mut input,
        axis,
        GridAutoRepeatMode::AutoFill,
        repeat_start,
        repeat_end,
    );
    input
}

fn fixed_min_flex_neighbors_input(axis: LayoutGridTrackSizingAxis) -> crate::LayoutInput {
    let (columns, rows, start, end, width, height) = match axis {
        LayoutGridTrackSizingAxis::Columns => (
            vec![
                minmax_px_flex(12.0, 0.5),
                minmax_px_flex(24.0, 1.0),
                minmax_px_flex(16.0, 2.0),
            ],
            vec![GridTrackSize::LengthPx(120.0)],
            1,
            2,
            220.0,
            120.0,
        ),
        LayoutGridTrackSizingAxis::Rows => (
            vec![GridTrackSize::LengthPx(120.0)],
            vec![
                minmax_px_flex(12.0, 0.5),
                minmax_px_flex(24.0, 1.0),
                minmax_px_flex(16.0, 2.0),
            ],
            1,
            2,
            120.0,
            220.0,
        ),
    };
    let gap = LayoutGap {
        column: crate::LayoutLengthPercentage::length(4.0),
        row: crate::LayoutLengthPercentage::length(4.0),
    };
    let mut input = grid_input(columns, rows, 0, gap);
    input.nodes[0].style.width = LayoutDimension::Fixed(width);
    input.nodes[0].style.height = LayoutDimension::Fixed(height);
    input.viewport = Viewport { width, height };
    set_track_sizing_profile(&mut input);
    set_auto_repeat_axis(
        &mut input,
        axis,
        crate::GridAutoRepeatMode::AutoFill,
        start,
        end,
    );
    input
}

#[test]
fn zero_and_sub_one_flex_factors_remain_valid_on_both_axes() {
    for axis in [
        LayoutGridTrackSizingAxis::Columns,
        LayoutGridTrackSizingAxis::Rows,
    ] {
        let input = zero_and_sub_one_flex_input(axis);
        let actual = tracks_at_stage(
            &input,
            axis,
            LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
        );
        assert_eq!(actual.len(), 2, "{axis:?} flex track count");
        assert!(actual.iter().all(|(collapsed, _)| !collapsed));
        assert!((actual[0].1 - 0.0).abs() <= 0.02, "{axis:?}: {actual:?}");
        assert!((actual[1].1 - 100.0).abs() <= 0.02, "{axis:?}: {actual:?}");
    }
}

#[test]
fn flex_factor_sum_overflow_is_rejected_with_axis_property_context() {
    for axis in [
        LayoutGridTrackSizingAxis::Columns,
        LayoutGridTrackSizingAxis::Rows,
    ] {
        let input = flex_overflow_without_repeat_input(axis);
        assert!(matches!(
            TaffyLayoutEngine.compute(&input),
            Err(crate::LayoutError::InvalidGridTrackList { node, property })
                if node == id(1) && property == axis_property(axis)
        ));
    }
}

#[test]
fn auto_repeat_flex_factor_bound_uses_the_track_ceiling_on_both_axes() {
    for axis in [
        LayoutGridTrackSizingAxis::Columns,
        LayoutGridTrackSizingAxis::Rows,
    ] {
        let input = flex_overflow_at_auto_repeat_ceiling_input(axis);
        assert!(matches!(
            TaffyLayoutEngine.compute(&input),
            Err(crate::LayoutError::InvalidGridTrackList { node, property })
                if node == id(1) && property == axis_property(axis)
        ));
    }
}

#[test]
fn fixed_minimum_flex_maximum_neighbors_size_auto_repeat_on_both_axes() {
    for axis in [
        LayoutGridTrackSizingAxis::Columns,
        LayoutGridTrackSizingAxis::Rows,
    ] {
        let input = fixed_min_flex_neighbors_input(axis);
        let tracks = tracks_at_stage(
            &input,
            axis,
            LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
        );
        assert_eq!(tracks.len(), 8, "{axis:?} repeat count uses fixed minima");
        assert!(tracks.iter().all(|(collapsed, _)| !collapsed), "{axis:?}");
    }
}

const FLEXIBLE_TRACK_EXPANSION_EXPECTED_JSON: &str = include_str!(
    "../../../../spec/internal/evidence/c11-4-flex-stage-oracle-precomparison-2026-10-28/expected.json"
);
const FLEXIBLE_TRACK_EXPANSION_TOLERANCE: f32 = 1.0e-4;

fn flexible_expansion_json_rational(value: &serde_json::Value, label: &str) -> f32 {
    let (numerator, denominator) = match value {
        serde_json::Value::Number(number) => (number.as_i64().expect("integer JSON number"), 1),
        serde_json::Value::String(text) => {
            let (numerator, denominator) = text.split_once('/').unwrap_or((text, "1"));
            (
                numerator.parse::<i64>().expect("rational numerator"),
                denominator.parse::<i64>().expect("rational denominator"),
            )
        }
        _ => panic!("{label}: expected integer or rational, got {value}"),
    };
    assert!(denominator > 0, "{label}: denominator must be positive");
    let value = (numerator as f64 / denominator as f64) as f32;
    assert!(value.is_finite(), "{label}: rational is not finite as f32");
    value
}

fn assert_flexible_expansion_close(actual: f32, expected: f32, label: &str) {
    assert!(
        actual.is_finite()
            && expected.is_finite()
            && (actual - expected).abs() <= FLEXIBLE_TRACK_EXPANSION_TOLERANCE,
        "{label}: expected {expected} CSS px, got {actual} CSS px"
    );
}

fn flexible_expansion_input(
    case: &serde_json::Value,
) -> (crate::LayoutInput, LayoutGridTrackSizingAxis, f32) {
    let case_id = case["id"].as_str().expect("case id string");
    let axis_name = case["axis"].as_str().expect("case axis string");
    let axis = if axis_name == "columns" {
        LayoutGridTrackSizingAxis::Columns
    } else {
        assert_eq!(axis_name, "rows", "{case_id}: unsupported axis");
        LayoutGridTrackSizingAxis::Rows
    };
    let px = |key| flexible_expansion_json_rational(&case[key], case_id);
    let [available, opposite, gap_px] = ["available_size_px", "opposite_size_px", "gap_px"].map(px);
    assert_eq!((available, opposite, gap_px), (200.0, 120.0, 10.0));

    let axis_tracks = case["tracks"]
        .as_array()
        .expect("tracks array")
        .iter()
        .map(|track| {
            let base = flexible_expansion_json_rational(&track["base_size_px"], case_id);
            let factor = (!track["flex_factor"].is_null())
                .then(|| flexible_expansion_json_rational(&track["flex_factor"], case_id));
            let descriptor = factor.map_or_else(
                || format!("{base}px"),
                |f| format!("minmax({base}px, {f}fr)"),
            );
            assert_eq!(track["css"].as_str(), Some(descriptor.as_str()));
            factor.map_or_else(
                || GridTrackSize::LengthPx(base),
                |f| minmax_px_flex(base, f),
            )
        })
        .collect::<Vec<_>>();
    let (columns, rows, width, height) = match axis {
        LayoutGridTrackSizingAxis::Columns => (
            axis_tracks,
            vec![GridTrackSize::LengthPx(opposite)],
            available,
            opposite,
        ),
        LayoutGridTrackSizingAxis::Rows => (
            vec![GridTrackSize::LengthPx(opposite)],
            axis_tracks,
            opposite,
            available,
        ),
    };
    let mut gap = LayoutGap::default();
    match axis {
        LayoutGridTrackSizingAxis::Columns => {
            gap.column = crate::LayoutLengthPercentage::length(gap_px);
        }
        LayoutGridTrackSizingAxis::Rows => {
            gap.row = crate::LayoutLengthPercentage::length(gap_px);
        }
    }
    let mut input = grid_input(columns, rows, 0, gap);
    input.nodes[0].style.width = LayoutDimension::Fixed(width);
    input.nodes[0].style.height = LayoutDimension::Fixed(height);
    input.viewport = Viewport { width, height };
    set_track_sizing_profile(&mut input);
    (input, axis, gap_px)
}

#[test]
fn flexible_track_expansion_stage_matches_css_grid_oracle() {
    let oracle: serde_json::Value =
        serde_json::from_str(FLEXIBLE_TRACK_EXPANSION_EXPECTED_JSON).expect("valid oracle JSON");
    assert_eq!(oracle["stage"].as_str(), Some("ExpandFlexibleTracks"));
    let case_array = oracle["cases"].as_array().expect("oracle cases array");
    let cases = case_array
        .iter()
        .map(|case| (case["id"].as_str().expect("case id string"), case))
        .collect::<std::collections::BTreeMap<_, _>>();
    let case_ids = [
        ("weighted-columns", "columns"),
        ("frozen-row", "rows"),
        ("subunit-columns", "columns"),
        ("zero-fr-row", "rows"),
    ];
    assert_eq!(cases.len(), case_ids.len(), "exact oracle case set");
    assert_eq!(case_array.len(), cases.len(), "unique oracle case ids");

    for (case_id, expected_axis) in case_ids {
        let case = cases
            .get(case_id)
            .unwrap_or_else(|| panic!("missing case {case_id}"));
        assert_eq!(
            case["axis"].as_str(),
            Some(expected_axis),
            "{case_id}: axis"
        );
        let (input, axis, gap_px) = flexible_expansion_input(case);
        assert!(
            input.nodes.len() == 1 && input.grid_items.is_empty(),
            "{case_id}: empty grid input"
        );
        let tracks = case["tracks"].as_array().expect("tracks array");
        assert_eq!(tracks.len(), 2, "{case_id}: two CSS tracks");
        let expected = case["expected_track_bases_px"]
            .as_array()
            .expect("expected track bases")
            .iter()
            .map(|value| flexible_expansion_json_rational(value, case_id))
            .collect::<Vec<_>>();
        assert_eq!(expected.len(), tracks.len(), "{case_id}: CSS track count");

        let output = TaffyLayoutEngine
            .compute(&input)
            .unwrap_or_else(|error| panic!("{case_id}: layout failed: {error:?}"));
        assert_eq!(
            output.grid_track_sizing_traces.len(),
            1,
            "{case_id}: root trace"
        );
        let trace = &output.grid_track_sizing_traces[0];
        assert_eq!(trace.node, id(1), "{case_id}: trace owner");
        let mut passes = trace.passes.iter().filter(|pass| pass.axis == axis);
        let pass = passes
            .next()
            .unwrap_or_else(|| panic!("{case_id}: missing {axis:?} pass"));
        assert!(
            passes.next().is_none(),
            "{case_id}: duplicate {axis:?} pass"
        );

        let position = |target| pass.stages.iter().position(|stage| stage.stage == target);
        let maximize = position(LayoutGridTrackSizingStageKind::MaximizeTracks)
            .unwrap_or_else(|| panic!("{case_id}: missing MaximizeTracks"));
        let expand = position(LayoutGridTrackSizingStageKind::ExpandFlexibleTracks)
            .unwrap_or_else(|| panic!("{case_id}: missing ExpandFlexibleTracks"));
        assert_eq!(
            pass.stages
                .iter()
                .filter(|stage| stage.stage == LayoutGridTrackSizingStageKind::ExpandFlexibleTracks)
                .count(),
            1,
            "{case_id}: unique ExpandFlexibleTracks"
        );
        let stretch = pass
            .stages
            .iter()
            .position(|stage| {
                matches!(
                    stage.stage,
                    LayoutGridTrackSizingStageKind::StretchAutoTracks { .. }
                )
            })
            .unwrap_or_else(|| panic!("{case_id}: missing StretchAutoTracks"));
        assert!(
            maximize < expand && expand < stretch,
            "{case_id}: stage order"
        );

        let stage = &pass.stages[expand];
        assert_eq!(stage.tracks.len(), tracks.len() * 2 + 1);
        let mut css_tracks = Vec::new();
        for (index, track) in stage.tracks.iter().enumerate() {
            let boundary = index == 0 || index + 1 == stage.tracks.len();
            assert!(track.base_size.is_finite(), "{case_id}: entry {index}");
            assert_eq!(
                track.is_gutter,
                index % 2 == 0,
                "{case_id}: entry {index} kind"
            );
            assert_eq!(
                track.is_collapsed,
                boundary && track.is_gutter,
                "{case_id}: sentinel {index}"
            );
            if track.is_gutter {
                assert_flexible_expansion_close(
                    track.base_size,
                    if boundary { 0.0 } else { gap_px },
                    &format!("{case_id}: gutter {index}"),
                );
            } else {
                css_tracks.push(track);
            }
        }
        assert_eq!(css_tracks.len(), expected.len(), "{case_id}: CSS tracks");
        for (index, (actual, expected)) in css_tracks.iter().zip(expected).enumerate() {
            assert_flexible_expansion_close(
                actual.base_size,
                expected,
                &format!("{case_id}: track {index}"),
            );
        }
    }
}
