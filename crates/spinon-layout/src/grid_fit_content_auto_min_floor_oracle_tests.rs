use std::collections::BTreeMap;

use spinon_core::{EnvironmentRevision, NodeId, Revision, StyleRevision};

use crate::{
    GridAutoFlow, GridAutoRows, GridContainerInput, GridPlacementMode, GridTrackLengthPercentage,
    GridTrackSize, LayoutAlignContent, LayoutDimension, LayoutDisplay, LayoutEngine, LayoutGap,
    LayoutGridTrackSizingAxis, LayoutGridTrackSizingPassTrace, LayoutGridTrackSizingStageKind,
    LayoutInput, LayoutInputRevision, LayoutJustifyContent, LayoutLengthPercentage, LayoutNode,
    LayoutSourceRevision, LayoutStyle, RootSizingPolicy, TaffyLayoutEngine, Viewport,
};

const ROOT_WIDTH: f32 = 60.0;
const ROOT_HEIGHT: f32 = 300.0;
const FIT_CONTENT_LIMIT: f32 = 40.0;
const EPSILON: f32 = 1.0e-4;

fn id(value: u64) -> NodeId {
    NodeId::new(value).unwrap()
}

fn input(minimize_second_item: bool) -> LayoutInput {
    let root = id(1);
    let first_item = id(2);
    let first_child = id(3);
    let second_item = id(4);
    let second_child = id(5);

    LayoutInput {
        root,
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: ROOT_WIDTH,
            height: ROOT_HEIGHT,
        },
        root_sizing: RootSizingPolicy::Match,
        nodes: vec![
            LayoutNode {
                id: root,
                children: vec![first_item, second_item],
                style: LayoutStyle {
                    display: LayoutDisplay::Grid,
                    width: LayoutDimension::Fixed(ROOT_WIDTH),
                    height: LayoutDimension::Fixed(ROOT_HEIGHT),
                    gap: LayoutGap {
                        row: LayoutLengthPercentage::ZERO,
                        column: LayoutLengthPercentage::ZERO,
                    },
                    align_content: Some(LayoutAlignContent::Normal),
                    justify_content: LayoutJustifyContent::FlexStart,
                    ..LayoutStyle::default()
                },
            },
            grid_item(first_item, first_child, false),
            fixed_child(first_child, 10.0),
            grid_item(second_item, second_child, minimize_second_item),
            fixed_child(second_child, 100.0),
        ],
        css_math: Vec::new(),
        positioning: BTreeMap::new(),
        grid_containers: BTreeMap::from([(
            root,
            GridContainerInput {
                columns: vec![GridTrackSize::LengthPx(ROOT_WIDTH)],
                rows: Vec::new(),
                auto_rows: GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
                    GridTrackLengthPercentage::LengthPx(FIT_CONTENT_LIMIT),
                )]),
                auto_flow: GridAutoFlow::Row,
                placement_mode: GridPlacementMode::TrackSizing,
                ..GridContainerInput::default()
            },
        )]),
        grid_items: BTreeMap::new(),
        text_paragraphs: BTreeMap::new(),
    }
}

fn grid_item(node: NodeId, child: NodeId, min_height_zero: bool) -> LayoutNode {
    LayoutNode {
        id: node,
        children: vec![child],
        style: LayoutStyle {
            display: LayoutDisplay::Block,
            width: LayoutDimension::Auto,
            height: LayoutDimension::Auto,
            min_height: if min_height_zero {
                LayoutDimension::Fixed(0.0)
            } else {
                LayoutDimension::Auto
            },
            ..LayoutStyle::default()
        },
    }
}

fn fixed_child(node: NodeId, height: f32) -> LayoutNode {
    LayoutNode {
        id: node,
        children: Vec::new(),
        style: LayoutStyle {
            display: LayoutDisplay::Block,
            width: LayoutDimension::Fixed(ROOT_WIDTH),
            height: LayoutDimension::Fixed(height),
            ..LayoutStyle::default()
        },
    }
}

fn close(actual: f32, expected: f32, label: &str) {
    assert!(
        actual.is_finite() && (actual - expected).abs() <= EPSILON,
        "{label}: expected {expected}, got {actual}"
    );
}

