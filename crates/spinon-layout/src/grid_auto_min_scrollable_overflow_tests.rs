use std::collections::BTreeMap;

use spinon_core::{ChangeBatch, EnvironmentRevision, NodeId, Operation, StyleRevision, Tree};

use crate::{
    GridContainerInput, GridItemInput, GridPlacementInput, GridPlacementMode,
    GridTrackMaxSizingFunction, GridTrackMinSizingFunction, GridTrackSize, LayoutAlignContent,
    LayoutAlignItems, LayoutDimension, LayoutDisplay, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, LayoutGridTrackSizingContributionTrace,
    LayoutGridTrackSizingMinimumContributionSource, LayoutJustifyContent, LayoutOverflow,
    LayoutStyle, ParagraphInput, ParagraphStyle, TaffyLayoutEngine, TextLayoutEngine, TextSource,
    Viewport, WhiteSpaceMode,
};

const AHEM: &[u8] = include_bytes!("../../../tests/fixtures/resources/c15/Ahem.ttf");

#[path = "grid_auto_min_scrollable_overflow_tests/max_content.rs"]
mod max_content;
#[path = "grid_auto_min_scrollable_overflow_tests/row_axis_replaced.rs"]
mod row_axis_replaced;
#[path = "grid_auto_min_scrollable_overflow_tests/two_track_span_max_content.rs"]
mod two_track_span_max_content;

#[derive(Clone, Copy)]
enum Axis {
    Columns,
    Rows,
}

#[derive(Clone, Copy)]
enum ExpectedMinimumSource {
    ContentBased,
    Overflow,
    Specified,
    TrackMinimum,
}

struct Case {
    id: &'static str,
    axis: Axis,
    overflow: LayoutOverflow,
    zero_item_minimum: bool,
    zero_track_minimum: bool,
    expected_track: f32,
    expected_minimum: Option<f32>,
    minimum_source: ExpectedMinimumSource,
}

const CASES: [Case; 14] = [
    Case {
        id: "column-visible-content-minimum",
        axis: Axis::Columns,
        overflow: LayoutOverflow::Visible,
        zero_item_minimum: false,
        zero_track_minimum: false,
        expected_track: 100.0,
        expected_minimum: Some(100.0),
        minimum_source: ExpectedMinimumSource::ContentBased,
    },
    Case {
        id: "column-clip-content-minimum",
        axis: Axis::Columns,
        overflow: LayoutOverflow::Clip,
        zero_item_minimum: false,
        zero_track_minimum: false,
        expected_track: 100.0,
        expected_minimum: Some(100.0),
        minimum_source: ExpectedMinimumSource::ContentBased,
    },
    Case {
        id: "column-hidden-zero-minimum",
        axis: Axis::Columns,
        overflow: LayoutOverflow::Hidden,
        zero_item_minimum: false,
        zero_track_minimum: false,
        expected_track: 60.0,
        expected_minimum: Some(0.0),
        minimum_source: ExpectedMinimumSource::Overflow,
    },
    Case {
        id: "column-auto-zero-minimum",
        axis: Axis::Columns,
        overflow: LayoutOverflow::Auto,
        zero_item_minimum: false,
        zero_track_minimum: false,
        expected_track: 60.0,
        expected_minimum: Some(0.0),
        minimum_source: ExpectedMinimumSource::Overflow,
    },
    Case {
        id: "column-scroll-zero-minimum",
        axis: Axis::Columns,
        overflow: LayoutOverflow::Scroll,
        zero_item_minimum: false,
        zero_track_minimum: false,
        expected_track: 60.0,
        expected_minimum: Some(0.0),
        minimum_source: ExpectedMinimumSource::Overflow,
    },
    Case {
        id: "column-specified-minimum-control",
        axis: Axis::Columns,
        overflow: LayoutOverflow::Visible,
        zero_item_minimum: true,
        zero_track_minimum: false,
        expected_track: 60.0,
        expected_minimum: Some(0.0),
        minimum_source: ExpectedMinimumSource::Specified,
    },
    Case {
        id: "column-zero-track-control",
        axis: Axis::Columns,
        overflow: LayoutOverflow::Visible,
        zero_item_minimum: false,
        zero_track_minimum: true,
        expected_track: 60.0,
        expected_minimum: None,
        minimum_source: ExpectedMinimumSource::TrackMinimum,
    },
    Case {
        id: "row-visible-content-minimum",
        axis: Axis::Rows,
        overflow: LayoutOverflow::Visible,
        zero_item_minimum: false,
        zero_track_minimum: false,
        expected_track: 10.0,
        expected_minimum: Some(10.0),
        minimum_source: ExpectedMinimumSource::ContentBased,
    },
    Case {
        id: "row-clip-content-minimum",
        axis: Axis::Rows,
        overflow: LayoutOverflow::Clip,
        zero_item_minimum: false,
        zero_track_minimum: false,
        expected_track: 10.0,
        expected_minimum: Some(10.0),
        minimum_source: ExpectedMinimumSource::ContentBased,
    },
    Case {
        id: "row-hidden-zero-minimum",
        axis: Axis::Rows,
        overflow: LayoutOverflow::Hidden,
        zero_item_minimum: false,
        zero_track_minimum: false,
        expected_track: 5.0,
        expected_minimum: Some(0.0),
        minimum_source: ExpectedMinimumSource::Overflow,
    },
    Case {
        id: "row-auto-zero-minimum",
        axis: Axis::Rows,
        overflow: LayoutOverflow::Auto,
        zero_item_minimum: false,
        zero_track_minimum: false,
        expected_track: 5.0,
        expected_minimum: Some(0.0),
        minimum_source: ExpectedMinimumSource::Overflow,
    },
    Case {
        id: "row-scroll-zero-minimum",
        axis: Axis::Rows,
        overflow: LayoutOverflow::Scroll,
        zero_item_minimum: false,
        zero_track_minimum: false,
        expected_track: 5.0,
        expected_minimum: Some(0.0),
        minimum_source: ExpectedMinimumSource::Overflow,
    },
    Case {
        id: "row-specified-minimum-control",
        axis: Axis::Rows,
        overflow: LayoutOverflow::Visible,
        zero_item_minimum: true,
        zero_track_minimum: false,
        expected_track: 5.0,
        expected_minimum: Some(0.0),
        minimum_source: ExpectedMinimumSource::Specified,
    },
    Case {
        id: "row-zero-track-control",
        axis: Axis::Rows,
        overflow: LayoutOverflow::Visible,
        zero_item_minimum: false,
        zero_track_minimum: true,
        expected_track: 5.0,
        expected_minimum: None,
        minimum_source: ExpectedMinimumSource::TrackMinimum,
    },
];

