use super::*;

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
