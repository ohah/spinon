use std::sync::Arc;

use spinon_core::{EnvironmentRevision, HostDocument, NodeId, StyleRevision};

use super::{
    CssRect, CssSize, RuntimeConfirmedFrameError, RuntimeConfirmedFrameRegistry, RuntimePaint,
    RuntimeRenderBox, RuntimeRenderKey, RuntimeRenderSnapshot, RuntimeSurfaceGeneration,
};

fn generation(value: u64) -> RuntimeSurfaceGeneration {
    RuntimeSurfaceGeneration::new(value).expect("test generation is nonzero")
}

fn snapshot(node_id: u64, x: f32) -> Arc<RuntimeRenderSnapshot> {
    let document = HostDocument::new().expect("test document").snapshot();
    let key = RuntimeRenderKey::new(
        document.generation(),
        document.document_revision(),
        document.render_tree_revision(),
        StyleRevision::INITIAL,
        EnvironmentRevision::INITIAL,
    );
    let render_box = RuntimeRenderBox::new(
        NodeId::new(node_id).expect("test node ID is nonzero"),
        CssRect::new(x, 0.0, 10.0, 10.0).expect("test frame is valid"),
        RuntimePaint::None,
        0,
    );
    Arc::new(
        RuntimeRenderSnapshot::new(
            key,
            CssSize::new(100.0, 80.0).expect("test viewport is valid"),
            vec![render_box],
        )
        .expect("test snapshot is valid"),
    )
}

fn registry(generation_value: u64, max_pending: usize) -> RuntimeConfirmedFrameRegistry {
    RuntimeConfirmedFrameRegistry::new(generation(generation_value), max_pending)
        .expect("registry parameters are valid")
}

#[test]
fn submit_does_not_publish_unconfirmed_frame_or_hit_target() {
    let mut registry = registry(1, 2);
    let submitted = registry.submit(snapshot(11, 0.0)).expect("submit succeeds");

    assert_eq!(submitted.frame_id().get(), 1);
    assert_eq!(registry.pending_count(), 1);
    assert!(registry.current_confirmed_frame().is_none());
}

#[test]
fn confirmation_publishes_exact_snapshot_and_existing_hit_test_target() {
    let mut registry = registry(1, 2);
    let source = snapshot(12, 20.0);
    let submitted = registry
        .submit(Arc::clone(&source))
        .expect("submit succeeds");

    let confirmed = registry
        .confirm_presented(&submitted)
        .expect("confirmation succeeds");

    assert_eq!(confirmed.token(), &submitted);
    assert!(Arc::ptr_eq(&confirmed.snapshot_handle(), &source));
    assert_eq!(confirmed.hit_test_css_point(22.0, 2.0), NodeId::new(12));
    assert_eq!(confirmed.hit_test_css_point(2.0, 2.0), None);
}

#[test]
fn newer_pending_frame_keeps_previous_confirmation_until_it_is_confirmed() {
    let mut registry = registry(1, 2);
    let first = registry.submit(snapshot(13, 0.0)).expect("first submit");
    let first_confirmed = registry
        .confirm_presented(&first)
        .expect("first confirmation");
    let second = registry.submit(snapshot(14, 20.0)).expect("second submit");

    let current = registry
        .current_confirmed_frame()
        .expect("first confirmation remains current");
    assert_eq!(current.token(), &first);
    assert_eq!(current.hit_test_css_point(2.0, 2.0), NodeId::new(13));
    assert_eq!(registry.pending_count(), 1);

    let second_confirmed = registry
        .confirm_presented(&second)
        .expect("second confirmation");
    assert_eq!(
        second_confirmed.hit_test_css_point(22.0, 2.0),
        NodeId::new(14)
    );
    assert_eq!(
        first_confirmed.hit_test_css_point(2.0, 2.0),
        NodeId::new(13)
    );
}

#[test]
fn older_confirmation_after_newer_confirmation_cannot_regress_current_frame() {
    let mut registry = registry(1, 3);
    let first = registry.submit(snapshot(15, 0.0)).expect("first submit");
    let second = registry.submit(snapshot(16, 20.0)).expect("second submit");
    let latest = registry
        .confirm_presented(&second)
        .expect("newer confirmation");

    assert_eq!(registry.pending_count(), 0);
    assert!(matches!(
        registry.confirm_presented(&first),
        Err(RuntimeConfirmedFrameError::StaleOrDuplicateConfirmation)
    ));
    assert_eq!(
        registry.current_confirmed_frame().unwrap().token(),
        latest.token()
    );
}

#[test]
fn duplicate_confirmation_is_rejected_without_mutating_current_frame() {
    let mut registry = registry(1, 2);
    let submitted = registry.submit(snapshot(17, 0.0)).expect("submit");
    registry
        .confirm_presented(&submitted)
        .expect("first confirmation");
    let before = registry.current_confirmed_frame().expect("current frame");

    assert!(matches!(
        registry.confirm_presented(&submitted),
        Err(RuntimeConfirmedFrameError::StaleOrDuplicateConfirmation)
    ));
    assert_eq!(
        registry.current_confirmed_frame().unwrap().token(),
        before.token()
    );
}

