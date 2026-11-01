use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, NodeId};
use spinon_style::ComputedElementStyle;

use super::super::{RuntimeLayoutFailure, layout_failure};
use super::style::property;

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

pub(super) struct AnonymousGridTextSource {
    pub(super) anchor: NodeId,
    pub(super) text: String,
}

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
) -> Result<(BTreeSet<NodeId>, BTreeMap<NodeId, AnonymousGridTextSource>), RuntimeLayoutFailure> {
    let mut pending = vec![(root, false)];
    let mut visible_elements = BTreeSet::new();
    let mut anonymous_grid_text = BTreeMap::new();
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
            if !hidden
                && display == "grid"
                && let Some(source) =
                    validate_anonymous_grid_text_children(snapshot, handle, styles)?
            {
                if source.text != "AB" || !anonymous_grid_text.is_empty() {
                    return Err(layout_failure(
                        "unsupported_root_html_grid_text_item",
                        Some(handle.id().get()),
                        Some(
                            "this slice supports one visible direct text run with value AB"
                                .to_owned(),
                        ),
                    ));
                }
                anonymous_grid_text.insert(handle.id(), source);
            }
            for child in children {
                pending.push((child, hidden));
            }
        }
    }
    Ok((visible_elements, anonymous_grid_text))
}

fn validate_anonymous_grid_text_children(
    snapshot: &HostDocumentSnapshot,
    grid: HostNodeHandle,
    styles: &BTreeMap<NodeId, &ComputedElementStyle>,
) -> Result<Option<AnonymousGridTextSource>, RuntimeLayoutFailure> {
    let children = snapshot
        .children(grid)
        .ok_or_else(|| layout_failure("missing_node", Some(grid.id().get()), None))?;
    let mut text = String::new();
    let mut anchor = None;
    let mut has_visible_text = false;
    let mut has_visible_element = false;
    for child in children {
        let node = snapshot
            .node(child)
            .ok_or_else(|| layout_failure("missing_node", Some(child.id().get()), None))?;
        match node.kind() {
            HostNodeKind::Text(data) => {
                let source = String::from_utf16(data.code_units()).map_err(|_| {
                    layout_failure(
                        "root_html_default_render_text_encoding_invalid",
                        Some(child.id().get()),
                        None,
                    )
                })?;
                if source
                    .chars()
                    .any(|character| !is_collapsible_html_whitespace(character))
                {
                    has_visible_text = true;
                    anchor.get_or_insert(child.id());
                }
                text.push_str(&source);
            }
            HostNodeKind::Element(_) => {
                let style = styles.get(&child.id()).copied().ok_or_else(|| {
                    layout_failure("missing_computed_style", Some(child.id().get()), None)
                })?;
                has_visible_element |= property(style, "display")? != "none";
            }
        }
    }
    if !has_visible_text {
        return Ok(None);
    }
    if has_visible_element {
        return Err(layout_failure(
            "unsupported_root_html_grid_text_item",
            Some(grid.id().get()),
            Some(
                "visible element siblings of anonymous Grid text are outside this slice".to_owned(),
            ),
        ));
    }
    let anchor = anchor.ok_or_else(|| {
        layout_failure(
            "unsupported_root_html_grid_text_item",
            Some(grid.id().get()),
            Some("visible Grid text has no source-node anchor".to_owned()),
        )
    })?;
    Ok(Some(AnonymousGridTextSource { anchor, text }))
}

fn is_collapsible_html_whitespace(character: char) -> bool {
    matches!(character, '\t' | '\n' | '\u{000c}' | '\r' | ' ')
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
