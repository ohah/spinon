use super::*;

#[test]
fn visibility_hidden_border_overflow_preserves_grid_frames_and_hidden_contribution() {
    for (axis, visible_control) in [
        (Axis::Columns, false),
        (Axis::Columns, true),
        (Axis::Rows, false),
        (Axis::Rows, true),
    ] {
        let scene = make_fixture_with_border_overflow(axis, visible_control, true);
        let fixture = &scene.fixture;
        let styles = cascade(fixture);
        let root_style = element(&styles, fixture.root);
        assert_eq!(
            root_style.layout_overflow_x,
            Some(spinon_style::ComputedCssOverflow::Visible)
        );
        assert_eq!(
            root_style.layout_overflow_y,
            Some(spinon_style::ComputedCssOverflow::Visible)
        );
        assert_eq!(
            root_style.layout_border,
            spinon_style::ComputedLayoutBorder {
                top: 2.0,
                right: 2.0,
                bottom: 2.0,
                left: 2.0,
            }
        );

        let input = super::super::super::prepare(&fixture.snapshot, fixture.root, &styles)
            .unwrap_or_else(|error| panic!("border overflow scene rejected: {error:?}"))
            .expect("four exact border overflow scenes route through mixed profile");
        let output = spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_and_intrinsic_sizes(
            &fixture.snapshot,
            fixture.root,
            styles,
            VIEWPORT,
            &input.paragraph_styles,
            &input.engine,
            input.intrinsic_sizes,
        )
        .unwrap_or_else(|error| panic!("border overflow Grid layout failed: {error:?}"));

        let root = output.layout.frames[&fixture.root.id()];
        let a = output.layout.frames[&fixture.a.id()];
        let span = output.layout.frames[&scene.span.id()];
        let image = output.layout.frames[&fixture.b.id()];
        let c = output.layout.frames[&fixture.c.id()];
        match axis {
            Axis::Columns => {
                close(root.width, 100.0, "border-box root width");
                close(root.height, 120.0, "border-box root height");
                for frame in [a, span] {
                    close(frame.x, 2.0, "bordered first item x");
                    close(frame.y, 2.0, "bordered first item y");
                    close(frame.width, 150.0, "overflowing first item width");
                    close(frame.height, 116.0, "first item content-axis height");
                }
                close(image.x, 162.0, "bordered image x");
                close(image.y, 2.0, "bordered image y");
                close(c.x, 212.0, "zero-width third-track x");
                close(c.width, 0.0, "zero-width third-track item");
                close(c.height, 116.0, "third-track item content-axis height");
            }
            Axis::Rows => {
                close(root.width, 120.0, "border-box root width");
                close(root.height, 100.0, "border-box root height");
                for frame in [a, span] {
                    close(frame.x, 2.0, "bordered first item x");
                    close(frame.y, 2.0, "bordered first item y");
                    close(frame.width, 116.0, "first item content-axis width");
                    close(frame.height, 150.0, "overflowing first item height");
                }
                close(image.x, 2.0, "bordered image x");
                close(image.y, 162.0, "bordered image y");
                close(c.x, 2.0, "third-track item x");
                close(c.y, 212.0, "zero-height third-track y");
                close(c.width, 116.0, "third-track item content-axis width");
                close(c.height, 0.0, "zero-height third-track item");
            }
        }
        close(image.width, 24.0, "image natural width");
        close(image.height, 16.0, "image natural height");

        let trace = output
            .layout
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == fixture.root.id())
            .expect("border overflow track-sizing trace");
        let target_axis = if axis == Axis::Rows {
            LayoutGridTrackSizingAxis::Rows
        } else {
            LayoutGridTrackSizingAxis::Columns
        };
        let pass = trace
            .passes
            .iter()
            .find(|pass| pass.axis == target_axis)
            .expect("target-axis sizing pass");
        assert!(pass.contributions.iter().any(|item| {
            item.item == scene.span.id()
                && item.kind == spinon_layout::LayoutGridTrackSizingContributionKind::MinContent
                && (item.value - 150.0).abs() < 1.0e-4
        }));
        let used = pass
            .stages
            .iter()
            .find(|stage| stage.stage == LayoutGridTrackSizingStageKind::ExpandFlexibleTracks)
            .expect("flex expansion stage")
            .tracks
            .iter()
            .filter(|track| !track.is_gutter && !track.is_collapsed)
            .map(|track| track.base_size)
            .collect::<Vec<_>>();
        assert_eq!(used, [150.0, 40.0, 0.0]);
    }
}

