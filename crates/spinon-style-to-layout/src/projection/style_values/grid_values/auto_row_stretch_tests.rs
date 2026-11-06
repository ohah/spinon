use super::*;

fn candidate_grid() -> ComputedCssGridLayout {
    ComputedCssGridLayout {
        columns: vec![ComputedCssGridTrackSize::LengthPx(40.0)],
        rows: vec![ComputedCssGridTrackSize::LengthPx(10.0)],
        columns_fixed_repeat_group_count: 0,
        rows_fixed_repeat_group_count: 0,
        columns_auto_repeat: None,
        rows_auto_repeat: None,
        columns_line_names: vec![Vec::new(); 2],
        rows_line_names: vec![Vec::new(); 2],
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: ComputedCssGridAutoColumns::Auto,
    }
}

#[test]
fn root_html_auto_row_stretch_candidate_requires_the_fixed_single_track_grid() {
    assert!(root_html_auto_row_stretch_candidate(&candidate_grid()));

    let mut invalid = candidate_grid();
    invalid.auto_rows =
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::MinContent]);
    assert!(!root_html_auto_row_stretch_candidate(&invalid));

    let mut invalid = candidate_grid();
    invalid
        .columns
        .push(ComputedCssGridTrackSize::LengthPx(10.0));
    assert!(!root_html_auto_row_stretch_candidate(&invalid));

    let mut invalid = candidate_grid();
    invalid.rows[0] = ComputedCssGridTrackSize::LengthPx(10.5);
    assert!(!root_html_auto_row_stretch_candidate(&invalid));

    let mut invalid = candidate_grid();
    invalid.auto_flow = ComputedCssGridAutoFlow::Column;
    assert!(!root_html_auto_row_stretch_candidate(&invalid));

    let mut invalid = candidate_grid();
    invalid.auto_columns = ComputedCssGridAutoColumns::LengthPx(10.0);
    assert!(!root_html_auto_row_stretch_candidate(&invalid));

    let mut invalid = candidate_grid();
    invalid.columns_fixed_repeat_group_count = 1;
    assert!(!root_html_auto_row_stretch_candidate(&invalid));
}
