use super::{grid_input, id};
use crate::grid_placement::{GridItemInput, GridPlacementInput};
use crate::{
    ContentAlignmentPosition, GridAutoRepeat, GridAutoRepeatMode, GridPlacementMode,
    GridTrackMaxSizingFunction, GridTrackMinSizingFunction, GridTrackSize, JustifyContentPosition,
    LayoutAlignContent, LayoutBoxSizing, LayoutDimension, LayoutEdges, LayoutEngine, LayoutError,
    LayoutGap, LayoutGridTrackSizingAxis, LayoutInput, LayoutJustifyContent,
    LayoutLengthPercentage, LayoutPosition, LayoutPositioning, LayoutStyle, RootSizingPolicy,
    TaffyLayoutEngine, Viewport,
};

fn row_target_input(height: LayoutDimension) -> LayoutInput {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(24.0)],
        vec![GridTrackSize::MinMax {
            min: GridTrackMinSizingFunction::LengthPx(0.0),
            max: GridTrackMaxSizingFunction::Percentage(0.5),
        }],
        1,
        LayoutGap {
            row: LayoutLengthPercentage::ZERO,
            column: LayoutLengthPercentage::ZERO,
        },
    );
    input.root_sizing = RootSizingPolicy::ResolveWithin;
    input.viewport = Viewport {
        width: 24.0,
        height: 80.0,
    };
    input.nodes[0].style = LayoutStyle {
        display: crate::LayoutDisplay::Grid,
        box_sizing: LayoutBoxSizing::ContentBox,
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
        ..LayoutStyle::default()
    };
    input.nodes[1].style = LayoutStyle {
        display: crate::LayoutDisplay::Block,
        width: LayoutDimension::Fixed(24.0),
        height: LayoutDimension::Fixed(16.0),
        min_width: LayoutDimension::Fixed(0.0),
        min_height: LayoutDimension::Fixed(0.0),
        flex_direction: crate::FlexDirection::Row,
        flex_shrink: 1.0,
        align_content: Some(LayoutAlignContent::Normal),
        ..LayoutStyle::default()
    };

    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.rows_auto_repeat = Some(GridAutoRepeat {
        mode: GridAutoRepeatMode::AutoFill,
        start: 0,
        end: 1,
        line_names: Vec::new(),
    });
    input.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Auto,
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Auto,
        },
    );
    input.positioning = [id(1), id(2)]
        .into_iter()
        .map(|node| {
            (
                node,
                LayoutPositioning {
                    position: LayoutPosition::Static,
                    inset: LayoutEdges {
                        top: LayoutLengthPercentage::Auto,
                        right: LayoutLengthPercentage::Auto,
                        bottom: LayoutLengthPercentage::Auto,
                        left: LayoutLengthPercentage::Auto,
                    },
                },
            )
        })
        .collect();
    input
}

fn used_rows(pass: &crate::LayoutGridTrackSizingPassTrace) -> Vec<(bool, f32)> {
    pass.stages
        .last()
        .unwrap()
        .tracks
        .iter()
        .filter(|track| !track.is_gutter)
        .map(|track| (track.is_collapsed, track.base_size))
        .collect()
}

fn assert_rows(actual: &[(bool, f32)], expected: &[(bool, f32)], scene: &str) {
    assert_eq!(actual.len(), expected.len(), "{scene} track count");
    for (index, ((actual_collapsed, actual_size), (expected_collapsed, expected_size))) in
        actual.iter().zip(expected).enumerate()
    {
        assert_eq!(
            actual_collapsed, expected_collapsed,
            "{scene} track {index} collapsed"
        );
        assert!(
            (actual_size - expected_size).abs() <= 1.0 / 64.0,
            "{scene} track {index}: {actual_size} != {expected_size}"
        );
    }
}

