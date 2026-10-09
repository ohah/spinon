use super::*;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};
use std::thread;
use std::time::{Duration, Instant};

const HTML: &str = "http://www.w3.org/1999/xhtml";

#[test]
fn elapsed_microseconds_rounds_sub_microsecond_durations_up() {
    assert_eq!(duration_microseconds(Duration::ZERO), 0);
    assert_eq!(duration_microseconds(Duration::from_nanos(1)), 1);
    assert_eq!(duration_microseconds(Duration::from_nanos(1_000)), 1);
    assert_eq!(duration_microseconds(Duration::from_nanos(1_001)), 2);
}

fn make_snapshot(
    tags: &[&str],
    top_text: Option<&str>,
    inline_style: Option<&str>,
) -> Arc<HostDocumentSnapshot> {
    let mut document = HostDocument::new().unwrap();
    let owner = OwnerId::new(1).unwrap();
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    for tag in tags {
        let node = document.reserve_node_handle().unwrap();
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: (*tag).to_owned(),
        });
        if let Some(style) = inline_style {
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "style").unwrap(),
                value: style.into(),
            });
        }
        batch.push(DocumentOperation::InsertBefore {
            parent: spinon_core::HostParent::Root,
            node,
            before: None,
        });
    }
    if let Some(text) = top_text {
        let node = document.reserve_node_handle().unwrap();
        batch.push(DocumentOperation::CreateText {
            node,
            data: text.into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: spinon_core::HostParent::Root,
            node,
            before: None,
        });
    }
    if !batch.operations().is_empty() {
        document.commit(batch).unwrap();
    }
    Arc::new(document.snapshot())
}

fn environment(handle: &RuntimeUaCascadeHandle, media: CssMediaEnvironment) -> EnvironmentRevision {
    handle.set_environment(390.0, 844.0, 3.0, media).unwrap()
}

fn wait_for_terminal_state(
    handle: &RuntimeUaCascadeHandle,
    expected: RuntimeUaCascadeState,
) -> RuntimeUaCascadeSnapshot {
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        let snapshot = handle.snapshot();
        if snapshot.state == expected {
            return snapshot;
        }
        assert!(
            Instant::now() < deadline,
            "cascade 상태 대기 시간 초과: {snapshot:?}"
        );
        thread::sleep(Duration::from_millis(1));
    }
}

#[test]
fn no_environment_means_no_implicit_desktop_calculation() {
    let coordinator = RuntimeUaCascadeCoordinator::new().unwrap();
    let handle = coordinator.handle();
    handle
        .register_document_snapshot(make_snapshot(&[], None, None))
        .unwrap();

    let before = handle.snapshot();
    assert_eq!(before.state, RuntimeUaCascadeState::NotConfigured);
    assert_eq!(
        handle.layout_snapshot().state,
        RuntimeLayoutState::NotConfigured
    );
    assert!(before.requested.is_none());
    assert!(before.completed.is_none());

    assert_eq!(environment(&handle, CssMediaEnvironment::MOBILE).get(), 0);
    let after = wait_for_terminal_state(&handle, RuntimeUaCascadeState::Empty);
    assert_eq!(after.requested.unwrap().environment_revision, 0);
    assert_eq!(after.completed.as_ref().unwrap().roots.len(), 0);
    assert_eq!(handle.layout_snapshot().state, RuntimeLayoutState::Empty);
    coordinator.shutdown().unwrap();
}

#[test]
fn identical_environment_keeps_revision_and_does_not_clear_completed_result() {
    let coordinator = RuntimeUaCascadeCoordinator::new().unwrap();
    let handle = coordinator.handle();
    handle
        .register_document_snapshot(make_snapshot(&[], None, None))
        .unwrap();
    assert_eq!(environment(&handle, CssMediaEnvironment::DESKTOP).get(), 0);
    let completed = wait_for_terminal_state(&handle, RuntimeUaCascadeState::Empty);
    let completed_key = completed.completed.unwrap().key;

    assert_eq!(environment(&handle, CssMediaEnvironment::DESKTOP).get(), 0);
    let unchanged = handle.snapshot();
    assert_eq!(unchanged.state, RuntimeUaCascadeState::Empty);
    assert_eq!(unchanged.completed.unwrap().key, completed_key);
    coordinator.shutdown().unwrap();
}

