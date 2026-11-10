use super::*;

fn tracks(base_sizes: &[f32]) -> Vec<crate::LayoutGridTrackSizingTrackState> {
    base_sizes
        .iter()
        .enumerate()
        .map(
            |(index, base_size)| crate::LayoutGridTrackSizingTrackState {
                internal_index: (index as u16) * 2 + 1,
                is_gutter: false,
                is_collapsed: false,
                base_size: *base_size,
                growth_limit: Some(*base_size),
                infinitely_growable: false,
            },
        )
        .collect()
}

fn container() -> crate::GridContainerInput {
    crate::GridContainerInput {
        auto_rows: crate::GridAutoRows::TrackList(vec![crate::GridTrackSize::Auto]),
        ..crate::GridContainerInput::default()
    }
}

fn frame() -> crate::LayoutFrame {
    crate::LayoutFrame {
        x: 0.0,
        y: 0.0,
        width: 40.0,
        height: 220.0,
    }
}

#[test]
fn blink_row_distribution_preserves_intrinsic_fractional_growth_on_unconstrained_tracks() {
    let row_sizes = [10.0, 0.0, 0.0, 40.0, 20.0, 20.0, 13.333333];
    let tracks = tracks(&row_sizes);
    assert_eq!(
        blink_layout_unit_distribute_rows_from_stages(
            &container(),
            frame(),
            &row_sizes,
            &tracks,
            &tracks,
            ResidualRounding::Up,
        ),
        Some(vec![640, 0, 0, 2560, 1280, 1280, 854])
    );
}

#[test]
fn blink_row_distribution_does_not_exceed_a_saturated_fit_content_limit() {
    let row_sizes = [0.75 / 64.0];
    let tracks = tracks(&row_sizes);
    let mut container = container();
    container.auto_rows = crate::GridAutoRows::TrackList(vec![crate::GridTrackSize::FitContent(
        crate::GridTrackLengthPercentage::LengthPx(row_sizes[0]),
    )]);
    assert_eq!(
        blink_layout_unit_distribute_rows_from_stages(
            &container,
            frame(),
            &row_sizes,
            &tracks,
            &tracks,
            ResidualRounding::Up,
        ),
        None
    );
}

#[test]
fn blink_nearest_row_rounding_respects_fit_content_cap_tolerance() {
    use crate::{GridAutoRows, GridTrackLengthPercentage, GridTrackSize};

    let accepted_row_size = 0.105;
    let accepted_rows = [accepted_row_size];
    let accepted_tracks = tracks(&accepted_rows);
    let accepted_container = crate::GridContainerInput {
        auto_rows: GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
            GridTrackLengthPercentage::LengthPx(accepted_row_size),
        )]),
        ..crate::GridContainerInput::default()
    };
    assert_eq!(
        blink_layout_unit_distribute_rows_from_stages(
            &accepted_container,
            frame(),
            &accepted_rows,
            &accepted_tracks,
            &accepted_tracks,
            ResidualRounding::Nearest,
        ),
        Some(vec![7]),
        "nearest rounding may exceed a min-forced base cap by less than 0.01px"
    );

    let rounded_down_row_size = 0.098;
    let rounded_down_rows = [rounded_down_row_size];
    let rounded_down_tracks = tracks(&rounded_down_rows);
    let rounded_down_container = crate::GridContainerInput {
        auto_rows: GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
            GridTrackLengthPercentage::LengthPx(rounded_down_row_size),
        )]),
        ..crate::GridContainerInput::default()
    };
    assert_eq!(
        blink_layout_unit_distribute_rows_from_stages(
            &rounded_down_container,
            frame(),
            &rounded_down_rows,
            &rounded_down_tracks,
            &rounded_down_tracks,
            ResidualRounding::Nearest,
        ),
        Some(vec![6]),
        "a row rounded down needs no cap growth allowance"
    );
}

#[test]
fn blink_nearest_rounding_skips_a_rejected_row_and_distributes_to_an_eligible_row() {
    use crate::{GridAutoRows, GridTrackLengthPercentage, GridTrackSize};

    let row_sizes = [0.098, 0.105];
    let tracks = tracks(&row_sizes);
    let container = crate::GridContainerInput {
        auto_rows: GridAutoRows::TrackList(
            row_sizes
                .iter()
                .copied()
                .map(|size| GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(size)))
                .collect(),
        ),
        ..crate::GridContainerInput::default()
    };
    assert_eq!(
        blink_layout_unit_distribute_rows_from_stages(
            &container,
            frame(),
            &row_sizes,
            &tracks,
            &tracks,
            ResidualRounding::Nearest,
        ),
        Some(vec![6, 7]),
        "rounding should skip the first row and assign the required unit to the second"
    );
}

