#[path = "../../../tests/fixtures/dom/s03/node-lifecycle-v1.rs"]
mod fixture;
#[path = "../../../tests/fixtures/dom/s03/node-lifecycle-limits-v1.rs"]
mod limits;

#[path = "s03_node_lifecycle_contract/lifecycle_model.rs"]
mod lifecycle_model;
use lifecycle_model::{
    CollectionResult, ExternalRoot, IdAllocator, Model, Registration, RootLeaseId,
    retained_components,
};
use std::collections::BTreeSet;

#[test]
fn tree_fixture_expectations_match_the_contract_oracle() {
    for case in fixture::TREE_CASES {
        let model = Model::from_case(case, 1);
        let retained = retained_components(
            &model.nodes,
            &model.children,
            &model.host_root_children,
            &case.wrapper_roots.iter().copied().collect(),
            &case.external_roots.iter().copied().collect(),
        )
        .expect("유효한 tree fixture여야 합니다");
        assert_eq!(
            retained,
            case.expected_retained.iter().copied().collect(),
            "{}",
            case.name
        );
    }
}

#[test]
fn detached_nodes_and_strings_remain_resident_until_successful_scan_and_sweep() {
    let case = &fixture::TREE_CASES[2];
    let mut model = Model::from_case(case, 7);
    let wrapper = model.lookup_or_create_wrapper(13);
    model.v8_automatically_resets_weak_handle(13);
    let before_nodes = model.nodes.len();
    let before_strings = model
        .nodes
        .values()
        .map(|node| node.string_units)
        .sum::<u64>();

    assert_eq!(before_nodes, 5);
    assert_eq!(before_strings, 15);
    assert_eq!(
        model.collect(true, true, true, true),
        CollectionResult::Deferred
    );
    assert_eq!(model.nodes.len(), before_nodes);
    assert_eq!(model.wrappers[&13].wrapper_identity, wrapper);
    assert_eq!(
        model
            .nodes
            .values()
            .map(|node| node.string_units)
            .sum::<u64>(),
        before_strings
    );
    assert_eq!(
        model.collect(true, true, true, false),
        CollectionResult::Completed {
            reclaimed_nodes: 5,
            reclaimed_string_units: 15,
        }
    );
    assert!(model.nodes.is_empty());
    assert!(model.wrappers.is_empty());
}

#[test]
fn live_weak_wrapper_keeps_its_canonical_identity_and_component() {
    let case = &fixture::TREE_CASES[2];
    let mut model = Model::from_case(case, 3);
    let first = model.lookup_or_create_wrapper(13);
    let second = model.lookup_or_create_wrapper(13);

    assert_eq!(first, second);
    assert_eq!(
        model.collect(true, true, true, false),
        CollectionResult::Completed {
            reclaimed_nodes: 1,
            reclaimed_string_units: 5,
        }
    );
    assert_eq!(model.document_revision, 0);
    assert_eq!(model.render_tree_revision, 0);
    assert_eq!(
        model.nodes.keys().copied().collect::<BTreeSet<_>>(),
        [10, 11, 12, 13].into()
    );
    assert_eq!(model.wrappers[&13].wrapper_identity, first);
}

#[test]
fn reset_handle_can_be_replaced_before_the_next_safe_point() {
    let case = &fixture::TREE_CASES[2];
    let mut model = Model::from_case(case, 5);
    let old_identity = model.lookup_or_create_wrapper(13);
    let facade_id = model.wrappers[&13].facade_id;
    model.v8_automatically_resets_weak_handle(13);
    let new_identity = model.lookup_or_create_wrapper(13);

    assert_ne!(old_identity, new_identity);
    assert_eq!(model.wrappers.len(), 1);
    assert_eq!(model.wrappers[&13].facade_id, facade_id);
    assert_eq!(model.node_facade_ids[&13], facade_id);
    assert_eq!(
        model.collect(true, true, true, false),
        CollectionResult::Completed {
            reclaimed_nodes: 1,
            reclaimed_string_units: 5,
        }
    );
    assert_eq!(model.wrappers[&13].wrapper_identity, new_identity);
    assert!(model.nodes.contains_key(&13));
    assert!(!model.nodes.contains_key(&20));
}

