use std::collections::BTreeMap;

use spinon_core::{EnvironmentRevision, NodeId, Revision, StyleRevision};

use crate::{
    GridAutoFlow, GridAutoRows, GridContainerInput, GridPlacementMode, GridTrackLengthPercentage,
    GridTrackSize, LayoutAlignContent, LayoutDimension, LayoutDisplay, LayoutEngine, LayoutGap,
    LayoutGridTrackSizingAxis, LayoutGridTrackSizingPassTrace, LayoutGridTrackSizingStageKind,
    LayoutGridTrackSizingStageTrace, LayoutInput, LayoutInputRevision, LayoutJustifyContent,
    LayoutLengthPercentage, LayoutNode, LayoutSourceRevision, LayoutStyle, RootSizingPolicy,
    TaffyLayoutEngine, Viewport,
};

const ROOT_WIDTH: f32 = 60.0;
const ROOT_HEIGHT: f32 = 300.0;
const INNER_HEIGHT: f32 = 10.0;
const EPSILON: f32 = 1.0e-4;

fn id(value: u64) -> NodeId {
    NodeId::new(value).unwrap()
}

fn input() -> LayoutInput {
    let root = id(1);
    let fit_content_item = id(2);
    let fit_content_inner = id(3);
    let auto_item = id(4);
    let auto_inner = id(5);

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
                children: vec![fit_content_item, auto_item],
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
            LayoutNode {
                id: fit_content_item,
                children: vec![fit_content_inner],
                style: LayoutStyle {
                    display: LayoutDisplay::Block,
                    width: LayoutDimension::Auto,
                    height: LayoutDimension::Auto,
                    ..LayoutStyle::default()
                },
            },
            fixed_block(fit_content_inner),
            LayoutNode {
                id: auto_item,
                children: vec![auto_inner],
                style: LayoutStyle {
                    display: LayoutDisplay::Block,
                    width: LayoutDimension::Auto,
                    height: LayoutDimension::Auto,
                    ..LayoutStyle::default()
                },
            },
            fixed_block(auto_inner),
        ],
        css_math: Vec::new(),
        positioning: BTreeMap::new(),
        grid_containers: BTreeMap::from([(
            root,
            GridContainerInput {
                columns: vec![GridTrackSize::LengthPx(ROOT_WIDTH)],
                rows: Vec::new(),
                auto_rows: GridAutoRows::TrackList(vec![
                    GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(40.0)),
                    GridTrackSize::Auto,
                ]),
                auto_flow: GridAutoFlow::Row,
                placement_mode: GridPlacementMode::TrackSizing,
                ..GridContainerInput::default()
            },
        )]),
        grid_items: BTreeMap::new(),
        text_paragraphs: BTreeMap::new(),
    }
}

fn fixed_block(node: NodeId) -> LayoutNode {
    LayoutNode {
        id: node,
        children: Vec::new(),
        style: LayoutStyle {
            display: LayoutDisplay::Block,
            width: LayoutDimension::Fixed(ROOT_WIDTH),
            height: LayoutDimension::Fixed(INNER_HEIGHT),
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

fn assert_track_snapshot(stage: &LayoutGridTrackSizingStageTrace, expected: &[f32], label: &str) {
    let css_tracks = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .collect::<Vec<_>>();
    assert_eq!(css_tracks.len(), expected.len(), "{label}: CSS track count");
    for (index, (track, expected)) in css_tracks.iter().zip(expected).enumerate() {
        close(
            track.base_size,
            *expected,
            &format!("{label}: row {index} base"),
        );
    }

    assert_eq!(stage.tracks.len(), 5, "{label}: two rows and three gutters");
    for (index, track) in stage.tracks.iter().enumerate() {
        let is_gutter = index % 2 == 0;
        assert_eq!(track.is_gutter, is_gutter, "{label}: entry {index} gutter");
        let is_boundary_gutter = is_gutter && (index == 0 || index + 1 == stage.tracks.len());
        assert_eq!(
            track.is_collapsed, is_boundary_gutter,
            "{label}: entry {index} collapsed sentinel"
        );
        if is_gutter {
            close(
                track.base_size,
                0.0,
                &format!("{label}: gutter {index} base"),
            );
        }
    }
}

fn rows_pass(output: &crate::LayoutOutput) -> &LayoutGridTrackSizingPassTrace {
    assert_eq!(
        output.grid_track_sizing_traces.len(),
        1,
        "expected one trace owned by the grid root"
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
    let (_, flexible_index) = unique_stage(
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
            && maximize_index < flexible_index
            && flexible_index < stretch_index,
        "row stages should order intrinsic, maximize, flexible expansion, then stretch"
    );
    (stretch, stretch_index)
}

fn assert_frame(node: NodeId, frame: &crate::LayoutFrame, x: f32, y: f32, width: f32, height: f32) {
    close(frame.x, x, &format!("node {node:?} x"));
    close(frame.y, y, &format!("node {node:?} y"));
    close(frame.width, width, &format!("node {node:?} width"));
    close(frame.height, height, &format!("node {node:?} height"));
}

#[test]
fn css_grid_2025_fit_content_row_leaves_space_for_neighbor_auto_row() {
    let input = input();
    let container = &input.grid_containers[&id(1)];
    assert!(
        container.rows.is_empty(),
        "both tested rows should be implicit"
    );
    assert_eq!(
        container.auto_rows,
        GridAutoRows::TrackList(vec![
            GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(40.0)),
            GridTrackSize::Auto,
        ]),
        "implicit row pattern should preserve fit-content then auto order"
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    let rows = rows_pass(&output);
    let (stretch_stage, stretch_index) = assert_stage_order(rows);
    assert_eq!(
        stretch_stage.stage,
        LayoutGridTrackSizingStageKind::StretchAutoTracks { applied: true }
    );
    assert_eq!(
        rows.stages
            .iter()
            .filter(|stage| matches!(
                stage.stage,
                LayoutGridTrackSizingStageKind::StretchAutoTracks { .. }
            ))
            .count(),
        1,
        "expected exactly one stretch stage"
    );

    for (stage_kind, expected, label) in [
        (
            LayoutGridTrackSizingStageKind::IntrinsicSizingComplete,
            &[10.0, 10.0][..],
            "intrinsic sizing complete",
        ),
        (
            LayoutGridTrackSizingStageKind::MaximizeTracks,
            &[10.0, 10.0][..],
            "maximize tracks",
        ),
        (
            LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
            &[10.0, 10.0][..],
            "expand flexible tracks",
        ),
    ] {
        let (stage, index) = unique_stage(rows, |stage| stage == stage_kind, label);
        assert!(index < stretch_index, "{label} should precede stretch");
        assert_track_snapshot(stage, expected, label);
    }
    assert_track_snapshot(stretch_stage, &[10.0, 290.0], "stretch auto row");

    for (node, x, y, width, height) in [
        (id(1), 0.0, 0.0, ROOT_WIDTH, ROOT_HEIGHT),
        (id(2), 0.0, 0.0, ROOT_WIDTH, 10.0),
        (id(3), 0.0, 0.0, ROOT_WIDTH, INNER_HEIGHT),
        (id(4), 0.0, 10.0, ROOT_WIDTH, 290.0),
        (id(5), 0.0, 10.0, ROOT_WIDTH, INNER_HEIGHT),
    ] {
        assert_frame(node, &output.frames[&node], x, y, width, height);
    }
}
