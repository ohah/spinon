use std::collections::BTreeMap;

use spinon_core::{EnvironmentRevision, NodeId, Revision, StyleRevision};

use super::*;
use crate::grid_auto_placement::{TestGridReservationSite, inject_failure_once};
use crate::grid_placement::{GridItemInput, GridPlacementInput};
use crate::{
    LayoutDimension, LayoutDisplay, LayoutEngine, LayoutGap, LayoutInputRevision, LayoutNode,
    LayoutSourceRevision, LayoutStyle, TaffyLayoutEngine, Viewport,
};

fn id(value: u64) -> NodeId {
    NodeId::new(value).unwrap()
}

fn grid_input(
    columns: Vec<GridTrackSize>,
    rows: Vec<GridTrackSize>,
    item_count: usize,
    gap: LayoutGap,
) -> LayoutInput {
    let root = id(1);
    let children = (0..item_count)
        .map(|index| id(index as u64 + 2))
        .collect::<Vec<_>>();
    let mut nodes = vec![LayoutNode {
        id: root,
        children: children.clone(),
        style: LayoutStyle {
            display: LayoutDisplay::Grid,
            width: LayoutDimension::Fixed(320.0),
            height: LayoutDimension::Fixed(200.0),
            gap,
            ..LayoutStyle::default()
        },
    }];
    nodes.extend(children.into_iter().map(|child| LayoutNode {
        id: child,
        children: Vec::new(),
        style: LayoutStyle {
            display: LayoutDisplay::Block,
            width: LayoutDimension::Fixed(20.0),
            height: LayoutDimension::Fixed(10.0),
            ..LayoutStyle::default()
        },
    }));
    LayoutInput {
        root,
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: 320.0,
            height: 200.0,
        },
        root_sizing: crate::RootSizingPolicy::Match,
        nodes,
        css_math: Vec::new(),
        positioning: BTreeMap::new(),
        grid_containers: BTreeMap::from([(
            root,
            GridContainerInput {
                columns,
                rows,
                columns_auto_repeat: None,
                rows_auto_repeat: None,
                columns_line_names: Vec::new(),
                rows_line_names: Vec::new(),
                template_areas: None,
                justify_items: Default::default(),
                auto_flow: GridAutoFlow::Row,
                auto_rows: GridAutoRows::Auto,
                auto_columns: crate::GridAutoColumns::Auto,
                placement_mode: GridPlacementMode::Existing,
                track_sizing_trace_enabled: false,
            },
        )]),
        grid_items: Default::default(),
        text_paragraphs: Default::default(),
    }
}

#[test]
fn track_sizing_indefinite_height_allows_bounded_intrinsic_rows_only() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(98.0)],
        vec![GridTrackSize::MinContent, GridTrackSize::MinContent],
        1,
        LayoutGap::default(),
    );
    input.nodes[0].style.height = LayoutDimension::Auto;
    input.root_sizing = crate::RootSizingPolicy::ResolveWithin;
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::TrackSizing;

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    assert_eq!(output.frames[&id(1)].height, 10.0);
    assert_eq!(
        final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Rows),
        [10.0, 0.0]
    );

    let mut percentage = input;
    percentage.grid_containers.get_mut(&id(1)).unwrap().rows[0] = GridTrackSize::Percentage(50.0);
    let error = TaffyLayoutEngine.compute(&percentage).unwrap_err();
    assert!(matches!(
        error,
        crate::LayoutError::IndefiniteGridBasis {
            node: grid,
            axis: "height"
        } if grid == id(1)
    ));
}

