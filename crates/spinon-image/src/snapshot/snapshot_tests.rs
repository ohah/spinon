use std::sync::Arc;
use std::{sync::mpsc, time::Duration};

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, IntrinsicSizeRevision, OwnerId,
};

use super::{
    ImageResourceRejectionReason, ImageResourceSnapshot, ImageSnapshotError,
    MAX_SNAPSHOT_IMAGE_BYTES, MAX_SNAPSHOT_IMAGES, next_snapshot_budget,
};
use crate::{ImageRequest, ImageResourceCompletion, PngDecoder};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const PNG: &[u8] = include_bytes!("../../../../tests/fixtures/resources/c14-1-png-basic.png");

#[test]
fn snapshot_image_count_limit_accepts_boundary_and_rejects_next_entry() {
    assert_eq!(
        next_snapshot_budget(MAX_SNAPSHOT_IMAGES - 1, 0, 0),
        Ok((MAX_SNAPSHOT_IMAGES, 0))
    );
    assert_eq!(
        next_snapshot_budget(MAX_SNAPSHOT_IMAGES, 0, 0),
        Err(ImageSnapshotError::TooManyImages {
            maximum: MAX_SNAPSHOT_IMAGES,
        })
    );
}

#[test]
fn snapshot_pixel_byte_limit_accepts_boundary_and_rejects_overflow() {
    assert_eq!(
        next_snapshot_budget(
            0,
            MAX_SNAPSHOT_IMAGE_BYTES - 32 * 1024 * 1024,
            32 * 1024 * 1024
        ),
        Ok((1, MAX_SNAPSHOT_IMAGE_BYTES))
    );
    assert_eq!(
        next_snapshot_budget(0, MAX_SNAPSHOT_IMAGE_BYTES, 1),
        Err(ImageSnapshotError::SnapshotImageBytesExceeded {
            maximum: MAX_SNAPSHOT_IMAGE_BYTES,
        })
    );
    assert_eq!(
        next_snapshot_budget(0, usize::MAX, 1),
        Err(ImageSnapshotError::SnapshotImageBytesExceeded {
            maximum: MAX_SNAPSHOT_IMAGE_BYTES,
        })
    );
}

#[test]
fn snapshot_constructor_rejects_more_than_the_image_count_limit() {
    let (document, images) = document_with_images(MAX_SNAPSHOT_IMAGES + 1);
    let snapshot = document.snapshot();
    let decoded = Arc::new(PngDecoder.decode(PNG).unwrap());
    let entries = images.into_iter().map(|image| {
        ImageResourceCompletion::from_parts(
            ImageRequest::from_document(&snapshot, image).unwrap(),
            decoded.clone(),
        )
    });

    assert!(matches!(
        ImageResourceSnapshot::new(&snapshot, IntrinsicSizeRevision::INITIAL, entries),
        Err(ImageSnapshotError::TooManyImages {
            maximum: MAX_SNAPSHOT_IMAGES
        })
    ));
}

#[test]
fn out_of_order_decode_keeps_current_success_and_classifies_late_success_as_stale() {
    let (mut document, images) = document_with_images(1);
    let image = images[0];
    let old_document = document.snapshot();
    let old_request = ImageRequest::from_document(&old_document, image).unwrap();
    set_image_src(&mut document, image, "https://spinon.invalid/b.png");
    let current_document = document.snapshot();
    let current_request = ImageRequest::from_document(&current_document, image).unwrap();

    let (release_old, wait_old) = mpsc::sync_channel(0);
    let (old_done, receive_old) = mpsc::channel();
    std::thread::spawn(move || {
        wait_old.recv().unwrap();
        old_done.send(old_request.decode_outcome(PNG)).unwrap();
    });
    let (current_done, receive_current) = mpsc::channel();
    std::thread::spawn(move || {
        current_done
            .send(current_request.decode_outcome(PNG))
            .unwrap();
    });

    let current = receive_current
        .recv_timeout(Duration::from_secs(2))
        .unwrap();
    release_old.send(()).unwrap();
    let late = receive_old.recv_timeout(Duration::from_secs(2)).unwrap();
    let (snapshot, rejected) = ImageResourceSnapshot::new_with_outcomes(
        &current_document,
        IntrinsicSizeRevision::INITIAL,
        [current, late],
    )
    .unwrap();

    assert_eq!(snapshot.images().len(), 1);
    assert!(snapshot.failures().is_empty());
    assert_eq!(rejected.len(), 1);
    assert_eq!(
        rejected[0].reason(),
        ImageResourceRejectionReason::SourceChanged
    );
    assert_eq!(
        rejected[0].request().src().to_string_lossy(),
        "https://spinon.invalid/image.png"
    );
}

