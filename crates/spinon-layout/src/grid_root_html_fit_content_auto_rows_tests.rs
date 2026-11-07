use std::collections::BTreeMap;

use spinon_core::{EnvironmentRevision, NodeId, Revision, StyleRevision};

use crate::{
    GridAutoFlow, GridAutoRows, GridContainerInput, GridItemInput, GridPlacementMode,
    GridTrackLengthPercentage, GridTrackSize, LayoutAlignContent, LayoutBoxSizing, LayoutDimension,
    LayoutDisplay, LayoutEngine, LayoutGap, LayoutGridTrackSizingAxis, LayoutInput,
    LayoutInputRevision, LayoutJustifyContent, LayoutLengthPercentage, LayoutNode,
    LayoutSourceRevision, LayoutStyle, RootSizingPolicy, TaffyLayoutEngine, Viewport,
};

const EPSILON: f32 = 1.0e-4;

fn id(value: u64) -> NodeId {
    NodeId::new(value).unwrap()
}

fn input(cap: f32, min_height: f32) -> LayoutInput {
    input_with_cap(GridTrackLengthPercentage::LengthPx(cap), min_height)
}

fn input_with_cap(cap: GridTrackLengthPercentage, min_height: f32) -> LayoutInput {
    let root = id(1);
    let first = id(2);
    let middle = id(3);
    let probe = id(4);
    let payload = id(5);
    let root_style = LayoutStyle {
        display: LayoutDisplay::Grid,
        box_sizing: LayoutBoxSizing::BorderBox,
        width: LayoutDimension::Fixed(40.0),
        height: LayoutDimension::Fixed(100.0),
        align_content: Some(LayoutAlignContent::FlexStart),
        justify_content: LayoutJustifyContent::FlexStart,
        gap: LayoutGap {
            row: LayoutLengthPercentage::ZERO,
            column: LayoutLengthPercentage::ZERO,
        },
        ..LayoutStyle::default()
    };
    LayoutInput {
        root,
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: 40.0,
            height: 100.0,
        },
        root_sizing: RootSizingPolicy::Match,
        nodes: vec![
            LayoutNode {
                id: root,
                children: vec![first, middle, probe],
                style: root_style,
            },
            LayoutNode {
                id: first,
                children: Vec::new(),
                style: LayoutStyle {
                    display: LayoutDisplay::Block,
                    width: LayoutDimension::Auto,
                    height: LayoutDimension::Fixed(10.0),
                    ..LayoutStyle::default()
                },
            },
            LayoutNode {
                id: middle,
                children: vec![payload],
                style: LayoutStyle {
                    display: LayoutDisplay::Block,
                    width: LayoutDimension::Auto,
                    height: LayoutDimension::Auto,
                    min_height: LayoutDimension::Fixed(min_height),
                    ..LayoutStyle::default()
                },
            },
            LayoutNode {
                id: probe,
                children: Vec::new(),
                style: LayoutStyle {
                    display: LayoutDisplay::Block,
                    width: LayoutDimension::Auto,
                    height: LayoutDimension::Fixed(6.0),
                    ..LayoutStyle::default()
                },
            },
            LayoutNode {
                id: payload,
                children: Vec::new(),
                style: LayoutStyle {
                    display: LayoutDisplay::Block,
                    width: LayoutDimension::Auto,
                    height: LayoutDimension::Fixed(34.0),
                    ..LayoutStyle::default()
                },
            },
        ],
        css_math: Vec::new(),
        positioning: BTreeMap::new(),
        grid_containers: BTreeMap::from([(
            root,
            GridContainerInput {
                columns: vec![GridTrackSize::LengthPx(40.0)],
                rows: vec![GridTrackSize::LengthPx(10.0)],
                auto_flow: GridAutoFlow::Row,
                auto_rows: GridAutoRows::TrackList(vec![GridTrackSize::FitContent(cap)]),
                placement_mode: GridPlacementMode::RootHtmlFitContentAutoRowsV1,
                track_sizing_trace_enabled: true,
                ..GridContainerInput::default()
            },
        )]),
        grid_items: BTreeMap::from([
            (first, GridItemInput::default()),
            (middle, GridItemInput::default()),
            (probe, GridItemInput::default()),
        ]),
        text_paragraphs: BTreeMap::new(),
    }
}

fn close(actual: f32, expected: f32, label: &str) {
    assert!(
        actual.is_finite() && (actual - expected).abs() <= EPSILON,
        "{label}: expected {expected}, got {actual}"
    );
}

fn final_rows(output: &crate::LayoutOutput) -> Vec<f32> {
    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == id(1))
        .expect("the Root HTML Grid trace should be recorded");
    let pass = trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .expect("the row track sizing pass should be recorded");
    pass.stages
        .last()
        .expect("the row track sizing should produce a final stage")
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect()
}

