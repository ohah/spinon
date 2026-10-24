use super::*;

fn computed_grid(style: &str) -> (Fixture, crate::ComputedCssGridLayout) {
    let fixture = fixture(style);
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap_or_else(|error| panic!("track sizing cascade for {style}: {error:?}"));
    let grid = snapshot
        .elements
        .iter()
        .find(|element| element.node_id == fixture.root)
        .and_then(|element| element.grid_layout.clone())
        .expect("grid root has a computed grid layout");
    (fixture, grid)
}

fn fixture_style(columns: &str, rows: &str) -> String {
    fixture_style_with_size(columns, rows, "200px", "200px")
}

fn fixture_style_with_size(columns: &str, rows: &str, width: &str, height: &str) -> String {
    format!(
        "display:grid;box-sizing:border-box;width:{width};height:{height};grid-template-columns:{columns};grid-template-rows:{rows};column-gap:8px;row-gap:8px;margin:0;padding:0"
    )
}

fn auto_repeat(
    mode: ComputedCssGridAutoRepeatMode,
    start: usize,
    end: usize,
) -> Option<ComputedCssGridAutoRepeat> {
    Some(ComputedCssGridAutoRepeat {
        mode,
        start,
        end,
        line_names: vec![Vec::new(); 2],
    })
}

fn fixed_tracks() -> [ComputedCssGridTrackSize; 4] {
    [
        ComputedCssGridTrackSize::LengthPx(20.0),
        ComputedCssGridTrackSize::LengthPx(10.0),
        ComputedCssGridTrackSize::LengthPx(20.0),
        ComputedCssGridTrackSize::LengthPx(10.0),
    ]
}

fn auto_track() -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::MinMax {
        min: ComputedCssGridTrackBreadth::LengthPx(32.0),
        max: ComputedCssGridTrackBreadth::Fraction(1.0),
    }
}

fn auto_track_30() -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::MinMax {
        min: ComputedCssGridTrackBreadth::LengthPx(30.0),
        max: ComputedCssGridTrackBreadth::Fraction(1.0),
    }
}

fn auto_track_200() -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::MinMax {
        min: ComputedCssGridTrackBreadth::LengthPx(200.0),
        max: ComputedCssGridTrackBreadth::Fraction(1.0),
    }
}

#[test]
fn fixed_repeat_prefix_and_suffix_expand_in_authored_order_on_both_axes() {
    let prefix_columns = fixture_style(
        "repeat(2,20px 10px) repeat(auto-fill,minmax(32px,1fr)) 16px",
        "200px",
    );
    let (_, grid) = computed_grid(&prefix_columns);
    assert_eq!(
        grid.columns,
        [
            fixed_tracks().as_slice(),
            &[auto_track(), ComputedCssGridTrackSize::LengthPx(16.0)],
        ]
        .concat()
    );
    assert_eq!(
        grid.columns_auto_repeat,
        auto_repeat(ComputedCssGridAutoRepeatMode::AutoFill, 4, 5)
    );
    assert_eq!(grid.columns_line_names, vec![Vec::<String>::new(); 7]);

    let suffix_columns = fixture_style(
        "16px repeat(auto-fill,minmax(32px,1fr)) repeat(2,20px 10px)",
        "200px",
    );
    let (_, grid) = computed_grid(&suffix_columns);
    assert_eq!(
        grid.columns,
        [
            &[ComputedCssGridTrackSize::LengthPx(16.0), auto_track()],
            fixed_tracks().as_slice(),
        ]
        .concat()
    );
    assert_eq!(
        grid.columns_auto_repeat,
        auto_repeat(ComputedCssGridAutoRepeatMode::AutoFill, 1, 2)
    );
    assert_eq!(grid.columns_line_names, vec![Vec::<String>::new(); 7]);

    let prefix_rows = fixture_style(
        "200px",
        "repeat(2,20px 10px) repeat(auto-fit,minmax(32px,1fr)) 16px",
    );
    let (_, grid) = computed_grid(&prefix_rows);
    assert_eq!(
        grid.rows,
        [
            fixed_tracks().as_slice(),
            &[auto_track(), ComputedCssGridTrackSize::LengthPx(16.0)],
        ]
        .concat()
    );
    assert_eq!(grid.columns_auto_repeat, None);
    assert_eq!(
        grid.rows_auto_repeat,
        auto_repeat(ComputedCssGridAutoRepeatMode::AutoFit, 4, 5)
    );
    assert_eq!(grid.rows_line_names, vec![Vec::<String>::new(); 7]);

    let suffix_rows = fixture_style(
        "200px",
        "16px repeat(auto-fit,minmax(32px,1fr)) repeat(2,20px 10px)",
    );
    let (_, grid) = computed_grid(&suffix_rows);
    assert_eq!(
        grid.rows,
        [
            &[ComputedCssGridTrackSize::LengthPx(16.0), auto_track()],
            fixed_tracks().as_slice(),
        ]
        .concat()
    );
    assert_eq!(
        grid.rows_auto_repeat,
        auto_repeat(ComputedCssGridAutoRepeatMode::AutoFit, 1, 2)
    );
    assert_eq!(grid.rows_line_names, vec![Vec::<String>::new(); 7]);
}

