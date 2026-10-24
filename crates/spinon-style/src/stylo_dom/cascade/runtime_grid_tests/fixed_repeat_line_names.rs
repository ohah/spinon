use super::*;

fn style(columns: &str) -> String {
    format!(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:{columns};grid-template-rows:40px;column-gap:4px;row-gap:4px;margin:0;padding:0"
    )
}

fn computed_grid(style: &str) -> (Fixture, crate::ComputedCssGridLayout) {
    let fixture = fixture(style);
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap_or_else(|error| panic!("track sizing cascade for fixed-repeat named lines: {error:?}"));
    let grid = snapshot
        .elements
        .iter()
        .find(|element| element.node_id == fixture.root)
        .and_then(|element| element.grid_layout.clone())
        .expect("grid root has a computed grid layout");
    (fixture, grid)
}

fn groups(source: &[&[&str]]) -> Vec<Vec<String>> {
    source
        .iter()
        .map(|group| group.iter().map(|name| (*name).to_owned()).collect())
        .collect()
}

#[test]
fn fixed_repeat_named_lines_expand_and_keep_auto_repeat_groups_separate() {
    let (_, grid) = computed_grid(&style(
        "[outer-start] repeat(2,[start] 12px [seam] 8px [join]) [join] repeat(2,[mid] 6px [seam] 6px [mid-end]) [seam] repeat(auto-fill,[seam] 24px [repeat-end]) [repeat-end] repeat(2,[join] 10px [tail]) [outer-end]",
    ));

    assert_eq!(grid.columns.len(), 11);
    assert_eq!(
        grid.columns_auto_repeat
            .as_ref()
            .map(|repeat| (repeat.start, repeat.end)),
        Some((8, 9))
    );
    assert_eq!(
        grid.columns_line_names,
        groups(&[
            &["outer-start", "start"],
            &["seam"],
            &["join", "start"],
            &["seam"],
            &["join", "join", "mid"],
            &["seam"],
            &["mid-end", "mid"],
            &["seam"],
            &["mid-end", "seam"],
            &["repeat-end", "join"],
            &["tail", "join"],
            &["tail", "outer-end"],
        ])
    );
    assert_eq!(
        grid.columns_auto_repeat.as_ref().unwrap().line_names,
        groups(&[&["seam"], &["repeat-end"]])
    );
}

#[test]
fn fixed_repeat_and_ordinary_neighbor_names_merge_in_authored_order() {
    let (_, grid) = computed_grid(&style(
        "[before] repeat(2,[fixed] 10px [edge]) [edge] 8px [ordinary-end auto-start] repeat(auto-fill,[auto-start] 12px [auto-end]) [after]",
    ));

    assert_eq!(
        grid.columns_line_names,
        groups(&[
            &["before", "fixed"],
            &["edge", "fixed"],
            &["edge", "edge"],
            &["ordinary-end", "auto-start"],
            &["after"],
        ])
    );
    assert_eq!(
        grid.columns_auto_repeat.as_ref().unwrap().line_names,
        groups(&[&["auto-start"], &["auto-end"]])
    );
}

#[test]
fn fixed_repeat_named_line_metadata_accepts_exact_budget_and_rejects_one_byte_over() {
    let group_bytes = std::mem::size_of::<Vec<String>>();
    let name_bytes = std::mem::size_of::<String>();
    let fixed_group_headers = 4 * group_bytes;
    let auto_repeat_group_headers = 2 * group_bytes;
    let name_headers = 2 * name_bytes;
    let exact_name_length =
        (1024 * 1024 - fixed_group_headers - auto_repeat_group_headers - name_headers) / 2;
    assert_eq!(
        fixed_group_headers + auto_repeat_group_headers + name_headers + 2 * exact_name_length,
        1024 * 1024
    );

    let exact_name = "n".repeat(exact_name_length);
    let exact = fixture(&style(&format!(
        "repeat(2,[{exact_name}] 1px) repeat(auto-fill,minmax(32px,1fr))"
    )));
    let exact_snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &exact.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap_or_else(|error| panic!("exact 1 MiB line metadata budget must pass: {error:?}"));
    assert_eq!(
        exact_snapshot.elements[0]
            .grid_layout
            .as_ref()
            .unwrap()
            .columns_line_names
            .iter()
            .flatten()
            .map(String::len)
            .sum::<usize>(),
        exact_name_length * 2
    );

    let over_name = "n".repeat(exact_name_length + 1);
    let over = fixture(&style(&format!(
        "repeat(2,[{over_name}] 1px) repeat(auto-fill,minmax(32px,1fr))"
    )));
    assert!(matches!(
        compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &over.view,
            &[],
            viewport(),
            Default::default(),
        ),
        Err(crate::CssCascadeError::UnsupportedComputedGridValue {
            node,
            property: "grid-template-columns",
            ..
        }) if node == over.root
    ));
}

#[test]
fn fixed_repeat_named_line_metadata_allocation_failures_recover() {
    use super::super::incremental::{
        IntegerRepeatReservationSite, inject_integer_repeat_reservation_failure,
    };

    let fixture = fixture(&style(
        "[outer] repeat(2,[start] 12px [seam] 8px [end]) repeat(auto-fill,[repeat-start] 24px [repeat-end])",
    ));
    for site in [
        IntegerRepeatReservationSite::FixedRepeatGroups,
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
            "reservation failure at {site:?}: {error:?}"
        );
        assert!(
            compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
                &fixture.view,
                &[],
                viewport(),
                Default::default(),
            )
            .is_ok(),
            "subsequent cascade should recover after {site:?}"
        );
    }
}