#[test]
fn discarded_frame_confirmation_is_rejected_without_state_change() {
    let mut registry = registry(1, 2);
    let discarded = registry.submit(snapshot(39, 0.0)).expect("submit");
    registry
        .discard_unpresented(&discarded)
        .expect("discard pending frame");

    assert!(matches!(
        registry.confirm_presented(&discarded),
        Err(RuntimeConfirmedFrameError::UnknownOrDiscardedFrame)
    ));
    assert!(registry.current_confirmed_frame().is_none());
    assert_eq!(registry.pending_count(), 0);
}

#[test]
fn foreign_registry_token_is_rejected_even_when_numeric_ids_match() {
    let mut first_registry = registry(1, 1);
    let mut second_registry = registry(1, 1);
    let foreign = first_registry
        .submit(snapshot(18, 0.0))
        .expect("first registry submit");
    let local = second_registry
        .submit(snapshot(19, 20.0))
        .expect("second registry submit");
    assert_eq!(foreign.frame_id(), local.frame_id());

    assert!(matches!(
        second_registry.confirm_presented(&foreign),
        Err(RuntimeConfirmedFrameError::ForeignRegistryToken)
    ));
    assert_eq!(second_registry.pending_count(), 1);
    assert!(second_registry.current_confirmed_frame().is_none());
}

#[test]
fn stale_generation_token_cannot_restore_old_frame_after_surface_advance() {
    let mut registry = registry(1, 2);
    let old = registry.submit(snapshot(20, 0.0)).expect("old submit");
    registry.confirm_presented(&old).expect("old confirmation");
    registry
        .advance_surface_generation(generation(2))
        .expect("generation advance");
    let new = registry.submit(snapshot(21, 20.0)).expect("new submit");

    assert!(matches!(
        registry.confirm_presented(&old),
        Err(RuntimeConfirmedFrameError::StaleSurfaceGeneration)
    ));
    assert!(registry.current_confirmed_frame().is_none());
    assert_eq!(registry.pending_count(), 1);
    registry.confirm_presented(&new).expect("new confirmation");
    assert_eq!(registry.current_confirmed_frame().unwrap().token(), &new);
}

#[test]
fn surface_generation_must_increase_without_mutating_state_on_error() {
    assert!(RuntimeSurfaceGeneration::new(0).is_none());
    let mut registry = registry(2, 2);
    let pending = registry.submit(snapshot(22, 0.0)).expect("submit");

    assert_eq!(
        registry.advance_surface_generation(generation(2)),
        Err(RuntimeConfirmedFrameError::SurfaceGenerationNotIncreasing)
    );
    assert_eq!(
        registry.advance_surface_generation(generation(1)),
        Err(RuntimeConfirmedFrameError::SurfaceGenerationNotIncreasing)
    );
    assert_eq!(registry.surface_generation(), generation(2));
    assert_eq!(registry.pending_count(), 1);
    assert_eq!(pending.surface_generation(), generation(2));
}

#[test]
fn generation_advance_clears_current_and_pending_frames() {
    let mut registry = registry(1, 2);
    let current = registry.submit(snapshot(23, 0.0)).expect("current submit");
    let pending = registry.submit(snapshot(24, 20.0)).expect("pending submit");
    registry
        .confirm_presented(&current)
        .expect("current confirmation");

    registry
        .advance_surface_generation(generation(3))
        .expect("generation advance");

    assert!(registry.current_confirmed_frame().is_none());
    assert_eq!(registry.pending_count(), 0);
    assert_eq!(registry.surface_generation(), generation(3));
    assert!(matches!(
        registry.confirm_presented(&pending),
        Err(RuntimeConfirmedFrameError::StaleSurfaceGeneration)
    ));
}

#[test]
fn higher_confirmation_releases_older_pending_submissions() {
    let mut registry = registry(1, 3);
    let first = registry.submit(snapshot(25, 0.0)).expect("first submit");
    let second = registry.submit(snapshot(26, 20.0)).expect("second submit");
    let third = registry.submit(snapshot(27, 40.0)).expect("third submit");
    assert_eq!(registry.pending_count(), 3);

    registry
        .confirm_presented(&second)
        .expect("second confirmation");

    assert_eq!(registry.pending_count(), 1);
    assert!(matches!(
        registry.confirm_presented(&first),
        Err(RuntimeConfirmedFrameError::StaleOrDuplicateConfirmation)
    ));
    assert_eq!(
        registry.discard_unpresented(&first),
        Err(RuntimeConfirmedFrameError::FrameSuperseded)
    );
    assert_eq!(registry.pending_count(), 1);
    assert_eq!(registry.current_confirmed_frame().unwrap().token(), &second);
    assert_eq!(registry.confirm_presented(&third).unwrap().token(), &third);
}

