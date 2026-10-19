use std::collections::BTreeMap;

use spinon_core::{
    DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument, HostNodeHandle,
    HostParent, IntrinsicSizeRevision, OwnerId, StyleRevision,
};

use crate::{
    GridContainerInput, GridItemInput, GridPlacementInput, GridPlacementMode,
    GridTrackLengthPercentage, GridTrackMaxSizingFunction, GridTrackMinSizingFunction,
    GridTrackSize, LayoutAlignContent, LayoutAlignItems, LayoutBorder, LayoutBoxSizing,
    LayoutDimension, LayoutDisplay, LayoutEdges, LayoutGap, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, LayoutGridTrackSizingMinimumContributionSource,
    LayoutInput, LayoutIntrinsicSize, LayoutIntrinsicSizeSnapshot, LayoutJustifyContent,
    LayoutLengthPercentage, LayoutSourceRevision, LayoutStyle, TaffyLayoutEngine, Viewport,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const NATURAL_IMAGE: LayoutIntrinsicSize = LayoutIntrinsicSize::new(24.0, 16.0);

struct FixedGutterCase {
    owner: u64,
    grid_size: (f32, f32),
    column_gap: f32,
    row_gap: f32,
    columns: Vec<GridTrackSize>,
    rows: Vec<GridTrackSize>,
    placement: GridItemInput,
    preferred_size: (LayoutDimension, LayoutDimension),
    preferred_axis: LayoutGridTrackSizingAxis,
    expected_limit: Option<f64>,
    expected_track_sum: Option<f64>,
    expected_gutter_sum: Option<f64>,
    expected_preferred: f32,
    expected_transferred: f32,
    target_axis: LayoutGridTrackSizingAxis,
    expected_frame: (f32, f32),
    content_box: bool,
}

fn document_with_image(owner: u64) -> (HostDocument, HostNodeHandle, HostNodeHandle) {
    let mut document = HostDocument::new().unwrap();
    let grid = document.reserve_node_handle().unwrap();
    let image = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(owner).unwrap(), document.document_revision());
    batch
        .push(DocumentOperation::CreateElement {
            node: grid,
            namespace: HTML.to_owned(),
            local_name: "main".to_owned(),
        })
        .push(DocumentOperation::CreateElement {
            node: image,
            namespace: HTML.to_owned(),
            local_name: "img".to_owned(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: grid,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(grid),
            node: image,
            before: None,
        });
    document.commit(batch).unwrap();
    (document, grid, image)
}

fn run_case(
    case: FixedGutterCase,
    min_max: Option<(
        LayoutDimension,
        LayoutDimension,
        LayoutDimension,
        LayoutDimension,
    )>,
) {
    let (document, grid, image) = document_with_image(case.owner);
    let snapshot = document.snapshot();
    let intrinsic = LayoutIntrinsicSizeSnapshot::new(
        LayoutSourceRevision::HostDocument {
            generation: snapshot.generation(),
            document: snapshot.document_revision(),
            render_tree: snapshot.render_tree_revision(),
        },
        IntrinsicSizeRevision::INITIAL.checked_next().unwrap(),
        BTreeMap::from([(image.id(), NATURAL_IMAGE)]),
    )
    .unwrap();
    let grid_style = LayoutStyle {
        display: LayoutDisplay::Grid,
        width: LayoutDimension::Fixed(case.grid_size.0),
        height: LayoutDimension::Fixed(case.grid_size.1),
        gap: LayoutGap {
            row: LayoutLengthPercentage::length(case.row_gap),
            column: LayoutLengthPercentage::length(case.column_gap),
        },
        align_items: LayoutAlignItems::FlexStart,
        align_content: Some(LayoutAlignContent::FlexStart),
        justify_content: LayoutJustifyContent::FlexStart,
        ..LayoutStyle::default()
    };
    let image_style = LayoutStyle {
        display: LayoutDisplay::Block,
        box_sizing: if case.content_box {
            LayoutBoxSizing::ContentBox
        } else {
            LayoutBoxSizing::BorderBox
        },
        width: case.preferred_size.0,
        height: case.preferred_size.1,
        min_width: min_max.map_or(LayoutDimension::Auto, |values| values.0),
        max_width: min_max.map_or(LayoutDimension::Auto, |values| values.1),
        min_height: min_max.map_or(LayoutDimension::Auto, |values| values.2),
        max_height: min_max.map_or(LayoutDimension::Auto, |values| values.3),
        aspect_ratio: Some(1.5),
        padding: if case.content_box {
            let edge = LayoutLengthPercentage::length(1.0);
            LayoutEdges {
                top: edge,
                right: edge,
                bottom: edge,
                left: edge,
            }
        } else {
            LayoutEdges::default()
        },
        border: if case.content_box {
            LayoutBorder {
                top: 1.0,
                right: 1.0,
                bottom: 1.0,
                left: 1.0,
            }
        } else {
            LayoutBorder::default()
        },
        ..LayoutStyle::default()
    };
    let input = LayoutInput::from_host_document(
        &snapshot,
        grid,
        Viewport {
            width: case.grid_size.0,
            height: case.grid_size.1,
        },
        &BTreeMap::from([(grid.id(), grid_style), (image.id(), image_style)]),
        StyleRevision::default(),
        EnvironmentRevision::default(),
    )
    .unwrap()
    .with_grid_layout(
        BTreeMap::from([(
            grid.id(),
            GridContainerInput {
                columns: case.columns,
                rows: case.rows,
                placement_mode: GridPlacementMode::TrackSizing,
                ..GridContainerInput::default()
            },
        )]),
        BTreeMap::from([(image.id(), case.placement)]),
    );
    let output = TaffyLayoutEngine
        .compute_with_intrinsic_size_snapshot(&input, &intrinsic)
        .expect("fixed gutter case should lay out");
    assert!(
        output.frames.values().all(|frame| {
            frame.x.is_finite()
                && frame.y.is_finite()
                && frame.width.is_finite()
                && frame.height.is_finite()
        }),
        "{}: layout returned a non-finite frame",
        case.owner
    );
    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == grid.id())
        .expect("Grid trace should exist");

    let preferred = trace
        .passes
        .iter()
        .find(|pass| pass.axis == case.preferred_axis)
        .unwrap();
    let preferred_minimum = preferred
        .contributions
        .iter()
        .find(|contribution| {
            contribution.item == image.id()
                && contribution.kind == LayoutGridTrackSizingContributionKind::Minimum
        })
        .expect("preferred-size minimum should be traced");
    assert!(
        matches!(
            preferred_minimum.minimum_source,
            Some(LayoutGridTrackSizingMinimumContributionSource::PreferredSize {
                fixed_track_limit,
                fixed_track_sum,
                fixed_gutter_sum,
                ..
            }) if fixed_track_limit == case.expected_limit
                && fixed_track_sum == case.expected_track_sum
                && fixed_gutter_sum == case.expected_gutter_sum
        ),
        "unexpected preferred minimum source/value: {:?} / {}",
        preferred_minimum.minimum_source,
        preferred_minimum.value
    );
    assert!(
        (preferred_minimum.value - case.expected_preferred).abs() <= 0.02,
        "{}: preferred contribution expected {}, got {}",
        case.owner,
        case.expected_preferred,
        preferred_minimum.value
    );

    let target = trace
        .passes
        .iter()
        .find(|pass| pass.axis == case.target_axis)
        .unwrap();
    let transferred = target
        .contributions
        .iter()
        .find(|contribution| {
            contribution.item == image.id()
                && contribution.kind == LayoutGridTrackSizingContributionKind::Minimum
        })
        .expect("transferred automatic minimum should be traced");
    assert!(
        (transferred.value - case.expected_transferred).abs() <= 0.02,
        "{}: transferred minimum expected {}, got {}",
        case.owner,
        case.expected_transferred,
        transferred.value
    );
    let item_frame = output.frames[&image.id()];
    assert!(
        (item_frame.width - case.expected_frame.0).abs() <= 0.02
            && (item_frame.height - case.expected_frame.1).abs() <= 0.02,
        "{}: item frame expected {:?}, got {}x{}",
        case.owner,
        case.expected_frame,
        item_frame.width,
        item_frame.height
    );
}

fn fixed_track(max: f32) -> GridTrackSize {
    GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::Auto,
        max: GridTrackMaxSizingFunction::LengthPx(max),
    }
}

fn placement(row_start: i32, row_end: i32, column_start: i32, column_end: i32) -> GridItemInput {
    GridItemInput {
        row_start: GridPlacementInput::Line(row_start),
        row_end: GridPlacementInput::Line(row_end),
        column_start: GridPlacementInput::Line(column_start),
        column_end: GridPlacementInput::Line(column_end),
    }
}

mod cases;
