use super::{PresentationScenes, scene_matches_layout_key};
use spinon_core::{EnvironmentRevision, HostDocument, StyleRevision};
use spinon_render::{CssSize, RuntimeRenderKey, RuntimeRenderSnapshot};
use spinon_runtime::RuntimeUaCascadeKey;
use std::sync::{Arc, Barrier};
use std::thread;

fn empty_scene() -> RuntimeRenderSnapshot {
    let document = HostDocument::new().unwrap();
    let snapshot = document.snapshot();
    RuntimeRenderSnapshot::new(
        RuntimeRenderKey::new(
            snapshot.generation(),
            snapshot.document_revision(),
            snapshot.render_tree_revision(),
            StyleRevision::INITIAL,
            EnvironmentRevision::INITIAL,
        ),
        CssSize::new(301.0, 100.0).unwrap(),
        Vec::new(),
    )
    .unwrap()
}

#[test]
fn render_scene_must_match_the_complete_runtime_key() {
    let document = HostDocument::new().unwrap();
    let snapshot = document.snapshot();
    let scene = RuntimeRenderSnapshot::new(
        RuntimeRenderKey::new(
            snapshot.generation(),
            snapshot.document_revision(),
            snapshot.render_tree_revision(),
            StyleRevision::INITIAL,
            EnvironmentRevision::INITIAL,
        ),
        CssSize::new(301.0, 100.0).unwrap(),
        Vec::new(),
    )
    .unwrap();
    let key = RuntimeUaCascadeKey {
        generation: snapshot.generation().get(),
        document_revision: snapshot.document_revision().get(),
        render_tree_revision: snapshot.render_tree_revision().get(),
        style_revision: StyleRevision::INITIAL.get(),
        environment_revision: EnvironmentRevision::INITIAL.get(),
    };
    assert!(scene_matches_layout_key(&scene, key));
    assert!(!scene_matches_layout_key(
        &scene,
        RuntimeUaCascadeKey {
            environment_revision: 1,
            ..key
        }
    ));
}

#[test]
fn new_presentation_sequence_hides_the_old_scene_and_accepts_only_current_publish() {
    let presentation = PresentationScenes::new();
    let first = presentation.invalidate().unwrap();
    assert!(presentation.publish(first, empty_scene()));

    let (draw_sequence, current) = presentation.lock_current();
    assert_eq!(draw_sequence, first);
    assert!(current.is_some());
    drop(current);

    let second = presentation.invalidate().unwrap();
    assert!(!presentation.publish(first, empty_scene()));
    let (draw_sequence, current) = presentation.lock_current();
    assert_eq!(draw_sequence, second);
    assert!(current.is_none());
    drop(current);

    assert!(presentation.publish(second, empty_scene()));
    assert_eq!(presentation.sequence(), second);
}

#[test]
fn invalidation_winning_a_publish_race_rejects_the_stale_scene() {
    let presentation = Arc::new(PresentationScenes::new());
    let stale_sequence = presentation.invalidate().unwrap();
    let barrier = Arc::new(Barrier::new(2));
    let publisher = {
        let presentation = Arc::clone(&presentation);
        let barrier = Arc::clone(&barrier);
        thread::spawn(move || {
            barrier.wait();
            presentation.publish(stale_sequence, empty_scene())
        })
    };

    presentation.invalidate().unwrap();
    barrier.wait();
    assert!(!publisher.join().unwrap());
    let (_, current) = presentation.lock_current();
    assert!(current.is_none());
}

#[cfg(target_os = "macos")]
#[test]
fn runtime_scene_readback_matches_the_fixed_chromium_color_samples() {
    use spinon_core::{HostDocument, NodeId, StyleRevision};
    use spinon_render::{
        CssRect, CssSize, OpaqueCssSrgb, RuntimePaint, RuntimeRenderBox, RuntimeRenderKey,
        RuntimeRenderSnapshot,
    };
    let document = HostDocument::new().unwrap();
    let document_snapshot = document.snapshot();
    let scene = RuntimeRenderSnapshot::new(
        RuntimeRenderKey::new(
            document_snapshot.generation(),
            document_snapshot.document_revision(),
            document_snapshot.render_tree_revision(),
            StyleRevision::INITIAL,
            spinon_core::EnvironmentRevision::INITIAL,
        ),
        CssSize::new(301.0, 100.0).unwrap(),
        vec![
            RuntimeRenderBox::new(
                NodeId::new(1).unwrap(),
                CssRect::new(0.0, 0.0, 301.0, 100.0).unwrap(),
                RuntimePaint::Opaque(OpaqueCssSrgb::new(18, 52, 86)),
                0,
            ),
            RuntimeRenderBox::new(
                NodeId::new(2).unwrap(),
                CssRect::new(0.0, 0.0, 51.0, 31.0).unwrap(),
                RuntimePaint::Opaque(OpaqueCssSrgb::new(51, 102, 255)),
                1,
            ),
        ],
    )
    .unwrap();
    let samples = spinon_render_wgpu::readback_runtime_scene_samples(
        &scene,
        301,
        100,
        &[(25, 15), (150, 50), (82, 15)],
    )
    .expect("실제 WGPU offscreen readback");
    assert_eq!(
        samples,
        [[51, 102, 255, 255], [18, 52, 86, 255], [18, 52, 86, 255],]
    );

    let empty_scene = RuntimeRenderSnapshot::new(
        RuntimeRenderKey::new(
            document_snapshot.generation(),
            document_snapshot.document_revision(),
            document_snapshot.render_tree_revision(),
            StyleRevision::INITIAL,
            spinon_core::EnvironmentRevision::INITIAL,
        ),
        CssSize::new(301.0, 100.0).unwrap(),
        Vec::new(),
    )
    .unwrap();
    let clear_sample =
        spinon_render_wgpu::readback_runtime_scene_samples(&empty_scene, 2, 2, &[(0, 0)])
            .expect("빈 runtime root의 WGPU clear readback");
    assert_eq!(clear_sample, [[0, 0, 0, 255]]);
}
