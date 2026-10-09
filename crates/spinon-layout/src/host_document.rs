use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{
    EnvironmentRevision, HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, NodeId,
    StyleRevision,
};

use crate::{
    LayoutError, LayoutInput, LayoutInputRevision, LayoutNode, LayoutSourceRevision, LayoutStyle,
    RootSizingPolicy, Viewport,
};

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
            RootSizingPolicy::MatchViewport,
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
            RootSizingPolicy::ResolveWithinViewport,
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
        let root_node = snapshot
            .node(root)
            .ok_or(LayoutError::InvalidHostDocumentRoot(root.id()))?;
        if snapshot.parent(root) != Some(HostParent::Root)
            || !matches!(root_node.kind(), HostNodeKind::Element(_))
        {
            return Err(LayoutError::InvalidHostDocumentRoot(root.id()));
        }

        let mut nodes = Vec::new();
        let mut included = BTreeSet::new();
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
                    if !hidden {
                        return Err(LayoutError::UnsupportedTextNode(child.id()));
                    }
                    continue;
                }
                children.push(child.id());
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
                        snapshot
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
        })
    }
}

#[cfg(test)]
#[path = "host_document_tests.rs"]
mod tests;
