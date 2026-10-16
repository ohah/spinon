use super::ImageCacheKey;
use spinon_core::{HostDocument, IntrinsicSizeRevision, NodeId};
use spinon_render::{
    RuntimeImageObjectStyle, RuntimeImageResourceKey, RuntimeImageSampling, RuntimeRenderImage,
};
use std::sync::Arc;

#[test]
fn texture_identity_includes_source_node_content_and_dimensions() {
    let document = HostDocument::new().unwrap().snapshot();
    let source = RuntimeImageResourceKey::new(
        document.generation(),
        document.document_revision(),
        document.render_tree_revision(),
        IntrinsicSizeRevision::INITIAL,
    );
    let image = RuntimeRenderImage::new(
        NodeId::new(1).unwrap(),
        source,
        1,
        1,
        Arc::from([1_u8, 2, 3, 255]),
    )
    .unwrap();
    let base = ImageCacheKey::new(&image);
    let changed_pixels =
        RuntimeRenderImage::new(image.node_id(), source, 1, 1, Arc::from([3_u8, 2, 1, 255]))
            .unwrap();
    assert_ne!(base, ImageCacheKey::new(&changed_pixels));
    let changed_dimensions = RuntimeRenderImage::new(
        image.node_id(),
        source,
        2,
        1,
        Arc::from([1_u8, 2, 3, 255, 4, 5, 6, 255]),
    )
    .unwrap();
    assert_ne!(base, ImageCacheKey::new(&changed_dimensions));
    let other_source = RuntimeImageResourceKey::new(
        document.generation(),
        document.document_revision(),
        document.render_tree_revision(),
        IntrinsicSizeRevision::INITIAL.checked_next().unwrap(),
    );
    let changed_source = RuntimeRenderImage::new(
        image.node_id(),
        other_source,
        1,
        1,
        Arc::from([1_u8, 2, 3, 255]),
    )
    .unwrap();
    assert_ne!(base, ImageCacheKey::new(&changed_source));
    let other_generation = HostDocument::new().unwrap().snapshot().generation();
    let changed_generation = RuntimeRenderImage::new(
        image.node_id(),
        RuntimeImageResourceKey::new(
            other_generation,
            document.document_revision(),
            document.render_tree_revision(),
            IntrinsicSizeRevision::INITIAL,
        ),
        1,
        1,
        Arc::from([1_u8, 2, 3, 255]),
    )
    .unwrap();
    assert_ne!(base, ImageCacheKey::new(&changed_generation));
    assert_eq!(
        base.source.document_revision(),
        document.document_revision()
    );
    let linear = RuntimeRenderImage::new_with_object_style(
        image.node_id(),
        source,
        1,
        1,
        Arc::from([1_u8, 2, 3, 255]),
        RuntimeImageObjectStyle::parse("fill", "50% 50%", "auto").unwrap(),
        spinon_core::StyleRevision::INITIAL,
    )
    .unwrap();
    assert_eq!(linear.sampling(), RuntimeImageSampling::LinearEncoded);
    assert_ne!(base, ImageCacheKey::new(&linear));
}