#[test]
fn blink_row_distribution_rounds_fractional_spanning_rows_within_point_zero_one_px() {
    use crate::{GridAutoRows, GridTrackLengthPercentage, GridTrackSize};
    let row_sizes = [10.0, 4.1666665, 19.166666, 16.666666];
    let tracks = tracks(&row_sizes);
    let container = crate::GridContainerInput {
        rows: vec![GridTrackSize::LengthPx(10.0)],
        auto_rows: GridAutoRows::TrackList(vec![
            GridTrackSize::MaxContent,
            GridTrackSize::MaxContent,
            GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(20.0)),
            GridTrackSize::Auto,
            GridTrackSize::MinContent,
            GridTrackSize::MinContent,
        ]),
        ..crate::GridContainerInput::default()
    };
    assert_eq!(
        blink_layout_unit_distribute_rows_from_stages(
            &container,
            frame(),
            &row_sizes,
            &tracks,
            &tracks,
            ResidualRounding::Nearest,
        ),
        Some(vec![640, 266, 1227, 1067])
    );
}

#[test]
fn blink_row_distribution_rounds_minimum_forced_fit_content_base_above_its_cap() {
    use crate::{GridAutoRows, GridTrackLengthPercentage, GridTrackSize};
    let row_sizes = [10.0, 4.1666665, 19.166666, 16.666666];
    let tracks = tracks(&row_sizes);
    let container = crate::GridContainerInput {
        rows: vec![GridTrackSize::LengthPx(10.0)],
        auto_rows: GridAutoRows::TrackList(vec![
            GridTrackSize::MaxContent,
            GridTrackSize::MaxContent,
            GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(0.0)),
            GridTrackSize::Auto,
            GridTrackSize::MinContent,
            GridTrackSize::MinContent,
        ]),
        ..crate::GridContainerInput::default()
    };
    assert_eq!(
        blink_layout_unit_distribute_rows_from_stages(
            &container,
            frame(),
            &row_sizes,
            &tracks,
            &tracks,
            ResidualRounding::Nearest,
        ),
        Some(vec![640, 266, 1227, 1067])
    );
}

#[test]
fn blink_stretch_auto_rows_assigns_an_indivisible_unit_to_the_later_row() {
    let container = crate::GridContainerInput {
        rows: vec![crate::GridTrackSize::LengthPx(10.0)],
        auto_rows: crate::GridAutoRows::TrackList(vec![
            crate::GridTrackSize::Auto,
            crate::GridTrackSize::Auto,
        ]),
        ..crate::GridContainerInput::default()
    };
    assert_eq!(
        blink_layout_unit_stretch_auto_rows(&container, 10.0 + 1.0 / 64.0, vec![640, 0, 0]),
        Some(vec![640, 0, 1])
    );
}

#[test]
fn blink_stretch_stage_keeps_a_used_prefix_without_auto_rows_unchanged() {
    let auto_rows = vec![
        crate::GridTrackSize::MaxContent,
        crate::GridTrackSize::FitContent(crate::GridTrackLengthPercentage::LengthPx(20.0)),
        crate::GridTrackSize::MaxContent,
        crate::GridTrackSize::Auto,
        crate::GridTrackSize::MinContent,
        crate::GridTrackSize::MinContent,
    ];
    let container = crate::GridContainerInput {
        rows: vec![crate::GridTrackSize::LengthPx(10.0)],
        auto_rows: crate::GridAutoRows::TrackList(auto_rows.clone()),
        ..crate::GridContainerInput::default()
    };
    let row_sizes = [10.0, 20.0, 0.0, 30.0];
    let before = tracks(&row_sizes);
    assert!(validate_auto_track_alignment(
        &before, &before, &auto_rows, true, 220.0
    ));
    assert_eq!(
        blink_layout_unit_stretch_auto_rows(&container, 220.0, vec![640, 1280, 0, 1920]),
        Some(vec![640, 1280, 0, 1920])
    );
}

