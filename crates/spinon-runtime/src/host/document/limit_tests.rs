use super::{
    DocumentBatchOperation as Operation, HostDocumentBridge, SpinonDocumentReceipt, commit_callback,
};
use std::ffi::{CStr, c_char, c_void};

fn element(id: i32, name: &str) -> Operation {
    Operation::CreateElement {
        id,
        namespace: "http://www.w3.org/1999/xhtml".to_owned(),
        name: name.to_owned(),
    }
}

fn commit(bridge: &mut HostDocumentBridge, operations: Vec<Operation>) -> SpinonDocumentReceipt {
    bridge
        .commit(&operations)
        .expect("유효한 변경 묶음이어야 합니다")
}

#[test]
fn oversized_batches_fail_before_allocating_handles() {
    let mut bridge = HostDocumentBridge::new().unwrap();
    let operations = (1..=257).map(|id| element(id, "div")).collect::<Vec<_>>();
    let error = bridge.commit(&operations).unwrap_err();
    assert!(error.contains("최대 256개"));
    assert_eq!(bridge.snapshot().node_count(), 0);
    assert_eq!(bridge.document_revision(), 0);
}

#[test]
fn exact_operation_limit_is_accepted() {
    let mut bridge = HostDocumentBridge::new().unwrap();
    let operations = (1..=256).map(|id| element(id, "div")).collect::<Vec<_>>();

    let receipt = commit(&mut bridge, operations);

    assert_eq!(receipt.changed, 1);
    assert_eq!(receipt.node_count, 256);
    assert_eq!(bridge.snapshot().node_count(), 256);
    assert_eq!(bridge.document_revision(), 1);
}

#[test]
fn exact_name_limit_is_accepted() {
    let mut bridge = HostDocumentBridge::new().unwrap();
    commit(
        &mut bridge,
        vec![element(1, "div"), Operation::Append { parent: 0, node: 1 }],
    );
    let name = "a".repeat(1024);

    let receipt = commit(
        &mut bridge,
        vec![Operation::SetAttribute {
            node: 1,
            name,
            value: Vec::new(),
        }],
    );

    assert_eq!(receipt.document_revision, 2);
    assert_eq!(receipt.node_count, 1);
}

#[test]
fn exact_utf16_value_limit_is_accepted() {
    let mut bridge = HostDocumentBridge::new().unwrap();
    let receipt = commit(
        &mut bridge,
        vec![Operation::CreateText {
            id: 1,
            data: vec![b'x' as u16; 1_048_576],
        }],
    );

    assert_eq!(receipt.document_revision, 1);
    assert_eq!(receipt.node_count, 1);
    assert_eq!(bridge.snapshot().node_count(), 1);
}

#[test]
fn aggregate_utf16_limit_rejects_before_reserving_any_node() {
    let mut bridge = HostDocumentBridge::new().unwrap();
    let before = bridge.snapshot();
    let error = bridge
        .commit(&[
            Operation::CreateText {
                id: 1,
                data: vec![b'a' as u16; 600_000],
            },
            Operation::CreateText {
                id: 2,
                data: vec![b'b' as u16; 500_001],
            },
        ])
        .unwrap_err();

    assert!(error.contains("묶음 문자열"));
    assert_eq!(bridge.snapshot(), before);
    assert_eq!(bridge.handle(1), None);
    assert_eq!(bridge.handle(2), None);
}

#[test]
fn runtime_node_limit_counts_detached_nodes_and_rejects_atomically() {
    let mut bridge = HostDocumentBridge::new().unwrap();
    let mut next_id = 1_i32;
    while (next_id as usize) <= super::MAX_DOCUMENT_NODES {
        let end =
            (next_id as usize + super::MAX_BATCH_OPERATIONS - 1).min(super::MAX_DOCUMENT_NODES);
        let operations = (next_id..=end as i32)
            .map(|id| Operation::CreateText {
                id,
                data: Vec::new(),
            })
            .collect();
        commit(&mut bridge, operations);
        next_id = end as i32 + 1;
    }
    let revision = bridge.document_revision();
    let error = bridge
        .commit(&[Operation::CreateText {
            id: next_id,
            data: Vec::new(),
        }])
        .unwrap_err();

    assert!(error.contains("최대 16384개"));
    assert_eq!(bridge.node_count(), super::MAX_DOCUMENT_NODES as u64);
    assert_eq!(bridge.document_revision(), revision);
    assert_eq!(bridge.handle(next_id), None);

    assert_eq!(
        bridge.collect_unreachable(&[]).unwrap(),
        super::MAX_DOCUMENT_NODES
    );
    assert_eq!(bridge.node_count(), 0);
    commit(
        &mut bridge,
        vec![Operation::CreateText {
            id: next_id + 1,
            data: Vec::new(),
        }],
    );
    assert!(bridge.handle(next_id + 1).is_some());
}

#[test]
fn replacement_reclaims_old_text_usage_and_document_budget_rolls_back() {
    let mut bridge = HostDocumentBridge::new().unwrap();
    let megabyte = vec![b'a' as u16; 1_048_576];
    for id in 1..=16 {
        commit(
            &mut bridge,
            vec![Operation::CreateText {
                id,
                data: megabyte.clone(),
            }],
        );
    }
    let before = bridge.snapshot();
    let revision = bridge.document_revision();
    let error = bridge
        .commit(&[Operation::CreateText {
            id: 17,
            data: megabyte.clone(),
        }])
        .unwrap_err();
    assert!(error.contains("UTF-16 코드 단위 16777216개"));
    assert_eq!(bridge.snapshot(), before);
    assert_eq!(bridge.document_revision(), revision);
    assert_eq!(bridge.handle(17), None);

    commit(
        &mut bridge,
        vec![Operation::SetText {
            node: 1,
            data: vec![b'b' as u16; 1_048_576],
        }],
    );
    assert_eq!(bridge.node_count(), 16);
}

#[test]
fn ffi_quota_failure_keeps_the_dom_exception_status_and_document_unchanged() {
    let mut bridge = HostDocumentBridge::new().unwrap();
    let megabyte = vec![b'a' as u16; 1_048_576];
    for id in 1..=16 {
        commit(
            &mut bridge,
            vec![Operation::CreateText {
                id,
                data: megabyte.clone(),
            }],
        );
    }
    let revision = bridge.document_revision();
    let operation = super::SpinonDocumentOperation {
        kind: super::OP_CREATE_TEXT,
        node_id: 17,
        value: megabyte.as_ptr(),
        value_length: megabyte.len(),
        ..super::SpinonDocumentOperation::default()
    };
    let mut receipt = super::SpinonDocumentReceipt::default();
    let mut error = [0_i8; 256];

    let status = unsafe {
        commit_callback(
            std::ptr::from_mut(&mut bridge).cast::<c_void>(),
            &operation,
            1,
            &mut receipt,
            error.as_mut_ptr().cast::<c_char>(),
            error.len(),
        )
    };

    assert_eq!(
        status,
        -4,
        "{}",
        unsafe { CStr::from_ptr(error.as_ptr().cast::<c_char>()) }.to_string_lossy()
    );
    assert!(
        unsafe { CStr::from_ptr(error.as_ptr().cast::<c_char>()) }
            .to_string_lossy()
            .contains("QuotaExceededError:")
    );
    assert_eq!(bridge.document_revision(), revision);
    assert_eq!(bridge.node_count(), 16);
    assert_eq!(bridge.handle(17), None);
}
