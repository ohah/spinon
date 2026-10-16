use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{
    EnvironmentRevision, HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, NodeId,
    StyleRevision,
};
use spinon_text_layout::{ParagraphInput, ParagraphStyle, TextSource};

use crate::{
    LayoutError, LayoutInput, LayoutInputRevision, LayoutNode, LayoutSourceRevision, LayoutStyle,
    RootSizingPolicy, Viewport,
};

struct HostDocumentLayoutProfile<'a> {
    root_sizing: RootSizingPolicy,
    paragraph_styles: Option<&'a BTreeMap<NodeId, ParagraphStyle>>,
}

impl LayoutInput {
    /// HostDocument의 요소 하위 트리를 순서 보존 레이아웃 snapshot으로 만듭니다.
    ///
    /// root는 HostRoot 직속 요소여야 합니다. 텍스트 intrinsic measurement가 없으므로
    /// 보이는 텍스트는 거부하고 `display:none` 아래 텍스트는 입력에서 제외합니다.
    pub fn from_host_document(
        snapshot: &HostDocumentSnapshot,
        root: HostNodeHandle,
        viewport: Viewport,
        styles: &BTreeMap<NodeId, LayoutStyle>,
        style_revision: StyleRevision,
        environment_revision: EnvironmentRevision,
    ) -> Result<Self, LayoutError> {
        Self::from_host_document_with_root_sizing(
            snapshot,
            root,
            viewport,
            styles,
            style_revision,
            environment_revision,
            RootSizingPolicy::Match,
        )
    }

    /// HostRoot 직속 요소를 viewport containing block 안에서 CSS 크기로 계산합니다.
    pub fn from_host_document_with_viewport_containing_block(
        snapshot: &HostDocumentSnapshot,
        root: HostNodeHandle,
        viewport: Viewport,
        styles: &BTreeMap<NodeId, LayoutStyle>,
        style_revision: StyleRevision,
        environment_revision: EnvironmentRevision,
    ) -> Result<Self, LayoutError> {
        Self::from_host_document_with_root_sizing(
            snapshot,
            root,
            viewport,
            styles,
            style_revision,
            environment_revision,
            RootSizingPolicy::ResolveWithin,
        )
    }

    /// Block formatting을 위해 가상 viewport containing block을 둡니다.
    /// HostRoot 직속 요소도 일반 자식 Block처럼 margin과 auto width를 계산합니다.
    pub fn from_host_document_with_block_formatting_viewport(
        snapshot: &HostDocumentSnapshot,
        root: HostNodeHandle,
        viewport: Viewport,
        styles: &BTreeMap<NodeId, LayoutStyle>,
        style_revision: StyleRevision,
        environment_revision: EnvironmentRevision,
    ) -> Result<Self, LayoutError> {
        Self::from_host_document_with_root_sizing(
            snapshot,
            root,
            viewport,
            styles,
            style_revision,
            environment_revision,
            RootSizingPolicy::BlockFormatting,
        )
    }

    /// 고정 text profile이 제공한 Block paragraph를 HostDocument의 text/span 순서와 연결합니다.
    ///
    /// paragraph 안에는 direct text와 상속형 HTML span만 허용합니다. visible non-span
    /// element child와 unpaired UTF-16 surrogate는 node가 식별되는 오류로 거부합니다.
    pub fn from_host_document_with_text_paragraphs(
        snapshot: &HostDocumentSnapshot,
        root: HostNodeHandle,
        viewport: Viewport,
        styles: &BTreeMap<NodeId, LayoutStyle>,
        paragraph_styles: &BTreeMap<NodeId, ParagraphStyle>,
        style_revision: StyleRevision,
        environment_revision: EnvironmentRevision,
    ) -> Result<Self, LayoutError> {
        Self::from_host_document_with_text_profile(
            snapshot,
            root,
            viewport,
            styles,
            style_revision,
            environment_revision,
            HostDocumentLayoutProfile {
                root_sizing: RootSizingPolicy::Match,
                paragraph_styles: Some(paragraph_styles),
            },
        )
    }

    fn from_host_document_with_root_sizing(
        snapshot: &HostDocumentSnapshot,
        root: HostNodeHandle,
        viewport: Viewport,
        styles: &BTreeMap<NodeId, LayoutStyle>,
        style_revision: StyleRevision,
        environment_revision: EnvironmentRevision,
        root_sizing: RootSizingPolicy,
    ) -> Result<Self, LayoutError> {
        Self::from_host_document_with_text_profile(
            snapshot,
            root,
            viewport,
            styles,
            style_revision,
            environment_revision,
            HostDocumentLayoutProfile {
                root_sizing,
                paragraph_styles: None,
            },
        )
    }

