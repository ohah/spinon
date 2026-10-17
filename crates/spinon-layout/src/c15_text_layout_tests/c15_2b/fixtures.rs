use std::collections::BTreeMap;

use spinon_core::{
    DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_text_layout::{InlineRunStyle, ParagraphStyle, WhiteSpaceMode};

use crate::{
    LayoutDimension, LayoutDisplay, LayoutInput, LayoutStyle, TextLayoutEngine, Viewport,
    WordBreakMode,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const AHEM: &[u8] = include_bytes!("../../../../../tests/fixtures/resources/c15/Ahem.ttf");
const ROBOTO: &[u8] =
    include_bytes!("../../../../../tests/fixtures/resources/c15/Roboto-Variable.ttf");

pub(super) struct MixedFixture {
    pub(super) document: HostDocument,
    pub(super) root: HostNodeHandle,
    pub(super) paragraph: HostNodeHandle,
    pub(super) leading_text: HostNodeHandle,
    pub(super) span: HostNodeHandle,
    pub(super) inner_span: HostNodeHandle,
    pub(super) middle_text: HostNodeHandle,
    pub(super) trailing_text: HostNodeHandle,
}

pub(super) fn mixed_fixture() -> MixedFixture {
    let mut document = HostDocument::new().unwrap();
    let owner = OwnerId::new(1).unwrap();
    let root = document.reserve_node_handle().unwrap();
    let paragraph = document.reserve_node_handle().unwrap();
    let leading_text = document.reserve_node_handle().unwrap();
    let span = document.reserve_node_handle().unwrap();
    let inner_span = document.reserve_node_handle().unwrap();
    let middle_text = document.reserve_node_handle().unwrap();
    let trailing_text = document.reserve_node_handle().unwrap();
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
        .push(DocumentOperation::CreateText {
            node: leading_text,
            data: "X".into(),
        })
        .push(DocumentOperation::CreateElement {
            node: span,
            namespace: HTML.to_owned(),
            local_name: "span".to_owned(),
        })
        .push(DocumentOperation::CreateElement {
            node: inner_span,
            namespace: HTML.to_owned(),
            local_name: "span".to_owned(),
        })
        .push(DocumentOperation::CreateText {
            node: middle_text,
            data: "Hgyp".into(),
        })
        .push(DocumentOperation::CreateText {
            node: trailing_text,
            data: "X".into(),
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
            node: leading_text,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(paragraph),
            node: span,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(span),
            node: inner_span,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(inner_span),
            node: middle_text,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(paragraph),
            node: trailing_text,
            before: None,
        });
    document.commit(batch).unwrap();
    MixedFixture {
        document,
        root,
        paragraph,
        leading_text,
        span,
        inner_span,
        middle_text,
        trailing_text,
    }
}

pub(super) struct FlowFixture {
    pub(super) document: HostDocument,
    pub(super) root: HostNodeHandle,
    pub(super) paragraph: HostNodeHandle,
    pub(super) leading_text: HostNodeHandle,
    pub(super) leading_span: HostNodeHandle,
    pub(super) leading_span_text: HostNodeHandle,
    pub(super) block: HostNodeHandle,
    pub(super) block_text: HostNodeHandle,
    pub(super) trailing_span: HostNodeHandle,
    pub(super) trailing_text: HostNodeHandle,
}

pub(super) fn flow_fixture() -> FlowFixture {
    let mut document = HostDocument::new().unwrap();
    let owner = OwnerId::new(2).unwrap();
    let root = document.reserve_node_handle().unwrap();
    let paragraph = document.reserve_node_handle().unwrap();
    let leading_text = document.reserve_node_handle().unwrap();
    let leading_span = document.reserve_node_handle().unwrap();
    let leading_span_text = document.reserve_node_handle().unwrap();
    let block = document.reserve_node_handle().unwrap();
    let block_text = document.reserve_node_handle().unwrap();
    let trailing_span = document.reserve_node_handle().unwrap();
    let trailing_text = document.reserve_node_handle().unwrap();
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    for (node, local_name) in [(root, "div"), (paragraph, "p"), (block, "div")] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: local_name.to_owned(),
        });
    }
    for (node, text) in [
        (leading_text, "X"),
        (leading_span_text, "XX"),
        (block_text, "XX"),
        (trailing_text, "X XX"),
    ] {
        batch.push(DocumentOperation::CreateText {
            node,
            data: text.into(),
        });
    }
    for node in [leading_span, trailing_span] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: "span".to_owned(),
        });
    }
    for (parent, node) in [
        (HostParent::Root, root),
        (HostParent::Node(root), paragraph),
        (HostParent::Node(paragraph), leading_text),
        (HostParent::Node(paragraph), leading_span),
        (HostParent::Node(leading_span), leading_span_text),
        (HostParent::Node(paragraph), block),
        (HostParent::Node(block), block_text),
        (HostParent::Node(paragraph), trailing_span),
        (HostParent::Node(trailing_span), trailing_text),
    ] {
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    FlowFixture {
        document,
        root,
        paragraph,
        leading_text,
        leading_span,
        leading_span_text,
        block,
        block_text,
        trailing_span,
        trailing_text,
    }
}

