use std::collections::BTreeMap;

use spinon_core::{ChangeBatch, EnvironmentRevision, NodeId, Operation, StyleRevision, Tree};

use crate::{
    LayoutDimension, LayoutDisplay, LayoutEngine, LayoutInput, LayoutStyle, ParagraphInput,
    ParagraphStyle, TaffyLayoutEngine, TextLayoutEngine, TextSource, Viewport, WhiteSpaceMode,
    WordBreakMode,
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
            font_family: "Ahem".to_owned(),
            font_size: 10.0,
            line_height: 10.0,
            white_space,
            word_break: WordBreakMode::Normal,
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
