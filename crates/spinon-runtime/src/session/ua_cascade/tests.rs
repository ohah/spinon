use super::*;
use spinon_core::{AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, OwnerId};
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
    assert!(before.requested.is_none());
    assert!(before.completed.is_none());

    assert_eq!(environment(&handle, CssMediaEnvironment::MOBILE).get(), 0);
    let after = wait_for_terminal_state(&handle, RuntimeUaCascadeState::Empty);
    assert_eq!(after.requested.unwrap().environment_revision, 0);
    assert_eq!(after.completed.as_ref().unwrap().roots.len(), 0);
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
    };
    let roots = compute_request(&request).unwrap();
    assert_eq!(roots.len(), 2);
    assert_eq!(roots[0].styles.elements.len(), 1);
    assert_eq!(roots[1].styles.elements.len(), 1);
    assert_eq!(roots[0].styles.elements[0].properties["display"], "block");
    assert_eq!(roots[1].styles.elements[0].properties["display"], "inline");
}

#[test]
fn inline_style_attribute_overrides_ua_and_preserves_parser_diagnostics() {
    let styled = make_snapshot(&["div"], None, Some("display: inline; color: nope;"));
    let request = WorkRequest {
        key: key_for(&styled, EnvironmentRevision::INITIAL),
        snapshot: styled,
        viewport: CssViewport::C04_FIXTURE,
    };
    let roots = compute_request(&request).unwrap();
    assert_eq!(roots[0].styles.elements[0].properties["display"], "inline");
    assert!(!roots[0].styles.diagnostics.is_empty());
}

#[test]
fn direct_text_root_fails_the_whole_request_instead_of_returning_partial_roots() {
    let snapshot = make_snapshot(&["div"], Some("not an element"), None);
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport::C04_FIXTURE,
    };
    let error = compute_request(&request).unwrap_err();
    assert!(error.contains("직속 텍스트"));
}

#[test]
fn stale_completion_cannot_replace_a_newer_requested_key() {
    let shared = Shared {
        state: Mutex::new(State::new()),
        wake: Condvar::new(),
    };
    let snapshot = make_snapshot(&["div"], None, None);
    let mut state = lock(&shared.state);
    state.document = Some(Arc::clone(&snapshot));
    state.environment = Some(RuntimeCssEnvironment {
        width_css_px: 390.0,
        height_css_px: 844.0,
        device_scale_factor: 3.0,
        media_environment: CssMediaEnvironment::MOBILE,
        revision: EnvironmentRevision::INITIAL,
    });
    request_latest(&shared, &mut state);
    let stale_key = state.requested.unwrap();
    state.environment = Some(RuntimeCssEnvironment {
        revision: EnvironmentRevision::INITIAL.checked_next().unwrap(),
        ..state.environment.unwrap()
    });
    request_latest(&shared, &mut state);
    let current_key = state.requested.unwrap();
    drop(state);

    publish_result(&shared, stale_key, Ok(Vec::new()), 1);
    let state = lock(&shared.state);
    assert_ne!(current_key, stale_key);
    assert_eq!(state.requested, Some(current_key));
    assert_eq!(state.status, RuntimeUaCascadeState::Pending);
    assert!(state.completed.is_none());
    assert_eq!(state.pending.as_ref().unwrap().key, current_key);
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