#[test]
fn track_sizing_indefinite_height_rejects_flexible_and_fit_content_rows() {
    for row in [
        GridTrackSize::Fraction(1.0),
        GridTrackSize::FitContent(crate::GridTrackLengthPercentage::LengthPx(20.0)),
        GridTrackSize::MinMax {
            min: crate::GridTrackMinSizingFunction::Auto,
            max: crate::GridTrackMaxSizingFunction::Fraction(1.0),
        },
    ] {
        let mut input = grid_input(
            vec![GridTrackSize::LengthPx(98.0)],
            vec![row],
            0,
            LayoutGap::default(),
        );
        input.nodes[0].style.height = LayoutDimension::Auto;
        input.root_sizing = crate::RootSizingPolicy::ResolveWithin;
        input
            .grid_containers
            .get_mut(&id(1))
            .unwrap()
            .placement_mode = GridPlacementMode::TrackSizing;

        assert!(matches!(
            TaffyLayoutEngine.compute(&input),
            Err(crate::LayoutError::IndefiniteGridBasis {
                node: grid,
                axis: "height"
            }) if grid == id(1)
        ));
    }
}

#[test]
fn track_sizing_cycles_positive_implicit_auto_track_lists_per_axis() {
    let mut rows = grid_input(
        vec![GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(12.0)],
        4,
        LayoutGap::default(),
    );
    let container = rows.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.auto_rows = GridAutoRows::TrackList(vec![
        GridTrackSize::LengthPx(10.0),
        GridTrackSize::LengthPx(20.0),
    ]);
    let row_output = TaffyLayoutEngine.compute(&rows).unwrap();
    assert_eq!(
        final_axis_track_sizes(&row_output, LayoutGridTrackSizingAxis::Rows),
        [12.0, 10.0, 20.0, 10.0]
    );

    let mut columns = grid_input(
        vec![GridTrackSize::LengthPx(24.0)],
        vec![GridTrackSize::LengthPx(40.0)],
        4,
        LayoutGap::default(),
    );
    let container = columns.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.auto_flow = GridAutoFlow::Column;
    container.auto_columns = crate::GridAutoColumns::TrackList(vec![
        GridTrackSize::LengthPx(10.0),
        GridTrackSize::LengthPx(20.0),
    ]);
    let column_output = TaffyLayoutEngine.compute(&columns).unwrap();
    assert_eq!(
        final_axis_track_sizes(&column_output, LayoutGridTrackSizingAxis::Columns),
        [24.0, 10.0, 20.0, 10.0]
    );
}

#[test]
fn track_sizing_auto_implicit_track_uses_item_block_size() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(12.0)],
        2,
        LayoutGap::default(),
    );
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::TrackSizing;
    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert_eq!(
        axis_track_sizes_at_stage(
            &output,
            LayoutGridTrackSizingAxis::Rows,
            LayoutGridTrackSizingStageKind::IntrinsicSizingComplete,
        ),
        [12.0, 10.0]
    );
}

#[test]
fn track_sizing_sizes_explicitly_placed_positive_implicit_rows() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(12.0)],
        1,
        LayoutGap::default(),
    );
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::TrackSizing;
    input.nodes[1].style.height = LayoutDimension::Auto;
    input.grid_containers.get_mut(&id(1)).unwrap().auto_rows =
        GridAutoRows::TrackList(vec![GridTrackSize::LengthPx(30.0)]);
    input.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(2),
            row_end: GridPlacementInput::Line(3),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );
    let index = input
        .nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (node.id, index))
        .collect();
    let normalized = crate::grid_placement::normalize_grid_layout(&input, &index).unwrap();
    assert_eq!(
        normalized.expanded_rows[&id(1)],
        [GridTrackSize::LengthPx(12.0), GridTrackSize::LengthPx(30.0),]
    );
    let output = TaffyLayoutEngine.compute(&input).unwrap();

    let frame = output.frames[&id(2)];
    assert_eq!((frame.y, frame.height), (12.0, 30.0));
}

