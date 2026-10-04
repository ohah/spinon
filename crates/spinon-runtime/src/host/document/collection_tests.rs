use super::{DocumentBatchOperation as Operation, HostDocumentBridge, MAX_PENDING_EXTERNAL_IDS};
use spinon_core::HostNodeHandle;

#[path = "../../../../../tests/fixtures/dom/s03/node-lifecycle-v1.rs"]
mod fixture;

const HTML: &str = "http://www.w3.org/1999/xhtml";

fn fixture_operations(case: &fixture::TreeCase) -> Vec<Operation> {
    let mut operations = Vec::new();
    for node in case.nodes {
        let id = i32::try_from(node.id).unwrap();
        let units = usize::try_from(node.string_units).unwrap();
        if case.nodes.iter().any(|child| child.parent == Some(node.id)) {
            operations.push(Operation::CreateElement {
                id,
                namespace: String::new(),
                name: "x".repeat(units),
            });
        } else {
            operations.push(Operation::CreateText {
                id,
                data: vec![b'x' as u16; units],
            });
        }
    }
    for node in case.nodes {
        if let Some(parent) = node.parent {
            operations.push(Operation::Append {
                parent: i32::try_from(parent).unwrap(),
                node: i32::try_from(node.id).unwrap(),
            });
        } else if case.host_root_children.contains(&node.id) {
            operations.push(Operation::Append {
                parent: 0,
                node: i32::try_from(node.id).unwrap(),
            });
        }
    }
    operations
}

fn root_handles(bridge: &HostDocumentBridge, case: &fixture::TreeCase) -> Vec<HostNodeHandle> {
    case.wrapper_roots
        .iter()
        .chain(case.external_roots)
        .map(|id| bridge.handle(i32::try_from(*id).unwrap()).unwrap())
        .collect()
}

#[test]
fn product_collector_matches_fixed_lifecycle_fixture_and_frees_string_accounting() {
    for case in fixture::TREE_CASES {
        let mut bridge = HostDocumentBridge::new().unwrap();
        bridge.commit(&fixture_operations(case)).unwrap();
        let roots = root_handles(&bridge, case);
        let before_document_revision = bridge.document_revision();
        let before_render_revision = bridge.render_tree_revision();
        let expected_retained = case.expected_retained;
        let expected_reclaimed = case.nodes.len() - expected_retained.len();
        let expected_string_units = case
            .nodes
            .iter()
            .filter(|node| expected_retained.contains(&node.id))
            .map(|node| usize::try_from(node.string_units).unwrap())
            .sum::<usize>();

        let reclaimed = bridge.collect_unreachable(&roots).unwrap();

        assert_eq!(reclaimed, expected_reclaimed, "{}", case.name);
        assert_eq!(
            bridge.node_count(),
            expected_retained.len() as u64,
            "{}",
            case.name
        );
        assert_eq!(
            bridge.string_units(),
            expected_string_units,
            "{}",
            case.name
        );
        assert_eq!(bridge.document_revision(), before_document_revision);
        assert_eq!(bridge.render_tree_revision(), before_render_revision);
        for node in case.nodes {
            let id = i32::try_from(node.id).unwrap();
            assert_eq!(
                bridge.handle(id).is_some(),
                expected_retained.contains(&node.id),
                "{} node {}",
                case.name,
                node.id
            );
        }
    }
}

#[test]
fn collection_rejects_bad_roots_and_broken_bridge_accounting_before_mutation() {
    let mut bridge = HostDocumentBridge::new().unwrap();
    bridge
        .commit(&[
            Operation::CreateText {
                id: 1,
                data: vec![b'a' as u16; 4],
            },
            Operation::CreateText {
                id: 2,
                data: vec![b'b' as u16; 7],
            },
        ])
        .unwrap();
    let before = bridge.snapshot();
    let before_units = bridge.string_units();
    let mut other = HostDocumentBridge::new().unwrap();
    other
        .commit(&[Operation::CreateText {
            id: 1,
            data: Vec::new(),
        }])
        .unwrap();
    let foreign = other.handle(1).unwrap();
    let own = bridge.handle(1).unwrap();

    assert!(bridge.collect_unreachable(&[foreign]).is_err());
    assert!(bridge.collect_unreachable(&[own, own]).is_err());
    assert_eq!(bridge.snapshot(), before);
    assert_eq!(bridge.node_count(), 2);
    assert_eq!(bridge.string_units(), before_units);

    bridge.string_units += 1;
    let corrupted_total = bridge.string_units();
    assert!(bridge.collect_unreachable(&[]).is_err());
    assert_eq!(bridge.snapshot(), before);
    assert_eq!(bridge.node_count(), 2);
    assert_eq!(bridge.string_units(), corrupted_total);
}

#[test]
fn collection_rejects_broken_reverse_indexes_before_removing_core_nodes() {
    let mut bridge = HostDocumentBridge::new().unwrap();
    bridge
        .commit(&[
            Operation::CreateText {
                id: 1,
                data: vec![b'a' as u16; 3],
            },
            Operation::CreateText {
                id: 2,
                data: vec![b'b' as u16; 5],
            },
        ])
        .unwrap();
    let before = bridge.snapshot();
    let before_units = bridge.string_units();
    let stale_index = bridge.handle(2).unwrap();
    bridge.external_ids.remove(&stale_index);

    assert!(bridge.collect_unreachable(&[]).is_err());
    assert_eq!(bridge.snapshot(), before);
    assert_eq!(bridge.node_count(), 2);
    assert_eq!(bridge.string_units(), before_units);
}

