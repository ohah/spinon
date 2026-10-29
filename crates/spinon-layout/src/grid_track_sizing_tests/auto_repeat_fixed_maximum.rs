use serde_json::Value;
use spinon_core::NodeId;

use super::{grid_input, id};
use crate::{
    GridAutoRepeat, GridAutoRepeatMode, GridPlacementMode, GridTrackMaxSizingFunction,
    GridTrackMinSizingFunction, GridTrackSize, LayoutBoxSizing, LayoutCalcId, LayoutCssMath,
    LayoutCssMathProperty, LayoutCssMathValue, LayoutDimension, LayoutEngine, LayoutGap,
    LayoutGridTrackSizingAxis, LayoutInput, LayoutLengthPercentage, RootSizingPolicy,
    TaffyLayoutEngine, Viewport,
};

const CHROME_OBSERVATIONS: &str = include_str!(
    "../../../../spec/internal/evidence/c11-4-auto-repeat-inflexible-fixed-maximum-precomparison-2026-10-29/chrome-headless-observed.json"
);

fn scene_axis(scene: &Value) -> LayoutGridTrackSizingAxis {
    match scene["axis"].as_str().unwrap() {
        "columns" => LayoutGridTrackSizingAxis::Columns,
        "rows" => LayoutGridTrackSizingAxis::Rows,
        value => panic!("unexpected axis: {value}"),
    }
}

fn track_minimum(value: &str) -> GridTrackMinSizingFunction {
    match value {
        "auto" => GridTrackMinSizingFunction::Auto,
        "min-content" => GridTrackMinSizingFunction::MinContent,
        "max-content" => GridTrackMinSizingFunction::MaxContent,
        "0px" => GridTrackMinSizingFunction::LengthPx(0.0),
        value => panic!("unexpected minimum breadth: {value}"),
    }
}

fn track_maximum(
    value: &str,
    math_id: LayoutCalcId,
) -> (GridTrackMaxSizingFunction, Option<LayoutCssMath>) {
    match value {
        "40px" => (GridTrackMaxSizingFunction::LengthPx(40.0), None),
        "30%" => (GridTrackMaxSizingFunction::Percentage(0.3), None),
        "10px" => (GridTrackMaxSizingFunction::LengthPx(10.0), None),
        "calc(15px + 25%)" => (
            GridTrackMaxSizingFunction::Calc(math_id),
            Some(LayoutCssMath::Sum(vec![
                LayoutCssMath::LengthPx(15.0),
                LayoutCssMath::Percentage(0.25),
            ])),
        ),
        value => panic!("unexpected maximum breadth: {value}"),
    }
}

fn observation_input(scene: &Value) -> LayoutInput {
    let axis = scene_axis(scene);
    let mode = match scene["mode"].as_str().unwrap() {
        "auto-fill" => GridAutoRepeatMode::AutoFill,
        "auto-fit" => GridAutoRepeatMode::AutoFit,
        value => panic!("unexpected repeat mode: {value}"),
    };
    let minimum = track_minimum(scene["minimum"].as_str().unwrap());
    let calc_id = LayoutCalcId(7);
    let (maximum, expression) = track_maximum(scene["maximum"].as_str().unwrap(), calc_id);
    let repeated_track = GridTrackSize::MinMax {
        min: minimum,
        max: maximum,
    };
    let mut repeated_tracks = vec![repeated_track];
    if scene["pattern"] == "two-track" {
        repeated_tracks.push(GridTrackSize::LengthPx(20.0));
    }
    let repeat = GridAutoRepeat {
        mode,
        start: 0,
        end: repeated_tracks.len(),
        line_names: Vec::new(),
    };
    let (columns, rows) = match axis {
        LayoutGridTrackSizingAxis::Columns => {
            (repeated_tracks, vec![GridTrackSize::LengthPx(20.0)])
        }
        LayoutGridTrackSizingAxis::Rows => (vec![GridTrackSize::LengthPx(20.0)], repeated_tracks),
    };
    let mut input = grid_input(
        columns,
        rows,
        1,
        LayoutGap {
            column: LayoutLengthPercentage::length(if axis == LayoutGridTrackSizingAxis::Columns {
                5.0
            } else {
                0.0
            }),
            row: LayoutLengthPercentage::length(if axis == LayoutGridTrackSizingAxis::Rows {
                5.0
            } else {
                0.0
            }),
        },
    );
    let basis = scene["basis"].as_f64().unwrap() as f32;
    let cross_axis = 40.0;
    input.nodes[0].style.box_sizing = LayoutBoxSizing::ContentBox;
    input.nodes[0].style.width =
        LayoutDimension::Fixed(if axis == LayoutGridTrackSizingAxis::Columns {
            basis
        } else {
            cross_axis
        });
    input.nodes[0].style.height =
        LayoutDimension::Fixed(if axis == LayoutGridTrackSizingAxis::Rows {
            basis
        } else {
            cross_axis
        });
    input.nodes[0].style.min_width = LayoutDimension::Fixed(0.0);
    input.nodes[0].style.min_height = LayoutDimension::Fixed(0.0);
    input.nodes[1].style.width = LayoutDimension::Fixed(20.0);
    input.nodes[1].style.height = LayoutDimension::Fixed(20.0);
    input.viewport = Viewport {
        width: if axis == LayoutGridTrackSizingAxis::Columns {
            basis
        } else {
            cross_axis
        },
        height: if axis == LayoutGridTrackSizingAxis::Rows {
            basis
        } else {
            cross_axis
        },
    };
    input.root_sizing = RootSizingPolicy::Match;

    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    match axis {
        LayoutGridTrackSizingAxis::Columns => container.columns_auto_repeat = Some(repeat),
        LayoutGridTrackSizingAxis::Rows => container.rows_auto_repeat = Some(repeat),
    }
    if let Some(expression) = expression {
        let property = match axis {
            LayoutGridTrackSizingAxis::Columns => LayoutCssMathProperty::GridTemplateColumns,
            LayoutGridTrackSizingAxis::Rows => LayoutCssMathProperty::GridTemplateRows,
        };
        input.css_math.push(LayoutCssMathValue {
            id: calc_id,
            node_id: NodeId::new(1).unwrap(),
            property,
            expression,
        });
    }
    input
}