#[test]
fn track_sizing_sizes_explicitly_placed_positive_implicit_columns() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(24.0)],
        vec![GridTrackSize::LengthPx(40.0)],
        1,
        LayoutGap::default(),
    );
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::TrackSizing;
    input.nodes[1].style.width = LayoutDimension::Auto;
    input.grid_containers.get_mut(&id(1)).unwrap().auto_columns =
        crate::GridAutoColumns::TrackList(vec![GridTrackSize::LengthPx(30.0)]);
    input.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(2),
            column_end: GridPlacementInput::Line(3),
        },
    );
    let index = input
        .nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (node.id, index))
        .collect();
    let normalized = crate::grid_placement::normalize_grid_layout(&input, &index).unwrap();
    assert_eq!(
        normalized.expanded_columns[&id(1)],
        [GridTrackSize::LengthPx(24.0), GridTrackSize::LengthPx(30.0),]
    );
    let output = TaffyLayoutEngine.compute(&input).unwrap();

    let frame = output.frames[&id(2)];
    assert_eq!((frame.x, frame.width), (24.0, 30.0));
}

#[test]
fn track_sizing_places_row_locked_item_on_positive_implicit_row() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(12.0)],
        1,
        LayoutGap::default(),
    );
    input
        .grid_containers
        .get_mut(&id(1))
        .unwrap()
        .placement_mode = GridPlacementMode::TrackSizing;
    input.nodes[1].style.height = LayoutDimension::Auto;
    input.grid_containers.get_mut(&id(1)).unwrap().auto_rows =
        GridAutoRows::TrackList(vec![GridTrackSize::LengthPx(30.0)]);
    input.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(2),
            row_end: GridPlacementInput::Line(3),
            column_start: GridPlacementInput::Auto,
            column_end: GridPlacementInput::Auto,
        },
    );
    let index = input
        .nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (node.id, index))
        .collect();
    let normalized = crate::grid_placement::normalize_grid_layout(&input, &index).unwrap();
    assert_eq!(
        normalized.expanded_rows[&id(1)],
        [GridTrackSize::LengthPx(12.0), GridTrackSize::LengthPx(30.0)]
    );
    let frame = TaffyLayoutEngine.compute(&input).unwrap().frames[&id(2)];
    assert_eq!((frame.y, frame.height), (12.0, 30.0));
}

#[test]
fn track_sizing_places_column_locked_item_on_positive_implicit_column() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(24.0)],
        vec![GridTrackSize::LengthPx(40.0)],
        1,
        LayoutGap::default(),
    );
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.auto_flow = crate::GridAutoFlow::Column;
    container.auto_columns = crate::GridAutoColumns::TrackList(vec![GridTrackSize::LengthPx(30.0)]);
    input.nodes[1].style.width = LayoutDimension::Auto;
    input.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Auto,
            row_end: GridPlacementInput::Auto,
            column_start: GridPlacementInput::Line(2),
            column_end: GridPlacementInput::Line(3),
        },
    );
    let index = input
        .nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (node.id, index))
        .collect();
    let normalized = crate::grid_placement::normalize_grid_layout(&input, &index).unwrap();
    assert_eq!(
        normalized.expanded_columns[&id(1)],
        [GridTrackSize::LengthPx(24.0), GridTrackSize::LengthPx(30.0)]
    );
    let frame = TaffyLayoutEngine.compute(&input).unwrap().frames[&id(2)];
    assert_eq!((frame.x, frame.width), (24.0, 30.0));
}

