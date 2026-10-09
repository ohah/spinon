use spinon_core::{DocumentRevision, HostDocumentSnapshot, HostNodeHandle, HostNodeKind};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum RuntimeStyleInvalidation {
    Full,
    Unchanged {
        generation: u64,
        from_document_revision: DocumentRevision,
    },
    Subtrees {
        generation: u64,
        from_document_revision: DocumentRevision,
        dirty_roots: Vec<HostNodeHandle>,
    },
}

impl RuntimeStyleInvalidation {
    pub(super) fn base_revision(&self) -> Option<(u64, DocumentRevision)> {
        match self {
            Self::Full => None,
            Self::Unchanged {
                generation,
                from_document_revision,
            }
            | Self::Subtrees {
                generation,
                from_document_revision,
                ..
            } => Some((*generation, *from_document_revision)),
        }
    }

    pub(super) fn dirty_roots(&self) -> &[HostNodeHandle] {
        match self {
            Self::Subtrees { dirty_roots, .. } => dirty_roots,
            Self::Full | Self::Unchanged { .. } => &[],
        }
    }
}

pub(super) fn classify_style_invalidation(
    previous: Option<&HostDocumentSnapshot>,
    current: &HostDocumentSnapshot,
) -> RuntimeStyleInvalidation {
    let Some(previous) = previous else {
        return RuntimeStyleInvalidation::Full;
    };
    let from_document_revision = previous.document_revision();
    if previous.generation() != current.generation()
        || current.document_revision().get() <= from_document_revision.get()
    {
        return RuntimeStyleInvalidation::Full;
    }

    let previous_roots = previous.root_children().collect::<Vec<_>>();
    let current_roots = current.root_children().collect::<Vec<_>>();
    if previous_roots.len() != 1
        || current_roots.len() != 1
        || previous_roots[0].id() != current_roots[0].id()
    {
        return RuntimeStyleInvalidation::Full;
    }

    let mut changed_style_nodes = BTreeSet::new();
    let mut pending = vec![(previous_roots[0], current_roots[0])];
    while let Some((previous_handle, current_handle)) = pending.pop() {
        let (Some(previous_node), Some(current_node)) =
            (previous.node(previous_handle), current.node(current_handle))
        else {
            return RuntimeStyleInvalidation::Full;
        };
        if previous_node.id() != current_node.id()
            || previous_node.owner() != current_node.owner()
            || previous.parent(previous_handle) != current.parent(current_handle)
            || previous_node.children() != current_node.children()
        {
            return RuntimeStyleInvalidation::Full;
        }

        match (previous_node.kind(), current_node.kind()) {
            (HostNodeKind::Text(previous_text), HostNodeKind::Text(current_text)) => {
                if previous_text != current_text {
                    return RuntimeStyleInvalidation::Full;
                }
            }
            (HostNodeKind::Element(previous_element), HostNodeKind::Element(current_element)) => {
                if previous_element.namespace() != current_element.namespace()
                    || previous_element.local_name() != current_element.local_name()
                    || previous_element.states().collect::<Vec<_>>()
                        != current_element.states().collect::<Vec<_>>()
                {
                    return RuntimeStyleInvalidation::Full;
                }

                let html_element = previous_element.namespace() == "http://www.w3.org/1999/xhtml";
                let Some(previous_style) = style_attribute(previous_element, html_element) else {
                    return RuntimeStyleInvalidation::Full;
                };
                let Some(current_style) = style_attribute(current_element, html_element) else {
                    return RuntimeStyleInvalidation::Full;
                };
                if !same_non_style_attributes(previous_element, current_element, html_element) {
                    return RuntimeStyleInvalidation::Full;
                }
                if previous_style != current_style {
                    changed_style_nodes.insert(current_handle);
                }

                let Some(previous_children) = previous
                    .children(previous_handle)
                    .map(|children| children.collect::<Vec<_>>())
                else {
                    return RuntimeStyleInvalidation::Full;
                };
                let Some(current_children) = current
                    .children(current_handle)
                    .map(|children| children.collect::<Vec<_>>())
                else {
                    return RuntimeStyleInvalidation::Full;
                };
                pending.extend(previous_children.into_iter().zip(current_children));
            }
            _ => return RuntimeStyleInvalidation::Full,
        }
    }

    if changed_style_nodes.is_empty() {
        return RuntimeStyleInvalidation::Unchanged {
            generation: current.generation().get(),
            from_document_revision,
        };
    }

    let mut dirty_roots = changed_style_nodes
        .iter()
        .copied()
        .filter(|handle| !has_changed_ancestor(current, *handle, &changed_style_nodes))
        .collect::<Vec<_>>();
    dirty_roots.sort_by_key(|handle| handle.id());
    RuntimeStyleInvalidation::Subtrees {
        generation: current.generation().get(),
        from_document_revision,
        dirty_roots,
    }
}

fn style_attribute(
    element: &spinon_core::HostElement,
    html_element: bool,
) -> Option<Option<&spinon_core::DomString>> {
    if !html_element {
        return Some(None);
    }
    let mut style_attributes = element.attributes().iter().filter(|(name, _)| {
        name.namespace().is_none() && name.local_name().eq_ignore_ascii_case("style")
    });
    let style = style_attributes.next().map(|(_, value)| value);
    style_attributes.next().is_none().then_some(style)
}

fn same_non_style_attributes(
    previous: &spinon_core::HostElement,
    current: &spinon_core::HostElement,
    html_element: bool,
) -> bool {
    let is_inline_style = |name: &spinon_core::AttributeName| {
        html_element
            && name.namespace().is_none()
            && name.local_name().eq_ignore_ascii_case("style")
    };
    previous
        .attributes()
        .iter()
        .filter(|(name, _)| !is_inline_style(name))
        .eq(current
            .attributes()
            .iter()
            .filter(|(name, _)| !is_inline_style(name)))
}

fn has_changed_ancestor(
    snapshot: &HostDocumentSnapshot,
    node: HostNodeHandle,
    changed: &BTreeSet<HostNodeHandle>,
) -> bool {
    let mut handle = node;
    while let Some(spinon_core::HostParent::Node(parent)) = snapshot.parent(handle) {
        if changed.contains(&parent) {
            return true;
        }
        handle = parent;
    }
    false
}
