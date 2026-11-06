use super::super::project_grid_layout;
use super::{compute_expanded_track_count, validate_grid_grammar};
use spinon_core::NodeId;
use spinon_layout::{GRID_MAX_TRACKS, GridAutoRepeatMode, GridTrackSize};
use spinon_style::{
    ComputedCssGridAutoColumns, ComputedCssGridAutoFlow, ComputedCssGridAutoRepeat,
    ComputedCssGridAutoRepeatMode, ComputedCssGridAutoRows, ComputedCssGridLayout,
    ComputedCssGridTrackBreadth, ComputedCssGridTrackSize, ComputedStyleProfile,
};

fn repeated(minimum: f32) -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::MinMax {
        min: ComputedCssGridTrackBreadth::LengthPx(minimum),
        max: ComputedCssGridTrackBreadth::Fraction(1.0),
    }
}

fn grid_with_column_repeat(minima: &[f32]) -> ComputedCssGridLayout {
    ComputedCssGridLayout {
        columns: minima.iter().copied().map(repeated).collect(),
        rows: vec![ComputedCssGridTrackSize::LengthPx(24.0)],
        columns_fixed_repeat_group_count: 0,
        rows_fixed_repeat_group_count: 0,
        columns_auto_repeat: Some(ComputedCssGridAutoRepeat {
            mode: ComputedCssGridAutoRepeatMode::AutoFill,
            start: 0,
            end: minima.len(),
            line_names: Vec::new(),
            fixed_repeat_minmax_track_indices: Vec::new(),
        }),
        rows_auto_repeat: None,
        columns_line_names: vec![Vec::new(); minima.len() + 1],
        rows_line_names: vec![Vec::new(), Vec::new()],
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: ComputedCssGridAutoColumns::Auto,
    }
}

#[test]
fn count_matches_single_track_169_170px_boundary() {
    assert_eq!(compute_expanded_track_count(169.0, 10.0, &[80.0]), Some(1));
    assert_eq!(compute_expanded_track_count(170.0, 10.0, &[80.0]), Some(2));
}

#[test]
fn count_applies_ua_one_pixel_floor_only_to_repeat_count() {
    assert_eq!(compute_expanded_track_count(3.0, 0.0, &[0.5]), Some(3));
    assert_eq!(compute_expanded_track_count(100.0, 0.0, &[0.5]), Some(100));
}

#[test]
fn count_accounts_for_two_track_pattern_and_all_inter_track_gaps() {
    assert_eq!(
        compute_expanded_track_count(350.0, 10.0, &[80.0, 80.0]),
        Some(4)
    );
}

#[test]
fn zero_minimum_count_matches_single_and_mixed_pattern_reference_cases() {
    assert_eq!(compute_expanded_track_count(11.0, 0.0, &[0.0]), Some(11));
    assert_eq!(compute_expanded_track_count(11.0, 1.0, &[0.0]), Some(6));
    assert_eq!(compute_expanded_track_count(12.5, 0.5, &[0.0]), Some(8));
    assert_eq!(
        compute_expanded_track_count(19.0, 1.0, &[0.0, 0.0]),
        Some(10)
    );
    assert_eq!(
        compute_expanded_track_count(19.0, 1.0, &[0.0, 2.0]),
        Some(8)
    );
    assert_eq!(compute_expanded_track_count(7.0, 1.0, &[0.0]), Some(4));
}

#[test]
fn count_accepts_the_track_ceiling_and_rejects_counts_above_it() {
    assert_eq!(
        compute_expanded_track_count(GRID_MAX_TRACKS as f32, 0.0, &[0.0]),
        Some(GRID_MAX_TRACKS)
    );
    assert_eq!(
        compute_expanded_track_count(GRID_MAX_TRACKS as f32, 0.0, &[0.0, 0.0]),
        Some(GRID_MAX_TRACKS)
    );
    assert_eq!(
        compute_expanded_track_count((GRID_MAX_TRACKS + 1) as f32, 0.0, &[0.0]),
        None
    );
    assert_eq!(
        compute_expanded_track_count(20_000.0, 0.0, &[1.0, 1.0]),
        None
    );
    assert_eq!(
        compute_expanded_track_count(10_002.0, 0.0, &[0.0, 0.0]),
        None
    );
}

