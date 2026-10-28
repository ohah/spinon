use std::collections::BTreeMap;

use spinon_core::{ChangeBatch, EnvironmentRevision, NodeId, Operation, StyleRevision, Tree};

use crate::{
    GridContainerInput, GridItemInput, GridPlacementInput, GridPlacementMode,
    GridTrackMaxSizingFunction, GridTrackMinSizingFunction, GridTrackSize, LayoutAlignContent,
    LayoutAlignItems, LayoutDimension, LayoutDisplay, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, LayoutGridTrackSizingMinimumContributionSource,
    LayoutOverflow, LayoutStyle, ParagraphInput, ParagraphStyle, TaffyLayoutEngine,
    TextLayoutEngine, TextSource, Viewport, WhiteSpaceMode,
};

use super::{AHEM, node};

#[derive(Clone, Copy)]
enum ExpectedMinimumSource {
    ContentBased,
    Overflow,
}

struct Case {
    id: &'static str,
    overflow: LayoutOverflow,
    expected_tracks: [f32; 2],
    expected_item_width: f32,
    expected_minimum: f32,
    minimum_source: ExpectedMinimumSource,
}

const CASES: [Case; 5] = [
    Case {
        id: "columns-span2-visible-content-minimum",
        overflow: LayoutOverflow::Visible,
        expected_tracks: [50.0, 50.0],
        expected_item_width: 100.0,
        expected_minimum: 100.0,
        minimum_source: ExpectedMinimumSource::ContentBased,
    },
    Case {
        id: "columns-span2-clip-content-minimum",
        overflow: LayoutOverflow::Clip,
        expected_tracks: [50.0, 50.0],
        expected_item_width: 100.0,
        expected_minimum: 100.0,
        minimum_source: ExpectedMinimumSource::ContentBased,
    },
    Case {
        id: "columns-span2-hidden-zero-minimum",
        overflow: LayoutOverflow::Hidden,
        expected_tracks: [40.0, 40.0],
        expected_item_width: 80.0,
        expected_minimum: 0.0,
        minimum_source: ExpectedMinimumSource::Overflow,
    },
    Case {
        id: "columns-span2-auto-zero-minimum",
        overflow: LayoutOverflow::Auto,
        expected_tracks: [40.0, 40.0],
        expected_item_width: 80.0,
        expected_minimum: 0.0,
        minimum_source: ExpectedMinimumSource::Overflow,
    },
    Case {
        id: "columns-span2-scroll-zero-minimum",
        overflow: LayoutOverflow::Scroll,
        expected_tracks: [40.0, 40.0],
        expected_item_width: 80.0,
        expected_minimum: 0.0,
        minimum_source: ExpectedMinimumSource::Overflow,
    },
];

#[test]
fn text_grid_auto_minmax_max_content_two_track_span_follows_overflow_scrollability() {
    let mut text_engine = TextLayoutEngine::new_without_system_fonts();
    text_engine.register_font("Ahem", AHEM).unwrap();

    for case in &CASES {
        run_case(case, &text_engine);
    }
}

fn run_case(case: &Case, text_engine: &TextLayoutEngine) {
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

    let (root_width, root_height) = (80.0, 20.0);
    let root_style = LayoutStyle {
        display: LayoutDisplay::Grid,
        width: LayoutDimension::Fixed(root_width),
        height: LayoutDimension::Fixed(root_height),
        align_items: LayoutAlignItems::Stretch,
        align_content: Some(LayoutAlignContent::FlexStart),
        justify_content: crate::LayoutJustifyContent::FlexStart,
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
                columns: vec![auto_max_content_track(), auto_max_content_track()],
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
                column_end: GridPlacementInput::Line(3),
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
    exact(root_frame.x, 0.0, case.id, "Grid x");
    exact(root_frame.y, 0.0, case.id, "Grid y");
    exact(root_frame.width, root_width, case.id, "Grid width");
    exact(root_frame.height, root_height, case.id, "Grid height");
    exact(
        item_frame.width,
        case.expected_item_width,
        case.id,
        "item width",
    );
    exact(item_frame.height, root_height, case.id, "item height");
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

fn assert_pass(case: &Case, pass: &crate::LayoutGridTrackSizingPassTrace, item: NodeId) {
    let final_stage = pass
        .stages
        .last()
        .unwrap_or_else(|| panic!("{}: final stage missing", case.id));
    let tracks = final_stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .collect::<Vec<_>>();
    assert_eq!(tracks.len(), 2, "{}: expected two target tracks", case.id);
    for (index, (track, expected)) in tracks.iter().zip(case.expected_tracks).enumerate() {
        exact(
            track.base_size,
            expected,
            case.id,
            &format!("final column track {index}"),
        );
    }

    let minimums = pass
        .contributions
        .iter()
        .filter(|contribution| {
            contribution.item == item
                && contribution.kind == LayoutGridTrackSizingContributionKind::Minimum
        })
        .collect::<Vec<_>>();
    assert_eq!(minimums.len(), 1, "{}: minimum trace count", case.id);
    exact(
        minimums[0].value,
        case.expected_minimum,
        case.id,
        "minimum contribution",
    );
    let source_matches = matches!(
        (case.minimum_source, minimums[0].minimum_source),
        (
            ExpectedMinimumSource::ContentBased,
            Some(
                LayoutGridTrackSizingMinimumContributionSource::AutomaticMinimum {
                    spans_auto_min_track: true,
                    spans_multiple_tracks: true,
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
        case.id, minimums[0].minimum_source
    );

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
    exact(
        max_contents[0].value,
        100.0,
        case.id,
        "max-content contribution",
    );
}

fn exact(actual: f32, expected: f32, case: &str, property: &str) {
    assert_eq!(
        actual, expected,
        "{case}: {property} expected {expected}, got {actual}"
    );
}

fn auto_max_content_track() -> GridTrackSize {
    GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::Auto,
        max: GridTrackMaxSizingFunction::MaxContent,
    }
}