#[test]
fn fixed_repeat_keeps_outer_component_groups_separate_from_multitrack_auto_repeat_groups() {
    let (_, grid) = computed_grid(&fixture_style(
        "repeat(2,20px) repeat(auto-fill,minmax(32px,1fr) 12px)",
        "200px",
    ));

    assert_eq!(grid.columns.len(), 4);
    assert_eq!(
        grid.columns_auto_repeat
            .as_ref()
            .map(|repeat| (repeat.start, repeat.end)),
        Some((2, 4))
    );
    assert_eq!(grid.columns_line_names, vec![Vec::<String>::new(); 4]);
    assert_eq!(
        grid.columns_auto_repeat.as_ref().unwrap().line_names,
        vec![Vec::<String>::new(); 3]
    );
}

#[test]
fn two_fixed_repeat_groups_expand_in_authored_order_and_rebase_auto_repeat() {
    let prefix = computed_grid(&fixture_style_with_size(
        "repeat(2,20px) repeat(3,10px) repeat(auto-fill,minmax(30px,1fr)) 16px",
        "320px",
        "320px",
        "320px",
    ))
    .1;
    assert_eq!(
        prefix.columns,
        vec![
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::LengthPx(10.0),
            ComputedCssGridTrackSize::LengthPx(10.0),
            ComputedCssGridTrackSize::LengthPx(10.0),
            auto_track_30(),
            ComputedCssGridTrackSize::LengthPx(16.0),
        ]
    );
    assert_eq!(
        prefix.columns_auto_repeat,
        auto_repeat(ComputedCssGridAutoRepeatMode::AutoFill, 5, 6)
    );
    assert_eq!(prefix.columns_line_names, vec![Vec::<String>::new(); 8]);

    let split = computed_grid(&fixture_style_with_size(
        "repeat(2,20px) repeat(auto-fill,minmax(30px,1fr)) repeat(3,10px) 16px",
        "320px",
        "320px",
        "320px",
    ))
    .1;
    assert_eq!(
        split.columns,
        vec![
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::LengthPx(20.0),
            auto_track_30(),
            ComputedCssGridTrackSize::LengthPx(10.0),
            ComputedCssGridTrackSize::LengthPx(10.0),
            ComputedCssGridTrackSize::LengthPx(10.0),
            ComputedCssGridTrackSize::LengthPx(16.0),
        ]
    );
    assert_eq!(
        split.columns_auto_repeat,
        auto_repeat(ComputedCssGridAutoRepeatMode::AutoFill, 2, 3)
    );

    let suffix = computed_grid(&fixture_style_with_size(
        "16px repeat(auto-fill,minmax(30px,1fr)) repeat(2,20px) repeat(3,10px)",
        "320px",
        "320px",
        "320px",
    ))
    .1;
    assert_eq!(
        suffix.columns,
        vec![
            ComputedCssGridTrackSize::LengthPx(16.0),
            auto_track_30(),
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::LengthPx(10.0),
            ComputedCssGridTrackSize::LengthPx(10.0),
            ComputedCssGridTrackSize::LengthPx(10.0),
        ]
    );
    assert_eq!(
        suffix.columns_auto_repeat,
        auto_repeat(ComputedCssGridAutoRepeatMode::AutoFill, 1, 2)
    );

    let reversed = computed_grid(&fixture_style_with_size(
        "repeat(3,10px) repeat(auto-fill,minmax(30px,1fr)) repeat(2,20px) 16px",
        "320px",
        "320px",
        "320px",
    ))
    .1;
    assert_eq!(
        reversed.columns,
        vec![
            ComputedCssGridTrackSize::LengthPx(10.0),
            ComputedCssGridTrackSize::LengthPx(10.0),
            ComputedCssGridTrackSize::LengthPx(10.0),
            auto_track_30(),
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::LengthPx(16.0),
        ]
    );
    assert_eq!(
        reversed.columns_auto_repeat,
        auto_repeat(ComputedCssGridAutoRepeatMode::AutoFill, 3, 4)
    );

    let multitrack = computed_grid(&fixture_style_with_size(
        "repeat(2,20px 12px) repeat(auto-fill,minmax(30px,1fr)) repeat(3,10px)",
        "320px",
        "320px",
        "320px",
    ))
    .1;
    assert_eq!(
        multitrack.columns,
        vec![
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::LengthPx(12.0),
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::LengthPx(12.0),
            auto_track_30(),
            ComputedCssGridTrackSize::LengthPx(10.0),
            ComputedCssGridTrackSize::LengthPx(10.0),
            ComputedCssGridTrackSize::LengthPx(10.0),
        ]
    );
    assert_eq!(
        multitrack.columns_auto_repeat,
        auto_repeat(ComputedCssGridAutoRepeatMode::AutoFill, 4, 5)
    );

    let row_split = computed_grid(&fixture_style_with_size(
        "320px",
        "repeat(2,20px) repeat(auto-fit,minmax(30px,1fr)) repeat(3,10px) 16px",
        "80px",
        "320px",
    ))
    .1;
    assert_eq!(row_split.columns_auto_repeat, None);
    assert_eq!(
        row_split.rows_auto_repeat,
        auto_repeat(ComputedCssGridAutoRepeatMode::AutoFit, 2, 3)
    );
    assert_eq!(row_split.rows_line_names, vec![Vec::<String>::new(); 8]);
    assert_eq!(row_split.rows.len(), 7);

    let row_prefix = computed_grid(&fixture_style_with_size(
        "80px",
        "repeat(2,20px) repeat(3,10px) repeat(auto-fill,minmax(30px,1fr)) 16px",
        "80px",
        "320px",
    ))
    .1;
    assert_eq!(
        row_prefix.rows_auto_repeat,
        auto_repeat(ComputedCssGridAutoRepeatMode::AutoFill, 5, 6)
    );
}

