use std::collections::BTreeMap;

use spinon_core::{
    DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument, HostNodeHandle,
    HostParent, NodeId, OwnerId, StyleRevision,
};
use spinon_text_layout::{ParagraphStyle, TextLayoutEngine, TextLayoutSnapshot, WhiteSpaceMode};

use crate::{
    LayoutBorder, LayoutDimension, LayoutDisplay, LayoutEdges, LayoutInput, LayoutLengthPercentage,
    LayoutStyle, TaffyLayoutEngine, Viewport, WordBreakMode,
};

const AHEM: &[u8] = include_bytes!("../../../../tests/fixtures/resources/c15/Ahem.ttf");
const HTML: &str = "http://www.w3.org/1999/xhtml";

struct NestedFixture {
    document: HostDocument,
    root: HostNodeHandle,
    paragraph: HostNodeHandle,
    spans: Vec<HostNodeHandle>,
    text: HostNodeHandle,
}

fn nested_fixture(text: &str, depth: usize) -> NestedFixture {
    let mut document = HostDocument::new().unwrap();
    let owner = OwnerId::new(1).unwrap();
    let root = document.reserve_node_handle().unwrap();
    let paragraph = document.reserve_node_handle().unwrap();
    let spans = (0..depth)
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let text_node = document.reserve_node_handle().unwrap();
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    batch
        .push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        })
        .push(DocumentOperation::CreateElement {
            node: paragraph,
            namespace: HTML.to_owned(),
            local_name: "p".to_owned(),
        });
    for span in &spans {
        batch.push(DocumentOperation::CreateElement {
            node: *span,
            namespace: HTML.to_owned(),
            local_name: "span".to_owned(),
        });
    }
    batch
        .push(DocumentOperation::CreateText {
            node: text_node,
            data: text.into(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: paragraph,
            before: None,
        });
    let mut parent = paragraph;
    for span in &spans {
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(parent),
            node: *span,
            before: None,
        });
        parent = *span;
    }
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(parent),
        node: text_node,
        before: None,
    });
    document.commit(batch).unwrap();
    NestedFixture {
        document,
        root,
        paragraph,
        spans,
        text: text_node,
    }
}

fn inline_style(border: f32, padding: f32) -> LayoutStyle {
    LayoutStyle {
        display: LayoutDisplay::Inline,
        border: LayoutBorder {
            top: border,
            right: border,
            bottom: border,
            left: border,
        },
        padding: LayoutEdges {
            top: LayoutLengthPercentage::length(padding),
            right: LayoutLengthPercentage::length(padding),
            bottom: LayoutLengthPercentage::length(padding),
            left: LayoutLengthPercentage::length(padding),
        },
        ..LayoutStyle::default()
    }
}

fn paragraph_input(
    fixture: &NestedFixture,
    width: f32,
    span_styles: &[LayoutStyle],
) -> LayoutInput {
    let mut styles = BTreeMap::from([
        (
            fixture.root.id(),
            LayoutStyle {
                width: LayoutDimension::Fixed(width),
                height: LayoutDimension::Fixed(80.0),
                display: LayoutDisplay::Block,
                ..LayoutStyle::default()
            },
        ),
        (
            fixture.paragraph.id(),
            LayoutStyle {
                width: LayoutDimension::Fixed(width),
                display: LayoutDisplay::Block,
                ..LayoutStyle::default()
            },
        ),
    ]);
    for (&span, &style) in fixture.spans.iter().zip(span_styles) {
        styles.insert(span.id(), style);
    }
    let paragraph_styles = BTreeMap::from([(
        fixture.paragraph.id(),
        ParagraphStyle {
            overflow_wrap: Default::default(),
            writing_mode: spinon_core::WritingMode::HorizontalTb,
            text_orientation: spinon_core::TextOrientation::Mixed,
            font_family: "Ahem".to_owned(),
            font_size: 10.0,
            line_height: 10.0,
            white_space: WhiteSpaceMode::Normal,
            word_break: WordBreakMode::Normal,
            direction: spinon_text_layout::TextDirection::Auto,
            line_break: spinon_text_layout::LineBreakMode::Strict,
            ligatures: spinon_text_layout::LigatureMode::Normal,
            ..ParagraphStyle::default()
        },
    )]);
    let snapshot = fixture.document.snapshot();
    LayoutInput::from_host_document_with_text_paragraphs(
        &snapshot,
        fixture.root,
        Viewport {
            width,
            height: 80.0,
        },
        &styles,
        &paragraph_styles,
        StyleRevision::INITIAL,
        EnvironmentRevision::INITIAL,
    )
    .unwrap()
}

