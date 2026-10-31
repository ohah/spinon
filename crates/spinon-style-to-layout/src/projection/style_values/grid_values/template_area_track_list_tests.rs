use super::{project_grid_layout, project_grid_layout_with_reservation};
use spinon_core::NodeId;
use spinon_layout::{GridTrackSize, LayoutError};
use spinon_style::{
    ComputedCssGridArea, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridLayout, ComputedCssGridTemplateAreas,
    ComputedCssGridTrackBreadth, ComputedCssGridTrackSize, ComputedStyleProfile,
};

fn px(value: f32) -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::LengthPx(value)
}

fn area_grid(
    rows: Vec<ComputedCssGridTrackSize>,
    columns: Vec<ComputedCssGridTrackSize>,
    area_rows: usize,
    area_columns: usize,
    auto_rows: Vec<ComputedCssGridTrackSize>,
    auto_columns: Vec<ComputedCssGridTrackSize>,
) -> ComputedCssGridLayout {
    ComputedCssGridLayout {
        columns_line_names: vec![Vec::new(); columns.len() + 1],
        rows_line_names: vec![Vec::new(); rows.len() + 1],
        columns,
        rows,
        columns_auto_repeat: None,
        rows_auto_repeat: None,
        template_areas: Some(ComputedCssGridTemplateAreas {
            rows: area_rows,
            columns: area_columns,
            areas: vec![ComputedCssGridArea {
                name: "all".to_owned(),
                row_start: 0,
                row_end: area_rows,
                column_start: 0,
                column_end: area_columns,
            }],
        }),
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::TrackList(auto_rows),
        auto_columns: ComputedCssGridAutoColumns::TrackList(auto_columns),
    }
}

fn project(
    grid: &ComputedCssGridLayout,
) -> Result<spinon_layout::GridContainerInput, crate::StyleLayoutError> {
    project_grid_layout(
        NodeId::new(11_241).unwrap(),
        grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
    )
}

#[test]
fn template_area_lists_cycle_forward_and_rotate_for_later_implicit_tracks() {
    let grid = area_grid(
        vec![px(20.0)],
        vec![px(30.0)],
        4,
        4,
        vec![px(5.0), px(7.0)],
        vec![px(11.0), px(13.0)],
    );

    let projected = project(&grid).unwrap();
    assert_eq!(
        projected.rows,
        vec![
            GridTrackSize::LengthPx(20.0),
            GridTrackSize::LengthPx(5.0),
            GridTrackSize::LengthPx(7.0),
            GridTrackSize::LengthPx(5.0),
        ]
    );
    assert_eq!(
        projected.columns,
        vec![
            GridTrackSize::LengthPx(30.0),
            GridTrackSize::LengthPx(11.0),
            GridTrackSize::LengthPx(13.0),
            GridTrackSize::LengthPx(11.0),
        ]
    );
    assert_eq!(
        projected.auto_rows,
        spinon_layout::GridAutoRows::TrackList(vec![
            GridTrackSize::LengthPx(7.0),
            GridTrackSize::LengthPx(5.0),
        ])
    );
    assert_eq!(
        projected.auto_columns,
        spinon_layout::GridAutoColumns::TrackList(vec![
            GridTrackSize::LengthPx(13.0),
            GridTrackSize::LengthPx(11.0),
        ])
    );
    assert_eq!(projected.rows_line_names.len(), projected.rows.len() + 1);
    assert_eq!(
        projected.columns_line_names.len(),
        projected.columns.len() + 1
    );
}

