use super::{
    project_grid_layout, project_grid_layout_with_math, project_grid_layout_with_reservation,
};
use crate::projection::typed_math::CssMathProjector;
use spinon_core::NodeId;
use spinon_layout::{GridTrackSize, LayoutError};
use spinon_style::{
    ComputedCssGridAutoFlow, ComputedCssGridAutoRows, ComputedCssGridLayout,
    ComputedCssGridTrackSize, ComputedStyleProfile,
};

#[test]
fn track_sizing_projects_axis_specific_auto_track_lists_and_math() {
    let node = NodeId::new(11_201).unwrap();
    let calc = spinon_style::ComputedCssMath::Sum(vec![
        spinon_style::ComputedCssMath::Percentage(0.1),
        spinon_style::ComputedCssMath::LengthPx(5.0),
    ]);
    let grid = ComputedCssGridLayout {
        columns: vec![ComputedCssGridTrackSize::LengthPx(24.0)],
        rows: vec![ComputedCssGridTrackSize::LengthPx(12.0)],
        columns_auto_repeat: None,
        rows_auto_repeat: None,
        columns_line_names: vec![Vec::new(), Vec::new()],
        rows_line_names: vec![Vec::new(), Vec::new()],
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::TrackList(vec![
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::Calc(calc.clone()),
        ]),
        auto_columns: spinon_style::ComputedCssGridAutoColumns::TrackList(vec![
            ComputedCssGridTrackSize::Fraction(1.0),
            ComputedCssGridTrackSize::Calc(calc),
        ]),
    };
    let mut math = CssMathProjector::default();
    let projected = project_grid_layout_with_math(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
        &mut math,
    )
    .unwrap();

    assert_eq!(
        projected.auto_rows,
        spinon_layout::GridAutoRows::TrackList(vec![
            GridTrackSize::MinContent,
            GridTrackSize::Calc(spinon_layout::LayoutCalcId(0)),
        ])
    );
    assert_eq!(
        projected.auto_columns,
        spinon_layout::GridAutoColumns::TrackList(vec![
            GridTrackSize::Fraction(1.0),
            GridTrackSize::Calc(spinon_layout::LayoutCalcId(1)),
        ])
    );
    let math = math.into_values();
    assert_eq!(
        math.iter().map(|value| value.property).collect::<Vec<_>>(),
        [
            spinon_layout::LayoutCssMathProperty::GridTemplateRows,
            spinon_layout::LayoutCssMathProperty::GridTemplateColumns,
        ]
    );
}

#[test]
fn non_track_sizing_profile_rejects_typed_auto_track_lists() {
    let node = NodeId::new(11_202).unwrap();
    let grid = ComputedCssGridLayout {
        columns: vec![ComputedCssGridTrackSize::LengthPx(24.0)],
        rows: vec![ComputedCssGridTrackSize::LengthPx(12.0)],
        columns_auto_repeat: None,
        rows_auto_repeat: None,
        columns_line_names: vec![Vec::new(), Vec::new()],
        rows_line_names: vec![Vec::new(), Vec::new()],
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::MinContent]),
        auto_columns: spinon_style::ComputedCssGridAutoColumns::Auto,
    };

    assert!(matches!(
        project_grid_layout(
            node,
            &grid,
            ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1,
        ),
        Err(crate::StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-auto-rows",
            ..
        }) if error_node == node
    ));
}

#[test]
fn auto_track_list_reservation_failure_keeps_axis_context() {
    let node = NodeId::new(11_203).unwrap();
    let grid = ComputedCssGridLayout {
        columns: vec![ComputedCssGridTrackSize::LengthPx(24.0)],
        rows: vec![ComputedCssGridTrackSize::LengthPx(12.0)],
        columns_auto_repeat: None,
        rows_auto_repeat: None,
        columns_line_names: vec![Vec::new(), Vec::new()],
        rows_line_names: vec![Vec::new(), Vec::new()],
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::TrackList(vec![
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::MaxContent,
        ]),
        auto_columns: spinon_style::ComputedCssGridAutoColumns::Auto,
    };
    let mut calls = 0;
    let error = project_grid_layout_with_reservation(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
        |_, _| {
            calls += 1;
            if calls == 3 { Err(()) } else { Ok(()) }
        },
    )
    .unwrap_err();

    assert!(
        matches!(
            error,
            crate::StyleLayoutError::Layout(LayoutError::GridAllocationFailure {
                node: error_node,
                property: "grid-auto-rows",
            }) if error_node == node
        ),
        "unexpected error: {error:?}"
    );
}
