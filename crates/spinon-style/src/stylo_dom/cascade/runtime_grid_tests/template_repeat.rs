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
fn integer_repeat_accepts_track_shorthands_but_rejects_areas() {
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
    let result = compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    );
    match result {
        Err(crate::CssCascadeError::UnsupportedComputedGridValue {
            node,
            property: "grid-template-areas",
            ..
        }) => assert_eq!(node, fixture.root),
        other => panic!("area shorthand 결과: {other:?}"),
    }
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