#[test]
fn template_area_row_and_column_patterns_keep_independent_phase() {
    let rows = area_grid(
        vec![px(17.0), px(19.0)],
        vec![px(22.0), px(24.0)],
        5,
        2,
        vec![px(6.0), px(8.0)],
        vec![px(10.0), px(12.0)],
    );
    let projected_rows = project(&rows).unwrap();
    assert_eq!(
        projected_rows.rows,
        vec![
            GridTrackSize::LengthPx(17.0),
            GridTrackSize::LengthPx(19.0),
            GridTrackSize::LengthPx(6.0),
            GridTrackSize::LengthPx(8.0),
            GridTrackSize::LengthPx(6.0),
        ]
    );
    assert_eq!(projected_rows.columns.len(), 2);
    assert_eq!(
        projected_rows.auto_rows,
        spinon_layout::GridAutoRows::TrackList(vec![
            GridTrackSize::LengthPx(8.0),
            GridTrackSize::LengthPx(6.0),
        ])
    );
    assert_eq!(
        projected_rows.auto_columns,
        spinon_layout::GridAutoColumns::TrackList(vec![
            GridTrackSize::LengthPx(10.0),
            GridTrackSize::LengthPx(12.0),
        ])
    );

    let columns = area_grid(
        vec![px(12.0), px(14.0)],
        vec![px(21.0), px(23.0), px(25.0)],
        2,
        5,
        vec![px(4.0), px(5.0)],
        vec![px(9.0), px(10.0)],
    );
    let projected_columns = project(&columns).unwrap();
    assert_eq!(projected_columns.rows.len(), 2);
    assert_eq!(
        projected_columns.columns,
        vec![
            GridTrackSize::LengthPx(21.0),
            GridTrackSize::LengthPx(23.0),
            GridTrackSize::LengthPx(25.0),
            GridTrackSize::LengthPx(9.0),
            GridTrackSize::LengthPx(10.0),
        ]
    );
    assert_eq!(
        projected_columns.auto_rows,
        spinon_layout::GridAutoRows::TrackList(vec![
            GridTrackSize::LengthPx(4.0),
            GridTrackSize::LengthPx(5.0),
        ])
    );
    assert_eq!(
        projected_columns.auto_columns,
        spinon_layout::GridAutoColumns::TrackList(vec![
            GridTrackSize::LengthPx(9.0),
            GridTrackSize::LengthPx(10.0),
        ])
    );
}

#[test]
fn template_area_lists_do_not_change_authored_tracks_when_no_extension_is_needed() {
    let grid = area_grid(
        vec![px(15.0), px(16.0), px(17.0)],
        vec![px(18.0), px(19.0), px(20.0)],
        2,
        2,
        vec![px(5.0), px(7.0)],
        vec![px(9.0), px(11.0)],
    );

    let projected = project(&grid).unwrap();
    assert_eq!(
        projected.rows,
        vec![
            GridTrackSize::LengthPx(15.0),
            GridTrackSize::LengthPx(16.0),
            GridTrackSize::LengthPx(17.0),
        ]
    );
    assert_eq!(
        projected.columns,
        vec![
            GridTrackSize::LengthPx(18.0),
            GridTrackSize::LengthPx(19.0),
            GridTrackSize::LengthPx(20.0),
        ]
    );
    assert_eq!(
        projected.auto_rows,
        spinon_layout::GridAutoRows::TrackList(vec![
            GridTrackSize::LengthPx(5.0),
            GridTrackSize::LengthPx(7.0),
        ])
    );
    assert_eq!(
        projected.auto_columns,
        spinon_layout::GridAutoColumns::TrackList(vec![
            GridTrackSize::LengthPx(9.0),
            GridTrackSize::LengthPx(11.0),
        ])
    );
}

