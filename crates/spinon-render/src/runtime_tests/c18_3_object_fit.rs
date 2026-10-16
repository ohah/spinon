use spinon_core::{
    EnvironmentRevision, HostDocument, IntrinsicSizeRevision, NodeId, StyleRevision,
};
use std::sync::Arc;

use super::super::{
    CssRect, CssSize, RuntimeImageObjectPosition, RuntimeImageObjectStyle, RuntimeImageResourceKey,
    RuntimePaint, RuntimeRenderBox, RuntimeRenderError, RuntimeRenderImage, RuntimeRenderKey,
    RuntimeRenderSnapshot,
};

fn image_with_style(fit: &str, position: &str) -> RuntimeRenderImage {
    let document = HostDocument::new().unwrap().snapshot();
    let source = RuntimeImageResourceKey::new(
        document.generation(),
        document.document_revision(),
        document.render_tree_revision(),
        IntrinsicSizeRevision::INITIAL,
    );
    RuntimeRenderImage::new_with_object_style(
        NodeId::new(7).unwrap(),
        source,
        24,
        12,
        Arc::from([255_u8; 24 * 12 * 4]),
        RuntimeImageObjectStyle::parse(fit, position, "auto").unwrap(),
        StyleRevision::INITIAL,
    )
    .unwrap()
}

#[test]
fn image_object_style_is_revision_bound_and_can_use_a_different_frame_size() {
    let document = HostDocument::new().unwrap().snapshot();
    let intrinsic_revision = spinon_core::IntrinsicSizeRevision::INITIAL;
    let style_revision = StyleRevision::INITIAL;
    let source = RuntimeImageResourceKey::new(
        document.generation(),
        document.document_revision(),
        document.render_tree_revision(),
        intrinsic_revision,
    );
    let node = NodeId::new(18).unwrap();
    let image = RuntimeRenderImage::new_with_object_style(
        node,
        source,
        2,
        1,
        Arc::from([255_u8, 0, 0, 255, 0, 255, 0, 255]),
        RuntimeImageObjectStyle::parse("cover", "50% 50%", "auto").unwrap(),
        style_revision,
    )
    .unwrap();
    let mut render_key = RuntimeRenderKey::new(
        document.generation(),
        document.document_revision(),
        document.render_tree_revision(),
        style_revision,
        EnvironmentRevision::INITIAL,
    )
    .with_intrinsic_size_revision(intrinsic_revision);
    let frame = CssRect::new(0.0, 0.0, 4.0, 4.0).unwrap();
    let scene = RuntimeRenderSnapshot::new(
        render_key,
        CssSize::new(8.0, 8.0).unwrap(),
        vec![RuntimeRenderBox::new(node, frame, RuntimePaint::None, 0)],
    )
    .unwrap();
    assert!(scene.with_images(vec![image.clone()]).is_ok());

    render_key = RuntimeRenderKey::new(
        document.generation(),
        document.document_revision(),
        document.render_tree_revision(),
        style_revision.checked_next().unwrap(),
        EnvironmentRevision::INITIAL,
    )
    .with_intrinsic_size_revision(intrinsic_revision);
    let stale_scene = RuntimeRenderSnapshot::new(
        render_key,
        CssSize::new(8.0, 8.0).unwrap(),
        vec![RuntimeRenderBox::new(node, frame, RuntimePaint::None, 0)],
    )
    .unwrap();
    assert!(matches!(
        stale_scene.with_images(vec![image]),
        Err(RuntimeRenderError::InvalidImageStyleRevision(id)) if id == node
    ));
}

#[test]
fn image_object_fit_computes_concrete_sizes_and_positions() {
    let frame = CssRect::new(104.0, 8.0, 24.0, 24.0).unwrap();
    assert_eq!(
        image_with_style("fill", "50% 50%")
            .object_rect_css_px(frame)
            .unwrap(),
        frame
    );
    assert_eq!(
        image_with_style("contain", "50% 50%")
            .object_rect_css_px(frame)
            .unwrap(),
        CssRect::new(104.0, 14.0, 24.0, 12.0).unwrap()
    );
    assert_eq!(
        image_with_style("cover", "50% 50%")
            .object_rect_css_px(frame)
            .unwrap(),
        CssRect::new(92.0, 8.0, 48.0, 24.0).unwrap()
    );
    assert_eq!(
        image_with_style("none", "100% 100%")
            .object_rect_css_px(frame)
            .unwrap(),
        CssRect::new(104.0, 20.0, 24.0, 12.0).unwrap()
    );
    assert_eq!(
        image_with_style("scale-down", "50% 50%")
            .object_rect_css_px(CssRect::new(40.0, 40.0, 16.0, 16.0).unwrap())
            .unwrap(),
        CssRect::new(40.0, 44.0, 16.0, 8.0).unwrap()
    );
    assert_eq!(
        image_with_style("scale-down", "50% 50%")
            .object_rect_css_px(CssRect::new(72.0, 40.0, 32.0, 20.0).unwrap())
            .unwrap(),
        CssRect::new(76.0, 44.0, 24.0, 12.0).unwrap()
    );
    assert_eq!(
        image_with_style("contain", "-25% 150%")
            .object_rect_css_px(CssRect::new(144.0, 40.0, 24.0, 24.0).unwrap())
            .unwrap(),
        CssRect::new(144.0, 58.0, 24.0, 12.0).unwrap()
    );
    assert_eq!(
        image_with_style("cover", "150% -25%")
            .object_rect_css_px(CssRect::new(176.0, 40.0, 24.0, 24.0).unwrap())
            .unwrap(),
        CssRect::new(140.0, 40.0, 48.0, 24.0).unwrap()
    );
}

#[test]
fn image_object_style_parser_fails_closed_for_unhandled_computed_values() {
    for (fit, position, rendering) in [
        ("scale", "50% 50%", "auto"),
        ("cover", "50%", "auto"),
        ("cover", "left 50%", "auto"),
        ("cover", "10px 50%", "auto"),
        ("cover", "10% 20% 30%", "auto"),
        ("cover", "calc(10%) 20%", "auto"),
        ("cover", "NaN% 20%", "auto"),
        ("cover", "50% 50%", "crisp-edges"),
    ] {
        assert!(
            RuntimeImageObjectStyle::parse(fit, position, rendering).is_err(),
            "fit={fit} position={position} rendering={rendering}"
        );
    }
    assert!(RuntimeImageObjectPosition::new(f32::INFINITY, 50.0).is_err());
}

#[test]
fn image_object_geometry_rejects_zero_and_unrepresentable_results() {
    let image = image_with_style("none", "-1e38% 50%");
    let overflowing_frame = CssRect::new(f32::MAX * 0.95, 0.0, 1.0, 12.0).unwrap();
    assert!(matches!(
        image.object_rect_css_px(overflowing_frame),
        Err(RuntimeRenderError::InvalidImageGeometry(node)) if node == NodeId::new(7).unwrap()
    ));
    assert!(matches!(
        image.object_rect_css_px(CssRect::new(0.0, 0.0, 0.0, 12.0).unwrap()),
        Err(RuntimeRenderError::InvalidImageGeometry(node)) if node == NodeId::new(7).unwrap()
    ));
}