fn text_engine() -> TextLayoutEngine {
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).unwrap();
    engine
}

fn fragment(
    layout: &TextLayoutSnapshot,
    node: NodeId,
    line: usize,
) -> &spinon_text_layout::InlineFragment {
    layout
        .inline_fragments
        .iter()
        .find(|fragment| fragment.node == node && fragment.line_index == line)
        .unwrap_or_else(|| panic!("missing fragment for node {node} on line {line}"))
}

fn close_to(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() <= 0.05,
        "actual {actual} differs from expected {expected} CSS px"
    );
}

#[test]
fn nested_short_fragments_include_decorations_without_changing_line_height() {
    let fixture = nested_fixture("XX", 2);
    let outer = fixture.spans[0].id();
    let inner = fixture.spans[1].id();
    let input = paragraph_input(
        &fixture,
        50.0,
        &[inline_style(1.0, 2.0), inline_style(2.0, 3.0)],
    );
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &text_engine())
        .unwrap();
    let frame = &output.frames[&fixture.paragraph.id()];
    let layout = &output.text_layouts[&fixture.paragraph.id()].layout;

    close_to(frame.width, 50.0);
    close_to(frame.height, 10.0);
    close_to(layout.available_width, 50.0);
    close_to(layout.width, 36.0);
    close_to(layout.height, 10.0);
    assert_eq!(layout.lines.len(), 1);
    close_to(layout.lines[0].baseline - layout.lines[0].line_top, 8.0);
    close_to(fragment(layout, outer, 0).rect.x, 0.0);
    close_to(fragment(layout, outer, 0).rect.y, -3.0);
    close_to(fragment(layout, outer, 0).rect.width, 36.0);
    close_to(fragment(layout, outer, 0).rect.height, 16.0);
    close_to(fragment(layout, inner, 0).rect.x, 3.0);
    close_to(fragment(layout, inner, 0).rect.y, -5.0);
    close_to(fragment(layout, inner, 0).rect.width, 30.0);
    close_to(fragment(layout, inner, 0).rect.height, 20.0);
    close_to(fragment(layout, fixture.text.id(), 0).rect.x, 8.0);
    close_to(fragment(layout, fixture.text.id(), 0).rect.y, 0.0);
    close_to(fragment(layout, fixture.text.id(), 0).rect.width, 20.0);
    close_to(fragment(layout, fixture.text.id(), 0).rect.height, 10.0);
}