#[test]
fn explicit_unpresented_discard_releases_slot_and_late_callback_fails() {
    let mut registry = registry(1, 1);
    let discarded = registry.submit(snapshot(28, 0.0)).expect("submit");

    registry
        .discard_unpresented(&discarded)
        .expect("known unpresented submission is discarded");
    assert_eq!(registry.pending_count(), 0);
    assert!(matches!(
        registry.confirm_presented(&discarded),
        Err(RuntimeConfirmedFrameError::UnknownOrDiscardedFrame)
    ));
    let next = registry
        .submit(snapshot(29, 20.0))
        .expect("slot is reusable");
    assert_eq!(next.frame_id().get(), discarded.frame_id().get() + 1);
}

#[test]
fn confirmed_frame_cannot_be_discarded() {
    let mut registry = registry(1, 1);
    let submitted = registry.submit(snapshot(30, 0.0)).expect("submit");
    registry
        .confirm_presented(&submitted)
        .expect("confirmation");

    assert_eq!(
        registry.discard_unpresented(&submitted),
        Err(RuntimeConfirmedFrameError::FrameAlreadyConfirmed)
    );
    assert_eq!(
        registry.current_confirmed_frame().unwrap().token(),
        &submitted
    );
}

#[test]
fn foreign_and_stale_tokens_cannot_be_discarded() {
    let mut first_registry = registry(1, 1);
    let mut second_registry = registry(1, 1);
    let foreign = first_registry.submit(snapshot(31, 0.0)).expect("submit");
    assert_eq!(
        second_registry.discard_unpresented(&foreign),
        Err(RuntimeConfirmedFrameError::ForeignRegistryToken)
    );

    first_registry
        .advance_surface_generation(generation(2))
        .expect("advance");
    assert_eq!(
        first_registry.discard_unpresented(&foreign),
        Err(RuntimeConfirmedFrameError::StaleSurfaceGeneration)
    );
}

#[test]
fn pending_capacity_rejects_without_overwriting_existing_tokens() {
    let mut registry = registry(1, 2);
    let first = registry.submit(snapshot(32, 0.0)).expect("first");
    let second = registry.submit(snapshot(33, 20.0)).expect("second");
    let before = registry.pending_count();

    assert_eq!(
        registry.submit(snapshot(34, 40.0)),
        Err(RuntimeConfirmedFrameError::PendingCapacityReached)
    );
    assert_eq!(registry.pending_count(), before);
    registry.confirm_presented(&first).expect("first confirm");
    registry.confirm_presented(&second).expect("second confirm");
}

#[test]
fn pending_capacity_constructor_enforces_hard_ceiling() {
    assert_eq!(
        RuntimeConfirmedFrameRegistry::new(generation(1), 0).err(),
        Some(RuntimeConfirmedFrameError::InvalidPendingCapacity)
    );
    assert_eq!(
        RuntimeConfirmedFrameRegistry::new(generation(1), 9).err(),
        Some(RuntimeConfirmedFrameError::InvalidPendingCapacity)
    );
    assert!(RuntimeConfirmedFrameRegistry::new(generation(1), 8).is_ok());
}

#[test]
fn input_handoff_clone_keeps_the_confirmed_snapshot_after_advance() {
    let mut registry = registry(1, 2);
    let first_source = snapshot(35, 0.0);
    let first_token = registry
        .submit(Arc::clone(&first_source))
        .expect("first submit");
    let queued_input_frame = registry
        .confirm_presented(&first_token)
        .expect("first confirmation");
    let retained_handle = queued_input_frame.snapshot_handle();
    let next = registry.submit(snapshot(36, 20.0)).expect("next submit");
    registry
        .confirm_presented(&next)
        .expect("next confirmation");
    registry
        .advance_surface_generation(generation(2))
        .expect("surface replaced");

    assert!(Arc::ptr_eq(&retained_handle, &first_source));
    assert_eq!(queued_input_frame.token(), &first_token);
    assert_eq!(
        queued_input_frame.hit_test_css_point(2.0, 2.0),
        NodeId::new(35)
    );
    assert!(registry.current_confirmed_frame().is_none());
}

#[test]
fn frame_ids_do_not_wrap_or_reuse_after_exhaustion() {
    let mut registry = registry(1, 2);
    registry.set_next_frame_id_for_test(u64::MAX);

    let last = registry
        .submit(snapshot(37, 0.0))
        .expect("maximum ID is issued once");
    assert_eq!(last.frame_id().get(), u64::MAX);
    assert_eq!(
        registry.submit(snapshot(38, 20.0)),
        Err(RuntimeConfirmedFrameError::FrameIdExhausted)
    );
    assert_eq!(registry.pending_count(), 1);
    registry
        .confirm_presented(&last)
        .expect("last frame confirms");
}

#[test]
fn frame_ids_remain_monotonic_across_surface_generations() {
    let mut registry = registry(1, 2);
    let first = registry.submit(snapshot(40, 0.0)).expect("first submit");
    registry
        .advance_surface_generation(generation(2))
        .expect("generation advance");
    let second = registry.submit(snapshot(41, 20.0)).expect("second submit");

    assert_eq!(first.frame_id().get(), 1);
    assert_eq!(second.frame_id().get(), 2);
    assert_eq!(second.surface_generation(), generation(2));
}
