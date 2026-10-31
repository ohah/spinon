use std::collections::BTreeMap;

use spinon_core::{
    ChangeBatch, DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument,
    HostNodeHandle, HostParent, IntrinsicSizeRevision, NodeId, Operation, OwnerId, StyleRevision,
    Tree,
};

use crate::{
    GridContainerInput, GridItemInput, GridPlacementInput, GridPlacementMode,
    GridTrackMaxSizingFunction, GridTrackMinSizingFunction, GridTrackSize, LayoutAlignContent,
    LayoutAlignItems, LayoutAlignSelf, LayoutDimension, LayoutDisplay, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, LayoutGridTrackSizingContributionTrace,
    LayoutGridTrackSizingMinimumContributionSource, LayoutGridTrackSizingPassTrace,
    LayoutIntrinsicSize, LayoutIntrinsicSizeSnapshot, LayoutOverflow, LayoutSourceRevision,
    LayoutStyle, ParagraphInput, ParagraphStyle, TaffyLayoutEngine, TextLayoutEngine, TextSource,
    Viewport, WhiteSpaceMode,
};

use super::{AHEM, close, node};

const HTML: &str = "http://www.w3.org/1999/xhtml";

#[derive(Clone, Copy)]
enum Axis {
    Columns,
    Rows,
}

#[derive(Clone, Copy)]
enum ExpectedMinimumSource {
    ContentBased,
    Overflow,
}

struct Case {
    id: &'static str,
    axis: Axis,
    overflow: LayoutOverflow,
    expected_track: f32,
    expected_minimum: f32,
    minimum_source: ExpectedMinimumSource,
    expected_max_content: f32,
}

const CASES: [Case; 10] = [
    Case {
        id: "column-visible-content-minimum",
        axis: Axis::Columns,
        overflow: LayoutOverflow::Visible,
        expected_track: 100.0,
        expected_minimum: 100.0,
        minimum_source: ExpectedMinimumSource::ContentBased,
        expected_max_content: 100.0,
    },
    Case {
        id: "column-clip-content-minimum",
        axis: Axis::Columns,
        overflow: LayoutOverflow::Clip,
        expected_track: 100.0,
        expected_minimum: 100.0,
        minimum_source: ExpectedMinimumSource::ContentBased,
        expected_max_content: 100.0,
    },
    Case {
        id: "column-hidden-zero-minimum",
        axis: Axis::Columns,
        overflow: LayoutOverflow::Hidden,
        expected_track: 60.0,
        expected_minimum: 0.0,
        minimum_source: ExpectedMinimumSource::Overflow,
        expected_max_content: 100.0,
    },
    Case {
        id: "column-auto-zero-minimum",
        axis: Axis::Columns,
        overflow: LayoutOverflow::Auto,
        expected_track: 60.0,
        expected_minimum: 0.0,
        minimum_source: ExpectedMinimumSource::Overflow,
        expected_max_content: 100.0,
    },
    Case {
        id: "column-scroll-zero-minimum",
        axis: Axis::Columns,
        overflow: LayoutOverflow::Scroll,
        expected_track: 60.0,
        expected_minimum: 0.0,
        minimum_source: ExpectedMinimumSource::Overflow,
        expected_max_content: 100.0,
    },
    Case {
        id: "row-visible-content-minimum",
        axis: Axis::Rows,
        overflow: LayoutOverflow::Visible,
        expected_track: 10.0,
        expected_minimum: 10.0,
        minimum_source: ExpectedMinimumSource::ContentBased,
        expected_max_content: 10.0,
    },
    Case {
        id: "row-clip-content-minimum",
        axis: Axis::Rows,
        overflow: LayoutOverflow::Clip,
        expected_track: 10.0,
        expected_minimum: 10.0,
        minimum_source: ExpectedMinimumSource::ContentBased,
        expected_max_content: 10.0,
    },
    Case {
        id: "row-hidden-zero-minimum",
        axis: Axis::Rows,
        overflow: LayoutOverflow::Hidden,
        expected_track: 5.0,
        expected_minimum: 0.0,
        minimum_source: ExpectedMinimumSource::Overflow,
        expected_max_content: 10.0,
    },
    Case {
        id: "row-auto-zero-minimum",
        axis: Axis::Rows,
        overflow: LayoutOverflow::Auto,
        expected_track: 5.0,
        expected_minimum: 0.0,
        minimum_source: ExpectedMinimumSource::Overflow,
        expected_max_content: 10.0,
    },
    Case {
        id: "row-scroll-zero-minimum",
        axis: Axis::Rows,
        overflow: LayoutOverflow::Scroll,
        expected_track: 5.0,
        expected_minimum: 0.0,
        minimum_source: ExpectedMinimumSource::Overflow,
        expected_max_content: 10.0,
    },
];