    fn from_host_document_with_text_profile(
        snapshot: &HostDocumentSnapshot,
        root: HostNodeHandle,
        viewport: Viewport,
        styles: &BTreeMap<NodeId, LayoutStyle>,
        style_revision: StyleRevision,
        environment_revision: EnvironmentRevision,
        profile: HostDocumentLayoutProfile<'_>,
    ) -> Result<Self, LayoutError> {
        let HostDocumentLayoutProfile {
            root_sizing,
            paragraph_styles,
        } = profile;
        let root_node = snapshot
            .node(root)
            .ok_or(LayoutError::InvalidHostDocumentRoot(root.id()))?;
        if snapshot.parent(root) != Some(HostParent::Root)
            || !matches!(root_node.kind(), HostNodeKind::Element(_))
        {
            return Err(LayoutError::InvalidHostDocumentRoot(root.id()));
        }

        let mut inline_nodes = BTreeSet::new();
        let mut text_source_nodes = BTreeSet::new();
        let mut excluded_layout_nodes = BTreeSet::new();
        let mut text_paragraphs = BTreeMap::new();
        if let Some(paragraph_styles) = paragraph_styles {
            let handles = collect_host_handles(snapshot, root)?;
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
                if layout_style.display == crate::LayoutDisplay::None {
                    continue;
                }
                if layout_style.display != crate::LayoutDisplay::Block {
                    return Err(LayoutError::UnsupportedTextParagraph {
                        node: paragraph_id,
                        reason: "paragraph owner는 display:block이어야 합니다",
                    });
                }
                if is_hidden_by_ancestor(snapshot, paragraph_handle, root, styles)? {
                    continue;
                }

                let mut sources = Vec::new();
                ParagraphSourceCollector {
                    snapshot,
                    styles,
                    inline_nodes: &mut inline_nodes,
                    text_source_nodes: &mut text_source_nodes,
                    excluded_layout_nodes: &mut excluded_layout_nodes,
                }
                .collect(paragraph_handle, paragraph_id, &mut sources)?;
                text_paragraphs.insert(
                    paragraph_id,
                    ParagraphInput {
                        node: paragraph_id,
                        sources,
                        style: paragraph_style.clone(),
                    },
                );
            }
        }

        let mut nodes = Vec::new();
        let mut included = excluded_layout_nodes.clone();
        let mut pending = vec![(root, false)];
        while let Some((handle, ancestor_hidden)) = pending.pop() {
            if !included.insert(handle.id()) {
                return Err(LayoutError::DuplicateNode(handle.id()));
            }

            let node = snapshot
                .node(handle)
                .ok_or(LayoutError::InvalidHostDocumentRoot(handle.id()))?;
            if matches!(node.kind(), HostNodeKind::Text(_)) {
                if ancestor_hidden {
                    continue;
                }
                return Err(LayoutError::UnsupportedTextNode(handle.id()));
            }

            let style = styles
                .get(&handle.id())
                .copied()
                .ok_or(LayoutError::MissingStyle(handle.id()))?;
            let hidden = ancestor_hidden || style.display == crate::LayoutDisplay::None;

            let child_handles = snapshot
                .children(handle)
                .ok_or(LayoutError::InvalidHostDocumentRoot(handle.id()))?
                .collect::<Vec<_>>();
            let mut children = Vec::with_capacity(child_handles.len());
            for child in &child_handles {
                let Some(child_node) = snapshot.node(*child) else {
                    return Err(LayoutError::MissingChild {
                        parent: handle.id(),
                        child: child.id(),
                    });
                };
                if matches!(child_node.kind(), HostNodeKind::Text(_)) {
                    if !hidden && !text_source_nodes.contains(&child.id()) {
                        return Err(LayoutError::UnsupportedTextNode(child.id()));
                    }
                    continue;
                }
                if inline_nodes.contains(&child.id()) {
                    continue;
                }
                if excluded_layout_nodes.contains(&child.id()) {
                    continue;
                }
                children.push(child.id());
            }

            if inline_nodes.contains(&handle.id()) {
                pending.extend(
                    child_handles
                        .into_iter()
                        .rev()
                        .filter(|child| {
                            !excluded_layout_nodes.contains(&child.id())
                                && snapshot.node(*child).is_some_and(|node| {
                                    matches!(node.kind(), HostNodeKind::Element(_))
                                })
                        })
                        .map(|child| (child, hidden)),
                );
                continue;
            }

            nodes.push(LayoutNode {
                id: handle.id(),
                children,
                style,
            });
            pending.extend(
                child_handles
                    .into_iter()
                    .rev()
                    .filter(|child| {
                        !excluded_layout_nodes.contains(&child.id())
                            && snapshot
                                .node(*child)
                                .is_some_and(|node| matches!(node.kind(), HostNodeKind::Element(_)))
                    })
                    .map(|child| (child, hidden)),
            );
        }

