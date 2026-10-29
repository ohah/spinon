use super::*;

#[test]
fn integer_repeat_expands_tracks_and_preserves_n_plus_one_line_names() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:[outer-start] 20px [outer-end repeat-start] repeat(2,[item-start] 30px [seam] 40px [item-end]) [trailing] 20px [outer-last];grid-template-rows:repeat(2,50px);margin:0;padding:0",
    );
    let snapshot = compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let style = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == fixture.root)
        .unwrap();
    let grid = style.grid_layout.as_ref().unwrap();

    assert_eq!(
        snapshot.profile,
        ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
    );
    assert_eq!(
        grid.columns,
        [
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::LengthPx(30.0),
            ComputedCssGridTrackSize::LengthPx(40.0),
            ComputedCssGridTrackSize::LengthPx(30.0),
            ComputedCssGridTrackSize::LengthPx(40.0),
            ComputedCssGridTrackSize::LengthPx(20.0),
        ]
    );
    assert_eq!(
        grid.columns_line_names,
        [
            vec!["outer-start"],
            vec!["outer-end", "repeat-start", "item-start"],
            vec!["seam"],
            vec!["item-end", "item-start"],
            vec!["seam"],
            vec!["item-end", "trailing"],
            vec!["outer-last"],
        ]
        .map(|group| group.into_iter().map(str::to_owned).collect::<Vec<_>>())
    );
    assert_eq!(
        grid.rows,
        [
            ComputedCssGridTrackSize::LengthPx(50.0),
            ComputedCssGridTrackSize::LengthPx(50.0),
        ]
    );
    assert_eq!(grid.rows_line_names, vec![Vec::<String>::new(); 3]);
}

#[test]
fn repeat_count_one_and_repeat_free_control_have_identical_tracks() {
    let repeated = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:repeat(1,1fr);grid-template-rows:repeat(1,40px);margin:0;padding:0",
    );
    let control = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:1fr;grid-template-rows:40px;margin:0;padding:0",
    );
    let repeated = compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
        &repeated.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let control = compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
        &control.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let repeated_grid = repeated.elements[0].grid_layout.as_ref().unwrap();
    let control_grid = control.elements[0].grid_layout.as_ref().unwrap();

    assert_eq!(repeated_grid.columns, control_grid.columns);
    assert_eq!(repeated_grid.rows, control_grid.rows);
    assert_eq!(
        repeated_grid.columns_line_names,
        vec![Vec::<String>::new(); 2]
    );
    assert_eq!(repeated_grid.rows_line_names, vec![Vec::<String>::new(); 2]);
}

#[test]
fn template_profile_accepts_repeat_shorthands_and_grid_template_areas() {
    for shorthand in ["grid-template", "grid"] {
        let fixture = fixture(&format!(
            "display:grid;box-sizing:border-box;width:300px;height:200px;{shorthand}:repeat(2,30px) / repeat(2,40px);margin:0;padding:0"
        ));
        let snapshot = compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
            &fixture.view,
            &[],
            viewport(),
            Default::default(),
        )
        .unwrap_or_else(|error| panic!("{shorthand} track shorthand: {error:?}"));
        let grid = snapshot.elements[0].grid_layout.as_ref().unwrap();
        assert_eq!(grid.columns.len(), 2, "{shorthand}");
        assert_eq!(grid.rows.len(), 2, "{shorthand}");
    }

    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:repeat(2,40px);grid-template-rows:40px;grid-template-areas:\"area area\";margin:0;padding:0",
    );
    let snapshot = compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let grid = snapshot.elements[0].grid_layout.as_ref().unwrap();
    let areas = grid.template_areas.as_ref().unwrap();
    assert_eq!((areas.rows, areas.columns), (1, 2));
    assert_eq!(areas.areas.len(), 1);
    assert_eq!(areas.areas[0].name, "area");
}

