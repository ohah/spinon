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

#[test]
fn negative_no_area_named_line_pair_sizes_leading_implicit_tracks() {
    let column_scenes = [
        ("-4 ghost / -1 ghost", (0.0, 0.0, 17.0, 10.0)),
        ("-1 ghost / -4 ghost", (0.0, 0.0, 17.0, 10.0)),
        ("-2 ghost / -1 ghost", (0.0, 0.0, 17.0, 10.0)),
        ("-2 ghost / -2 ghost", (0.0, 0.0, 17.0, 10.0)),
        ("-5 ghost / -2 ghost", (0.0, 0.0, 17.0, 10.0)),
    ];
    for (placement, expected) in column_scenes {
        let fixture = Fixture::new(
            "display:grid;box-sizing:border-box;width:160px;height:160px;grid-template-columns:20px 30px 40px;grid-template-rows:20px 30px 40px;grid-auto-columns:5px 7px;grid-auto-rows:5px 7px;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
            &[(
                "negative-pair-item",
                &format!(
                    "box-sizing:border-box;width:17px;height:10px;grid-column:{placement};grid-row:1 / 2;margin:0;padding:0;border:0"
                ),
            )],
        );
        let output = fixture.compute_with_implicit_named_lines(true).unwrap();
        let frame = output.layout.frames[&fixture.nodes["negative-pair-item"].id()];
        assert_eq!(
            (frame.x, frame.y, frame.width, frame.height),
            expected,
            "grid-column:{placement}"
        );
    }

    let columns_auto = Fixture::new(
        "display:grid;box-sizing:border-box;width:108px;height:160px;grid-template-columns:20px 30px 40px;grid-template-rows:20px 30px 40px;grid-auto-columns:auto;grid-auto-rows:5px 7px;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[(
            "negative-auto-item",
            "box-sizing:border-box;width:18px;height:10px;grid-column:-4 ghost / -1 ghost;grid-row:1 / 2;margin:0;padding:0;border:0",
        )],
    );
    let output = columns_auto
        .compute_with_implicit_named_lines(true)
        .unwrap();
    let frame = output.layout.frames[&columns_auto.nodes["negative-auto-item"].id()];
    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        (0.0, 0.0, 18.0, 10.0)
    );

    let rows = Fixture::new(
        "display:grid;box-sizing:border-box;width:160px;height:160px;grid-template-columns:20px 30px 40px;grid-template-rows:20px 30px 40px;grid-auto-columns:5px 7px;grid-auto-rows:5px 7px;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[(
            "negative-row-item",
            "box-sizing:border-box;width:10px;height:17px;grid-column:1 / 2;grid-row:-4 ghost / -1 ghost;margin:0;padding:0;border:0",
        )],
    );
    let output = rows.compute_with_implicit_named_lines(true).unwrap();
    let frame = output.layout.frames[&rows.nodes["negative-row-item"].id()];
    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        (0.0, 0.0, 10.0, 17.0)
    );

    let rows_auto = Fixture::new(
        "display:grid;box-sizing:border-box;width:160px;height:108px;grid-template-columns:20px 30px 40px;grid-template-rows:20px 30px 40px;grid-auto-columns:5px 7px;grid-auto-rows:auto;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[(
            "negative-row-auto-item",
            "box-sizing:border-box;width:10px;height:18px;grid-column:1 / 2;grid-row:-4 ghost / -1 ghost;margin:0;padding:0;border:0",
        )],
    );
    let output = rows_auto.compute_with_implicit_named_lines(true).unwrap();
    let frame = output.layout.frames[&rows_auto.nodes["negative-row-auto-item"].id()];
    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        (0.0, 0.0, 10.0, 18.0)
    );

    let authored_line = Fixture::new(
        "display:grid;box-sizing:border-box;width:160px;height:160px;grid-template-columns:[ghost] 20px 30px 40px;grid-template-rows:20px 30px 40px;grid-auto-columns:5px 7px;grid-auto-rows:5px 7px;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[(
            "authored-control-item",
            "box-sizing:border-box;width:17px;height:10px;grid-column:-2 ghost / -1 ghost;grid-row:1 / 2;margin:0;padding:0;border:0",
        )],
    );
    let output = authored_line
        .compute_with_implicit_named_lines(true)
        .unwrap();
    let frame = output.layout.frames[&authored_line.nodes["authored-control-item"].id()];
    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        (0.0, 0.0, 17.0, 10.0)
    );
}

