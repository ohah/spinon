use std::collections::BTreeMap;

use spinon_core::{EnvironmentRevision, NodeId, Revision, StyleRevision};

use crate::{
    ContentAlignmentPosition, GridAutoFlow, GridContainerInput, GridPlacementMode, GridTrackSize,
    LayoutAlignContent, LayoutDimension, LayoutDisplay, LayoutEngine, LayoutGap,
    LayoutGridTrackSizingAxis, LayoutGridTrackSizingPassTrace, LayoutGridTrackSizingStageKind,
    LayoutGridTrackSizingStageTrace, LayoutInput, LayoutInputRevision, LayoutJustifyContent,
    LayoutLengthPercentage, LayoutNode, LayoutSourceRevision, LayoutStyle, RootSizingPolicy,
    TaffyLayoutEngine, Viewport,
};

const ROOT_SIZE: (f32, f32) = (20.0, 100.0);
const CHILD_HEIGHTS: [f32; 3] = [10.0, 20.0, 5.0];

fn id(value: u64) -> NodeId {
    NodeId::new(value).unwrap()
}

fn input(align_content: LayoutAlignContent) -> LayoutInput {
    let root = id(1);
    let children = [id(2), id(3), id(4)];
    let mut nodes = vec![LayoutNode {
        id: root,
        children: children.to_vec(),
        style: LayoutStyle {
            display: LayoutDisplay::Grid,
            width: LayoutDimension::Fixed(ROOT_SIZE.0),
            height: LayoutDimension::Fixed(ROOT_SIZE.1),
            gap: LayoutGap {
                row: LayoutLengthPercentage::LengthPx(10.0),
                column: LayoutLengthPercentage::ZERO,
            },
            align_content: Some(align_content),
            justify_content: LayoutJustifyContent::FlexStart,
            ..LayoutStyle::default()
        },
    }];
    nodes.extend(
        children
            .into_iter()
            .zip(CHILD_HEIGHTS)
            .map(|(child, height)| LayoutNode {
                id: child,
                children: Vec::new(),
                style: LayoutStyle {
                    display: LayoutDisplay::Block,
                    width: LayoutDimension::Fixed(ROOT_SIZE.0),
                    height: LayoutDimension::Fixed(height),
                    ..LayoutStyle::default()
                },
            }),
    );

    LayoutInput {
        root,
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: ROOT_SIZE.0,
            height: ROOT_SIZE.1,
        },
        root_sizing: RootSizingPolicy::Match,
        nodes,
        css_math: Vec::new(),
        positioning: BTreeMap::new(),
        grid_containers: BTreeMap::from([(
            root,
            GridContainerInput {
                columns: vec![GridTrackSize::LengthPx(ROOT_SIZE.0)],
                rows: vec![
                    GridTrackSize::Auto,
                    GridTrackSize::Auto,
                    GridTrackSize::LengthPx(CHILD_HEIGHTS[2]),
                ],
                columns_auto_repeat: None,
                rows_auto_repeat: None,
                columns_line_names: Vec::new(),
                rows_line_names: Vec::new(),
                template_areas: None,
                auto_flow: GridAutoFlow::Row,
                auto_rows: Default::default(),
                auto_columns: Default::default(),
                placement_mode: GridPlacementMode::TrackSizing,
            },
        )]),
        grid_items: Default::default(),
        text_paragraphs: Default::default(),
    }
}

fn close(actual: f32, expected: f32, label: &str) {
    assert!(
        actual.is_finite() && (actual - expected).abs() <= 1.0e-4,
        "{label}: expected {expected}, got {actual}"
    );
}

fn unique_stage<'a>(
    pass: &'a LayoutGridTrackSizingPassTrace,
    matches: impl Fn(LayoutGridTrackSizingStageKind) -> bool,
    label: &str,
) -> (&'a LayoutGridTrackSizingStageTrace, usize) {
    let matches = pass
        .stages
        .iter()
        .enumerate()
        .filter(|(_, stage)| matches(stage.stage))
        .collect::<Vec<_>>();
    assert_eq!(matches.len(), 1, "expected exactly one {label} stage");
    let (index, stage) = matches[0];
    (stage, index)
}

fn css_track_bases(stage: &LayoutGridTrackSizingStageTrace) -> Vec<f32> {
    stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect()
}

fn unused_row_space(stage: &LayoutGridTrackSizingStageTrace) -> f32 {
    ROOT_SIZE.1
        - stage
            .tracks
            .iter()
            .map(|track| track.base_size)
            .sum::<f32>()
}

fn assert_css_track_bases(stage: &LayoutGridTrackSizingStageTrace, expected: &[f32], label: &str) {
    let actual = css_track_bases(stage);
    assert_eq!(actual.len(), expected.len(), "{label}: CSS track count");
    for (index, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        close(actual, *expected, &format!("{label}: track {index}"));
    }

    assert_eq!(
        stage.tracks.len(),
        expected.len() * 2 + 1,
        "{label}: tracks and boundary gutter count"
    );
    for (index, track) in stage.tracks.iter().enumerate() {
        let is_gutter = index % 2 == 0;
        assert_eq!(
            track.is_gutter, is_gutter,
            "{label}: track entry {index} gutter flag"
        );
        let is_boundary_gutter = is_gutter && (index == 0 || index + 1 == stage.tracks.len());
        assert_eq!(
            track.is_collapsed, is_boundary_gutter,
            "{label}: track entry {index} collapsed flag"
        );
    }

    let gutters = stage
        .tracks
        .iter()
        .filter(|track| track.is_gutter)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(
        gutters.len(),
        4,
        "{label}: boundary and internal gutter count"
    );
    close(
        gutters[0],
        0.0,
        &format!("{label}: leading boundary gutter"),
    );
    close(gutters[1], 10.0, &format!("{label}: first row gap"));
    close(gutters[2], 10.0, &format!("{label}: second row gap"));
    close(
        gutters[3],
        0.0,
        &format!("{label}: trailing boundary gutter"),
    );
}

