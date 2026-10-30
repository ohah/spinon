use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, NodeId};
use spinon_text_layout::{
    InlineBoxStyle, InlineEdges, InlineRunStyle, ParagraphInput, ParagraphStyle, TextSource,
    WhiteSpaceMode,
};

use crate::{
    LayoutDisplay, LayoutError, LayoutGridItemTextSource, LayoutLengthPercentage, LayoutStyle,
    TextFlowInput, TextFlowItem, TextParagraphInput,
};

#[path = "text_paragraphs/anonymous_grid.rs"]
mod anonymous_grid;
#[path = "text_paragraphs/dom_traversal.rs"]
mod dom_traversal;

use dom_traversal::{collect_hidden_layout_nodes, collect_host_handles, is_hidden_by_ancestor};

#[derive(Default)]
pub(super) struct HostTextLayouts {
    pub inline_nodes: BTreeSet<NodeId>,
    pub text_source_nodes: BTreeSet<NodeId>,
    pub excluded_layout_nodes: BTreeSet<NodeId>,
    pub paragraphs: BTreeMap<NodeId, ParagraphInput>,
    pub flows: BTreeMap<NodeId, TextFlowInput>,
    pub anonymous_grid_items: BTreeMap<NodeId, Vec<AnonymousGridTextItem>>,
}

pub(super) struct AnonymousGridTextItem {
    pub(super) anchor: NodeId,
    pub(super) source_nodes: Vec<LayoutGridItemTextSource>,
    pub(super) paragraph: ParagraphInput,
    pub(super) style: LayoutStyle,
}

pub(super) fn collect(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &BTreeMap<NodeId, LayoutStyle>,
    paragraph_styles: &BTreeMap<NodeId, ParagraphStyle>,
    inline_run_styles: &BTreeMap<NodeId, InlineRunStyle>,
    allow_c11_4_anonymous_grid_text_items: bool,
) -> Result<HostTextLayouts, LayoutError> {
    let handles = collect_host_handles(snapshot, root)?;
    let mut result = HostTextLayouts {
        inline_nodes: BTreeSet::new(),
        text_source_nodes: BTreeSet::new(),
        excluded_layout_nodes: BTreeSet::new(),
        paragraphs: BTreeMap::new(),
        flows: BTreeMap::new(),
        anonymous_grid_items: BTreeMap::new(),
    };

    for (&paragraph_id, paragraph_style) in paragraph_styles {
        let Some(&paragraph_handle) = handles.get(&paragraph_id) else {
            return Err(LayoutError::UnknownTextParagraph(paragraph_id));
        };
        let paragraph_node = snapshot
            .node(paragraph_handle)
            .ok_or(LayoutError::InvalidHostDocumentRoot(paragraph_id))?;
        if !matches!(paragraph_node.kind(), HostNodeKind::Element(_)) {
            return Err(LayoutError::UnsupportedTextParagraph {
                node: paragraph_id,
                reason: "paragraph owner가 element가 아닙니다",
            });
        }
        let layout_style = styles
            .get(&paragraph_id)
            .copied()
            .ok_or(LayoutError::MissingStyle(paragraph_id))?;
        if layout_style.display == LayoutDisplay::None {
            continue;
        }
        if layout_style.display == LayoutDisplay::Grid && allow_c11_4_anonymous_grid_text_items {
            if is_hidden_by_ancestor(snapshot, paragraph_handle, root, styles)? {
                continue;
            }
            if !matches!(
                paragraph_style.white_space,
                WhiteSpaceMode::Normal | WhiteSpaceMode::NoWrap | WhiteSpaceMode::PreLine
            ) || layout_style.order != 0
            {
                return Err(LayoutError::UnsupportedTextParagraph {
                    node: paragraph_id,
                    reason: "C11.4 anonymous Grid text requires white-space:normal, nowrap, or pre-line and order:0",
                });
            }
            let mut inline_styles = BTreeMap::new();
            let mut collector = ParagraphSourceCollector {
                snapshot,
                styles,
                inline_nodes: &mut result.inline_nodes,
                text_source_nodes: &mut result.text_source_nodes,
                excluded_layout_nodes: &mut result.excluded_layout_nodes,
                inline_styles: &mut inline_styles,
            };
            let items = anonymous_grid::collect_anonymous_grid_items(
                &mut collector,
                paragraph_handle,
                paragraph_id,
                layout_style,
                paragraph_style,
            )?;
            if !items.is_empty() {
                result.anonymous_grid_items.insert(paragraph_id, items);
            }
            continue;
        }
        if layout_style.display != LayoutDisplay::Block {
            return Err(LayoutError::UnsupportedTextParagraph {
                node: paragraph_id,
                reason: "paragraph owner는 display:block이어야 합니다",
            });
        }
        if is_hidden_by_ancestor(snapshot, paragraph_handle, root, styles)? {
            continue;
        }

        let mut inline_styles = BTreeMap::new();
        let mut collector = ParagraphSourceCollector {
            snapshot,
            styles,
            inline_nodes: &mut result.inline_nodes,
            text_source_nodes: &mut result.text_source_nodes,
            excluded_layout_nodes: &mut result.excluded_layout_nodes,
            inline_styles: &mut inline_styles,
        };
        let collected = collector.collect(
            paragraph_handle,
            paragraph_id,
            paragraph_style,
            inline_run_styles,
        )?;
        match collected {
            TextParagraphInput::Paragraph(paragraph) => {
                result.paragraphs.insert(paragraph_id, paragraph);
            }
            TextParagraphInput::AnonymousGridText { .. } => {
                return Err(LayoutError::UnsupportedTextParagraph {
                    node: paragraph_id,
                    reason: "anonymous Grid text item cannot be nested in a text paragraph",
                });
            }
            TextParagraphInput::Flow(flow) => {
                result.flows.insert(paragraph_id, flow);
            }
        }
    }

    Ok(result)
}