#[test]
fn root_html_fit_content_implicit_row_matches_cap_max_content_and_minimum_oracle() {
    for (cap, min_height, row_height, probe_y) in [
        (GridTrackLengthPercentage::LengthPx(0.0), 0.0, 0.0, 10.0),
        (GridTrackLengthPercentage::LengthPx(24.0), 0.0, 24.0, 34.0),
        (GridTrackLengthPercentage::LengthPx(40.0), 0.0, 34.0, 44.0),
        (GridTrackLengthPercentage::LengthPx(24.0), 28.0, 28.0, 38.0),
        (GridTrackLengthPercentage::Percentage(0.0), 0.0, 0.0, 10.0),
        (GridTrackLengthPercentage::Percentage(0.2), 0.0, 20.0, 30.0),
        (GridTrackLengthPercentage::Percentage(0.24), 0.0, 24.0, 34.0),
        (GridTrackLengthPercentage::Percentage(0.34), 0.0, 34.0, 44.0),
        (GridTrackLengthPercentage::Percentage(0.4), 0.0, 34.0, 44.0),
        (GridTrackLengthPercentage::Percentage(1.2), 0.0, 34.0, 44.0),
        (GridTrackLengthPercentage::Percentage(0.2), 28.0, 28.0, 38.0),
    ] {
        let input = input_with_cap(cap, min_height);
        let output = TaffyLayoutEngine
            .compute(&input)
            .expect("supported Root HTML fit-content scene should compute");
        assert_eq!(
            final_rows(&output),
            [10.0, row_height, 6.0],
            "cap={cap:?} min-height={min_height}"
        );

        for (node, expected_y, expected_height) in [
            (id(1), 0.0, 100.0),
            (id(2), 0.0, 10.0),
            (id(3), 10.0, row_height),
            (id(4), probe_y, 6.0),
            (id(5), 10.0, 34.0),
        ] {
            let frame = &output.frames[&node];
            close(frame.x, 0.0, "frame x");
            close(frame.y, expected_y, "frame y");
            close(frame.width, 40.0, "frame width");
            close(frame.height, expected_height, "frame height");
        }
    }
}

#[test]
fn root_html_fit_content_implicit_row_rejects_out_of_profile_inputs() {
    for cap in [-1.0, f32::NAN, f32::INFINITY] {
        let mut invalid_cap = input(24.0, 0.0);
        invalid_cap
            .grid_containers
            .get_mut(&id(1))
            .unwrap()
            .auto_rows = GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
            GridTrackLengthPercentage::LengthPx(cap),
        )]);
        assert!(matches!(
            TaffyLayoutEngine.compute(&invalid_cap),
            Err(crate::LayoutError::UnsupportedGridAutoRows { node, .. }) if node == id(1)
        ));
    }

    for cap in [-0.01, f32::NAN, f32::INFINITY] {
        let percentage_cap = input_with_cap(GridTrackLengthPercentage::Percentage(cap), 0.0);
        assert!(matches!(
            TaffyLayoutEngine.compute(&percentage_cap),
            Err(crate::LayoutError::UnsupportedGridAutoRows { node, .. }) if node == id(1)
        ));
    }

    for tracks in [
        Vec::new(),
        vec![
            GridTrackSize::LengthPx(12.0),
            GridTrackSize::Percentage(0.1),
        ],
        vec![
            GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.2)),
            GridTrackSize::LengthPx(-1.0),
        ],
        vec![
            GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.2)),
            GridTrackSize::Percentage(f32::NAN),
        ],
    ] {
        let mut invalid_list = input(24.0, 0.0);
        invalid_list
            .grid_containers
            .get_mut(&id(1))
            .unwrap()
            .auto_rows = GridAutoRows::TrackList(tracks);
        assert!(matches!(
            TaffyLayoutEngine.compute(&invalid_list),
            Err(crate::LayoutError::UnsupportedGridAutoRows { node, .. }) if node == id(1)
        ));
    }

    let mut nonzero_gap = input(24.0, 0.0);
    nonzero_gap.nodes[0].style.gap.row = LayoutLengthPercentage::LengthPx(1.0);
    assert!(matches!(
        TaffyLayoutEngine.compute(&nonzero_gap),
        Err(crate::LayoutError::UnsupportedGridContainerStyle { node, .. }) if node == id(1)
    ));

    let mut mixed_rows = input(24.0, 0.0);
    mixed_rows
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .auto_rows = GridAutoRows::TrackList(vec![
        GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(24.0)),
        GridTrackSize::Auto,
    ]);
    assert!(matches!(
        TaffyLayoutEngine.compute(&mixed_rows),
        Err(crate::LayoutError::UnsupportedGridAutoRows { node, .. }) if node == id(1)
    ));

    let mut oversized_list = input(24.0, 0.0);
    let mut tracks = vec![GridTrackSize::LengthPx(1.0); crate::GRID_MAX_TRACKS + 1];
    tracks[0] = GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(24.0));
    oversized_list
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .auto_rows = GridAutoRows::TrackList(tracks);
    assert!(matches!(
        TaffyLayoutEngine.compute(&oversized_list),
        Err(crate::LayoutError::UnsupportedGridAutoRows { node, .. }) if node == id(1)
    ));

    let mut wrong_flow = input(24.0, 0.0);
    wrong_flow
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .auto_flow = GridAutoFlow::Column;
    assert!(matches!(
        TaffyLayoutEngine.compute(&wrong_flow),
        Err(crate::LayoutError::UnsupportedGridContainerStyle { node, .. }) if node == id(1)
    ));

    let mut placed_item = input(24.0, 0.0);
    placed_item.grid_items.insert(
        id(3),
        crate::GridItemInput {
            row_start: crate::GridPlacementInput::Line(2),
            row_end: crate::GridPlacementInput::Line(3),
            column_start: crate::GridPlacementInput::Line(1),
            column_end: crate::GridPlacementInput::Line(2),
        },
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&placed_item),
        Err(crate::LayoutError::UnsupportedGridContainerStyle { node, .. }) if node == id(1)
    ));
}

mod track_lists;
