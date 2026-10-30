use super::*;

fn run_grid_scene(
    children: &[Child<'_>],
    width: f32,
    white_space: crate::WhiteSpaceMode,
    columns: Vec<GridTrackSize>,
    rows: Vec<GridTrackSize>,
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
        rows,
    );
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &ahem_engine())
        .unwrap();
    (document, root, child_handles, output)
}

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

#[test]
fn anonymous_item_wraps_at_min_content_width() {
    let (document, root, children, styles) = fixture(&[Child::Text("XX XX")], 20.0);
    let snapshot = document.snapshot();
    let input = input(
        &snapshot,
        root,
        &styles,
        paragraph_style(),
        vec![GridTrackSize::MinContent, GridTrackSize::Fraction(1.0)],
        vec![GridTrackSize::Auto],
    );
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &ahem_engine())
        .unwrap();
    let item = &output.grid_item_intrinsic_inputs[0];
    let LayoutGridItemIntrinsicSource::Text { sizes } = &item.source else {
        panic!("anonymous Grid text must use text intrinsic measurements");
    };
    assert_eq!((sizes.min_content(), sizes.max_content()), (20.0, 50.0));
    assert_frame(&output.frames, children[0].id(), (0.0, 0.0, 20.0, 20.0));
    assert_eq!(output.text_layouts[&children[0].id()].layout.height, 20.0);
}

#[test]
fn anonymous_item_normal_min_content_control_keeps_two_lines_at_twenty_px() {
    let (document, root, children, mut styles) = fixture(&[Child::Text("XX XX")], 10.0);
    let root_style = styles.get_mut(&root.id()).unwrap();
    root_style.width = LayoutDimension::Fixed(20.0);
    root_style.height = LayoutDimension::Auto;
    let snapshot = document.snapshot();
    let input = input(
        &snapshot,
        root,
        &styles,
        paragraph_style(),
        vec![GridTrackSize::MinContent],
        vec![GridTrackSize::Auto],
    );
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &ahem_engine())
        .unwrap();
    let item = &output.grid_item_intrinsic_inputs[0];
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
        (20.0, 50.0, 20.0, 10.0)
    );
    assert_frame(&output.frames, root.id(), (0.0, 0.0, 20.0, 20.0));
    assert_frame(&output.frames, children[0].id(), (0.0, 0.0, 20.0, 20.0));
    assert_eq!(output.text_layouts[&children[0].id()].layout.height, 20.0);
}

#[test]
fn anonymous_item_nowrap_min_content_overflows_without_wrapping() {
    let (document, root, children, mut styles) = fixture(&[Child::Text("XX XX")], 10.0);
    let root_style = styles.get_mut(&root.id()).unwrap();
    root_style.width = LayoutDimension::Fixed(20.0);
    root_style.height = LayoutDimension::Auto;
    let snapshot = document.snapshot();
    let input = input(
        &snapshot,
        root,
        &styles,
        paragraph_style_with_white_space(crate::WhiteSpaceMode::NoWrap),
        vec![GridTrackSize::MinContent],
        vec![GridTrackSize::Auto],
    );
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &ahem_engine())
        .unwrap();
    let item = &output.grid_item_intrinsic_inputs[0];
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
        (50.0, 50.0, 10.0, 10.0)
    );
    assert_frame(&output.frames, root.id(), (0.0, 0.0, 20.0, 10.0));
    assert_frame(&output.frames, children[0].id(), (0.0, 0.0, 50.0, 10.0));
    assert_eq!(output.text_layouts[&children[0].id()].layout.height, 10.0);
    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == root.id())
        .expect("Grid sizing trace for the anonymous text container");
    assert!(trace.passes.iter().any(|pass| {
        pass.axis == crate::LayoutGridTrackSizingAxis::Columns
            && pass.contributions.iter().any(|contribution| {
                contribution.kind == crate::LayoutGridTrackSizingContributionKind::MinContent
                    && contribution.value == 50.0
            })
    }));
}