struct ParagraphSourceCollector<'a> {
    snapshot: &'a HostDocumentSnapshot,
    styles: &'a BTreeMap<NodeId, LayoutStyle>,
    inline_nodes: &'a mut BTreeSet<NodeId>,
    text_source_nodes: &'a mut BTreeSet<NodeId>,
    excluded_layout_nodes: &'a mut BTreeSet<NodeId>,
    inline_styles: &'a mut BTreeMap<NodeId, InlineBoxStyle>,
}

impl ParagraphSourceCollector<'_> {
    fn collect(
        &mut self,
        paragraph: HostNodeHandle,
        paragraph_id: NodeId,
        paragraph_style: &ParagraphStyle,
        inline_run_styles: &BTreeMap<NodeId, InlineRunStyle>,
    ) -> Result<TextParagraphInput, LayoutError> {
        let children = self
            .snapshot
            .children(paragraph)
            .ok_or(LayoutError::InvalidHostDocumentRoot(paragraph_id))?
            .collect::<Vec<_>>();
        let mut items = Vec::new();
        let mut sources = Vec::new();
        let mut has_block_child = false;

        for handle in children {
            let node = self
                .snapshot
                .node(handle)
                .ok_or(LayoutError::InvalidHostDocumentRoot(handle.id()))?;
            match node.kind() {
                HostNodeKind::Text(_) => self.push_text(handle, Vec::new(), &mut sources)?,
                HostNodeKind::Element(element) => {
                    let style = self
                        .styles
                        .get(&handle.id())
                        .copied()
                        .ok_or(LayoutError::MissingStyle(handle.id()))?;
                    if style.display == LayoutDisplay::None {
                        collect_hidden_layout_nodes(
                            self.snapshot,
                            handle,
                            self.styles,
                            self.excluded_layout_nodes,
                        )?;
                    } else if style.display == LayoutDisplay::Block {
                        push_anonymous_paragraph(
                            &mut items,
                            paragraph_id,
                            paragraph_style,
                            &mut sources,
                            self.inline_styles,
                            inline_run_styles,
                        );
                        items.push(TextFlowItem::Block(handle.id()));
                        has_block_child = true;
                    } else if is_supported_inline_text_element(
                        element.namespace(),
                        element.local_name(),
                    ) && style.display == LayoutDisplay::Inline
                    {
                        self.collect_inline_span(handle, paragraph_id, Vec::new(), &mut sources)?;
                    } else {
                        return Err(LayoutError::UnsupportedTextParagraph {
                            node: paragraph_id,
                            reason: "visible direct child는 HTML span 또는 Block이어야 합니다",
                        });
                    }
                }
            }
        }

        push_anonymous_paragraph(
            &mut items,
            paragraph_id,
            paragraph_style,
            &mut sources,
            self.inline_styles,
            inline_run_styles,
        );
        if has_block_child {
            Ok(TextParagraphInput::Flow(TextFlowInput {
                owner: paragraph_id,
                items,
            }))
        } else {
            match items.pop() {
                Some(TextFlowItem::Paragraph(paragraph)) if items.is_empty() => {
                    Ok(TextParagraphInput::Paragraph(paragraph))
                }
                None => Ok(TextParagraphInput::Paragraph(ParagraphInput {
                    node: paragraph_id,
                    sources: Vec::new(),
                    style: paragraph_style.clone(),
                    inline_styles: BTreeMap::new(),
                    inline_run_styles: BTreeMap::new(),
                })),
                _ => Err(LayoutError::UnsupportedTextParagraph {
                    node: paragraph_id,
                    reason: "inline paragraph sequence가 비어 있거나 잘못되었습니다",
                }),
            }
        }
    }

    fn collect_inline_span(
        &mut self,
        span: HostNodeHandle,
        paragraph_id: NodeId,
        ancestors: Vec<NodeId>,
        sources: &mut Vec<TextSource>,
    ) -> Result<(), LayoutError> {
        let mut pending = vec![(span, ancestors)];
        while let Some((handle, ancestors)) = pending.pop() {
            let node = self
                .snapshot
                .node(handle)
                .ok_or(LayoutError::InvalidHostDocumentRoot(handle.id()))?;
            match node.kind() {
                HostNodeKind::Text(_) => self.push_text(handle, ancestors, sources)?,
                HostNodeKind::Element(element) => {
                    let style = self
                        .styles
                        .get(&handle.id())
                        .copied()
                        .ok_or(LayoutError::MissingStyle(handle.id()))?;
                    if style.display == LayoutDisplay::None {
                        collect_hidden_layout_nodes(
                            self.snapshot,
                            handle,
                            self.styles,
                            self.excluded_layout_nodes,
                        )?;
                        continue;
                    }
                    if !is_supported_inline_text_element(element.namespace(), element.local_name())
                    {
                        return Err(LayoutError::UnsupportedTextParagraph {
                            node: paragraph_id,
                            reason: "inline subtree는 HTML span/strong/b/em/i와 text만 포함할 수 있습니다",
                        });
                    }
                    if ancestors.len() >= 2 {
                        return Err(LayoutError::UnsupportedTextParagraph {
                            node: paragraph_id,
                            reason: "C15.2a는 두 단계까지의 nested inline span만 지원합니다",
                        });
                    }
                    let Some(inline_style) = fixed_inline_box_style(style) else {
                        return Err(LayoutError::UnsupportedTextParagraph {
                            node: paragraph_id,
                            reason: "inline span은 고정된 nonnegative px border/padding만 가질 수 있습니다",
                        });
                    };
                    if !inline_style_is_empty(inline_style) {
                        self.inline_styles.insert(handle.id(), inline_style);
                    }
                    self.inline_nodes.insert(handle.id());
                    let mut next_ancestors = ancestors;
                    next_ancestors.push(handle.id());
                    let children = self
                        .snapshot
                        .children(handle)
                        .ok_or(LayoutError::InvalidHostDocumentRoot(handle.id()))?;
                    for child in children.collect::<Vec<_>>().into_iter().rev() {
                        pending.push((child, next_ancestors.clone()));
                    }
                }
            }
        }
        Ok(())
    }

    fn push_text(
        &mut self,
        handle: HostNodeHandle,
        ancestors: Vec<NodeId>,
        sources: &mut Vec<TextSource>,
    ) -> Result<(), LayoutError> {
        let node = self
            .snapshot
            .node(handle)
            .ok_or(LayoutError::InvalidHostDocumentRoot(handle.id()))?;
        let HostNodeKind::Text(data) = node.kind() else {
            return Err(LayoutError::UnsupportedTextNode(handle.id()));
        };
        if !self.text_source_nodes.insert(handle.id()) {
            return Err(LayoutError::DuplicateTextParagraph(handle.id()));
        }
        let text = String::from_utf16(data.code_units())
            .map_err(|_| LayoutError::InvalidTextEncoding(handle.id()))?;
        if !text.is_empty() {
            sources.push(TextSource {
                node: handle.id(),
                inline_ancestors: ancestors,
                text,
            });
        }
        Ok(())
    }
}

