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
    let cross_axis_fixture = fixture(&fixture_style(
        "repeat(2,20px) repeat(auto-fill,minmax(32px,1fr))",
        "repeat(auto-fit,minmax(32px,1fr))",
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

    let fixed_repeat_on_other_axis = fixture(&fixture_style(
        "repeat(auto-fill,minmax(32px,1fr))",
        "repeat(2,20px)",
    ));
    assert!(matches!(
        compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &fixed_repeat_on_other_axis.view,
            &[],
            viewport(),
            Default::default(),
        ),
        Err(crate::CssCascadeError::UnsupportedComputedGridValue {
            node,
            property: "grid-template-columns",
            ..
        }) if node == fixed_repeat_on_other_axis.root
    ));
}

#[test]
fn fixed_repeat_mixture_rejects_template_areas_without_changing_auto_repeat_area_profile() {
    let area_fixture = fixture(
        "display:grid;box-sizing:border-box;width:200px;height:200px;grid-template-columns:repeat(2,20px) repeat(auto-fill,minmax(32px,1fr));grid-template-rows:40px;grid-template-areas:'area';column-gap:8px;row-gap:8px;margin:0;padding:0",
    );
    assert!(matches!(
        compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &area_fixture.view,
            &[],
            viewport(),
            Default::default(),
        ),
        Err(crate::CssCascadeError::UnsupportedComputedGridValue {
            node,
            property: "grid-template-areas",
            ..
        }) if node == area_fixture.root
    ));

    let existing_auto_repeat_profile = fixture(
        "display:grid;box-sizing:border-box;width:200px;height:200px;grid-template-columns:repeat(auto-fill,minmax(32px,1fr));grid-template-rows:40px;grid-template-areas:'area';column-gap:8px;row-gap:8px;margin:0;padding:0",
    );
    assert!(
        compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &existing_auto_repeat_profile.view,
            &[],
            viewport(),
            Default::default(),
        )
        .is_ok()
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
