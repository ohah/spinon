use super::project_grid_layout;
use spinon_core::NodeId;
use spinon_layout::{
    GridAutoRepeat, GridTrackMaxSizingFunction, GridTrackMinSizingFunction, GridTrackSize,
};
use spinon_style::{
    ComputedCssGridAutoFlow, ComputedCssGridAutoRepeat, ComputedCssGridAutoRepeatMode,
    ComputedCssGridAutoRows, ComputedCssGridLayout, ComputedCssGridTrackBreadth,
    ComputedCssGridTrackSize, ComputedCssMath, ComputedStyleProfile,
};

#[test]
fn intrinsic_minimum_fixed_maximum_auto_repeat_tracks_are_preserved_by_projection() {
    let node = NodeId::new(11_141).unwrap();
    let minima = [
        (
            ComputedCssGridTrackBreadth::Auto,
            GridTrackMinSizingFunction::Auto,
        ),
        (
            ComputedCssGridTrackBreadth::MinContent,
            GridTrackMinSizingFunction::MinContent,
        ),
        (
            ComputedCssGridTrackBreadth::MaxContent,
            GridTrackMinSizingFunction::MaxContent,
        ),
    ];
    let maxima = [
        (
            ComputedCssGridTrackBreadth::LengthPx(40.0),
            GridTrackMaxSizingFunction::LengthPx(40.0),
        ),
        (
            ComputedCssGridTrackBreadth::Percentage(0.3),
            GridTrackMaxSizingFunction::Percentage(0.3),
        ),
    ];

    for (computed_minimum, expected_minimum) in &minima {
        for (computed_maximum, expected_maximum) in maxima.iter().cloned() {
            let grid = ComputedCssGridLayout {
                columns: vec![ComputedCssGridTrackSize::MinMax {
                    min: computed_minimum.clone(),
                    max: computed_maximum,
                }],
                rows: vec![ComputedCssGridTrackSize::LengthPx(20.0)],
                columns_auto_repeat: Some(ComputedCssGridAutoRepeat {
                    mode: ComputedCssGridAutoRepeatMode::AutoFill,
                    start: 0,
                    end: 1,
                    line_names: vec![Vec::new(), Vec::new()],
                    fixed_repeat_minmax_track_indices: Vec::new(),
                }),
                rows_auto_repeat: None,
                columns_line_names: vec![Vec::new(), Vec::new()],
                rows_line_names: vec![Vec::new(), Vec::new()],
                template_areas: None,
                auto_flow: ComputedCssGridAutoFlow::Row,
                auto_rows: ComputedCssGridAutoRows::Auto,
                auto_columns: spinon_style::ComputedCssGridAutoColumns::Auto,
            };

            let projected = project_grid_layout(
                node,
                &grid,
                ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
            )
            .unwrap();

            assert_eq!(
                projected.columns,
                vec![GridTrackSize::MinMax {
                    min: *expected_minimum,
                    max: expected_maximum,
                }]
            );
            assert!(matches!(
                projected.columns_auto_repeat,
                Some(GridAutoRepeat {
                    mode: _,
                    start: 0,
                    end: 1,
                    ..
                })
            ));
        }

        let grid = ComputedCssGridLayout {
            columns: vec![ComputedCssGridTrackSize::MinMax {
                min: computed_minimum.clone(),
                max: ComputedCssGridTrackBreadth::Calc(ComputedCssMath::Sum(vec![
                    ComputedCssMath::LengthPx(15.0),
                    ComputedCssMath::Percentage(0.25),
                ])),
            }],
            rows: vec![ComputedCssGridTrackSize::LengthPx(20.0)],
            columns_auto_repeat: Some(ComputedCssGridAutoRepeat {
                mode: ComputedCssGridAutoRepeatMode::AutoFit,
                start: 0,
                end: 1,
                line_names: vec![Vec::new(), Vec::new()],
                fixed_repeat_minmax_track_indices: Vec::new(),
            }),
            rows_auto_repeat: None,
            columns_line_names: vec![Vec::new(), Vec::new()],
            rows_line_names: vec![Vec::new(), Vec::new()],
            template_areas: None,
            auto_flow: ComputedCssGridAutoFlow::Row,
            auto_rows: ComputedCssGridAutoRows::Auto,
            auto_columns: spinon_style::ComputedCssGridAutoColumns::Auto,
        };

        let projected = project_grid_layout(
            node,
            &grid,
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
        )
        .unwrap();

        assert!(matches!(
            projected.columns.as_slice(),
            [GridTrackSize::MinMax {
                min,
                max: GridTrackMaxSizingFunction::Calc(_),
            }] if min == expected_minimum
        ));
        assert!(matches!(
            projected.columns_auto_repeat,
            Some(GridAutoRepeat {
                mode: _,
                start: 0,
                end: 1,
                ..
            })
        ));
    }
}
