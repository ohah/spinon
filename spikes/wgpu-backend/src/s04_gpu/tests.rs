use super::geometry::{build_vertices, srgb_to_linear, FIXTURE_HEIGHT, FIXTURE_WIDTH};
use super::readback::{
    sample_columns, sample_rows, validate_samples, READBACK_BYTES_PER_ROW, READBACK_SIZE,
};
use super::validate_surface_generation;
use crate::s04_snapshot;
use spinon_core::{EnvironmentRevision, HostDocument, StyleRevision};
use spinon_render::{
    ComputedStyleProfileId, CssRect, CssSize, LayoutProjectionId, OpaqueCssSrgb, PaintProfileId,
    StaticRenderBox, StaticRenderSnapshot, StaticRenderSource,
};

#[test]
fn fixture_surface_maps_css_points_once_and_letterboxes_without_stretching() {
    let snapshot = s04_snapshot::build_asymmetric_y_snapshot().expect("S04 snapshot");
    let surface_vertices = build_vertices(&snapshot, 1080, 2400, 3.0).expect("surface vertices");
    let readback_vertices =
        build_vertices(&snapshot, FIXTURE_WIDTH, FIXTURE_HEIGHT, 1.0).expect("readback vertices");
    assert_eq!(surface_vertices.len(), 4 * 6 * 6);
    assert_eq!(readback_vertices.len(), 4 * 6 * 6);
    assert!(surface_vertices.iter().all(|value| value.is_finite()));
    assert!((surface_vertices[0] + 0.8361111).abs() < 0.001);
    assert!((surface_vertices[6] - 0.8361111).abs() < 0.001);
    assert!((surface_vertices[1] - 0.08125).abs() < 0.001);
    assert!((surface_vertices[13] + 0.08125).abs() < 0.001);
    assert_eq!(readback_vertices[0], -1.0);
    assert_eq!(readback_vertices[1], 1.0);
    assert_eq!(readback_vertices[13], -1.0);
}

#[test]
fn asymmetric_nonzero_y_frames_map_to_top_down_surface_coordinates() {
    let mut document = HostDocument::new().expect("테스트 source 문서 생성");
    let source_snapshot = document.snapshot();
    let ids = (0..3)
        .map(|_| document.reserve_node_handle().expect("노드 ID 예약").id())
        .collect::<Vec<_>>();
    let source = StaticRenderSource {
        document_generation: source_snapshot.generation(),
        document_revision: source_snapshot.document_revision(),
        render_tree_revision: source_snapshot.render_tree_revision(),
        style_revision: StyleRevision::default(),
        environment_revision: EnvironmentRevision::default(),
        computed_style_profile: ComputedStyleProfileId::S04FlexPaintV1,
        layout_projection: LayoutProjectionId::TaffyFlexSubsetV1,
        paint_profile: PaintProfileId::OpaqueBackgroundColorV1,
        fixture_id: "s04-asymmetric-y-coordinate-test".to_owned(),
        fixture_sha256: [1; 32],
        stylesheet_sha256: [2; 32],
        chromium_reference_id: "s04-coordinate-math-test".to_owned(),
        chromium_reference_sha256: [3; 32],
    };
    let color = OpaqueCssSrgb::new(1, 2, 3);
    let frames = [
        (10.0, 7.0, 20.0, 11.0),
        (38.0, 25.0, 17.0, 23.0),
        (70.0, 56.0, 12.0, 14.0),
    ];
    let boxes = ids
        .into_iter()
        .zip(frames)
        .enumerate()
        .map(|(index, (id, (x, y, width, height)))| {
            StaticRenderBox::new(
                id,
                CssRect::new(x, y, width, height).expect("양수 비대칭 frame"),
                color,
                index as u32,
            )
        })
        .collect();
    let snapshot = StaticRenderSnapshot::new(
        source,
        CssSize::new(100.0, 80.0).expect("CSS viewport"),
        boxes,
    )
    .expect("비대칭 y snapshot");
    let vertices = build_vertices(&snapshot, 240, 180, 1.5).expect("표면 좌표 변환");

    assert_box_corners(&vertices, 0, (-0.5, 0.55, -0.25, 0.36666667));
    assert_box_corners(&vertices, 1, (-0.15, 0.25, 0.0625, -0.13333333));
    assert_box_corners(&vertices, 2, (0.25, -0.26666667, 0.4, -0.5));
}

