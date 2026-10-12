use super::*;

#[test]
fn numeric_positive_and_negative_lines_match_pinned_chrome_frames() {
    let fixture = Fixture::new(
        grid_style(),
        &[
            &auto_sized_item("grid-row:1 / 2;grid-column:1 / 3"),
            &auto_sized_item("grid-row:-2 / -1;grid-column:-2 / -1"),
        ],
    );
    let output = fixture.compute().unwrap();
    assert_eq!(
        output.computed_styles.profile,
        ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
    );
    compare_case_to_chrome(
        &output,
        "numeric-lines",
        &[
            ("c1121-numeric-grid", fixture.grid.id()),
            ("c1121-positive-lines", fixture.items[0].id()),
            ("c1121-negative-lines", fixture.items[1].id()),
        ],
    );
}

#[test]
fn spans_from_start_and_end_match_pinned_chrome_frames() {
    let fixture = Fixture::new(
        grid_style(),
        &[
            &auto_sized_item("grid-column:1 / span 2;grid-row:1 / span 2"),
            &auto_sized_item("grid-column:span 2 / 4;grid-row:span 1 / 3"),
        ],
    );
    let output = fixture.compute().unwrap();
    compare_case_to_chrome(
        &output,
        "positive-spans",
        &[
            ("c1121-span-grid", fixture.grid.id()),
            ("c1121-span-from-start", fixture.items[0].id()),
            ("c1121-span-from-end", fixture.items[1].id()),
        ],
    );
}

#[test]
fn reversed_line_pairs_are_swapped_and_match_pinned_chrome_frames() {
    let fixture = Fixture::new(
        grid_style(),
        &[&auto_sized_item("grid-column:3 / 1;grid-row:2 / 1")],
    );
    let output = fixture.compute().unwrap();
    compare_case_to_chrome(
        &output,
        "reversed-lines",
        &[
            ("c1121-reverse-grid", fixture.grid.id()),
            ("c1121-reversed-item", fixture.items[0].id()),
        ],
    );
}

#[test]
fn grid_area_and_longhand_override_match_pinned_chrome_frames() {
    let fixture = Fixture::new(
        grid_style(),
        &[
            &auto_sized_item("grid-area:1 / 1 / 3 / 3"),
            &auto_sized_item("grid-area:1 / 2 / 2 / 4;grid-column-start:1"),
        ],
    );
    let output = fixture.compute().unwrap();
    compare_case_to_chrome(
        &output,
        "shorthand-cascade",
        &[
            ("c1121-shorthand-grid", fixture.grid.id()),
            ("c1121-grid-area-item", fixture.items[0].id()),
            ("c1121-cascade-item", fixture.items[1].id()),
        ],
    );
}

#[test]
fn overlapping_explicit_items_match_pinned_chrome_geometry() {
    let fixture = Fixture::new(
        grid_style(),
        &[
            &auto_sized_item("grid-column:1 / 3;grid-row:1 / 3"),
            &auto_sized_item("grid-column:1 / 3;grid-row:1 / 3"),
        ],
    );
    let output = fixture.compute().unwrap();
    compare_case_to_chrome(
        &output,
        "overlap-source-order",
        &[
            ("c1121-overlap-grid", fixture.grid.id()),
            ("c1121-overlap-first", fixture.items[0].id()),
            ("c1121-overlap-second", fixture.items[1].id()),
        ],
    );
}
