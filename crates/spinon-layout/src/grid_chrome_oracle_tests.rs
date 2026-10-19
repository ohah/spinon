use std::collections::BTreeMap;

use spinon_core::{
    DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument, HostNodeHandle,
    HostParent, IntrinsicSizeRevision, OwnerId, StyleRevision,
};

use crate::grid_placement::{GridItemInput, GridPlacementInput};
use crate::{
    GridAutoRepeatMode, GridContainerInput, GridPlacementMode, GridTrackLengthPercentage,
    GridTrackMaxSizingFunction, GridTrackMinSizingFunction, GridTrackSize, LayoutAlignContent,
    LayoutDimension, LayoutDisplay, LayoutEngine, LayoutGap, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingStageKind, LayoutInput, LayoutIntrinsicSize, LayoutIntrinsicSizeSnapshot,
    LayoutJustifyContent, LayoutSourceRevision, LayoutStyle, TaffyLayoutEngine, Viewport,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const NATURAL_IMAGE: LayoutIntrinsicSize = LayoutIntrinsicSize::new(24.0, 16.0);

struct ChromeImageCase {
    id: &'static str,
    columns: Vec<GridTrackSize>,
    rows: Vec<GridTrackSize>,
    grid: (f32, f32),
    expected_columns: Vec<f32>,
    expected_rows: Vec<f32>,
    expected_item: (f32, f32, f32, f32),
}

struct ChromeSpanCase {
    id: &'static str,
    item_kind: &'static str,
    columns: Vec<GridTrackSize>,
    rows: Vec<GridTrackSize>,
    grid: (f32, f32),
    column_gap: f32,
    placement: GridItemInput,
    item_style: LayoutStyle,
    has_natural_image: bool,
    expected_columns: Vec<f32>,
    expected_rows: Vec<f32>,
    expected_item: (f32, f32, f32, f32),
}

fn document_with_item(
    owner: u64,
    item_kind: &str,
) -> (HostDocument, HostNodeHandle, HostNodeHandle) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let image = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(owner).unwrap(), document.document_revision());
    batch
        .push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML.to_owned(),
            local_name: "main".to_owned(),
        })
        .push(DocumentOperation::CreateElement {
            node: image,
            namespace: HTML.to_owned(),
            local_name: item_kind.to_owned(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: image,
            before: None,
        });
    document.commit(batch).unwrap();
    (document, root, image)
}

fn image_document(owner: u64) -> (HostDocument, HostNodeHandle, HostNodeHandle) {
    document_with_item(owner, "img")
}

fn intrinsic_snapshot(
    document: &HostDocument,
    image: HostNodeHandle,
) -> LayoutIntrinsicSizeSnapshot {
    let snapshot = document.snapshot();
    let source = LayoutSourceRevision::HostDocument {
        generation: snapshot.generation(),
        document: snapshot.document_revision(),
        render_tree: snapshot.render_tree_revision(),
    };
    LayoutIntrinsicSizeSnapshot::new(
        source,
        IntrinsicSizeRevision::INITIAL.checked_next().unwrap(),
        BTreeMap::from([(image.id(), NATURAL_IMAGE)]),
    )
    .unwrap()
}