#[test]
fn late_decode_failure_retains_request_identity_and_does_not_clear_current_success() {
    let (mut document, images) = document_with_images(1);
    let image = images[0];
    let old_document = document.snapshot();
    let old_request = ImageRequest::from_document(&old_document, image).unwrap();
    set_image_src(&mut document, image, "https://spinon.invalid/recovery.png");
    let current_document = document.snapshot();
    let current_request = ImageRequest::from_document(&current_document, image).unwrap();

    let late_failure = old_request.decode_outcome(b"not a PNG");
    assert!(late_failure.error().is_some());
    assert_eq!(
        late_failure.request().src().to_string_lossy(),
        "https://spinon.invalid/image.png"
    );
    let current_success = current_request.decode_outcome(PNG);
    let (snapshot, rejected) = ImageResourceSnapshot::new_with_outcomes(
        &current_document,
        IntrinsicSizeRevision::INITIAL,
        [current_success, late_failure],
    )
    .unwrap();

    assert_eq!(snapshot.images().len(), 1);
    assert!(snapshot.failures().is_empty());
    assert_eq!(rejected.len(), 1);
    assert_eq!(
        rejected[0].reason(),
        ImageResourceRejectionReason::SourceChanged
    );
}

#[test]
fn same_src_completion_from_an_older_document_revision_is_rejected() {
    let (mut document, images) = document_with_images(1);
    let image = images[0];
    let old_document = document.snapshot();
    let old_request = ImageRequest::from_document(&old_document, image).unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(92).unwrap(), document.document_revision());
    batch.push(DocumentOperation::SetAttribute {
        node: image,
        name: AttributeName::new(None, "alt").unwrap(),
        value: "updated while decode is pending".to_owned().into(),
    });
    document.commit(batch).unwrap();
    let current_document = document.snapshot();

    assert_eq!(
        old_request.src().to_string_lossy(),
        "https://spinon.invalid/image.png"
    );
    let (snapshot, rejected) = ImageResourceSnapshot::new_with_outcomes(
        &current_document,
        IntrinsicSizeRevision::INITIAL,
        [old_request.decode_outcome(PNG)],
    )
    .unwrap();

    assert!(snapshot.images().is_empty());
    assert!(snapshot.failures().is_empty());
    assert_eq!(rejected.len(), 1);
    assert_eq!(
        rejected[0].reason(),
        ImageResourceRejectionReason::StaleRequest
    );
}

#[test]
fn current_decode_failure_is_recorded_without_an_image_and_next_source_recovers() {
    let (mut document, images) = document_with_images(1);
    let image = images[0];
    let failed_document = document.snapshot();
    let failed_request = ImageRequest::from_document(&failed_document, image).unwrap();
    let (failed_snapshot, stale) = ImageResourceSnapshot::new_with_outcomes(
        &failed_document,
        IntrinsicSizeRevision::INITIAL,
        [failed_request.decode_outcome(b"not a PNG")],
    )
    .unwrap();
    assert!(stale.is_empty());
    assert!(failed_snapshot.images().is_empty());
    assert!(failed_snapshot.failures().contains_key(&image.id()));

    set_image_src(&mut document, image, "https://spinon.invalid/recovery.png");
    let recovered_document = document.snapshot();
    let recovered_request = ImageRequest::from_document(&recovered_document, image).unwrap();
    let (recovered_snapshot, stale) = ImageResourceSnapshot::new_with_outcomes(
        &recovered_document,
        IntrinsicSizeRevision::INITIAL.checked_next().unwrap(),
        [recovered_request.decode_outcome(PNG)],
    )
    .unwrap();
    assert!(stale.is_empty());
    assert!(recovered_snapshot.failures().is_empty());
    assert_eq!(recovered_snapshot.images().len(), 1);
}