#[test]
fn negative_named_line_pair_auto_tracks_accept_start_content_alignment() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:120px;height:100px;grid-template-columns:20px;grid-template-rows:15px;grid-auto-columns:auto;grid-auto-rows:auto;justify-content:start;align-content:start;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[(
            "two-axis-item",
            "box-sizing:border-box;width:18px;height:12px;grid-column:-4 ghost / -1 ghost;grid-row:-4 ghost / -1 ghost;margin:0;padding:0;border:0",
        )],
    );
    let output = fixture.compute_with_implicit_named_lines(true).unwrap();
    let frame = output.layout.frames[&fixture.nodes["two-axis-item"].id()];
    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        (0.0, 0.0, 18.0, 12.0)
    );
}

#[test]
fn negative_named_line_auto_pair_keeps_unsupported_shapes_closed() {
    for (grid_style, item_styles, expected_error) in [
        (
            "display:grid;box-sizing:border-box;width:160px;height:160px;grid-template-columns:20px 30px 40px;grid-template-rows:20px;grid-auto-columns:auto;grid-auto-rows:20px;margin:0;padding:0;border:0",
            vec![
                "box-sizing:border-box;width:auto;height:10px;grid-column:-4 ghost / -1 ghost;grid-row:1 / 2;margin:0;padding:0;border:0",
            ],
            "UnsupportedGridAutoColumns",
        ),
        (
            "display:grid;box-sizing:border-box;width:160px;height:160px;grid-template-columns:20px 30px 40px;grid-template-rows:20px;grid-auto-columns:auto;grid-auto-rows:20px;margin:0;padding:0;border:0",
            vec![
                "box-sizing:border-box;width:17px;height:10px;grid-column:-4 start-ghost / -1 end-ghost;grid-row:1 / 2;margin:0;padding:0;border:0",
            ],
            "UnsupportedGridAutoColumns",
        ),
        (
            "display:grid;box-sizing:border-box;width:160px;height:160px;grid-template-columns:20px 30px 40px;grid-template-rows:20px 30px 40px;grid-auto-columns:5px 7px;grid-auto-rows:5px 7px;margin:0;padding:0;border:0",
            vec![
                "box-sizing:border-box;width:17px;height:10px;grid-column:-4 ghost / -1 ghost;grid-row:1 / 2;margin:0;padding:0;border:0",
                "box-sizing:border-box;width:10px;height:10px;grid-column:1 / 2;grid-row:1 / 2;margin:0;padding:0;border:0",
            ],
            "InvalidGridPlacement",
        ),
        (
            "display:grid;box-sizing:border-box;width:160px;height:160px;grid-template-columns:20px 30px 40px;grid-template-rows:20px 30px 40px;grid-auto-columns:auto;grid-auto-rows:auto;margin:0;padding:0;border:0",
            vec![
                "box-sizing:border-box;width:17px;height:10px;grid-column:-4 ghost / -1 ghost;grid-row:-4 row-ghost / -1 row-ghost;margin:0;padding:0;border:0",
            ],
            "UnsupportedGridAutoRows",
        ),
    ] {
        let items = item_styles
            .iter()
            .enumerate()
            .map(|(index, style)| (format!("negative-item-{index}"), *style))
            .collect::<Vec<_>>();
        let item_refs = items
            .iter()
            .map(|(name, style)| (name.as_str(), *style))
            .collect::<Vec<_>>();
        let fixture = Fixture::new(grid_style, &item_refs);
        let error = fixture.compute_with_implicit_named_lines(true).unwrap_err();
        assert!(
            format!("{error:?}").contains(expected_error),
            "expected {expected_error}, got {error:?}"
        );
    }
}