fn track_values(serialized: &str) -> Vec<f32> {
    serialized
        .split_whitespace()
        .map(|value| value.strip_suffix("px").unwrap().parse().unwrap())
        .collect()
}

fn assert_size(actual: crate::LayoutFrame, expected: &Value, scene_id: &str, label: &str) {
    for (property, value) in [("width", actual.width), ("height", actual.height)] {
        let expected = expected[property].as_f64().unwrap() as f32;
        assert!(
            (value - expected).abs() <= 1.0 / 64.0,
            "{scene_id} {label}.{property}: {value} != {expected}"
        );
    }
}

fn assert_item_frame(
    actual: crate::LayoutFrame,
    actual_root: crate::LayoutFrame,
    expected: &Value,
    expected_root: &Value,
    scene_id: &str,
) {
    for (property, value, root_offset) in [
        ("x", actual.x, actual_root.x),
        ("y", actual.y, actual_root.y),
        ("width", actual.width, 0.0),
        ("height", actual.height, 0.0),
    ] {
        let expected_value = expected[property].as_f64().unwrap() as f32;
        let expected_root_offset = if property == "x" || property == "y" {
            expected_root[property].as_f64().unwrap() as f32
        } else {
            0.0
        };
        assert!(
            (value - root_offset - (expected_value - expected_root_offset)).abs() <= 1.0 / 64.0,
            "{scene_id} item.{property}: {} != {} relative to root",
            value - root_offset,
            expected_value - expected_root_offset
        );
    }
}

#[test]
fn fixed_maximum_auto_repeat_matches_all_chrome_precomparison_scenes() {
    let scenes: Value = serde_json::from_str(CHROME_OBSERVATIONS).unwrap();
    let scenes = scenes.as_array().unwrap();
    assert_eq!(scenes.len(), 49);

    for scene in scenes {
        let scene_id = scene["id"].as_str().unwrap();
        let input = observation_input(scene);
        let output = TaffyLayoutEngine
            .compute(&input)
            .unwrap_or_else(|error| panic!("{scene_id}: unexpected layout error: {error:?}"));
        let trace = output
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == id(1))
            .unwrap_or_else(|| panic!("{scene_id}: missing track-sizing trace"));
        let axis = scene_axis(scene);
        let expected_serialized = match axis {
            LayoutGridTrackSizingAxis::Columns => scene["gridTemplateColumns"].as_str().unwrap(),
            LayoutGridTrackSizingAxis::Rows => scene["gridTemplateRows"].as_str().unwrap(),
        };
        let actual_serialized = match axis {
            LayoutGridTrackSizingAxis::Columns => &trace.used_columns.used_track_list,
            LayoutGridTrackSizingAxis::Rows => &trace.used_rows.used_track_list,
        };
        let expected_tracks = track_values(expected_serialized);
        let actual_tracks = track_values(actual_serialized);
        assert_eq!(
            actual_tracks.len(),
            expected_tracks.len(),
            "{scene_id} track count"
        );
        for (actual, expected) in actual_tracks.iter().zip(expected_tracks) {
            assert!(
                (actual - expected).abs() <= 1.0 / 64.0,
                "{scene_id} tracks: {actual_serialized} != {expected_serialized}"
            );
        }
        let root_frame = output.frames[&id(1)];
        assert_size(root_frame, &scene["root"], scene_id, "root");
        assert_item_frame(
            output.frames[&id(2)],
            root_frame,
            &scene["item"],
            &scene["root"],
            scene_id,
        );
    }
}
