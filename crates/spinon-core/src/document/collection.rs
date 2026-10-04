use std::collections::{HashMap, HashSet};

use crate::NodeId;

use super::{
    DocumentError, DocumentErrorKind, DocumentRevision, HostDocument, HostNodeHandle, HostNodeKind,
    ParentRef, RenderTreeRevision,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum VisitState {
    Visiting,
    Visited,
}

/// 검증된 도달성 계산 결과입니다. 문서 revision이 달라지면 적용할 수 없습니다.
#[derive(Debug, Eq, PartialEq)]
pub struct CollectionPlan {
    generation: super::DocumentGeneration,
    document_revision: DocumentRevision,
    render_tree_revision: RenderTreeRevision,
    reclaimed_handles: Vec<HostNodeHandle>,
}

impl CollectionPlan {
    /// 회수할 노드 handle을 NodeId 순서로 반환합니다.
    pub fn reclaimed_handles(&self) -> &[HostNodeHandle] {
        &self.reclaimed_handles
    }

    pub fn reclaimed_node_count(&self) -> usize {
        self.reclaimed_handles.len()
    }
}

impl HostDocument {
    /// 연결 성분의 root를 보존하고 다른 분리 노드를 회수할 계획을 만듭니다.
    ///
    /// `roots`에는 V8 weak wrapper scan과 native external root lease가 관찰한
    /// handle을 함께 전달합니다. 이 단계는 문서를 변경하지 않습니다.
    /// caller는 계획을 commit할 때까지 해당 root 집합을 바꾸는 실행·재진입을 막아야
    /// 합니다. 제품 경로는 owner safe point 안에서 plan과 commit을 연속 실행합니다.
    pub fn plan_collection(
        &self,
        roots: &[HostNodeHandle],
    ) -> Result<CollectionPlan, DocumentError> {
        self.validate_graph()?;
        let reachable = self.reachable_nodes(roots)?;
        let mut reclaimed_handles = Vec::new();
        reclaimed_handles
            .try_reserve(self.nodes.len().saturating_sub(reachable.len()))
            .map_err(|_| collection_error(DocumentErrorKind::CollectionAllocationFailed))?;

        for id in self.nodes.keys() {
            if !reachable.contains(id) {
                reclaimed_handles.push(HostNodeHandle {
                    generation: self.generation,
                    id: *id,
                });
            }
        }

        Ok(CollectionPlan {
            generation: self.generation,
            document_revision: self.document_revision,
            render_tree_revision: self.render_tree_revision,
            reclaimed_handles,
        })
    }

    /// 미리 검증한 회수 계획을 적용합니다. 회수는 DOM/render revision을 바꾸지 않습니다.
    pub fn commit_collection(&mut self, plan: &CollectionPlan) -> Result<(), DocumentError> {
        if plan.generation != self.generation
            || plan.document_revision != self.document_revision
            || plan.render_tree_revision != self.render_tree_revision
        {
            return Err(collection_error(DocumentErrorKind::StaleCollectionPlan));
        }

        let mut previous = None;
        for handle in plan.reclaimed_handles() {
            let Some(node) = self.nodes.get(&handle.id) else {
                return Err(collection_error(DocumentErrorKind::StaleCollectionPlan));
            };
            if handle.generation != self.generation
                || previous.is_some_and(|id| id >= handle.id)
                || node.parent == Some(ParentRef::Root)
            {
                return Err(collection_error(DocumentErrorKind::StaleCollectionPlan));
            }
            previous = Some(handle.id);
        }

        for handle in plan.reclaimed_handles() {
            self.nodes.remove(&handle.id);
        }
        Ok(())
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn validate_graph(&self) -> Result<(), DocumentError> {
        if self.root_children.len() > self.nodes.len() {
            return Err(collection_error(DocumentErrorKind::InvalidCollectionGraph));
        }

        let mut root_children = HashSet::new();
        root_children
            .try_reserve(self.root_children.len())
            .map_err(|_| collection_error(DocumentErrorKind::CollectionAllocationFailed))?;
        for id in &self.root_children {
            let Some(node) = self.nodes.get(id) else {
                return Err(collection_error(DocumentErrorKind::InvalidCollectionGraph));
            };
            if !root_children.insert(*id) || node.parent != Some(ParentRef::Root) {
                return Err(collection_error(DocumentErrorKind::InvalidCollectionGraph));
            }
        }

        let mut edge_count = 0usize;
        for node in self.nodes.values() {
            edge_count = edge_count
                .checked_add(node.children.len())
                .filter(|count| *count <= self.nodes.len())
                .ok_or_else(|| collection_error(DocumentErrorKind::InvalidCollectionGraph))?;
        }
        let mut edges = HashSet::new();
        edges
            .try_reserve(edge_count)
            .map_err(|_| collection_error(DocumentErrorKind::CollectionAllocationFailed))?;

        for (id, node) in &self.nodes {
            if node.id != *id {
                return Err(collection_error(DocumentErrorKind::InvalidCollectionGraph));
            }
            match node.parent {
                None => {}
                Some(ParentRef::Root) => {
                    if !root_children.contains(id) {
                        return Err(collection_error(DocumentErrorKind::InvalidCollectionGraph));
                    }
                }
                Some(ParentRef::Node(parent_id)) => {
                    let Some(parent) = self.nodes.get(&parent_id) else {
                        return Err(collection_error(DocumentErrorKind::InvalidCollectionGraph));
                    };
                    if parent.owner != node.owner
                        || !matches!(parent.kind, HostNodeKind::Element(_))
                    {
                        return Err(collection_error(DocumentErrorKind::InvalidCollectionGraph));
                    }
                }
            }

            for child_id in &node.children {
                let Some(child) = self.nodes.get(child_id) else {
                    return Err(collection_error(DocumentErrorKind::InvalidCollectionGraph));
                };
                if child.parent != Some(ParentRef::Node(*id)) || !edges.insert((*id, *child_id)) {
                    return Err(collection_error(DocumentErrorKind::InvalidCollectionGraph));
                }
            }
        }

        for (id, node) in &self.nodes {
            if let Some(ParentRef::Node(parent_id)) = node.parent
                && !edges.contains(&(parent_id, *id))
            {
                return Err(collection_error(DocumentErrorKind::InvalidCollectionGraph));
            }
        }

        self.validate_acyclic()
    }

    fn validate_acyclic(&self) -> Result<(), DocumentError> {
        let mut states = HashMap::new();
        states
            .try_reserve(self.nodes.len())
            .map_err(|_| collection_error(DocumentErrorKind::CollectionAllocationFailed))?;
        let mut stack = Vec::new();
        stack
            .try_reserve(self.nodes.len())
            .map_err(|_| collection_error(DocumentErrorKind::CollectionAllocationFailed))?;

        for start in self.nodes.keys().copied() {
            if states.get(&start) == Some(&VisitState::Visited) {
                continue;
            }
            states.insert(start, VisitState::Visiting);
            stack.push((start, 0usize));

            while let Some((current, next_child)) = stack.last_mut() {
                let current_id = *current;
                let child_count = self.nodes[&current_id].children.len();
                if *next_child == child_count {
                    stack.pop();
                    states.insert(current_id, VisitState::Visited);
                    continue;
                }

                let child = self.nodes[&current_id].children[*next_child];
                *next_child += 1;
                match states.get(&child) {
                    Some(VisitState::Visiting) => {
                        return Err(collection_error(DocumentErrorKind::InvalidCollectionGraph));
                    }
                    Some(VisitState::Visited) => {}
                    None => {
                        states.insert(child, VisitState::Visiting);
                        stack.push((child, 0));
                    }
                }
            }
        }

        Ok(())
    }

    fn reachable_nodes(&self, roots: &[HostNodeHandle]) -> Result<HashSet<NodeId>, DocumentError> {
        if roots.len() > self.nodes.len() {
            return Err(collection_error(DocumentErrorKind::InvalidCollectionGraph));
        }
        let mut explicit_roots = HashSet::new();
        explicit_roots
            .try_reserve(roots.len())
            .map_err(|_| collection_error(DocumentErrorKind::CollectionAllocationFailed))?;
        for handle in roots {
            if handle.generation != self.generation {
                return Err(collection_error(DocumentErrorKind::StaleGeneration {
                    expected: self.generation.get(),
                    actual: handle.generation.get(),
                }));
            }
            if !explicit_roots.insert(handle.id) {
                return Err(collection_error(
                    DocumentErrorKind::DuplicateCollectionRoot(handle.id),
                ));
            }
            if !self.nodes.contains_key(&handle.id) {
                return Err(collection_error(DocumentErrorKind::UnknownNode(handle.id)));
            }
        }

        let mut reachable = HashSet::new();
        reachable
            .try_reserve(self.nodes.len())
            .map_err(|_| collection_error(DocumentErrorKind::CollectionAllocationFailed))?;
        let mut pending = Vec::new();
        pending
            .try_reserve(self.nodes.len())
            .map_err(|_| collection_error(DocumentErrorKind::CollectionAllocationFailed))?;
        for id in self.root_children.iter().copied().chain(explicit_roots) {
            if reachable.insert(id) {
                pending.push(id);
            }
        }

        while let Some(id) = pending.pop() {
            let node = self
                .nodes
                .get(&id)
                .ok_or_else(|| collection_error(DocumentErrorKind::InvalidCollectionGraph))?;
            if let Some(ParentRef::Node(parent)) = node.parent
                && reachable.insert(parent)
            {
                pending.push(parent);
            }
            for child in &node.children {
                if reachable.insert(*child) {
                    pending.push(*child);
                }
            }
        }

        Ok(reachable)
    }
}

fn collection_error(kind: DocumentErrorKind) -> DocumentError {
    DocumentError::batch(kind)
}
