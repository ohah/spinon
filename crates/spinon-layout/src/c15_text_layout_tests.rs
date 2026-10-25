use std::collections::BTreeMap;

use spinon_core::{ChangeBatch, EnvironmentRevision, NodeId, Operation, StyleRevision, Tree};

use crate::{
    GridContainerInput, GridItemInput, GridPlacementInput, GridPlacementMode, GridTrackSize,
    LayoutDimension, LayoutDisplay, LayoutEngine, LayoutGridItemIntrinsicSource, LayoutInput,
    LayoutIntrinsicMeasurementSource, LayoutNode, LayoutStyle, ParagraphInput, ParagraphStyle,
    TaffyLayoutEngine, TextLayoutEngine, TextSource, Viewport, WhiteSpaceMode, WordBreakMode,
};

const AHEM: &[u8] = include_bytes!("../../../tests/fixtures/resources/c15/Ahem.ttf");

fn node(value: u64) -> NodeId {
    NodeId::new(value).unwrap()
}

fn text_engine() -> TextLayoutEngine {
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).unwrap();
    engine
}

fn input(width: f32, text: &str, white_space: WhiteSpaceMode) -> LayoutInput {
    let mut tree = Tree::new();
    let mut batch = ChangeBatch::new(tree.revision());
    batch
        .push(Operation::Create {
            id: node(1),
            tag: "div".to_owned(),
        })
        .push(Operation::Insert {
            id: node(1),
            parent: None,
            index: 0,
        })
        .push(Operation::Create {
            id: node(2),
            tag: "p".to_owned(),
        })
        .push(Operation::Insert {
            id: node(2),
            parent: Some(node(1)),
            index: 0,
        });
    tree.commit(batch).unwrap();
    let styles = BTreeMap::from([
        (
            node(1),
            LayoutStyle {
                width: LayoutDimension::Fixed(100.0),
                height: LayoutDimension::Fixed(80.0),
                display: LayoutDisplay::Block,
                ..LayoutStyle::default()
            },
        ),
        (
            node(2),
            LayoutStyle {
                width: LayoutDimension::Fixed(width),
                display: LayoutDisplay::Block,
                ..LayoutStyle::default()
            },
        ),
    ]);
    LayoutInput::from_tree(
        &tree,
        Viewport {
            width: 100.0,
            height: 80.0,
        },
        &styles,
        StyleRevision::INITIAL,
        EnvironmentRevision::INITIAL,
    )
    .unwrap()
    .with_text_paragraphs([ParagraphInput {
        node: node(2),
        sources: vec![TextSource {
            node: node(3),
            inline_ancestors: Vec::new(),
            text: text.to_owned(),
        }],
        style: ParagraphStyle {
            writing_mode: spinon_core::WritingMode::HorizontalTb,
            text_orientation: spinon_core::TextOrientation::Mixed,
            font_family: "Ahem".to_owned(),
            font_size: 10.0,
            line_height: 10.0,
            white_space,
            word_break: WordBreakMode::Normal,
            direction: spinon_text_layout::TextDirection::Auto,
            line_break: spinon_text_layout::LineBreakMode::Strict,
            ligatures: spinon_text_layout::LigatureMode::Normal,
            ..ParagraphStyle::default()
        },
        inline_styles: BTreeMap::new(),
        inline_run_styles: BTreeMap::new(),
    }])
    .unwrap()
}

#[test]
fn taffy_measure_and_final_layout_use_the_same_definite_content_width() {
    let engine = text_engine();
    let cases = [
        (input(30.0, "XX XX", WhiteSpaceMode::Normal), 20.0, 2),
        (input(15.0, "XX", WhiteSpaceMode::Normal), 20.0, 1),
        (input(30.0, "XX XX", WhiteSpaceMode::NoWrap), 50.0, 1),
    ];

    for (input, text_width, line_count) in cases {
        let output = TaffyLayoutEngine
            .compute_with_text_layout_engine(&input, &engine)
            .unwrap();
        assert!(output.intrinsic_measurements.is_empty());
        let frame = output.frames[&node(2)];
        let text = &output.text_layouts[&node(2)];

        assert_eq!(text.revision, output.revision);
        assert_eq!(text.input.node, node(2));
        let paragraph_input = input.text_paragraphs[&node(2)].paragraph().unwrap();
        assert_eq!(text.input.sources, paragraph_input.sources);
        assert_eq!(text.input.style, paragraph_input.style);
        assert_eq!(frame.width, text.layout.available_width);
        assert_eq!(text.layout.available_width.to_bits(), frame.width.to_bits());
        assert_eq!(text.layout.lines.len(), line_count);
        assert!((text.layout.width - text_width).abs() <= 0.05);
        assert!((frame.height - text.layout.height).abs() <= 0.05);
        assert_eq!(text.layout.font_registry_revision.0, 1);
        assert_eq!(output.revision, input.revision());
    }
}

