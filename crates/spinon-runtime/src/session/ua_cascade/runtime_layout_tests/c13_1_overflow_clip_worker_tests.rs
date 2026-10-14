use super::*;
use std::sync::atomic::{AtomicBool, Ordering};

fn make_overflow_clip_recovery_snapshot() -> Arc<HostDocumentSnapshot> {
    let styles = [
        "display:block;position:relative;width:200px;height:160px;background-color:#111111",
        "position:relative;left:20px;top:10px;width:80px;height:60px;padding:8px;border:4px solid;overflow:clip;overflow-clip-margin:padding-box 8px;background-color:#333333",
        "position:absolute;left:50px;top:40px;width:60px;height:40px;background-color:#ff0000",
    ];
    let parents = [None, Some(0), Some(1)];
    let mut document = spinon_core::HostDocument::new().unwrap();
    let handles = styles
        .iter()
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let mut batch = spinon_core::DocumentChangeBatch::new(
        spinon_core::OwnerId::new(8132).unwrap(),
        document.document_revision(),
    );
    for (index, (handle, style)) in handles.iter().zip(styles).enumerate() {
        batch.push(spinon_core::DocumentOperation::CreateElement {
            node: *handle,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(spinon_core::DocumentOperation::SetAttribute {
            node: *handle,
            name: spinon_core::AttributeName::new(None, "style").unwrap(),
            value: style.to_owned().into(),
        });
        batch.push(spinon_core::DocumentOperation::InsertBefore {
            parent: parents[index]
                .map(|parent| HostParent::Node(handles[parent]))
                .unwrap_or(HostParent::Root),
            node: *handle,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    Arc::new(document.snapshot())
}

fn fail_overflow_clip_ancestry_reservation(
    document: &spinon_core::HostDocumentSnapshot,
    root: HostNodeHandle,
    output: &spinon_style_to_layout::StyleLayoutOutput,
    current: spinon_style_to_render::CurrentLayoutInputs,
) -> Result<spinon_render::RuntimeRenderSnapshot, spinon_style_to_render::StyleRenderError> {
    spinon_style_to_render::build_runtime_render_snapshot_with_overflow_clip_ancestry_allocation_failure(
        document,
        root,
        output,
        current,
    )
}

#[test]
fn clip_ancestry_allocation_failure_does_not_kill_runtime_worker() {
    let snapshot = make_overflow_clip_recovery_snapshot();
    let fail_next_request = Arc::new(AtomicBool::new(false));
    let fail_next_request_for_worker = Arc::clone(&fail_next_request);
    let compute = Arc::new(move |request: &WorkRequest| {
        if fail_next_request_for_worker.swap(false, Ordering::SeqCst) {
            let mut injected_builder = fail_overflow_clip_ancestry_reservation;
            calculation::compute_request_with_profile_and_render_builder(
                request,
                calculation::RuntimeCalculationProfile::BlockFlexOverflowClip,
                &mut injected_builder,
            )
        } else {
            compute_request_for_block_flex_overflow_clip(request)
        }
    });
    let coordinator = RuntimeUaCascadeCoordinator::new_with_computer(compute).unwrap();
    let worker_id = lock(&coordinator.worker)
        .as_ref()
        .expect("worker handle")
        .thread()
        .id();
    let handle = coordinator.handle();
    handle
        .register_document_snapshot(Arc::clone(&snapshot))
        .unwrap();
    let initial_revision = handle
        .set_environment(390.0, 844.0, 3.0, CssMediaEnvironment::MOBILE)
        .unwrap();
    assert_eq!(initial_revision.get(), 0);
    let baseline_layout = wait_for_layout_state(&handle, RuntimeLayoutState::Ready);
    let baseline_key = baseline_layout.completed.as_ref().unwrap().key;

    fail_next_request.store(true, Ordering::SeqCst);
    let failed_revision = handle
        .set_environment(391.0, 844.0, 3.0, CssMediaEnvironment::MOBILE)
        .unwrap();
    assert_eq!(failed_revision.get(), 1);

    let failed_cascade = wait_for_terminal_state(&handle, RuntimeUaCascadeState::Ready);
    let failed_layout = wait_for_layout_state(&handle, RuntimeLayoutState::Failed);
    assert_eq!(failed_cascade.state, RuntimeUaCascadeState::Ready);
    assert!(failed_cascade.completed.is_some());
    assert_eq!(failed_layout.state, RuntimeLayoutState::Failed);
    assert!(
        failed_layout.completed.is_none(),
        "부분 runtime snapshot을 게시하면 안 됩니다"
    );
    assert_ne!(failed_layout.requested, Some(baseline_key));
    assert_eq!(
        failed_layout.error,
        Some(layout_failure(
            "render_allocation_failed",
            None,
            Some("overflow clip ancestry".to_owned()),
        ))
    );
    assert!(lock(&handle.shared.state).worker_available);

    let recovered_revision = handle
        .set_environment(392.0, 844.0, 3.0, CssMediaEnvironment::MOBILE)
        .expect("레이아웃 실패 뒤에도 worker가 다음 요청을 받아야 합니다");
    assert_eq!(recovered_revision.get(), 2);
    let recovered_layout = wait_for_layout_state(&handle, RuntimeLayoutState::Ready);
    let actual = recovered_layout
        .completed
        .unwrap()
        .render_snapshot
        .clone()
        .expect("복구된 요청은 완전한 render snapshot을 게시해야 합니다");

    let expected_request = WorkRequest {
        key: key_for(&snapshot, recovered_revision),
        snapshot: Arc::clone(&snapshot),
        viewport: CssViewport {
            width_css_px: 392.0,
            height_css_px: 844.0,
            device_scale_factor: 3.0,
            environment_revision: recovered_revision,
            media_environment: CssMediaEnvironment::MOBILE,
        },
        previous_snapshot: None,
        force_full: true,
        invalidation: None,
        previous_styles: None,
    };
    let expected = compute_request_for_block_flex_overflow_clip(&expected_request)
        .unwrap()
        .layout
        .unwrap()
        .render_snapshot
        .unwrap();
    assert_eq!(
        actual, expected,
        "복구 snapshot은 fault-free 직접 계산과 같아야 합니다"
    );
    assert_eq!(actual.boxes().len(), 3);
    assert_eq!(
        lock(&coordinator.worker)
            .as_ref()
            .expect("same worker handle")
            .thread()
            .id(),
        worker_id,
        "재시도는 같은 runtime worker에서 처리되어야 합니다"
    );
    coordinator.shutdown().unwrap();
}