fn run_case(case: &ChromeImageCase, owner: u64) {
    let (document, root, image) = image_document(owner);
    let snapshot = intrinsic_snapshot(&document, image);
    let root_style = LayoutStyle {
        display: LayoutDisplay::Grid,
        width: LayoutDimension::Fixed(case.grid.0),
        height: LayoutDimension::Fixed(case.grid.1),
        align_content: Some(LayoutAlignContent::FlexStart),
        justify_content: LayoutJustifyContent::FlexStart,
        ..LayoutStyle::default()
    };
    let image_style = LayoutStyle {
        display: LayoutDisplay::Block,
        ..LayoutStyle::default()
    };
    let input = LayoutInput::from_host_document(
        &document.snapshot(),
        root,
        Viewport {
            width: case.grid.0,
            height: case.grid.1,
        },
        &BTreeMap::from([(root.id(), root_style), (image.id(), image_style)]),
        StyleRevision::default(),
        EnvironmentRevision::default(),
    )
    .unwrap()
    .with_grid_layout(
        BTreeMap::from([(
            root.id(),
            GridContainerInput {
                columns: case.columns.clone(),
                rows: case.rows.clone(),
                placement_mode: GridPlacementMode::TrackSizing,
                ..GridContainerInput::default()
            },
        )]),
        BTreeMap::new(),
    );
    let output = TaffyLayoutEngine
        .compute_with_intrinsic_size_snapshot(&input, &snapshot)
        .unwrap_or_else(|error| panic!("{}: {error:?}", case.id));

    let grid = output.frames[&root.id()];
    let item = output.frames[&image.id()];
    assert_close(grid.width, case.grid.0, case.id, "grid.width");
    assert_close(grid.height, case.grid.1, case.id, "grid.height");
    for (actual, expected, property) in [
        (item.x, case.expected_item.0, "item.x"),
        (item.y, case.expected_item.1, "item.y"),
        (item.width, case.expected_item.2, "item.width"),
        (item.height, case.expected_item.3, "item.height"),
    ] {
        assert_close(actual, expected, case.id, property);
    }

    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == root.id())
        .unwrap_or_else(|| panic!("{}: missing Grid track trace", case.id));
    assert_axis_tracks(
        trace,
        LayoutGridTrackSizingAxis::Columns,
        &case.expected_columns,
        case.id,
    );
    assert_axis_tracks(
        trace,
        LayoutGridTrackSizingAxis::Rows,
        &case.expected_rows,
        case.id,
    );
}

fn assert_axis_tracks(
    trace: &crate::LayoutGridTrackSizingTrace,
    axis: LayoutGridTrackSizingAxis,
    expected: &[f32],
    case: &str,
) {
    let pass = trace
        .passes
        .iter()
        .find(|pass| pass.axis == axis)
        .unwrap_or_else(|| panic!("{case}: missing {axis:?} sizing pass"));
    assert!(
        pass.stages.iter().any(|stage| {
            matches!(
                stage.stage,
                LayoutGridTrackSizingStageKind::IntrinsicSizingComplete
                    | LayoutGridTrackSizingStageKind::FixedTrackEarlyReturn
            )
        }),
        "{case}: missing {axis:?} sizing completion stage"
    );
    let stage = pass
        .stages
        .last()
        .unwrap_or_else(|| panic!("{case}: missing final {axis:?} stage"));
    let actual = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(actual.len(), expected.len(), "{case}: {axis:?} track count");
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert_close(*actual, *expected, case, &format!("{axis:?}[{index}]"));
    }
}

fn assert_close(actual: f32, expected: f32, case: &str, property: &str) {
    assert!(
        (actual - expected).abs() <= 0.05,
        "{case}: {property} expected {expected}, got {actual}"
    );
}

fn run_span_case(case: &ChromeSpanCase, owner: u64) {
    let (document, root, item) = document_with_item(owner, case.item_kind);
    let root_style = LayoutStyle {
        display: LayoutDisplay::Grid,
        width: LayoutDimension::Fixed(case.grid.0),
        height: LayoutDimension::Fixed(case.grid.1),
        gap: LayoutGap {
            column: crate::LayoutLengthPercentage::length(case.column_gap),
            ..LayoutGap::default()
        },
        align_content: Some(LayoutAlignContent::FlexStart),
        justify_content: LayoutJustifyContent::FlexStart,
        ..LayoutStyle::default()
    };
    let input = LayoutInput::from_host_document(
        &document.snapshot(),
        root,
        Viewport {
            width: case.grid.0,
            height: case.grid.1,
        },
        &BTreeMap::from([(root.id(), root_style), (item.id(), case.item_style)]),
        StyleRevision::default(),
        EnvironmentRevision::default(),
    )
    .unwrap()
    .with_grid_layout(
        BTreeMap::from([(
            root.id(),
            GridContainerInput {
                columns: case.columns.clone(),
                rows: case.rows.clone(),
                placement_mode: GridPlacementMode::TrackSizing,
                ..GridContainerInput::default()
            },
        )]),
        BTreeMap::from([(item.id(), case.placement)]),
    );
    let output = if case.has_natural_image {
        let snapshot = intrinsic_snapshot(&document, item);
        TaffyLayoutEngine
            .compute_with_intrinsic_size_snapshot(&input, &snapshot)
            .unwrap_or_else(|error| panic!("{}: {error:?}", case.id))
    } else {
        TaffyLayoutEngine
            .compute(&input)
            .unwrap_or_else(|error| panic!("{}: {error:?}", case.id))
    };

    let grid = output.frames[&root.id()];
    let item_frame = output.frames[&item.id()];
    assert_close(grid.width, case.grid.0, case.id, "grid.width");
    assert_close(grid.height, case.grid.1, case.id, "grid.height");
    for (actual, expected, property) in [
        (item_frame.x, case.expected_item.0, "item.x"),
        (item_frame.y, case.expected_item.1, "item.y"),
        (item_frame.width, case.expected_item.2, "item.width"),
        (item_frame.height, case.expected_item.3, "item.height"),
    ] {
        assert_close(actual, expected, case.id, property);
    }

    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == root.id())
        .unwrap_or_else(|| panic!("{}: missing Grid track trace", case.id));
    assert_axis_tracks(
        trace,
        LayoutGridTrackSizingAxis::Columns,
        &case.expected_columns,
        case.id,
    );
    assert_axis_tracks(
        trace,
        LayoutGridTrackSizingAxis::Rows,
        &case.expected_rows,
        case.id,
    );
}

