use super::*;

#[test]
fn root_html_column_flow_projects_direct_fraction_auto_column_lists() {
    let node = NodeId::new(11_247).unwrap();
    let mut input = grid(
        vec![ComputedCssGridTrackSize::LengthPx(30.0)],
        vec![ComputedCssGridTrackSize::LengthPx(20.0)],
    );
    input.auto_flow = ComputedCssGridAutoFlow::Column;
    input.auto_columns = ComputedCssGridAutoColumns::TrackList(vec![
        ComputedCssGridTrackSize::Fraction(1.0),
        ComputedCssGridTrackSize::LengthPx(10.0),
        ComputedCssGridTrackSize::Fraction(2.0),
    ]);

    let projected = project_grid_layout(
        node,
        &input,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("finite direct fr tracks should project for parsed Root HTML column flow");

    assert_eq!(
        projected.auto_columns,
        spinon_layout::GridAutoColumns::TrackList(vec![
            GridTrackSize::Fraction(1.0),
            GridTrackSize::LengthPx(10.0),
            GridTrackSize::Fraction(2.0),
        ])
    );
}
