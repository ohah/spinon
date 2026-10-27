use std::collections::BTreeMap;

mod expected;

use serde_json::Value;
use spinon_core::{EnvironmentRevision, HostNodeHandle, StyleRevision};

use crate::grid_placement::{GridItemInput, GridPlacementInput};
use crate::{
    GridContainerInput, GridPlacementMode, GridTrackMaxSizingFunction, GridTrackMinSizingFunction,
    GridTrackSize, LayoutAlignContent, LayoutDimension, LayoutDisplay, LayoutEngine, LayoutFrame,
    LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
    LayoutGridTrackSizingStageKind, LayoutGridTrackSizingStageTrace, LayoutInput,
    LayoutJustifyContent, LayoutStyle, TaffyLayoutEngine, Viewport,
};

use super::{assert_span_stage_snapshots_with_expectation, multi_span_document};
use expected::{assert_expected_scene_registry, row_axis_expectation};

const ROOT_SIZE: f32 = 220.0;
const TRACE_TOLERANCE: f32 = 1.0e-4;
const CHROME_TOLERANCE: f32 = 1.0 / 64.0;
const ROW_AXIS_EXPECTED_JSON: &str = include_str!(
    "../../../../spec/internal/evidence/c11-4-row-axis-span-stage-precomparison-2026-10-27/expected.json"
);

#[derive(Clone, Copy)]
struct RowAxisItem {
    id: &'static str,
    height: f32,
    row_start: i32,
    row_end: i32,
    chrome_local_y: f32,
}

struct ChromeRowAxisCase {
    id: &'static str,
    items_in_source_order: &'static [RowAxisItem],
    expected_span_batches: &'static [u16],
}

struct RowAxisObservation {
    stages: Vec<LayoutGridTrackSizingStageTrace>,
    item_frames: BTreeMap<&'static str, LayoutFrame>,
}

const SPAN2_A: RowAxisItem = RowAxisItem {
    id: "A",
    height: 80.0,
    row_start: 1,
    row_end: 3,
    chrome_local_y: 0.0,
};
const SPAN2_B: RowAxisItem = RowAxisItem {
    id: "B",
    height: 120.0,
    row_start: 2,
    row_end: 4,
    chrome_local_y: 40.0,
};
const SPAN23_A: RowAxisItem = RowAxisItem {
    id: "A",
    height: 80.0,
    row_start: 1,
    row_end: 3,
    chrome_local_y: 0.0,
};
const SPAN23_B: RowAxisItem = RowAxisItem {
    id: "B",
    height: 120.0,
    row_start: 2,
    row_end: 4,
    chrome_local_y: 53.328125,
};
const SPAN23_C: RowAxisItem = RowAxisItem {
    id: "C",
    height: 200.0,
    row_start: 1,
    row_end: 4,
    chrome_local_y: 0.0,
};

const SPAN2_FORWARD_ITEMS: &[RowAxisItem] = &[SPAN2_A, SPAN2_B];
const SPAN2_REVERSED_ITEMS: &[RowAxisItem] = &[SPAN2_B, SPAN2_A];
const SPAN23_FORWARD_ITEMS: &[RowAxisItem] = &[SPAN23_A, SPAN23_B, SPAN23_C];
const SPAN23_REVERSED_ITEMS: &[RowAxisItem] = &[SPAN23_C, SPAN23_B, SPAN23_A];

const CASES: [ChromeRowAxisCase; 4] = [
    ChromeRowAxisCase {
        id: "span2-forward",
        items_in_source_order: SPAN2_FORWARD_ITEMS,
        expected_span_batches: &[2],
    },
    ChromeRowAxisCase {
        id: "span2-reversed",
        items_in_source_order: SPAN2_REVERSED_ITEMS,
        expected_span_batches: &[2],
    },
    ChromeRowAxisCase {
        id: "span2-before-span3",
        items_in_source_order: SPAN23_FORWARD_ITEMS,
        expected_span_batches: &[2, 3],
    },
    ChromeRowAxisCase {
        id: "span3-before-span2",
        items_in_source_order: SPAN23_REVERSED_ITEMS,
        expected_span_batches: &[2, 3],
    },
];