#[test]
fn three_four_and_eight_fixed_repeat_groups_expand_in_authored_order() {
    let three = computed_grid(&fixture_style_with_size(
        "repeat(2,11px) repeat(1,13px) repeat(auto-fill,minmax(200px,1fr)) repeat(2,17px)",
        "80px",
        "320px",
        "80px",
    ))
    .1;
    assert_eq!(
        three.columns,
        vec![
            ComputedCssGridTrackSize::LengthPx(11.0),
            ComputedCssGridTrackSize::LengthPx(11.0),
            ComputedCssGridTrackSize::LengthPx(13.0),
            auto_track_200(),
            ComputedCssGridTrackSize::LengthPx(17.0),
            ComputedCssGridTrackSize::LengthPx(17.0),
        ]
    );
    assert_eq!(
        three.columns_auto_repeat,
        auto_repeat(ComputedCssGridAutoRepeatMode::AutoFill, 3, 4)
    );
    assert_eq!(three.columns_line_names, vec![Vec::<String>::new(); 7]);

    let four = computed_grid(&fixture_style_with_size(
        "repeat(1,11px) repeat(1,12px) repeat(auto-fill,minmax(200px,1fr)) repeat(1,13px) repeat(1,14px)",
        "80px",
        "320px",
        "80px",
    ))
    .1;
    assert_eq!(
        four.columns,
        vec![
            ComputedCssGridTrackSize::LengthPx(11.0),
            ComputedCssGridTrackSize::LengthPx(12.0),
            auto_track_200(),
            ComputedCssGridTrackSize::LengthPx(13.0),
            ComputedCssGridTrackSize::LengthPx(14.0),
        ]
    );
    assert_eq!(
        four.columns_auto_repeat,
        auto_repeat(ComputedCssGridAutoRepeatMode::AutoFill, 2, 3)
    );

    let eight = computed_grid(&fixture_style_with_size(
        "80px",
        "repeat(1,11px) repeat(1,12px) repeat(auto-fill,minmax(200px,1fr)) repeat(1,13px) repeat(1,14px) repeat(1,15px) repeat(1,16px) repeat(1,17px) repeat(1,18px)",
        "80px",
        "320px",
    ))
    .1;
    assert_eq!(
        eight.rows,
        vec![
            ComputedCssGridTrackSize::LengthPx(11.0),
            ComputedCssGridTrackSize::LengthPx(12.0),
            auto_track_200(),
            ComputedCssGridTrackSize::LengthPx(13.0),
            ComputedCssGridTrackSize::LengthPx(14.0),
            ComputedCssGridTrackSize::LengthPx(15.0),
            ComputedCssGridTrackSize::LengthPx(16.0),
            ComputedCssGridTrackSize::LengthPx(17.0),
            ComputedCssGridTrackSize::LengthPx(18.0),
        ]
    );
    assert_eq!(
        eight.rows_auto_repeat,
        auto_repeat(ComputedCssGridAutoRepeatMode::AutoFill, 2, 3)
    );
    assert_eq!(eight.columns_auto_repeat, None);
    assert_eq!(eight.rows_line_names, vec![Vec::<String>::new(); 10]);
}