#[test]
fn visibility_hidden_border_overflow_rejects_overflow_and_border_mutations() {
    let scene = make_fixture_with_border_overflow(Axis::Columns, false, true);
    let fixture = &scene.fixture;
    let styles = cascade(fixture);

    let mut clipped = styles.clone();
    element_mut(&mut clipped, fixture.root).layout_overflow_x =
        Some(spinon_style::ComputedCssOverflow::Hidden);
    assert_rejected(fixture, &clipped, "root overflow-x mutation");

    let mut clipped_y = styles.clone();
    element_mut(&mut clipped_y, fixture.root).layout_overflow_y =
        Some(spinon_style::ComputedCssOverflow::Hidden);
    assert_rejected(fixture, &clipped_y, "root overflow-y mutation");

    let mut changed_border = styles.clone();
    element_mut(&mut changed_border, fixture.root)
        .layout_border
        .right = 3.0;
    assert_rejected(fixture, &changed_border, "root border width mutation");

    let mut changed_border_paint = styles.clone();
    element_mut(&mut changed_border_paint, fixture.root).layout_border_paint = None;
    assert_rejected(fixture, &changed_border_paint, "root border paint mutation");

    let mut changed_tracks = styles.clone();
    element_mut(&mut changed_tracks, fixture.root)
        .grid_layout
        .as_mut()
        .unwrap()
        .rows[0] = spinon_style::ComputedCssGridTrackSize::LengthPx(120.0);
    assert_rejected(
        fixture,
        &changed_tracks,
        "border-adjusted cross-axis track mutation",
    );

    let mut legacy_geometry = styles.clone();
    let legacy_root = element_mut(&mut legacy_geometry, fixture.root);
    legacy_root.layout_dimensions.width = spinon_style::ComputedCssDimension::LengthPx(300.0);
    legacy_root.layout_border = Default::default();
    legacy_root.layout_border_paint = None;
    legacy_root.grid_layout.as_mut().unwrap().rows[0] =
        spinon_style::ComputedCssGridTrackSize::LengthPx(120.0);
    assert_rejected(
        fixture,
        &legacy_geometry,
        "border-overflow ID with legacy scene style",
    );

    let legacy_scene = make_fixture(Axis::Columns, false);
    let mut border_geometry = cascade(&legacy_scene.fixture);
    let legacy_root = element_mut(&mut border_geometry, legacy_scene.fixture.root);
    legacy_root.layout_dimensions.width = spinon_style::ComputedCssDimension::LengthPx(100.0);
    legacy_root.layout_border = super::super::super::root_contract::border_layout();
    legacy_root.layout_border_paint = Some(super::super::super::root_contract::border_paint());
    legacy_root.grid_layout.as_mut().unwrap().rows[0] =
        spinon_style::ComputedCssGridTrackSize::LengthPx(116.0);
    assert_rejected(
        &legacy_scene.fixture,
        &border_geometry,
        "legacy visibility ID with border-overflow scene style",
    );
}

fn element(
    styles: &spinon_style::ComputedStyleSnapshot,
    handle: HostNodeHandle,
) -> &spinon_style::ComputedElementStyle {
    styles
        .elements
        .iter()
        .find(|style| style.node_id == handle.id())
        .expect("fixture computed style")
}