#[test]
fn template_area_typed_track_lists_cycle_and_preserve_later_implicit_phase() {
    let minmax = ComputedCssGridTrackSize::MinMax {
        min: ComputedCssGridTrackBreadth::LengthPx(20.0),
        max: ComputedCssGridTrackBreadth::Fraction(1.0),
    };
    let grid = area_grid(
        vec![px(20.0)],
        vec![px(30.0), px(31.0)],
        2,
        3,
        vec![minmax.clone(), ComputedCssGridTrackSize::MinContent],
        vec![
            ComputedCssGridTrackSize::Percentage(0.25),
            ComputedCssGridTrackSize::Fraction(1.0),
        ],
    );

    let projected = project(&grid).unwrap();
    assert_eq!(
        projected.rows,
        vec![
            GridTrackSize::LengthPx(20.0),
            GridTrackSize::MinMax {
                min: spinon_layout::GridTrackMinSizingFunction::LengthPx(20.0),
                max: spinon_layout::GridTrackMaxSizingFunction::Fraction(1.0),
            },
        ]
    );
    assert_eq!(
        projected.columns,
        vec![
            GridTrackSize::LengthPx(30.0),
            GridTrackSize::LengthPx(31.0),
            GridTrackSize::Percentage(0.25),
        ]
    );
    assert_eq!(
        projected.auto_rows,
        spinon_layout::GridAutoRows::TrackList(vec![
            GridTrackSize::MinContent,
            GridTrackSize::MinMax {
                min: spinon_layout::GridTrackMinSizingFunction::LengthPx(20.0),
                max: spinon_layout::GridTrackMaxSizingFunction::Fraction(1.0),
            },
        ])
    );
    assert_eq!(
        projected.auto_columns,
        spinon_layout::GridAutoColumns::TrackList(vec![
            GridTrackSize::Fraction(1.0),
            GridTrackSize::Percentage(0.25),
        ])
    );
    assert_eq!(projected.rows_line_names.len(), projected.rows.len() + 1);
    assert_eq!(
        projected.columns_line_names.len(),
        projected.columns.len() + 1
    );
}

#[test]
fn template_area_track_lists_reject_empty_but_project_typed_values() {
    let node = NodeId::new(11_242).unwrap();
    let mut empty = area_grid(vec![px(20.0)], vec![px(30.0)], 2, 2, vec![], vec![px(9.0)]);
    assert!(matches!(
        project_grid_layout(
            node,
            &empty,
            ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
        ),
        Err(crate::StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-auto-rows",
            ..
        }) if error_node == node
    ));

    empty.auto_rows =
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::MinContent]);
    empty.auto_columns =
        ComputedCssGridAutoColumns::TrackList(vec![ComputedCssGridTrackSize::Percentage(0.5)]);
    let projected = project_grid_layout(
        node,
        &empty,
        ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
    )
    .unwrap();
    assert_eq!(
        projected.auto_rows,
        spinon_layout::GridAutoRows::TrackList(vec![GridTrackSize::MinContent])
    );
    assert_eq!(
        projected.auto_columns,
        spinon_layout::GridAutoColumns::TrackList(vec![GridTrackSize::Percentage(0.5)])
    );

    empty.template_areas = None;
    assert!(matches!(
        project_grid_layout(
            node,
            &empty,
            ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
        ),
        Err(crate::StyleLayoutError::UnsupportedComputedValue {
            property: "grid-auto-rows",
            ..
        })
    ));
}

#[test]
fn template_area_track_reservation_failures_keep_axis_context_and_recover() {
    let node = NodeId::new(11_243).unwrap();
    let grid = area_grid(
        vec![px(20.0)],
        vec![px(30.0)],
        2,
        2,
        vec![px(5.0), px(7.0)],
        vec![px(11.0), px(13.0)],
    );

    for (failure_call, property) in [(5, "grid-auto-columns"), (6, "grid-auto-rows")] {
        let mut calls = 0;
        let error = project_grid_layout_with_reservation(
            node,
            &grid,
            ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
            |tracks, count| {
                calls += 1;
                if calls == failure_call {
                    Err(())
                } else {
                    tracks.try_reserve_exact(count).map_err(|_| ())
                }
            },
        )
        .unwrap_err();

        assert!(
            matches!(
                error,
                crate::StyleLayoutError::Layout(LayoutError::GridAllocationFailure {
                    node: error_node,
                    property: actual_property,
                }) if error_node == node && actual_property == property
            ),
            "unexpected reservation error: {error:?}"
        );
        assert!(
            project(&grid).is_ok(),
            "projection must recover after {property}"
        );
    }
}