#[test]
fn stale_handle_after_collection_fails_closed_and_facade_ids_keep_advancing() {
    let mut bridge = HostDocumentBridge::new().unwrap();
    bridge
        .commit(&[
            Operation::CreateElement {
                id: 1,
                namespace: HTML.to_owned(),
                name: "section".to_owned(),
            },
            Operation::SetAttribute {
                node: 1,
                name: "id".to_owned(),
                value: "stale".encode_utf16().collect(),
            },
        ])
        .unwrap();
    let stale = bridge.handle(1).unwrap();
    assert_eq!(
        bridge.string_units(),
        HTML.encode_utf16().count() + "section".len() + 2 + 5
    );

    assert_eq!(bridge.collect_unreachable(&[]).unwrap(), 1);
    assert_eq!(bridge.string_units(), 0);
    assert_eq!(bridge.handle(1), None);
    assert!(bridge.collect_unreachable(&[stale]).is_err());

    let next_id = bridge.next_external_id().unwrap();
    assert_eq!(next_id, 2);
    bridge
        .commit(&[Operation::CreateText {
            id: next_id,
            data: Vec::new(),
        }])
        .unwrap();
    let new_handle = bridge.handle(next_id).unwrap();
    assert_ne!(new_handle, stale);
    assert!(
        bridge
            .commit(&[Operation::CreateText {
                id: 1,
                data: Vec::new(),
            }])
            .unwrap_err()
            .contains("재사용")
    );
    assert_eq!(bridge.node_count(), 1);
}

#[test]
fn failed_creation_burns_reserved_id_and_pending_reservation_is_bounded() {
    let mut bridge = HostDocumentBridge::new().unwrap();
    let failed_id = bridge.next_external_id().unwrap();
    assert_eq!(failed_id, 1);
    assert!(
        bridge
            .commit(&[
                Operation::CreateText {
                    id: failed_id,
                    data: Vec::new(),
                },
                Operation::SetAttribute {
                    node: 99,
                    name: "id".to_owned(),
                    value: Vec::new(),
                },
            ])
            .is_err()
    );
    assert_eq!(bridge.next_external_id().unwrap(), 2);
    assert!(
        bridge
            .commit(&[Operation::CreateText {
                id: failed_id,
                data: Vec::new(),
            }])
            .unwrap_err()
            .contains("재사용")
    );

    for _ in 1..MAX_PENDING_EXTERNAL_IDS {
        bridge.next_external_id().unwrap();
    }
    assert!(bridge.next_external_id().unwrap_err().contains("대기 중"));
    bridge
        .commit(&[Operation::CreateText {
            id: 2,
            data: Vec::new(),
        }])
        .unwrap();
    assert_eq!(bridge.next_external_id().unwrap(), 258);
}

#[test]
fn batch_cannot_issue_a_lower_facade_id_after_a_higher_one() {
    let mut bridge = HostDocumentBridge::new().unwrap();
    let error = bridge
        .commit(&[
            Operation::CreateText {
                id: 10,
                data: Vec::new(),
            },
            Operation::CreateText {
                id: 2,
                data: Vec::new(),
            },
        ])
        .unwrap_err();

    assert!(error.contains("재사용"));
    assert_eq!(bridge.node_count(), 0);
    assert_eq!(bridge.next_external_id().unwrap(), 11);
    assert!(
        bridge
            .commit(&[Operation::CreateText {
                id: 10,
                data: Vec::new(),
            }])
            .unwrap_err()
            .contains("재사용")
    );
}

#[test]
fn reclaimed_node_and_string_quota_can_be_used_again() {
    let mut bridge = HostDocumentBridge::new().unwrap();
    let megabyte = vec![b'z' as u16; 1_048_576];
    for id in 1..=16 {
        bridge
            .commit(&[Operation::CreateText {
                id,
                data: megabyte.clone(),
            }])
            .unwrap();
    }
    assert_eq!(bridge.string_units(), 16_777_216);
    assert_eq!(bridge.collect_unreachable(&[]).unwrap(), 16);
    assert_eq!(bridge.node_count(), 0);
    assert_eq!(bridge.string_units(), 0);
    bridge
        .commit(&[Operation::CreateText {
            id: 17,
            data: megabyte,
        }])
        .unwrap();
    assert_eq!(bridge.node_count(), 1);
    assert_eq!(bridge.string_units(), 1_048_576);
}

#[test]
fn exhausted_facade_id_space_stays_closed_after_collection() {
    let mut bridge = HostDocumentBridge::new().unwrap();
    bridge
        .commit(&[Operation::CreateText {
            id: i32::MAX,
            data: Vec::new(),
        }])
        .unwrap();
    assert!(bridge.next_external_id().unwrap_err().contains("공간"));

    assert_eq!(bridge.collect_unreachable(&[]).unwrap(), 1);
    assert_eq!(bridge.node_count(), 0);
    assert!(bridge.next_external_id().unwrap_err().contains("공간"));
    assert!(
        bridge
            .commit(&[Operation::CreateText {
                id: i32::MAX,
                data: Vec::new(),
            }])
            .unwrap_err()
            .contains("재사용")
    );
}