#[test]
fn chrome155_spanning_track_oracle_cases_match_traces_and_frames() {
    let cases = [
        ChromeSpanCase {
            id: "spanning-minmax-items",
            item_kind: "div",
            columns: vec![
                GridTrackSize::MinMax {
                    min: GridTrackMinSizingFunction::LengthPx(0.0),
                    max: GridTrackMaxSizingFunction::Auto,
                },
                GridTrackSize::MinMax {
                    min: GridTrackMinSizingFunction::LengthPx(0.0),
                    max: GridTrackMaxSizingFunction::Auto,
                },
            ],
            rows: vec![GridTrackSize::Auto],
            grid: (100.0, 30.0),
            column_gap: 4.0,
            placement: GridItemInput {
                row_start: GridPlacementInput::Line(1),
                row_end: GridPlacementInput::Line(2),
                column_start: GridPlacementInput::Line(1),
                column_end: GridPlacementInput::Line(3),
            },
            item_style: LayoutStyle {
                display: LayoutDisplay::Block,
                width: LayoutDimension::Fixed(70.0),
                height: LayoutDimension::Fixed(10.0),
                ..LayoutStyle::default()
            },
            has_natural_image: false,
            expected_columns: vec![33.0, 33.0],
            expected_rows: vec![10.0],
            expected_item: (0.0, 0.0, 70.0, 10.0),
        },
        ChromeSpanCase {
            id: "spanning-min-content-items",
            item_kind: "img",
            columns: vec![GridTrackSize::MinContent, GridTrackSize::MinContent],
            rows: vec![GridTrackSize::Auto],
            grid: (100.0, 30.0),
            column_gap: 4.0,
            placement: GridItemInput {
                row_start: GridPlacementInput::Line(1),
                row_end: GridPlacementInput::Line(2),
                column_start: GridPlacementInput::Line(1),
                column_end: GridPlacementInput::Line(3),
            },
            item_style: LayoutStyle {
                display: LayoutDisplay::Block,
                ..LayoutStyle::default()
            },
            has_natural_image: true,
            expected_columns: vec![10.0, 10.0],
            expected_rows: vec![16.0],
            expected_item: (0.0, 0.0, 24.0, 16.0),
        },
    ];

    for (index, case) in cases.iter().enumerate() {
        run_span_case(case, 150 + index as u64);
    }
}