pub(super) fn paragraph_style(
    font_family: &str,
    font_size: f32,
    line_height: f32,
) -> ParagraphStyle {
    ParagraphStyle {
        font_family: font_family.to_owned(),
        font_size,
        line_height,
        white_space: WhiteSpaceMode::Normal,
        word_break: WordBreakMode::Normal,
        direction: spinon_text_layout::TextDirection::Auto,
        line_break: spinon_text_layout::LineBreakMode::Strict,
        ligatures: spinon_text_layout::LigatureMode::Normal,
    }
}

pub(super) fn mixed_layout(fixture: &MixedFixture) -> LayoutInput {
    let styles = BTreeMap::from([
        (
            fixture.root.id(),
            LayoutStyle {
                width: LayoutDimension::Fixed(100.0),
                height: LayoutDimension::Fixed(60.0),
                display: LayoutDisplay::Block,
                ..LayoutStyle::default()
            },
        ),
        (
            fixture.paragraph.id(),
            LayoutStyle {
                width: LayoutDimension::Fixed(100.0),
                display: LayoutDisplay::Block,
                ..LayoutStyle::default()
            },
        ),
        (
            fixture.span.id(),
            LayoutStyle {
                display: LayoutDisplay::Inline,
                ..LayoutStyle::default()
            },
        ),
        (
            fixture.inner_span.id(),
            LayoutStyle {
                display: LayoutDisplay::Inline,
                ..LayoutStyle::default()
            },
        ),
    ]);
    let paragraph_styles =
        BTreeMap::from([(fixture.paragraph.id(), paragraph_style("Ahem", 10.0, 10.0))]);
    let run_styles = BTreeMap::from([(
        fixture.span.id(),
        InlineRunStyle {
            font_family: "Roboto".to_owned(),
            font_size: 20.0,
            line_height: 24.0,
        },
    )]);
    LayoutInput::from_host_document_with_text_paragraphs_and_run_styles(
        &fixture.document.snapshot(),
        fixture.root,
        Viewport {
            width: 100.0,
            height: 60.0,
        },
        &styles,
        crate::host_document::HostTextStyleProfile {
            paragraphs: &paragraph_styles,
            inline_runs: &run_styles,
        },
        StyleRevision::INITIAL,
        EnvironmentRevision::INITIAL,
    )
    .unwrap()
}

pub(super) fn flow_layout(fixture: &FlowFixture) -> LayoutInput {
    let block_style = LayoutStyle {
        display: LayoutDisplay::Block,
        ..LayoutStyle::default()
    };
    let inline_style = LayoutStyle {
        display: LayoutDisplay::Inline,
        ..LayoutStyle::default()
    };
    let styles = BTreeMap::from([
        (
            fixture.root.id(),
            LayoutStyle {
                width: LayoutDimension::Fixed(100.0),
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
        (fixture.leading_span.id(), inline_style),
        (fixture.block.id(), block_style),
        (fixture.trailing_span.id(), inline_style),
    ]);
    let paragraph_styles = BTreeMap::from([
        (fixture.paragraph.id(), paragraph_style("Ahem", 10.0, 10.0)),
        (fixture.block.id(), paragraph_style("Ahem", 10.0, 10.0)),
    ]);
    LayoutInput::from_host_document_with_text_paragraphs(
        &fixture.document.snapshot(),
        fixture.root,
        Viewport {
            width: 100.0,
            height: 80.0,
        },
        &styles,
        &paragraph_styles,
        StyleRevision::INITIAL,
        EnvironmentRevision::INITIAL,
    )
    .unwrap()
}

pub(super) fn ahem_engine() -> TextLayoutEngine {
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).unwrap();
    engine
}

pub(super) fn both_fonts_engine() -> TextLayoutEngine {
    let mut engine = ahem_engine();
    engine.register_font("Roboto", ROBOTO).unwrap();
    engine
}

pub(super) fn close_to(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() <= 0.05,
        "actual {actual} differs from expected {expected} CSS px"
    );
}