#[test]
fn template_profile_accepts_empty_explicit_tracks_when_areas_define_grid() {
    let fixture_with_areas = fixture(
        "display:grid;box-sizing:border-box;width:120px;height:80px;grid-template-columns:none;grid-template-rows:none;grid-template-areas:'a b' 'c d';margin:0;padding:0",
    );
    let snapshot = compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
        &fixture_with_areas.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let grid = snapshot.elements[0].grid_layout.as_ref().unwrap();

    assert!(grid.columns.is_empty());
    assert!(grid.rows.is_empty());
    assert_eq!(
        grid.template_areas
            .as_ref()
            .map(|areas| (areas.rows, areas.columns)),
        Some((2, 2))
    );

    let no_areas = fixture(
        "display:grid;box-sizing:border-box;width:120px;height:80px;grid-template-columns:none;grid-template-rows:none;margin:0;padding:0",
    );
    assert!(
        compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
            &no_areas.view,
            &[],
            viewport(),
            Default::default(),
        )
        .is_err()
    );
}

#[test]
fn integer_repeat_accepts_author_stylesheet_declarations() {
    let fixture =
        fixture("display:grid;box-sizing:border-box;width:300px;height:200px;margin:0;padding:0");
    let stylesheet = StylesheetSource {
        id: "c1131-author-repeat".to_owned(),
        base_url: "https://spinon.invalid/c1131.css".to_owned(),
        origin: CssOrigin::Author,
        css: "#root { grid-template: 40px / repeat(2, 30px); }".to_owned(),
    };
    let snapshot = compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
        &fixture.view,
        &[stylesheet],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let grid = snapshot.elements[0].grid_layout.as_ref().unwrap();

    assert_eq!(grid.columns.len(), 2);
    assert_eq!(grid.rows.len(), 1);
}

#[test]
fn track_sizing_profile_preserves_fixed_minimum_auto_repeat_modes() {
    for (keyword, expected) in [
        ("auto-fill", ComputedCssGridAutoRepeatMode::AutoFill),
        ("auto-fit", ComputedCssGridAutoRepeatMode::AutoFit),
    ] {
        let fixture = fixture(&format!(
            "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:repeat({keyword},minmax(24px,1fr));grid-template-rows:40px;margin:0;padding:0"
        ));
        let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &fixture.view,
            &[],
            viewport(),
            Default::default(),
        )
        .unwrap_or_else(|error| panic!("{keyword} cascade: {error:?}"));
        let grid = snapshot.elements[0].grid_layout.as_ref().unwrap();

        assert_eq!(
            grid.columns_auto_repeat,
            Some(ComputedCssGridAutoRepeat {
                mode: expected,
                start: 0,
                end: 1,
                line_names: vec![Vec::new(); 2],
                fixed_repeat_minmax_track_indices: Vec::new(),
            }),
            "{keyword}"
        );
        assert_eq!(grid.rows_auto_repeat, None, "{keyword}");
        assert_eq!(grid.columns.len(), 1, "{keyword} retains one pattern track");
        assert_eq!(grid.columns_line_names, vec![Vec::<String>::new(); 2]);
        assert_eq!(
            grid.columns_auto_repeat.as_ref().unwrap().line_names,
            vec![Vec::<String>::new(); 2]
        );
        assert_eq!(grid.rows, [ComputedCssGridTrackSize::LengthPx(40.0)]);
    }
}

#[test]
fn track_sizing_profile_preserves_independent_auto_repeats_on_both_axes() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:200px;height:266px;grid-template-columns:20px repeat(auto-fit,minmax(40px,1fr)) 10px;grid-template-rows:20px repeat(auto-fill,minmax(40px,1fr)) 10px;margin:0;padding:0",
    );
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap_or_else(|error| panic!("independent two-axis auto-repeat: {error:?}"));
    let grid = snapshot.elements[0].grid_layout.as_ref().unwrap();

    assert_eq!(
        grid.columns_auto_repeat,
        Some(ComputedCssGridAutoRepeat {
            mode: ComputedCssGridAutoRepeatMode::AutoFit,
            start: 1,
            end: 2,
            line_names: vec![Vec::new(); 2],
            fixed_repeat_minmax_track_indices: Vec::new(),
        })
    );
    assert_eq!(
        grid.rows_auto_repeat,
        Some(ComputedCssGridAutoRepeat {
            mode: ComputedCssGridAutoRepeatMode::AutoFill,
            start: 1,
            end: 2,
            line_names: vec![Vec::new(); 2],
            fixed_repeat_minmax_track_indices: Vec::new(),
        })
    );
    assert_eq!(grid.columns.len(), 3);
    assert_eq!(grid.rows.len(), 3);
}

