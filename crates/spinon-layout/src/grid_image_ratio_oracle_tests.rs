use std::collections::BTreeMap;

use spinon_core::{
    DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument, HostNodeHandle,
    HostParent, IntrinsicSizeRevision, OwnerId, StyleRevision,
};

use crate::{
    GridContainerInput, GridPlacementMode, GridTrackMaxSizingFunction, GridTrackMinSizingFunction,
    GridTrackSize, LayoutAlignContent, LayoutAlignItems, LayoutEngine, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, LayoutInput, LayoutIntrinsicSize,
    LayoutIntrinsicSizeSnapshot, LayoutJustifyContent, LayoutSourceRevision, LayoutStyle,
    TaffyLayoutEngine, Viewport,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const NATURAL_IMAGE: LayoutIntrinsicSize = LayoutIntrinsicSize::new(24.0, 16.0);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Axis {
    Columns,
    Rows,
}

struct ImageRatioCase {
    id: &'static str,
    axis: Axis,
    columns: GridTrackSize,
    rows: GridTrackSize,
    grid: (f32, f32),
    image_width: f32,
    image_height: f32,
    aspect_ratio: Option<f32>,
    min_width: f32,
    max_width: f32,
    min_height: f32,
    max_height: f32,
    expects_minimum_query: bool,
    expected_track: f32,
    expected_item: (f32, f32),
    expected_overflow: (bool, bool),
}

fn create_document(owner: u64) -> (HostDocument, HostNodeHandle, HostNodeHandle) {
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

fn run_case(case: &ImageRatioCase, owner: u64) {
    run_case_with_overflow(case, owner, crate::LayoutOverflow::Visible);
}

fn run_case_with_overflow(case: &ImageRatioCase, owner: u64, overflow_x: crate::LayoutOverflow) {
    run_case_with_box_decoration(
        case,
        owner,
        overflow_x,
        crate::LayoutBoxSizing::BorderBox,
        crate::LayoutEdges::default(),
        crate::LayoutBorder::default(),
    );
}

fn run_case_with_box_decoration(
    case: &ImageRatioCase,
    owner: u64,
    overflow_x: crate::LayoutOverflow,
    box_sizing: crate::LayoutBoxSizing,
    padding: crate::LayoutEdges,
    border: crate::LayoutBorder,
) {
    let (document, grid, image) = create_document(owner);
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
        display: crate::LayoutDisplay::Grid,
        width: crate::LayoutDimension::Fixed(case.grid.0),
        height: crate::LayoutDimension::Fixed(case.grid.1),
        align_items: LayoutAlignItems::FlexStart,
        align_content: Some(LayoutAlignContent::FlexStart),
        justify_content: LayoutJustifyContent::FlexStart,
        ..LayoutStyle::default()
    };
    let image_style = LayoutStyle {
        display: crate::LayoutDisplay::Block,
        width: dimension(case.image_width),
        height: dimension(case.image_height),
        aspect_ratio: case.aspect_ratio,
        min_width: dimension(case.min_width),
        max_width: dimension(case.max_width),
        min_height: dimension(case.min_height),
        max_height: dimension(case.max_height),
        box_sizing,
        padding,
        border,
        overflow_x,
        ..LayoutStyle::default()
    };
    let input = LayoutInput::from_host_document(
        &snapshot,
        grid,
        Viewport {
            width: case.grid.0,
            height: case.grid.1,
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
                columns: vec![case.columns],
                rows: vec![case.rows],
                placement_mode: GridPlacementMode::TrackSizing,
                ..GridContainerInput::default()
            },
        )]),
        BTreeMap::new(),
    );
    let output = TaffyLayoutEngine
        .compute_with_intrinsic_size_snapshot(&input, &intrinsic)
        .unwrap_or_else(|error| panic!("{}: {error:?}", case.id));
    let grid_frame = output.frames[&grid.id()];
    let image_frame = output.frames[&image.id()];
    close(grid_frame.width, case.grid.0, case.id, "grid.width");
    close(grid_frame.height, case.grid.1, case.id, "grid.height");
    close(
        image_frame.width,
        case.expected_item.0,
        case.id,
        "image.width",
    );
    close(
        image_frame.height,
        case.expected_item.1,
        case.id,
        "image.height",
    );
    assert_eq!(image_frame.x, grid_frame.x, "{}: image.x", case.id);
    assert_eq!(image_frame.y, grid_frame.y, "{}: image.y", case.id);

    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == grid.id())
        .unwrap_or_else(|| panic!("{}: missing track trace", case.id));
    let axis = match case.axis {
        Axis::Columns => LayoutGridTrackSizingAxis::Columns,
        Axis::Rows => LayoutGridTrackSizingAxis::Rows,
    };
    let pass = trace
        .passes
        .iter()
        .find(|pass| pass.axis == axis)
        .unwrap_or_else(|| panic!("{}: missing {axis:?} pass", case.id));
    let final_stage = pass
        .stages
        .last()
        .unwrap_or_else(|| panic!("{}: missing final {axis:?} stage", case.id));
    let tracks = final_stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .collect::<Vec<_>>();
    assert_eq!(tracks.len(), 1, "{}: expected one track", case.id);
    close(
        tracks[0].base_size,
        case.expected_track,
        case.id,
        "final track base size",
    );
    if case.expects_minimum_query {
        assert!(
            pass.contributions.iter().any(|contribution| {
                contribution.kind == LayoutGridTrackSizingContributionKind::Minimum
            }),
            "{}: missing minimum contribution query",
            case.id
        );
    }

    let overflow = (
        image_frame.x + image_frame.width > grid_frame.x + grid_frame.width,
        image_frame.y + image_frame.height > grid_frame.y + grid_frame.height,
    );
    assert_eq!(
        overflow, case.expected_overflow,
        "{}: overflow direction",
        case.id
    );
}

