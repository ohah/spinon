use super::*;

#[test]
fn track_sizing_profile_preserves_inflexible_minimum_fixed_maximum_auto_repeat_tracks() {
    for (axis, mode) in [
        ("columns", "auto-fill"),
        ("columns", "auto-fit"),
        ("rows", "auto-fill"),
        ("rows", "auto-fit"),
    ] {
        for minimum in ["auto", "min-content", "max-content"] {
            for maximum in ["40px", "30%", "calc(15px + 25%)"] {
                let repeated_axis =
                    format!("grid-template-{axis}:repeat({mode},minmax({minimum},{maximum}))");
                let other_axis = if axis == "columns" {
                    "grid-template-rows:20px"
                } else {
                    "grid-template-columns:20px"
                };
                let fixture = fixture(&format!(
                    "display:grid;box-sizing:border-box;width:130px;height:130px;min-width:0;min-height:0;max-width:none;max-height:none;margin:0;padding:0;{repeated_axis};{other_axis}"
                ));
                let snapshot =
                    compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
                        &fixture.view,
                        &[],
                        viewport(),
                        Default::default(),
                    )
                    .unwrap_or_else(|error| {
                        panic!("{axis}/{mode}/minmax({minimum},{maximum}): {error:?}")
                    });
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

                assert_eq!(tracks.len(), 1, "{axis}/{mode}/{minimum}/{maximum}");
                let ComputedCssGridTrackSize::MinMax { min, max } = &tracks[0] else {
                    panic!("{axis}/{mode}/{minimum}/{maximum} did not retain minmax");
                };
                let expected_minimum = match minimum {
                    "auto" => ComputedCssGridTrackBreadth::Auto,
                    "min-content" => ComputedCssGridTrackBreadth::MinContent,
                    "max-content" => ComputedCssGridTrackBreadth::MaxContent,
                    _ => unreachable!(),
                };
                assert_eq!(min, &expected_minimum, "{axis}/{mode}/{minimum}/{maximum}");
                match (max, maximum) {
                    (ComputedCssGridTrackBreadth::LengthPx(actual), "40px") => {
                        assert_eq!(*actual, 40.0, "{axis}/{mode}/{minimum}/{maximum}");
                    }
                    (ComputedCssGridTrackBreadth::Percentage(actual), "30%") => {
                        assert_eq!(*actual, 0.3, "{axis}/{mode}/{minimum}/{maximum}");
                    }
                    (
                        ComputedCssGridTrackBreadth::Calc(ComputedCssMath::Sum(terms)),
                        "calc(15px + 25%)",
                    ) => {
                        assert_eq!(terms.len(), 2, "{axis}/{mode}/{minimum}/{maximum}");
                        assert!(terms.iter().any(
                            |term| matches!(term, ComputedCssMath::LengthPx(value) if *value == 15.0)
                        ));
                        assert!(terms.iter().any(
                            |term| matches!(term, ComputedCssMath::Percentage(value) if *value == 0.25)
                        ));
                    }
                    (actual, _) => {
                        panic!("{axis}/{mode}/{minimum}/{maximum} unexpected maximum {actual:?}")
                    }
                }
                assert_eq!(
                    repeat.as_ref().map(|repeat| repeat.mode),
                    Some(if mode == "auto-fill" {
                        ComputedCssGridAutoRepeatMode::AutoFill
                    } else {
                        ComputedCssGridAutoRepeatMode::AutoFit
                    }),
                    "{axis}/{mode}/{minimum}/{maximum} repeat mode"
                );
            }
        }
    }
}