#[test]
fn track_sizing_profile_preserves_auto_repeat_inner_and_component_line_names() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:210px;height:100px;grid-template-columns:[container-start] 20px [prefix-end repeat-start] repeat(auto-fill,[item-start] minmax(40px,1fr) [item-end]) [repeat-end suffix-start] 10px [container-end];grid-template-rows:40px;margin:0;padding:0",
    );
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap_or_else(|error| panic!("named auto-repeat cascade: {error:?}"));
    let grid = snapshot.elements[0].grid_layout.as_ref().unwrap();

    assert_eq!(
        grid.columns_line_names,
        [
            vec!["container-start"],
            vec!["prefix-end", "repeat-start"],
            vec!["repeat-end", "suffix-start"],
            vec!["container-end"],
        ]
        .map(|group| group.into_iter().map(str::to_owned).collect::<Vec<_>>())
    );
    assert_eq!(
        grid.columns_auto_repeat.as_ref().unwrap().line_names,
        [vec!["item-start"], vec!["item-end"]]
            .map(|group| group.into_iter().map(str::to_owned).collect::<Vec<_>>())
    );
}

#[test]
fn track_sizing_profile_preserves_auto_repeat_neighbors_and_fragment_order() {
    let cases = [
        (
            "20px repeat(auto-fill,minmax(24px,1fr)) 10px",
            vec![
                ComputedCssGridTrackSize::LengthPx(20.0),
                ComputedCssGridTrackSize::MinMax {
                    min: ComputedCssGridTrackBreadth::LengthPx(24.0),
                    max: ComputedCssGridTrackBreadth::Fraction(1.0),
                },
                ComputedCssGridTrackSize::LengthPx(10.0),
            ],
            (1, 2),
        ),
        (
            "repeat(auto-fit,40px) 10px",
            vec![
                ComputedCssGridTrackSize::LengthPx(40.0),
                ComputedCssGridTrackSize::LengthPx(10.0),
            ],
            (0, 1),
        ),
        (
            "20px repeat(auto-fill,40px)",
            vec![
                ComputedCssGridTrackSize::LengthPx(20.0),
                ComputedCssGridTrackSize::LengthPx(40.0),
            ],
            (1, 2),
        ),
        (
            "20px repeat(auto-fill,minmax(30px,1fr) 20px) 10px",
            vec![
                ComputedCssGridTrackSize::LengthPx(20.0),
                ComputedCssGridTrackSize::MinMax {
                    min: ComputedCssGridTrackBreadth::LengthPx(30.0),
                    max: ComputedCssGridTrackBreadth::Fraction(1.0),
                },
                ComputedCssGridTrackSize::LengthPx(20.0),
                ComputedCssGridTrackSize::LengthPx(10.0),
            ],
            (1, 3),
        ),
        (
            "10% repeat(auto-fill,minmax(24px,1fr)) 5%",
            vec![
                ComputedCssGridTrackSize::Percentage(0.1),
                ComputedCssGridTrackSize::MinMax {
                    min: ComputedCssGridTrackBreadth::LengthPx(24.0),
                    max: ComputedCssGridTrackBreadth::Fraction(1.0),
                },
                ComputedCssGridTrackSize::Percentage(0.05),
            ],
            (1, 2),
        ),
        (
            "0% repeat(auto-fit,40px) 12.5%",
            vec![
                ComputedCssGridTrackSize::Percentage(0.0),
                ComputedCssGridTrackSize::LengthPx(40.0),
                ComputedCssGridTrackSize::Percentage(0.125),
            ],
            (1, 2),
        ),
    ];

    for (columns, expected_tracks, (start, end)) in cases {
        let fixture = fixture(&format!(
            "display:grid;box-sizing:border-box;width:200px;height:200px;grid-template-columns:{columns};grid-template-rows:40px;margin:0;padding:0"
        ));
        let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &fixture.view,
            &[],
            viewport(),
            Default::default(),
        )
        .unwrap_or_else(|error| panic!("{columns}: {error:?}"));
        let grid = snapshot.elements[0].grid_layout.as_ref().unwrap();

        assert_eq!(grid.columns, expected_tracks, "{columns}");
        assert_eq!(
            grid.columns_auto_repeat,
            Some(ComputedCssGridAutoRepeat {
                mode: if columns.contains("auto-fit") {
                    ComputedCssGridAutoRepeatMode::AutoFit
                } else {
                    ComputedCssGridAutoRepeatMode::AutoFill
                },
                start,
                end,
                line_names: vec![Vec::new(); end - start + 1],
                fixed_repeat_minmax_track_indices: Vec::new(),
            }),
            "{columns}"
        );
        assert_eq!(
            grid.columns_line_names,
            vec![Vec::<String>::new(); grid.columns.len() - (end - start) + 2]
        );
        assert_eq!(
            grid.columns_auto_repeat.as_ref().unwrap().line_names,
            vec![Vec::<String>::new(); end - start + 1]
        );
    }
}