fn dimension(value: f32) -> crate::LayoutDimension {
    if value < 0.0 {
        crate::LayoutDimension::Auto
    } else {
        crate::LayoutDimension::Fixed(value)
    }
}

fn close(actual: f32, expected: f32, case: &str, property: &str) {
    assert!(
        (actual - expected).abs() <= 0.05,
        "{case}: {property} expected {expected}, got {actual}"
    );
}

#[test]
fn chrome155_image_transferred_size_suggestions_match_in_both_grid_axes() {
    let auto_fr = GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::Auto,
        max: GridTrackMaxSizingFunction::Fraction(1.0),
    };
    let zero_fr = GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::LengthPx(0.0),
        max: GridTrackMaxSizingFunction::Fraction(1.0),
    };
    let fixed_row = GridTrackSize::LengthPx(20.0);
    let fixed_column = GridTrackSize::LengthPx(20.0);
    let cases = [
        ImageRatioCase {
            id: "column-auto-natural-ratio",
            axis: Axis::Columns,
            columns: auto_fr,
            rows: fixed_row,
            grid: (8.0, 20.0),
            image_width: -1.0,
            image_height: 8.0,
            aspect_ratio: None,
            min_width: -1.0,
            max_width: -1.0,
            min_height: -1.0,
            max_height: -1.0,
            expects_minimum_query: true,
            expected_track: 12.0,
            expected_item: (12.0, 8.0),
            expected_overflow: (true, false),
        },
        ImageRatioCase {
            id: "column-zero-natural-ratio",
            axis: Axis::Columns,
            columns: zero_fr,
            rows: fixed_row,
            grid: (8.0, 20.0),
            image_width: -1.0,
            image_height: 8.0,
            aspect_ratio: None,
            min_width: -1.0,
            max_width: -1.0,
            min_height: -1.0,
            max_height: -1.0,
            expects_minimum_query: false,
            expected_track: 8.0,
            expected_item: (12.0, 8.0),
            expected_overflow: (true, false),
        },
        ImageRatioCase {
            id: "column-max-height-clamp",
            axis: Axis::Columns,
            columns: auto_fr,
            rows: fixed_row,
            grid: (4.0, 20.0),
            image_width: -1.0,
            image_height: 8.0,
            aspect_ratio: None,
            min_width: -1.0,
            max_width: -1.0,
            min_height: -1.0,
            max_height: 4.0,
            expects_minimum_query: true,
            expected_track: 6.0,
            expected_item: (6.0, 4.0),
            expected_overflow: (true, false),
        },
        ImageRatioCase {
            id: "column-min-height-clamp",
            axis: Axis::Columns,
            columns: auto_fr,
            rows: fixed_row,
            grid: (16.0, 20.0),
            image_width: -1.0,
            image_height: 8.0,
            aspect_ratio: None,
            min_width: -1.0,
            max_width: -1.0,
            min_height: 12.0,
            max_height: -1.0,
            expects_minimum_query: true,
            expected_track: 18.0,
            expected_item: (18.0, 12.0),
            expected_overflow: (true, false),
        },
        ImageRatioCase {
            id: "row-auto-natural-ratio",
            axis: Axis::Rows,
            columns: fixed_column,
            rows: auto_fr,
            grid: (20.0, 6.0),
            image_width: 12.0,
            image_height: -1.0,
            aspect_ratio: None,
            min_width: -1.0,
            max_width: -1.0,
            min_height: -1.0,
            max_height: -1.0,
            expects_minimum_query: true,
            expected_track: 8.0,
            expected_item: (12.0, 8.0),
            expected_overflow: (false, true),
        },
        ImageRatioCase {
            id: "row-zero-natural-ratio",
            axis: Axis::Rows,
            columns: fixed_column,
            rows: zero_fr,
            grid: (20.0, 6.0),
            image_width: 12.0,
            image_height: -1.0,
            aspect_ratio: None,
            min_width: -1.0,
            max_width: -1.0,
            min_height: -1.0,
            max_height: -1.0,
            expects_minimum_query: false,
            expected_track: 6.0,
            expected_item: (12.0, 8.0),
            expected_overflow: (false, true),
        },
        ImageRatioCase {
            id: "row-max-width-clamp",
            axis: Axis::Rows,
            columns: fixed_column,
            rows: auto_fr,
            grid: (20.0, 3.0),
            image_width: 12.0,
            image_height: -1.0,
            aspect_ratio: None,
            min_width: -1.0,
            max_width: 6.0,
            min_height: -1.0,
            max_height: -1.0,
            expects_minimum_query: true,
            expected_track: 4.0,
            expected_item: (6.0, 4.0),
            expected_overflow: (false, true),
        },
        ImageRatioCase {
            id: "row-min-width-clamp",
            axis: Axis::Rows,
            columns: fixed_column,
            rows: auto_fr,
            grid: (20.0, 9.0),
            image_width: 12.0,
            image_height: -1.0,
            aspect_ratio: None,
            min_width: 18.0,
            max_width: -1.0,
            min_height: -1.0,
            max_height: -1.0,
            expects_minimum_query: true,
            expected_track: 12.0,
            expected_item: (18.0, 12.0),
            expected_overflow: (false, true),
        },
        ImageRatioCase {
            id: "column-min-content-natural-ratio",
            axis: Axis::Columns,
            columns: GridTrackSize::MinContent,
            rows: fixed_row,
            grid: (8.0, 20.0),
            image_width: -1.0,
            image_height: 8.0,
            aspect_ratio: None,
            min_width: -1.0,
            max_width: -1.0,
            min_height: -1.0,
            max_height: -1.0,
            expects_minimum_query: false,
            expected_track: 12.0,
            expected_item: (12.0, 8.0),
            expected_overflow: (true, false),
        },
        ImageRatioCase {
            id: "row-min-content-natural-ratio",
            axis: Axis::Rows,
            columns: fixed_column,
            rows: GridTrackSize::MinContent,
            grid: (20.0, 6.0),
            image_width: 12.0,
            image_height: -1.0,
            aspect_ratio: None,
            min_width: -1.0,
            max_width: -1.0,
            min_height: -1.0,
            max_height: -1.0,
            expects_minimum_query: false,
            expected_track: 8.0,
            expected_item: (12.0, 8.0),
            expected_overflow: (false, true),
        },
        ImageRatioCase {
            id: "column-authored-ratio",
            axis: Axis::Columns,
            columns: auto_fr,
            rows: fixed_row,
            grid: (8.0, 20.0),
            image_width: -1.0,
            image_height: 8.0,
            aspect_ratio: Some(1.5),
            min_width: -1.0,
            max_width: -1.0,
            min_height: -1.0,
            max_height: -1.0,
            expects_minimum_query: true,
            expected_track: 12.0,
            expected_item: (12.0, 8.0),
            expected_overflow: (true, false),
        },
        ImageRatioCase {
            id: "column-authored-max-height-clamp",
            axis: Axis::Columns,
            columns: auto_fr,
            rows: fixed_row,
            grid: (4.0, 20.0),
            image_width: -1.0,
            image_height: 8.0,
            aspect_ratio: Some(1.5),
            min_width: -1.0,
            max_width: -1.0,
            min_height: -1.0,
            max_height: 4.0,
            expects_minimum_query: true,
            expected_track: 6.0,
            expected_item: (6.0, 4.0),
            expected_overflow: (true, false),
        },
        ImageRatioCase {
            id: "column-authored-min-height-clamp",
            axis: Axis::Columns,
            columns: auto_fr,
            rows: fixed_row,
            grid: (16.0, 20.0),
            image_width: -1.0,
            image_height: 8.0,
            aspect_ratio: Some(1.5),
            min_width: -1.0,
            max_width: -1.0,
            min_height: 12.0,
            max_height: -1.0,
            expects_minimum_query: true,
            expected_track: 18.0,
            expected_item: (18.0, 12.0),
            expected_overflow: (true, false),
        },
        ImageRatioCase {
            id: "column-specified-size-precedes-transfer",
            axis: Axis::Columns,
            columns: auto_fr,
            rows: fixed_row,
            grid: (8.0, 20.0),
            image_width: 10.0,
            image_height: 8.0,
            aspect_ratio: Some(1.5),
            min_width: -1.0,
            max_width: -1.0,
            min_height: -1.0,
            max_height: -1.0,
            expects_minimum_query: true,
            expected_track: 10.0,
            expected_item: (10.0, 8.0),
            expected_overflow: (true, false),
        },
        ImageRatioCase {
            id: "column-target-max-width-clamp",
            axis: Axis::Columns,
            columns: auto_fr,
            rows: fixed_row,
            grid: (8.0, 20.0),
            image_width: -1.0,
            image_height: 8.0,
            aspect_ratio: Some(1.5),
            min_width: -1.0,
            max_width: 9.0,
            min_height: -1.0,
            max_height: -1.0,
            expects_minimum_query: true,
            expected_track: 9.0,
            expected_item: (9.0, 8.0),
            expected_overflow: (true, false),
        },
        ImageRatioCase {
            id: "column-opposite-min-over-max",
            axis: Axis::Columns,
            columns: auto_fr,
            rows: fixed_row,
            grid: (8.0, 20.0),
            image_width: -1.0,
            image_height: 8.0,
            aspect_ratio: Some(1.5),
            min_width: -1.0,
            max_width: -1.0,
            min_height: 12.0,
            max_height: 4.0,
            expects_minimum_query: true,
            expected_track: 18.0,
            expected_item: (18.0, 12.0),
            expected_overflow: (true, false),
        },
        ImageRatioCase {
            id: "column-target-min-width-clamp",
            axis: Axis::Columns,
            columns: auto_fr,
            rows: fixed_row,
            grid: (8.0, 20.0),
            image_width: -1.0,
            image_height: 8.0,
            aspect_ratio: Some(1.5),
            min_width: 18.0,
            max_width: -1.0,
            min_height: -1.0,
            max_height: -1.0,
            expects_minimum_query: true,
            expected_track: 18.0,
            expected_item: (18.0, 8.0),
            expected_overflow: (true, false),
        },
        ImageRatioCase {
            id: "row-authored-ratio",
            axis: Axis::Rows,
            columns: fixed_column,
            rows: auto_fr,
            grid: (20.0, 6.0),
            image_width: 12.0,
            image_height: -1.0,
            aspect_ratio: Some(1.5),
            min_width: -1.0,
            max_width: -1.0,
            min_height: -1.0,
            max_height: -1.0,
            expects_minimum_query: true,
            expected_track: 8.0,
            expected_item: (12.0, 8.0),
            expected_overflow: (false, true),
        },
        ImageRatioCase {
            id: "row-authored-max-width-clamp",
            axis: Axis::Rows,
            columns: fixed_column,
            rows: auto_fr,
            grid: (20.0, 3.0),
            image_width: 12.0,
            image_height: -1.0,
            aspect_ratio: Some(1.5),
            min_width: -1.0,
            max_width: 6.0,
            min_height: -1.0,
            max_height: -1.0,
            expects_minimum_query: true,
            expected_track: 4.0,
            expected_item: (6.0, 4.0),
            expected_overflow: (false, true),
        },
        ImageRatioCase {
            id: "row-authored-min-width-clamp",
            axis: Axis::Rows,
            columns: fixed_column,
            rows: auto_fr,
            grid: (20.0, 9.0),
            image_width: 12.0,
            image_height: -1.0,
            aspect_ratio: Some(1.5),
            min_width: 18.0,
            max_width: -1.0,
            min_height: -1.0,
            max_height: -1.0,
            expects_minimum_query: true,
            expected_track: 12.0,
            expected_item: (18.0, 12.0),
            expected_overflow: (false, true),
        },
        ImageRatioCase {
            id: "row-specified-size-precedes-transfer",
            axis: Axis::Rows,
            columns: fixed_column,
            rows: auto_fr,
            grid: (20.0, 6.0),
            image_width: 12.0,
            image_height: 6.0,
            aspect_ratio: Some(1.5),
            min_width: -1.0,
            max_width: -1.0,
            min_height: -1.0,
            max_height: -1.0,
            expects_minimum_query: true,
            expected_track: 6.0,
            expected_item: (12.0, 6.0),
            expected_overflow: (false, false),
        },
        ImageRatioCase {
            id: "row-target-max-height-clamp",
            axis: Axis::Rows,
            columns: fixed_column,
            rows: auto_fr,
            grid: (20.0, 4.0),
            image_width: 12.0,
            image_height: -1.0,
            aspect_ratio: Some(1.5),
            min_width: -1.0,
            max_width: -1.0,
            min_height: -1.0,
            max_height: 6.0,
            expects_minimum_query: true,
            expected_track: 6.0,
            expected_item: (12.0, 6.0),
            expected_overflow: (false, true),
        },
        ImageRatioCase {
            id: "row-opposite-min-over-max",
            axis: Axis::Rows,
            columns: fixed_column,
            rows: auto_fr,
            grid: (20.0, 4.0),
            image_width: 12.0,
            image_height: -1.0,
            aspect_ratio: Some(1.5),
            min_width: 18.0,
            max_width: 6.0,
            min_height: -1.0,
            max_height: -1.0,
            expects_minimum_query: true,
            expected_track: 12.0,
            expected_item: (18.0, 12.0),
            expected_overflow: (false, true),
        },
    ];

    for (index, case) in cases.iter().enumerate() {
        run_case(case, 2_200 + index as u64);
    }

    let scrollable_overflow = ImageRatioCase {
        id: "column-authored-ratio-scrollable-overflow-min-zero",
        axis: Axis::Columns,
        columns: auto_fr,
        rows: fixed_row,
        grid: (8.0, 20.0),
        image_width: -1.0,
        image_height: 8.0,
        aspect_ratio: Some(1.5),
        min_width: -1.0,
        max_width: -1.0,
        min_height: -1.0,
        max_height: -1.0,
        expects_minimum_query: true,
        expected_track: 8.0,
        expected_item: (12.0, 8.0),
        expected_overflow: (true, false),
    };
    run_case_with_overflow(&scrollable_overflow, 2_240, crate::LayoutOverflow::Scroll);
}

