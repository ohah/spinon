use serde_json::Value;

use super::{grid_input, id};
use crate::grid_placement::{GridItemInput, GridPlacementInput};
use crate::{
    GridAutoRepeat, GridAutoRepeatMode, GridPlacementMode, GridTrackMaxSizingFunction,
    GridTrackMinSizingFunction, GridTrackSize, LayoutBoxSizing, LayoutDimension, LayoutEngine,
    LayoutGap, LayoutGridTrackSizingAxis, LayoutLengthPercentage, RootSizingPolicy,
    TaffyLayoutEngine, Viewport,
};

const CHROME_OBSERVATIONS: &str = include_str!(
    "../../../../spec/internal/evidence/c11-4-auto-repeat-zero-fixed-maximum-precomparison-2026-10-29/chrome-156-observations.json"
);

fn observed_input(scene: &Value) -> crate::LayoutInput {
    let axis = match scene["axis"].as_str().unwrap() {
        "columns" => LayoutGridTrackSizingAxis::Columns,
        "rows" => LayoutGridTrackSizingAxis::Rows,
        value => panic!("unexpected axis: {value}"),
    };
    let mode = match scene["mode"].as_str().unwrap() {
        "auto-fill" => GridAutoRepeatMode::AutoFill,
        "auto-fit" => GridAutoRepeatMode::AutoFit,
        value => panic!("unexpected repeat mode: {value}"),
    };
    let minimum = match scene["minimum"].as_str().unwrap() {
        "auto" => GridTrackMinSizingFunction::Auto,
        "min-content" => GridTrackMinSizingFunction::MinContent,
        "max-content" => GridTrackMinSizingFunction::MaxContent,
        value => panic!("unexpected minimum: {value}"),
    };
    let track = GridTrackSize::MinMax {
        min: minimum,
        max: GridTrackMaxSizingFunction::LengthPx(0.0),
    };
    let repeat = GridAutoRepeat {
        mode,
        start: 0,
        end: 1,
        line_names: Vec::new(),
    };
    let fixed_axis = vec![GridTrackSize::LengthPx(20.0)];
    let (columns, rows) = match axis {
        LayoutGridTrackSizingAxis::Columns => (vec![track], fixed_axis),
        LayoutGridTrackSizingAxis::Rows => (fixed_axis, vec![track]),
    };
    let gap = scene["gap"].as_f64().unwrap() as f32;
    let mut input = grid_input(
        columns,
        rows,
        1,
        LayoutGap {
            column: LayoutLengthPercentage::length(if axis == LayoutGridTrackSizingAxis::Columns {
                gap
            } else {
                0.0
            }),
            row: LayoutLengthPercentage::length(if axis == LayoutGridTrackSizingAxis::Rows {
                gap
            } else {
                0.0
            }),
        },
    );
    let basis = scene["basis"].as_f64().unwrap() as f32;
    let (root_width, root_height) = match axis {
        LayoutGridTrackSizingAxis::Columns => (basis, 40.0),
        LayoutGridTrackSizingAxis::Rows => (40.0, basis),
    };
    input.nodes[0].style.box_sizing = LayoutBoxSizing::ContentBox;
    input.nodes[0].style.width = LayoutDimension::Fixed(root_width);
    input.nodes[0].style.height = LayoutDimension::Fixed(root_height);
    input.nodes[0].style.min_width = LayoutDimension::Fixed(0.0);
    input.nodes[0].style.min_height = LayoutDimension::Fixed(0.0);
    input.nodes[1].style.width = LayoutDimension::Fixed(12.0);
    input.nodes[1].style.height = LayoutDimension::Fixed(12.0);
    input.nodes[1].style.min_width = LayoutDimension::Fixed(12.0);
    input.nodes[1].style.min_height = LayoutDimension::Fixed(12.0);
    input.viewport = Viewport {
        width: root_width,
        height: root_height,
    };
    input.root_sizing = RootSizingPolicy::Match;

    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    match axis {
        LayoutGridTrackSizingAxis::Columns => container.columns_auto_repeat = Some(repeat),
        LayoutGridTrackSizingAxis::Rows => container.rows_auto_repeat = Some(repeat),
    }
    input.grid_items.insert(
        id(2),
        GridItemInput {
            column_start: GridPlacementInput::Line(if axis == LayoutGridTrackSizingAxis::Columns {
                3
            } else {
                1
            }),
            row_start: GridPlacementInput::Line(if axis == LayoutGridTrackSizingAxis::Rows {
                3
            } else {
                1
            }),
            ..GridItemInput::default()
        },
    );
    input
}

