use std::collections::BTreeMap;

use spinon_core::{
    DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument, HostNodeHandle,
    HostParent, IntrinsicSizeRevision, OwnerId, StyleRevision,
};

use crate::{
    GridContainerInput, GridItemInput, GridPlacementInput, GridPlacementMode, GridTrackSize,
    LayoutAlignContent, LayoutAlignItems, LayoutDimension, LayoutDisplay, LayoutIntrinsicSize,
    LayoutIntrinsicSizeSnapshot, LayoutJustifyContent, LayoutSourceRevision, LayoutStyle,
    TaffyLayoutEngine, Viewport,
};

use super::{Axis, CASES, Case, assert_minimum_trace, auto_fr_track, close};

const HTML: &str = "http://www.w3.org/1999/xhtml";

#[test]
fn replaced_grid_row_auto_minimum_follows_overflow_scrollability() {
    for (index, case) in CASES
        .iter()
        .filter(|case| matches!(case.axis, Axis::Rows))
        .enumerate()
    {
        run_case(case, 3_000 + index as u64);
    }
}

fn run_case(case: &Case, owner: u64) {
    let (document, root, image) = create_document(owner);
    let snapshot = document.snapshot();
    let intrinsic = LayoutIntrinsicSizeSnapshot::new(
        LayoutSourceRevision::HostDocument {
            generation: snapshot.generation(),
            document: snapshot.document_revision(),
            render_tree: snapshot.render_tree_revision(),
        },
        IntrinsicSizeRevision::INITIAL.checked_next().unwrap(),
        BTreeMap::from([(image.id(), LayoutIntrinsicSize::new(10.0, 10.0))]),
    )
    .unwrap();
    let root_style = LayoutStyle {
        display: LayoutDisplay::Grid,
        width: LayoutDimension::Fixed(30.0),
        height: LayoutDimension::Fixed(5.0),
        align_items: LayoutAlignItems::Stretch,
        align_content: Some(LayoutAlignContent::FlexStart),
        justify_content: LayoutJustifyContent::FlexStart,
        ..LayoutStyle::default()
    };
    let image_style = LayoutStyle {
        display: LayoutDisplay::Block,
        width: LayoutDimension::Fixed(10.0),
        min_height: if case.zero_item_minimum {
            LayoutDimension::Fixed(0.0)
        } else {
            LayoutDimension::Auto
        },
        overflow_x: case.overflow,
        overflow_y: case.overflow,
        ..LayoutStyle::default()
    };
    let input = crate::LayoutInput::from_host_document(
        &snapshot,
        root,
        Viewport {
            width: 30.0,
            height: 5.0,
        },
        &BTreeMap::from([(root.id(), root_style), (image.id(), image_style)]),
        StyleRevision::INITIAL,
        EnvironmentRevision::INITIAL,
    )
    .unwrap()
    .with_grid_layout(
        BTreeMap::from([(
            root.id(),
            GridContainerInput {
                columns: vec![GridTrackSize::LengthPx(10.0)],
                rows: vec![auto_fr_track(case.zero_track_minimum)],
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
        .compute_with_intrinsic_size_snapshot(&input, &intrinsic)
        .unwrap_or_else(|error| panic!("{}: {error:?}", case.id));

    let root_frame = output.frames[&root.id()];
    let image_frame = output.frames[&image.id()];
    close(root_frame.width, 30.0, case.id, "Grid width");
    close(root_frame.height, 5.0, case.id, "Grid height");
    close(image_frame.width, 10.0, case.id, "image width");
    close(
        image_frame.height,
        case.expected_track,
        case.id,
        "image height",
    );

    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == root.id())
        .unwrap_or_else(|| panic!("{}: Grid trace missing", case.id));
    let pass = trace
        .passes
        .iter()
        .find(|pass| pass.axis == crate::LayoutGridTrackSizingAxis::Rows)
        .unwrap_or_else(|| panic!("{}: Rows pass missing", case.id));
    let final_stage = pass
        .stages
        .last()
        .unwrap_or_else(|| panic!("{}: final Rows stage missing", case.id));
    let tracks = final_stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .collect::<Vec<_>>();
    assert_eq!(tracks.len(), 1, "{}: expected one row track", case.id);
    close(
        tracks[0].base_size,
        case.expected_track,
        case.id,
        "final row base",
    );

    let minimums = pass
        .contributions
        .iter()
        .filter(|contribution| {
            contribution.item == image.id()
                && contribution.kind == crate::LayoutGridTrackSizingContributionKind::Minimum
        })
        .collect::<Vec<_>>();
    assert_minimum_trace(case, &minimums);
}

fn create_document(owner: u64) -> (HostDocument, HostNodeHandle, HostNodeHandle) {
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
            local_name: "img".to_owned(),
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