#[test]
fn border_box_ratio_transfer_keeps_preferred_size_in_border_box_coordinates() {
    let case = ImageRatioCase {
        id: "column-border-box-transfer",
        axis: Axis::Columns,
        columns: GridTrackSize::MinMax {
            min: GridTrackMinSizingFunction::Auto,
            max: GridTrackMaxSizingFunction::Fraction(1.0),
        },
        rows: GridTrackSize::LengthPx(20.0),
        grid: (8.0, 20.0),
        image_width: -1.0,
        image_height: 8.0,
        aspect_ratio: Some(1.5),
        min_width: -1.0,
        max_width: -1.0,
        min_height: -1.0,
        max_height: -1.0,
        expects_minimum_query: true,
        expected_track: 12.0,
        expected_item: (12.0, 8.0),
        expected_overflow: (true, false),
    };
    let one_px = crate::LayoutLengthPercentage::length(1.0);
    run_case_with_box_decoration(
        &case,
        2_299,
        crate::LayoutOverflow::Visible,
        crate::LayoutBoxSizing::BorderBox,
        crate::LayoutEdges {
            top: one_px,
            right: one_px,
            bottom: one_px,
            left: one_px,
        },
        crate::LayoutBorder {
            top: 1.0,
            right: 1.0,
            bottom: 1.0,
            left: 1.0,
        },
    );
}

