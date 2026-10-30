use super::*;

fn assert_grid_tracks(
    output: &crate::LayoutOutput,
    root: HostNodeHandle,
    expected_columns: &[f32],
    expected_rows: &[f32],
    expect_contributions: [bool; 2],
) {
    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == root.id())
        .expect("Grid track sizing trace");
    for ((axis, expected), expect_contributions) in [
        (crate::LayoutGridTrackSizingAxis::Columns, expected_columns),
        (crate::LayoutGridTrackSizingAxis::Rows, expected_rows),
    ]
    .into_iter()
    .zip(expect_contributions)
    {
        let pass = trace
            .passes
            .iter()
            .find(|pass| pass.axis == axis)
            .expect("axis sizing pass");
        assert_eq!(
            pass.stages
                .last()
                .expect("final sizing stage")
                .tracks
                .iter()
                .filter(|track| !track.is_gutter && !track.is_collapsed)
                .map(|track| track.base_size)
                .collect::<Vec<_>>(),
            expected
        );
        assert_eq!(
            !pass.contributions.is_empty(),
            expect_contributions,
            "{axis:?} contributions"
        );
    }
}

fn run_grid_scene(
    children: &[Child<'_>],
    width: f32,
    white_space: crate::WhiteSpaceMode,
    columns: Vec<GridTrackSize>,
) -> (
    HostDocument,
    HostNodeHandle,
    Vec<HostNodeHandle>,
    crate::LayoutOutput,
) {
    let (document, root, child_handles, mut styles) = fixture(children, 10.0);
    let root_style = styles.get_mut(&root.id()).unwrap();
    root_style.width = LayoutDimension::Fixed(width);
    root_style.height = LayoutDimension::Auto;
    let snapshot = document.snapshot();
    let input = input(
        &snapshot,
        root,
        &styles,
        paragraph_style_with_white_space(white_space),
        columns,
        if children
            .iter()
            .any(|child| matches!(child, Child::Block { .. }))
        {
            vec![GridTrackSize::LengthPx(10.0)]
        } else {
            vec![GridTrackSize::Auto]
        },
    );
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &ahem_engine())
        .unwrap();
    (document, root, child_handles, output)
}

fn assert_sizes(
    output: &crate::LayoutOutput,
    node: spinon_core::NodeId,
    expected: (f32, f32, f32, f32),
) {
    let item = output
        .grid_item_intrinsic_inputs
        .iter()
        .find(|item| item.node == node)
        .expect("anonymous text item intrinsic input");
    let LayoutGridItemIntrinsicSource::Text { sizes } = &item.source else {
        panic!("anonymous Grid text must use text intrinsic measurements");
    };
    assert_eq!(
        (
            sizes.min_content(),
            sizes.max_content(),
            sizes.min_content_height(),
            sizes.max_content_height(),
        ),
        expected
    );
}

#[test]
fn anonymous_break_spaces_max_and_min_content_use_non_hanging_spaces() {
    let (_document, root, children, max_content) = run_grid_scene(
        &[Child::Text("XX  XX")],
        60.0,
        crate::WhiteSpaceMode::BreakSpaces,
        vec![GridTrackSize::MaxContent],
    );
    assert_sizes(&max_content, children[0].id(), (30.0, 60.0, 20.0, 10.0));
    assert_frame(&max_content.frames, root.id(), (0.0, 0.0, 60.0, 10.0));
    assert_frame(
        &max_content.frames,
        children[0].id(),
        (0.0, 0.0, 60.0, 10.0),
    );
    assert_eq!(
        max_content.text_layouts[&children[0].id()].layout.text,
        "XX  XX"
    );
    assert_grid_tracks(&max_content, root, &[60.0], &[10.0], [true, true]);

    let (_document, root, children, min_content) = run_grid_scene(
        &[Child::Text("XX  XX")],
        30.0,
        crate::WhiteSpaceMode::BreakSpaces,
        vec![GridTrackSize::MinContent],
    );
    assert_sizes(&min_content, children[0].id(), (30.0, 60.0, 20.0, 10.0));
    assert_frame(&min_content.frames, root.id(), (0.0, 0.0, 30.0, 20.0));
    assert_frame(
        &min_content.frames,
        children[0].id(),
        (0.0, 0.0, 30.0, 20.0),
    );
    let layout = &min_content.text_layouts[&children[0].id()].layout;
    assert_eq!(layout.lines.len(), 2);
    assert_eq!(layout.height, 20.0);
    assert_grid_tracks(&min_content, root, &[30.0], &[20.0], [true, true]);
}

#[test]
fn anonymous_break_spaces_comment_split_remains_one_item_with_source_identity() {
    let (_document, root, children, output) = run_grid_scene(
        &[Child::Text("XX "), Child::Text(" XX")],
        60.0,
        crate::WhiteSpaceMode::BreakSpaces,
        vec![GridTrackSize::MaxContent],
    );
    assert_eq!(output.grid_item_intrinsic_inputs.len(), 1);
    assert_sizes(&output, children[0].id(), (30.0, 60.0, 20.0, 10.0));
    let item = &output.grid_item_intrinsic_inputs[0];
    assert_eq!(
        item.text_sources
            .iter()
            .map(|source| (source.node, source.text.as_str()))
            .collect::<Vec<_>>(),
        vec![(children[0].id(), "XX "), (children[1].id(), " XX")]
    );
    let layout = &output.text_layouts[&children[0].id()].layout;
    assert_eq!(layout.text, "XX  XX");
    assert_eq!(layout.sources[2].node, children[0].id());
    assert_eq!(layout.sources[3].node, children[1].id());
    assert_frame(&output.frames, root.id(), (0.0, 0.0, 60.0, 10.0));
    assert_grid_tracks(&output, root, &[60.0], &[10.0], [true, true]);
}