#[test]
fn text_grid_auto_minmax_max_content_follows_overflow_scrollability_in_columns() {
    let mut text_engine = TextLayoutEngine::new_without_system_fonts();
    text_engine.register_font("Ahem", AHEM).unwrap();

    for case in CASES
        .iter()
        .filter(|case| matches!(case.axis, Axis::Columns))
    {
        run_columns_case(case, &text_engine);
    }
}

#[test]
fn replaced_grid_auto_minmax_max_content_follows_overflow_scrollability_in_rows() {
    for (index, case) in CASES
        .iter()
        .filter(|case| matches!(case.axis, Axis::Rows))
        .enumerate()
    {
        run_rows_case(case, 4_000 + index as u64);
    }
}

fn run_columns_case(case: &Case, text_engine: &TextLayoutEngine) {
    let root = node(1);
    let item = node(2);
    let text = node(3);
    let mut tree = Tree::new();
    let mut batch = ChangeBatch::new(tree.revision());
    batch
        .push(Operation::Create {
            id: root,
            tag: "main".to_owned(),
        })
        .push(Operation::Insert {
            id: root,
            parent: None,
            index: 0,
        })
        .push(Operation::Create {
            id: item,
            tag: "div".to_owned(),
        })
        .push(Operation::Insert {
            id: item,
            parent: Some(root),
            index: 0,
        });
    tree.commit(batch).unwrap();

    let (root_width, root_height) = (60.0, 20.0);
    let root_style = LayoutStyle {
        display: LayoutDisplay::Grid,
        width: LayoutDimension::Fixed(root_width),
        height: LayoutDimension::Fixed(root_height),
        align_items: LayoutAlignItems::Stretch,
        align_content: Some(LayoutAlignContent::FlexStart),
        ..LayoutStyle::default()
    };
    let item_style = LayoutStyle {
        display: LayoutDisplay::Block,
        overflow_x: case.overflow,
        overflow_y: LayoutOverflow::Visible,
        ..LayoutStyle::default()
    };
    let input = crate::LayoutInput::from_tree(
        &tree,
        Viewport {
            width: root_width,
            height: root_height,
        },
        &BTreeMap::from([(root, root_style), (item, item_style)]),
        StyleRevision::INITIAL,
        EnvironmentRevision::INITIAL,
    )
    .unwrap()
    .with_grid_layout(
        BTreeMap::from([(
            root,
            GridContainerInput {
                columns: vec![auto_max_content_track()],
                rows: vec![GridTrackSize::LengthPx(root_height)],
                placement_mode: GridPlacementMode::TrackSizing,
                ..GridContainerInput::default()
            },
        )]),
        BTreeMap::from([(
            item,
            GridItemInput {
                row_start: GridPlacementInput::Line(1),
                row_end: GridPlacementInput::Line(2),
                column_start: GridPlacementInput::Line(1),
                column_end: GridPlacementInput::Line(2),
            },
        )]),
    )
    .with_text_paragraphs([ParagraphInput {
        node: item,
        sources: vec![TextSource {
            node: text,
            inline_ancestors: Vec::new(),
            text: "XXXXXXXXXX".to_owned(),
        }],
        style: ParagraphStyle {
            overflow_wrap: Default::default(),
            writing_mode: spinon_core::WritingMode::HorizontalTb,
            text_orientation: spinon_core::TextOrientation::Mixed,
            font_family: "Ahem".to_owned(),
            font_size: 10.0,
            line_height: 10.0,
            white_space: WhiteSpaceMode::Normal,
            ..ParagraphStyle::default()
        },
        inline_styles: BTreeMap::new(),
        inline_run_styles: BTreeMap::new(),
    }])
    .unwrap();

    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, text_engine)
        .unwrap_or_else(|error| panic!("{}: {error:?}", case.id));
    let root_frame = output.frames[&root];
    let item_frame = output.frames[&item];
    close(root_frame.width, root_width, case.id, "Grid width");
    close(root_frame.height, root_height, case.id, "Grid height");
    close(item_frame.width, case.expected_track, case.id, "item width");
    close(item_frame.height, root_height, case.id, "item height");
    assert_eq!(item_frame.x, root_frame.x, "{}: item x", case.id);
    assert_eq!(item_frame.y, root_frame.y, "{}: item y", case.id);

    let pass = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == root)
        .unwrap_or_else(|| panic!("{}: Grid trace missing", case.id))
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .unwrap_or_else(|| panic!("{}: Columns pass missing", case.id));
    assert_pass(case, pass, item);
}