#[test]
fn track_sizing_combines_explicit_auto_and_positive_implicit_tracks_with_text() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(100.0)],
        vec![GridTrackSize::Auto],
        3,
        LayoutGap::default(),
    );
    input.viewport = crate::Viewport {
        width: 100.0,
        height: 200.0,
    };
    input.root_sizing = crate::RootSizingPolicy::ResolveWithin;
    input.nodes[0].style.width = LayoutDimension::Fixed(100.0);
    input.nodes[0].style.height = LayoutDimension::Auto;
    for item in &mut input.nodes[1..] {
        item.style.width = LayoutDimension::Auto;
        item.style.height = LayoutDimension::Auto;
    }

    let root = id(1);
    let a = id(2);
    let b = id(3);
    let c = id(4);
    let b_content = id(5);
    input.nodes[0].children = vec![a, b, c];
    input.nodes[2].children = vec![b_content];
    input.nodes.push(LayoutNode {
        id: b_content,
        children: Vec::new(),
        style: LayoutStyle {
            display: LayoutDisplay::Block,
            width: LayoutDimension::Fixed(50.0),
            height: LayoutDimension::Auto,
            ..LayoutStyle::default()
        },
    });

    let container = input.grid_containers.get_mut(&root).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.auto_rows = crate::GridAutoRows::Auto;
    container.auto_columns = crate::GridAutoColumns::Auto;
    input.grid_items = BTreeMap::from([
        (
            a,
            GridItemInput {
                row_start: GridPlacementInput::Line(1),
                row_end: GridPlacementInput::Line(2),
                column_start: GridPlacementInput::Line(1),
                column_end: GridPlacementInput::Line(2),
            },
        ),
        (
            b,
            GridItemInput {
                row_start: GridPlacementInput::Line(1),
                row_end: GridPlacementInput::Line(2),
                column_start: GridPlacementInput::Line(2),
                column_end: GridPlacementInput::Line(3),
            },
        ),
        (
            c,
            GridItemInput {
                row_start: GridPlacementInput::Line(2),
                row_end: GridPlacementInput::Line(3),
                column_start: GridPlacementInput::Line(1),
                column_end: GridPlacementInput::Line(2),
            },
        ),
    ]);

    let paragraph = |owner, source| crate::ParagraphInput {
        node: owner,
        sources: vec![crate::TextSource {
            node: source,
            inline_ancestors: Vec::new(),
            text: "\u{00a0}".to_owned(),
        }],
        style: crate::ParagraphStyle {
            writing_mode: spinon_core::WritingMode::HorizontalTb,
            text_orientation: spinon_core::TextOrientation::Mixed,
            font_family: "Ahem".to_owned(),
            font_size: 10.0,
            line_height: 10.0,
            white_space: crate::WhiteSpaceMode::Normal,
            ..crate::ParagraphStyle::default()
        },
        inline_styles: BTreeMap::new(),
        inline_run_styles: BTreeMap::new(),
    };
    let input = input
        .with_text_paragraphs([
            paragraph(a, id(20)),
            paragraph(b_content, id(21)),
            paragraph(c, id(22)),
        ])
        .unwrap();
    let mut text_engine = crate::TextLayoutEngine::new_without_system_fonts();
    text_engine
        .register_font(
            "Ahem",
            include_bytes!("../../../tests/fixtures/resources/c15/Ahem.ttf"),
        )
        .unwrap();

    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &text_engine)
        .unwrap();

    assert_eq!(
        final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Columns),
        [100.0, 50.0]
    );
    assert_eq!(
        final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Rows),
        [10.0, 10.0]
    );
    assert_eq!(
        (output.frames[&root].width, output.frames[&root].height),
        (100.0, 20.0)
    );
    assert_eq!(
        (
            output.frames[&a].x,
            output.frames[&a].y,
            output.frames[&a].width,
            output.frames[&a].height,
        ),
        (0.0, 0.0, 100.0, 10.0)
    );
    assert_eq!(
        (
            output.frames[&b].x,
            output.frames[&b].y,
            output.frames[&b].width,
            output.frames[&b].height,
        ),
        (100.0, 0.0, 50.0, 10.0)
    );
    assert_eq!(
        (
            output.frames[&c].x,
            output.frames[&c].y,
            output.frames[&c].width,
            output.frames[&c].height,
        ),
        (0.0, 10.0, 100.0, 10.0)
    );
    assert_eq!(
        (
            output.frames[&b_content].x,
            output.frames[&b_content].y,
            output.frames[&b_content].width,
            output.frames[&b_content].height,
        ),
        (100.0, 0.0, 50.0, 10.0)
    );

    let row_pass = output.grid_track_sizing_traces[0]
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .unwrap();
    assert_eq!(
        row_pass
            .contributions
            .iter()
            .filter(|contribution| contribution.kind
                == crate::LayoutGridTrackSizingContributionKind::Minimum)
            .map(|contribution| contribution.item)
            .collect::<Vec<_>>(),
        [a, b, c]
    );
    assert!(row_pass.contributions.iter().all(|contribution| {
        contribution.kind != crate::LayoutGridTrackSizingContributionKind::Minimum
            || contribution.value == 10.0
    }));
}