#[test]
fn chrome155_image_track_oracle_cases_match_traces_and_frames() {
    // Expected values come from spec/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/chrome/chrome155-observation.json.
    let cases = [
        ChromeImageCase {
            id: "auto-image-definite",
            columns: vec![GridTrackSize::Auto],
            rows: vec![GridTrackSize::Auto],
            grid: (100.0, 40.0),
            expected_columns: vec![24.0],
            expected_rows: vec![16.0],
            expected_item: (0.0, 0.0, 24.0, 16.0),
        },
        ChromeImageCase {
            id: "minmax-auto-image",
            columns: vec![GridTrackSize::MinMax {
                min: GridTrackMinSizingFunction::LengthPx(0.0),
                max: GridTrackMaxSizingFunction::Auto,
            }],
            rows: vec![GridTrackSize::Auto],
            grid: (100.0, 40.0),
            expected_columns: vec![24.0],
            expected_rows: vec![16.0],
            expected_item: (0.0, 0.0, 24.0, 16.0),
        },
        ChromeImageCase {
            id: "minmax-min-max-image",
            columns: vec![GridTrackSize::MinMax {
                min: GridTrackMinSizingFunction::MinContent,
                max: GridTrackMaxSizingFunction::MaxContent,
            }],
            rows: vec![GridTrackSize::Auto],
            grid: (100.0, 40.0),
            expected_columns: vec![24.0],
            expected_rows: vec![16.0],
            expected_item: (0.0, 0.0, 24.0, 16.0),
        },
        ChromeImageCase {
            id: "fit-content-below-minimum",
            columns: vec![GridTrackSize::FitContent(
                GridTrackLengthPercentage::LengthPx(12.0),
            )],
            rows: vec![GridTrackSize::Auto],
            grid: (100.0, 40.0),
            expected_columns: vec![24.0],
            expected_rows: vec![16.0],
            expected_item: (0.0, 0.0, 24.0, 16.0),
        },
        ChromeImageCase {
            id: "fit-content-above-intrinsic",
            columns: vec![GridTrackSize::FitContent(
                GridTrackLengthPercentage::LengthPx(40.0),
            )],
            rows: vec![GridTrackSize::Auto],
            grid: (100.0, 40.0),
            expected_columns: vec![24.0],
            expected_rows: vec![16.0],
            expected_item: (0.0, 0.0, 24.0, 16.0),
        },
        ChromeImageCase {
            id: "fit-content-percent",
            columns: vec![GridTrackSize::FitContent(
                GridTrackLengthPercentage::Percentage(0.5),
            )],
            rows: vec![GridTrackSize::Auto],
            grid: (100.0, 40.0),
            expected_columns: vec![24.0],
            expected_rows: vec![16.0],
            expected_item: (0.0, 0.0, 24.0, 16.0),
        },
        ChromeImageCase {
            id: "fixed-small-track-overflow",
            columns: vec![GridTrackSize::MinMax {
                min: GridTrackMinSizingFunction::LengthPx(0.0),
                max: GridTrackMaxSizingFunction::LengthPx(12.0),
            }],
            rows: vec![GridTrackSize::Auto],
            grid: (12.0, 40.0),
            expected_columns: vec![12.0],
            expected_rows: vec![16.0],
            expected_item: (0.0, 0.0, 24.0, 16.0),
        },
        ChromeImageCase {
            id: "min-content-row-image",
            columns: vec![GridTrackSize::LengthPx(100.0)],
            rows: vec![GridTrackSize::MinContent],
            grid: (100.0, 80.0),
            expected_columns: vec![100.0],
            expected_rows: vec![16.0],
            expected_item: (0.0, 0.0, 24.0, 16.0),
        },
    ];

    for (index, case) in cases.iter().enumerate() {
        run_case(case, 1_400 + index as u64);
    }
}

