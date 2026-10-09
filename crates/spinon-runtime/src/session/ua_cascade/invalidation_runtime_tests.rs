use super::calculation::compute_request_for_registered_properties_gpu;
use super::environment::RuntimeCssEnvironment;
use super::invalidation::{RuntimeStyleInvalidation, classify_style_invalidation};
use super::invalidation_test_support::TestTree;
use super::{RuntimeUaCascadeCoordinator, RuntimeUaCascadeState, Shared, State, WorkRequest};
use super::{key_for, lock, request_latest};
use spinon_style::{CssMediaEnvironment, CssViewport};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[test]
fn runtime_calculation_reuses_only_the_changed_inline_style_subtree() {
    let mut tree = TestTree::new();
    let viewport = CssViewport {
        width_css_px: 320.0,
        height_css_px: 180.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    };
    let previous_snapshot = Arc::new(tree.snapshot());
    let initial_request = WorkRequest {
        key: key_for(&previous_snapshot, viewport.environment_revision),
        snapshot: Arc::clone(&previous_snapshot),
        viewport,
        previous_snapshot: None,
        force_full: true,
        invalidation: Some(RuntimeStyleInvalidation::Full),
        previous_styles: None,
    };
    let initial = compute_request_for_registered_properties_gpu(&initial_request).unwrap();
    assert_eq!(initial.cascade_recomputed_style_elements, 5);
    assert_eq!(initial.cascade_reused_style_elements, 0);

    tree.set_attribute(tree.left, "style", "--size:46px");
    let current_snapshot = Arc::new(tree.snapshot());
    let invalidation = classify_style_invalidation(Some(&previous_snapshot), &current_snapshot);
    assert_eq!(invalidation.dirty_roots(), &[tree.left]);
    let request = WorkRequest {
        key: key_for(&current_snapshot, viewport.environment_revision),
        snapshot: current_snapshot,
        viewport,
        previous_snapshot: None,
        force_full: false,
        invalidation: Some(invalidation),
        previous_styles: Some(Arc::clone(&initial.roots)),
    };
    let partial = compute_request_for_registered_properties_gpu(&request).unwrap();

    assert_eq!(partial.roots[0].styles.elements.len(), 5);
    assert_eq!(partial.cascade_recomputed_style_elements, 2);
    assert_eq!(partial.cascade_reused_style_elements, 3);
    assert_eq!(partial.cascade_context_style_elements, 1);
    let left_child = partial.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.node_id == tree.left_child.id())
        .unwrap();
    let right_child = partial.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.node_id == tree.right_child.id())
        .unwrap();
    assert_eq!(left_child.properties["width"], "46px");
    assert_eq!(right_child.properties["width"], "41px");
}

#[test]
fn removing_inline_style_reuses_only_the_changed_subtree_and_matches_full_cascade() {
    let mut tree = TestTree::new();
    let viewport = CssViewport {
        width_css_px: 320.0,
        height_css_px: 180.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    };
    let previous_snapshot = Arc::new(tree.snapshot());
    let initial_request = WorkRequest {
        key: key_for(&previous_snapshot, viewport.environment_revision),
        snapshot: Arc::clone(&previous_snapshot),
        viewport,
        previous_snapshot: None,
        force_full: true,
        invalidation: Some(RuntimeStyleInvalidation::Full),
        previous_styles: None,
    };
    let initial = compute_request_for_registered_properties_gpu(&initial_request).unwrap();

    tree.remove_attribute(tree.left, "style");
    let current_snapshot = Arc::new(tree.snapshot());
    let invalidation = classify_style_invalidation(Some(&previous_snapshot), &current_snapshot);
    assert_eq!(invalidation.dirty_roots(), &[tree.left]);
    let partial_request = WorkRequest {
        key: key_for(&current_snapshot, viewport.environment_revision),
        snapshot: Arc::clone(&current_snapshot),
        viewport,
        previous_snapshot: Some(Arc::clone(&previous_snapshot)),
        force_full: false,
        invalidation: Some(invalidation),
        previous_styles: Some(Arc::clone(&initial.roots)),
    };
    let partial = compute_request_for_registered_properties_gpu(&partial_request).unwrap();
    let full_request = WorkRequest {
        key: partial_request.key,
        snapshot: current_snapshot,
        viewport,
        previous_snapshot: None,
        force_full: true,
        invalidation: Some(RuntimeStyleInvalidation::Full),
        previous_styles: None,
    };
    let full = compute_request_for_registered_properties_gpu(&full_request).unwrap();

    assert_eq!(
        partial.roots[0].styles.elements,
        full.roots[0].styles.elements
    );
    assert_eq!(
        partial.roots[0].styles.diagnostics,
        full.roots[0].styles.diagnostics
    );
    assert_eq!(
        partial.layout.as_ref().unwrap().frames,
        full.layout.as_ref().unwrap().frames
    );
    assert_eq!(partial.cascade_recomputed_style_elements, 2);
    assert_eq!(partial.cascade_reused_style_elements, 3);
    assert_eq!(partial.cascade_context_style_elements, 1);
}