#[test]
fn grid_text_intrinsic_measurement_records_font_and_layout_revisions() {
    let mut grid_input = input(100.0, "XX XXXXXXX", WhiteSpaceMode::Normal);
    grid_input.nodes[0].style.display = LayoutDisplay::Grid;
    // The root must continue to match the 80px viewport height.
    grid_input.nodes[0].style.height = LayoutDimension::Fixed(80.0);
    grid_input.nodes[1].style.height = LayoutDimension::Auto;
    grid_input.grid_containers.insert(
        node(1),
        GridContainerInput {
            columns: vec![GridTrackSize::LengthPx(100.0)],
            rows: vec![GridTrackSize::LengthPx(10.0)],
            placement_mode: GridPlacementMode::TemplateGrid,
            ..GridContainerInput::default()
        },
    );
    grid_input.grid_items.insert(
        node(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );

    let engine = text_engine();
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&grid_input, &engine)
        .unwrap();
    let measurement = output
        .intrinsic_measurements
        .iter()
        .find(|measurement| measurement.node == node(2))
        .expect("Grid text measure callback is recorded");

    assert_eq!(measurement.revision, output.revision);
    assert!(matches!(
        measurement.source,
        LayoutIntrinsicMeasurementSource::Text {
            font_registry_revision: revision,
        } if revision.0 == 1
    ));
    assert_eq!(measurement.item_style, grid_input.nodes[1].style);
    assert_eq!(
        measurement.available_space.width,
        crate::LayoutIntrinsicMeasureSpace::Definite(100.0)
    );
    assert_eq!(
        measurement.available_space.height,
        crate::LayoutIntrinsicMeasureSpace::Definite(10.0)
    );
    assert_eq!(
        measurement.known_dimensions,
        crate::LayoutIntrinsicMeasureKnownDimensions::default()
    );
    assert_eq!(
        measurement.parent_size,
        crate::LayoutIntrinsicMeasureParentSize {
            width: Some(100.0),
            height: Some(10.0),
        }
    );
    assert_eq!(measurement.content_size.width, 100.0);
    assert_eq!(measurement.content_size.height, 10.0);
}