#[test]
fn chrome155_indefinite_percentage_track_matches_max_content_grid_and_image_overflow() {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let grid = document.reserve_node_handle().unwrap();
    let image = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(158).unwrap(), document.document_revision());
    batch
        .push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML.to_owned(),
            local_name: "main".to_owned(),
        })
        .push(DocumentOperation::CreateElement {
            node: grid,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        })
        .push(DocumentOperation::CreateElement {
            node: image,
            namespace: HTML.to_owned(),
            local_name: "img".to_owned(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: grid,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(grid),
            node: image,
            before: None,
        });
    document.commit(batch).unwrap();
    let snapshot = intrinsic_snapshot(&document, image);
    let root_style = LayoutStyle {
        width: crate::LayoutDimension::Fixed(100.0),
        height: crate::LayoutDimension::Fixed(100.0),
        ..LayoutStyle::default()
    };
    let grid_style = LayoutStyle {
        display: LayoutDisplay::Grid,
        width: crate::LayoutDimension::MaxContent,
        // The adapter requires a definite grid block axis; keep the measured Chrome height.
        height: crate::LayoutDimension::Fixed(16.0),
        align_content: Some(LayoutAlignContent::FlexStart),
        justify_content: LayoutJustifyContent::FlexStart,
        ..LayoutStyle::default()
    };
    let image_style = LayoutStyle {
        display: LayoutDisplay::Block,
        ..LayoutStyle::default()
    };
    let input = LayoutInput::from_host_document(
        &document.snapshot(),
        root,
        Viewport {
            width: 100.0,
            height: 100.0,
        },
        &BTreeMap::from([
            (root.id(), root_style),
            (grid.id(), grid_style),
            (image.id(), image_style),
        ]),
        StyleRevision::default(),
        EnvironmentRevision::default(),
    )
    .unwrap()
    .with_grid_layout(
        BTreeMap::from([(
            grid.id(),
            GridContainerInput {
                columns: vec![GridTrackSize::Percentage(0.5)],
                rows: vec![GridTrackSize::Auto],
                placement_mode: GridPlacementMode::TrackSizing,
                ..GridContainerInput::default()
            },
        )]),
        BTreeMap::from([(
            image.id(),
            GridItemInput {
                row_start: GridPlacementInput::Line(1),
                row_end: GridPlacementInput::Line(2),
                column_start: GridPlacementInput::Line(1),
                column_end: GridPlacementInput::Line(2),
            },
        )]),
    );
    let output = TaffyLayoutEngine
        .compute_with_intrinsic_size_snapshot(&input, &snapshot)
        .expect("the indefinite percentage Grid should lay out");

    let grid_frame = output.frames[&grid.id()];
    let item = output.frames[&image.id()];
    assert_close(
        grid_frame.width,
        24.0,
        "indefinite-percent-track",
        "grid.width",
    );
    assert_close(
        grid_frame.height,
        16.0,
        "indefinite-percent-track",
        "grid.height",
    );
    assert_close(item.x, 0.0, "indefinite-percent-track", "item.x");
    assert_close(item.y, 0.0, "indefinite-percent-track", "item.y");
    assert_close(item.width, 24.0, "indefinite-percent-track", "item.width");
    assert_close(item.height, 16.0, "indefinite-percent-track", "item.height");

    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == grid.id())
        .expect("the Grid should retain the track sizing trace");
    assert_axis_tracks(
        trace,
        LayoutGridTrackSizingAxis::Columns,
        &[12.0],
        "indefinite-percent-track",
    );
    assert_axis_tracks(
        trace,
        LayoutGridTrackSizingAxis::Rows,
        &[16.0],
        "indefinite-percent-track",
    );
}

struct ChromeAutoRepeatCase {
    id: &'static str,
    mode: GridAutoRepeatMode,
    track: GridTrackSize,
    row_track: GridTrackSize,
    grid_width: LayoutDimension,
    grid_height: LayoutDimension,
    column_gap: f32,
    expected_grid: (f32, f32),
    expected_columns: &'static [f32],
    expected_collapsed: &'static [bool],
    expected_item: Option<(f32, f32)>,
}

