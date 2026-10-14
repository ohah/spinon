use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, IntrinsicSizeRevision, OwnerId,
};

use super::{
    ImageResourceSnapshot, ImageSnapshotError, MAX_SNAPSHOT_IMAGE_BYTES, MAX_SNAPSHOT_IMAGES,
    next_snapshot_budget,
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
