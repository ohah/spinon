use super::{
    A_PNG, EXPECTED_IMAGES, EXPECTED_SAMPLES, RUNTIME_CSS_C18_4_ASYNC_IMAGE_FIXTURE_SOURCE,
    run_transition_sequence,
};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use std::{sync::mpsc, thread};

#[test]
fn transition_sequence_rejects_stale_results_clears_failure_and_recovers() {
    let report: serde_json::Value =
        serde_json::from_str(&run_transition_sequence().unwrap()).unwrap();
    assert_eq!(
        report["race"]["completionOrder"],
        serde_json::json!(["B", "A"])
    );
    assert_eq!(report["race"]["staleARejected"], 1);
    assert_eq!(report["race"]["activeSrc"], super::B_SRC);
    assert_eq!(report["replacement"]["pendingImageCount"], 0);
    assert_eq!(
        report["replacement"]["retainedIntrinsicSize"],
        serde_json::json!([1, 4])
    );
    assert_eq!(
        report["replacement"]["successNaturalSize"],
        serde_json::json!([4, 2])
    );
    assert_eq!(report["failure"]["imageCount"], 0);
    assert_eq!(report["failure"]["recordedErrors"], 1);
    assert_eq!(report["recovery"]["naturalSize"], serde_json::json!([2, 1]));
    assert_eq!(
        report["autoSize"]["replacementPendingNaturalSize"],
        serde_json::json!([2, 2])
    );
    assert_eq!(
        report["autoSize"]["replacementSuccessNaturalSize"],
        serde_json::json!([4, 2])
    );
    assert_eq!(
        report["autoSize"]["firstPendingFrame"],
        serde_json::json!([0.0, 0.0])
    );
    assert_eq!(
        report["autoSize"]["firstSuccessFrame"],
        serde_json::json!([2.0, 2.0])
    );
    assert_eq!(
        report["autoSize"]["replacementPendingFrame"],
        serde_json::json!([2.0, 2.0])
    );
    assert_eq!(
        report["autoSize"]["replacementSuccessFrame"],
        serde_json::json!([4.0, 2.0])
    );
}

#[test]
fn canceled_or_panicking_decode_worker_does_not_publish_into_pending_snapshot() {
    let (document, image) = super::transition_document(super::A_SRC).unwrap();
    let source = document.snapshot();
    let request = ImageRequest::from_document(&source, image).unwrap();
    let pending = ImageResourceSnapshot::new_with_outcomes(
        &source,
        spinon_core::IntrinsicSizeRevision::INITIAL,
        [],
    )
    .unwrap()
    .0;

    let (continue_decode, wait_for_continue) = mpsc::sync_channel::<()>(0);
    let (acknowledge_start, worker_started) = mpsc::sync_channel::<()>(0);
    let (send_result, receive_result) = mpsc::channel();
    let canceled_worker = thread::spawn(move || {
        acknowledge_start.send(()).unwrap();
        if wait_for_continue.recv().is_ok() {
            let _ = send_result.send(request.decode_outcome(A_PNG));
        }
    });
    worker_started.recv().unwrap();
    drop(continue_decode);
    canceled_worker.join().unwrap();
    assert!(matches!(
        receive_result.try_recv(),
        Err(mpsc::TryRecvError::Disconnected)
    ));
    assert!(pending.images().is_empty());
    assert!(pending.failures().is_empty());

    let panicking_worker = thread::spawn(|| panic!("injected C18.4 decode worker panic"));
    assert!(panicking_worker.join().is_err());
    assert!(pending.images().is_empty());
    assert!(pending.failures().is_empty());
}

#[test]
fn opt_in_source_and_pixel_contract_are_complete_and_stable() {
    assert_eq!(EXPECTED_IMAGES.len(), 5);
    assert_eq!(EXPECTED_SAMPLES.len(), 6);
    for expected in EXPECTED_IMAGES {
        assert!(RUNTIME_CSS_C18_4_ASYNC_IMAGE_FIXTURE_SOURCE.contains(expected.id));
        assert!(RUNTIME_CSS_C18_4_ASYNC_IMAGE_FIXTURE_SOURCE.contains(expected.src));
    }
}