#[test]
fn track_sizing_keeps_auto_track_pattern_aligned_across_explicit_boundary_spans() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(12.0)],
        1,
        LayoutGap::default(),
    );
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.auto_rows = GridAutoRows::TrackList(vec![
        GridTrackSize::LengthPx(10.0),
        GridTrackSize::LengthPx(20.0),
    ]);
    input.nodes[1].style.height = LayoutDimension::Auto;
    input.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(4),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert_eq!(
        final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Rows),
        [12.0, 10.0, 20.0]
    );
    assert_eq!(output.frames[&id(2)].height, 42.0);
}

#[test]
fn track_sizing_supports_negative_implicit_lines_and_ignores_hidden_positive_items() {
    let mut negative = grid_input(
        vec![GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(12.0)],
        1,
        LayoutGap::default(),
    );
    let container = negative.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.auto_rows = GridAutoRows::TrackList(vec![GridTrackSize::LengthPx(30.0)]);
    negative.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(-3),
            row_end: GridPlacementInput::Line(-2),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );
    let negative_output = TaffyLayoutEngine.compute(&negative).unwrap();
    assert_eq!(
        final_axis_track_sizes(&negative_output, LayoutGridTrackSizingAxis::Rows),
        [30.0, 12.0]
    );
    assert_eq!(negative_output.frames[&id(2)].y, 0.0);

    let mut hidden = grid_input(
        vec![GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(12.0)],
        1,
        LayoutGap::default(),
    );
    let container = hidden.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.auto_rows = GridAutoRows::TrackList(vec![GridTrackSize::LengthPx(30.0)]);
    hidden.nodes[1].style.display = LayoutDisplay::None;
    hidden.grid_items.insert(
        id(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(2),
            row_end: GridPlacementInput::Line(3),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );

    let index = hidden
        .nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (node.id, index))
        .collect();
    let normalized = crate::grid_placement::normalize_grid_layout(&hidden, &index).unwrap();
    assert!(!normalized.expanded_rows.contains_key(&id(1)));
    let output = TaffyLayoutEngine.compute(&hidden).unwrap();
    assert_eq!(
        final_axis_track_sizes(&output, LayoutGridTrackSizingAxis::Rows),
        [12.0]
    );
    assert_eq!(
        (output.frames[&id(2)].width, output.frames[&id(2)].height),
        (0.0, 0.0)
    );
}

#[test]
fn track_sizing_accepts_fixed_px_auto_track_lists_with_auto_repeat() {
    let mut columns = grid_input(
        vec![GridTrackSize::LengthPx(80.0)],
        vec![GridTrackSize::LengthPx(12.0)],
        0,
        LayoutGap::default(),
    );
    let container = columns.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.columns_auto_repeat = Some(crate::GridAutoRepeat {
        mode: crate::GridAutoRepeatMode::AutoFill,
        start: 0,
        end: 1,
        line_names: Vec::new(),
    });
    container.auto_columns = crate::GridAutoColumns::TrackList(vec![GridTrackSize::LengthPx(10.0)]);
    assert!(TaffyLayoutEngine.compute(&columns).is_ok());

    let mut rows = grid_input(
        vec![GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(80.0)],
        0,
        LayoutGap::default(),
    );
    let container = rows.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.rows_auto_repeat = Some(crate::GridAutoRepeat {
        mode: crate::GridAutoRepeatMode::AutoFit,
        start: 0,
        end: 1,
        line_names: Vec::new(),
    });
    container.auto_rows = GridAutoRows::TrackList(vec![GridTrackSize::LengthPx(10.0)]);
    assert!(TaffyLayoutEngine.compute(&rows).is_ok());
}