fn is_supported_inline_text_element(namespace: &str, local_name: &str) -> bool {
    namespace == "http://www.w3.org/1999/xhtml"
        && matches!(local_name, "span" | "strong" | "b" | "em" | "i")
}

fn push_anonymous_paragraph(
    items: &mut Vec<TextFlowItem>,
    owner: NodeId,
    style: &ParagraphStyle,
    sources: &mut Vec<TextSource>,
    inline_styles: &BTreeMap<NodeId, InlineBoxStyle>,
    inline_run_styles: &BTreeMap<NodeId, InlineRunStyle>,
) {
    if sources.is_empty() {
        return;
    }
    let sources = std::mem::take(sources);
    let owners = sources
        .iter()
        .flat_map(|source| source.inline_ancestors.iter().copied())
        .collect::<BTreeSet<_>>();
    items.push(TextFlowItem::Paragraph(ParagraphInput {
        node: owner,
        sources,
        style: style.clone(),
        inline_styles: inline_styles
            .iter()
            .filter(|(node, _)| owners.contains(node))
            .map(|(&node, &style)| (node, style))
            .collect(),
        inline_run_styles: inline_run_styles
            .iter()
            .filter(|(node, _)| owners.contains(node))
            .map(|(&node, style)| (node, style.clone()))
            .collect(),
    }));
}