#[test]
fn blink_row_distribution_prefers_an_auto_track_for_an_equal_growth_remainder() {
    let row_sizes = [13.333333; 3];
    let tracks = tracks(&row_sizes);
    let container = crate::GridContainerInput {
        auto_rows: crate::GridAutoRows::TrackList(vec![
            crate::GridTrackSize::FitContent(crate::GridTrackLengthPercentage::LengthPx(44.0)),
            crate::GridTrackSize::Auto,
            crate::GridTrackSize::FitContent(crate::GridTrackLengthPercentage::LengthPx(264.0)),
        ]),
        ..crate::GridContainerInput::default()
    };
    assert_eq!(
        blink_layout_unit_distribute_rows_from_stages(
            &container,
            frame(),
            &row_sizes,
            &tracks,
            &tracks,
            ResidualRounding::Up,
        ),
        Some(vec![853, 854, 853])
    );
}

fn two_fit_container() -> crate::GridContainerInput {
    use crate::{GridAutoRows, GridTrackLengthPercentage, GridTrackSize};
    crate::GridContainerInput {
        auto_rows: GridAutoRows::TrackList(vec![
            GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(10.0)),
            GridTrackSize::MinContent,
            GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.2)),
            GridTrackSize::MinContent,
            GridTrackSize::MaxContent,
            GridTrackSize::Auto,
        ]),
        ..crate::GridContainerInput::default()
    }
}

fn two_fit_trace(grid: NodeId, row_sizes: &[f32]) -> crate::LayoutGridTrackSizingTrace {
    use crate::{
        LayoutGridTrackSizingAxis, LayoutGridTrackSizingPassTrace,
        LayoutGridTrackSizingResolvedAxis, LayoutGridTrackSizingStageKind,
        LayoutGridTrackSizingStageTrace, LayoutGridTrackSizingTrackState,
    };
    let tracks = row_sizes
        .iter()
        .enumerate()
        .map(|(index, base_size)| LayoutGridTrackSizingTrackState {
            internal_index: (index as u16) * 2 + 1,
            is_gutter: false,
            is_collapsed: false,
            base_size: *base_size,
            growth_limit: Some(*base_size),
            infinitely_growable: false,
        })
        .collect::<Vec<_>>();
    crate::LayoutGridTrackSizingTrace {
        node: grid,
        passes: vec![LayoutGridTrackSizingPassTrace {
            sequence: 1,
            axis: LayoutGridTrackSizingAxis::Rows,
            stages: vec![
                LayoutGridTrackSizingStageTrace {
                    stage: LayoutGridTrackSizingStageKind::IntrinsicSizingComplete,
                    tracks: tracks.clone(),
                },
                LayoutGridTrackSizingStageTrace {
                    stage: LayoutGridTrackSizingStageKind::MaximizeTracks,
                    tracks: tracks.clone(),
                },
                LayoutGridTrackSizingStageTrace {
                    stage: LayoutGridTrackSizingStageKind::StretchAutoTracks { applied: false },
                    tracks,
                },
            ],
            contributions: Vec::new(),
            #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
            fixture_phase: None,
            #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
            fixture_auto_repeat_count: None,
            #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
            auto_repeat_count: None,
            #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
            fixture_percentage_basis: None,
        }],
        used_columns: LayoutGridTrackSizingResolvedAxis::default(),
        used_rows: LayoutGridTrackSizingResolvedAxis {
            used_track_list: row_sizes
                .iter()
                .map(|size| format!("{size}px"))
                .collect::<Vec<_>>()
                .join(" "),
            explicit_track_count: 1,
            negative_implicit_track_count: 0,
            positive_implicit_track_count: u16::try_from(row_sizes.len() - 1).unwrap(),
            collapsed_track_indices: Vec::new(),
        },
    }
}

fn two_fit_output(traces: Vec<crate::LayoutGridTrackSizingTrace>) -> crate::LayoutOutput {
    crate::LayoutOutput {
        grid_track_sizing_traces: traces,
        ..crate::LayoutOutput::default()
    }
}

#[test]
fn repeated_text_two_fit_auto_placement_accepts_a_complete_ordered_trace() {
    let grid = NodeId::new(1).unwrap();
    let row_sizes = [10.0, 0.0, 20.0, 20.0, 30.0, 20.0, 20.0, 0.0, 10.0];
    let trace = two_fit_trace(grid, &row_sizes);
    let output = two_fit_output(vec![trace]);
    assert_eq!(
        repeated_text_final_row_track_sizes(
            &output,
            grid,
            row_sizes.len(),
            220.0,
            &two_fit_container(),
        ),
        Some(row_sizes.to_vec())
    );
}