fn assert_box_corners(vertices: &[f32], box_index: usize, expected: (f32, f32, f32, f32)) {
    let base = box_index * 36;
    let expected_positions = [
        (expected.0, expected.1),
        (expected.2, expected.1),
        (expected.0, expected.3),
        (expected.0, expected.3),
        (expected.2, expected.1),
        (expected.2, expected.3),
    ];
    for (vertex, (expected_x, expected_y)) in expected_positions.into_iter().enumerate() {
        for (axis_offset, expected_value) in [(0, expected_x), (1, expected_y)] {
            let index = base + vertex * 6 + axis_offset;
            assert!(
                (vertices[index] - expected_value).abs() < 0.00001,
                "vertex {index}: {} != {expected_value}",
                vertices[index]
            );
        }
    }
}

#[test]
fn fixture_readback_samples_use_the_pinned_columns_rows_and_padded_stride() {
    let snapshot = s04_snapshot::build_asymmetric_y_snapshot().expect("S04 snapshot");
    assert_eq!(READBACK_SIZE, 1280 * 65);
    let mut bytes = vec![0; READBACK_SIZE as usize];
    for y in sample_rows() {
        for x in sample_columns() {
            let offset = (y * READBACK_BYTES_PER_ROW + x * 4) as usize;
            bytes[offset..offset + 4].copy_from_slice(
                &super::readback::expected_color(&snapshot, x, y).expect("fixture sample color"),
            );
        }
    }
    validate_samples(&snapshot, &bytes).expect("fixed readback sample agreement");
    bytes[14 * READBACK_BYTES_PER_ROW as usize + 150 * 4] ^= 1;
    assert!(validate_samples(&snapshot, &bytes).is_err());
}

#[test]
fn readback_rejects_wrong_lengths_and_ignores_alignment_padding() {
    let snapshot = s04_snapshot::build_asymmetric_y_snapshot().expect("S04 snapshot");
    let mut bytes = vec![0; READBACK_SIZE as usize];
    let padding_offset = 64 * READBACK_BYTES_PER_ROW as usize + 301 * 4;
    bytes[padding_offset..(65 * READBACK_BYTES_PER_ROW as usize)].fill(0xff);
    for y in sample_rows() {
        for x in sample_columns() {
            let offset = (y * READBACK_BYTES_PER_ROW + x * 4) as usize;
            bytes[offset..offset + 4].copy_from_slice(
                &super::readback::expected_color(&snapshot, x, y).expect("sample box"),
            );
        }
    }
    assert!(validate_samples(&snapshot, &bytes).is_ok());
    assert!(validate_samples(&snapshot, &bytes[..bytes.len() - 1]).is_err());
    let mut oversized = bytes;
    oversized.push(0);
    assert!(validate_samples(&snapshot, &oversized).is_err());
}

#[test]
fn srgb_transfer_function_preserves_exact_linear_endpoints() {
    assert_eq!(srgb_to_linear(0.0), 0.0);
    assert_eq!(srgb_to_linear(1.0), 1.0);
    assert!((srgb_to_linear(0.04045) - 0.0031308).abs() < 0.000001);
}

#[test]
fn asymmetric_y_samples_cover_three_columns_and_twelve_distinct_rows() {
    let columns: Vec<_> = sample_columns().collect();
    assert_eq!(columns, [0, 150, 300]);
    assert!(columns.windows(2).all(|pair| pair[0] < pair[1]));
    let rows: Vec<_> = sample_rows().collect();
    assert_eq!(rows, [0, 11, 12, 14, 15, 32, 33, 35, 36, 59, 60, 64]);
    assert!(rows.windows(2).all(|pair| pair[0] < pair[1]));
}

#[test]
fn surface_generation_must_advance_strictly() {
    assert!(validate_surface_generation(1, 2).is_ok());
    assert!(validate_surface_generation(1, 1).is_err());
    assert!(validate_surface_generation(2, 1).is_err());
    assert!(validate_surface_generation(u64::MAX, u64::MAX).is_err());
}
