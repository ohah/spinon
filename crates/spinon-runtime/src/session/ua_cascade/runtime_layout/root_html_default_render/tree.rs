use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, NodeId};
use spinon_style::ComputedElementStyle;

use super::super::{RuntimeLayoutFailure, layout_failure};
use super::style::property;

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

pub(super) fn validate_root(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
) -> Result<(), RuntimeLayoutFailure> {
    let element = snapshot
        .node(root)
        .and_then(|node| match node.kind() {
            HostNodeKind::Element(element) => Some(element),
            HostNodeKind::Text(_) => None,
        })
        .ok_or_else(|| layout_failure("invalid_root_html", Some(root.id().get()), None))?;
    if element.namespace() != HTML_NAMESPACE || element.local_name() != "html" {
        return Err(layout_failure(
            "invalid_root_html",
            Some(root.id().get()),
            Some("documentElement must be HTML html".to_owned()),
        ));
    }
    Ok(())
}

pub(super) fn validate_visible_tree(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &BTreeMap<NodeId, &ComputedElementStyle>,
) -> Result<BTreeSet<NodeId>, RuntimeLayoutFailure> {
    let mut pending = vec![(root, false)];
    let mut visible_elements = BTreeSet::new();
    while let Some((handle, ancestor_hidden)) = pending.pop() {
        let node = snapshot
            .node(handle)
            .ok_or_else(|| layout_failure("missing_node", Some(handle.id().get()), None))?;
        let HostNodeKind::Element(element) = node.kind() else {
            continue;
        };
        let style = styles.get(&handle.id()).copied().ok_or_else(|| {
            layout_failure("missing_computed_style", Some(handle.id().get()), None)
        })?;
        let display = property(style, "display")?;
        let hidden = ancestor_hidden || display == "none";
        if !hidden {
            visible_elements.insert(handle.id());
            if element.namespace() != HTML_NAMESPACE {
                return Err(layout_failure(
                    "unsupported_visible_root_html_element",
                    Some(handle.id().get()),
                    Some(element.local_name().to_owned()),
                ));
            }
            if is_non_rendered_html_element(element.local_name()) {
                return Err(layout_failure(
                    "unsupported_visible_root_html_element",
                    Some(handle.id().get()),
                    Some(element.local_name().to_owned()),
                ));
            }
            if display == "inline"
                && !matches!(element.local_name(), "span" | "strong" | "b" | "em" | "i")
            {
                return Err(layout_failure(
                    "unsupported_visible_root_html_inline_element",
                    Some(handle.id().get()),
                    Some(element.local_name().to_owned()),
                ));
            }
        }
        if let Some(children) = snapshot.children(handle) {
            pending.extend(children.map(|child| (child, hidden)));
        }
    }
    Ok(visible_elements)
}

fn is_non_rendered_html_element(local_name: &str) -> bool {
    matches!(
        local_name,
        "area"
            | "audio"
            | "base"
            | "br"
            | "canvas"
            | "embed"
            | "hr"
            | "iframe"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "object"
            | "script"
            | "source"
            | "style"
            | "svg"
            | "textarea"
            | "track"
            | "video"
    )
}

pub(super) fn collect_handles(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
) -> Result<BTreeMap<NodeId, HostNodeHandle>, RuntimeLayoutFailure> {
    let mut handles = BTreeMap::new();
    let mut pending = vec![root];
    while let Some(handle) = pending.pop() {
        if handles.insert(handle.id(), handle).is_some() {
            return Err(layout_failure(
                "duplicate_root_html_document_node",
                Some(handle.id().get()),
                None,
            ));
        }
        let node = snapshot
            .node(handle)
            .ok_or_else(|| layout_failure("missing_node", Some(handle.id().get()), None))?;
        if let Some(children) = snapshot.children(handle) {
            pending.extend(children);
        } else if !matches!(node.kind(), HostNodeKind::Text(_)) {
            return Err(layout_failure(
                "invalid_root_html_document_tree",
                Some(handle.id().get()),
                None,
            ));
        }
    }
    Ok(handles)
}
