use std::collections::BTreeMap;

use spinon_core::{
    DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};

use crate::grid_placement::{GridItemInput, GridPlacementInput};
use crate::{
    GridContainerInput, GridPlacementMode, GridTrackMaxSizingFunction, GridTrackMinSizingFunction,
    GridTrackSize, LayoutAlignContent, LayoutDimension, LayoutDisplay, LayoutEngine,
    LayoutGridTrackSizingAxis, LayoutGridTrackSizingStageKind, LayoutInput, LayoutJustifyContent,
    LayoutStyle, TaffyLayoutEngine, Viewport,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";

#[derive(Clone, Copy)]
struct MultiSpanItem {
    width: f32,
    column_start: i32,
    column_end: i32,
}

struct ChromeMultiSpanCase {
    id: &'static str,
    items_in_source_order: &'static [MultiSpanItem],
    expected_columns: &'static [f32],
    expected_span_batches: &'static [u16],
}

fn multi_span_document(
    owner: u64,
    item_count: usize,
) -> (HostDocument, HostNodeHandle, Vec<HostNodeHandle>) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let items = (0..item_count)
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(owner).unwrap(), document.document_revision());
    batch
        .push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML.to_owned(),
            local_name: "main".to_owned(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        });
    for item in &items {
        batch
            .push(DocumentOperation::CreateElement {
                node: *item,
                namespace: HTML.to_owned(),
                local_name: "div".to_owned(),
            })
            .push(DocumentOperation::InsertBefore {
                parent: HostParent::Node(root),
                node: *item,
                before: None,
            });
    }
    document.commit(batch).unwrap();
    (document, root, items)
}

fn run_multi_span_case(case: &ChromeMultiSpanCase, owner: u64) {
    let (document, root, items) = multi_span_document(owner, case.items_in_source_order.len());
    let mut styles = BTreeMap::from([(
        root.id(),
        LayoutStyle {
            display: LayoutDisplay::Grid,
            width: LayoutDimension::Fixed(220.0),
            height: LayoutDimension::Fixed(40.0),
            align_content: Some(LayoutAlignContent::FlexStart),
            justify_content: LayoutJustifyContent::FlexStart,
            ..LayoutStyle::default()
        },
    )]);
    let mut placements = BTreeMap::new();
    for (index, (item, spec)) in items.iter().zip(case.items_in_source_order).enumerate() {
        styles.insert(
            item.id(),
            LayoutStyle {
                display: LayoutDisplay::Block,
                width: LayoutDimension::Fixed(spec.width),
                height: LayoutDimension::Fixed(10.0),
                ..LayoutStyle::default()
            },
        );
        let row_start = i32::try_from(index + 1).unwrap();
        placements.insert(
            item.id(),
            GridItemInput {
                row_start: GridPlacementInput::Line(row_start),
                row_end: GridPlacementInput::Line(row_start + 1),
                column_start: GridPlacementInput::Line(spec.column_start),
                column_end: GridPlacementInput::Line(spec.column_end),
            },
        );
    }
    let columns = vec![
        GridTrackSize::MinMax {
            min: GridTrackMinSizingFunction::LengthPx(0.0),
            max: GridTrackMaxSizingFunction::MaxContent,
        };
        4
    ];
    let input = LayoutInput::from_host_document(
        &document.snapshot(),
        root,
        Viewport {
            width: 220.0,
            height: 40.0,
        },
        &styles,
        StyleRevision::default(),
        EnvironmentRevision::default(),
    )
    .unwrap()
    .with_grid_layout(
        BTreeMap::from([(
            root.id(),
            GridContainerInput {
                columns,
                rows: vec![GridTrackSize::LengthPx(10.0)],
                placement_mode: GridPlacementMode::TrackSizing,
                ..GridContainerInput::default()
            },
        )]),
        placements,
    );
    let output = TaffyLayoutEngine
        .compute(&input)
        .unwrap_or_else(|error| panic!("{}: {error:?}", case.id));
    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == root.id())
        .unwrap_or_else(|| panic!("{}: missing Grid track trace", case.id));
    let column_pass = trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .unwrap_or_else(|| panic!("{}: missing Columns pass", case.id));
    let final_stage = column_pass
        .stages
        .last()
        .unwrap_or_else(|| panic!("{}: missing final Columns stage", case.id));
    assert!(
        column_pass.stages.iter().any(|stage| {
            matches!(
                stage.stage,
                LayoutGridTrackSizingStageKind::IntrinsicSizingComplete
                    | LayoutGridTrackSizingStageKind::FixedTrackEarlyReturn
            )
        }),
        "{}: missing Columns sizing completion",
        case.id
    );
    let columns = final_stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(
        columns.len(),
        case.expected_columns.len(),
        "{}: track count",
        case.id
    );
    for (index, (actual, expected)) in columns.iter().zip(case.expected_columns).enumerate() {
        assert_chrome_close(*actual, *expected, case.id, &format!("Columns[{index}]"));
    }

    let observed_spans = column_pass
        .stages
        .iter()
        .filter_map(|stage| match stage.stage {
            LayoutGridTrackSizingStageKind::IntrinsicItemBatch { span, .. } => Some(span),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        observed_spans, case.expected_span_batches,
        "{}: span order",
        case.id
    );
    let observed_contributions = column_pass
        .contributions
        .iter()
        .map(|contribution| {
            (
                contribution.item,
                contribution.kind,
                contribution.span,
                contribution.value,
            )
        })
        .collect::<Vec<_>>();
    let mut expected_contributions = Vec::new();
    for span in case.expected_span_batches {
        for kind in [
            crate::LayoutGridTrackSizingContributionKind::Minimum,
            crate::LayoutGridTrackSizingContributionKind::MinContent,
            crate::LayoutGridTrackSizingContributionKind::MaxContent,
        ] {
            expected_contributions.extend(
                items
                    .iter()
                    .zip(case.items_in_source_order)
                    .filter(|(_, item)| (item.column_end - item.column_start) as u16 == *span)
                    .map(|(item, spec)| (item.id(), kind, *span, spec.width)),
            );
        }
    }
    assert_eq!(
        observed_contributions.len(),
        expected_contributions.len(),
        "{}: contribution count",
        case.id
    );
    for (actual, expected) in observed_contributions.iter().zip(expected_contributions) {
        assert_eq!(
            (actual.0, actual.1, actual.2),
            (expected.0, expected.1, expected.2)
        );
        assert_chrome_close(actual.3, expected.3, case.id, "contribution value");
    }

    for (row, (item, spec)) in items.iter().zip(case.items_in_source_order).enumerate() {
        let expected_x = case.expected_columns[..(spec.column_start - 1) as usize]
            .iter()
            .sum::<f32>();
        let frame = output.frames[&item.id()];
        for (actual, expected, property) in [
            (frame.x, expected_x, "item.x"),
            (frame.y, row as f32 * 10.0, "item.y"),
            (frame.width, spec.width, "item.width"),
            (frame.height, 10.0, "item.height"),
        ] {
            assert_chrome_close(actual, expected, case.id, property);
        }
    }
}