#[test]
fn external_root_keeps_component_after_wrapper_reset_until_lease_release() {
    let case = &fixture::TREE_CASES[2];
    let mut model = Model::from_case(case, 9);
    let old_identity = model.lookup_or_create_wrapper(13);
    let facade_id = model.wrappers[&13].facade_id;
    model.v8_automatically_resets_weak_handle(13);
    let lease = model.register_external_root(13).unwrap();

    assert_eq!(
        model.collect(true, true, true, false),
        CollectionResult::Completed {
            reclaimed_nodes: 1,
            reclaimed_string_units: 5,
        }
    );
    assert_eq!(
        model.nodes.keys().copied().collect::<BTreeSet<_>>(),
        [10, 11, 12, 13].into()
    );
    assert!(model.wrappers.is_empty());
    assert_eq!(model.node_facade_ids[&13], facade_id);
    let new_identity = model.lookup_or_create_wrapper(13);
    assert_ne!(old_identity, new_identity);
    assert_eq!(model.wrappers[&13].facade_id, facade_id);
    model.v8_automatically_resets_weak_handle(13);
    let before_foreign_release = model.clone();
    assert!(!model.release_external_root(RootLeaseId {
        session: model.session + 1,
        sequence: lease.sequence,
    }));
    assert_eq!(model, before_foreign_release);
    assert!(model.release_external_root(lease));
    let before_stale_release = model.clone();
    assert!(!model.release_external_root(lease));
    assert_eq!(model, before_stale_release);
    assert_eq!(
        model.collect(true, true, true, false),
        CollectionResult::Completed {
            reclaimed_nodes: 4,
            reclaimed_string_units: 10,
        }
    );
    assert!(model.nodes.is_empty());
    assert!(model.node_facade_ids.is_empty());
    assert_eq!(model.next_facade_id, Some(2));
    let replacement = fixture::Node {
        id: 30,
        parent: None,
        string_units: 0,
    };
    assert!(model.create_node_and_wrapper(replacement, 2, true, true));
    assert_eq!(model.node_facade_ids[&30], 2);
}

#[test]
fn owner_mismatch_non_safe_point_and_scan_failure_preserve_state() {
    let case = &fixture::TREE_CASES[2];
    let mut model = Model::from_case(case, 4);
    model.lookup_or_create_wrapper(13);
    model.v8_automatically_resets_weak_handle(13);
    let before = model.clone();

    assert_eq!(
        model.collect(false, true, true, false),
        CollectionResult::Deferred
    );
    assert_eq!(model, before);
    assert_eq!(
        model.collect(true, false, true, false),
        CollectionResult::Deferred
    );
    assert_eq!(model, before);
    assert_eq!(
        model.collect(true, true, false, false),
        CollectionResult::Deferred
    );
    assert_eq!(model, before);
}

#[test]
fn invalid_external_root_defers_the_entire_collection() {
    let case = &fixture::TREE_CASES[2];
    for (lease_session, root_session, generation, node) in [
        (10, 10, 1, 13),
        (10, 11, 1, 13),
        (11, 11, 2, 13),
        (11, 11, 1, 99),
    ] {
        let mut model = Model::from_case(case, 11);
        let lease = RootLeaseId {
            session: lease_session,
            sequence: 1,
        };
        model.external_roots.insert(
            lease,
            ExternalRoot {
                session: root_session,
                generation,
                node,
            },
        );
        assert_collection_defers_unchanged(model);
    }
}