#[test]
fn pending_intrinsic_snapshot_rebinds_only_prior_dimensions_to_current_source() {
    let (mut document, images) = document_with_images(1);
    let image = images[0];
    let previous_document = document.snapshot();
    let request = ImageRequest::from_document(&previous_document, image).unwrap();
    let completion = request.decode(PNG).unwrap();
    let previous = ImageResourceSnapshot::new(
        &previous_document,
        IntrinsicSizeRevision::INITIAL,
        [completion],
    )
    .unwrap();

    set_image_src(&mut document, image, "https://spinon.invalid/pending.png");
    let current_document = document.snapshot();
    let pending = previous
        .to_pending_layout_intrinsic_size_snapshot(
            &current_document,
            IntrinsicSizeRevision::INITIAL.checked_next().unwrap(),
            [image],
        )
        .unwrap();

    assert_eq!(
        pending.source(),
        spinon_layout::LayoutSourceRevision::HostDocument {
            generation: current_document.generation(),
            document: current_document.document_revision(),
            render_tree: current_document.render_tree_revision(),
        }
    );
    assert_eq!(pending.sizes().len(), 1);
    assert_eq!(pending.sizes()[&image.id()].width, Some(24.0));
    assert_eq!(pending.sizes()[&image.id()].height, Some(16.0));
}

#[test]
fn new_document_generation_uses_zero_size_without_inheriting_old_measurements() {
    let (old_document, old_images) = document_with_images(1);
    let old_snapshot = old_document.snapshot();
    let old_request = ImageRequest::from_document(&old_snapshot, old_images[0]).unwrap();
    let previous = ImageResourceSnapshot::new(
        &old_snapshot,
        IntrinsicSizeRevision::INITIAL,
        [old_request.decode(PNG).unwrap()],
    )
    .unwrap();
    let (new_document, new_images) = document_with_images(1);
    let new_snapshot = new_document.snapshot();

    let pending = previous
        .to_pending_layout_intrinsic_size_snapshot(
            &new_snapshot,
            IntrinsicSizeRevision::INITIAL,
            [new_images[0]],
        )
        .unwrap();
    assert_eq!(pending.sizes().len(), 1);
    assert_eq!(pending.sizes()[&new_images[0].id()].width, Some(0.0));
    assert_eq!(pending.sizes()[&new_images[0].id()].height, Some(0.0));
}

#[test]
fn changed_document_tree_and_removed_element_reject_success_or_failure_outcomes() {
    let (mut document, images) = document_with_images(1);
    let image = images[0];
    let request = ImageRequest::from_document(&document.snapshot(), image).unwrap();
    set_image_src(&mut document, image, "https://spinon.invalid/changed.png");
    let (snapshot, rejected) = ImageResourceSnapshot::new_with_outcomes(
        &document.snapshot(),
        IntrinsicSizeRevision::INITIAL,
        [request.decode_outcome(PNG)],
    )
    .unwrap();
    assert!(snapshot.images().is_empty());
    assert_eq!(
        rejected[0].reason(),
        ImageResourceRejectionReason::SourceChanged
    );

    let (removed_request, current) = {
        let request = ImageRequest::from_document(&document.snapshot(), image).unwrap();
        let snapshot = document.snapshot();
        (request, snapshot)
    };
    let mut removal =
        DocumentChangeBatch::new(OwnerId::new(92).unwrap(), document.document_revision());
    removal.push(DocumentOperation::RemoveChild {
        parent: HostParent::Node(document.snapshot().root_children().next().unwrap()),
        node: image,
    });
    document.commit(removal).unwrap();
    let (snapshot, rejected) = ImageResourceSnapshot::new_with_outcomes(
        &document.snapshot(),
        IntrinsicSizeRevision::INITIAL,
        [removed_request.decode_outcome(b"broken")],
    )
    .unwrap();
    assert!(snapshot.failures().is_empty());
    assert_eq!(
        rejected[0].reason(),
        ImageResourceRejectionReason::InvalidRequest
    );
    assert!(current.node(image).is_some());
}