#[test]
fn chrome155_auto_repeat_track_oracle_cases_match_traces_and_frames() {
    let cases = [
        ChromeAutoRepeatCase {
            id: "auto-fill-definite",
            mode: GridAutoRepeatMode::AutoFill,
            track: GridTrackSize::MinMax {
                min: GridTrackMinSizingFunction::LengthPx(24.0),
                max: GridTrackMaxSizingFunction::Fraction(1.0),
            },
            row_track: GridTrackSize::LengthPx(20.0),
            grid_width: LayoutDimension::Fixed(100.0),
            grid_height: LayoutDimension::Fixed(30.0),
            column_gap: 4.0,
            expected_grid: (100.0, 30.0),
            expected_columns: &[30.65625, 30.671875, 30.671875],
            expected_collapsed: &[false, false, false],
            expected_item: Some((30.65625, 20.0)),
        },
        ChromeAutoRepeatCase {
            id: "auto-fit-definite",
            mode: GridAutoRepeatMode::AutoFit,
            track: GridTrackSize::MinMax {
                min: GridTrackMinSizingFunction::LengthPx(24.0),
                max: GridTrackMaxSizingFunction::Fraction(1.0),
            },
            row_track: GridTrackSize::LengthPx(20.0),
            grid_width: LayoutDimension::Fixed(100.0),
            grid_height: LayoutDimension::Fixed(30.0),
            column_gap: 4.0,
            expected_grid: (100.0, 30.0),
            expected_columns: &[100.0, 0.0, 0.0],
            expected_collapsed: &[false, true, true],
            expected_item: Some((100.0, 20.0)),
        },
        ChromeAutoRepeatCase {
            id: "auto-fit-all-empty",
            mode: GridAutoRepeatMode::AutoFit,
            track: GridTrackSize::MinMax {
                min: GridTrackMinSizingFunction::LengthPx(24.0),
                max: GridTrackMaxSizingFunction::Fraction(1.0),
            },
            row_track: GridTrackSize::LengthPx(20.0),
            grid_width: LayoutDimension::Fixed(100.0),
            grid_height: LayoutDimension::Fixed(30.0),
            column_gap: 4.0,
            expected_grid: (100.0, 30.0),
            expected_columns: &[0.0, 0.0, 0.0],
            expected_collapsed: &[true, true, true],
            expected_item: None,
        },
        ChromeAutoRepeatCase {
            id: "auto-fill-indefinite",
            mode: GridAutoRepeatMode::AutoFill,
            track: GridTrackSize::LengthPx(24.0),
            row_track: GridTrackSize::Auto,
            grid_width: LayoutDimension::MaxContent,
            grid_height: LayoutDimension::Fixed(0.0),
            column_gap: 0.0,
            expected_grid: (24.0, 0.0),
            expected_columns: &[24.0],
            expected_collapsed: &[false],
            expected_item: None,
        },
    ];
    for (index, case) in cases.iter().enumerate() {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let grid = document.reserve_node_handle().unwrap();
        let item = case
            .expected_item
            .map(|_| document.reserve_node_handle().unwrap());
        let mut batch = DocumentChangeBatch::new(
            OwnerId::new(160 + index as u64).unwrap(),
            document.document_revision(),
        );
        batch
            .push(DocumentOperation::CreateElement {
                node: root,
                namespace: HTML.to_owned(),
                local_name: "main".to_owned(),
            })
            .push(DocumentOperation::CreateElement {
                node: grid,
                namespace: HTML.to_owned(),
                local_name: "div".to_owned(),
            })
            .push(DocumentOperation::InsertBefore {
                parent: HostParent::Root,
                node: root,
                before: None,
            })
            .push(DocumentOperation::InsertBefore {
                parent: HostParent::Node(root),
                node: grid,
                before: None,
            });
        if let Some(item) = item {
            batch
                .push(DocumentOperation::CreateElement {
                    node: item,
                    namespace: HTML.to_owned(),
                    local_name: "div".to_owned(),
                })
                .push(DocumentOperation::InsertBefore {
                    parent: HostParent::Node(grid),
                    node: item,
                    before: None,
                });
        }
        document.commit(batch).unwrap();

        let root_style = LayoutStyle {
            display: LayoutDisplay::Block,
            width: LayoutDimension::Fixed(100.0),
            height: LayoutDimension::Fixed(100.0),
            ..LayoutStyle::default()
        };
        let grid_style = LayoutStyle {
            display: LayoutDisplay::Grid,
            width: case.grid_width,
            height: case.grid_height,
            gap: LayoutGap {
                column: crate::LayoutLengthPercentage::length(case.column_gap),
                ..LayoutGap::default()
            },
            align_content: Some(LayoutAlignContent::FlexStart),
            justify_content: LayoutJustifyContent::FlexStart,
            ..LayoutStyle::default()
        };
        let mut styles = BTreeMap::from([(root.id(), root_style), (grid.id(), grid_style)]);
        if let Some(item) = item {
            styles.insert(
                item.id(),
                LayoutStyle {
                    display: LayoutDisplay::Block,
                    ..LayoutStyle::default()
                },
            );
        }
        let layout_input = LayoutInput::from_host_document(
            &document.snapshot(),
            root,
            Viewport {
                width: 100.0,
                height: 100.0,
            },
            &styles,
            StyleRevision::default(),
            EnvironmentRevision::default(),
        )
        .unwrap()
        .with_grid_layout(
            BTreeMap::from([(
                grid.id(),
                GridContainerInput {
                    columns: vec![case.track],
                    rows: vec![case.row_track],
                    columns_auto_repeat: Some(case.mode),
                    placement_mode: GridPlacementMode::TrackSizing,
                    ..GridContainerInput::default()
                },
            )]),
            item.map(|item| {
                BTreeMap::from([(
                    item.id(),
                    GridItemInput {
                        row_start: GridPlacementInput::Line(1),
                        row_end: GridPlacementInput::Line(2),
                        column_start: GridPlacementInput::Line(1),
                        column_end: GridPlacementInput::Line(2),
                    },
                )])
            })
            .unwrap_or_default(),
        );
        let output = TaffyLayoutEngine
            .compute(&layout_input)
            .unwrap_or_else(|error| panic!("{}: {error:?}", case.id));
        let grid_frame = output.frames[&grid.id()];
        assert_close(
            grid_frame.width,
            case.expected_grid.0,
            case.id,
            "grid.width",
        );
        assert_close(
            grid_frame.height,
            case.expected_grid.1,
            case.id,
            "grid.height",
        );
        if let (Some(item), Some((width, height))) = (item, case.expected_item) {
            let item_frame = output.frames[&item.id()];
            assert_close(item_frame.x, 0.0, case.id, "item.x");
            assert_close(item_frame.y, 0.0, case.id, "item.y");
            assert_close(item_frame.width, width, case.id, "item.width");
            assert_close(item_frame.height, height, case.id, "item.height");
        } else {
            assert!(item.is_none());
        }

        let trace = output
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == grid.id())
            .unwrap_or_else(|| panic!("{}: missing track trace", case.id));
        let pass = trace
            .passes
            .iter()
            .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
            .unwrap_or_else(|| panic!("{}: missing columns pass", case.id));
        let final_tracks = &pass.stages.last().expect("final column stage").tracks;
        let columns = final_tracks
            .iter()
            .filter(|track| !track.is_gutter)
            .collect::<Vec<_>>();
        assert_eq!(columns.len(), case.expected_columns.len(), "{}", case.id);
        for (index, (track, expected)) in columns.iter().zip(case.expected_columns).enumerate() {
            assert_close(
                track.base_size,
                *expected,
                case.id,
                &format!("columns[{index}]"),
            );
            assert_eq!(
                track.is_collapsed, case.expected_collapsed[index],
                "{}",
                case.id
            );
        }
    }
}