#[test]
fn text_grid_auto_minimum_follows_overflow_scrollability_in_columns() {
    let mut text_engine = TextLayoutEngine::new_without_system_fonts();
    text_engine.register_font("Ahem", AHEM).unwrap();

    for case in CASES
        .iter()
        .filter(|case| matches!(case.axis, Axis::Columns))
    {
        run_case(case, &text_engine);
    }
}

fn run_case(case: &Case, text_engine: &TextLayoutEngine) {
    assert!(
        matches!(case.axis, Axis::Columns),
        "Rows cases use the replaced-image layout path"
    );
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
        justify_content: LayoutJustifyContent::FlexStart,
        ..LayoutStyle::default()
    };
    let item_style = LayoutStyle {
        display: LayoutDisplay::Block,
        min_width: if case.zero_item_minimum && matches!(case.axis, Axis::Columns) {
            LayoutDimension::Fixed(0.0)
        } else {
            LayoutDimension::Auto
        },
        min_height: if case.zero_item_minimum && matches!(case.axis, Axis::Rows) {
            LayoutDimension::Fixed(0.0)
        } else {
            LayoutDimension::Auto
        },
        overflow_x: case.overflow,
        overflow_y: case.overflow,
        ..LayoutStyle::default()
    };
    let mut input = crate::LayoutInput::from_tree(
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
                columns: vec![auto_fr_track(case.zero_track_minimum)],
                rows: vec![GridTrackSize::LengthPx(20.0)],
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
    );
    input = input
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

    let grid_trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == root)
        .unwrap_or_else(|| panic!("{}: Grid trace missing", case.id));
    let target_axis = LayoutGridTrackSizingAxis::Columns;
    let pass = grid_trace
        .passes
        .iter()
        .find(|pass| pass.axis == target_axis)
        .unwrap_or_else(|| panic!("{}: {target_axis:?} pass missing", case.id));
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
}

fn assert_minimum_trace(case: &Case, minimums: &[&LayoutGridTrackSizingContributionTrace]) {
    if matches!(case.minimum_source, ExpectedMinimumSource::TrackMinimum) {
        assert!(
            minimums.is_empty(),
            "{}: a fixed zero track minimum must not query the item automatic minimum",
            case.id
        );
        assert!(case.expected_minimum.is_none());
        return;
    }
    assert_eq!(minimums.len(), 1, "{}: target minimum trace count", case.id);
    let minimum = minimums[0];
    close(
        minimum.value,
        case.expected_minimum
            .expect("a queried minimum contribution must have an expected value"),
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
                    spans_flexible_max_track: true,
                    content_based: true,
                    ..
                }
            )
        ) | (
            ExpectedMinimumSource::Overflow,
            Some(LayoutGridTrackSizingMinimumContributionSource::OverflowAutomaticMinimum)
        ) | (
            ExpectedMinimumSource::Specified,
            Some(LayoutGridTrackSizingMinimumContributionSource::SpecifiedMinimum)
        )
    );
    assert!(
        source_matches,
        "{}: unexpected minimum contribution source {:?}",
        case.id, minimum.minimum_source
    );
}

fn auto_fr_track(zero_minimum: bool) -> GridTrackSize {
    GridTrackSize::MinMax {
        min: if zero_minimum {
            GridTrackMinSizingFunction::LengthPx(0.0)
        } else {
            GridTrackMinSizingFunction::Auto
        },
        max: GridTrackMaxSizingFunction::Fraction(1.0),
    }
}

fn node(value: u64) -> NodeId {
    NodeId::new(value).unwrap()
}

fn close(actual: f32, expected: f32, case: &str, property: &str) {
    assert!(
        (actual - expected).abs() <= 1.0 / 64.0,
        "{case}: {property} expected {expected}, got {actual}"
    );
}