#[test]
fn grid_text_intrinsic_input_preserves_minmax_beyond_definite_callback_trace() {
    let mut grid_input = input(100.0, "XX XXXXXXX", WhiteSpaceMode::Normal);
    let hidden = node(4);
    grid_input.nodes[0].children.insert(0, hidden);
    grid_input.nodes.insert(
        1,
        LayoutNode {
            id: hidden,
            children: Vec::new(),
            style: LayoutStyle {
                display: LayoutDisplay::None,
                ..LayoutStyle::default()
            },
        },
    );
    grid_input.nodes[2].style.box_sizing = crate::LayoutBoxSizing::ContentBox;
    grid_input.nodes[2].style.padding.left = crate::LayoutLengthPercentage::length(4.0);
    grid_input.nodes[2].style.padding.right = crate::LayoutLengthPercentage::length(4.0);
    grid_input.nodes[2].style.border.left = 1.0;
    grid_input.nodes[2].style.border.right = 1.0;
    grid_input.nodes[0].style.display = LayoutDisplay::Grid;
    grid_input.nodes[0].style.height = LayoutDimension::Fixed(80.0);
    grid_input.nodes[1].style.height = LayoutDimension::Fixed(10.0);
    grid_input.grid_containers.insert(
        node(1),
        GridContainerInput {
            columns: vec![GridTrackSize::LengthPx(100.0)],
            rows: vec![GridTrackSize::LengthPx(10.0)],
            placement_mode: GridPlacementMode::TemplateGrid,
            ..GridContainerInput::default()
        },
    );
    grid_input.grid_items.insert(
        node(2),
        GridItemInput {
            row_start: GridPlacementInput::Line(1),
            row_end: GridPlacementInput::Line(2),
            column_start: GridPlacementInput::Line(1),
            column_end: GridPlacementInput::Line(2),
        },
    );

    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&grid_input, &text_engine())
        .unwrap();

    let callback_measurements = output
        .intrinsic_measurements
        .iter()
        .filter(|measurement| measurement.node == node(2))
        .collect::<Vec<_>>();
    assert!(!callback_measurements.is_empty());
    assert!(callback_measurements.iter().all(|measurement| {
        matches!(
            measurement.available_space.width,
            crate::LayoutIntrinsicMeasureSpace::Definite(_)
        ) && matches!(
            measurement.available_space.height,
            crate::LayoutIntrinsicMeasureSpace::Definite(_)
        )
    }));
    let item = output
        .grid_item_intrinsic_inputs
        .iter()
        .find(|item| item.node == node(2))
        .expect("Grid intrinsic input is collected independently of callback trace contents");
    assert_eq!(item.grid_container, node(1));
    assert_eq!(item.source_order, 1);
    assert_eq!(item.revision, output.revision);
    assert_eq!(item.item_style, grid_input.nodes[2].style);
    assert!(
        !output
            .grid_item_intrinsic_inputs
            .iter()
            .any(|item| item.node == hidden)
    );
    let LayoutGridItemIntrinsicSource::Text { sizes } = &item.source else {
        panic!("text Grid item must retain Parley intrinsic sizes");
    };
    assert_eq!(
        (sizes.min_content(), sizes.min_content_height()),
        (70.0, 20.0)
    );
    assert_eq!(
        (sizes.max_content(), sizes.max_content_height()),
        (100.0, 10.0)
    );
    assert_eq!(sizes.font_registry_revision().0, 1);

    let contribution = item.contribution_input().unwrap();
    assert_eq!(contribution.grid_container, node(1));
    assert_eq!(contribution.source_order, 1);
    assert_eq!(contribution.revision, output.revision);
    assert_eq!(contribution.item_style, grid_input.nodes[2].style);
    assert_eq!(
        contribution.item_style.box_sizing,
        crate::LayoutBoxSizing::ContentBox
    );
    assert_eq!(contribution.width.min_content, Some(70.0));
    assert_eq!(contribution.width.max_content, Some(100.0));
    assert_eq!(contribution.width.min_size, crate::LayoutDimension::Auto);
    assert_eq!(
        contribution.width.padding_start,
        crate::LayoutLengthPercentage::length(4.0)
    );
    assert_eq!(
        contribution.width.padding_end,
        crate::LayoutLengthPercentage::length(4.0)
    );
    assert_eq!(
        (
            contribution.width.border_start,
            contribution.width.border_end
        ),
        (1.0, 1.0)
    );
    assert_eq!(
        contribution
            .width
            .outer_size(contribution.width.min_content, node(2), "width")
            .unwrap(),
        Some(80.0)
    );
    assert_eq!(
        contribution
            .width
            .outer_size(contribution.width.max_content, node(2), "width")
            .unwrap(),
        Some(110.0)
    );
    let mut unresolved = contribution.width;
    unresolved.padding_start = crate::LayoutLengthPercentage::percent(0.25);
    assert!(matches!(
        unresolved.outer_size(Some(70.0), node(2), "width"),
        Err(crate::LayoutError::UnsupportedIntrinsicSizing { node: actual, .. })
            if actual == node(2)
    ));
    unresolved.padding_start = crate::LayoutLengthPercentage::length(4.0);
    unresolved.margin_start = crate::LayoutLengthPercentage::Auto;
    assert_eq!(
        unresolved.outer_size(Some(70.0), node(2), "width").unwrap(),
        Some(80.0)
    );
    assert_eq!(contribution.height.min_content, Some(20.0));
    assert_eq!(contribution.height.max_content, Some(10.0));
}

