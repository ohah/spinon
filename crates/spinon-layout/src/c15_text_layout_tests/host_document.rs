use std::collections::BTreeMap;

use spinon_core::{
    DocumentChangeBatch, DocumentOperation, DomString, EnvironmentRevision, HostDocument,
    HostNodeHandle, HostParent, NodeId, OwnerId, StyleRevision,
};

use crate::{
    LayoutBorder, LayoutDimension, LayoutDisplay, LayoutEdges, LayoutInput, LayoutLengthPercentage,
    LayoutStyle, ParagraphStyle, TaffyLayoutEngine, Viewport, WhiteSpaceMode, WordBreakMode,
};

use super::text_engine;

fn host_text_fixture() -> (
    HostDocument,
    HostNodeHandle,
    HostNodeHandle,
    HostNodeHandle,
    HostNodeHandle,
    HostNodeHandle,
) {
    const HTML: &str = "http://www.w3.org/1999/xhtml";
    let mut document = HostDocument::new().unwrap();
    let owner = OwnerId::new(1).unwrap();
    let root = document.reserve_node_handle().unwrap();
    let paragraph = document.reserve_node_handle().unwrap();
    let span = document.reserve_node_handle().unwrap();
    let hidden_span = document.reserve_node_handle().unwrap();
    let direct_before = document.reserve_node_handle().unwrap();
    let span_text = document.reserve_node_handle().unwrap();
    let direct_after = document.reserve_node_handle().unwrap();
    let hidden_text = document.reserve_node_handle().unwrap();
    let empty_text = document.reserve_node_handle().unwrap();

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
        })
        .push(DocumentOperation::CreateElement {
            node: span,
            namespace: HTML.to_owned(),
            local_name: "span".to_owned(),
        })
        .push(DocumentOperation::CreateElement {
            node: hidden_span,
            namespace: HTML.to_owned(),
            local_name: "span".to_owned(),
        })
        .push(DocumentOperation::CreateText {
            node: direct_before,
            data: "X".into(),
        })
        .push(DocumentOperation::CreateText {
            node: span_text,
            data: "X".into(),
        })
        .push(DocumentOperation::CreateText {
            node: direct_after,
            data: "X".into(),
        })
        .push(DocumentOperation::CreateText {
            node: hidden_text,
            data: "hidden".into(),
        })
        .push(DocumentOperation::CreateText {
            node: empty_text,
            data: "".into(),
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
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(paragraph),
            node: direct_before,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(paragraph),
            node: span,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(span),
            node: span_text,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(paragraph),
            node: direct_after,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(paragraph),
            node: hidden_span,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(hidden_span),
            node: hidden_text,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(paragraph),
            node: empty_text,
            before: None,
        });
    document.commit(batch).unwrap();
    (document, root, paragraph, span, hidden_span, direct_before)
}