#[test]
fn track_sizing_profile_preserves_percentage_auto_repeat_neighbors_on_rows() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:80px;height:200px;grid-template-columns:40px;grid-template-rows:10% repeat(auto-fit,minmax(24px,1fr)) 5%;margin:0;padding:0",
    );
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap_or_else(|error| panic!("row percentage auto-repeat neighbors: {error:?}"));
    let grid = snapshot.elements[0].grid_layout.as_ref().unwrap();
    assert_eq!(
        grid.rows,
        [
            ComputedCssGridTrackSize::Percentage(0.1),
            ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::LengthPx(24.0),
                max: ComputedCssGridTrackBreadth::Fraction(1.0),
            },
            ComputedCssGridTrackSize::Percentage(0.05),
        ]
    );
    assert_eq!(
        grid.rows_auto_repeat,
        Some(ComputedCssGridAutoRepeat {
            mode: ComputedCssGridAutoRepeatMode::AutoFit,
            start: 1,
            end: 2,
            line_names: vec![Vec::new(); 2],
            fixed_repeat_minmax_track_indices: Vec::new(),
        })
    );
}

#[test]
fn auto_repeat_rejects_profiles_and_preserves_typed_neighbor_shapes() {
    let template_fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:repeat(auto-fill,minmax(24px,1fr));grid-template-rows:40px;margin:0;padding:0",
    );
    assert!(
        compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
            &template_fixture.view,
            &[],
            viewport(),
            Default::default(),
        )
        .is_err()
    );

    for (style, neighbor_index) in [
        (
            "grid-template-columns:minmax(10px,20px) repeat(auto-fill,minmax(24px,1fr))",
            0,
        ),
        (
            "grid-template-columns:20% repeat(auto-fill,minmax(24px,1fr)) calc(10px + 5% + clamp(0px,0%,1px))",
            2,
        ),
    ] {
        let fixture = fixture(&format!(
            "display:grid;box-sizing:border-box;width:300px;height:200px;{style};margin:0;padding:0"
        ));
        let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &fixture.view,
            &[],
            viewport(),
            Default::default(),
        )
        .unwrap_or_else(|error| {
            panic!("typed neighbor must reach layout projection: {style}: {error:?}")
        });
        let grid = snapshot.elements[0].grid_layout.as_ref().unwrap();
        assert!(
            matches!(
                grid.columns.get(neighbor_index),
                Some(ComputedCssGridTrackSize::MinMax { .. } | ComputedCssGridTrackSize::Calc(_))
            ),
            "expected typed neighbor at index {neighbor_index} in {style}: {:?}",
            grid.columns
        );
    }

    let multiple_auto_repeats = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:repeat(2,20px) repeat(auto-fill,minmax(24px,1fr)) repeat(auto-fit,minmax(24px,1fr));grid-template-rows:40px;margin:0;padding:0",
    );
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &multiple_auto_repeats.view,
        &[],
        viewport(),
        Default::default(),
    )
    .expect("Stylo should reject the second auto-repeat declaration during parsing");
    assert!(
        snapshot.elements[0]
            .grid_layout
            .as_ref()
            .is_some_and(|grid| grid.columns.is_empty())
    );
    assert!(snapshot.diagnostics.iter().any(|diagnostic| {
        diagnostic
            .diagnostic
            .message
            .contains("grid-template-columns:")
    }));

    let one_axis_with_areas = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:repeat(auto-fill,minmax(24px,1fr));grid-template-rows:40px;grid-template-areas:\"area\";margin:0;padding:0",
    );
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &one_axis_with_areas.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap_or_else(|error| panic!("one-axis auto-repeat with areas: {error:?}"));
    let grid = snapshot.elements[0].grid_layout.as_ref().unwrap();
    assert!(grid.columns_auto_repeat.is_some());
    assert_eq!(grid.rows_auto_repeat, None);
    assert_eq!(
        (
            grid.template_areas.as_ref().unwrap().rows,
            grid.template_areas.as_ref().unwrap().columns
        ),
        (1, 1)
    );

    let two_axes_with_areas = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:repeat(auto-fill,minmax(24px,1fr));grid-template-rows:repeat(auto-fill,minmax(24px,1fr));grid-template-areas:\"area\";margin:0;padding:0",
    );
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &two_axes_with_areas.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap_or_else(|error| panic!("two-axis auto-repeat with areas: {error:?}"));
    let grid = snapshot.elements[0].grid_layout.as_ref().unwrap();
    assert!(grid.columns_auto_repeat.is_some());
    assert!(grid.rows_auto_repeat.is_some());
    assert_eq!(
        (
            grid.template_areas.as_ref().unwrap().rows,
            grid.template_areas.as_ref().unwrap().columns
        ),
        (1, 1)
    );
    assert!(
        compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
            &two_axes_with_areas.view,
            &[],
            viewport(),
            Default::default(),
        )
        .is_err(),
        "TemplateV1 must keep rejecting auto-repeat even when areas are present"
    );

    // CSS discards a declaration with two auto-repeat functions, leaving the
    // computed `none` value that the track-sizing profile now supports.
    let invalid_repeat = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:20px repeat(auto-fill,minmax(24px,1fr)) repeat(auto-fit,24px);grid-template-rows:40px;margin:0;padding:0",
    );
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &invalid_repeat.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();
    assert!(
        snapshot.elements[0]
            .grid_layout
            .as_ref()
            .unwrap()
            .columns
            .is_empty()
    );
}

