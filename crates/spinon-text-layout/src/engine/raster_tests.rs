use crate::{GlyphBitmapContent, TextLayoutError};

use super::raster::{accumulated_raster_bytes, checked_bitmap};

fn node() -> spinon_core::NodeId {
    spinon_core::NodeId::new(1).unwrap()
}

#[test]
fn empty_swash_image_becomes_a_no_ink_glyph() {
    let image = swash::scale::image::Image::new();
    assert!(checked_bitmap(image, node(), 7).unwrap().is_none());
}

#[test]
fn malformed_or_oversized_swash_images_fail_closed() {
    let mut truncated = swash::scale::image::Image::new();
    truncated.placement.width = 2;
    truncated.placement.height = 2;
    truncated.data = vec![255; 3];
    assert!(matches!(
        checked_bitmap(truncated, node(), 8),
        Err(TextLayoutError::InvalidGlyphBitmap {
            paragraph,
            glyph_id: 8,
        }) if paragraph == node()
    ));

    let mut oversized = swash::scale::image::Image::new();
    oversized.placement.width = u32::MAX;
    oversized.placement.height = u32::MAX;
    assert!(matches!(
        checked_bitmap(oversized, node(), 9),
        Err(TextLayoutError::InvalidGlyphBitmap {
            paragraph,
            glyph_id: 9,
        }) if paragraph == node()
    ));

    let mut zero_width = swash::scale::image::Image::new();
    zero_width.placement.width = 0;
    zero_width.placement.height = 1;
    assert!(checked_bitmap(zero_width, node(), 12).unwrap().is_none());

    let mut invalid_top = swash::scale::image::Image::new();
    invalid_top.placement.width = 1;
    invalid_top.placement.height = 1;
    invalid_top.placement.top = i32::MIN;
    invalid_top.data = vec![255];
    assert!(matches!(
        checked_bitmap(invalid_top, node(), 10),
        Err(TextLayoutError::InvalidGlyphBitmap {
            paragraph,
            glyph_id: 10,
        }) if paragraph == node()
    ));
}

#[test]
fn rgba_swash_image_keeps_color_content_and_y_down_placement() {
    let mut image = swash::scale::image::Image::new();
    image.content = swash::scale::image::Content::Color;
    image.placement.left = -2;
    image.placement.top = 5;
    image.placement.width = 1;
    image.placement.height = 1;
    image.data = vec![255, 32, 16, 128];

    let bitmap = checked_bitmap(image, node(), 11).unwrap().unwrap();
    assert_eq!(bitmap.content, GlyphBitmapContent::ColorRgba8);
    assert_eq!((bitmap.left_px, bitmap.top_px), (-2, -5));
    assert_eq!(&*bitmap.pixels, &[255, 32, 16, 128]);
}

#[test]
fn accumulated_raster_bytes_accepts_the_limit_and_rejects_overflow() {
    let limit = 64 * 1024 * 1024;
    assert_eq!(
        accumulated_raster_bytes(limit - 4, 4, node()).unwrap(),
        limit
    );
    assert!(matches!(
        accumulated_raster_bytes(limit, 1, node()),
        Err(TextLayoutError::RasterSnapshotTooLarge {
            paragraph,
            max_bytes,
        }) if paragraph == node() && max_bytes == limit
    ));
    assert!(matches!(
        accumulated_raster_bytes(usize::MAX, 1, node()),
        Err(TextLayoutError::RasterSnapshotTooLarge { .. })
    ));
}