#[test]
fn invalid_environment_preserves_the_last_valid_revision_and_result() {
    let coordinator = RuntimeUaCascadeCoordinator::new().unwrap();
    let handle = coordinator.handle();
    handle
        .register_document_snapshot(make_snapshot(&[], None, None))
        .unwrap();
    assert_eq!(environment(&handle, CssMediaEnvironment::MOBILE).get(), 0);
    let before = wait_for_terminal_state(&handle, RuntimeUaCascadeState::Empty);

    assert_eq!(
        handle.set_environment(f32::NAN, 844.0, 3.0, CssMediaEnvironment::MOBILE,),
        Err(RuntimeUaCascadeError::InvalidEnvironment)
    );
    assert_eq!(environment(&handle, CssMediaEnvironment::MOBILE).get(), 0);
    assert_eq!(
        handle.snapshot().completed.unwrap().key,
        before.completed.unwrap().key
    );
    coordinator.shutdown().unwrap();
}

#[test]
fn all_host_root_elements_are_computed_as_independent_roots() {
    let snapshot = make_snapshot(&["div", "span"], None, None);
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport::C04_FIXTURE,

        previous_snapshot: None,
        force_full: true,
        invalidation: None,
        previous_styles: None,
    };
    let calculation = compute_request(&request).unwrap();
    assert_eq!(calculation.roots.len(), 2);
    assert_eq!(calculation.roots[0].styles.elements.len(), 1);
    assert_eq!(calculation.roots[1].styles.elements.len(), 1);
    assert_eq!(
        calculation.roots[0].styles.elements[0].properties["display"],
        "block"
    );
    assert_eq!(
        calculation.roots[1].styles.elements[0].properties["display"],
        "inline"
    );
    assert_eq!(calculation.layout.unwrap_err().code, "multiple_host_roots");
}

#[test]
fn inline_style_attribute_overrides_ua_and_preserves_parser_diagnostics() {
    let styled = make_snapshot(&["div"], None, Some("display: inline; color: nope;"));
    let request = WorkRequest {
        key: key_for(&styled, EnvironmentRevision::INITIAL),
        snapshot: styled,
        viewport: CssViewport::C04_FIXTURE,

        previous_snapshot: None,
        force_full: true,
        invalidation: None,
        previous_styles: None,
    };
    let calculation = compute_request(&request).unwrap();
    assert_eq!(
        calculation.roots[0].styles.elements[0].properties["display"],
        "inline"
    );
    assert!(!calculation.roots[0].styles.diagnostics.is_empty());
    assert_eq!(
        calculation.layout.unwrap_err().code,
        "unsupported_computed_value"
    );
}

#[test]
fn worker_failure_is_terminal_for_environment_updates_but_not_session_state() {
    let coordinator = RuntimeUaCascadeCoordinator::new().unwrap();
    let handle = coordinator.handle();
    handle
        .register_document_snapshot(make_snapshot(&[], None, None))
        .unwrap();
    environment(&handle, CssMediaEnvironment::MOBILE);
    fail_worker(&handle.shared, "테스트 worker panic");

    let failed = handle.snapshot();
    assert_eq!(failed.state, RuntimeUaCascadeState::Failed);
    assert_eq!(failed.error.as_deref(), Some("테스트 worker panic"));
    assert_eq!(
        environment_result(&handle),
        Err(RuntimeUaCascadeError::WorkerUnavailable)
    );
    coordinator.shutdown().unwrap();
}

#[test]
fn worker_panic_is_caught_and_transitions_the_latest_request_to_failed() {
    let coordinator = RuntimeUaCascadeCoordinator::new_with_computer(Arc::new(|_| {
        panic!("의도적 CSS worker panic 검증")
    }))
    .unwrap();
    let handle = coordinator.handle();
    handle
        .register_document_snapshot(make_snapshot(&["div"], None, None))
        .unwrap();
    environment(&handle, CssMediaEnvironment::MOBILE);

    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        let snapshot = handle.snapshot();
        if snapshot.state == RuntimeUaCascadeState::Failed {
            assert!(snapshot.error.unwrap().contains("panic"));
            break;
        }
        assert!(
            Instant::now() < deadline,
            "worker panic 뒤 failed 상태가 되지 않았습니다"
        );
        thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(
        environment_result(&handle),
        Err(RuntimeUaCascadeError::WorkerUnavailable)
    );
    coordinator.shutdown().unwrap();
}

fn environment_result(
    handle: &RuntimeUaCascadeHandle,
) -> Result<EnvironmentRevision, RuntimeUaCascadeError> {
    handle.set_environment(400.0, 800.0, 2.0, CssMediaEnvironment::MOBILE)
}

#[path = "runtime_layout_tests.rs"]
mod runtime_layout_tests;

#[path = "custom_properties_tests.rs"]
mod custom_properties_tests;
