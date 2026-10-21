use std::collections::BTreeMap;
use std::collections::BTreeSet;

use spinon_core::{HostNodeHandle, HostNodeKind, NodeId};
use spinon_text_layout::{ParagraphInput, ParagraphStyle, TextSource};

use crate::{LayoutDisplay, LayoutError, LayoutGridItemTextSource, LayoutStyle};

use super::{AnonymousGridTextItem, ParagraphSourceCollector};

pub(super) fn collect_anonymous_grid_items(
    collector: &mut ParagraphSourceCollector<'_>,
    grid: HostNodeHandle,
    grid_id: NodeId,
    grid_style: LayoutStyle,
    paragraph_style: &ParagraphStyle,
) -> Result<Vec<AnonymousGridTextItem>, LayoutError> {
    let children = collector
        .snapshot
        .children(grid)
        .ok_or(LayoutError::InvalidHostDocumentRoot(grid_id))?
        .collect::<Vec<_>>();
    let mut items = Vec::new();
    let mut anchor = None;
    let mut source_nodes = Vec::new();
    let mut sources = Vec::new();

    for child in children {
        let node = collector
            .snapshot
            .node(child)
            .ok_or(LayoutError::InvalidHostDocumentRoot(child.id()))?;
        match node.kind() {
            HostNodeKind::Text(data) => {
                let text = String::from_utf16(data.code_units())
                    .map_err(|_| LayoutError::InvalidTextEncoding(child.id()))?;
                if anchor.is_none()
                    && text
                        .chars()
                        .any(|character| !is_collapsible_html_whitespace(character))
                {
                    anchor = Some(child.id());
                }
                source_nodes.push(LayoutGridItemTextSource {
                    node: child.id(),
                    text,
                });
                collector.push_text(child, Vec::new(), &mut sources)?;
            }
            HostNodeKind::Element(_) => {
                let style = collector
                    .styles
                    .get(&child.id())
                    .copied()
                    .ok_or(LayoutError::MissingStyle(child.id()))?;
                match style.display {
                    LayoutDisplay::None => super::dom_traversal::collect_hidden_layout_nodes(
                        collector.snapshot,
                        child,
                        collector.styles,
                        collector.excluded_layout_nodes,
                    )?,
                    LayoutDisplay::Block => flush_anonymous_grid_text_run(
                        grid_style,
                        paragraph_style,
                        &mut anchor,
                        &mut source_nodes,
                        &mut sources,
                        &mut items,
                    ),
                    LayoutDisplay::Inline
                    | LayoutDisplay::Flex
                    | LayoutDisplay::Grid
                    | LayoutDisplay::FlowRoot => {
                        return Err(LayoutError::UnsupportedTextParagraph {
                            node: grid_id,
                            reason: "C11.4 anonymous Grid text supports only direct block separators",
                        });
                    }
                }
            }
        }
    }

    flush_anonymous_grid_text_run(
        grid_style,
        paragraph_style,
        &mut anchor,
        &mut source_nodes,
        &mut sources,
        &mut items,
    );
    validate_unique_source_nodes(&items, grid_id)?;
    Ok(items)
}

fn validate_unique_source_nodes(
    items: &[AnonymousGridTextItem],
    grid_id: NodeId,
) -> Result<(), LayoutError> {
    let mut seen = BTreeSet::new();
    for item in items {
        for source in &item.source_nodes {
            if !seen.insert(source.node) {
                return Err(LayoutError::DuplicateTextParagraph(source.node));
            }
        }
    }
    if items.iter().any(|item| item.paragraph.node != item.anchor) {
        return Err(LayoutError::UnsupportedTextParagraph {
            node: grid_id,
            reason: "anonymous Grid text item anchor does not match its paragraph source",
        });
    }
    Ok(())
}

fn is_collapsible_html_whitespace(character: char) -> bool {
    matches!(character, '\t' | '\n' | '\u{000c}' | '\r' | ' ')
}

fn flush_anonymous_grid_text_run(
    grid_style: LayoutStyle,
    paragraph_style: &ParagraphStyle,
    anchor: &mut Option<NodeId>,
    source_nodes: &mut Vec<LayoutGridItemTextSource>,
    sources: &mut Vec<TextSource>,
    items: &mut Vec<AnonymousGridTextItem>,
) {
    let Some(anchor) = anchor.take() else {
        source_nodes.clear();
        sources.clear();
        return;
    };
    let paragraph = ParagraphInput {
        node: anchor,
        sources: std::mem::take(sources),
        style: paragraph_style.clone(),
        inline_styles: BTreeMap::new(),
        inline_run_styles: BTreeMap::new(),
    };
    items.push(AnonymousGridTextItem {
        anchor,
        source_nodes: std::mem::take(source_nodes),
        paragraph,
        style: LayoutStyle {
            display: LayoutDisplay::Block,
            direction: grid_style.direction,
            ..LayoutStyle::default()
        },
    });
}