#[test]
fn malformed_parent_graphs_and_registry_entries_defer_without_mutation() {
    let detached = &fixture::TREE_CASES[1];

    let mut cycle = Model::from_case(detached, 21);
    cycle.nodes.get_mut(&10).unwrap().parent = Some(13);
    cycle.children.entry(13).or_default().push(10);
    assert_collection_defers_unchanged(cycle);

    let mut dangling_parent = Model::from_case(detached, 22);
    dangling_parent.nodes.get_mut(&13).unwrap().parent = Some(99);
    assert_collection_defers_unchanged(dangling_parent);

    let mut inconsistent_host_root = Model::from_case(&fixture::TREE_CASES[0], 23);
    inconsistent_host_root.host_root_children.insert(2);
    assert_collection_defers_unchanged(inconsistent_host_root);

    let mut inconsistent_child_list = Model::from_case(detached, 29);
    inconsistent_child_list
        .children
        .entry(20)
        .or_default()
        .push(13);
    assert_collection_defers_unchanged(inconsistent_child_list);

    let mut stale_empty_wrapper = Model::from_case(detached, 24);
    stale_empty_wrapper.wrappers.insert(
        99,
        Registration {
            wrapper_identity: 1,
            facade_id: 99,
            weak_handle_empty: true,
        },
    );
    assert_collection_defers_unchanged(stale_empty_wrapper);

    let mut inconsistent_wrapper_id = Model::from_case(&fixture::TREE_CASES[2], 30);
    inconsistent_wrapper_id.lookup_or_create_wrapper(13);
    inconsistent_wrapper_id
        .wrappers
        .get_mut(&13)
        .unwrap()
        .facade_id = 99;
    assert_collection_defers_unchanged(inconsistent_wrapper_id);

    let mut duplicate_facade_id = Model::from_case(&fixture::TREE_CASES[2], 31);
    duplicate_facade_id.lookup_or_create_wrapper(13);
    duplicate_facade_id.node_facade_ids.insert(12, 1);
    assert_collection_defers_unchanged(duplicate_facade_id);
}

#[test]
fn malformed_registry_sizes_defer_before_marking() {
    let case = &fixture::TREE_CASES[1];

    let mut too_many_nodes = Model::from_case(case, 25);
    for id in 100..=(limits::EXPECTED_NODE_QUOTA as u64 + 100) {
        too_many_nodes.nodes.insert(
            id,
            fixture::Node {
                id,
                parent: None,
                string_units: 0,
            },
        );
    }
    assert_collection_defers_unchanged(too_many_nodes);

    let mut too_many_wrappers = Model::from_case(case, 26);
    for id in 100..=(limits::EXPECTED_NODE_QUOTA as u64 + 100) {
        too_many_wrappers.wrappers.insert(
            id,
            Registration {
                wrapper_identity: id,
                facade_id: id as i32,
                weak_handle_empty: true,
            },
        );
    }
    assert_collection_defers_unchanged(too_many_wrappers);

    let mut too_many_external_roots = Model::from_case(case, 27);
    for sequence in 1..=(limits::EXPECTED_EXTERNAL_ROOT_LIMIT as u64 + 1) {
        let lease = RootLeaseId {
            session: 27,
            sequence,
        };
        too_many_external_roots.external_roots.insert(
            lease,
            ExternalRoot {
                session: 27,
                generation: 1,
                node: 10,
            },
        );
    }
    assert_collection_defers_unchanged(too_many_external_roots);
}

#[test]
fn facade_ids_are_monotonic_even_when_a_reserved_creation_fails() {
    let mut allocator = IdAllocator { next: Some(1) };
    assert_eq!(allocator.reserve(), Some(1));
    let failed_creation_id = allocator.reserve().unwrap();
    assert_eq!(failed_creation_id, 2);
    assert_eq!(allocator.reserve(), Some(3));

    let mut exhausted = IdAllocator {
        next: Some(i32::MAX),
    };
    assert_eq!(exhausted.reserve(), Some(i32::MAX));
    assert_eq!(exhausted.reserve(), None);
}

#[test]
fn failed_node_or_wrapper_preparation_publishes_no_partial_create() {
    let case = &fixture::TREE_CASES[2];
    let mut model = Model::from_case(case, 12);
    let before = model.clone();
    let candidate = fixture::Node {
        id: 30,
        parent: None,
        string_units: 2,
    };
    let mut ids = IdAllocator { next: Some(1) };

    let first_id = ids.reserve().unwrap();
    assert_eq!(first_id, 1);
    assert!(!model.create_node_and_wrapper(candidate, first_id, false, true));
    assert_eq!(model, before);
    let second_id = ids.reserve().unwrap();
    assert_eq!(second_id, 2);
    assert!(!model.create_node_and_wrapper(candidate, second_id, true, false));
    assert_eq!(model, before);
    let third_id = ids.reserve().unwrap();
    assert_eq!(third_id, 3);

    assert!(model.create_node_and_wrapper(candidate, third_id, true, true));
    assert!(model.nodes.contains_key(&30));
    assert!(!model.wrappers[&30].weak_handle_empty);
    assert_eq!(model.document_revision, 1);
    assert_eq!(model.render_tree_revision, 0);
    assert_eq!(model.node_facade_ids[&30], third_id);
    assert_eq!(model.wrappers[&30].facade_id, third_id);
}