fn assert_invalid_target(input: LayoutInput, mutation: &str) {
    assert!(
        matches!(
            TaffyLayoutEngine.compute(&input),
            Err(LayoutError::InvalidGridTrackList {
                node,
                property: "grid-template-rows",
            }) if node == id(1)
        ),
        "{mutation} must fail closed at root grid-template-rows"
    );
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn append_static_child(input: &mut LayoutInput) {
    let child = id(3);
    input.nodes[0].children.push(child);
    input.nodes.push(crate::LayoutNode {
        id: child,
        children: Vec::new(),
        style: LayoutStyle {
            display: crate::LayoutDisplay::Block,
            width: LayoutDimension::Fixed(1.0),
            height: LayoutDimension::Fixed(1.0),
            ..LayoutStyle::default()
        },
    });
    input
        .positioning
        .insert(child, LayoutPositioning::default());
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn cyclic_percentage_auto_repeat_rows_keeps_one_track_across_the_percentage_rerun() {
    let input = row_target_input(LayoutDimension::MaxContent);
    assert_eq!(input.nodes[0].style.min_height, LayoutDimension::Auto);
    let output = TaffyLayoutEngine.compute(&input).unwrap();
    let root = &output.frames[&id(1)];
    let item = &output.frames[&id(2)];
    assert_eq!(
        (root.x, root.y, root.width, root.height),
        (0.0, 0.0, 24.0, 16.0)
    );
    assert_eq!(
        (item.x, item.y, item.width, item.height),
        (0.0, 0.0, 24.0, 16.0)
    );

    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == id(1))
        .unwrap();
    let row_passes = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .collect::<Vec<_>>();
    assert_eq!(
        row_passes.len(),
        2,
        "row trace must record both sizing passes"
    );
    assert_eq!(
        row_passes[0].fixture_phase,
        Some(crate::LayoutGridTrackSizingFixturePhase::IntrinsicMaxContent),
        "intrinsic row sizing must precede final definite sizing"
    );
    assert_eq!(
        row_passes[1].fixture_phase,
        Some(crate::LayoutGridTrackSizingFixturePhase::FinalDefinite),
        "final definite row sizing must follow intrinsic sizing"
    );
    assert!(
        row_passes[0].sequence < row_passes[1].sequence,
        "row pass sequence must increase with intrinsic-to-final phase order"
    );
    let intrinsic = row_passes
        .iter()
        .copied()
        .find(|pass| {
            pass.fixture_phase
                == Some(crate::LayoutGridTrackSizingFixturePhase::IntrinsicMaxContent)
        })
        .unwrap();
    let final_definite = row_passes
        .iter()
        .copied()
        .find(|pass| {
            pass.fixture_phase == Some(crate::LayoutGridTrackSizingFixturePhase::FinalDefinite)
        })
        .unwrap();

    assert_eq!(intrinsic.fixture_auto_repeat_count, Some(1));
    assert_eq!(intrinsic.fixture_percentage_basis, None);
    assert_rows(&used_rows(intrinsic), &[(false, 16.0)], "intrinsic target");
    assert_eq!(final_definite.fixture_auto_repeat_count, Some(1));
    assert_eq!(final_definite.fixture_percentage_basis, Some(16.0));
    assert_rows(&used_rows(final_definite), &[(false, 8.0)], "final target");
    assert_eq!(item.y + item.height - (root.y + 8.0), 8.0);
}

#[test]
fn definite_40px_row_control_uses_the_general_auto_repeat_path() {
    let output = TaffyLayoutEngine
        .compute(&row_target_input(LayoutDimension::Fixed(40.0)))
        .unwrap();
    let root = &output.frames[&id(1)];
    let item = &output.frames[&id(2)];
    assert_eq!(
        (root.x, root.y, root.width, root.height),
        (0.0, 0.0, 24.0, 40.0)
    );
    assert_eq!(
        (item.x, item.y, item.width, item.height),
        (0.0, 0.0, 24.0, 16.0)
    );

    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    {
        let trace = output
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == id(1))
            .unwrap();
        let row_passes = trace
            .passes
            .iter()
            .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
            .collect::<Vec<_>>();
        assert_eq!(row_passes.len(), 1, "control must have one row pass");
        assert_eq!(
            row_passes[0].fixture_phase,
            Some(crate::LayoutGridTrackSizingFixturePhase::FinalDefinite)
        );
        assert!(
            row_passes[0].sequence > 0,
            "row control sequence is present"
        );
        let row_pass = row_passes[0];
        assert_eq!(row_pass.fixture_auto_repeat_count, Some(2));
        assert_eq!(row_pass.fixture_percentage_basis, Some(40.0));
        assert_rows(
            &used_rows(row_pass),
            &[(false, 20.0), (false, 20.0)],
            "definite control",
        );
    }
    #[cfg(not(feature = "c11-4-grid-track-sizing-fixture-test"))]
    {
        let trace = output
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == id(1))
            .unwrap();
        let row_pass = trace
            .passes
            .iter()
            .rev()
            .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
            .unwrap();
        assert_rows(
            &used_rows(row_pass),
            &[(false, 20.0), (false, 20.0)],
            "definite control",
        );
    }
}

