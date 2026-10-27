use super::*;

#[test]
fn track_sizing_profile_preserves_definite_percentage_auto_repeat_tracks() {
    for (axis, mode) in [("columns", "auto-fit"), ("rows", "auto-fill")] {
        let repeated_axis = format!("grid-template-{axis}:repeat({mode},30%)");
        let other_axis = if axis == "columns" {
            "grid-template-rows:48px"
        } else {
            "grid-template-columns:48px"
        };
        let fixture = fixture(&format!(
            "display:grid;box-sizing:border-box;width:184px;height:184px;margin:0;padding:0;{repeated_axis};{other_axis}"
        ));
        let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &fixture.view,
            &[],
            viewport(),
            Default::default(),
        )
        .unwrap_or_else(|error| panic!("{axis}/{mode}: {error:?}"));
        let grid = snapshot
            .elements
            .iter()
            .find(|style| style.node_id == fixture.root)
            .unwrap()
            .grid_layout
            .as_ref()
            .unwrap();
        let (tracks, repeat) = if axis == "columns" {
            (&grid.columns, &grid.columns_auto_repeat)
        } else {
            (&grid.rows, &grid.rows_auto_repeat)
        };

        assert_eq!(tracks, &[ComputedCssGridTrackSize::Percentage(0.3)]);
        assert_eq!(
            repeat.as_ref().map(|repeat| repeat.mode),
            Some(if mode == "auto-fit" {
                ComputedCssGridAutoRepeatMode::AutoFit
            } else {
                ComputedCssGridAutoRepeatMode::AutoFill
            })
        );
    }
}

#[test]
fn track_sizing_profile_preserves_definite_percentage_auto_repeat_maximums() {
    for (axis, mode, minimum, maximum, expected_min) in [
        ("columns", "auto-fill", "16px", "25%", 16.0),
        ("columns", "auto-fit", "16px", "calc(20px + 10%)", 16.0),
        ("columns", "auto-fill", "16px", "calc(10% + 20px)", 16.0),
        ("rows", "auto-fill", "25%", "20%", 0.25),
        ("rows", "auto-fit", "25%", "0%", 0.25),
    ] {
        let repeated_axis =
            format!("grid-template-{axis}:repeat({mode},minmax({minimum},{maximum}))");
        let other_axis = if axis == "columns" {
            "grid-template-rows:48px"
        } else {
            "grid-template-columns:48px"
        };
        let fixture = fixture(&format!(
            "display:grid;box-sizing:border-box;width:184px;height:184px;min-width:0;min-height:0;max-width:none;max-height:none;margin:0;padding:0;{repeated_axis};{other_axis}"
        ));
        let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &fixture.view,
            &[],
            viewport(),
            Default::default(),
        )
        .unwrap_or_else(|error| panic!("{axis}/{mode}/{minimum}/{maximum}: {error:?}"));
        let grid = snapshot
            .elements
            .iter()
            .find(|style| style.node_id == fixture.root)
            .unwrap()
            .grid_layout
            .as_ref()
            .unwrap();
        let (tracks, repeat) = if axis == "columns" {
            (&grid.columns, &grid.columns_auto_repeat)
        } else {
            (&grid.rows, &grid.rows_auto_repeat)
        };

        assert_eq!(tracks.len(), 1, "{axis}/{mode}/{maximum}");
        let ComputedCssGridTrackSize::MinMax { min, max } = &tracks[0] else {
            panic!("{axis}/{mode}/{maximum} did not retain minmax");
        };
        match (min, expected_min) {
            (ComputedCssGridTrackBreadth::LengthPx(actual), expected) => {
                assert_eq!(*actual, expected, "{axis}/{mode}/{maximum} minimum");
            }
            (ComputedCssGridTrackBreadth::Percentage(actual), expected) => {
                assert_eq!(*actual, expected, "{axis}/{mode}/{maximum} minimum");
            }
            (actual, _) => panic!("{axis}/{mode}/{maximum} unexpected minimum {actual:?}"),
        }
        match (max, maximum) {
            (ComputedCssGridTrackBreadth::Percentage(actual), "25%") => {
                assert_eq!(*actual, 0.25, "{axis}/{mode} maximum");
            }
            (ComputedCssGridTrackBreadth::Percentage(actual), "20%") => {
                assert_eq!(*actual, 0.2, "{axis}/{mode} maximum");
            }
            (ComputedCssGridTrackBreadth::Percentage(actual), "0%") => {
                assert_eq!(*actual, 0.0, "{axis}/{mode} maximum");
            }
            (
                ComputedCssGridTrackBreadth::Calc(ComputedCssMath::Sum(terms)),
                "calc(20px + 10%)" | "calc(10% + 20px)",
            ) => {
                assert_eq!(terms.len(), 2, "{axis}/{mode} maximum terms");
                assert!(
                    terms
                        .iter()
                        .any(|term| matches!(term, ComputedCssMath::LengthPx(v) if *v == 20.0))
                );
                assert!(
                    terms
                        .iter()
                        .any(|term| matches!(term, ComputedCssMath::Percentage(v) if *v == 0.1))
                );
            }
            (actual, _) => panic!("{axis}/{mode} unexpected maximum {actual:?}"),
        }
        assert_eq!(
            repeat.as_ref().map(|repeat| repeat.mode),
            Some(if mode == "auto-fill" {
                ComputedCssGridAutoRepeatMode::AutoFill
            } else {
                ComputedCssGridAutoRepeatMode::AutoFit
            }),
            "{axis}/{mode} repeat mode"
        );
    }
}

#[test]
fn track_sizing_profile_rejects_unsupported_auto_repeat_maximum_calc_shapes() {
    for maximum in ["min(20px,10%)", "calc(20px - 10%)"] {
        let fixture = fixture(&format!(
            "display:grid;box-sizing:border-box;width:184px;height:184px;min-width:0;min-height:0;grid-template-columns:repeat(auto-fill,minmax(16px,{maximum}));grid-template-rows:48px;margin:0;padding:0"
        ));
        let error = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &fixture.view,
            &[],
            viewport(),
            Default::default(),
        )
        .unwrap_err();

        assert!(
            matches!(
                error,
                crate::stylo_dom::cascade::CssCascadeError::UnsupportedComputedGridValue {
                    node,
                    property: "grid-template-columns",
                    ..
                } if node == fixture.root
            ),
            "out-of-scope maximum {maximum} must fail closed"
        );
    }
}
