use std::collections::{BTreeMap, BTreeSet};

use parley::{InlineBox, InlineBoxKind, VerticalAlign};
use spinon_core::NodeId;

use crate::{ParagraphInput, TextLayoutError};

#[derive(Clone)]
struct InlineRange {
    start: usize,
    end: usize,
    depth: usize,
    order: usize,
    owners: Vec<NodeId>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum InlineEdge {
    End,
    Start,
}

struct InlineBoxEvent {
    index: usize,
    edge: InlineEdge,
    depth: usize,
    order: usize,
    owners: Vec<NodeId>,
    width: f32,
}

pub(super) struct InlineBoundary {
    pub(super) owners: Vec<NodeId>,
    pub(super) width: f32,
}

pub(super) fn prepare_inline_boxes(
    input: &ParagraphInput,
    normalized: &crate::normalize::NormalizedText,
) -> Result<(Vec<InlineBox>, BTreeMap<u64, InlineBoundary>), TextLayoutError> {
    for source in &input.sources {
        if let Some(&third_level_node) = source.inline_ancestors.get(2) {
            return Err(TextLayoutError::UnsupportedInlineDepth(third_level_node));
        }
        let mut seen = BTreeSet::new();
        if source
            .inline_ancestors
            .iter()
            .any(|ancestor| !seen.insert(*ancestor))
        {
            return Err(TextLayoutError::InvalidInlineHierarchy(source.node));
        }
    }

    validate_inline_source_order(&normalized.sources)?;

    let mut ranges = BTreeMap::<NodeId, InlineRange>::new();
    let mut next_order = 0;
    for source in &normalized.sources {
        if source.paragraph_utf8.start > source.paragraph_utf8.end
            || source.paragraph_utf8.end > normalized.text.len()
            || !normalized
                .text
                .is_char_boundary(source.paragraph_utf8.start)
            || !normalized.text.is_char_boundary(source.paragraph_utf8.end)
        {
            return Err(TextLayoutError::InvalidInlineHierarchy(source.node));
        }
        for (depth, &owner) in source.inline_ancestors.iter().enumerate() {
            let owners = source.inline_ancestors[..=depth].to_vec();
            let range = ranges.entry(owner).or_insert_with(|| {
                let order = next_order;
                next_order += 1;
                InlineRange {
                    start: source.paragraph_utf8.start,
                    end: source.paragraph_utf8.end,
                    depth,
                    order,
                    owners: owners.clone(),
                }
            });
            if range.depth != depth || range.owners != owners {
                return Err(TextLayoutError::InvalidInlineHierarchy(owner));
            }
            range.start = range.start.min(source.paragraph_utf8.start);
            range.end = range.end.max(source.paragraph_utf8.end);
        }
    }

    let mut events = Vec::new();
    for (&node, &style) in &input.inline_styles {
        if !style.is_valid() || style.is_empty() {
            return Err(TextLayoutError::InvalidInlineBoxStyle(node));
        }
        let Some(range) = ranges.get(&node) else {
            return Err(TextLayoutError::InlineBoxWithoutText(node));
        };
        if range.start == range.end {
            return Err(TextLayoutError::InlineBoxWithoutText(node));
        }
        for (edge, index, width) in [
            (InlineEdge::Start, range.start, style.inline_start()),
            (InlineEdge::End, range.end, style.inline_end()),
        ] {
            if width > 0.0 {
                events.push(InlineBoxEvent {
                    index,
                    edge,
                    depth: range.depth,
                    order: range.order,
                    owners: range.owners.clone(),
                    width,
                });
            }
        }
    }
    events.sort_by(|left, right| {
        left.index
            .cmp(&right.index)
            .then_with(|| match (left.edge, right.edge) {
                (InlineEdge::End, InlineEdge::Start) => std::cmp::Ordering::Less,
                (InlineEdge::Start, InlineEdge::End) => std::cmp::Ordering::Greater,
                (InlineEdge::Start, InlineEdge::Start) => left.depth.cmp(&right.depth),
                (InlineEdge::End, InlineEdge::End) => right.depth.cmp(&left.depth),
            })
            .then_with(|| match (left.edge, right.edge) {
                (InlineEdge::End, InlineEdge::End) => right.order.cmp(&left.order),
                _ => left.order.cmp(&right.order),
            })
    });

    let mut boxes = Vec::with_capacity(events.len());
    let mut boundaries = BTreeMap::new();
    for (index, event) in events.into_iter().enumerate() {
        let id = u64::try_from(index).map_err(|_| TextLayoutError::InlineBoxIdOverflow)?;
        boxes.push(InlineBox {
            id,
            kind: InlineBoxKind::InFlow,
            index: event.index,
            width: event.width,
            height: 0.0,
            baseline: Some(0.0),
            vertical_align: VerticalAlign::BASELINE,
        });
        boundaries.insert(
            id,
            InlineBoundary {
                owners: event.owners,
                width: event.width,
            },
        );
    }
    Ok((boxes, boundaries))
}

fn validate_inline_source_order(sources: &[crate::TextSourceRange]) -> Result<(), TextLayoutError> {
    let mut active_owners = Vec::new();
    let mut closed_owners = BTreeSet::new();

    for source in sources {
        let common_prefix = active_owners
            .iter()
            .zip(&source.inline_ancestors)
            .take_while(|(active, next)| active == next)
            .count();
        closed_owners.extend(active_owners[common_prefix..].iter().copied());

        for &owner in &source.inline_ancestors[common_prefix..] {
            if closed_owners.contains(&owner) {
                return Err(TextLayoutError::InvalidInlineHierarchy(owner));
            }
        }
        active_owners.clone_from(&source.inline_ancestors);
    }

    Ok(())
}