#[test]
fn out_of_span_auto_track_does_not_change_text_item_minimum() {
    fn layout(
        columns: Vec<GridTrackSize>,
        column_start: i32,
        column_end: i32,
    ) -> crate::LayoutOutput {
        let mut grid_input = input(100.0, "XXXXXXXX", WhiteSpaceMode::NoWrap);
        grid_input.nodes[0].style.display = LayoutDisplay::Grid;
        grid_input.nodes[1].style.width = LayoutDimension::Auto;
        grid_input.nodes[1].style.height = LayoutDimension::Auto;
        grid_input.grid_containers.insert(
            node(1),
            GridContainerInput {
                columns,
                rows: vec![GridTrackSize::LengthPx(10.0)],
                placement_mode: GridPlacementMode::TrackSizing,
                ..GridContainerInput::default()
            },
        );
        grid_input.grid_items.insert(
            node(2),
            GridItemInput {
                row_start: GridPlacementInput::Line(1),
                row_end: GridPlacementInput::Line(2),
                column_start: GridPlacementInput::Line(column_start),
                column_end: GridPlacementInput::Line(column_end),
            },
        );

        TaffyLayoutEngine
            .compute_with_text_layout_engine(&grid_input, &text_engine())
            .unwrap()
    }

    fn item_width(columns: Vec<GridTrackSize>, column_start: i32, column_end: i32) -> f32 {
        layout(columns, column_start, column_end).frames[&node(2)].width
    }

    let fixed_only = item_width(vec![GridTrackSize::LengthPx(20.0)], 1, 2);
    let auto_outside_span = item_width(
        vec![GridTrackSize::LengthPx(20.0), GridTrackSize::Auto],
        1,
        2,
    );
    let flex_outside_span = item_width(
        vec![GridTrackSize::LengthPx(20.0), GridTrackSize::Fraction(1.0)],
        1,
        2,
    );
    let auto_inside_layout = layout(
        vec![GridTrackSize::Auto, GridTrackSize::LengthPx(20.0)],
        1,
        2,
    );
    let auto_inside_span = auto_inside_layout.frames[&node(2)].width;

    assert_eq!(fixed_only, 20.0);
    assert_eq!(auto_outside_span, fixed_only);
    assert_eq!(flex_outside_span, fixed_only);
    assert_eq!(auto_inside_span, 80.0);

    let auto_minimum = auto_inside_layout
        .grid_track_sizing_traces
        .iter()
        .flat_map(|trace| trace.passes.iter())
        .flat_map(|pass| pass.contributions.iter())
        .find(|contribution| {
            contribution.item == node(2)
                && contribution.kind == crate::LayoutGridTrackSizingContributionKind::Minimum
        })
        .expect("the auto track should request a minimum contribution");
    assert_eq!(auto_minimum.value, 80.0);
    assert!(matches!(
        auto_minimum.minimum_source,
        Some(
            crate::LayoutGridTrackSizingMinimumContributionSource::AutomaticMinimum {
                spans_auto_min_track: true,
                spans_multiple_tracks: false,
                spans_flexible_max_track: false,
                content_based: true,
                ..
            }
        )
    ));
}

#[test]
fn zero_width_block_keeps_text_advance_outside_its_box() {
    let zero_input = input(0.0, "X", WhiteSpaceMode::Normal);
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&zero_input, &text_engine())
        .unwrap();

    let frame = output.frames[&node(2)];
    let text = &output.text_layouts[&node(2)].layout;
    assert_eq!(frame.width, 0.0);
    assert_eq!(text.available_width, 0.0);
    assert_eq!(text.lines.len(), 1);
    assert!((text.width - 10.0).abs() <= 0.05);
    assert!(text.inline_fragments.iter().any(|fragment| {
        fragment.node == node(3) && (fragment.rect.width - 10.0).abs() <= 0.05
    }));

    let negative_zero_input = input(-0.0, "X", WhiteSpaceMode::Normal);
    let negative_zero_output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&negative_zero_input, &text_engine())
        .unwrap();
    assert_eq!(
        negative_zero_output.text_layouts[&node(2)]
            .layout
            .available_width
            .to_bits(),
        0.0_f32.to_bits()
    );
}

#[test]
fn inline_source_display_is_rejected_as_a_standalone_layout_box() {
    let mut input = input(100.0, "X", WhiteSpaceMode::Normal);
    input.nodes[1].style.display = LayoutDisplay::Inline;

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::InvalidStyle {
            node: node_id,
            field: "display:inline"
        }) if node_id == node(2)
    ));
}

#[test]
fn taffy_requires_a_shaper_and_rejects_mismatched_paragraph_owner() {
    let without_engine = input(30.0, "X", WhiteSpaceMode::Normal);
    assert!(matches!(
        TaffyLayoutEngine.compute(&without_engine),
        Err(crate::LayoutError::MissingTextLayoutEngine(id)) if id == node(2)
    ));

    let mut mismatched = input(30.0, "X", WhiteSpaceMode::Normal);
    let crate::TextParagraphInput::Paragraph(mut paragraph) =
        mismatched.text_paragraphs.remove(&node(2)).unwrap()
    else {
        panic!("plain fixture should contain a paragraph input");
    };
    paragraph.node = node(999);
    mismatched
        .text_paragraphs
        .insert(node(2), crate::TextParagraphInput::Paragraph(paragraph));
    assert!(matches!(
        TaffyLayoutEngine.compute_with_text_layout_engine(&mismatched, &text_engine()),
        Err(crate::LayoutError::UnsupportedTextParagraph { node: actual, .. }) if actual == node(2)
    ));
}

#[path = "c15_text_layout_tests/host_document.rs"]
mod host_document;

#[path = "c15_text_layout_tests/c15_2.rs"]
mod c15_2;

#[path = "c15_text_layout_tests/c15_2b.rs"]
mod c15_2b;

#[path = "c15_text_layout_tests/grid_chrome_oracle.rs"]
mod grid_chrome_oracle;