fn row_axis_input(
    case: &ChromeRowAxisCase,
    owner: u64,
) -> (
    LayoutInput,
    spinon_core::HostNodeHandle,
    Vec<HostNodeHandle>,
) {
    let (document, root, items) = multi_span_document(owner, case.items_in_source_order.len());
    let mut styles = BTreeMap::from([(
        root.id(),
        LayoutStyle {
            display: LayoutDisplay::Grid,
            width: LayoutDimension::Fixed(ROOT_SIZE),
            height: LayoutDimension::Fixed(ROOT_SIZE),
            align_content: Some(LayoutAlignContent::FlexStart),
            justify_content: LayoutJustifyContent::FlexStart,
            ..LayoutStyle::default()
        },
    )]);
    let mut placements = BTreeMap::new();
    for (handle, spec) in items.iter().zip(case.items_in_source_order) {
        styles.insert(
            handle.id(),
            LayoutStyle {
                display: LayoutDisplay::Block,
                width: LayoutDimension::Fixed(ROOT_SIZE),
                height: LayoutDimension::Fixed(spec.height),
                ..LayoutStyle::default()
            },
        );
        placements.insert(
            handle.id(),
            GridItemInput {
                row_start: GridPlacementInput::Line(spec.row_start),
                row_end: GridPlacementInput::Line(spec.row_end),
                column_start: GridPlacementInput::Line(1),
                column_end: GridPlacementInput::Line(2),
            },
        );
    }
    let input = LayoutInput::from_host_document(
        &document.snapshot(),
        root,
        Viewport {
            width: ROOT_SIZE,
            height: ROOT_SIZE,
        },
        &styles,
        StyleRevision::default(),
        EnvironmentRevision::default(),
    )
    .unwrap()
    .with_grid_layout(
        BTreeMap::from([(
            root.id(),
            GridContainerInput {
                columns: vec![GridTrackSize::LengthPx(ROOT_SIZE)],
                rows: vec![
                    GridTrackSize::MinMax {
                        min: GridTrackMinSizingFunction::LengthPx(0.0),
                        max: GridTrackMaxSizingFunction::MaxContent,
                    };
                    4
                ],
                placement_mode: GridPlacementMode::TrackSizing,
                ..GridContainerInput::default()
            },
        )]),
        placements,
    );
    (input, root, items)
}