#[test]
fn anonymous_item_normal_segment_break_uses_collapsed_space_for_max_content() {
    let (document, root, children, mut styles) = fixture(&[Child::Text("XX\nXX")], 10.0);
    let root_style = styles.get_mut(&root.id()).unwrap();
    root_style.width = LayoutDimension::Fixed(20.0);
    root_style.height = LayoutDimension::Auto;
    let snapshot = document.snapshot();
    let input = input(
        &snapshot,
        root,
        &styles,
        paragraph_style(),
        vec![GridTrackSize::MaxContent],
        vec![GridTrackSize::Auto],
    );
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &ahem_engine())
        .unwrap();
    let item = &output.grid_item_intrinsic_inputs[0];
    assert_eq!(item.node, children[0].id());
    assert_eq!(item.source_order, 0);
    assert_eq!(
        item.text_sources
            .iter()
            .map(|source| (source.node, source.text.as_str()))
            .collect::<Vec<_>>(),
        vec![(children[0].id(), "XX\nXX")]
    );
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
        (20.0, 50.0, 20.0, 10.0)
    );
    assert_frame(&output.frames, root.id(), (0.0, 0.0, 20.0, 10.0));
    assert_frame(&output.frames, children[0].id(), (0.0, 0.0, 50.0, 10.0));
    assert_eq!(output.text_layouts[&children[0].id()].layout.height, 10.0);
    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == root.id())
        .expect("Grid sizing trace for the anonymous text container");
    let columns = trace
        .passes
        .iter()
        .find(|pass| pass.axis == crate::LayoutGridTrackSizingAxis::Columns)
        .expect("Columns sizing pass");
    assert_eq!(
        columns
            .stages
            .last()
            .expect("final Columns stage")
            .tracks
            .iter()
            .filter(|track| !track.is_gutter && !track.is_collapsed)
            .map(|track| track.base_size)
            .collect::<Vec<_>>(),
        [50.0]
    );
    assert!(columns.contributions.iter().any(|contribution| {
        contribution.item == children[0].id()
            && contribution.kind == crate::LayoutGridTrackSizingContributionKind::MaxContent
            && contribution.value == 50.0
    }));
    let rows = trace
        .passes
        .iter()
        .find(|pass| pass.axis == crate::LayoutGridTrackSizingAxis::Rows)
        .expect("Rows sizing pass");
    assert_eq!(
        rows.stages
            .last()
            .expect("final Rows stage")
            .tracks
            .iter()
            .filter(|track| !track.is_gutter && !track.is_collapsed)
            .map(|track| track.base_size)
            .collect::<Vec<_>>(),
        [10.0]
    );
    assert!(rows.contributions.iter().any(|contribution| {
        contribution.item == children[0].id() && contribution.value == 10.0
    }));
}

#[test]
fn anonymous_item_pre_line_preserves_segment_break_for_max_content() {
    let (document, root, children, mut styles) = fixture(&[Child::Text("XX\nXX")], 10.0);
    let root_style = styles.get_mut(&root.id()).unwrap();
    root_style.width = LayoutDimension::Fixed(20.0);
    root_style.height = LayoutDimension::Auto;
    let snapshot = document.snapshot();
    let input = input(
        &snapshot,
        root,
        &styles,
        paragraph_style_with_white_space(crate::WhiteSpaceMode::PreLine),
        vec![GridTrackSize::MaxContent],
        vec![GridTrackSize::Auto],
    );
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &ahem_engine())
        .unwrap();
    let item = &output.grid_item_intrinsic_inputs[0];
    assert_eq!(item.node, children[0].id());
    assert_eq!(item.source_order, 0);
    assert_eq!(
        item.text_sources
            .iter()
            .map(|source| (source.node, source.text.as_str()))
            .collect::<Vec<_>>(),
        vec![(children[0].id(), "XX\nXX")]
    );
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
        (20.0, 20.0, 20.0, 20.0)
    );
    assert_frame(&output.frames, root.id(), (0.0, 0.0, 20.0, 20.0));
    assert_frame(&output.frames, children[0].id(), (0.0, 0.0, 20.0, 20.0));
    assert_eq!(output.text_layouts[&children[0].id()].layout.height, 20.0);
    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == root.id())
        .expect("Grid sizing trace for the anonymous text container");
    let columns = trace
        .passes
        .iter()
        .find(|pass| pass.axis == crate::LayoutGridTrackSizingAxis::Columns)
        .expect("Columns sizing pass");
    assert_eq!(
        columns
            .stages
            .last()
            .expect("final Columns stage")
            .tracks
            .iter()
            .filter(|track| !track.is_gutter && !track.is_collapsed)
            .map(|track| track.base_size)
            .collect::<Vec<_>>(),
        [20.0]
    );
    assert!(columns.contributions.iter().any(|contribution| {
        contribution.item == children[0].id()
            && contribution.kind == crate::LayoutGridTrackSizingContributionKind::MaxContent
            && contribution.value == 20.0
    }));
    let rows = trace
        .passes
        .iter()
        .find(|pass| pass.axis == crate::LayoutGridTrackSizingAxis::Rows)
        .expect("Rows sizing pass");
    assert_eq!(
        rows.stages
            .last()
            .expect("final Rows stage")
            .tracks
            .iter()
            .filter(|track| !track.is_gutter && !track.is_collapsed)
            .map(|track| track.base_size)
            .collect::<Vec<_>>(),
        [20.0]
    );
    assert!(rows.contributions.iter().any(|contribution| {
        contribution.item == children[0].id() && contribution.value == 20.0
    }));
}