fn assert_chrome_close(actual: f32, expected: f32, case: &str, property: &str) {
    assert!(
        (actual - expected).abs() <= 1.0 / 64.0,
        "{case}: {property} differs from Chrome 155 by more than 1/64 CSS px: expected {expected}, got {actual}"
    );
}

#[test]
fn chrome155_multi_span_contributions_are_independent_of_item_source_order() {
    const A: MultiSpanItem = MultiSpanItem {
        width: 80.0,
        column_start: 1,
        column_end: 3,
    };
    const B: MultiSpanItem = MultiSpanItem {
        width: 120.0,
        column_start: 2,
        column_end: 4,
    };
    const C: MultiSpanItem = MultiSpanItem {
        width: 200.0,
        column_start: 1,
        column_end: 4,
    };
    let cases = [
        ChromeMultiSpanCase {
            id: "competing-span2-forward",
            items_in_source_order: &[A, B],
            expected_columns: &[40.0, 60.0, 60.0, 0.0],
            expected_span_batches: &[2],
        },
        ChromeMultiSpanCase {
            id: "competing-span2-reversed",
            items_in_source_order: &[B, A],
            expected_columns: &[40.0, 60.0, 60.0, 0.0],
            expected_span_batches: &[2],
        },
        ChromeMultiSpanCase {
            id: "span2-before-span3",
            items_in_source_order: &[A, B, C],
            expected_columns: &[53.328125, 73.328125, 73.34375, 0.0],
            expected_span_batches: &[2, 3],
        },
        ChromeMultiSpanCase {
            id: "span3-before-span2",
            items_in_source_order: &[C, B, A],
            expected_columns: &[53.328125, 73.328125, 73.34375, 0.0],
            expected_span_batches: &[2, 3],
        },
    ];
    for (index, case) in cases.iter().enumerate() {
        run_multi_span_case(case, 1_600 + index as u64);
    }
}
