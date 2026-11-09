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