#[test]
fn anonymous_item_normal_repeated_spaces_are_the_max_content_control() {
    let (_document, root, children, output) = run_grid_scene(
        &[Child::Text("XX  XX")],
        20.0,
        crate::WhiteSpaceMode::Normal,
        vec![GridTrackSize::MaxContent],
        vec![GridTrackSize::Auto],
    );
    let item = &output.grid_item_intrinsic_inputs[0];
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
        (20.0, 50.0, 20.0, 10.0)
    );
    assert_frame(&output.frames, root.id(), (0.0, 0.0, 20.0, 10.0));
    assert_frame(&output.frames, children[0].id(), (0.0, 0.0, 50.0, 10.0));
    assert_grid_tracks(&output, root, &[50.0], &[10.0], [true, true]);
}

#[test]
fn anonymous_item_pre_wrap_preserves_repeated_spaces_for_max_content() {
    let (_document, root, children, output) = run_grid_scene(
        &[Child::Text("XX  XX")],
        20.0,
        crate::WhiteSpaceMode::PreWrap,
        vec![GridTrackSize::MaxContent],
        vec![GridTrackSize::Auto],
    );
    let item = &output.grid_item_intrinsic_inputs[0];
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
        (20.0, 60.0, 20.0, 10.0)
    );
    assert_frame(&output.frames, root.id(), (0.0, 0.0, 20.0, 10.0));
    assert_frame(&output.frames, children[0].id(), (0.0, 0.0, 60.0, 10.0));
    assert_grid_tracks(&output, root, &[60.0], &[10.0], [true, true]);
}

#[test]
fn anonymous_item_pre_wrap_comment_split_keeps_one_item_and_both_sources() {
    let (_document, root, children, output) = run_grid_scene(
        &[Child::Text("XX "), Child::Text(" XX")],
        20.0,
        crate::WhiteSpaceMode::PreWrap,
        vec![GridTrackSize::MaxContent],
        vec![GridTrackSize::Auto],
    );
    assert_eq!(output.grid_item_intrinsic_inputs.len(), 1);
    let item = &output.grid_item_intrinsic_inputs[0];
    assert_eq!(item.node, children[0].id());
    assert_eq!(item.source_order, 0);
    assert_eq!(
        item.text_sources
            .iter()
            .map(|source| (source.node, source.text.as_str()))
            .collect::<Vec<_>>(),
        vec![(children[0].id(), "XX "), (children[1].id(), " XX")]
    );
    let LayoutGridItemIntrinsicSource::Text { sizes } = &item.source else {
        panic!("anonymous Grid text must use text intrinsic measurements");
    };
    assert_eq!((sizes.min_content(), sizes.max_content()), (20.0, 60.0));
    assert_frame(&output.frames, children[0].id(), (0.0, 0.0, 60.0, 10.0));
    assert_grid_tracks(&output, root, &[60.0], &[10.0], [true, true]);
}