#[test]
fn anonymous_break_spaces_fixed_25px_track_keeps_non_hanging_wraps() {
    let (_document, root, children, output) = run_grid_scene(
        &[Child::Text("XX  XX")],
        25.0,
        crate::WhiteSpaceMode::BreakSpaces,
        vec![GridTrackSize::LengthPx(25.0)],
    );
    assert_sizes(&output, children[0].id(), (30.0, 60.0, 20.0, 10.0));
    assert_frame(&output.frames, root.id(), (0.0, 0.0, 25.0, 20.0));
    let layout = &output.text_layouts[&children[0].id()].layout;
    assert_eq!(layout.lines.len(), 2);
    assert_eq!(layout.height, 20.0);
    assert_eq!(layout.lines[0].advance, 30.0);
    assert_eq!(layout.lines[0].hanging_advance, 0.0);
    assert_eq!(layout.lines[1].advance, 30.0);
    assert_eq!(layout.lines[1].hanging_advance, 0.0);
    assert_grid_tracks(&output, root, &[25.0], &[20.0], [false, true]);
}

#[test]
fn anonymous_break_spaces_forced_break_contributes_spaces_to_tracks() {
    let (_document, root, children, max_content) = run_grid_scene(
        &[Child::Text("XX  \nXX")],
        40.0,
        crate::WhiteSpaceMode::BreakSpaces,
        vec![GridTrackSize::MaxContent],
    );
    assert_sizes(&max_content, children[0].id(), (30.0, 40.0, 30.0, 20.0));
    assert_frame(&max_content.frames, root.id(), (0.0, 0.0, 40.0, 20.0));
    assert_grid_tracks(&max_content, root, &[40.0], &[20.0], [true, true]);

    let (_document, root, children, min_content) = run_grid_scene(
        &[Child::Text("XX  \nXX")],
        30.0,
        crate::WhiteSpaceMode::BreakSpaces,
        vec![GridTrackSize::MinContent],
    );
    assert_sizes(&min_content, children[0].id(), (30.0, 40.0, 30.0, 20.0));
    let layout = &min_content.text_layouts[&children[0].id()].layout;
    assert_eq!(layout.lines.len(), 3);
    assert_eq!(layout.height, 30.0);
    assert_frame(&min_content.frames, root.id(), (0.0, 0.0, 30.0, 30.0));
    assert_grid_tracks(&min_content, root, &[30.0], &[30.0], [true, true]);
}

#[test]
fn anonymous_break_spaces_trailing_spaces_wrap_and_size_without_hanging() {
    let (_document, root, children, wrapped) = run_grid_scene(
        &[Child::Text("XX  ")],
        25.0,
        crate::WhiteSpaceMode::BreakSpaces,
        vec![GridTrackSize::LengthPx(25.0)],
    );
    assert_frame(&wrapped.frames, root.id(), (0.0, 0.0, 25.0, 20.0));
    let layout = &wrapped.text_layouts[&children[0].id()].layout;
    assert_eq!(layout.lines.len(), 2);
    assert_eq!(layout.lines[0].advance, 30.0);
    assert_eq!(layout.lines[0].hanging_advance, 0.0);
    assert_eq!(layout.lines[1].advance, 10.0);
    assert_eq!(layout.lines[1].hanging_advance, 0.0);
    assert_grid_tracks(&wrapped, root, &[25.0], &[20.0], [false, true]);

    let (_document, root, children, max_content) = run_grid_scene(
        &[Child::Text("XX  ")],
        40.0,
        crate::WhiteSpaceMode::BreakSpaces,
        vec![GridTrackSize::MaxContent],
    );
    assert_sizes(&max_content, children[0].id(), (30.0, 40.0, 20.0, 10.0));
    assert_frame(&max_content.frames, root.id(), (0.0, 0.0, 40.0, 10.0));
    assert_grid_tracks(&max_content, root, &[40.0], &[10.0], [true, true]);
}

#[test]
fn break_spaces_whitespace_only_does_not_create_an_anonymous_grid_item() {
    let (_document, root, children, output) = run_grid_scene(
        &[Child::Text("  ")],
        0.0,
        crate::WhiteSpaceMode::BreakSpaces,
        vec![GridTrackSize::Auto],
    );
    assert!(output.grid_item_intrinsic_inputs.is_empty());
    assert!(!output.text_layouts.contains_key(&children[0].id()));
    assert_frame(&output.frames, root.id(), (0.0, 0.0, 0.0, 0.0));
    assert_grid_tracks(&output, root, &[0.0], &[0.0], [false, false]);
}

#[test]
fn break_spaces_whitespace_before_block_does_not_consume_an_auto_cell() {
    let (_document, root, children, output) = run_grid_scene(
        &[
            Child::Text("  "),
            Child::Block {
                id: "break-spaces-whitespace-before-block",
                width: 10.0,
            },
        ],
        40.0,
        crate::WhiteSpaceMode::BreakSpaces,
        vec![GridTrackSize::LengthPx(20.0), GridTrackSize::LengthPx(20.0)],
    );
    assert!(output.grid_item_intrinsic_inputs.is_empty());
    assert!(!output.text_layouts.contains_key(&children[0].id()));
    assert_frame(&output.frames, root.id(), (0.0, 0.0, 40.0, 10.0));
    assert_frame(&output.frames, children[1].id(), (0.0, 0.0, 10.0, 10.0));
    assert_grid_tracks(&output, root, &[20.0, 20.0], &[10.0], [false, false]);
}