#[test]
fn constant_absolute_calc_and_absolute_units_normalize_to_computed_length_px() {
    let literal = computed_grid(&fixture_style(
        "repeat(2,20px 10px) repeat(auto-fill,minmax(32px,1fr)) 16px",
        "200px",
    ))
    .1;
    let constant_calc = computed_grid(&fixture_style(
        "repeat(2,calc(20px) 10px) repeat(auto-fill,minmax(32px,1fr)) 16px",
        "200px",
    ))
    .1;
    assert_eq!(constant_calc.columns, literal.columns);
    assert_eq!(
        constant_calc.columns_auto_repeat,
        literal.columns_auto_repeat
    );
    assert_eq!(constant_calc.columns_line_names, literal.columns_line_names);

    let absolute_unit = computed_grid(&fixture_style(
        "repeat(2,0.25in 10px) repeat(auto-fill,minmax(32px,1fr)) 16px",
        "200px",
    ))
    .1;
    assert_eq!(
        absolute_unit.columns[0],
        ComputedCssGridTrackSize::LengthPx(24.0)
    );
    assert_eq!(
        absolute_unit.columns[2],
        ComputedCssGridTrackSize::LengthPx(24.0)
    );
}

#[test]
fn fixed_repeat_mixed_profile_rejects_non_px_tracks_and_unsupported_shapes() {
    for columns in [
        "repeat(2,20%) repeat(auto-fill,minmax(32px,1fr))",
        "repeat(2,calc(20px + 5%)) repeat(auto-fill,minmax(32px,1fr))",
        "repeat(2,minmax(20px,40px)) repeat(auto-fill,minmax(32px,1fr))",
        "repeat(2,20px 10px 5px) repeat(auto-fill,minmax(32px,1fr))",
    ] {
        let fixture = fixture(&fixture_style(columns, "200px"));
        let error = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &fixture.view,
            &[],
            viewport(),
            Default::default(),
        )
        .expect_err("out-of-profile fixed-repeat mixture must fail closed");
        assert!(
            matches!(
                error,
                crate::CssCascadeError::UnsupportedComputedGridValue {
                    node,
                    property: "grid-template-columns",
                    ..
                } if node == fixture.root
            ),
            "{columns}: {error:?}"
        );
    }

    let fixture = fixture(&fixture_style(
        "repeat(2,20px) repeat(auto-fill,minmax(32px,1fr)) repeat(auto-fit,minmax(32px,1fr))",
        "200px",
    ));
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
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
}

#[test]
fn fixed_repeat_mixed_profile_preserves_named_auto_repeat_lines() {
    let columns = "repeat(2,20px) repeat(auto-fill,[named] minmax(32px,1fr))";
    let (_, grid) = computed_grid(&fixture_style(columns, "200px"));
    assert_eq!(grid.columns_line_names, vec![Vec::<String>::new(); 4]);
    assert_eq!(
        grid.columns_auto_repeat.as_ref().unwrap().line_names,
        vec![vec!["named".to_owned()], Vec::new()]
    );
}

#[test]
fn fixed_repeat_mixed_profile_records_stylo_parser_rejection_of_inflexible_fixed_breadths() {
    for columns in [
        "repeat(2,1fr) repeat(auto-fill,minmax(32px,1fr))",
        "repeat(2,min-content) repeat(auto-fill,minmax(32px,1fr))",
    ] {
        let fixture = fixture(&fixture_style(columns, "200px"));
        let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &fixture.view,
            &[],
            viewport(),
            Default::default(),
        )
        .unwrap_or_else(|error| {
            panic!("Stylo should drop invalid fixed-repeat syntax {columns}: {error:?}")
        });
        assert!(
            snapshot.diagnostics.iter().any(|diagnostic| {
                diagnostic.node_id == Some(fixture.root)
                    && diagnostic
                        .diagnostic
                        .message
                        .contains("grid-template-columns")
            }),
            "Stylo parser rejection diagnostic missing for {columns}: {:?}",
            snapshot.diagnostics
        );
        let root = snapshot
            .elements
            .iter()
            .find(|element| element.node_id == fixture.root)
            .expect("root computed style");
        assert_eq!(
            root.properties
                .get("grid-template-columns")
                .map(String::as_str),
            Some("none"),
            "invalid CSS declaration must not reach the mixed-repeat adapter: {columns}"
        );
    }
}