fn run_case(case: &ChromeRowAxisCase, expected_json: &Value, owner: u64) -> RowAxisObservation {
    let expected = row_axis_expectation(expected_json, case);
    let (input, root, items) = row_axis_input(case, owner);
    let output = TaffyLayoutEngine
        .compute(&input)
        .unwrap_or_else(|error| panic!("{}: {error:?}", case.id));
    let traces = output
        .grid_track_sizing_traces
        .iter()
        .filter(|trace| trace.node == root.id())
        .collect::<Vec<_>>();
    assert_eq!(traces.len(), 1, "{}: exactly one root trace", case.id);
    let trace = traces[0];

    let column_passes = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .collect::<Vec<_>>();
    assert_eq!(
        column_passes.len(),
        1,
        "{}: exactly one Columns control",
        case.id
    );
    let column_pass = column_passes[0];
    assert_eq!(
        column_pass
            .stages
            .iter()
            .filter(|stage| stage.stage == LayoutGridTrackSizingStageKind::FixedTrackEarlyReturn)
            .count(),
        1,
        "{}: fixed 220px Columns early return",
        case.id
    );
    assert!(
        column_pass.stages.iter().all(|stage| !matches!(
            stage.stage,
            LayoutGridTrackSizingStageKind::IntrinsicItemBatch { .. }
        )),
        "{}: fixed Columns control has no intrinsic batches",
        case.id
    );
    let fixed_column_stage = column_pass
        .stages
        .iter()
        .find(|stage| stage.stage == LayoutGridTrackSizingStageKind::FixedTrackEarlyReturn)
        .expect("the fixed Columns control has an early-return snapshot");
    let fixed_css_tracks = fixed_column_stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .collect::<Vec<_>>();
    assert_eq!(
        fixed_css_tracks.len(),
        1,
        "{}: fixed Columns control has one CSS track",
        case.id
    );
    assert_close(
        fixed_css_tracks[0].base_size,
        ROOT_SIZE,
        TRACE_TOLERANCE,
        case.id,
        "fixed Columns track size",
    );

    let row_passes = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .collect::<Vec<_>>();
    assert_eq!(row_passes.len(), 1, "{}: exactly one Rows pass", case.id);
    let row_pass = row_passes[0];
    let observed_spans = row_pass
        .stages
        .iter()
        .filter_map(|stage| match stage.stage {
            LayoutGridTrackSizingStageKind::IntrinsicItemBatch { span, .. } => Some(span),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        observed_spans, expected.span_batches,
        "{}: row-axis span batch order",
        case.id
    );
    assert_span_stage_snapshots_with_expectation(row_pass, case.id, &expected);
    assert_contributions(row_pass, case, &items);
    assert_frame(
        output.frames[&root.id()],
        [0.0, 0.0, ROOT_SIZE, ROOT_SIZE],
        CHROME_TOLERANCE,
        case.id,
        "root",
    );

    let item_frames = items
        .iter()
        .zip(case.items_in_source_order)
        .map(|(handle, spec)| {
            let frame = output.frames[&handle.id()];
            assert_frame(
                frame,
                [0.0, spec.chrome_local_y, ROOT_SIZE, spec.height],
                CHROME_TOLERANCE,
                case.id,
                spec.id,
            );
            (spec.id, frame)
        })
        .collect();

    RowAxisObservation {
        stages: row_pass.stages.clone(),
        item_frames,
    }
}

fn assert_contributions(
    pass: &crate::LayoutGridTrackSizingPassTrace,
    case: &ChromeRowAxisCase,
    items: &[HostNodeHandle],
) {
    let mut expected = Vec::new();
    for span in case.expected_span_batches {
        for kind in [
            LayoutGridTrackSizingContributionKind::Minimum,
            LayoutGridTrackSizingContributionKind::MinContent,
            LayoutGridTrackSizingContributionKind::MaxContent,
        ] {
            expected.extend(
                items
                    .iter()
                    .zip(case.items_in_source_order)
                    .filter(|(_, item)| (item.row_end - item.row_start) as u16 == *span)
                    .map(|(handle, item)| (handle.id(), kind, *span, item.height)),
            );
        }
    }
    assert_eq!(
        pass.contributions.len(),
        expected.len(),
        "{}: row-axis contribution count",
        case.id
    );
    for (actual, (item, kind, span, expected_height)) in pass.contributions.iter().zip(expected) {
        assert_eq!(
            (
                actual.item,
                actual.kind,
                actual.span,
                actual.crosses_flexible_track
            ),
            (item, kind, span, false),
            "{}: row-axis contribution selector",
            case.id
        );
        assert_close(
            actual.value,
            expected_height,
            TRACE_TOLERANCE,
            case.id,
            "contribution height",
        );
    }
}

fn assert_frame(actual: LayoutFrame, expected: [f32; 4], tolerance: f32, case: &str, label: &str) {
    for (actual, expected, property) in [
        (actual.x, expected[0], "x"),
        (actual.y, expected[1], "y"),
        (actual.width, expected[2], "width"),
        (actual.height, expected[3], "height"),
    ] {
        assert_close(
            actual,
            expected,
            tolerance,
            case,
            &format!("{label}.{property}"),
        );
    }
}

fn assert_close(actual: f32, expected: f32, tolerance: f32, case: &str, label: &str) {
    assert!(
        actual.is_finite() && (actual - expected).abs() <= tolerance,
        "{case}: {label} expected {expected} within {tolerance} CSS px, got {actual}"
    );
}

fn assert_source_order_equivalent(
    forward: &RowAxisObservation,
    reversed: &RowAxisObservation,
    case: &str,
) {
    assert_eq!(
        forward.stages.len(),
        reversed.stages.len(),
        "{case}: stage count changed with source order"
    );
    for (stage_index, (left, right)) in forward.stages.iter().zip(&reversed.stages).enumerate() {
        assert_eq!(
            left.stage, right.stage,
            "{case}: stage {stage_index} kind changed with source order"
        );
        assert_eq!(
            left.tracks.len(),
            right.tracks.len(),
            "{case}: stage {stage_index} track count changed with source order"
        );
        for (track_index, (left, right)) in left.tracks.iter().zip(&right.tracks).enumerate() {
            assert_eq!(
                (
                    left.internal_index,
                    left.is_gutter,
                    left.is_collapsed,
                    left.infinitely_growable,
                ),
                (
                    right.internal_index,
                    right.is_gutter,
                    right.is_collapsed,
                    right.infinitely_growable,
                ),
                "{case}: stage {stage_index} track {track_index} shape changed with source order"
            );
            assert_close(
                left.base_size,
                right.base_size,
                TRACE_TOLERANCE,
                case,
                "source-order base size",
            );
            match (left.growth_limit, right.growth_limit) {
                (None, None) => {}
                (Some(left), Some(right)) => assert_close(
                    left,
                    right,
                    TRACE_TOLERANCE,
                    case,
                    "source-order growth limit",
                ),
                (left, right) => {
                    panic!("{case}: source-order growth-limit shape differs: {left:?} vs {right:?}")
                }
            }
        }
    }

    assert_eq!(
        forward.item_frames.keys().collect::<Vec<_>>(),
        reversed.item_frames.keys().collect::<Vec<_>>(),
        "{case}: item set changed with source order"
    );
    for (item, left) in &forward.item_frames {
        let right = reversed.item_frames[item];
        assert_frame(
            *left,
            [right.x, right.y, right.width, right.height],
            TRACE_TOLERANCE,
            case,
            item,
        );
    }
}

#[test]
fn chrome156_row_axis_multi_span_stages_match_normative_oracle_and_source_order() {
    let expected_json: Value = serde_json::from_str(ROW_AXIS_EXPECTED_JSON)
        .unwrap_or_else(|error| panic!("row-axis expected.json: {error}"));
    assert_expected_scene_registry(&expected_json);

    let span2_forward = run_case(&CASES[0], &expected_json, 1_660);
    let span2_reversed = run_case(&CASES[1], &expected_json, 1_661);
    assert_source_order_equivalent(&span2_forward, &span2_reversed, "span-2");

    let span23_forward = run_case(&CASES[2], &expected_json, 1_662);
    let span23_reversed = run_case(&CASES[3], &expected_json, 1_663);
    assert_source_order_equivalent(&span23_forward, &span23_reversed, "span-2/3");
}
