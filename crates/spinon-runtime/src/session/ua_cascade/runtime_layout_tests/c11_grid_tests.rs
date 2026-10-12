use super::*;
use crate::session::ua_cascade::calculation::compute_request_for_block_flex_grid_placement;

#[test]
fn c11_grid_profile_publishes_taffy_frames_and_ordered_paint_boxes() {
    let (snapshot, root, children) = super::make_layout_snapshot(
        "display:grid;width:320px;height:130px;grid-template-columns:80px 100px 120px;grid-template-rows:50px 70px;column-gap:8px;row-gap:6px;background-color:#edf1f7",
        &[
            "width:40px;height:20px;background-color:#2866f6",
            "width:50px;height:24px;background-color:#e5484d",
            "width:60px;height:26px;background-color:#19a974",
            "width:30px;height:18px;background-color:#ffc857",
        ],
        false,
        None,
    );
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport {
            width_css_px: 320.0,
            height_css_px: 240.0,
            device_scale_factor: 1.0,
            environment_revision: EnvironmentRevision::INITIAL,
            media_environment: CssMediaEnvironment::MOBILE,
        },
        previous_snapshot: None,
        force_full: true,
        invalidation: None,
        previous_styles: None,
    };

    let calculation = compute_request_for_block_flex_grid(&request).unwrap();
    let layout = calculation.layout.unwrap();
    assert_eq!(layout.frames.len(), 5);
    let expected = [
        (root, 0.0, 0.0, 320.0, 130.0),
        (children[0], 0.0, 0.0, 40.0, 20.0),
        (children[1], 88.0, 0.0, 50.0, 24.0),
        (children[2], 196.0, 0.0, 60.0, 26.0),
        (children[3], 0.0, 56.0, 30.0, 18.0),
    ];
    for (handle, x, y, width, height) in expected {
        let frame = layout
            .frames
            .iter()
            .find(|frame| frame.node_id == handle.id().get())
            .unwrap();
        assert_eq!(
            (frame.x, frame.y, frame.width, frame.height),
            (x, y, width, height)
        );
    }

    let scene = layout.render_snapshot.unwrap();
    assert_eq!(scene.boxes().len(), 5);
    assert_eq!(scene.boxes()[0].node_id(), root.id());
    assert_eq!(scene.boxes()[1].node_id(), children[0].id());
    assert_eq!(scene.boxes()[2].node_id(), children[1].id());
    assert_eq!(scene.boxes()[3].node_id(), children[2].id());
    assert_eq!(scene.boxes()[4].node_id(), children[3].id());
}

#[test]
fn c1121_explicit_grid_placement_publishes_line_and_span_frames() {
    let (snapshot, root, children) = super::make_layout_snapshot(
        "display:grid;width:320px;height:130px;grid-template-columns:80px 100px 120px;grid-template-rows:50px 70px;column-gap:8px;row-gap:6px;background-color:#edf1f7",
        &[
            "width:40px;height:20px;grid-column:1 / span 2;grid-row:1 / 2;background-color:#2866f6",
            "width:50px;height:24px;grid-column:-2 / -1;grid-row:2 / 3;background-color:#e5484d",
        ],
        false,
        None,
    );
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport {
            width_css_px: 320.0,
            height_css_px: 240.0,
            device_scale_factor: 1.0,
            environment_revision: EnvironmentRevision::INITIAL,
            media_environment: CssMediaEnvironment::MOBILE,
        },
        previous_snapshot: None,
        force_full: true,
        invalidation: None,
        previous_styles: None,
    };

    let calculation = compute_request_for_block_flex_grid_placement(&request).unwrap();
    let layout = calculation.layout.unwrap();
    assert_eq!(layout.frames.len(), 3);
    let expected = [
        (root, 0.0, 0.0, 320.0, 130.0),
        (children[0], 0.0, 0.0, 40.0, 20.0),
        (children[1], 196.0, 56.0, 50.0, 24.0),
    ];
    for (handle, x, y, width, height) in expected {
        let frame = layout
            .frames
            .iter()
            .find(|frame| frame.node_id == handle.id().get())
            .unwrap();
        assert_eq!(
            (frame.x, frame.y, frame.width, frame.height),
            (x, y, width, height)
        );
    }
    let scene = layout.render_snapshot.unwrap();
    assert_eq!(scene.boxes().len(), 3);
}

#[test]
fn c1121_overlapping_items_keep_dom_order_in_paint_snapshot() {
    let (snapshot, root, children) = super::make_layout_snapshot(
        "display:grid;width:320px;height:130px;grid-template-columns:320px;grid-template-rows:130px",
        &[
            "width:40px;height:20px;grid-column:1 / 2;grid-row:1 / 2;background-color:#2866f6",
            "width:50px;height:24px;grid-column:1 / 2;grid-row:1 / 2;background-color:#e5484d",
        ],
        false,
        None,
    );
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport {
            width_css_px: 320.0,
            height_css_px: 240.0,
            device_scale_factor: 1.0,
            environment_revision: EnvironmentRevision::INITIAL,
            media_environment: CssMediaEnvironment::MOBILE,
        },
        previous_snapshot: None,
        force_full: true,
        invalidation: None,
        previous_styles: None,
    };

    let calculation = compute_request_for_block_flex_grid_placement(&request).unwrap();
    let layout = calculation.layout.unwrap();
    let first = layout
        .frames
        .iter()
        .find(|frame| frame.node_id == children[0].id().get())
        .unwrap();
    let second = layout
        .frames
        .iter()
        .find(|frame| frame.node_id == children[1].id().get())
        .unwrap();
    assert_eq!((first.x, first.y), (second.x, second.y));
    assert!(first.width > 0.0 && first.height > 0.0);
    assert!(second.width > 0.0 && second.height > 0.0);
    assert!(first.x < second.x + second.width && second.x < first.x + first.width);
    assert!(first.y < second.y + second.height && second.y < first.y + first.height);

    let scene = layout.render_snapshot.unwrap();
    assert_eq!(scene.boxes().len(), 3);
    assert_eq!(scene.boxes()[0].node_id(), root.id());
    assert_eq!(scene.boxes()[1].node_id(), children[0].id());
    assert_eq!(scene.boxes()[2].node_id(), children[1].id());
}

#[test]
fn c1121_runtime_profile_rejects_mixed_auto_placement() {
    let (snapshot, _, _) = super::make_layout_snapshot(
        "display:grid;width:320px;height:130px;grid-template-columns:80px 100px 120px;grid-template-rows:50px 70px;column-gap:8px;row-gap:6px",
        &[
            "width:40px;height:20px;grid-column:1 / 2;grid-row:1 / 2",
            "width:50px;height:24px",
        ],
        false,
        None,
    );
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport {
            width_css_px: 320.0,
            height_css_px: 240.0,
            device_scale_factor: 1.0,
            environment_revision: EnvironmentRevision::INITIAL,
            media_environment: CssMediaEnvironment::MOBILE,
        },
        previous_snapshot: None,
        force_full: true,
        invalidation: None,
        previous_styles: None,
    };

    let calculation = compute_request_for_block_flex_grid_placement(&request).unwrap();
    let failure = calculation.layout.unwrap_err();
    assert_eq!(failure.code, "invalid_grid_placement");
    assert!(failure.node_id.is_some());
}