#[test]
fn integer_repeat_retains_duplicate_names_on_a_single_line() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:[same] repeat(2,[same] 30px [same]) [same] 30px [end];grid-template-rows:40px;margin:0;padding:0",
    );
    let snapshot = compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let grid = snapshot.elements[0].grid_layout.as_ref().unwrap();

    assert_eq!(grid.columns.len(), 3);
    assert_eq!(
        grid.columns_line_names,
        [
            vec!["same", "same"],
            vec!["same", "same"],
            vec!["same", "same"],
            vec!["end"],
        ]
        .map(|group| group.into_iter().map(str::to_owned).collect::<Vec<_>>())
    );
}

#[test]
fn integer_repeat_rejects_auto_minmax_and_over_limit_without_partial_tracks() {
    for tracks in [
        "repeat(auto-fill,40px)",
        "repeat(2,minmax(20px,1fr))",
        "repeat(0,40px)",
        "repeat(-1,40px)",
        "repeat(1.5,40px)",
        "repeat(2,repeat(2,40px))",
        "repeat(10001,1px)",
    ] {
        let fixture = fixture(&format!(
            "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:{tracks};grid-template-rows:40px;margin:0;padding:0"
        ));
        assert!(
            matches!(
                compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
                    &fixture.view,
                    &[],
                    viewport(),
                    Default::default(),
                ),
                Err(crate::CssCascadeError::UnsupportedComputedGridValue {
                    node,
                    property: "grid-template-columns",
                    ..
                }) if node == fixture.root
            ),
            "{tracks}"
        );
    }
}