fn rows_pass(output: &crate::LayoutOutput) -> &LayoutGridTrackSizingPassTrace {
    assert_eq!(
        output.grid_track_sizing_traces.len(),
        1,
        "expected exactly one grid track sizing trace"
    );
    let trace = &output.grid_track_sizing_traces[0];
    assert_eq!(trace.node, id(1), "trace should belong to the grid root");
    let mut rows = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows);
    let pass = rows.next().expect("expected a row sizing pass");
    assert!(
        rows.next().is_none(),
        "expected exactly one row sizing pass"
    );
    pass
}

fn assert_frame(node: NodeId, frame: &crate::LayoutFrame, x: f32, y: f32, width: f32, height: f32) {
    close(frame.x, x, &format!("node {node:?} x"));
    close(frame.y, y, &format!("node {node:?} y"));
    close(frame.width, width, &format!("node {node:?} width"));
    close(frame.height, height, &format!("node {node:?} height"));
}

fn assert_stage_order(
    pass: &LayoutGridTrackSizingPassTrace,
) -> (&LayoutGridTrackSizingStageTrace, usize) {
    let (_, intrinsic_index) = unique_stage(
        pass,
        |stage| stage == LayoutGridTrackSizingStageKind::IntrinsicSizingComplete,
        "IntrinsicSizingComplete",
    );
    let (_, maximize_index) = unique_stage(
        pass,
        |stage| stage == LayoutGridTrackSizingStageKind::MaximizeTracks,
        "MaximizeTracks",
    );
    let (_, flex_index) = unique_stage(
        pass,
        |stage| stage == LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
        "ExpandFlexibleTracks",
    );
    let (stretch, stretch_index) = unique_stage(
        pass,
        |stage| {
            matches!(
                stage,
                LayoutGridTrackSizingStageKind::StretchAutoTracks { .. }
            )
        },
        "StretchAutoTracks",
    );
    assert!(
        intrinsic_index < maximize_index
            && maximize_index < flex_index
            && flex_index < stretch_index,
        "row stage order should be intrinsic, maximize, flex expansion, then auto stretch"
    );
    (stretch, stretch_index)
}

#[test]
fn css_grid_2025_stretch_auto_tracks_distributes_definite_free_space_equally() {
    let stretch_output = TaffyLayoutEngine
        .compute(&input(LayoutAlignContent::Stretch))
        .unwrap();
    let positive_rows = rows_pass(&stretch_output);
    let (stretch_stage, _) = assert_stage_order(positive_rows);
    assert_eq!(
        stretch_stage.stage,
        LayoutGridTrackSizingStageKind::StretchAutoTracks { applied: true }
    );

    for (stage, label) in [
        (
            unique_stage(
                positive_rows,
                |stage| stage == LayoutGridTrackSizingStageKind::IntrinsicSizingComplete,
                "IntrinsicSizingComplete",
            )
            .0,
            "stretch intrinsic complete",
        ),
        (
            unique_stage(
                positive_rows,
                |stage| stage == LayoutGridTrackSizingStageKind::MaximizeTracks,
                "MaximizeTracks",
            )
            .0,
            "stretch maximize tracks",
        ),
        (
            unique_stage(
                positive_rows,
                |stage| stage == LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
                "ExpandFlexibleTracks",
            )
            .0,
            "stretch flexible tracks",
        ),
    ] {
        assert_css_track_bases(stage, &[10.0, 20.0, 5.0], label);
    }
    assert_css_track_bases(stretch_stage, &[32.5, 42.5, 5.0], "stretch stage");
    close(
        unused_row_space(stretch_stage),
        0.0,
        "stretch remaining space",
    );

    assert_frame(
        id(1),
        &stretch_output.frames[&id(1)],
        0.0,
        0.0,
        ROOT_SIZE.0,
        ROOT_SIZE.1,
    );
    for (node, y, height) in [(id(2), 0.0, 10.0), (id(3), 42.5, 20.0), (id(4), 95.0, 5.0)] {
        assert_frame(
            node,
            &stretch_output.frames[&node],
            0.0,
            y,
            ROOT_SIZE.0,
            height,
        );
    }

    let start_output = TaffyLayoutEngine
        .compute(&input(LayoutAlignContent::Position {
            position: ContentAlignmentPosition::Start,
            safety: None,
        }))
        .unwrap();
    let start_rows = rows_pass(&start_output);
    let (start_stage, _) = assert_stage_order(start_rows);
    assert_eq!(
        start_stage.stage,
        LayoutGridTrackSizingStageKind::StretchAutoTracks { applied: false }
    );
    assert_css_track_bases(start_stage, &[10.0, 20.0, 5.0], "start control");
    close(
        unused_row_space(start_stage),
        45.0,
        "start unused free space",
    );

    assert_frame(
        id(1),
        &start_output.frames[&id(1)],
        0.0,
        0.0,
        ROOT_SIZE.0,
        ROOT_SIZE.1,
    );
    for (node, y, height) in [(id(2), 0.0, 10.0), (id(3), 20.0, 20.0), (id(4), 50.0, 5.0)] {
        assert_frame(
            node,
            &start_output.frames[&node],
            0.0,
            y,
            ROOT_SIZE.0,
            height,
        );
    }
}
