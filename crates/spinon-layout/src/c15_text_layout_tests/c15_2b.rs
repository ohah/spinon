use std::collections::BTreeMap;

use spinon_core::{
    DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostParent, NodeId, OwnerId,
    StyleRevision,
};
use spinon_text_layout::InlineRunStyle;

use crate::{
    LayoutBorder, LayoutDimension, LayoutDisplay, LayoutEdges, LayoutInput, LayoutLengthPercentage,
    LayoutPosition, LayoutPositioning, LayoutStyle, TaffyLayoutEngine, TextFlowItem,
    TextParagraphInput, Viewport,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";

#[path = "c15_2b/fixtures.rs"]
mod fixtures;

use fixtures::{
    ahem_engine, both_fonts_engine, close_to, flow_fixture, flow_layout, mixed_fixture,
    mixed_layout, paragraph_style,
};

#[test]
fn mixed_font_run_metrics_share_the_chrome_baseline_and_line_box() {
    let fixture = mixed_fixture();
    let input = mixed_layout(&fixture);
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &both_fonts_engine())
        .unwrap();
    let layout = &output.text_layouts[&fixture.paragraph.id()].layout;

    assert_eq!(layout.lines.len(), 1);
    close_to(layout.height, 24.0);
    close_to(layout.lines[0].baseline - layout.lines[0].line_top, 19.0);
    assert_eq!(layout.runs.len(), 3);
    assert!(layout.runs.iter().any(|run| {
        run.font_family == "Roboto"
            && run
                .sources
                .iter()
                .any(|source| source.node == fixture.middle_text.id())
    }));
    for run in &layout.runs {
        close_to(run.baseline, 19.0);
        if run.font_family == "Roboto" {
            assert_eq!(run.font_size, 20.0);
            close_to(run.line_height, 24.0);
        } else {
            assert_eq!(run.font_family, "Ahem");
            assert_eq!(run.font_size, 10.0);
            close_to(run.line_height, 10.0);
        }
    }
    let leading = layout
        .inline_fragments
        .iter()
        .find(|fragment| fragment.node == fixture.leading_text.id())
        .unwrap();
    let middle = layout
        .inline_fragments
        .iter()
        .find(|fragment| fragment.node == fixture.middle_text.id())
        .unwrap();
    let trailing = layout
        .inline_fragments
        .iter()
        .find(|fragment| fragment.node == fixture.trailing_text.id())
        .unwrap();
    close_to(leading.rect.y, 11.0);
    close_to(leading.rect.height, 10.0);
    close_to(middle.rect.y, 0.0);
    close_to(middle.rect.x, 10.0);
    close_to(middle.rect.width, 46.2031);
    close_to(middle.rect.height, 24.0);
    close_to(trailing.rect.y, 11.0);
    close_to(trailing.rect.height, 10.0);
}

#[test]
fn anonymous_inline_blocks_keep_source_order_and_flow_geometry() {
    let fixture = flow_fixture();
    let input = flow_layout(&fixture);
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &ahem_engine())
        .unwrap();
    let parent = output.frames[&fixture.paragraph.id()];
    let block = output.frames[&fixture.block.id()];
    let segments = &output.text_flow_layouts[&fixture.paragraph.id()];

    close_to(parent.width, 30.0);
    close_to(parent.height, 40.0);
    close_to(block.x, 0.0);
    close_to(block.y, 10.0);
    close_to(block.width, 30.0);
    close_to(block.height, 10.0);
    assert_eq!(
        segments
            .iter()
            .map(|segment| segment.item_index)
            .collect::<Vec<_>>(),
        vec![0, 2]
    );
    close_to(segments[0].frame.y, 0.0);
    close_to(segments[0].frame.width, 30.0);
    close_to(segments[0].frame.height, 10.0);
    assert_eq!(segments[0].snapshot.layout.text, "XXX");
    assert_eq!(
        segments[0]
            .snapshot
            .input
            .sources
            .iter()
            .map(|source| source.node)
            .collect::<Vec<_>>(),
        vec![fixture.leading_text.id(), fixture.leading_span_text.id()]
    );
    close_to(segments[1].frame.y, 20.0);
    close_to(segments[1].frame.width, 30.0);
    close_to(segments[1].frame.height, 20.0);
    assert_eq!(segments[1].snapshot.layout.text, "X XX");
    assert_eq!(segments[1].snapshot.layout.lines.len(), 2);
    close_to(segments[1].snapshot.layout.lines[0].line_top, 0.0);
    close_to(segments[1].snapshot.layout.lines[1].line_top, 10.0);
    assert_eq!(
        segments[1]
            .snapshot
            .input
            .sources
            .iter()
            .map(|source| source.node)
            .collect::<Vec<_>>(),
        vec![fixture.trailing_text.id()]
    );
    assert_eq!(
        output.text_layouts[&fixture.block.id()].input.sources[0].node,
        fixture.block_text.id()
    );

    let mut inset_input = input;
    let paragraph_style = inset_input
        .nodes
        .iter_mut()
        .find(|node| node.id == fixture.paragraph.id())
        .map(|node| &mut node.style)
        .unwrap();
    paragraph_style.border = LayoutBorder {
        top: 1.0,
        right: 1.0,
        bottom: 1.0,
        left: 1.0,
    };
    paragraph_style.padding = LayoutEdges {
        top: LayoutLengthPercentage::length(2.0),
        right: LayoutLengthPercentage::length(2.0),
        bottom: LayoutLengthPercentage::length(2.0),
        left: LayoutLengthPercentage::length(2.0),
    };
    let inset_output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&inset_input, &ahem_engine())
        .unwrap();
    let inset_block = inset_output.frames[&fixture.block.id()];
    let inset_segments = &inset_output.text_flow_layouts[&fixture.paragraph.id()];
    close_to(inset_block.x, 3.0);
    close_to(inset_block.y, 13.0);
    close_to(inset_segments[0].frame.x, 3.0);
    close_to(inset_segments[0].frame.y, 3.0);
    close_to(inset_segments[1].frame.y, 23.0);
}