fn rows_pass(output: &crate::LayoutOutput) -> &LayoutGridTrackSizingPassTrace {
    assert_eq!(output.grid_track_sizing_traces.len(), 1);
    let trace = &output.grid_track_sizing_traces[0];
    assert_eq!(trace.node, id(1));
    let mut passes = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows);
    let rows = passes.next().expect("expected row sizing pass");
    assert!(passes.next().is_none());
    rows
}

fn assert_rows(pass: &LayoutGridTrackSizingPassTrace, expected: [f32; 2]) {
    let expected_kinds = [
        LayoutGridTrackSizingStageKind::Initialize,
        LayoutGridTrackSizingStageKind::IntrinsicItemBatch {
            span: 1,
            crosses_flexible_track: false,
        },
        LayoutGridTrackSizingStageKind::IntrinsicSizingComplete,
        LayoutGridTrackSizingStageKind::MaximizeTracks,
        LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
        LayoutGridTrackSizingStageKind::StretchAutoTracks { applied: true },
    ];
    assert_eq!(pass.stages.len(), expected_kinds.len());
    let intrinsic = if expected[1] == FIT_CONTENT_LIMIT {
        [expected[0], 0.0]
    } else {
        expected
    };
    let snapshots = [
        [0.0, 0.0],
        intrinsic,
        intrinsic,
        expected,
        expected,
        expected,
    ];
    for (index, (stage, kind)) in pass.stages.iter().zip(expected_kinds).enumerate() {
        assert_eq!(stage.stage, kind, "row sizing stage {index}");
        let tracks = stage
            .tracks
            .iter()
            .filter(|track| !track.is_gutter && !track.is_collapsed)
            .collect::<Vec<_>>();
        assert_eq!(tracks.len(), 2, "row sizing stage {index} track count");
        for (row, (track, expected)) in tracks.iter().zip(snapshots[index]).enumerate() {
            close(
                track.base_size,
                expected,
                &format!("stage {index} row {row}"),
            );
        }
        assert_eq!(stage.tracks.len(), 5, "row sizing stage {index} entries");
        for (entry_index, track) in stage.tracks.iter().enumerate() {
            assert_eq!(track.is_gutter, entry_index % 2 == 0);
            if track.is_gutter {
                close(track.base_size, 0.0, "collapsed row gutter");
            }
        }
    }
}

fn assert_frame(output: &crate::LayoutOutput, node: NodeId, expected: (f32, f32, f32, f32)) {
    let frame = output.frames.get(&node).unwrap();
    for (actual, wanted, axis) in [
        (frame.x, expected.0, "x"),
        (frame.y, expected.1, "y"),
        (frame.width, expected.2, "width"),
        (frame.height, expected.3, "height"),
    ] {
        close(actual, wanted, &format!("node {} {axis}", node.get()));
    }
}

#[test]
fn fit_content_auto_minimum_floor_uses_contribution_above_argument() {
    let output = TaffyLayoutEngine.compute(&input(false)).unwrap();
    assert_rows(rows_pass(&output), [10.0, 100.0]);
    assert_frame(&output, id(1), (0.0, 0.0, 60.0, 300.0));
    assert_frame(&output, id(2), (0.0, 0.0, 60.0, 10.0));
    assert_frame(&output, id(3), (0.0, 0.0, 60.0, 10.0));
    assert_frame(&output, id(4), (0.0, 10.0, 60.0, 100.0));
    assert_frame(&output, id(5), (0.0, 10.0, 60.0, 100.0));
}

#[test]
fn fit_content_auto_minimum_zero_control_applies_argument_and_keeps_overflow() {
    let output = TaffyLayoutEngine.compute(&input(true)).unwrap();
    assert_rows(rows_pass(&output), [10.0, 40.0]);
    assert_frame(&output, id(1), (0.0, 0.0, 60.0, 300.0));
    assert_frame(&output, id(2), (0.0, 0.0, 60.0, 10.0));
    assert_frame(&output, id(3), (0.0, 0.0, 60.0, 10.0));
    assert_frame(&output, id(4), (0.0, 10.0, 60.0, 40.0));
    assert_frame(&output, id(5), (0.0, 10.0, 60.0, 100.0));
}