#[test]
fn chrome155_auto_fill_indefinite_grid_repeats_one_fixed_track() {
    let mut document = HostDocument::new().unwrap();
    let grid = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(164).unwrap(), document.document_revision());
    batch
        .push(DocumentOperation::CreateElement {
            node: grid,
            namespace: HTML.to_owned(),
            local_name: "main".to_owned(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: grid,
            before: None,
        });
    document.commit(batch).unwrap();

    let mut input = LayoutInput::from_host_document(
        &document.snapshot(),
        grid,
        Viewport {
            width: 100.0,
            height: 100.0,
        },
        &BTreeMap::from([(
            grid.id(),
            LayoutStyle {
                display: LayoutDisplay::Grid,
                width: LayoutDimension::MaxContent,
                height: LayoutDimension::Fixed(0.0),
                ..LayoutStyle::default()
            },
        )]),
        StyleRevision::default(),
        EnvironmentRevision::default(),
    )
    .unwrap();
    input.root_sizing = crate::RootSizingPolicy::ResolveWithin;
    let input = input.with_grid_layout(
        BTreeMap::from([(
            grid.id(),
            GridContainerInput {
                columns: vec![GridTrackSize::LengthPx(24.0)],
                rows: vec![GridTrackSize::Auto],
                columns_auto_repeat: Some(GridAutoRepeatMode::AutoFill),
                placement_mode: GridPlacementMode::TrackSizing,
                ..GridContainerInput::default()
            },
        )]),
        BTreeMap::new(),
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    assert_close(
        output.frames[&grid.id()].width,
        24.0,
        "auto-fill-indefinite",
        "grid.width",
    );
    assert_close(
        output.frames[&grid.id()].height,
        0.0,
        "auto-fill-indefinite",
        "grid.height",
    );
}