fn track_values(serialized: &str) -> Vec<f32> {
    serialized
        .split_whitespace()
        .map(|value| value.strip_suffix("px").unwrap().parse().unwrap())
        .collect()
}

fn assert_close(actual: f32, expected: f32, scene_id: &str, label: &str) {
    assert!(
        (actual - expected).abs() <= 1.0 / 64.0,
        "{scene_id} {label}: {actual} != {expected}"
    );
}

#[test]
fn zero_fixed_maximum_auto_repeat_matches_all_chrome_precomparison_scenes() {
    let observations: Value = serde_json::from_str(CHROME_OBSERVATIONS).unwrap();
    let scenes = observations["scenes"].as_array().unwrap();
    assert_eq!(scenes.len(), 24);

    for scene in scenes {
        let scene_id = scene["id"].as_str().unwrap();
        let axis = match scene["axis"].as_str().unwrap() {
            "columns" => LayoutGridTrackSizingAxis::Columns,
            "rows" => LayoutGridTrackSizingAxis::Rows,
            value => panic!("{scene_id}: unexpected axis {value}"),
        };
        let mode = scene["mode"].as_str().unwrap();
        let gap = scene["gap"].as_f64().unwrap() as usize;
        let basis = scene["basis"].as_f64().unwrap() as usize;
        let expected_repeat_count = (basis + gap) / (1 + gap);
        assert_eq!(
            scene["repeatedTrackCount"].as_u64().unwrap() as usize,
            expected_repeat_count,
            "{scene_id} independent UA floor count"
        );

        let input = observed_input(scene);
        let output = TaffyLayoutEngine
            .compute(&input)
            .unwrap_or_else(|error| panic!("{scene_id}: unexpected layout error: {error:?}"));
        let trace = output
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == id(1))
            .unwrap_or_else(|| panic!("{scene_id}: missing track-sizing trace"));
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        {
            let repeat_pass = trace
                .passes
                .iter()
                .find(|pass| pass.axis == axis && pass.auto_repeat_count.is_some())
                .unwrap_or_else(|| panic!("{scene_id}: missing auto-repeat count trace"));
            assert_eq!(
                repeat_pass.auto_repeat_count,
                Some(expected_repeat_count as u16),
                "{scene_id} traced repeat count"
            );
        }
        let (axis_trace, expected_serialized) = match axis {
            LayoutGridTrackSizingAxis::Columns => (
                &trace.used_columns,
                scene["gridTemplateColumns"].as_str().unwrap(),
            ),
            LayoutGridTrackSizingAxis::Rows => (
                &trace.used_rows,
                scene["gridTemplateRows"].as_str().unwrap(),
            ),
        };
        let expected_tracks = track_values(expected_serialized);
        let actual_tracks = track_values(&axis_trace.used_track_list);
        assert_eq!(
            actual_tracks.len(),
            expected_repeat_count,
            "{scene_id} count"
        );
        assert_eq!(
            actual_tracks.len(),
            expected_tracks.len(),
            "{scene_id} CSSOM count"
        );
        for (actual, expected) in actual_tracks.iter().zip(expected_tracks) {
            assert_close(*actual, expected, scene_id, "used track");
        }

        let expected_collapsed = if mode == "auto-fit" {
            (0..expected_repeat_count)
                .filter(|index| *index != 2)
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        assert_eq!(
            axis_trace.collapsed_track_indices, expected_collapsed,
            "{scene_id} collapsed tracks"
        );
        assert_eq!(
            axis_trace.explicit_track_count as usize, expected_repeat_count,
            "{scene_id} explicit count"
        );

        let root = output.frames[&id(1)];
        let item = output.frames[&id(2)];
        assert_close(
            root.width,
            scene["root"]["width"].as_f64().unwrap() as f32,
            scene_id,
            "root width",
        );
        assert_close(
            root.height,
            scene["root"]["height"].as_f64().unwrap() as f32,
            scene_id,
            "root height",
        );
        assert_close(
            item.x - root.x,
            scene["itemOffset"]["x"].as_f64().unwrap() as f32,
            scene_id,
            "item x",
        );
        assert_close(
            item.y - root.y,
            scene["itemOffset"]["y"].as_f64().unwrap() as f32,
            scene_id,
            "item y",
        );
        assert_close(
            item.width,
            scene["item"]["width"].as_f64().unwrap() as f32,
            scene_id,
            "item width",
        );
        assert_close(
            item.height,
            scene["item"]["height"].as_f64().unwrap() as f32,
            scene_id,
            "item height",
        );
    }
}