#[test]
fn integer_repeat_accepts_exact_track_limit() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:repeat(10000,1px);grid-template-rows:40px;margin:0;padding:0",
    );
    let snapshot = compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let grid = snapshot.elements[0].grid_layout.as_ref().unwrap();

    assert_eq!(grid.columns.len(), 10_000);
    assert_eq!(grid.columns_line_names.len(), 10_001);
}

#[test]
fn integer_repeat_rejects_expanded_line_metadata_over_budget() {
    let name = "n".repeat(64);
    let style = format!(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:repeat(10000,[{name}] 1px);grid-template-rows:40px;margin:0;padding:0"
    );
    let fixture = fixture(&style);

    assert!(matches!(
        compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
            &fixture.view,
            &[],
            viewport(),
            Default::default(),
        ),
        Err(crate::CssCascadeError::UnsupportedComputedGridValue {
            node,
            property: "grid-template-columns",
            ..
        }) if node == fixture.root
    ));
}

#[test]
fn integer_repeat_allocation_failures_are_contextual_and_recoverable() {
    use super::super::incremental::{
        IntegerRepeatReservationSite, inject_integer_repeat_reservation_failure,
    };

    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:repeat(2,[item] 30px);grid-template-rows:40px;margin:0;padding:0",
    );
    for site in [
        IntegerRepeatReservationSite::Tracks,
        IntegerRepeatReservationSite::LineGroups,
        IntegerRepeatReservationSite::LineNames,
        IntegerRepeatReservationSite::LineNameText,
    ] {
        let error = {
            let _guard = inject_integer_repeat_reservation_failure(site);
            compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
                &fixture.view,
                &[],
                viewport(),
                Default::default(),
            )
            .unwrap_err()
        };
        assert!(
            matches!(
                error,
                crate::CssCascadeError::GridAllocationFailure {
                    node,
                    property: "grid-template-columns"
                } if node == fixture.root
            ),
            "예약 실패 지점: {site:?}, 실제 오류: {error:?}"
        );
        assert!(
            compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
                &fixture.view,
                &[],
                viewport(),
                Default::default(),
            )
            .is_ok()
        );
    }
}

#[test]
fn auto_repeat_line_metadata_allocation_failures_are_contextual_and_recoverable() {
    use super::super::incremental::{
        IntegerRepeatReservationSite, inject_integer_repeat_reservation_failure,
    };

    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:[outer] repeat(auto-fill,[item] minmax(40px,1fr) [end]) [tail];grid-template-rows:40px;margin:0;padding:0",
    );
    for site in [
        IntegerRepeatReservationSite::Tracks,
        IntegerRepeatReservationSite::LineGroups,
        IntegerRepeatReservationSite::LineNames,
        IntegerRepeatReservationSite::LineNameText,
    ] {
        let error = {
            let _guard = inject_integer_repeat_reservation_failure(site);
            compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
                &fixture.view,
                &[],
                viewport(),
                Default::default(),
            )
            .unwrap_err()
        };
        assert!(
            matches!(
                error,
                crate::CssCascadeError::GridAllocationFailure {
                    node,
                    property: "grid-template-columns"
                } if node == fixture.root
            ),
            "예약 실패 지점: {site:?}, 실제 오류: {error:?}"
        );
        assert!(
            compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
                &fixture.view,
                &[],
                viewport(),
                Default::default(),
            )
            .is_ok()
        );
    }
}
