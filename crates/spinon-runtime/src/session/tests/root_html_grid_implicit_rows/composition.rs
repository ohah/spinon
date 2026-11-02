use super::*;

fn replace_once(source: &str, needle: &str, replacement: &str) -> String {
    assert_eq!(
        source.matches(needle).count(),
        1,
        "fixture token {needle:?}"
    );
    source.replacen(needle, replacement, 1)
}

fn composed_track_source(
    width: u32,
    height: u32,
    columns: &str,
    rows: &str,
    column_gap: u32,
    row_gap: u32,
    item_widths: [u32; 3],
) -> String {
    let mut source = SOURCE.to_owned();
    for (needle, replacement) in [
        ("width: 100px;", format!("width: {width}px;")),
        ("height: 40px;", format!("height: {height}px;")),
        (
            "grid-template-columns: 40px 55px;",
            format!("grid-template-columns: {columns};"),
        ),
        (
            "grid-template-rows: 20px;",
            format!("grid-template-rows: {rows};"),
        ),
        ("column-gap: 5px;", format!("column-gap: {column_gap}px;")),
        ("row-gap: 5px;", format!("row-gap: {row_gap}px;")),
        (
            "#a { width: 40px; height: 20px;",
            format!("#a {{ width: {}px; height: 10px;", item_widths[0]),
        ),
        (
            "#b { width: 55px; height: 20px;",
            format!("#b {{ width: {}px; height: 10px;", item_widths[1]),
        ),
        (
            "#c { width: 40px; height: 15px;",
            format!("#c {{ width: {}px; height: 10px;", item_widths[2]),
        ),
    ] {
        source = replace_once(&source, needle, &replacement);
    }
    source
}

fn assert_computed_tracks(
    session: &RuntimeSession,
    columns: &[spinon_style::ComputedCssGridTrackSize],
    rows: &[spinon_style::ComputedCssGridTrackSize],
) {
    let cascade = session.ua_cascade_snapshot();
    let completed = cascade
        .completed
        .expect("Root HTML computed styles should be available");
    let grid = completed.roots[0]
        .styles
        .elements
        .iter()
        .find_map(|style| style.grid_layout.as_ref())
        .expect("Root HTML Grid computed values should remain observable");
    assert_eq!(grid.columns, columns);
    assert_eq!(grid.rows, rows);
}

#[test]
fn c11_1_root_html_default_runtime_combines_fraction_percentage_and_fixed_implicit_rows() {
    let fraction_columns = composed_track_source(120, 35, "1fr 2fr", "20px", 0, 0, [10; 3]);
    let (session, layout) = run(&fraction_columns);
    let scene = ready_scene(&session, &layout);
    assert_computed_tracks(
        &session,
        &[
            spinon_style::ComputedCssGridTrackSize::Fraction(1.0),
            spinon_style::ComputedCssGridTrackSize::Fraction(2.0),
        ],
        &[spinon_style::ComputedCssGridTrackSize::LengthPx(20.0)],
    );
    assert_frames(
        &scene,
        &[
            (ROOT, [0.0, 0.0, 120.0, 35.0]),
            (RED, [0.0, 0.0, 10.0, 10.0]),
            (BLUE, [40.0, 0.0, 10.0, 10.0]),
            (GREEN, [0.0, 20.0, 10.0, 10.0]),
        ],
    );

    let percentage_columns = composed_track_source(120, 40, "25% 75%", "20px", 5, 5, [10, 100, 10]);
    let (session, layout) = run(&percentage_columns);
    let scene = ready_scene(&session, &layout);
    assert_computed_tracks(
        &session,
        &[
            spinon_style::ComputedCssGridTrackSize::Percentage(0.25),
            spinon_style::ComputedCssGridTrackSize::Percentage(0.75),
        ],
        &[spinon_style::ComputedCssGridTrackSize::LengthPx(20.0)],
    );
    assert_frames(
        &scene,
        &[
            (ROOT, [0.0, 0.0, 120.0, 40.0]),
            (RED, [0.0, 0.0, 10.0, 10.0]),
            (BLUE, [35.0, 0.0, 100.0, 10.0]),
            (GREEN, [0.0, 25.0, 10.0, 10.0]),
        ],
    );
    assert_eq!(
        frame_for_paint(&scene, BLUE)[0] + frame_for_paint(&scene, BLUE)[2],
        135.0
    );

    let fraction_rows = composed_track_source(120, 60, "120px", "1fr 2fr", 0, 0, [10; 3]);
    let (session, layout) = run(&fraction_rows);
    let scene = ready_scene(&session, &layout);
    assert_computed_tracks(
        &session,
        &[spinon_style::ComputedCssGridTrackSize::LengthPx(120.0)],
        &[
            spinon_style::ComputedCssGridTrackSize::Fraction(1.0),
            spinon_style::ComputedCssGridTrackSize::Fraction(2.0),
        ],
    );
    assert_frames(
        &scene,
        &[
            (ROOT, [0.0, 0.0, 120.0, 60.0]),
            (RED, [0.0, 0.0, 10.0, 10.0]),
            (BLUE, [0.0, 15.0, 10.0, 10.0]),
            (GREEN, [0.0, 45.0, 10.0, 10.0]),
        ],
    );

    let percentage_rows = composed_track_source(120, 80, "120px", "25% 25%", 0, 0, [10; 3]);
    let (session, layout) = run(&percentage_rows);
    let scene = ready_scene(&session, &layout);
    assert_computed_tracks(
        &session,
        &[spinon_style::ComputedCssGridTrackSize::LengthPx(120.0)],
        &[
            spinon_style::ComputedCssGridTrackSize::Percentage(0.25),
            spinon_style::ComputedCssGridTrackSize::Percentage(0.25),
        ],
    );
    assert_frames(
        &scene,
        &[
            (ROOT, [0.0, 0.0, 120.0, 80.0]),
            (RED, [0.0, 0.0, 10.0, 10.0]),
            (BLUE, [0.0, 20.0, 10.0, 10.0]),
            (GREEN, [0.0, 40.0, 10.0, 10.0]),
        ],
    );
}