#[test]
fn content_box_ratio_transfer_adds_padding_and_border_after_conversion() {
    let case = ImageRatioCase {
        id: "column-content-box-transfer",
        axis: Axis::Columns,
        columns: GridTrackSize::MinMax {
            min: GridTrackMinSizingFunction::Auto,
            max: GridTrackMaxSizingFunction::Fraction(1.0),
        },
        rows: GridTrackSize::LengthPx(20.0),
        grid: (8.0, 20.0),
        image_width: -1.0,
        image_height: 8.0,
        aspect_ratio: Some(1.5),
        min_width: -1.0,
        max_width: -1.0,
        min_height: -1.0,
        max_height: -1.0,
        expects_minimum_query: true,
        expected_track: 16.0,
        expected_item: (16.0, 12.0),
        expected_overflow: (true, false),
    };
    let one_px = crate::LayoutLengthPercentage::length(1.0);
    run_case_with_box_decoration(
        &case,
        2_300,
        crate::LayoutOverflow::Visible,
        crate::LayoutBoxSizing::ContentBox,
        crate::LayoutEdges {
            top: one_px,
            right: one_px,
            bottom: one_px,
            left: one_px,
        },
        crate::LayoutBorder {
            top: 1.0,
            right: 1.0,
            bottom: 1.0,
            left: 1.0,
        },
    );
}

