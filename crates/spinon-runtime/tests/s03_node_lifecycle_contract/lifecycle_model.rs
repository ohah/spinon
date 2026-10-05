//! 제품 collector나 V8 GC가 아니라 S03.3 계약을 고정하는 test-only reference model입니다.
use super::{fixture, limits};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct RootLeaseId {
    pub(super) session: u64,
    pub(super) sequence: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Registration {
    pub(super) wrapper_identity: u64,
    pub(super) facade_id: i32,
    pub(super) weak_handle_empty: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ExternalRoot {
    pub(super) session: u64,
    pub(super) generation: u64,
    pub(super) node: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct IdAllocator {
    pub(super) next: Option<i32>,
}

impl IdAllocator {
    pub(super) fn reserve(&mut self) -> Option<i32> {
        let issued = self.next?;
        self.next = issued.checked_add(1);
        Some(issued)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Model {
    pub(super) session: u64,
    pub(super) generation: u64,
    pub(super) nodes: BTreeMap<u64, fixture::Node>,
    pub(super) children: BTreeMap<u64, Vec<u64>>,
    pub(super) node_facade_ids: BTreeMap<u64, i32>,
    pub(super) host_root_children: BTreeSet<u64>,
    pub(super) wrappers: BTreeMap<u64, Registration>,
    pub(super) external_roots: BTreeMap<RootLeaseId, ExternalRoot>,
    pub(super) document_revision: u64,
    pub(super) render_tree_revision: u64,
    pub(super) next_wrapper_identity: u64,
    pub(super) next_facade_id: Option<i32>,
    pub(super) next_external_root_sequence: Option<u64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum CollectionResult {
    Deferred,
    Completed {
        reclaimed_nodes: usize,
        reclaimed_string_units: u64,
    },
}

impl Model {
    pub(super) fn from_case(case: &fixture::TreeCase, session: u64) -> Self {
        let nodes = case
            .nodes
            .iter()
            .map(|node| (node.id, *node))
            .collect::<BTreeMap<_, _>>();
        let mut children = BTreeMap::<u64, Vec<u64>>::new();
        for node in nodes.values() {
            if let Some(parent) = node.parent {
                children.entry(parent).or_default().push(node.id);
            }
        }
        Self {
            session,
            generation: 1,
            nodes,
            children,
            node_facade_ids: BTreeMap::new(),
            host_root_children: case.host_root_children.iter().copied().collect(),
            wrappers: BTreeMap::new(),
            external_roots: BTreeMap::new(),
            document_revision: 0,
            render_tree_revision: 0,
            next_wrapper_identity: 1,
            next_facade_id: Some(1),
            next_external_root_sequence: Some(1),
        }
    }

    pub(super) fn lookup_or_create_wrapper(&mut self, node: u64) -> u64 {
        assert!(self.nodes.contains_key(&node));
        let facade_id = if let Some(facade_id) = self.node_facade_ids.get(&node) {
            *facade_id
        } else {
            let facade_id = self
                .next_facade_id
                .expect("테스트 façade ID 공간이 충분해야 합니다");
            self.next_facade_id = facade_id.checked_add(1);
            self.node_facade_ids.insert(node, facade_id);
            facade_id
        };
        if let Some(registration) = self.wrappers.get(&node)
            && !registration.weak_handle_empty
        {
            return registration.wrapper_identity;
        }

        let identity = self.next_wrapper_identity;
        self.next_wrapper_identity = self
            .next_wrapper_identity
            .checked_add(1)
            .expect("테스트 wrapper identity 공간이 충분해야 합니다");
        self.wrappers.insert(
            node,
            Registration {
                wrapper_identity: identity,
                facade_id,
                weak_handle_empty: false,
            },
        );
        identity
    }

    pub(super) fn v8_automatically_resets_weak_handle(&mut self, node: u64) {
        if let Some(registration) = self.wrappers.get_mut(&node) {
            registration.weak_handle_empty = true;
        }
    }

    pub(super) fn create_node_and_wrapper(
        &mut self,
        node: fixture::Node,
        facade_id: i32,
        wrapper_preparation_succeeds: bool,
        document_commit_succeeds: bool,
    ) -> bool {
        if self.nodes.contains_key(&node.id)
            || node.parent.is_some()
            || facade_id <= 0
            || self.next_facade_id.is_none_or(|next| facade_id < next)
            || self
                .node_facade_ids
                .values()
                .any(|existing| *existing == facade_id)
            || !wrapper_preparation_succeeds
            || !document_commit_succeeds
        {
            return false;
        }

        let wrapper_identity = self.next_wrapper_identity;
        let Some(next_identity) = wrapper_identity.checked_add(1) else {
            return false;
        };
        let Some(next_revision) = self.document_revision.checked_add(1) else {
            return false;
        };
        self.nodes.insert(node.id, node);
        self.node_facade_ids.insert(node.id, facade_id);
        self.wrappers.insert(
            node.id,
            Registration {
                wrapper_identity,
                facade_id,
                weak_handle_empty: false,
            },
        );
        self.next_wrapper_identity = next_identity;
        self.next_facade_id = facade_id.checked_add(1);
        self.document_revision = next_revision;
        true
    }

    pub(super) fn register_external_root(&mut self, node: u64) -> Option<RootLeaseId> {
        if !self.nodes.contains_key(&node)
            || self.external_roots.len() >= limits::EXPECTED_EXTERNAL_ROOT_LIMIT
        {
            return None;
        }

        let sequence = self.next_external_root_sequence?;
        self.next_external_root_sequence = sequence.checked_add(1);
        let lease = RootLeaseId {
            session: self.session,
            sequence,
        };
        self.external_roots.insert(
            lease,
            ExternalRoot {
                session: self.session,
                generation: self.generation,
                node,
            },
        );
        Some(lease)
    }

    pub(super) fn release_external_root(&mut self, lease: RootLeaseId) -> bool {
        if lease.session != self.session {
            return false;
        }
        self.external_roots.remove(&lease).is_some()
    }

    pub(super) fn collect(
        &mut self,
        owner_matches: bool,
        at_safe_point: bool,
        scan_storage_available: bool,
        fail_before_commit: bool,
    ) -> CollectionResult {
        if !owner_matches
            || !at_safe_point
            || !scan_storage_available
            || self.external_roots.len() > limits::EXPECTED_EXTERNAL_ROOT_LIMIT
        {
            return CollectionResult::Deferred;
        }

        let mut facade_ids = BTreeSet::new();
        let facade_mappings_are_valid = self.node_facade_ids.iter().all(|(node, facade_id)| {
            *facade_id > 0 && self.nodes.contains_key(node) && facade_ids.insert(*facade_id)
        }) && self.wrappers.iter().all(|(node, registration)| {
            self.node_facade_ids.get(node) == Some(&registration.facade_id)
        }) && match self.next_facade_id {
            Some(next) => self
                .node_facade_ids
                .values()
                .all(|facade_id| *facade_id < next),
            None => true,
        };
        let roots_are_valid = facade_mappings_are_valid
            && self
                .host_root_children
                .iter()
                .all(|node| self.nodes.contains_key(node))
            && self
                .wrappers
                .keys()
                .all(|node| self.nodes.contains_key(node))
            && self.external_roots.iter().all(|(lease, root)| {
                lease.session == self.session
                    && root.session == self.session
                    && root.generation == self.generation
                    && self.nodes.contains_key(&root.node)
            });
        if !roots_are_valid {
            return CollectionResult::Deferred;
        }

        let expired_wrappers = self
            .wrappers
            .iter()
            .filter_map(|(node, registration)| registration.weak_handle_empty.then_some(*node))
            .collect::<BTreeSet<_>>();
        let wrapper_roots = self
            .wrappers
            .iter()
            .filter_map(|(node, registration)| (!registration.weak_handle_empty).then_some(*node))
            .collect::<BTreeSet<_>>();
        let external_roots = self
            .external_roots
            .values()
            .map(|root| root.node)
            .collect::<BTreeSet<_>>();
        let Some(retained) = retained_components(
            &self.nodes,
            &self.children,
            &self.host_root_children,
            &wrapper_roots,
            &external_roots,
        ) else {
            return CollectionResult::Deferred;
        };
        let reclaimed = self
            .nodes
            .keys()
            .filter(|node| !retained.contains(node))
            .copied()
            .collect::<Vec<_>>();
        let reclaimed_string_units = reclaimed
            .iter()
            .filter_map(|node| self.nodes.get(node))
            .map(|node| node.string_units)
            .sum();

        if fail_before_commit {
            return CollectionResult::Deferred;
        }

        for node in &reclaimed {
            self.nodes.remove(node);
            self.node_facade_ids.remove(node);
            self.children.remove(node);
        }
        for children in self.children.values_mut() {
            children.retain(|child| !reclaimed.contains(child));
        }
        for node in expired_wrappers {
            self.wrappers.remove(&node);
        }

        CollectionResult::Completed {
            reclaimed_nodes: reclaimed.len(),
            reclaimed_string_units,
        }
    }
}

pub(super) fn retained_components(
    nodes: &BTreeMap<u64, fixture::Node>,
    children: &BTreeMap<u64, Vec<u64>>,
    host_root_children: &BTreeSet<u64>,
    wrapper_roots: &BTreeSet<u64>,
    external_roots: &BTreeSet<u64>,
) -> Option<BTreeSet<u64>> {
    for node in nodes.values() {
        if let Some(parent) = node.parent {
            if !nodes.contains_key(&parent) {
                return None;
            }
            if !children
                .get(&parent)
                .is_some_and(|siblings| siblings.contains(&node.id))
            {
                return None;
            }
        }
    }
    for (parent, siblings) in children {
        if !nodes.contains_key(parent) {
            return None;
        }
        let mut unique = BTreeSet::new();
        if siblings.iter().any(|child| {
            !unique.insert(*child)
                || nodes
                    .get(child)
                    .is_none_or(|node| node.parent != Some(*parent))
        }) {
            return None;
        }
    }
    if host_root_children
        .iter()
        .any(|root| nodes.get(root).is_none_or(|node| node.parent.is_some()))
    {
        return None;
    }

    for node in nodes.values() {
        let mut ancestors = BTreeSet::new();
        let mut current = Some(node.id);
        while let Some(id) = current {
            if !ancestors.insert(id) {
                return None;
            }
            current = nodes.get(&id)?.parent;
        }
    }

    let mut pending = host_root_children
        .iter()
        .chain(wrapper_roots)
        .chain(external_roots)
        .copied()
        .collect::<VecDeque<_>>();
    let mut retained = BTreeSet::new();
    while let Some(node) = pending.pop_front() {
        if !nodes.contains_key(&node) || !retained.insert(node) {
            continue;
        }
        if let Some(parent) = nodes.get(&node).and_then(|entry| entry.parent) {
            pending.push_back(parent);
        }
        if let Some(node_children) = children.get(&node) {
            pending.extend(node_children.iter().copied());
        }
    }
    Some(retained)
}