#[test]
fn nested_wrap_keeps_start_and_end_edges_on_their_respective_fragments() {
    let fixture = nested_fixture("XX XX", 2);
    let outer = fixture.spans[0].id();
    let inner = fixture.spans[1].id();
    let input = paragraph_input(
        &fixture,
        30.0,
        &[inline_style(1.0, 1.0), inline_style(1.0, 1.0)],
    );
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &text_engine())
        .unwrap();
    let frame = &output.frames[&fixture.paragraph.id()];
    let layout = &output.text_layouts[&fixture.paragraph.id()].layout;

    close_to(frame.width, 30.0);
    close_to(frame.height, 20.0);
    close_to(layout.width, 24.0);
    assert_eq!(layout.lines.len(), 2);
    for (line, expected_top) in layout.lines.iter().zip([0.0, 10.0]) {
        close_to(line.advance, 24.0);
        close_to(line.line_top, expected_top);
        close_to(line.baseline - line.line_top, 8.0);
    }
    for (line, outer_x, outer_y, outer_width, inner_x, inner_y, inner_width, text_x) in [
        (0, 0.0, -2.0, 24.0, 2.0, -2.0, 22.0, 4.0),
        (1, 0.0, 8.0, 24.0, 0.0, 8.0, 22.0, 0.0),
    ] {
        close_to(fragment(layout, outer, line).rect.x, outer_x);
        close_to(fragment(layout, outer, line).rect.y, outer_y);
        close_to(fragment(layout, outer, line).rect.width, outer_width);
        close_to(fragment(layout, outer, line).rect.height, 14.0);
        close_to(fragment(layout, inner, line).rect.x, inner_x);
        close_to(fragment(layout, inner, line).rect.y, inner_y);
        close_to(fragment(layout, inner, line).rect.width, inner_width);
        close_to(fragment(layout, inner, line).rect.height, 14.0);
        close_to(fragment(layout, fixture.text.id(), line).rect.x, text_x);
        let text_y = if line == 0 { 0.0 } else { 10.0 };
        close_to(fragment(layout, fixture.text.id(), line).rect.y, text_y);
        close_to(fragment(layout, fixture.text.id(), line).rect.width, 20.0);
        close_to(fragment(layout, fixture.text.id(), line).rect.height, 10.0);
    }
}

#[test]
fn inline_depth_above_two_levels_remains_an_explicit_diagnostic() {
    let fixture = nested_fixture("X", 3);
    let styles = vec![LayoutStyle::default(); 3]
        .into_iter()
        .map(|style| LayoutStyle {
            display: LayoutDisplay::Inline,
            ..style
        })
        .collect::<Vec<_>>();
    let mut root_and_paragraph = BTreeMap::from([
        (
            fixture.root.id(),
            LayoutStyle {
                width: LayoutDimension::Fixed(30.0),
                height: LayoutDimension::Fixed(80.0),
                display: LayoutDisplay::Block,
                ..LayoutStyle::default()
            },
        ),
        (
            fixture.paragraph.id(),
            LayoutStyle {
                width: LayoutDimension::Fixed(30.0),
                display: LayoutDisplay::Block,
                ..LayoutStyle::default()
            },
        ),
    ]);
    for (span, style) in fixture.spans.iter().zip(styles) {
        root_and_paragraph.insert(span.id(), style);
    }
    let paragraph_styles = BTreeMap::from([(
        fixture.paragraph.id(),
        ParagraphStyle {
            overflow_wrap: Default::default(),
            writing_mode: spinon_core::WritingMode::HorizontalTb,
            text_orientation: spinon_core::TextOrientation::Mixed,
            font_family: "Ahem".to_owned(),
            font_size: 10.0,
            line_height: 10.0,
            white_space: WhiteSpaceMode::Normal,
            word_break: WordBreakMode::Normal,
            direction: spinon_text_layout::TextDirection::Auto,
            line_break: spinon_text_layout::LineBreakMode::Strict,
            ligatures: spinon_text_layout::LigatureMode::Normal,
            ..ParagraphStyle::default()
        },
    )]);
    let snapshot = fixture.document.snapshot();
    assert!(matches!(
        LayoutInput::from_host_document_with_text_paragraphs(
            &snapshot,
            fixture.root,
            Viewport {
                width: 30.0,
                height: 80.0,
            },
            &root_and_paragraph,
            &paragraph_styles,
            StyleRevision::INITIAL,
            EnvironmentRevision::INITIAL,
        ),
        Err(crate::LayoutError::UnsupportedTextParagraph { node, .. })
            if node == fixture.paragraph.id()
    ));
}