#[test]
fn anonymous_item_pre_wrap_min_content_keeps_two_lines() {
    let (_document, root, children, output) = run_grid_scene(
        &[Child::Text("XX  XX")],
        20.0,
        crate::WhiteSpaceMode::PreWrap,
        vec![GridTrackSize::MinContent],
        vec![GridTrackSize::Auto],
    );
    let item = &output.grid_item_intrinsic_inputs[0];
    let LayoutGridItemIntrinsicSource::Text { sizes } = &item.source else {
        panic!("anonymous Grid text must use text intrinsic measurements");
    };
    assert_eq!((sizes.min_content(), sizes.max_content()), (20.0, 60.0));
    assert_frame(&output.frames, root.id(), (0.0, 0.0, 20.0, 20.0));
    assert_frame(&output.frames, children[0].id(), (0.0, 0.0, 20.0, 20.0));
    assert_eq!(output.text_layouts[&children[0].id()].layout.height, 20.0);
    assert_grid_tracks(&output, root, &[20.0], &[20.0], [true, true]);
}

#[test]
fn anonymous_item_pre_wrap_soft_wrap_excludes_the_hanging_separator_from_track_contributions() {
    let (_document, root, children, output) = run_grid_scene(
        &[Child::Text("XX XX")],
        25.0,
        crate::WhiteSpaceMode::PreWrap,
        vec![GridTrackSize::LengthPx(25.0)],
        vec![GridTrackSize::Auto],
    );
    let item = &output.grid_item_intrinsic_inputs[0];
    let LayoutGridItemIntrinsicSource::Text { sizes } = &item.source else {
        panic!("anonymous Grid text must use text intrinsic measurements");
    };
    assert_eq!((sizes.min_content(), sizes.max_content()), (20.0, 50.0));
    assert_frame(&output.frames, root.id(), (0.0, 0.0, 25.0, 20.0));
    assert_frame(&output.frames, children[0].id(), (0.0, 0.0, 25.0, 20.0));
    let layout = &output.text_layouts[&children[0].id()].layout;
    assert_eq!(layout.lines.len(), 2);
    assert!((layout.lines[0].advance - 20.0).abs() <= 1.0 / 64.0);
    assert!((layout.lines[0].hanging_advance - 10.0).abs() <= 1.0 / 64.0);
    assert_grid_tracks(&output, root, &[25.0], &[20.0], [false, true]);
}

#[test]
fn anonymous_item_pre_wrap_forced_break_creates_two_max_content_rows() {
    let (_document, root, children, output) = run_grid_scene(
        &[Child::Text("XX\nXX")],
        20.0,
        crate::WhiteSpaceMode::PreWrap,
        vec![GridTrackSize::MaxContent],
        vec![GridTrackSize::Auto],
    );
    let item = &output.grid_item_intrinsic_inputs[0];
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
        (20.0, 20.0, 20.0, 20.0)
    );
    assert_frame(&output.frames, root.id(), (0.0, 0.0, 20.0, 20.0));
    assert_frame(&output.frames, children[0].id(), (0.0, 0.0, 20.0, 20.0));
    assert_grid_tracks(&output, root, &[20.0], &[20.0], [true, true]);
}

#[test]
fn pre_wrap_whitespace_before_block_does_not_consume_an_auto_placement_cell() {
    let (_document, root, children, output) = run_grid_scene(
        &[
            Child::Text("  "),
            Child::Block {
                id: "whitespace-before-block-item",
                width: 10.0,
            },
        ],
        40.0,
        crate::WhiteSpaceMode::PreWrap,
        vec![GridTrackSize::LengthPx(20.0), GridTrackSize::LengthPx(20.0)],
        vec![GridTrackSize::LengthPx(10.0)],
    );
    assert!(output.grid_item_intrinsic_inputs.is_empty());
    assert_frame(&output.frames, root.id(), (0.0, 0.0, 40.0, 10.0));
    assert_frame(&output.frames, children[1].id(), (0.0, 0.0, 10.0, 10.0));
    assert!(!output.text_layouts.contains_key(&children[0].id()));
    assert_grid_tracks(&output, root, &[20.0, 20.0], &[10.0], [false, false]);
}