#[test]
fn definite_row_control_with_authored_max_height_stays_fail_closed() {
    let mut input = row_target_input(LayoutDimension::Fixed(40.0));
    input.nodes[0].style.max_height = LayoutDimension::Fixed(41.0);
    assert_invalid_target(input, "control-authored-max-height");
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn cyclic_percentage_row_scene_mutations_fail_closed_at_grid_template_rows() {
    let mut input = row_target_input(LayoutDimension::MaxContent);
    input.nodes[0].style.width = LayoutDimension::Fixed(23.0);
    assert_invalid_target(input, "target-width");

    let mut input = row_target_input(LayoutDimension::MaxContent);
    input.grid_containers.get_mut(&id(1)).unwrap().columns[0] = GridTrackSize::LengthPx(23.0);
    assert_invalid_target(input, "target-columns");

    let mut input = row_target_input(LayoutDimension::MaxContent);
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .rows_auto_repeat
        .as_mut()
        .unwrap()
        .mode = GridAutoRepeatMode::AutoFit;
    assert_invalid_target(input, "target-repeat-mode");

    let mut input = row_target_input(LayoutDimension::MaxContent);
    input.nodes[0].style.gap.row = LayoutLengthPercentage::LengthPx(1.0);
    assert_invalid_target(input, "target-row-gap");

    let mut input = row_target_input(LayoutDimension::MaxContent);
    input.nodes[0].style.align_content = Some(LayoutAlignContent::Position {
        position: ContentAlignmentPosition::Center,
        safety: None,
    });
    assert_invalid_target(input, "target-alignment");

    let mut input = row_target_input(LayoutDimension::MaxContent);
    input.grid_containers.get_mut(&id(1)).unwrap().auto_flow = crate::GridAutoFlow::Column;
    assert_invalid_target(input, "target-grid-flow");

    let mut input = row_target_input(LayoutDimension::MaxContent);
    input.grid_items.get_mut(&id(2)).unwrap().row_start = GridPlacementInput::Line(2);
    assert_invalid_target(input, "target-placement");

    let mut input = row_target_input(LayoutDimension::MaxContent);
    input.nodes[1].style.min_height = LayoutDimension::Fixed(1.0);
    assert_invalid_target(input, "target-min-height");

    let mut input = row_target_input(LayoutDimension::MaxContent);
    input.nodes[0].style.padding.top = LayoutLengthPercentage::LengthPx(1.0);
    assert_invalid_target(input, "target-inset");

    let mut input = row_target_input(LayoutDimension::MaxContent);
    append_static_child(&mut input);
    assert_invalid_target(input, "target-extra-inflow-child");

    let mut input = row_target_input(LayoutDimension::MaxContent);
    let marker = id(3);
    input.nodes[0].children.push(marker);
    input.nodes.push(crate::LayoutNode {
        id: marker,
        children: Vec::new(),
        style: LayoutStyle {
            display: crate::LayoutDisplay::Block,
            width: LayoutDimension::Fixed(24.0),
            height: LayoutDimension::Fixed(1.0),
            ..LayoutStyle::default()
        },
    });
    input.positioning.insert(
        marker,
        LayoutPositioning {
            position: LayoutPosition::Absolute,
            inset: LayoutEdges {
                top: LayoutLengthPercentage::Percentage(0.5),
                ..LayoutEdges::default()
            },
        },
    );
    assert_invalid_target(input, "target-browser-marker-as-node");

    let mut input = row_target_input(LayoutDimension::MaxContent);
    input.nodes[0].style.min_height = LayoutDimension::Fixed(0.0);
    assert_invalid_target(input, "target-root-min-height-fixed-zero");
}

#[cfg(not(feature = "c11-4-grid-track-sizing-fixture-test"))]
#[test]
fn cyclic_percentage_row_target_remains_rejected_without_fixture_feature() {
    assert_invalid_target(
        row_target_input(LayoutDimension::MaxContent),
        "fixture feature disabled",
    );
}

#[path = "cyclic_percentage_auto_repeat_rows/generalized.rs"]
mod generalized;
