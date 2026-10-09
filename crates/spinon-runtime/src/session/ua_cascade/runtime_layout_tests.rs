use super::*;

fn make_layout_snapshot(
    root_style: &str,
    child_styles_in_dom_order: &[&str],
    reverse_child_ids: bool,
    text_child: Option<&str>,
) -> (
    Arc<HostDocumentSnapshot>,
    HostNodeHandle,
    Vec<HostNodeHandle>,
) {
    let mut document = HostDocument::new().unwrap();
    let owner = OwnerId::new(2).unwrap();
    let root = document.reserve_node_handle().unwrap();
    let mut children = child_styles_in_dom_order
        .iter()
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    if reverse_child_ids {
        children.reverse();
    }
    let text = text_child.map(|_| document.reserve_node_handle().unwrap());
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    batch.push(DocumentOperation::CreateElement {
        node: root,
        namespace: HTML.to_owned(),
        local_name: "div".to_owned(),
    });
    batch.push(DocumentOperation::SetAttribute {
        node: root,
        name: AttributeName::new(None, "style").unwrap(),
        value: root_style.into(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Root,
        node: root,
        before: None,
    });
    for (handle, style) in children.iter().zip(child_styles_in_dom_order) {
        batch.push(DocumentOperation::CreateElement {
            node: *handle,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: *handle,
            name: AttributeName::new(None, "style").unwrap(),
            value: (*style).into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: *handle,
            before: None,
        });
    }
    if let (Some(text), Some(text_value)) = (text, text_child) {
        batch.push(DocumentOperation::CreateText {
            node: text,
            data: text_value.into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: text,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    (Arc::new(document.snapshot()), root, children)
}

fn wait_for_layout_state(
    handle: &RuntimeUaCascadeHandle,
    expected: RuntimeLayoutState,
) -> RuntimeLayoutSnapshot {
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        let snapshot = handle.layout_snapshot();
        if snapshot.state == expected {
            return snapshot;
        }
        assert!(
            Instant::now() < deadline,
            "layout 상태 대기 시간 초과: {snapshot:?}"
        );
        thread::sleep(Duration::from_millis(1));
    }
}

#[test]
fn runtime_layout_frames_follow_dom_order_not_node_id_order() {
    let (snapshot, root, children) = make_layout_snapshot(
        "display:flex;width:300px;height:140px;flex-direction:row",
        &[
            "display:block;width:20px;height:10px",
            "display:block;width:30px;height:10px",
        ],
        true,
        None,
    );
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport::C04_FIXTURE,
    };
    let calculation = compute_request(&request).unwrap();
    let layout = calculation.layout.unwrap();
    let frame_ids = layout
        .frames
        .iter()
        .map(|frame| frame.node_id)
        .collect::<Vec<_>>();
    assert_eq!(
        frame_ids,
        [
            root.id().get(),
            children[0].id().get(),
            children[1].id().get()
        ]
    );
}

#[test]
fn unsupported_layout_style_does_not_fail_the_ua_cascade_snapshot() {
    let (snapshot, _, _) = make_layout_snapshot(
        "display:block;width:300px;height:140px;color:red",
        &[],
        false,
        None,
    );
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot: Arc::clone(&snapshot),
        viewport: CssViewport::C04_FIXTURE,
    };
    let calculation = compute_request(&request).unwrap();
    assert_eq!(calculation.roots.len(), 1);
    assert_eq!(
        calculation.roots[0].styles.elements[0].properties["display"],
        "block"
    );
    assert_eq!(
        calculation.layout.unwrap_err(),
        layout_failure(
            "unsupported_inline_property",
            Some(1),
            Some("color".to_owned())
        )
    );

    let coordinator = RuntimeUaCascadeCoordinator::new().unwrap();
    let handle = coordinator.handle();
    handle.register_document_snapshot(snapshot).unwrap();
    environment(&handle, CssMediaEnvironment::DESKTOP);
    let cascade = wait_for_terminal_state(&handle, RuntimeUaCascadeState::Ready);
    let layout = wait_for_layout_state(&handle, RuntimeLayoutState::Failed);
    assert_eq!(cascade.state, RuntimeUaCascadeState::Ready);
    assert_eq!(
        layout.error.as_ref().unwrap().code,
        "unsupported_inline_property"
    );
    assert_eq!(
        layout.error.as_ref().unwrap().property.as_deref(),
        Some("color")
    );
    coordinator.shutdown().unwrap();
}

#[test]
fn recoverable_inline_parse_diagnostics_do_not_discard_layout() {
    let (snapshot, _, _) = make_layout_snapshot(
        "display:block;width:300px;height:140px;color:nope",
        &[],
        false,
        None,
    );
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport::C04_FIXTURE,
    };
    let calculation = compute_request(&request).unwrap();
    assert!(!calculation.roots[0].styles.diagnostics.is_empty());
    assert!(calculation.layout.is_ok());
    assert!(!calculation.layout_diagnostics.is_empty());
}

#[test]
fn text_inside_element_fails_layout_without_discarding_computed_style() {
    let (snapshot, _, _) = make_layout_snapshot(
        "display:block;width:300px;height:140px",
        &[],
        false,
        Some("text"),
    );
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport::C04_FIXTURE,
    };
    let calculation = compute_request(&request).unwrap();
    assert_eq!(calculation.roots.len(), 1);
    assert_eq!(
        calculation.layout.unwrap_err().code,
        "unsupported_text_node"
    );
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

    publish_result(
        &shared,
        stale_key,
        Ok(RuntimeCalculation {
            roots: Vec::new(),
            cascade_duration_us: 1,
            layout: Ok(RuntimeLayoutCompleted {
                key: stale_key,
                frames: Vec::new(),
                projection_duration_us: 0,
            }),
            layout_diagnostics: Vec::new(),
        }),
    );
    let state = lock(&shared.state);
    assert_ne!(current_key, stale_key);
    assert_eq!(state.requested, Some(current_key));
    assert_eq!(state.status, RuntimeUaCascadeState::Pending);
    assert_eq!(state.layout_status, RuntimeLayoutState::Pending);
    assert!(state.completed.is_none());
    assert!(state.layout_completed.is_none());
    assert_eq!(state.pending.as_ref().unwrap().key, current_key);
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