#[test]
fn fixed_repeat_mixture_is_limited_to_one_auto_repeat_axis() {
    let cross_axis_fixture = fixture(&format!(
        "{};grid-template-areas:'left right'",
        fixture_style(
            "repeat(2,20px) repeat(auto-fill,minmax(32px,1fr))",
            "repeat(auto-fit,minmax(32px,1fr))",
        )
    ));
    assert!(matches!(
        compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &cross_axis_fixture.view,
            &[],
            viewport(),
            Default::default(),
        ),
        Err(crate::CssCascadeError::UnsupportedComputedGridValue {
            node,
            property: "grid-template-rows",
            ..
        }) if node == cross_axis_fixture.root
    ));

    let fixed_repeat_on_other_axis = fixture(&format!(
        "{};grid-template-areas:'left' 'right'",
        fixture_style("repeat(auto-fill,20px)", "repeat(2,20px)",)
    ));
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixed_repeat_on_other_axis.view,
        &[],
        viewport(),
        Default::default(),
    )
    .expect("one fixed row repeat and one bounded auto column repeat with areas");
    let grid = snapshot
        .elements
        .iter()
        .find(|element| element.node_id == fixed_repeat_on_other_axis.root)
        .and_then(|element| element.grid_layout.as_ref())
        .expect("cross-axis grid layout");
    assert_eq!(
        grid.rows,
        vec![
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::LengthPx(20.0),
        ]
    );
    assert!(grid.columns_auto_repeat.is_some());
    assert_eq!(
        grid.template_areas
            .as_ref()
            .map(|areas| (areas.rows, areas.columns)),
        Some((2, 1))
    );

    let fixed_columns_auto_rows = computed_grid(&format!(
        "{};grid-template-areas:'left left right right'",
        fixture_style("repeat(2,16px 20px)", "repeat(auto-fit,20px)")
    ));
    assert_eq!(
        fixed_columns_auto_rows.1.columns,
        [16.0, 20.0, 16.0, 20.0].map(ComputedCssGridTrackSize::LengthPx)
    );
    assert!(fixed_columns_auto_rows.1.rows_auto_repeat.is_some());
    assert_eq!(
        fixed_columns_auto_rows
            .1
            .template_areas
            .as_ref()
            .map(|areas| (areas.rows, areas.columns)),
        Some((1, 4))
    );

    for (columns, rows, areas) in [
        ("repeat(2,20px)", "repeat(auto-fill,20px)", None),
        (
            "repeat(2,20px) 8px",
            "repeat(auto-fill,20px)",
            Some("'a b c'"),
        ),
        (
            "repeat(2,20px)",
            "8px repeat(auto-fill,20px)",
            Some("'a b'"),
        ),
        (
            "repeat(2,20px)",
            "repeat(auto-fill,20px 24px)",
            Some("'a b'"),
        ),
        (
            "repeat(2,20px) repeat(2,24px)",
            "repeat(auto-fill,20px)",
            Some("'a b c d'"),
        ),
        (
            "repeat(2,20px)",
            "repeat(2,20px) repeat(auto-fill,20px)",
            Some("'a b'"),
        ),
        (
            "repeat(2,20px 24px 28px)",
            "repeat(auto-fill,20px)",
            Some("'a b c d e f'"),
        ),
    ] {
        let area_declaration = areas
            .map(|areas| format!(";grid-template-areas:{areas}"))
            .unwrap_or_default();
        let invalid = fixture(&format!(
            "{}{}",
            fixture_style(columns, rows),
            area_declaration
        ));
        assert!(
            compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
                &invalid.view,
                &[],
                viewport(),
                Default::default(),
            )
            .is_err(),
            "unsupported cross-axis combination accepted: columns={columns}, rows={rows}, areas={areas:?}"
        );
    }

    let unsupported_fixed_repeat_breadths = [
        "minmax(20px,40px)",
        "20%",
        "1fr",
        "min-content",
        "calc(10px + 5%)",
    ];
    for (fixed_axis, area, expected_rejection_property) in [
        ("columns", "'left right'", "grid-template-rows"),
        ("rows", "'top' 'bottom'", "grid-template-columns"),
    ] {
        for breadth in unsupported_fixed_repeat_breadths {
            let (columns, rows) = if fixed_axis == "columns" {
                (
                    format!("repeat(2,{breadth})"),
                    "repeat(auto-fill,20px)".to_owned(),
                )
            } else {
                (
                    "repeat(auto-fill,20px)".to_owned(),
                    format!("repeat(2,{breadth})"),
                )
            };
            let area_declaration = format!(
                "{};grid-template-areas:{area}",
                fixture_style(&columns, &rows)
            );
            let invalid = fixture(&area_declaration);
            let error = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
                &invalid.view,
                &[],
                viewport(),
                Default::default(),
            )
            .expect_err("unsupported cross-axis fixed-repeat breadth must fail closed");
            assert!(
                matches!(
                    error,
                    crate::CssCascadeError::UnsupportedComputedGridValue {
                        node,
                        property,
                        ..
                    } if node == invalid.root && property == expected_rejection_property
                ),
                "unsupported fixed {fixed_axis} breadth {breadth} must reject its track-list property: {error:?}"
            );
        }
    }
}

