use super::*;
use crate::session::ua_cascade::calculation::compute_request_for_block_flex_grid_placement;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
use crate::session::ua_cascade::calculation::compute_request_for_block_flex_grid_track_sizing;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn c114_min_content_text_contributes_measured_intrinsic_sizes_to_grid_layout() {
    let (snapshot, root, item) = c114_text_grid_snapshot();
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport {
            width_css_px: 120.0,
            height_css_px: 40.0,
            device_scale_factor: 1.0,
            environment_revision: EnvironmentRevision::INITIAL,
            media_environment: CssMediaEnvironment::MOBILE,
        },
        previous_snapshot: None,
        force_full: true,
        invalidation: None,
        previous_styles: None,
    };

    let calculation = compute_request_for_block_flex_grid_track_sizing(&request).unwrap();
    let layout = calculation.layout.unwrap();
    let frame = |node: HostNodeHandle| {
        layout
            .frames
            .iter()
            .find(|frame| frame.node_id == node.id().get())
            .unwrap()
    };
    assert_eq!((frame(root).width, frame(root).height), (120.0, 40.0));
    assert_eq!((frame(item).width, frame(item).height), (70.0, 20.0));

    let diagnostics = layout.grid_track_sizing_diagnostics.unwrap();
    let input = diagnostics
        .item_inputs
        .iter()
        .find(|input| input.node == item.id())
        .unwrap();
    let spinon_layout::LayoutGridItemIntrinsicSource::Text { sizes } = &input.source else {
        panic!("C11.4 text fixture must produce a text intrinsic input");
    };
    assert_eq!((sizes.min_content(), sizes.max_content()), (70.0, 100.0));
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn c114_text_grid_snapshot() -> (Arc<HostDocumentSnapshot>, HostNodeHandle, HostNodeHandle) {
    const HTML: &str = "http://www.w3.org/1999/xhtml";
    const ROOT_STYLE: &str = "box-sizing:border-box;display:grid;width:120px;height:40px;margin:0;padding:0;border:0;gap:0;align-items:start;grid-template-columns:min-content;grid-template-rows:auto;background-color:#101820";
    const ITEM_STYLE: &str = "box-sizing:border-box;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d";

    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let item = document.reserve_node_handle().unwrap();
    let text = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(11_414).unwrap(), document.document_revision());
    for (node, id, parent, style) in [
        (root, "c114-text-grid", HostParent::Root, ROOT_STYLE),
        (
            item,
            "c114-track-text-item",
            HostParent::Node(root),
            ITEM_STYLE,
        ),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "id").unwrap(),
            value: id.to_owned().into(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "style").unwrap(),
            value: style.to_owned().into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    batch.push(DocumentOperation::CreateText {
        node: text,
        data: "XX XXXXXXX".into(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(item),
        node: text,
        before: None,
    });
    document.commit(batch).unwrap();
    (Arc::new(document.snapshot()), root, item)
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn c114_min_content_image_contributes_decoded_natural_size_to_grid_layout() {
    let (snapshot, root, item) =
        c114_image_grid_snapshot("https://spinon.invalid/c14-1-png-basic.png");
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport {
            width_css_px: 100.0,
            height_css_px: 40.0,
            device_scale_factor: 1.0,
            environment_revision: EnvironmentRevision::INITIAL,
            media_environment: CssMediaEnvironment::MOBILE,
        },
        previous_snapshot: None,
        force_full: true,
        invalidation: None,
        previous_styles: None,
    };

    let calculation = compute_request_for_block_flex_grid_track_sizing(&request).unwrap();
    let layout = calculation.layout.unwrap();
    let frame = |node: HostNodeHandle| {
        layout
            .frames
            .iter()
            .find(|frame| frame.node_id == node.id().get())
            .unwrap()
    };
    assert_eq!((frame(root).width, frame(root).height), (100.0, 40.0));
    assert_eq!((frame(item).width, frame(item).height), (24.0, 16.0));

    let diagnostics = layout.grid_track_sizing_diagnostics.unwrap();
    let input = diagnostics
        .item_inputs
        .iter()
        .find(|input| input.node == item.id())
        .unwrap();
    let spinon_layout::LayoutGridItemIntrinsicSource::Replaced { natural } = &input.source else {
        panic!("C11.4 image fixture must preserve a replaced intrinsic input");
    };
    assert_eq!((natural.width, natural.height), (Some(24.0), Some(16.0)));
    assert_eq!(input.grid_container, root.id());
    assert_eq!(input.revision, diagnostics.revision);

    let column_pass = diagnostics.traces[0]
        .passes
        .iter()
        .find(|pass| pass.axis == spinon_layout::LayoutGridTrackSizingAxis::Columns)
        .unwrap();
    assert!(
        column_pass
            .contributions
            .iter()
            .any(|contribution| contribution.item == item.id())
    );
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn c114_image_fixture_rejects_unpinned_image_source_without_zero_fallback() {
    let (snapshot, _, _) = c114_image_grid_snapshot("https://spinon.invalid/unpinned.png");
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport {
            width_css_px: 100.0,
            height_css_px: 40.0,
            device_scale_factor: 1.0,
            environment_revision: EnvironmentRevision::INITIAL,
            media_environment: CssMediaEnvironment::MOBILE,
        },
        previous_snapshot: None,
        force_full: true,
        invalidation: None,
        previous_styles: None,
    };

    let calculation = compute_request_for_block_flex_grid_track_sizing(&request).unwrap();
    assert_eq!(
        calculation.layout.unwrap_err().code,
        "unsupported_c114_image_case"
    );
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn c114_image_grid_snapshot(
    source: &str,
) -> (Arc<HostDocumentSnapshot>, HostNodeHandle, HostNodeHandle) {
    const HTML: &str = "http://www.w3.org/1999/xhtml";
    const ROOT_STYLE: &str = "box-sizing:border-box;display:grid;width:100px;height:40px;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:auto;grid-template-rows:auto;background-color:#101820";
    const ITEM_STYLE: &str = "box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;background-color:#e5484d";

    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let item = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(11_414).unwrap(), document.document_revision());
    for (node, tag, id, parent, style) in [
        (
            root,
            "div",
            "c114-auto-image-grid",
            HostParent::Root,
            ROOT_STYLE,
        ),
        (
            item,
            "img",
            "c114-auto-image-item",
            HostParent::Node(root),
            ITEM_STYLE,
        ),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: tag.to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "id").unwrap(),
            value: id.to_owned().into(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "style").unwrap(),
            value: style.to_owned().into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    batch.push(DocumentOperation::SetAttribute {
        node: item,
        name: AttributeName::new(None, "src").unwrap(),
        value: source.to_owned().into(),
    });
    document.commit(batch).unwrap();
    (Arc::new(document.snapshot()), root, item)
}

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
