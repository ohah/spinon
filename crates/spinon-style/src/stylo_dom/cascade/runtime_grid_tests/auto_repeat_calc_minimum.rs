use super::*;

#[test]
fn track_sizing_profile_normalizes_absolute_calc_auto_repeat_minimums() {
    for (axis, mode) in [
        ("columns", "auto-fill"),
        ("columns", "auto-fit"),
        ("rows", "auto-fill"),
        ("rows", "auto-fit"),
    ] {
        let repeated_axis =
            format!("grid-template-{axis}:repeat({mode},minmax(calc(20px + 20px),1fr))");
        let other_axis = if axis == "columns" {
            "grid-template-rows:48px"
        } else {
            "grid-template-columns:48px"
        };
        let calc_fixture = fixture(&format!(
            "display:grid;box-sizing:border-box;width:184px;height:184px;{repeated_axis};{other_axis};margin:0;padding:0"
        ));
        let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &calc_fixture.view,
            &[],
            viewport(),
            Default::default(),
        )
        .unwrap_or_else(|error| panic!("{axis}/{mode} calc minimum: {error:?}"));
        let grid = snapshot
            .elements
            .iter()
            .find(|style| style.node_id == calc_fixture.root)
            .unwrap()
            .grid_layout
            .as_ref()
            .unwrap();
        let control_fixture = fixture(&format!(
            "display:grid;box-sizing:border-box;width:184px;height:184px;grid-template-{axis}:repeat({mode},minmax(40px,1fr));{other_axis};margin:0;padding:0"
        ));
        let control_snapshot =
            compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
                &control_fixture.view,
                &[],
                viewport(),
                Default::default(),
            )
            .unwrap();
        let control_grid = control_snapshot
            .elements
            .iter()
            .find(|style| style.node_id == control_fixture.root)
            .unwrap()
            .grid_layout
            .as_ref()
            .unwrap();
        let (tracks, repeat) = if axis == "columns" {
            (&grid.columns, &grid.columns_auto_repeat)
        } else {
            (&grid.rows, &grid.rows_auto_repeat)
        };
        let control_tracks = if axis == "columns" {
            &control_grid.columns
        } else {
            &control_grid.rows
        };

        assert_eq!(tracks.len(), 1, "{axis}/{mode}");
        assert_eq!(
            tracks, control_tracks,
            "{axis}/{mode} must match its fixed control"
        );
        assert_eq!(
            tracks[0],
            ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::LengthPx(40.0),
                max: ComputedCssGridTrackBreadth::Fraction(1.0),
            },
            "{axis}/{mode} absolute calc is simplified by the Stylo computed value"
        );
        assert_eq!(
            repeat.as_ref().map(|repeat| repeat.mode),
            Some(if mode == "auto-fill" {
                ComputedCssGridAutoRepeatMode::AutoFill
            } else {
                ComputedCssGridAutoRepeatMode::AutoFit
            }),
            "{axis}/{mode}"
        );
    }
}

#[test]
fn track_sizing_profile_preserves_definite_percentage_calc_auto_repeat_minimums() {
    for (axis, mode) in [
        ("columns", "auto-fill"),
        ("columns", "auto-fit"),
        ("rows", "auto-fill"),
        ("rows", "auto-fit"),
    ] {
        let repeated_axis =
            format!("grid-template-{axis}:repeat({mode},minmax(calc(20px + 10%),1fr))");
        let dimensions = if axis == "columns" {
            "width:112px;height:48px;grid-template-rows:48px"
        } else {
            "width:48px;height:112px;grid-template-columns:48px"
        };
        let fixture = fixture(&format!(
            "display:grid;box-sizing:border-box;{dimensions};{repeated_axis};margin:0;padding:0"
        ));
        let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &fixture.view,
            &[],
            viewport(),
            Default::default(),
        )
        .unwrap_or_else(|error| panic!("{axis}/{mode} percentage calc minimum: {error:?}"));
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

        assert_eq!(tracks.len(), 1, "{axis}/{mode}");
        assert_eq!(
            tracks[0],
            ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::Calc(ComputedCssMath::Sum(vec![
                    ComputedCssMath::Percentage(0.1),
                    ComputedCssMath::LengthPx(20.0),
                ])),
                max: ComputedCssGridTrackBreadth::Fraction(1.0),
            },
            "{axis}/{mode} must retain the supported length-plus-percentage math tree"
        );
        assert_eq!(
            repeat.as_ref().map(|repeat| repeat.mode),
            Some(if mode == "auto-fill" {
                ComputedCssGridAutoRepeatMode::AutoFill
            } else {
                ComputedCssGridAutoRepeatMode::AutoFit
            }),
            "{axis}/{mode}"
        );
    }
}

