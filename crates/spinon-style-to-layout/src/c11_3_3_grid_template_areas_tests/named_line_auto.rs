use super::Fixture;

#[test]
fn positive_no_area_named_lines_use_auto_implicit_tracks_on_both_axes() {
    let columns = Fixture::new(
        "display:grid;box-sizing:border-box;width:80px;height:20px;grid-template-columns:40px 40px;grid-template-rows:20px;grid-auto-rows:auto;grid-auto-columns:auto;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[(
            "column-item",
            "box-sizing:border-box;width:30px;height:10px;grid-column-start:missing-column;grid-row:1;margin:0;padding:0;border:0",
        )],
    );
    let output = columns.compute_with_implicit_named_lines(true).unwrap();
    let frame = output.layout.frames[&columns.nodes["column-item"].id()];
    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        (80.0, 0.0, 30.0, 10.0)
    );

    let fixed_columns = Fixture::new(
        "display:grid;box-sizing:border-box;width:80px;height:20px;grid-template-columns:40px 40px;grid-template-rows:20px;grid-auto-rows:auto;grid-auto-columns:30px;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[(
            "fixed-column-item",
            "box-sizing:border-box;width:30px;height:10px;grid-column-start:missing-column;grid-row:1;margin:0;padding:0;border:0",
        )],
    );
    let output = fixed_columns
        .compute_with_implicit_named_lines(true)
        .unwrap();
    let frame = output.layout.frames[&fixed_columns.nodes["fixed-column-item"].id()];
    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        (110.0, 0.0, 30.0, 10.0)
    );

    let rows = Fixture::new(
        "display:grid;box-sizing:border-box;width:40px;height:20px;grid-template-columns:40px;grid-template-rows:20px;grid-auto-rows:auto;grid-auto-columns:auto;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[(
            "row-item",
            "box-sizing:border-box;width:20px;height:25px;grid-column:1;grid-row-start:missing-row;margin:0;padding:0;border:0",
        )],
    );
    let output = rows.compute_with_implicit_named_lines(true).unwrap();
    let frame = output.layout.frames[&rows.nodes["row-item"].id()];
    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        (0.0, 20.0, 20.0, 25.0)
    );
}

#[test]
fn positive_no_area_named_line_pair_sizes_auto_implicit_tracks() {
    let column_scenes = [
        ("alpha / omega", (80.0, 0.0, 30.0, 10.0)),
        ("2 alpha / 3 omega", (80.0, 0.0, 30.0, 10.0)),
        ("1 alpha / 1 alpha", (80.0, 0.0, 30.0, 10.0)),
        ("2 omega / 1 alpha", (80.0, 0.0, 30.0, 10.0)),
        ("1 alpha / 3 omega", (80.0, 0.0, 30.0, 10.0)),
    ];
    for (placement, expected) in column_scenes {
        let fixture = Fixture::new(
            "display:grid;box-sizing:border-box;width:80px;height:20px;grid-template-columns:40px 40px;grid-template-rows:20px;grid-auto-rows:auto;grid-auto-columns:auto;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
            &[(
                "pair-item",
                &format!(
                    "box-sizing:border-box;width:30px;height:10px;grid-column:{placement};grid-row:1;margin:0;padding:0;border:0"
                ),
            )],
        );
        let output = fixture.compute_with_implicit_named_lines(true).unwrap();
        let frame = output.layout.frames[&fixture.nodes["pair-item"].id()];
        assert_eq!(
            (frame.x, frame.y, frame.width, frame.height),
            expected,
            "grid-column:{placement}"
        );
    }

    let rows = Fixture::new(
        "display:grid;box-sizing:border-box;width:40px;height:40px;grid-template-columns:40px;grid-template-rows:20px 20px;grid-auto-rows:auto;grid-auto-columns:auto;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[(
            "pair-item",
            "box-sizing:border-box;width:20px;height:30px;grid-column:1;grid-row:1 alpha / 3 omega;margin:0;padding:0;border:0",
        )],
    );
    let output = rows.compute_with_implicit_named_lines(true).unwrap();
    let frame = output.layout.frames[&rows.nodes["pair-item"].id()];
    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        (0.0, 40.0, 20.0, 30.0)
    );

    let fixed = Fixture::new(
        "display:grid;box-sizing:border-box;width:80px;height:20px;grid-template-columns:40px 40px;grid-template-rows:20px;grid-auto-rows:auto;grid-auto-columns:30px;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[(
            "pair-item",
            "box-sizing:border-box;width:30px;height:10px;grid-column:1 alpha / 2 omega;grid-row:1;margin:0;padding:0;border:0",
        )],
    );
    let output = fixed.compute_with_implicit_named_lines(true).unwrap();
    let frame = output.layout.frames[&fixed.nodes["pair-item"].id()];
    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        (110.0, 0.0, 30.0, 10.0)
    );
}

#[test]
fn positive_no_area_named_line_pair_auto_sizing_fails_closed_outside_the_slice() {
    for (item_style, expected_error) in [
        (
            "box-sizing:border-box;width:30px;height:10px;grid-column:alpha / 2;grid-row:1;margin:0;padding:0;border:0",
            "UnsupportedGridAutoColumns",
        ),
        (
            "box-sizing:border-box;width:auto;height:10px;grid-column:alpha / omega;grid-row:1;margin:0;padding:0;border:0",
            "UnsupportedGridAutoColumns",
        ),
        (
            "box-sizing:border-box;width:30px;height:10px;grid-column:alpha / omega;grid-row:alpha / omega;margin:0;padding:0;border:0",
            "UnsupportedGridAutoRows",
        ),
    ] {
        let fixture = Fixture::new(
            "display:grid;box-sizing:border-box;width:80px;height:20px;grid-template-columns:40px 40px;grid-template-rows:20px;grid-auto-rows:auto;grid-auto-columns:auto;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
            &[("pair-item", item_style)],
        );
        let error = fixture.compute_with_implicit_named_lines(true).unwrap_err();
        assert!(
            format!("{error:?}").contains(expected_error),
            "item style {item_style:?} returned {error:?}"
        );
    }

    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:80px;height:20px;grid-template-columns:40px 40px;grid-template-rows:20px;grid-auto-rows:auto;grid-auto-columns:auto;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[
            (
                "pair-item",
                "box-sizing:border-box;width:30px;height:10px;grid-column:alpha / omega;grid-row:1;margin:0;padding:0;border:0",
            ),
            (
                "second-item",
                "box-sizing:border-box;width:20px;height:10px;grid-column:1;grid-row:1;margin:0;padding:0;border:0",
            ),
        ],
    );
    let error = fixture.compute_with_implicit_named_lines(true).unwrap_err();
    assert!(format!("{error:?}").contains("InvalidGridPlacement"));
}
