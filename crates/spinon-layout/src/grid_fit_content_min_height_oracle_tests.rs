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
const ROOT_MIN_HEIGHT: f32 = 300.0;
const ITEM_HEIGHT: f32 = 10.0;
const EPSILON: f32 = 1.0e-4;

fn id(value: u64) -> NodeId {
    NodeId::new(value).unwrap()
}

fn input() -> LayoutInput {
    let root = id(1);
    let item = id(2);
    let child = id(3);

    LayoutInput {
        root,
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: ROOT_WIDTH,
            height: ROOT_MIN_HEIGHT,
        },
        root_sizing: RootSizingPolicy::ResolveWithin,
        nodes: vec![
            LayoutNode {
                id: root,
                children: vec![item],
                style: LayoutStyle {
                    display: LayoutDisplay::Grid,
                    width: LayoutDimension::Fixed(ROOT_WIDTH),
                    height: LayoutDimension::Auto,
                    min_height: LayoutDimension::Fixed(ROOT_MIN_HEIGHT),
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
                id: item,
                children: vec![child],
                style: LayoutStyle {
                    display: LayoutDisplay::Block,
                    width: LayoutDimension::Auto,
                    height: LayoutDimension::Auto,
                    ..LayoutStyle::default()
                },
            },
            LayoutNode {
                id: child,
                children: Vec::new(),
                style: LayoutStyle {
                    display: LayoutDisplay::Block,
                    width: LayoutDimension::Fixed(ROOT_WIDTH),
                    height: LayoutDimension::Fixed(ITEM_HEIGHT),
                    ..LayoutStyle::default()
                },
            },
        ],
        css_math: Vec::new(),
        positioning: BTreeMap::new(),
        grid_containers: BTreeMap::from([(
            root,
            GridContainerInput {
                columns: vec![GridTrackSize::LengthPx(ROOT_WIDTH)],
                rows: Vec::new(),
                auto_rows: GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
                    GridTrackLengthPercentage::LengthPx(40.0),
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

fn close(actual: f32, expected: f32, label: &str) {
    assert!(
        actual.is_finite() && (actual - expected).abs() <= EPSILON,
        "{label}: expected {expected}, got {actual}"
    );
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

fn assert_one_row(stage: &LayoutGridTrackSizingStageTrace, expected: f32, label: &str) {
    let css_tracks = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .collect::<Vec<_>>();
    assert_eq!(css_tracks.len(), 1, "{label}: CSS track count");
    close(css_tracks[0].base_size, expected, label);

    assert_eq!(stage.tracks.len(), 3, "{label}: one row and two gutters");
    assert!(stage.tracks[0].is_gutter && stage.tracks[0].is_collapsed);
    assert!(!stage.tracks[1].is_gutter && !stage.tracks[1].is_collapsed);
    assert!(stage.tracks[2].is_gutter && stage.tracks[2].is_collapsed);
    close(stage.tracks[0].base_size, 0.0, label);
    close(stage.tracks[2].base_size, 0.0, label);
}

#[test]
fn css_grid_2025_fit_content_row_does_not_stretch_to_min_height() {
    let input = input();
    let root_style = &input.nodes[0].style;
    assert_eq!(root_style.height, LayoutDimension::Auto);
    assert_eq!(
        root_style.min_height,
        LayoutDimension::Fixed(ROOT_MIN_HEIGHT)
    );

    let container = &input.grid_containers[&id(1)];
    assert!(
        container.rows.is_empty(),
        "the tested row should be implicit"
    );
    assert_eq!(
        container.auto_rows,
        GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
            GridTrackLengthPercentage::LengthPx(40.0),
        )])
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    let rows = rows_pass(&output);
    assert_eq!(
        rows.stages
            .iter()
            .map(|stage| stage.stage)
            .collect::<Vec<_>>(),
        [
            LayoutGridTrackSizingStageKind::Initialize,
            LayoutGridTrackSizingStageKind::IntrinsicItemBatch {
                span: 1,
                crosses_flexible_track: false,
            },
            LayoutGridTrackSizingStageKind::IntrinsicSizingComplete,
            LayoutGridTrackSizingStageKind::MaximizeTracks,
            LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
            LayoutGridTrackSizingStageKind::StretchAutoTracks { applied: true },
        ],
        "the complete row sizing trace should match the pinned fixture"
    );
    assert_one_row(&rows.stages[0], 0.0, "initial row size");
    assert_one_row(
        &rows.stages[1],
        ITEM_HEIGHT,
        "intrinsic item batch row size",
    );
    let (stretch_stage, stretch_index) = unique_stage(
        rows,
        |stage| {
            matches!(
                stage,
                LayoutGridTrackSizingStageKind::StretchAutoTracks { .. }
            )
        },
        "StretchAutoTracks",
    );
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

    for (stage_kind, label) in [
        (
            LayoutGridTrackSizingStageKind::IntrinsicSizingComplete,
            "intrinsic sizing complete",
        ),
        (
            LayoutGridTrackSizingStageKind::MaximizeTracks,
            "maximize tracks",
        ),
        (
            LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
            "expand flexible tracks",
        ),
    ] {
        let (stage, index) = unique_stage(rows, |stage| stage == stage_kind, label);
        assert!(index < stretch_index, "{label} should precede stretch");
        assert_one_row(stage, ITEM_HEIGHT, label);
    }
    assert_one_row(stretch_stage, ITEM_HEIGHT, "fit-content row after stretch");

    for (node, expected_y, expected_height) in [
        (id(1), 0.0, ROOT_MIN_HEIGHT),
        (id(2), 0.0, ITEM_HEIGHT),
        (id(3), 0.0, ITEM_HEIGHT),
    ] {
        let frame = &output.frames[&node];
        close(frame.x, 0.0, "frame x");
        close(frame.y, expected_y, "frame y");
        close(frame.width, ROOT_WIDTH, "frame width");
        close(frame.height, expected_height, "frame height");
    }
}

#[test]
fn fit_content_percentage_limit_is_rejected_with_indefinite_height_basis() {
    let mut input = input();
    input.grid_containers.get_mut(&id(1)).unwrap().auto_rows =
        GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
            GridTrackLengthPercentage::Percentage(50.0),
        )]);

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::IndefiniteGridBasis {
            node,
            axis: "height"
        }) if node == id(1)
    ));
}

#[test]
fn fit_content_length_without_definite_min_height_remains_rejected() {
    let mut input = input();
    input.nodes[0].style.min_height = LayoutDimension::Auto;

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::IndefiniteGridBasis {
            node,
            axis: "height"
        }) if node == id(1)
    ));
}

#[test]
fn invalid_min_height_is_rejected_by_style_validation() {
    for value in [-1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        let mut input = input();
        input.nodes[0].style.min_height = LayoutDimension::Fixed(value);

        assert!(
            matches!(
                TaffyLayoutEngine.compute(&input),
                Err(crate::LayoutError::InvalidStyle {
                    node,
                    field: "min_height"
                }) if node == id(1)
            ),
            "invalid min-height {value:?} must be rejected"
        );
    }
}

#[test]
fn invalid_fit_content_length_is_rejected_by_grid_validation() {
    for value in [-1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        let mut input = input();
        input.grid_containers.get_mut(&id(1)).unwrap().auto_rows =
            GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
                GridTrackLengthPercentage::LengthPx(value),
            )]);

        assert!(
            matches!(
                TaffyLayoutEngine.compute(&input),
                Err(crate::LayoutError::InvalidGridTrack {
                    node,
                    property: "grid-auto-rows"
                }) if node == id(1)
            ),
            "invalid fit-content({value:?}) must be rejected"
        );
    }
}