#[test]
fn same_axis_fixed_auto_repeat_areas_preserve_tracks_lines_and_rectangles_on_both_axes() {
    let columns_style = format!(
        "{};grid-template-areas:'left left left left hero hero hero hero right right'",
        fixture_style_with_size(
            "repeat(2,[hero-start] 10px 6px [hero-end]) repeat(auto-fill,[hero-start] 20px [hero-end]) repeat(2,10px)",
            "20px",
            "168px",
            "20px",
        )
    );
    let (_, columns_grid) = computed_grid(&columns_style);
    assert_eq!(columns_grid.columns.len(), 7);
    assert_eq!(columns_grid.columns_line_names.len(), 8);
    assert_eq!(
        columns_grid.columns_auto_repeat.as_ref().map(|repeat| {
            (
                repeat.mode,
                repeat.start,
                repeat.end,
                repeat.line_names.clone(),
            )
        }),
        Some((
            ComputedCssGridAutoRepeatMode::AutoFill,
            4,
            5,
            vec![vec!["hero-start".to_owned()], vec!["hero-end".to_owned()]],
        ))
    );
    assert_eq!(columns_grid.rows_auto_repeat, None);
    assert_eq!(
        columns_grid.template_areas.as_ref().map(|areas| (
            areas.rows,
            areas.columns,
            areas.areas.len()
        )),
        Some((1, 10, 3))
    );
    let column_areas = columns_grid.template_areas.as_ref().unwrap();
    let mut column_rectangles = column_areas
        .areas
        .iter()
        .map(|area| {
            (
                area.name.as_str(),
                area.row_start,
                area.row_end,
                area.column_start,
                area.column_end,
            )
        })
        .collect::<Vec<_>>();
    column_rectangles.sort_unstable();
    assert_eq!(
        column_rectangles,
        [
            ("hero", 0, 1, 4, 8),
            ("left", 0, 1, 0, 4),
            ("right", 0, 1, 8, 10),
        ]
    );

    let rows_style = format!(
        "{};grid-template-areas:'left' 'left' 'left' 'left' 'hero' 'hero' 'hero' 'hero' 'right' 'right'",
        fixture_style_with_size(
            "20px",
            "repeat(2,[hero-start] 10px 6px [hero-end]) repeat(auto-fill,[hero-start] 20px [hero-end]) repeat(2,10px)",
            "20px",
            "168px",
        )
    );
    let (_, rows_grid) = computed_grid(&rows_style);
    assert_eq!(rows_grid.rows.len(), 7);
    assert_eq!(rows_grid.rows_line_names.len(), 8);
    assert_eq!(
        rows_grid.rows_auto_repeat.as_ref().map(|repeat| {
            (
                repeat.mode,
                repeat.start,
                repeat.end,
                repeat.line_names.clone(),
            )
        }),
        Some((
            ComputedCssGridAutoRepeatMode::AutoFill,
            4,
            5,
            vec![vec!["hero-start".to_owned()], vec!["hero-end".to_owned()]],
        ))
    );
    assert_eq!(rows_grid.columns_auto_repeat, None);
    assert_eq!(
        rows_grid.template_areas.as_ref().map(|areas| (
            areas.rows,
            areas.columns,
            areas.areas.len()
        )),
        Some((10, 1, 3))
    );
    let row_areas = rows_grid.template_areas.as_ref().unwrap();
    let mut row_rectangles = row_areas
        .areas
        .iter()
        .map(|area| {
            (
                area.name.as_str(),
                area.row_start,
                area.row_end,
                area.column_start,
                area.column_end,
            )
        })
        .collect::<Vec<_>>();
    row_rectangles.sort_unstable();
    assert_eq!(
        row_rectangles,
        [
            ("hero", 4, 8, 0, 1),
            ("left", 0, 4, 0, 1),
            ("right", 8, 10, 0, 1),
        ]
    );

    let shorter_area_style = format!(
        "{};grid-template-areas:'left left left left hero hero hero right'",
        fixture_style_with_size(
            "repeat(2,[hero-start] 10px 6px [hero-end]) repeat(auto-fill,[hero-start] 20px [hero-end]) repeat(2,10px)",
            "20px",
            "188px",
            "20px",
        )
    );
    let (_, shorter_area_grid) = computed_grid(&shorter_area_style);
    assert_eq!(shorter_area_grid.columns.len(), 7);
    assert_eq!(
        shorter_area_grid.template_areas.as_ref().map(|areas| (
            areas.rows,
            areas.columns,
            areas.areas.len()
        )),
        Some((1, 8, 3))
    );
}