#[test]
fn new_document_generation_rejects_old_success_and_failure_outcomes() {
    let (old_document, old_images) = document_with_images(1);
    let old_snapshot = old_document.snapshot();
    let old_request = ImageRequest::from_document(&old_snapshot, old_images[0]).unwrap();
    let (new_document, _) = document_with_images(1);
    let current = new_document.snapshot();

    for outcome in [
        old_request.clone().decode_outcome(PNG),
        old_request.decode_outcome(b"broken"),
    ] {
        let (snapshot, rejected) = ImageResourceSnapshot::new_with_outcomes(
            &current,
            IntrinsicSizeRevision::INITIAL,
            [outcome],
        )
        .unwrap();
        assert!(snapshot.images().is_empty());
        assert!(snapshot.failures().is_empty());
        assert_eq!(
            rejected[0].reason(),
            ImageResourceRejectionReason::StaleRequest
        );
    }
}

#[test]
fn duplicate_current_success_and_failure_for_one_node_are_rejected() {
    let (document, images) = document_with_images(1);
    let snapshot = document.snapshot();
    let request = ImageRequest::from_document(&snapshot, images[0]).unwrap();
    assert!(matches!(
        ImageResourceSnapshot::new_with_outcomes(
            &snapshot,
            IntrinsicSizeRevision::INITIAL,
            [
                request.clone().decode_outcome(PNG),
                request.decode_outcome(b"broken"),
            ],
        ),
        Err(ImageSnapshotError::DuplicateNode(node)) if node == images[0].id()
    ));
}

#[test]
fn failed_image_does_not_block_another_node_success() {
    let (document, images) = document_with_images(2);
    let snapshot = document.snapshot();
    let failed = ImageRequest::from_document(&snapshot, images[0]).unwrap();
    let succeeded = ImageRequest::from_document(&snapshot, images[1]).unwrap();
    let (resources, rejected) = ImageResourceSnapshot::new_with_outcomes(
        &snapshot,
        IntrinsicSizeRevision::INITIAL,
        [
            failed.decode_outcome(b"broken"),
            succeeded.decode_outcome(PNG),
        ],
    )
    .unwrap();

    assert!(rejected.is_empty());
    assert_eq!(resources.failures().len(), 1);
    assert_eq!(resources.images().len(), 1);
    assert!(resources.failures().contains_key(&images[0].id()));
    assert!(resources.images().contains_key(&images[1].id()));
}

#[test]
fn outcome_count_limit_includes_failures() {
    let (document, images) = document_with_images(MAX_SNAPSHOT_IMAGES + 1);
    let snapshot = document.snapshot();
    let outcomes = images.into_iter().map(|image| {
        ImageRequest::from_document(&snapshot, image)
            .unwrap()
            .decode_outcome(b"x")
    });

    assert!(matches!(
        ImageResourceSnapshot::new_with_outcomes(
            &snapshot,
            IntrinsicSizeRevision::INITIAL,
            outcomes,
        ),
        Err(ImageSnapshotError::TooManyImages {
            maximum: MAX_SNAPSHOT_IMAGES
        })
    ));
}

fn document_with_images(count: usize) -> (HostDocument, Vec<HostNodeHandle>) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(92).unwrap(), document.document_revision());
    let mut images = Vec::with_capacity(count);
    batch.push(DocumentOperation::CreateElement {
        node: root,
        namespace: HTML.to_owned(),
        local_name: "div".to_owned(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Root,
        node: root,
        before: None,
    });

    for _ in 0..count {
        let image = document.reserve_node_handle().unwrap();
        batch
            .push(DocumentOperation::CreateElement {
                node: image,
                namespace: HTML.to_owned(),
                local_name: "img".to_owned(),
            })
            .push(DocumentOperation::SetAttribute {
                node: image,
                name: AttributeName::new(None, "src").unwrap(),
                value: "https://spinon.invalid/image.png".to_owned().into(),
            })
            .push(DocumentOperation::InsertBefore {
                parent: HostParent::Node(root),
                node: image,
                before: None,
            });
        images.push(image);
    }
    document.commit(batch).unwrap();
    (document, images)
}

fn set_image_src(document: &mut HostDocument, image: HostNodeHandle, src: &str) {
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(92).unwrap(), document.document_revision());
    batch.push(DocumentOperation::SetAttribute {
        node: image,
        name: AttributeName::new(None, "src").unwrap(),
        value: src.to_owned().into(),
    });
    document.commit(batch).unwrap();
}