#[test]
fn track_sizing_rejects_empty_invalid_and_oversized_auto_track_lists() {
    let mut empty = grid_input(
        vec![GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(12.0)],
        0,
        LayoutGap::default(),
    );
    let container = empty.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.auto_rows = GridAutoRows::TrackList(Vec::new());
    assert!(matches!(
        TaffyLayoutEngine.compute(&empty),
        Err(LayoutError::InvalidGridTrackList {
            node,
            property: "grid-auto-rows",
        }) if node == id(1)
    ));

    let mut invalid = grid_input(
        vec![GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(12.0)],
        0,
        LayoutGap::default(),
    );
    let container = invalid.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.auto_columns = crate::GridAutoColumns::TrackList(vec![GridTrackSize::LengthPx(-1.0)]);
    assert!(matches!(
        TaffyLayoutEngine.compute(&invalid),
        Err(LayoutError::InvalidGridTrack {
            node,
            property: "grid-auto-columns",
        }) if node == id(1)
    ));

    let mut oversized = grid_input(
        vec![GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(12.0)],
        0,
        LayoutGap::default(),
    );
    let container = oversized.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.auto_rows = GridAutoRows::TrackList(vec![
        GridTrackSize::LengthPx(1.0);
        super::GRID_MAX_TRACKS + 1
    ]);
    assert!(matches!(
        TaffyLayoutEngine.compute(&oversized),
        Err(LayoutError::GridTrackLimitExceeded {
            node,
            property: "grid-auto-rows",
            count,
            maximum,
        }) if node == id(1) && count == super::GRID_MAX_TRACKS + 1 && maximum == super::GRID_MAX_TRACKS
    ));
}

#[test]
fn previous_auto_placement_profile_rejects_intrinsic_auto_track_lists() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(12.0)],
        1,
        LayoutGap::default(),
    );
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::DenseAutoPlacement;
    container.auto_rows = GridAutoRows::TrackList(vec![GridTrackSize::MinContent]);

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::UnsupportedGridAutoRows { node, .. }) if node == id(1)
    ));
}

#[test]
fn expanded_track_allocation_failure_is_atomic_and_recovers() {
    let mut input = grid_input(
        vec![GridTrackSize::LengthPx(40.0)],
        vec![GridTrackSize::LengthPx(12.0)],
        2,
        LayoutGap::default(),
    );
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.placement_mode = GridPlacementMode::TrackSizing;
    container.auto_rows = GridAutoRows::TrackList(vec![GridTrackSize::MinContent]);

    let error = {
        let _guard = inject_failure_once(TestGridReservationSite::ExpandedRows);
        TaffyLayoutEngine.compute(&input).unwrap_err()
    };
    assert!(matches!(
        error,
        LayoutError::GridAllocationFailure {
            node,
            property: "grid-template-rows",
        } if node == id(1)
    ));
    assert_eq!(
        final_axis_track_sizes(
            &TaffyLayoutEngine.compute(&input).unwrap(),
            LayoutGridTrackSizingAxis::Rows,
        ),
        [12.0, 10.0]
    );
}

fn final_axis_track_sizes(
    output: &crate::LayoutOutput,
    axis: LayoutGridTrackSizingAxis,
) -> Vec<f32> {
    let pass = output.grid_track_sizing_traces[0]
        .passes
        .iter()
        .find(|pass| pass.axis == axis)
        .unwrap();
    pass.stages
        .last()
        .unwrap()
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect()
}

fn axis_track_sizes_at_stage(
    output: &crate::LayoutOutput,
    axis: LayoutGridTrackSizingAxis,
    stage_kind: LayoutGridTrackSizingStageKind,
) -> Vec<f32> {
    let pass = output.grid_track_sizing_traces[0]
        .passes
        .iter()
        .find(|pass| pass.axis == axis)
        .unwrap();
    pass.stages
        .iter()
        .find(|stage| stage.stage == stage_kind)
        .unwrap()
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect()
}

#[path = "grid_auto_track_sizing_tests/auto_repeat.rs"]
mod auto_repeat;