#[test]
fn authored_ratio_constraints_are_rejected_without_a_track_sizing_grid() {
    let (document, grid, image) = create_document(2_300);
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
    let input = LayoutInput::from_host_document(
        &snapshot,
        grid,
        Viewport {
            width: 40.0,
            height: 20.0,
        },
        &BTreeMap::from([
            (
                grid.id(),
                LayoutStyle {
                    display: crate::LayoutDisplay::Block,
                    width: crate::LayoutDimension::Fixed(40.0),
                    height: crate::LayoutDimension::Fixed(20.0),
                    ..LayoutStyle::default()
                },
            ),
            (
                image.id(),
                LayoutStyle {
                    display: crate::LayoutDisplay::Block,
                    aspect_ratio: Some(1.5),
                    min_height: crate::LayoutDimension::Fixed(12.0),
                    ..LayoutStyle::default()
                },
            ),
        ]),
        StyleRevision::default(),
        EnvironmentRevision::default(),
    )
    .unwrap();
    let result = TaffyLayoutEngine.compute_with_intrinsic_size_snapshot(&input, &intrinsic);
    assert!(
        matches!(
            &result,
            Err(crate::LayoutError::UnsupportedIntrinsicSizing {
                node,
                reason: "aspect-ratio와 authored min/max 조합은 지원하지 않습니다",
            }) if *node == image.id()
        ),
        "unexpected result outside track sizing: {result:?}"
    );
}

