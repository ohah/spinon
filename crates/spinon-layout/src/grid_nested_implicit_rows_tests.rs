use std::collections::BTreeMap;

use spinon_core::{EnvironmentRevision, NodeId, Revision, StyleRevision};

use crate::{
    GridAutoRows, GridContainerInput, GridItemInput, GridPlacementInput, GridPlacementMode,
    GridTrackSize, LayoutDimension, LayoutDisplay, LayoutEngine, LayoutGap,
    LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind, LayoutInput,
    LayoutInputRevision, LayoutLengthPercentage, LayoutNode, LayoutSourceRevision, LayoutStyle,
    RootSizingPolicy, TaffyLayoutEngine, Viewport,
};

#[derive(Clone, Copy)]
struct Scene {
    root_rows: [f32; 2],
    root_height: f32,
    nested_row_end: i32,
    contribution_span: u16,
}

const SCENES: [Scene; 2] = [
    Scene {
        root_rows: [18.0, 18.0],
        root_height: 40.0,
        nested_row_end: 3,
        contribution_span: 2,
    },
    Scene {
        root_rows: [40.0, 0.0],
        root_height: 44.0,
        nested_row_end: 2,
        contribution_span: 1,
    },
];

fn id(value: u64) -> NodeId {
    NodeId::new(value).unwrap()
}

fn row_gap() -> LayoutGap {
    LayoutGap {
        row: LayoutLengthPercentage::length(4.0),
        column: LayoutLengthPercentage::ZERO,
    }
}

fn input(scene: Scene) -> LayoutInput {
    let root = id(1);
    let nested = id(2);
    let explicit = id(3);
    let first_implicit = id(4);
    let second_implicit = id(5);
    let nodes = vec![
        LayoutNode {
            id: root,
            children: vec![nested],
            style: LayoutStyle {
                display: LayoutDisplay::Grid,
                width: LayoutDimension::Fixed(24.0),
                height: LayoutDimension::Auto,
                gap: row_gap(),
                ..LayoutStyle::default()
            },
        },
        LayoutNode {
            id: nested,
            children: vec![explicit, first_implicit, second_implicit],
            style: LayoutStyle {
                display: LayoutDisplay::Grid,
                width: LayoutDimension::Fixed(24.0),
                height: LayoutDimension::Auto,
                gap: row_gap(),
                ..LayoutStyle::default()
            },
        },
        fixed_block(explicit, 24.0, 8.0),
        fixed_block(first_implicit, 4.0, 8.0),
        fixed_block(second_implicit, 24.0, 16.0),
    ];
    let explicit_item = |row_start, row_end| GridItemInput {
        row_start: GridPlacementInput::Line(row_start),
        row_end: GridPlacementInput::Line(row_end),
        column_start: GridPlacementInput::Line(1),
        column_end: GridPlacementInput::Line(2),
    };
    LayoutInput {
        root,
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: 24.0,
            height: 100.0,
        },
        root_sizing: RootSizingPolicy::ResolveWithin,
        nodes,
        css_math: Vec::new(),
        positioning: BTreeMap::new(),
        grid_containers: BTreeMap::from([
            (
                root,
                GridContainerInput {
                    columns: vec![GridTrackSize::LengthPx(24.0)],
                    rows: vec![GridTrackSize::MinContent, GridTrackSize::MinContent],
                    placement_mode: GridPlacementMode::TrackSizing,
                    ..GridContainerInput::default()
                },
            ),
            (
                nested,
                GridContainerInput {
                    columns: vec![GridTrackSize::LengthPx(24.0)],
                    rows: vec![GridTrackSize::MinContent],
                    auto_rows: GridAutoRows::TrackList(vec![
                        GridTrackSize::MinContent,
                        GridTrackSize::MaxContent,
                    ]),
                    placement_mode: GridPlacementMode::TrackSizing,
                    ..GridContainerInput::default()
                },
            ),
        ]),
        grid_items: BTreeMap::from([
            (nested, explicit_item(1, scene.nested_row_end)),
            (explicit, explicit_item(1, 2)),
            (first_implicit, explicit_item(2, 3)),
            (second_implicit, explicit_item(3, 4)),
        ]),
        text_paragraphs: BTreeMap::new(),
    }
}

fn fixed_block(id: NodeId, width: f32, height: f32) -> LayoutNode {
    LayoutNode {
        id,
        children: Vec::new(),
        style: LayoutStyle {
            display: LayoutDisplay::Block,
            width: LayoutDimension::Fixed(width),
            height: LayoutDimension::Fixed(height),
            ..LayoutStyle::default()
        },
    }
}

#[test]
fn nested_positive_implicit_rows_contribute_through_the_outer_grid() {
    for scene in SCENES {
        let input = input(scene);
        let output = TaffyLayoutEngine.compute(&input).unwrap();
        let root = output.frames[&id(1)];
        let nested = output.frames[&id(2)];
        assert_eq!((root.width, root.height), (24.0, scene.root_height));
        assert_eq!(
            (nested.x, nested.y, nested.width, nested.height),
            (0.0, 0.0, 24.0, 40.0)
        );

        for (node, expected_y, expected_height, expected_width) in [
            (id(3), 0.0, 8.0, 24.0),
            (id(4), 12.0, 8.0, 4.0),
            (id(5), 24.0, 16.0, 24.0),
        ] {
            let frame = output.frames[&node];
            assert_eq!(
                (frame.x, frame.y - nested.y, frame.width, frame.height),
                (0.0, expected_y, expected_width, expected_height)
            );
        }

        let outer_trace = output
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == id(1))
            .expect("outer Grid Rows trace");
        let nested_trace = output
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == id(2))
            .expect("nested Grid Rows trace");
        assert_axis_tracks(outer_trace, &scene.root_rows);
        assert_axis_tracks(nested_trace, &[8.0, 8.0, 16.0]);

        let outer_rows = outer_trace
            .passes
            .iter()
            .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
            .expect("outer Rows pass");
        assert!(outer_rows.contributions.iter().any(|contribution| {
            contribution.item == id(2)
                && contribution.kind == LayoutGridTrackSizingContributionKind::MinContent
                && contribution.span == scene.contribution_span
                && (contribution.value - 40.0).abs() <= 1.0 / 64.0
        }));
    }
}

fn assert_axis_tracks(trace: &crate::LayoutGridTrackSizingTrace, expected: &[f32]) {
    let pass = trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .expect("Rows pass");
    let final_stage = pass.stages.last().expect("final Rows stage");
    let tracks = final_stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(tracks, expected);
}