#[test]
fn connected_author_stylesheet_forces_full_cascade_after_inline_style_change() {
    let mut tree = TestTree::new();
    tree.add_author_stylesheet("div { height: 10px; }");
    let viewport = CssViewport {
        width_css_px: 320.0,
        height_css_px: 180.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    };
    let previous_snapshot = Arc::new(tree.snapshot());
    let initial_request = WorkRequest {
        key: key_for(&previous_snapshot, viewport.environment_revision),
        snapshot: Arc::clone(&previous_snapshot),
        viewport,
        previous_snapshot: None,
        force_full: true,
        invalidation: Some(RuntimeStyleInvalidation::Full),
        previous_styles: None,
    };
    let initial = compute_request_for_registered_properties_gpu(&initial_request).unwrap();

    tree.set_attribute(tree.left, "style", "--size:46px");
    let current_snapshot = Arc::new(tree.snapshot());
    let invalidation = classify_style_invalidation(Some(&previous_snapshot), &current_snapshot);
    assert_eq!(invalidation.dirty_roots(), &[tree.left]);
    let request = WorkRequest {
        key: key_for(&current_snapshot, viewport.environment_revision),
        snapshot: Arc::clone(&current_snapshot),
        viewport,
        previous_snapshot: Some(previous_snapshot),
        force_full: false,
        invalidation: Some(invalidation),
        previous_styles: Some(Arc::clone(&initial.roots)),
    };
    let calculation = compute_request_for_registered_properties_gpu(&request).unwrap();

    assert_eq!(calculation.cascade_recomputed_style_elements, 6);
    assert_eq!(calculation.cascade_reused_style_elements, 0);
    assert_eq!(calculation.cascade_context_style_elements, 0);
}

#[test]
fn worker_publishes_subtree_metrics_only_for_latest_style_revision() {
    let coordinator = RuntimeUaCascadeCoordinator::new_runtime_gpu_registered_properties().unwrap();
    let handle = coordinator.handle();
    let mut tree = TestTree::new();
    let environment = handle
        .set_environment(320.0, 180.0, 1.0, CssMediaEnvironment::MOBILE)
        .unwrap();
    let previous = Arc::new(tree.snapshot());
    handle
        .register_document_snapshot(Arc::clone(&previous))
        .unwrap();
    let initial = wait_for_revision(&handle, previous.document_revision().get());
    assert_eq!(initial.cascade_recomputed_style_elements, 5);
    assert_eq!(initial.cascade_reused_style_elements, 0);

    tree.set_attribute(tree.left, "style", "--size:46px");
    let current = Arc::new(tree.snapshot());
    handle
        .register_document_snapshot(Arc::clone(&current))
        .unwrap();
    let partial = wait_for_revision(&handle, current.document_revision().get());

    assert_eq!(partial.cascade_recomputed_style_elements, 2);
    assert_eq!(partial.cascade_reused_style_elements, 3);
    assert_eq!(partial.cascade_context_style_elements, 1);
    assert_eq!(partial.key.generation, current.generation().get());
    assert_eq!(
        partial.key.document_revision,
        current.document_revision().get()
    );
    assert_eq!(partial.key.environment_revision, environment.get());
    assert_eq!(
        partial.roots[0]
            .styles
            .elements
            .iter()
            .find(|style| style.node_id == tree.left_child.id())
            .unwrap()
            .properties["width"],
        "46px"
    );
    assert_eq!(
        partial.roots[0]
            .styles
            .elements
            .iter()
            .find(|style| style.node_id == tree.right_child.id())
            .unwrap()
            .properties["width"],
        "41px"
    );
    coordinator.shutdown().unwrap();
}

#[test]
fn pending_requests_keep_the_earliest_contiguous_baseline_and_reject_gaps() {
    let mut tree = TestTree::new();
    let baseline = Arc::new(tree.snapshot());
    tree.set_attribute(tree.left, "style", "--size:46px");
    let middle = Arc::new(tree.snapshot());
    let shared = Shared {
        state: Mutex::new(State::new()),
        wake: Condvar::new(),
    };
    {
        let mut state = lock(&shared.state);
        state.document = Some(Arc::clone(&middle));
        state.environment = Some(RuntimeCssEnvironment {
            width_css_px: 320.0,
            height_css_px: 180.0,
            device_scale_factor: 1.0,
            media_environment: CssMediaEnvironment::MOBILE,
            revision: Default::default(),
        });
        request_latest(&shared, &mut state, Some(Arc::clone(&baseline)), false);
    }

    tree.set_attribute(tree.right, "style", "--size:51px");
    let newest = Arc::new(tree.snapshot());
    {
        let mut state = lock(&shared.state);
        state.document = Some(Arc::clone(&newest));
        request_latest(&shared, &mut state, Some(Arc::clone(&middle)), false);
        let pending = state.pending.as_ref().unwrap();
        assert_eq!(
            pending
                .previous_snapshot
                .as_ref()
                .unwrap()
                .document_revision(),
            baseline.document_revision()
        );
        assert_eq!(
            pending.snapshot.document_revision(),
            newest.document_revision()
        );
        assert!(!pending.force_full);
    }

    tree.set_attribute(tree.left, "style", "--size:49px");
    let skipped = Arc::new(tree.snapshot());
    tree.set_attribute(tree.right, "style", "--size:55px");
    let after_gap = Arc::new(tree.snapshot());
    {
        let mut state = lock(&shared.state);
        state.document = Some(Arc::clone(&after_gap));
        request_latest(&shared, &mut state, Some(Arc::clone(&skipped)), false);
        let pending = state.pending.as_ref().unwrap();
        assert!(pending.force_full);
        assert!(pending.previous_snapshot.is_none());
    }
}

fn wait_for_revision(
    handle: &super::RuntimeUaCascadeHandle,
    document_revision: u64,
) -> Arc<super::RuntimeUaCascadeCompleted> {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let snapshot = handle.snapshot();
        if let Some(ref completed) = snapshot.completed
            && completed.key.document_revision == document_revision
        {
            assert_eq!(snapshot.state, RuntimeUaCascadeState::Ready);
            return Arc::clone(completed);
        }
        assert!(
            Instant::now() < deadline,
            "cascade 결과 대기 시간 초과: {snapshot:?}"
        );
        thread::sleep(Duration::from_millis(1));
    }
}