#[test]
fn count_rejects_non_finite_inputs_and_arithmetic_overflow() {
    assert_eq!(
        compute_expanded_track_count(f32::INFINITY, 0.0, &[1.0]),
        None
    );
    assert_eq!(
        compute_expanded_track_count(100.0, f32::MAX, &[1.0, 1.0]),
        None
    );
    assert_eq!(compute_expanded_track_count(100.0, 0.0, &[-0.01]), None);
    assert_eq!(compute_expanded_track_count(100.0, 0.0, &[f32::NAN]), None);
}

#[test]
fn root_html_grammar_accepts_one_or_two_named_free_tracks_on_one_axis() {
    let node = NodeId::new(31_101).unwrap();
    for minima in [&[40.0][..], &[40.0, 60.0][..]] {
        assert!(matches!(
            validate_grid_grammar(node, &grid_with_column_repeat(minima), false, false),
            Ok(true)
        ));
    }
}

#[test]
fn root_html_projection_reuses_typed_auto_repeat_track_conversion() {
    let node = NodeId::new(31_104).unwrap();
    let grid = grid_with_column_repeat(&[40.0]);
    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .unwrap();

    assert_eq!(
        projected.columns,
        vec![GridTrackSize::MinMax {
            min: spinon_layout::GridTrackMinSizingFunction::LengthPx(40.0),
            max: spinon_layout::GridTrackMaxSizingFunction::Fraction(1.0),
        }]
    );
    assert_eq!(
        projected.columns_auto_repeat.unwrap().mode,
        GridAutoRepeatMode::AutoFill
    );
}

#[test]
fn root_html_projection_preserves_zero_minimum_for_taffy_sizing() {
    let node = NodeId::new(31_105).unwrap();
    let projected = project_grid_layout(
        node,
        &grid_with_column_repeat(&[0.0]),
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .unwrap();

    assert_eq!(
        projected.columns,
        vec![GridTrackSize::MinMax {
            min: spinon_layout::GridTrackMinSizingFunction::LengthPx(0.0),
            max: spinon_layout::GridTrackMaxSizingFunction::Fraction(1.0),
        }]
    );
}

#[test]
fn root_html_grammar_rejects_neighbors_dense_flow_and_invalid_minimum() {
    let node = NodeId::new(31_102).unwrap();
    let mut with_neighbor = grid_with_column_repeat(&[40.0]);
    with_neighbor
        .columns
        .insert(0, ComputedCssGridTrackSize::LengthPx(4.0));
    with_neighbor.columns_auto_repeat.as_mut().unwrap().start = 1;
    with_neighbor.columns_auto_repeat.as_mut().unwrap().end = 2;
    assert!(validate_grid_grammar(node, &with_neighbor, false, false).is_err());

    let mut dense = grid_with_column_repeat(&[40.0]);
    dense.auto_flow = ComputedCssGridAutoFlow::RowDense;
    assert!(validate_grid_grammar(node, &dense, false, false).is_err());

    let invalid_minimum = grid_with_column_repeat(&[-0.01]);
    assert!(validate_grid_grammar(node, &invalid_minimum, false, false).is_err());
}

#[test]
fn root_html_grammar_rejects_repeats_on_both_axes_and_special_placement_routes() {
    let node = NodeId::new(31_103).unwrap();
    let mut both_axes = grid_with_column_repeat(&[40.0]);
    both_axes.rows_auto_repeat = Some(ComputedCssGridAutoRepeat {
        mode: ComputedCssGridAutoRepeatMode::AutoFit,
        start: 0,
        end: 1,
        line_names: Vec::new(),
        fixed_repeat_minmax_track_indices: Vec::new(),
    });
    assert!(validate_grid_grammar(node, &both_axes, false, false).is_err());
    assert!(validate_grid_grammar(node, &grid_with_column_repeat(&[40.0]), true, false).is_err());
    assert!(validate_grid_grammar(node, &grid_with_column_repeat(&[40.0]), false, true).is_err());
}