#[test]
fn host_document_collects_direct_text_and_inherited_spans_in_source_order() {
    let (mut document, root, paragraph, span, hidden_span, direct_before) = host_text_fixture();
    let styles = BTreeMap::from([
        (
            root.id(),
            LayoutStyle {
                width: LayoutDimension::Fixed(100.0),
                height: LayoutDimension::Fixed(80.0),
                display: LayoutDisplay::Block,
                ..LayoutStyle::default()
            },
        ),
        (
            paragraph.id(),
            LayoutStyle {
                width: LayoutDimension::Fixed(30.0),
                display: LayoutDisplay::Block,
                ..LayoutStyle::default()
            },
        ),
        (
            span.id(),
            LayoutStyle {
                display: LayoutDisplay::Inline,
                ..LayoutStyle::default()
            },
        ),
        (
            hidden_span.id(),
            LayoutStyle {
                display: LayoutDisplay::None,
                ..LayoutStyle::default()
            },
        ),
    ]);
    let paragraph_styles = BTreeMap::from([(
        paragraph.id(),
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
    let snapshot = document.snapshot();
    let input = LayoutInput::from_host_document_with_text_paragraphs(
        &snapshot,
        root,
        Viewport {
            width: 100.0,
            height: 80.0,
        },
        &styles,
        &paragraph_styles,
        StyleRevision::INITIAL,
        EnvironmentRevision::INITIAL,
    )
    .unwrap();
    let paragraph_input = input.text_paragraphs[&paragraph.id()].paragraph().unwrap();

    assert_eq!(
        paragraph_input
            .sources
            .iter()
            .map(|source| source.text.as_str())
            .collect::<Vec<_>>(),
        vec!["X", "X", "X"]
    );
    assert_eq!(paragraph_input.sources[1].inline_ancestors, vec![span.id()]);
    assert_eq!(
        input
            .nodes()
            .iter()
            .find(|node| node.id == paragraph.id())
            .unwrap()
            .children,
        Vec::<NodeId>::new()
    );

    let mut missing_hidden_style = styles.clone();
    missing_hidden_style.remove(&hidden_span.id());
    assert!(matches!(
        LayoutInput::from_host_document_with_text_paragraphs(
            &snapshot,
            root,
            Viewport {
                width: 100.0,
                height: 80.0,
            },
            &missing_hidden_style,
            &paragraph_styles,
            StyleRevision::INITIAL,
            EnvironmentRevision::INITIAL,
        ),
        Err(crate::LayoutError::MissingStyle(id)) if id == hidden_span.id()
    ));

    let unsupported_span_styles = [
        LayoutStyle {
            display: LayoutDisplay::Inline,
            margin: LayoutEdges {
                left: LayoutLengthPercentage::length(1.0),
                ..LayoutEdges::default()
            },
            ..LayoutStyle::default()
        },
        LayoutStyle {
            display: LayoutDisplay::Inline,
            padding: LayoutEdges {
                left: LayoutLengthPercentage::percent(0.5),
                ..LayoutEdges::default()
            },
            ..LayoutStyle::default()
        },
        LayoutStyle {
            display: LayoutDisplay::Inline,
            border: LayoutBorder {
                bottom: -1.0,
                ..LayoutBorder::default()
            },
            ..LayoutStyle::default()
        },
        LayoutStyle {
            display: LayoutDisplay::Inline,
            border: LayoutBorder {
                right: f32::MAX,
                ..LayoutBorder::default()
            },
            padding: LayoutEdges {
                right: LayoutLengthPercentage::length(f32::MAX),
                ..LayoutEdges::default()
            },
            ..LayoutStyle::default()
        },
    ];
    for inline_style in unsupported_span_styles {
        let mut rejected_styles = styles.clone();
        rejected_styles.insert(span.id(), inline_style);
        assert!(matches!(
            LayoutInput::from_host_document_with_text_paragraphs(
                &snapshot,
                root,
                Viewport {
                    width: 100.0,
                    height: 80.0,
                },
                &rejected_styles,
                &paragraph_styles,
                StyleRevision::INITIAL,
                EnvironmentRevision::INITIAL,
            ),
            Err(crate::LayoutError::UnsupportedTextParagraph { node, .. })
                if node == paragraph.id()
        ));
    }

    for display in [
        LayoutDisplay::Flex,
        LayoutDisplay::Grid,
        LayoutDisplay::FlowRoot,
    ] {
        let mut non_inline_display_styles = styles.clone();
        non_inline_display_styles.insert(
            span.id(),
            LayoutStyle {
                display,
                ..LayoutStyle::default()
            },
        );
        assert!(matches!(
            LayoutInput::from_host_document_with_text_paragraphs(
                &snapshot,
                root,
                Viewport {
                    width: 100.0,
                    height: 80.0,
                },
                &non_inline_display_styles,
                &paragraph_styles,
                StyleRevision::INITIAL,
                EnvironmentRevision::INITIAL,
            ),
            Err(crate::LayoutError::UnsupportedTextParagraph { node, .. })
                if node == paragraph.id()
        ));
    }

    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &text_engine())
        .unwrap();
    let text = &output.text_layouts[&paragraph.id()];
    assert_eq!(text.revision, input.revision());
    assert_eq!(text.layout.text, "XXX");
    assert!(
        text.layout
            .sources
            .iter()
            .all(|source| source.node != hidden_span.id())
    );
    assert_eq!(output.frames[&paragraph.id()].height, 10.0);
    assert!(
        text.layout
            .inline_fragments
            .iter()
            .any(|fragment| fragment.node == span.id())
    );

    let nested_span = document.reserve_node_handle().unwrap();
    let nested_text = document.reserve_node_handle().unwrap();
    let mut nested_batch =
        DocumentChangeBatch::new(OwnerId::new(1).unwrap(), document.document_revision());
    nested_batch
        .push(DocumentOperation::CreateElement {
            node: nested_span,
            namespace: "http://www.w3.org/1999/xhtml".to_owned(),
            local_name: "span".to_owned(),
        })
        .push(DocumentOperation::CreateText {
            node: nested_text,
            data: "nested".into(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(span),
            node: nested_span,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(nested_span),
            node: nested_text,
            before: None,
        });
    document.commit(nested_batch).unwrap();
    let mut nested_styles = styles.clone();
    nested_styles.insert(nested_span.id(), LayoutStyle::default());
    assert!(matches!(
        LayoutInput::from_host_document_with_text_paragraphs(
            &document.snapshot(),
            root,
            Viewport {
                width: 100.0,
                height: 80.0,
            },
            &nested_styles,
            &paragraph_styles,
            StyleRevision::INITIAL,
            EnvironmentRevision::INITIAL,
        ),
        Err(crate::LayoutError::UnsupportedTextParagraph { node, .. })
            if node == paragraph.id()
    ));

    let mut invalid_text_batch =
        DocumentChangeBatch::new(OwnerId::new(1).unwrap(), document.document_revision());
    invalid_text_batch.push(DocumentOperation::SetTextData {
        node: direct_before,
        data: DomString::from_utf16(vec![0xD800]),
    });
    document.commit(invalid_text_batch).unwrap();
    assert!(matches!(
        LayoutInput::from_host_document_with_text_paragraphs(
            &document.snapshot(),
            root,
            Viewport {
                width: 100.0,
                height: 80.0,
            },
            &nested_styles,
            &paragraph_styles,
            StyleRevision::INITIAL,
            EnvironmentRevision::INITIAL,
        ),
        Err(crate::LayoutError::InvalidTextEncoding(id)) if id == direct_before.id()
    ));
}

#[test]
fn host_document_rejects_visible_non_span_element_inside_paragraph() {
    const HTML: &str = "http://www.w3.org/1999/xhtml";
    let (mut document, root, paragraph, span, hidden_span, _) = host_text_fixture();
    let visible_element = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(1).unwrap(), document.document_revision());
    batch
        .push(DocumentOperation::CreateElement {
            node: visible_element,
            namespace: HTML.to_owned(),
            local_name: "strong".to_owned(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(paragraph),
            node: visible_element,
            before: None,
        });
    document.commit(batch).unwrap();

    let styles = BTreeMap::from([
        (
            root.id(),
            LayoutStyle {
                width: LayoutDimension::Fixed(100.0),
                height: LayoutDimension::Fixed(80.0),
                display: LayoutDisplay::Block,
                ..LayoutStyle::default()
            },
        ),
        (
            paragraph.id(),
            LayoutStyle {
                width: LayoutDimension::Fixed(30.0),
                display: LayoutDisplay::Block,
                ..LayoutStyle::default()
            },
        ),
        (
            span.id(),
            LayoutStyle {
                display: LayoutDisplay::Inline,
                ..LayoutStyle::default()
            },
        ),
        (
            hidden_span.id(),
            LayoutStyle {
                display: LayoutDisplay::None,
                ..LayoutStyle::default()
            },
        ),
        (visible_element.id(), LayoutStyle::default()),
    ]);
    let paragraph_styles = BTreeMap::from([(
        paragraph.id(),
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

    assert!(matches!(
        LayoutInput::from_host_document_with_text_paragraphs(
            &document.snapshot(),
            root,
            Viewport {
                width: 100.0,
                height: 80.0,
            },
            &styles,
            &paragraph_styles,
            StyleRevision::INITIAL,
            EnvironmentRevision::INITIAL,
        ),
        Err(crate::LayoutError::UnsupportedTextParagraph { node, .. })
            if node == paragraph.id()
    ));
}