#[test]
fn flow_rejects_duplicated_block_item_before_taffy_layout() {
    let fixture = flow_fixture();
    let mut input = flow_layout(&fixture);
    let Some(TextParagraphInput::Flow(flow)) =
        input.text_paragraphs.get_mut(&fixture.paragraph.id())
    else {
        panic!("mixed fixture should produce a flow input");
    };
    let block_index = flow
        .items
        .iter()
        .position(|item| matches!(item, TextFlowItem::Block(_)))
        .unwrap();
    let duplicate = flow.items[block_index].clone();
    flow.items.insert(block_index + 1, duplicate);

    assert!(matches!(
        TaffyLayoutEngine.compute_with_text_layout_engine(&input, &ahem_engine()),
        Err(crate::LayoutError::UnsupportedTextParagraph { node, .. })
            if node == fixture.paragraph.id()
    ));
}

#[test]
fn flow_rejects_out_of_flow_block_child_before_reparenting() {
    let fixture = flow_fixture();
    let input = flow_layout(&fixture).with_positioning(BTreeMap::from([(
        fixture.block.id(),
        LayoutPositioning {
            position: LayoutPosition::Absolute,
            ..LayoutPositioning::default()
        },
    )]));

    assert!(matches!(
        TaffyLayoutEngine.compute_with_text_layout_engine(&input, &ahem_engine()),
        Err(crate::LayoutError::UnsupportedTextParagraph { node, .. })
            if node == fixture.paragraph.id()
    ));
}

#[test]
fn flow_rejects_block_reference_outside_the_layout_graph() {
    let fixture = flow_fixture();
    let mut input = flow_layout(&fixture);
    let Some(TextParagraphInput::Flow(flow)) =
        input.text_paragraphs.get_mut(&fixture.paragraph.id())
    else {
        panic!("mixed fixture should produce a flow input");
    };
    let block = flow
        .items
        .iter_mut()
        .find(|item| matches!(item, TextFlowItem::Block(_)))
        .unwrap();
    *block = TextFlowItem::Block(NodeId::new(999).unwrap());

    assert!(matches!(
        TaffyLayoutEngine.compute_with_text_layout_engine(&input, &ahem_engine()),
        Err(crate::LayoutError::UnsupportedTextParagraph { node, .. })
            if node == fixture.paragraph.id()
    ));
}

#[test]
fn inline_span_rejects_nested_block_topology() {
    let mut fixture = mixed_fixture();
    let nested_block = fixture.document.reserve_node_handle().unwrap();
    let nested_text = fixture.document.reserve_node_handle().unwrap();
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(1).unwrap(),
        fixture.document.document_revision(),
    );
    batch
        .push(DocumentOperation::CreateElement {
            node: nested_block,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        })
        .push(DocumentOperation::CreateText {
            node: nested_text,
            data: "XX".into(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(fixture.span),
            node: nested_block,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(nested_block),
            node: nested_text,
            before: None,
        });
    fixture.document.commit(batch).unwrap();
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
        (
            nested_block.id(),
            LayoutStyle {
                display: LayoutDisplay::Block,
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
    let result = LayoutInput::from_host_document_with_text_paragraphs_and_run_styles(
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
    .unwrap_err();
    assert!(matches!(
        result,
        crate::LayoutError::UnsupportedTextParagraph { node, .. }
            if node == fixture.paragraph.id()
    ));
}

#[test]
fn run_styles_fail_closed_for_missing_fonts_and_invalid_metrics() {
    let fixture = mixed_fixture();
    let input = mixed_layout(&fixture);
    assert!(matches!(
        TaffyLayoutEngine.compute_with_text_layout_engine(&input, &ahem_engine()),
        Err(crate::LayoutError::TextLayout(
            spinon_text_layout::TextLayoutError::MissingFontFamily(family)
        )) if family == "Roboto"
    ));

    let mut invalid = input;
    let Some(TextParagraphInput::Paragraph(paragraph)) =
        invalid.text_paragraphs.get_mut(&fixture.paragraph.id())
    else {
        panic!("mixed fixture should produce a paragraph input");
    };
    paragraph
        .inline_run_styles
        .get_mut(&fixture.span.id())
        .unwrap()
        .font_size = f32::NAN;
    assert!(matches!(
        TaffyLayoutEngine.compute_with_text_layout_engine(&invalid, &both_fonts_engine()),
        Err(crate::LayoutError::TextLayout(
            spinon_text_layout::TextLayoutError::InvalidStyle("font-size")
        ))
    ));
}