fn run_rows_case(case: &Case, owner: u64) {
    let (document, root, image) = create_document(owner);
    let snapshot = document.snapshot();
    let intrinsic = LayoutIntrinsicSizeSnapshot::new(
        LayoutSourceRevision::HostDocument {
            generation: snapshot.generation(),
            document: snapshot.document_revision(),
            render_tree: snapshot.render_tree_revision(),
        },
        IntrinsicSizeRevision::INITIAL.checked_next().unwrap(),
        BTreeMap::from([(image.id(), LayoutIntrinsicSize::new(24.0, 16.0))]),
    )
    .unwrap();
    let root_style = LayoutStyle {
        display: LayoutDisplay::Grid,
        width: LayoutDimension::Fixed(30.0),
        height: LayoutDimension::Fixed(5.0),
        align_items: LayoutAlignItems::Stretch,
        align_content: Some(LayoutAlignContent::FlexStart),
        ..LayoutStyle::default()
    };
    let image_style = LayoutStyle {
        display: LayoutDisplay::Block,
        width: LayoutDimension::Fixed(15.0),
        align_self: LayoutAlignSelf::FlexStart,
        overflow_x: LayoutOverflow::Visible,
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
                columns: vec![GridTrackSize::LengthPx(15.0)],
                rows: vec![auto_max_content_track()],
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
    close(image_frame.width, 15.0, case.id, "image width");
    close(image_frame.height, 10.0, case.id, "image height");
    assert_eq!(image_frame.x, root_frame.x, "{}: image x", case.id);
    assert_eq!(image_frame.y, root_frame.y, "{}: image y", case.id);

    let pass = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == root.id())
        .unwrap_or_else(|| panic!("{}: Grid trace missing", case.id))
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .unwrap_or_else(|| panic!("{}: Rows pass missing", case.id));
    assert_pass(case, pass, image.id());
}

fn assert_pass(case: &Case, pass: &LayoutGridTrackSizingPassTrace, item: NodeId) {
    let final_stage = pass
        .stages
        .last()
        .unwrap_or_else(|| panic!("{}: final stage missing", case.id));
    let tracks = final_stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .collect::<Vec<_>>();
    assert_eq!(tracks.len(), 1, "{}: expected one target track", case.id);
    close(
        tracks[0].base_size,
        case.expected_track,
        case.id,
        "final track base",
    );

    let minimums = pass
        .contributions
        .iter()
        .filter(|contribution| {
            contribution.item == item
                && contribution.kind == LayoutGridTrackSizingContributionKind::Minimum
        })
        .collect::<Vec<_>>();
    assert_minimum_trace(case, &minimums);

    let max_contents = pass
        .contributions
        .iter()
        .filter(|contribution| {
            contribution.item == item
                && contribution.kind == LayoutGridTrackSizingContributionKind::MaxContent
        })
        .collect::<Vec<_>>();
    assert_eq!(
        max_contents.len(),
        1,
        "{}: max-content contribution trace count",
        case.id
    );
    close(
        max_contents[0].value,
        case.expected_max_content,
        case.id,
        "max-content contribution",
    );
}

fn assert_minimum_trace(case: &Case, minimums: &[&LayoutGridTrackSizingContributionTrace]) {
    assert_eq!(minimums.len(), 1, "{}: minimum trace count", case.id);
    let minimum = minimums[0];
    close(
        minimum.value,
        case.expected_minimum,
        case.id,
        "minimum contribution",
    );
    let source_matches = matches!(
        (case.minimum_source, minimum.minimum_source),
        (
            ExpectedMinimumSource::ContentBased,
            Some(
                LayoutGridTrackSizingMinimumContributionSource::AutomaticMinimum {
                    spans_auto_min_track: true,
                    spans_multiple_tracks: false,
                    spans_flexible_max_track: false,
                    content_based: true,
                    ..
                }
            )
        ) | (
            ExpectedMinimumSource::Overflow,
            Some(LayoutGridTrackSizingMinimumContributionSource::OverflowAutomaticMinimum)
        )
    );
    assert!(
        source_matches,
        "{}: unexpected minimum contribution source {:?}",
        case.id, minimum.minimum_source
    );
}

fn auto_max_content_track() -> GridTrackSize {
    GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::Auto,
        max: GridTrackMaxSizingFunction::MaxContent,
    }
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