#[test]
fn same_axis_fixed_auto_repeat_areas_match_grid_template_and_grid_shorthands() {
    let columns = "repeat(2,[hero-start] 10px 6px [hero-end]) repeat(auto-fill,[hero-start] 20px [hero-end]) repeat(2,10px)";
    let area_row = "left left left left hero hero hero hero right right";
    let longhand_style = format!(
        "{};grid-template-areas:'{area_row}'",
        fixture_style_with_size(columns, "20px", "168px", "20px")
    );
    let (_, longhand) = computed_grid(&longhand_style);

    for shorthand in ["grid-template", "grid"] {
        let fixture = fixture(
            "display:grid;box-sizing:border-box;width:168px;height:20px;margin:0;padding:0;column-gap:8px;row-gap:8px",
        );
        let stylesheet = StylesheetSource {
            id: format!("c114-{shorthand}-areas"),
            base_url: format!("https://spinon.invalid/c114-{shorthand}-areas.css"),
            origin: CssOrigin::Author,
            css: format!(
                "#root {{{shorthand}:20px / {columns};grid-template-areas:\"{area_row}\";}}"
            ),
        };
        let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &fixture.view,
            &[stylesheet],
            viewport(),
            Default::default(),
        )
        .unwrap_or_else(|error| panic!("{shorthand} cascade: {error:?}"));
        let computed = snapshot.elements[0].grid_layout.as_ref().unwrap();
        assert_eq!(computed.columns.len(), 7, "{shorthand}");
        assert_eq!(computed.columns, longhand.columns, "{shorthand}");
        assert_eq!(
            computed.columns_line_names, longhand.columns_line_names,
            "{shorthand}"
        );
        assert_eq!(
            computed.columns_auto_repeat, longhand.columns_auto_repeat,
            "{shorthand}"
        );
        assert_eq!(
            computed.template_areas, longhand.template_areas,
            "{shorthand}"
        );
    }
}

#[test]
fn same_node_area_none_area_revisions_do_not_retain_stale_area_metadata() {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let child = document.reserve_node_handle().unwrap();
    let base_style = "display:grid;box-sizing:border-box;width:168px;height:20px;grid-template-columns:repeat(2,[area-start] 10px [area-end]) repeat(auto-fill,[area-start] 20px [area-end]) repeat(2,10px);grid-template-rows:20px;column-gap:8px;row-gap:8px;margin:0;padding:0";
    let with_area = format!("{base_style};grid-template-areas:'area area area area'");
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(6_802).unwrap(), document.document_revision());
    for (node, name, parent, style) in [
        (root, "root", HostParent::Root, with_area.as_str()),
        (
            child,
            "child",
            HostParent::Node(root),
            "display:block;width:10px;height:10px;margin:0;padding:0",
        ),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "id").unwrap(),
            value: name.to_owned().into(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "style").unwrap(),
            value: style.to_owned().into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();

    let snapshot_for_style = |document: &HostDocument, style_revision| {
        let view =
            StyloDocumentView::new_html_fragment_child_shared(Arc::new(document.snapshot()), root)
                .unwrap();
        compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &view,
            &[],
            viewport(),
            style_revision,
        )
        .unwrap()
    };
    let initial = snapshot_for_style(&document, StyleRevision::INITIAL);
    let initial_grid = initial
        .elements
        .iter()
        .find(|element| element.node_id == root.id())
        .and_then(|element| element.grid_layout.clone())
        .unwrap();
    assert_eq!(initial_grid.template_areas.as_ref().unwrap().columns, 4);

    let without_area = format!("{base_style};grid-template-areas:none");
    let mut update =
        DocumentChangeBatch::new(OwnerId::new(6_802).unwrap(), document.document_revision());
    update.push(DocumentOperation::SetAttribute {
        node: root,
        name: AttributeName::new(None, "style").unwrap(),
        value: without_area.into(),
    });
    document.commit(update).unwrap();
    let without_area_revision = StyleRevision::INITIAL.checked_next().unwrap();
    let cleared = snapshot_for_style(&document, without_area_revision);
    let cleared_grid = cleared
        .elements
        .iter()
        .find(|element| element.node_id == root.id())
        .and_then(|element| element.grid_layout.as_ref())
        .unwrap();
    assert_eq!(cleared_grid.template_areas, None);
    assert_eq!(
        cleared_grid.columns_line_names,
        initial_grid.columns_line_names
    );

    let mut update =
        DocumentChangeBatch::new(OwnerId::new(6_802).unwrap(), document.document_revision());
    update.push(DocumentOperation::SetAttribute {
        node: root,
        name: AttributeName::new(None, "style").unwrap(),
        value: with_area.into(),
    });
    document.commit(update).unwrap();
    let restored_revision = without_area_revision.checked_next().unwrap();
    let restored = snapshot_for_style(&document, restored_revision);
    let restored_grid = restored
        .elements
        .iter()
        .find(|element| element.node_id == root.id())
        .and_then(|element| element.grid_layout.as_ref())
        .unwrap();
    assert_eq!(restored_grid.template_areas, initial_grid.template_areas);
    assert_eq!(
        restored_grid.columns_line_names,
        initial_grid.columns_line_names
    );
    assert_eq!(
        restored_grid.columns_auto_repeat,
        initial_grid.columns_auto_repeat
    );
}