fn fixed_inline_box_style(style: LayoutStyle) -> Option<InlineBoxStyle> {
    if style.display != LayoutDisplay::Inline
        || [
            style.margin.top,
            style.margin.right,
            style.margin.bottom,
            style.margin.left,
        ]
        .into_iter()
        .any(|edge| !zero_or_auto(edge))
    {
        return None;
    }
    let border = style.border;
    if [border.top, border.right, border.bottom, border.left]
        .into_iter()
        .any(|edge| !edge.is_finite() || edge < 0.0)
    {
        return None;
    }
    let padding = style.padding;
    let edges = InlineEdges {
        top: fixed_nonnegative_px(padding.top)?,
        right: fixed_nonnegative_px(padding.right)?,
        bottom: fixed_nonnegative_px(padding.bottom)?,
        left: fixed_nonnegative_px(padding.left)?,
    };
    let inline = InlineBoxStyle {
        border: InlineEdges {
            top: normalize_zero(border.top),
            right: normalize_zero(border.right),
            bottom: normalize_zero(border.bottom),
            left: normalize_zero(border.left),
        },
        padding: edges,
    };
    let sums = [
        inline.border.left + inline.padding.left,
        inline.border.right + inline.padding.right,
        inline.border.top + inline.padding.top,
        inline.border.bottom + inline.padding.bottom,
    ];
    sums.into_iter().all(f32::is_finite).then_some(inline)
}

fn zero_or_auto(value: LayoutLengthPercentage) -> bool {
    match value {
        LayoutLengthPercentage::Auto => true,
        LayoutLengthPercentage::LengthPx(value) => value == 0.0,
        LayoutLengthPercentage::Percentage(_) | LayoutLengthPercentage::Calc(_) => false,
    }
}

fn fixed_nonnegative_px(value: LayoutLengthPercentage) -> Option<f32> {
    match value {
        LayoutLengthPercentage::LengthPx(value) if value.is_finite() && value >= 0.0 => {
            Some(normalize_zero(value))
        }
        LayoutLengthPercentage::Auto
        | LayoutLengthPercentage::LengthPx(_)
        | LayoutLengthPercentage::Percentage(_)
        | LayoutLengthPercentage::Calc(_) => None,
    }
}

fn normalize_zero(value: f32) -> f32 {
    if value == 0.0 { 0.0 } else { value }
}

fn inline_style_is_empty(style: InlineBoxStyle) -> bool {
    [
        style.border.top,
        style.border.right,
        style.border.bottom,
        style.border.left,
        style.padding.top,
        style.padding.right,
        style.padding.bottom,
        style.padding.left,
    ]
    .into_iter()
    .all(|edge| edge == 0.0)
}