#[test]
fn failed_sweep_preserves_nodes_weak_registry_roots_and_accounting() {
    let case = &fixture::TREE_CASES[2];
    let mut model = Model::from_case(case, 6);
    model.lookup_or_create_wrapper(13);
    model.v8_automatically_resets_weak_handle(13);
    model.external_roots.insert(
        RootLeaseId {
            session: 6,
            sequence: 1,
        },
        ExternalRoot {
            session: 6,
            generation: 1,
            node: 20,
        },
    );
    let before = model.clone();

    assert_eq!(
        model.collect(true, true, true, true),
        CollectionResult::Deferred
    );
    assert_eq!(model, before);
}

#[test]
fn s032_resident_quotas_match_the_contract_fixture() {
    assert_eq!(limits::EXPECTED_NODE_QUOTA, 16_384);
    assert_eq!(limits::EXPECTED_EXTERNAL_ROOT_LIMIT, 16_384);
    assert_eq!(limits::EXPECTED_STRING_QUOTA_UTF16, 16_777_216);
}

#[test]
fn external_root_capacity_applies_backpressure_without_reusing_tokens() {
    let case = &fixture::TREE_CASES[2];
    let mut model = Model::from_case(case, 13);
    let mut last_lease = None;

    for _ in 0..limits::EXPECTED_EXTERNAL_ROOT_LIMIT {
        last_lease = model.register_external_root(13);
        assert!(last_lease.is_some());
    }
    assert_eq!(
        model.external_roots.len(),
        limits::EXPECTED_EXTERNAL_ROOT_LIMIT
    );
    assert_eq!(model.register_external_root(13), None);
    assert!(model.release_external_root(last_lease.unwrap()));

    let replacement = model.register_external_root(13).unwrap();
    assert_eq!(
        replacement.sequence as usize,
        limits::EXPECTED_EXTERNAL_ROOT_LIMIT + 1
    );
}

#[test]
fn external_root_token_exhaustion_does_not_wrap_or_reuse_a_token() {
    let case = &fixture::TREE_CASES[1];
    let mut model = Model::from_case(case, 28);
    model.next_external_root_sequence = Some(u64::MAX);

    let last_lease = model.register_external_root(10).unwrap();
    assert_eq!(last_lease.sequence, u64::MAX);
    assert_eq!(model.next_external_root_sequence, None);
    let before = model.clone();
    assert_eq!(model.register_external_root(10), None);
    assert_eq!(model, before);
    assert!(model.release_external_root(last_lease));
    assert_eq!(model.register_external_root(10), None);
}

#[test]
fn exhausted_facade_id_space_stays_closed_after_its_last_node_is_swept() {
    let case = &fixture::TREE_CASES[0];
    let mut model = Model::from_case(case, 32);
    let last_node = fixture::Node {
        id: 30,
        parent: None,
        string_units: 0,
    };
    assert!(model.create_node_and_wrapper(last_node, i32::MAX, true, true));
    model.v8_automatically_resets_weak_handle(30);

    assert_eq!(
        model.collect(true, true, true, false),
        CollectionResult::Completed {
            reclaimed_nodes: 1,
            reclaimed_string_units: 0,
        }
    );
    assert!(!model.node_facade_ids.contains_key(&30));
    assert_eq!(model.next_facade_id, None);

    let before = model.clone();
    assert!(!model.create_node_and_wrapper(
        fixture::Node {
            id: 31,
            parent: None,
            string_units: 0,
        },
        1,
        true,
        true,
    ));
    assert_eq!(model, before);
}

fn assert_collection_defers_unchanged(mut model: Model) {
    let before = model.clone();
    assert_eq!(
        model.collect(true, true, true, false),
        CollectionResult::Deferred
    );
    assert_eq!(model, before);
}