#[test]
fn four_fixed_repeat_groups_obey_descriptor_limit_without_partial_tracks() {
    let within_limit = fixture(&fixture_style(
        "repeat(2499,1px) repeat(2500,1px) repeat(2500,1px) repeat(2500,1px) repeat(auto-fill,minmax(32px,1fr))",
        "200px",
    ));
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &within_limit.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let grid = snapshot.elements[0].grid_layout.as_ref().unwrap();
    assert_eq!(grid.columns.len(), 10_000);
    assert_eq!(
        grid.columns_auto_repeat
            .as_ref()
            .map(|repeat| (repeat.start, repeat.end)),
        Some((9_999, 10_000))
    );
    assert_eq!(grid.columns_line_names.len(), 10_001);

    let over_limit = fixture(&fixture_style(
        "repeat(2500,1px) repeat(2500,1px) repeat(2500,1px) repeat(2500,1px) repeat(auto-fill,minmax(32px,1fr))",
        "200px",
    ));
    assert!(matches!(
        compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &over_limit.view,
            &[],
            viewport(),
            Default::default(),
        ),
        Err(crate::CssCascadeError::UnsupportedComputedGridValue {
            node,
            property: "grid-template-columns",
            ..
        }) if node == over_limit.root
    ));
}

#[test]
fn fixed_repeat_allocation_failures_are_contextual_and_recoverable() {
    use super::super::incremental::{
        IntegerRepeatReservationSite, inject_integer_repeat_reservation_failure,
    };

    let fixture = fixture(&fixture_style(
        "repeat(2,20px 10px) repeat(3,12px) repeat(4,8px) repeat(auto-fill,minmax(32px,1fr))",
        "200px",
    ));
    for site in [
        IntegerRepeatReservationSite::FixedRepeatGroups,
        IntegerRepeatReservationSite::Tracks,
        IntegerRepeatReservationSite::LineGroups,
        IntegerRepeatReservationSite::LineNames,
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

#[test]
fn fixed_repeat_mixed_into_both_auto_repeat_axes_is_rejected_in_both_orientations() {
    for (columns, rows, areas, expected_property) in [
        (
            "repeat(2,20px) repeat(auto-fill,20px)",
            "repeat(auto-fill,20px)",
            "'a b'",
            "grid-template-rows",
        ),
        (
            "repeat(auto-fill,20px)",
            "repeat(2,20px) repeat(auto-fill,20px)",
            "'a' 'b'",
            "grid-template-columns",
        ),
    ] {
        let invalid = fixture(&format!(
            "{};grid-template-areas:{areas}",
            fixture_style(columns, rows)
        ));
        let error = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &invalid.view,
            &[],
            viewport(),
            Default::default(),
        )
        .expect_err("fixed repeat mixed into two auto-repeat axes must be unsupported");

        assert!(
            matches!(
                error,
                crate::CssCascadeError::UnsupportedComputedGridValue {
                    node,
                    property,
                    ..
                } if node == invalid.root && property == expected_property
            ),
            "wrong rejection for columns={columns}, rows={rows}, areas={areas}: {error:?}"
        );
    }
}
