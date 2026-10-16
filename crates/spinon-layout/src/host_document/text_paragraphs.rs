use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, NodeId};
use spinon_text_layout::{
    InlineBoxStyle, InlineEdges, InlineRunStyle, ParagraphInput, ParagraphStyle, TextSource,
};

use crate::{
    LayoutDisplay, LayoutError, LayoutLengthPercentage, LayoutStyle, TextFlowInput, TextFlowItem,
    TextParagraphInput,
};

#[derive(Default)]
pub(super) struct HostTextLayouts {
    pub inline_nodes: BTreeSet<NodeId>,
    pub text_source_nodes: BTreeSet<NodeId>,
    pub excluded_layout_nodes: BTreeSet<NodeId>,
    pub paragraphs: BTreeMap<NodeId, ParagraphInput>,
    pub flows: BTreeMap<NodeId, TextFlowInput>,
}

pub(super) fn collect(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &BTreeMap<NodeId, LayoutStyle>,
    paragraph_styles: &BTreeMap<NodeId, ParagraphStyle>,
    inline_run_styles: &BTreeMap<NodeId, InlineRunStyle>,
) -> Result<HostTextLayouts, LayoutError> {
    let handles = collect_host_handles(snapshot, root)?;
    let mut result = HostTextLayouts {
        inline_nodes: BTreeSet::new(),
        text_source_nodes: BTreeSet::new(),
        excluded_layout_nodes: BTreeSet::new(),
        paragraphs: BTreeMap::new(),
        flows: BTreeMap::new(),
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
            TextParagraphInput::Flow(flow) => {
                result.flows.insert(paragraph_id, flow);
            }
        }
    }

    Ok(result)
}

fn collect_host_handles(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
) -> Result<BTreeMap<NodeId, HostNodeHandle>, LayoutError> {
    let mut handles = BTreeMap::new();
    let mut pending = vec![root];
    while let Some(handle) = pending.pop() {
        if handles.insert(handle.id(), handle).is_some() {
            return Err(LayoutError::DuplicateNode(handle.id()));
        }
        let node = snapshot
            .node(handle)
            .ok_or(LayoutError::InvalidHostDocumentRoot(handle.id()))?;
        if let Some(children) = snapshot.children(handle) {
            pending.extend(children);
        } else if !matches!(node.kind(), HostNodeKind::Text(_)) {
            return Err(LayoutError::InvalidHostDocumentRoot(handle.id()));
        }
    }
    Ok(handles)
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
                    } else if element.namespace() == "http://www.w3.org/1999/xhtml"
                        && element.local_name() == "span"
                        && style.display == LayoutDisplay::Inline
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
                    if element.namespace() != "http://www.w3.org/1999/xhtml"
                        || element.local_name() != "span"
                    {
                        return Err(LayoutError::UnsupportedTextParagraph {
                            node: paragraph_id,
                            reason: "inline subtree는 HTML span과 text만 포함할 수 있습니다",
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
        || style
            != (LayoutStyle {
                display: LayoutDisplay::Inline,
                border: style.border,
                padding: style.padding,
                ..LayoutStyle::default()
            })
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

fn collect_hidden_layout_nodes(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &BTreeMap<NodeId, LayoutStyle>,
    excluded: &mut BTreeSet<NodeId>,
) -> Result<(), LayoutError> {
    let mut pending = vec![root];
    while let Some(handle) = pending.pop() {
        let node = snapshot
            .node(handle)
            .ok_or(LayoutError::InvalidHostDocumentRoot(handle.id()))?;
        if matches!(node.kind(), HostNodeKind::Element(_)) {
            if !styles.contains_key(&handle.id()) {
                return Err(LayoutError::MissingStyle(handle.id()));
            }
            excluded.insert(handle.id());
            pending.extend(
                snapshot
                    .children(handle)
                    .ok_or(LayoutError::InvalidHostDocumentRoot(handle.id()))?
                    .filter(|child| {
                        snapshot
                            .node(*child)
                            .is_some_and(|node| matches!(node.kind(), HostNodeKind::Element(_)))
                    }),
            );
        }
    }
    Ok(())
}

fn is_hidden_by_ancestor(
    snapshot: &HostDocumentSnapshot,
    handle: HostNodeHandle,
    root: HostNodeHandle,
    styles: &BTreeMap<NodeId, LayoutStyle>,
) -> Result<bool, LayoutError> {
    let mut current = handle;
    loop {
        let style = styles
            .get(&current.id())
            .copied()
            .ok_or(LayoutError::MissingStyle(current.id()))?;
        if style.display == LayoutDisplay::None {
            return Ok(true);
        }
        if current == root {
            return Ok(false);
        }
        match snapshot.parent(current) {
            Some(HostParent::Node(parent)) => current = parent,
            Some(HostParent::Root) | None => return Ok(false),
        }
    }
}