        if let Some(id) = styles.keys().find(|id| !included.contains(*id)) {
            return Err(LayoutError::UnknownStyleNode(*id));
        }
        nodes.sort_by_key(|node| node.id);

        Ok(Self {
            root: root.id(),
            revision: LayoutInputRevision::new(
                LayoutSourceRevision::HostDocument {
                    generation: snapshot.generation(),
                    document: snapshot.document_revision(),
                    render_tree: snapshot.render_tree_revision(),
                },
                style_revision,
                environment_revision,
            ),
            viewport,
            root_sizing,
            nodes,
            css_math: Vec::new(),
            positioning: Default::default(),
            grid_containers: Default::default(),
            grid_items: Default::default(),
            text_paragraphs,
        })
    }
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
}

impl ParagraphSourceCollector<'_> {
    fn collect(
        &mut self,
        paragraph: HostNodeHandle,
        paragraph_id: NodeId,
        sources: &mut Vec<TextSource>,
    ) -> Result<(), LayoutError> {
        let mut pending = self
            .snapshot
            .children(paragraph)
            .ok_or(LayoutError::InvalidHostDocumentRoot(paragraph_id))?
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .map(|handle| (handle, Vec::<NodeId>::new()))
            .collect::<Vec<_>>();

        while let Some((handle, ancestors)) = pending.pop() {
            let node = self
                .snapshot
                .node(handle)
                .ok_or(LayoutError::InvalidHostDocumentRoot(handle.id()))?;
            match node.kind() {
                HostNodeKind::Text(data) => {
                    if !self.text_source_nodes.insert(handle.id()) {
                        return Err(LayoutError::DuplicateTextParagraph(handle.id()));
                    }
                    let text = String::from_utf16(data.code_units())
                        .map_err(|_| LayoutError::InvalidTextEncoding(handle.id()))?;
                    if text.is_empty() {
                        continue;
                    }
                    sources.push(TextSource {
                        node: handle.id(),
                        inline_ancestors: ancestors,
                        text,
                    });
                }
                HostNodeKind::Element(element) => {
                    let style = self
                        .styles
                        .get(&handle.id())
                        .copied()
                        .ok_or(LayoutError::MissingStyle(handle.id()))?;
                    if style.display == crate::LayoutDisplay::None {
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
                            reason: "visible child element은 상속형 HTML span이어야 합니다",
                        });
                    }
                    if !ancestors.is_empty() {
                        return Err(LayoutError::UnsupportedTextParagraph {
                            node: paragraph_id,
                            reason: "C15.1에서는 중첩 inline span을 지원하지 않습니다",
                        });
                    }
                    if !is_plain_inline_layout_style(style) {
                        return Err(LayoutError::UnsupportedTextParagraph {
                            node: paragraph_id,
                            reason: "C15.1 span은 border·padding·margin 등 box style을 가질 수 없습니다",
                        });
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
}

fn is_plain_inline_layout_style(style: LayoutStyle) -> bool {
    style.display == crate::LayoutDisplay::Inline
        && style
            == LayoutStyle {
                display: crate::LayoutDisplay::Inline,
                ..LayoutStyle::default()
            }
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
        if style.display == crate::LayoutDisplay::None {
            return Ok(true);
        }
        if current == root {
            return Ok(false);
        }
        match snapshot.parent(current) {
            Some(HostParent::Node(parent)) => current = parent,
            Some(HostParent::Root) | None => {
                return Ok(false);
            }
        }
    }
}

#[cfg(test)]
#[path = "host_document_tests.rs"]
mod tests;
