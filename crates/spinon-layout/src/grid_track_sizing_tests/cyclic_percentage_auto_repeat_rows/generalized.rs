use super::*;
use crate::LayoutDisplay;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
use crate::{LayoutGridTrackSizingContributionKind, LayoutGridTrackSizingFixturePhase};

fn generalized_row_input(height: LayoutDimension, mode: GridAutoRepeatMode) -> LayoutInput {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(24.0)],
        vec![
            GridTrackSize::LengthPx(8.0),
            GridTrackSize::MinMax {
                min: GridTrackMinSizingFunction::LengthPx(0.0),
                max: GridTrackMaxSizingFunction::Percentage(0.25),
            },
            GridTrackSize::LengthPx(8.0),
        ],
        3,
        LayoutGap {
            row: LayoutLengthPercentage::LengthPx(2.0),
            column: LayoutLengthPercentage::ZERO,
        },
    );
    input.root_sizing = RootSizingPolicy::ResolveWithin;
    input.viewport = Viewport {
        width: 24.0,
        height: 80.0,
    };
    input.nodes[0].style = LayoutStyle {
        display: LayoutDisplay::Grid,
        box_sizing: crate::LayoutBoxSizing::ContentBox,
        width: LayoutDimension::Fixed(24.0),
        height,
        min_height: LayoutDimension::Auto,
        flex_direction: crate::FlexDirection::Row,
        flex_shrink: 1.0,
        align_content: Some(LayoutAlignContent::Position {
            position: ContentAlignmentPosition::Start,
            safety: None,
        }),
        justify_content: LayoutJustifyContent::Position {
            position: JustifyContentPosition::Start,
            safety: None,
        },
        gap: LayoutGap {
            row: LayoutLengthPercentage::LengthPx(2.0),
            column: LayoutLengthPercentage::ZERO,
        },
        ..LayoutStyle::default()
    };
    let item_style = |width, height| LayoutStyle {
        display: LayoutDisplay::Block,
        box_sizing: crate::LayoutBoxSizing::BorderBox,
        width: LayoutDimension::Fixed(width),
        height: LayoutDimension::Fixed(height),
        min_width: LayoutDimension::Fixed(0.0),
        min_height: LayoutDimension::Fixed(0.0),
        flex_direction: crate::FlexDirection::Row,
        flex_shrink: 1.0,
        align_content: Some(LayoutAlignContent::Normal),
        ..LayoutStyle::default()
    };
    input.nodes[1].style = item_style(24.0, 8.0);
    input.nodes[2].style = item_style(24.0, 24.0);
    input.nodes[3].style = item_style(16.0, 16.0);

    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.rows_auto_repeat = Some(GridAutoRepeat {
        mode,
        start: 1,
        end: 2,
        line_names: Vec::new(),
    });
    for (item, row) in [(id(2), 1), (id(3), 2), (id(4), 2)] {
        input.grid_items.insert(
            item,
            GridItemInput {
                row_start: GridPlacementInput::Line(row),
                row_end: GridPlacementInput::Auto,
                column_start: GridPlacementInput::Line(1),
                column_end: GridPlacementInput::Auto,
            },
        );
    }
    input.positioning = [id(1), id(2), id(3), id(4)]
        .into_iter()
        .map(|node| (node, LayoutPositioning::default()))
        .collect();
    input
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn row_passes(output: &crate::LayoutOutput) -> Vec<&crate::LayoutGridTrackSizingPassTrace> {
    output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == id(1))
        .expect("root Grid trace")
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .collect()
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn assert_track_states(
    pass: &crate::LayoutGridTrackSizingPassTrace,
    expected: &[(bool, bool, f32)],
    scene: &str,
) {
    let tracks = &pass.stages.last().expect("final pass stage").tracks;
    assert_eq!(tracks.len(), expected.len(), "{scene} track-state count");
    for (index, (actual, (is_gutter, is_collapsed, base_size))) in
        tracks.iter().zip(expected).enumerate()
    {
        assert_eq!(actual.internal_index, index as u16, "{scene} index {index}");
        assert_eq!(actual.is_gutter, *is_gutter, "{scene} index {index} gutter");
        assert_eq!(
            actual.is_collapsed, *is_collapsed,
            "{scene} index {index} collapsed"
        );
        assert_eq!(actual.base_size, *base_size, "{scene} index {index} size");
    }
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn four_scene_matrix_preserves_rows_gutters_and_frames() {
    let scenes = [
        (
            "max-content auto-fill",
            LayoutDimension::MaxContent,
            GridAutoRepeatMode::AutoFill,
            1,
            Some(44.0),
            &[
                (true, true, 0.0),
                (false, false, 8.0),
                (true, false, 2.0),
                (false, false, 11.0),
                (true, false, 2.0),
                (false, false, 8.0),
                (true, true, 0.0),
            ][..],
            23.0,
        ),
        (
            "max-content auto-fit",
            LayoutDimension::MaxContent,
            GridAutoRepeatMode::AutoFit,
            1,
            Some(44.0),
            &[
                (true, true, 0.0),
                (false, false, 8.0),
                (true, false, 2.0),
                (false, false, 11.0),
                (true, false, 2.0),
                (false, false, 8.0),
                (true, true, 0.0),
            ][..],
            23.0,
        ),
        (
            "definite auto-fill",
            LayoutDimension::Fixed(44.0),
            GridAutoRepeatMode::AutoFill,
            2,
            Some(44.0),
            &[
                (true, true, 0.0),
                (false, false, 8.0),
                (true, false, 2.0),
                (false, false, 11.0),
                (true, false, 2.0),
                (false, false, 11.0),
                (true, false, 2.0),
                (false, false, 8.0),
                (true, true, 0.0),
            ][..],
            36.0,
        ),
        (
            "definite auto-fit",
            LayoutDimension::Fixed(44.0),
            GridAutoRepeatMode::AutoFit,
            2,
            Some(44.0),
            &[
                (true, true, 0.0),
                (false, false, 8.0),
                (true, false, 2.0),
                (false, false, 11.0),
                (true, false, 2.0),
                (false, true, 0.0),
                (true, true, 0.0),
                (false, false, 8.0),
                (true, true, 0.0),
            ][..],
            23.0,
        ),
    ];

    for (scene, height, mode, repeat_count, basis, expected_tracks, suffix_y) in scenes {
        let output = TaffyLayoutEngine
            .compute(&generalized_row_input(height, mode))
            .unwrap_or_else(|error| panic!("{scene} failed: {error:?}"));
        let root = &output.frames[&id(1)];
        assert_eq!(
            (root.x, root.y, root.width, root.height),
            (0.0, 0.0, 24.0, 44.0),
            "{scene} root"
        );
        assert_eq!(
            (
                output.frames[&id(2)].x,
                output.frames[&id(2)].y,
                output.frames[&id(2)].width,
                output.frames[&id(2)].height
            ),
            (0.0, 0.0, 24.0, 8.0),
            "{scene} prefix"
        );
        assert_eq!(
            (
                output.frames[&id(3)].x,
                output.frames[&id(3)].y,
                output.frames[&id(3)].width,
                output.frames[&id(3)].height
            ),
            (0.0, 10.0, 24.0, 24.0),
            "{scene} primary"
        );
        assert_eq!(
            (
                output.frames[&id(4)].x,
                output.frames[&id(4)].y,
                output.frames[&id(4)].width,
                output.frames[&id(4)].height
            ),
            (0.0, 10.0, 16.0, 16.0),
            "{scene} secondary"
        );

        let passes = row_passes(&output);
        if height == LayoutDimension::MaxContent {
            assert_eq!(passes.len(), 2, "{scene} pass count");
            assert_eq!(
                passes[0].fixture_phase,
                Some(LayoutGridTrackSizingFixturePhase::IntrinsicMaxContent),
                "{scene} intrinsic phase"
            );
            assert_eq!(
                passes[0].fixture_auto_repeat_count,
                Some(1),
                "{scene} initial count"
            );
            assert_eq!(
                passes[0].fixture_percentage_basis, None,
                "{scene} intrinsic basis"
            );
            let contributions = &passes[0].contributions;
            assert_eq!(
                contributions.len(),
                2,
                "{scene} intrinsic contribution count"
            );
            for (item, expected) in [(id(3), 24.0), (id(4), 16.0)] {
                let contribution = contributions
                    .iter()
                    .find(|entry| entry.item == item)
                    .unwrap_or_else(|| panic!("{scene} missing item {item:?} contribution"));
                assert_eq!(
                    contribution.kind,
                    LayoutGridTrackSizingContributionKind::MaxContent
                );
                assert_eq!(contribution.span, 1);
                assert_eq!(contribution.value, expected);
            }
            assert_track_states(
                passes[0],
                &[
                    (true, true, 0.0),
                    (false, false, 8.0),
                    (true, false, 2.0),
                    (false, false, 24.0),
                    (true, false, 2.0),
                    (false, false, 8.0),
                    (true, true, 0.0),
                ],
                scene,
            );
            assert_eq!(
                passes[1].fixture_phase,
                Some(LayoutGridTrackSizingFixturePhase::FinalDefinite),
                "{scene} final phase"
            );
            assert_eq!(
                passes[1].fixture_auto_repeat_count,
                Some(1),
                "{scene} final count"
            );
            assert_eq!(
                passes[1].fixture_percentage_basis,
                Some(44.0),
                "{scene} final basis"
            );
        } else {
            assert_eq!(passes.len(), 1, "{scene} pass count");
            assert_eq!(
                passes[0].fixture_phase,
                Some(LayoutGridTrackSizingFixturePhase::FinalDefinite),
                "{scene} final phase"
            );
        }

        let final_pass = *passes.last().unwrap();
        assert_eq!(
            final_pass.fixture_auto_repeat_count,
            Some(repeat_count),
            "{scene} count"
        );
        assert_eq!(final_pass.fixture_percentage_basis, basis, "{scene} basis");
        assert_track_states(final_pass, expected_tracks, scene);
        let final_tracks = &final_pass.stages.last().unwrap().tracks;
        let suffix_index = final_tracks.len() - 2;
        let suffix_start = final_tracks[..suffix_index]
            .iter()
            .map(|track| track.base_size)
            .sum::<f32>();
        assert_eq!(suffix_start, suffix_y, "{scene} suffix start");
    }
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn generalized_row_profile_rejects_unsupported_mutations_at_the_expected_axis() {
    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input.nodes[0].style.width = LayoutDimension::Fixed(23.0);
    assert_invalid_target(input, "generalized-root-width");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input.grid_containers.get_mut(&id(1)).unwrap().rows[0] = GridTrackSize::LengthPx(7.0);
    assert_invalid_target(input, "generalized-prefix-track");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input.grid_containers.get_mut(&id(1)).unwrap().rows[2] = GridTrackSize::LengthPx(9.0);
    assert_invalid_target(input, "generalized-suffix-track");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .rows_auto_repeat
        .as_mut()
        .unwrap()
        .start = 0;
    assert_invalid_target(input, "generalized-repeat-range");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input.nodes[0].style.gap.row = LayoutLengthPercentage::LengthPx(1.0);
    assert_invalid_target(input, "generalized-row-gap");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input.nodes[0].style.gap.column = LayoutLengthPercentage::LengthPx(1.0);
    assert_invalid_target(input, "generalized-column-gap");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input.nodes[0].style.min_height = LayoutDimension::Fixed(0.0);
    assert_invalid_target(input, "generalized-root-min-height-zero");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input.root_sizing = crate::RootSizingPolicy::Match;
    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::RootSizeMismatch { axis: "height" })
    ));

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input.root_sizing = crate::RootSizingPolicy::BlockFormatting;
    assert_invalid_target(input, "generalized-root-sizing-policy-block-formatting");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .rows_line_names = vec![Vec::new(), vec!["named-row".into()], Vec::new(), Vec::new()];
    assert_invalid_target(input, "generalized-row-line-name");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .rows_auto_repeat
        .as_mut()
        .unwrap()
        .line_names = vec![vec!["repeat-start".into()], vec!["repeat-end".into()]];
    assert_invalid_target(input, "generalized-repeat-line-name-metadata");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .template_areas = Some(crate::GridTemplateAreasInput {
        rows: 3,
        columns: 1,
        areas: vec![crate::GridTemplateAreaInput {
            name: "content".into(),
            row_start: 1,
            row_end: 2,
            column_start: 0,
            column_end: 1,
        }],
    });
    assert_invalid_target(input, "generalized-template-areas");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input.nodes[0].style.padding.top = LayoutLengthPercentage::LengthPx(1.0);
    assert_invalid_target(input, "generalized-root-padding");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input.nodes[0].style.border.top = 1.0;
    assert_invalid_target(input, "generalized-root-border");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input.nodes[0].style.align_content = Some(LayoutAlignContent::Normal);
    assert_invalid_target(input, "generalized-root-align-content");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input.nodes[0].style.justify_content = LayoutJustifyContent::Position {
        position: JustifyContentPosition::End,
        safety: None,
    };
    assert_invalid_target(input, "generalized-root-justify-content");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input.nodes[2].style.width = LayoutDimension::Fixed(23.0);
    assert_invalid_target(input, "generalized-primary-width");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input.nodes[2].style.height = LayoutDimension::Fixed(23.0);
    assert_invalid_target(input, "generalized-primary-height");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input.nodes[2].style.min_width = LayoutDimension::Fixed(1.0);
    assert_invalid_target(input, "generalized-primary-min-width");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input.nodes[2].style.min_height = LayoutDimension::Fixed(1.0);
    assert_invalid_target(input, "generalized-primary-min-height");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input.positioning.get_mut(&id(2)).unwrap().inset.top = LayoutLengthPercentage::LengthPx(1.0);
    assert_invalid_target(input, "generalized-static-inset");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input.grid_items.get_mut(&id(3)).unwrap().row_start = GridPlacementInput::Line(3);
    assert_invalid_target(input, "generalized-placement");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input.nodes[0].children.pop();
    input.nodes.pop();
    input.grid_items.remove(&id(4));
    input.positioning.remove(&id(4));
    assert_invalid_target(input, "generalized-missing-child");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .columns_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFill,
        start: 0,
        end: 1,
        line_names: Vec::new(),
    });
    input.grid_containers.get_mut(&id(1)).unwrap().columns = vec![GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::LengthPx(0.0),
        max: GridTrackMaxSizingFunction::Percentage(0.25),
    }];
    assert_invalid_target(input, "generalized-other-axis-auto-repeat");

    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input.nodes[0].style.width = LayoutDimension::MaxContent;
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .columns_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFill,
        start: 0,
        end: 1,
        line_names: Vec::new(),
    });
    input.grid_containers.get_mut(&id(1)).unwrap().columns = vec![GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::LengthPx(0.0),
        max: GridTrackMaxSizingFunction::Percentage(0.25),
    }];
    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::InvalidGridTrackList { node, property: "grid-template-columns" })
            if node == id(1)
    ));
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn generalized_row_rejects_unknown_positioning_key_before_grid_profile_validation() {
    let mut input =
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill);
    input.positioning.remove(&id(4));
    input
        .positioning
        .insert(id(9), LayoutPositioning::default());

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::UnknownPositioningNode(node)) if node == id(9)
    ));
}

#[cfg(not(feature = "c11-4-grid-track-sizing-fixture-test"))]
#[test]
fn generalized_cyclic_rows_remain_rejected_without_fixture_feature() {
    assert_invalid_target(
        generalized_row_input(LayoutDimension::MaxContent, GridAutoRepeatMode::AutoFill),
        "generalized fixture feature disabled",
    );
}