#[test]
fn non_replaced_items_keep_their_min_content_suggestion_instead_of_transfer() {
    let (document, grid, image) = create_document(2_500);
    let snapshot = document.snapshot();
    let auto_fr = GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::Auto,
        max: GridTrackMaxSizingFunction::Fraction(1.0),
    };
    let input = LayoutInput::from_host_document(
        &snapshot,
        grid,
        Viewport {
            width: 8.0,
            height: 20.0,
        },
        &BTreeMap::from([
            (
                grid.id(),
                LayoutStyle {
                    display: crate::LayoutDisplay::Grid,
                    width: crate::LayoutDimension::Fixed(8.0),
                    height: crate::LayoutDimension::Fixed(20.0),
                    align_items: LayoutAlignItems::FlexStart,
                    ..LayoutStyle::default()
                },
            ),
            (
                image.id(),
                LayoutStyle {
                    display: crate::LayoutDisplay::Block,
                    width: crate::LayoutDimension::Auto,
                    height: crate::LayoutDimension::Fixed(8.0),
                    aspect_ratio: Some(1.5),
                    ..LayoutStyle::default()
                },
            ),
        ]),
        StyleRevision::default(),
        EnvironmentRevision::default(),
    )
    .unwrap()
    .with_grid_layout(
        BTreeMap::from([(
            grid.id(),
            GridContainerInput {
                columns: vec![auto_fr],
                rows: vec![GridTrackSize::LengthPx(20.0)],
                placement_mode: GridPlacementMode::TrackSizing,
                ..GridContainerInput::default()
            },
        )]),
        BTreeMap::new(),
    );
    let output = TaffyLayoutEngine.compute(&input).unwrap();
    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == grid.id())
        .unwrap();
    let columns = trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .unwrap();
    let final_stage = columns.stages.last().unwrap();
    let track = final_stage
        .tracks
        .iter()
        .find(|track| !track.is_gutter && !track.is_collapsed)
        .unwrap();

    assert_eq!(track.base_size, 12.0);
    assert_eq!(output.frames[&image.id()].width, 12.0);
    assert!(matches!(
        columns
            .contributions
            .iter()
            .find(|contribution| {
                contribution.kind == LayoutGridTrackSizingContributionKind::Minimum
            })
            .and_then(|contribution| contribution.minimum_source),
        Some(
            crate::LayoutGridTrackSizingMinimumContributionSource::AutomaticMinimum {
                content_based: true,
                is_compressible_replaced: false,
                ..
            }
        )
    ));
}
