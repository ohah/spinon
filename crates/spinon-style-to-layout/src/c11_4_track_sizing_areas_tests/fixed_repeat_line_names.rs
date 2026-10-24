use super::*;

fn item_style(column: &str, row: &str) -> String {
    format!(
        "box-sizing:border-box;width:auto;height:auto;min-width:0;min-height:0;margin:0;padding:0;border:0;grid-column:{column};grid-row:{row}"
    )
}

fn frame(fixture: &Fixture, output: &StyleLayoutOutput, id: &str) -> (f32, f32, f32, f32) {
    let value = output.layout.frames[&fixture.nodes[id].id()];
    (value.x, value.y, value.width, value.height)
}

#[test]
fn fixed_repeat_named_lines_resolve_across_fixed_and_auto_repeat_seams() {
    let root_style = "display:grid;box-sizing:border-box;width:300px;height:40px;grid-template-columns:[outer-start] repeat(2,[start] 12px [seam] 8px [join]) [join] repeat(2,[mid] 6px [seam] 6px [mid-end]) [seam] repeat(auto-fill,[seam] 24px [repeat-end]) [repeat-end] repeat(2,[join] 10px [tail]) [outer-end];grid-template-rows:40px;column-gap:4px;row-gap:4px;margin:0;padding:0;border:0";
    let fixture = Fixture::new(
        root_style,
        &[
            ("start-join", &item_style("start 2 / join 2", "1 / 2")),
            ("seam-forward", &item_style("seam 5 / seam 6", "1 / 2")),
            ("seam-negative", &item_style("seam -3 / seam -2", "1 / 2")),
            ("mid", &item_style("mid 2 / mid-end 2", "1 / 2")),
            ("tail", &item_style("tail 1 / tail 2", "1 / 2")),
            (
                "repeat-end",
                &item_style("repeat-end 1 / repeat-end 2", "1 / 2"),
            ),
            ("named-span", &item_style("span 2 mid / mid-end 2", "1 / 2")),
        ],
    );
    let output = fixture.compute();

    for (id, expected) in [
        ("start-join", (28.0, 0.0, 24.0, 40.0)),
        ("seam-forward", (96.0, 0.0, 24.0, 40.0)),
        ("seam-negative", (180.0, 0.0, 24.0, 40.0)),
        ("mid", (76.0, 0.0, 16.0, 40.0)),
        ("tail", (278.0, 0.0, 10.0, 40.0)),
        ("repeat-end", (124.0, 0.0, 24.0, 40.0)),
        ("named-span", (56.0, 0.0, 36.0, 40.0)),
    ] {
        assert_eq!(frame(&fixture, &output, id), expected, "{id}");
    }

    let grid = output
        .computed_styles
        .elements
        .iter()
        .find(|element| element.node_id == fixture.root.id())
        .and_then(|element| element.grid_layout.as_ref())
        .expect("root computed grid layout");
    assert_eq!(grid.columns_line_names.len(), 12);
    assert_eq!(grid.columns_line_names[4], vec!["join", "join", "mid"]);
    assert_eq!(
        grid.columns_auto_repeat.as_ref().unwrap().line_names,
        vec![vec!["seam"], vec!["repeat-end"]]
    );
}

#[test]
fn fixed_repeat_auto_fit_named_placement_precedes_empty_track_collapse() {
    let root_style = "display:grid;box-sizing:border-box;width:300px;height:40px;grid-template-columns:[outer-start] repeat(2,[start] 12px [seam] 8px [join]) [join] repeat(2,[mid] 6px [seam] 6px [mid-end]) [seam] repeat(auto-fit,[seam] 24px [repeat-end]) [repeat-end] repeat(2,[join] 10px [tail]) [outer-end];grid-template-rows:40px;column-gap:4px;row-gap:4px;margin:0;padding:0;border:0";
    let fixture = Fixture::new(
        root_style,
        &[(
            "fit-item",
            &item_style("repeat-end 3 / repeat-end 4", "1 / 2"),
        )],
    );
    let output = fixture.compute();

    assert_eq!(
        frame(&fixture, &output, "fit-item"),
        (96.0, 0.0, 24.0, 40.0)
    );
    let trace = output
        .layout
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == fixture.root.id())
        .expect("root Grid track-sizing trace");
    let columns = trace
        .passes
        .iter()
        .rev()
        .find(|pass| pass.axis == spinon_layout::LayoutGridTrackSizingAxis::Columns)
        .expect("final columns track-sizing pass");
    let tracks = &columns.stages.last().expect("final columns stage").tracks;
    assert_eq!(
        tracks
            .iter()
            .filter(|track| !track.is_gutter)
            .map(|track| track.base_size)
            .collect::<Vec<_>>(),
        vec![
            12.0, 8.0, 12.0, 8.0, 6.0, 6.0, 6.0, 6.0, 0.0, 0.0, 0.0, 24.0, 0.0, 0.0, 10.0, 10.0
        ]
    );
}

#[test]
fn fixed_repeat_names_merge_with_an_ordinary_neighbor_and_auto_repeat() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:120px;height:40px;grid-template-columns:[before] repeat(2,[fixed] 10px [edge]) [edge] 8px [ordinary-end auto-start] repeat(auto-fill,[auto-start] 12px [auto-end]) [after];grid-template-rows:40px;column-gap:4px;row-gap:4px;margin:0;padding:0;border:0",
        &[
            (
                "ordinary-neighbor",
                &item_style("edge 2 / auto-start 1", "1 / 2"),
            ),
            (
                "auto-neighbor",
                &item_style("auto-start 2 / auto-end 2", "1 / 2"),
            ),
        ],
    );
    let output = fixture.compute();

    assert_eq!(
        frame(&fixture, &output, "ordinary-neighbor"),
        (28.0, 0.0, 8.0, 40.0)
    );
    assert_eq!(
        frame(&fixture, &output, "auto-neighbor"),
        (56.0, 0.0, 12.0, 40.0)
    );
}

#[test]
fn row_axis_named_lines_expand_with_multiple_fixed_repeats() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:40px;height:300px;grid-template-columns:40px;grid-template-rows:[outer-start] repeat(2,[start] 12px [seam] 8px [join]) [join] repeat(2,[mid] 6px [seam] 6px [mid-end]) [seam] repeat(auto-fill,[seam] 24px [repeat-end]) [repeat-end] repeat(2,[join] 10px [tail]) [outer-end];column-gap:4px;row-gap:4px;margin:0;padding:0;border:0",
        &[
            ("row-seam", &item_style("1 / 2", "seam 5 / seam 6")),
            ("row-tail", &item_style("1 / 2", "tail -2 / tail -1")),
        ],
    );
    let output = fixture.compute();

    assert_eq!(
        frame(&fixture, &output, "row-seam"),
        (0.0, 96.0, 40.0, 24.0)
    );
    assert_eq!(
        frame(&fixture, &output, "row-tail"),
        (0.0, 278.0, 40.0, 10.0)
    );
    let grid = output
        .computed_styles
        .elements
        .iter()
        .find(|element| element.node_id == fixture.root.id())
        .and_then(|element| element.grid_layout.as_ref())
        .expect("root computed grid layout");
    assert_eq!(grid.rows_line_names.len(), 12);
    assert_eq!(grid.columns_line_names.len(), 2);
}