#[test]
fn track_sizing_profile_admits_positive_percentage_auto_repeat_minimums() {
    for (axis, mode, minimum) in [
        ("columns", "auto-fill", "calc(0px + 10%)"),
        ("columns", "auto-fit", "10%"),
        ("rows", "auto-fill", "10%"),
        ("rows", "auto-fit", "calc(0px + 10%)"),
    ] {
        let repeated_axis = format!("grid-template-{axis}:repeat({mode},minmax({minimum},1fr))");
        let other_axis = if axis == "columns" {
            "grid-template-rows:48px"
        } else {
            "grid-template-columns:48px"
        };
        let fixture = fixture(&format!(
            "display:grid;box-sizing:border-box;width:124px;height:124px;min-width:0;min-height:0;border:2px solid;padding:0;{repeated_axis};{other_axis};margin:0"
        ));
        let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &fixture.view,
            &[],
            viewport(),
            Default::default(),
        )
        .unwrap_or_else(|error| panic!("{axis}/{mode} {minimum}: {error:?}"));
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

        assert_eq!(tracks.len(), 1, "{axis}/{mode}");
        assert_eq!(
            tracks[0],
            ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::Percentage(0.1),
                max: ComputedCssGridTrackBreadth::Fraction(1.0),
            },
            "{axis}/{mode} computed minimum"
        );
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
fn track_sizing_profile_admits_positive_percentage_minimum_with_fixed_maximum() {
    let fixture = fixture(
        "display:grid;box-sizing:content-box;width:120px;height:48px;min-width:0;min-height:0;border:2px solid;padding:0;grid-template-columns:repeat(auto-fill,minmax(10%,12px));grid-template-rows:48px;margin:0",
    );
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let grid = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == fixture.root)
        .unwrap()
        .grid_layout
        .as_ref()
        .unwrap();

    assert_eq!(
        grid.columns,
        vec![ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::Percentage(0.1),
            max: ComputedCssGridTrackBreadth::LengthPx(12.0),
        }]
    );
    assert!(grid.columns_auto_repeat.is_some());
}

#[test]
fn track_sizing_profile_admits_zero_percentage_minimum_with_flexible_maximum() {
    for minimum in ["0%", "calc(0px + 0%)"] {
        let fixture = fixture(&format!(
            "display:grid;box-sizing:content-box;width:120px;height:48px;min-width:0;min-height:0;border:2px solid;padding:0;grid-template-columns:repeat(auto-fill,minmax({minimum},1fr));grid-template-rows:48px;margin:0"
        ));
        let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &fixture.view,
            &[],
            viewport(),
            Default::default(),
        )
        .unwrap();
        let grid = snapshot
            .elements
            .iter()
            .find(|style| style.node_id == fixture.root)
            .unwrap()
            .grid_layout
            .as_ref()
            .unwrap();

        assert_eq!(
            grid.columns,
            vec![ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::Percentage(0.0),
                max: ComputedCssGridTrackBreadth::Fraction(1.0),
            }],
            "Stylo computed minimum for {minimum}"
        );
        assert_eq!(
            grid.columns_auto_repeat.as_ref().map(|repeat| repeat.mode),
            Some(ComputedCssGridAutoRepeatMode::AutoFill),
            "auto-repeat metadata for {minimum}"
        );
    }
}

#[test]
fn track_sizing_profile_rejects_out_of_scope_percentage_auto_repeat_minimums() {
    let template = "repeat(auto-fill,10%)";
    let bad_percent_fixture = fixture(&format!(
        "display:grid;box-sizing:border-box;width:124px;height:124px;min-width:0;min-height:0;border:2px solid;padding:0;grid-template-columns:{template};grid-template-rows:48px;margin:0"
    ));
    let error = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &bad_percent_fixture.view,
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
            } if node == bad_percent_fixture.root
        ),
        "out-of-scope {template} must remain fail-closed"
    );
    for minimum in ["0%", "10%"] {
        for maximum in ["auto", "max-content"] {
            let fixture = fixture(&format!(
                "display:grid;box-sizing:border-box;width:124px;height:124px;min-width:0;min-height:0;border:2px solid;padding:0;grid-template-columns:repeat(auto-fill,minmax({minimum},{maximum}));grid-template-rows:48px;margin:0"
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
                "out-of-scope minmax({minimum},{maximum}) must remain fail-closed"
            );
        }
    }
}

#[test]
fn track_sizing_profile_rejects_unsupported_percentage_calc_auto_repeat_minimums() {
    for calc in ["calc(20px - 5%)", "max(20px, 5%)"] {
        let fixture = fixture(&format!(
            "display:grid;box-sizing:border-box;width:112px;height:48px;grid-template-columns:repeat(auto-fill,minmax({calc},1fr));grid-template-rows:48px;margin:0;padding:0"
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
            "unsupported minimum {calc} must remain fail-closed"
        );
    }
}