#[test]
fn repeated_text_two_fit_auto_placement_rejects_missing_duplicate_and_wrong_owner_trace() {
    let grid = NodeId::new(1).unwrap();
    let other = NodeId::new(2).unwrap();
    let row_sizes = [10.0, 0.0, 20.0, 20.0, 30.0, 20.0, 20.0, 0.0, 10.0];
    let container = two_fit_container();
    for (label, traces) in [
        ("missing", Vec::new()),
        ("wrong owner", vec![two_fit_trace(other, &row_sizes)]),
        (
            "duplicate same-owner trace",
            vec![
                two_fit_trace(grid, &row_sizes),
                two_fit_trace(grid, &row_sizes),
            ],
        ),
    ] {
        assert_eq!(
            repeated_text_final_row_track_sizes(
                &two_fit_output(traces),
                grid,
                row_sizes.len(),
                220.0,
                &container,
            ),
            None,
            "{label} trace must fail closed"
        );
    }
}

#[test]
fn repeated_text_two_fit_auto_placement_rejects_out_of_order_and_incomplete_trace_stages() {
    let grid = NodeId::new(1).unwrap();
    let row_sizes = [10.0, 0.0, 20.0, 20.0, 30.0, 20.0, 20.0, 0.0, 10.0];
    let container = two_fit_container();
    let mut out_of_order_track = two_fit_trace(grid, &row_sizes);
    out_of_order_track.passes[0].stages[2].tracks[3].internal_index = 1;
    let mut missing_stage = two_fit_trace(grid, &row_sizes);
    missing_stage.passes[0].stages.remove(0);
    let mut duplicate_stage = two_fit_trace(grid, &row_sizes);
    let repeated_stage = duplicate_stage.passes[0].stages[1].clone();
    duplicate_stage.passes[0].stages.insert(2, repeated_stage);
    let mut mismatched_used_rows = two_fit_trace(grid, &row_sizes);
    mismatched_used_rows.used_rows.used_track_list = "10px 20px 0px".to_owned();
    for (label, trace) in [
        ("out-of-order internal track index", out_of_order_track),
        ("missing intrinsic stage", missing_stage),
        ("duplicate maximize stage", duplicate_stage),
        ("used row count mismatch", mismatched_used_rows),
    ] {
        assert_eq!(
            repeated_text_final_row_track_sizes(
                &two_fit_output(vec![trace]),
                grid,
                row_sizes.len(),
                220.0,
                &container,
            ),
            None,
            "{label} must fail closed"
        );
    }
}

#[test]
fn repeated_text_two_fit_auto_placement_rejects_stale_revision_without_mutating_output() {
    use crate::{
        GridAutoRows, GridContainerInput, GridPlacementMode, GridTrackLengthPercentage,
        GridTrackSize, LayoutInput, LayoutInputRevision, LayoutNode, LayoutSourceRevision,
        LayoutStyle, RootSizingPolicy, Viewport,
    };
    use spinon_core::{EnvironmentRevision, Revision, StyleRevision};
    use std::collections::BTreeMap;
    let grid = NodeId::new(1).unwrap();
    let revision = LayoutInputRevision::new(
        LayoutSourceRevision::Tree(Revision::default()),
        StyleRevision::default(),
        EnvironmentRevision::default(),
    );
    let container = GridContainerInput {
        placement_mode: GridPlacementMode::RootHtmlFitContentAutoRowsV1,
        auto_rows: GridAutoRows::TrackList(vec![
            GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(10.0)),
            GridTrackSize::MinContent,
            GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.2)),
            GridTrackSize::MinContent,
            GridTrackSize::MaxContent,
            GridTrackSize::Auto,
        ]),
        ..GridContainerInput::default()
    };
    let input = LayoutInput {
        root: grid,
        revision,
        viewport: Viewport {
            width: 40.0,
            height: 220.0,
        },
        root_sizing: RootSizingPolicy::Match,
        nodes: vec![LayoutNode {
            id: grid,
            children: Vec::new(),
            style: LayoutStyle::default(),
        }],
        css_math: Vec::new(),
        positioning: BTreeMap::new(),
        grid_containers: BTreeMap::from([(grid, container)]),
        grid_items: BTreeMap::new(),
        text_paragraphs: BTreeMap::new(),
    };
    let mut output = crate::LayoutOutput {
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default().checked_next().unwrap(),
            EnvironmentRevision::default(),
        ),
        ..crate::LayoutOutput::default()
    };
    let before = output.clone();
    assert!(normalize_layout_frames(&input, &mut output).is_err());
    assert_eq!(
        output, before,
        "stale input revision must not mutate output"
    );
}
