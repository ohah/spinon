use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, IntrinsicSizeRevision, OwnerId,
};
use spinon_layout::{LayoutInputRevision, LayoutSourceRevision};
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
use spinon_style::compute_runtime_block_flex_grid_track_sizing_visibility_collapse_fixture_test_cascade_with_stylesheets;
use spinon_style::{
    ComputedBackgroundPaint, CssMediaEnvironment, CssViewport, StyloDocumentView,
    compute_runtime_flex_custom_properties_paint_cascade, compute_runtime_flex_paint_cascade,
};
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
use spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout;
use spinon_style_to_layout::compute_runtime_style_layout;

use crate::{CurrentLayoutInputs, StyleRenderError, build_runtime_render_snapshot};

const HTML: &str = "http://www.w3.org/1999/xhtml";

fn fixture(
    child_styles: &[&str],
    reverse_ids: bool,
) -> (HostDocument, HostNodeHandle, Vec<HostNodeHandle>) {
    fixture_with_root_style(
        "display:flex;box-sizing:border-box;width:301px;height:100px;flex-direction:row;align-items:flex-start;justify-content:flex-start;column-gap:11px;background-color:#123456",
        child_styles,
        reverse_ids,
    )
}

fn fixture_with_root_style(
    root_style: &str,
    child_styles: &[&str],
    reverse_ids: bool,
) -> (HostDocument, HostNodeHandle, Vec<HostNodeHandle>) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let mut children = child_styles
        .iter()
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    if reverse_ids {
        children.reverse();
    }
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(410).unwrap(), document.document_revision());
    create_element(&mut batch, root, root_style);
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Root,
        node: root,
        before: None,
    });
    for (handle, style) in children.iter().zip(child_styles) {
        create_element(&mut batch, *handle, style);
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: *handle,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    (document, root, children)
}

fn create_element(batch: &mut DocumentChangeBatch, node: HostNodeHandle, style: &str) {
    batch.push(DocumentOperation::CreateElement {
        node,
        namespace: HTML.to_owned(),
        local_name: "div".to_owned(),
    });
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, "style").unwrap(),
        value: style.into(),
    });
}

fn viewport() -> CssViewport {
    CssViewport {
        width_css_px: 301.0,
        height_css_px: 100.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    }
}

fn build_fixture(
    document: &HostDocument,
    root: HostNodeHandle,
) -> (
    spinon_style_to_layout::StyleLayoutOutput,
    CurrentLayoutInputs,
) {
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), root)
        .unwrap();
    let css_viewport = viewport();
    let styles =
        compute_runtime_flex_paint_cascade(&view, css_viewport, Default::default()).unwrap();
    let output = compute_runtime_style_layout(&snapshot, root, styles, css_viewport).unwrap();
    let current =
        CurrentLayoutInputs::for_host_document(&snapshot, Default::default(), css_viewport);
    (output, current)
}

#[test]
fn runtime_scene_matches_chromium_dom_order_geometry_and_paint() {
    let (document, root, children) = fixture(
        &[
            "display:block;box-sizing:border-box;width:51px;height:31px;background-color:#3366ff",
            "display:block;box-sizing:border-box;width:41px;height:31px;background-color:transparent",
            "display:none;width:23px;height:17px;background-color:#ff0000",
        ],
        true,
    );
    let snapshot = document.snapshot();
    let (output, current) = build_fixture(&document, root);
    let render = build_runtime_render_snapshot(&snapshot, root, &output, current).unwrap();

    assert_eq!(render.boxes().len(), 3);
    assert_eq!(render.boxes()[0].node_id(), root.id());
    assert_eq!(render.boxes()[1].node_id(), children[0].id());
    assert_eq!(render.boxes()[2].node_id(), children[1].id());
    assert_eq!(render.boxes()[0].frame_css_px().width(), 301.0);
    assert_eq!(render.boxes()[1].frame_css_px().width(), 51.0);
    assert_eq!(render.boxes()[2].frame_css_px().x(), 62.0);
    assert_eq!(
        render.boxes()[0].paint(),
        spinon_render::RuntimePaint::Opaque(spinon_render::OpaqueCssSrgb::new(18, 52, 86))
    );
    assert_eq!(
        render.boxes()[1].paint(),
        spinon_render::RuntimePaint::Opaque(spinon_render::OpaqueCssSrgb::new(51, 102, 255))
    );
    assert_eq!(render.boxes()[2].paint(), spinon_render::RuntimePaint::None);
    assert_eq!(render.boxes()[2].paint_order(), 2);
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn c114_runtime_scene_preserves_solid_border_geometry_and_paint() {
    let (document, root, _) = fixture_with_root_style(
        "box-sizing:border-box;display:grid;width:40px;height:184px;margin:0;padding:0;border:2px solid #f0c674;gap:0;grid-template-columns:repeat(4,min-content);grid-template-rows:repeat(3,60px);background-color:#101820",
        &[],
        false,
    );
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), root)
        .unwrap();
    let viewport = viewport();
    let output = compute_runtime_block_flex_grid_track_sizing_style_layout(
        &snapshot,
        &view,
        root,
        &[],
        viewport,
        Default::default(),
    )
    .unwrap();
    let current = CurrentLayoutInputs::for_host_document(&snapshot, Default::default(), viewport);
    let render = build_runtime_render_snapshot(&snapshot, root, &output, current).unwrap();
    let border = render.boxes()[0].border();

    assert_eq!(border.top().width_css_px(), 2.0);
    assert_eq!(border.right().width_css_px(), 2.0);
    assert_eq!(border.bottom().width_css_px(), 2.0);
    assert_eq!(border.left().width_css_px(), 2.0);
    assert_eq!(
        border.top().paint(),
        spinon_render::RuntimePaint::Opaque(spinon_render::OpaqueCssSrgb::new(240, 198, 116,))
    );
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn c114_visibility_hidden_keeps_layout_frame_but_omits_the_painted_box() {
    let (document, root, children) = fixture_with_root_style(
        "box-sizing:border-box;display:grid;width:100px;height:100px;grid-template-columns:100px;grid-template-rows:100px;background-color:#112233",
        &[
            "display:block;box-sizing:border-box;width:100px;height:100px;grid-column:1/2;grid-row:1/2;visibility:hidden;background-color:#e500a4",
            "display:block;box-sizing:border-box;width:100px;height:100px;grid-column:1/2;grid-row:1/2;visibility:visible;background-color:#2864dc",
        ],
        false,
    );
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), root)
        .unwrap();
    let viewport = viewport();
    let output = compute_runtime_block_flex_grid_track_sizing_style_layout(
        &snapshot,
        &view,
        root,
        &[],
        viewport,
        Default::default(),
    )
    .unwrap();
    let hidden_frame = output.layout.frames.get(&children[0].id()).unwrap();
    assert_eq!(
        [
            hidden_frame.x,
            hidden_frame.y,
            hidden_frame.width,
            hidden_frame.height,
        ],
        [0.0, 0.0, 100.0, 100.0]
    );
    let current = CurrentLayoutInputs::for_host_document(&snapshot, Default::default(), viewport);
    let render = build_runtime_render_snapshot(&snapshot, root, &output, current).unwrap();
    assert_eq!(render.boxes().len(), 2);
    assert_eq!(render.boxes()[0].node_id(), root.id());
    assert_eq!(render.boxes()[1].node_id(), children[1].id());
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn c114_visibility_collapse_keeps_grid_frame_but_omits_paint_and_hit_box() {
    let (document, root, children) = fixture_with_root_style(
        "box-sizing:border-box;display:grid;width:100px;height:100px;grid-template-columns:100px;grid-template-rows:100px;background-color:#112233",
        &[
            "display:block;box-sizing:border-box;width:100px;height:100px;grid-column:1/2;grid-row:1/2;visibility:collapse;background-color:#e500a4",
        ],
        false,
    );
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), root)
        .unwrap();
    let viewport = viewport();
    let computed_styles = compute_runtime_block_flex_grid_track_sizing_visibility_collapse_fixture_test_cascade_with_stylesheets(
        &view,
        &[],
        viewport,
        Default::default(),
    )
    .unwrap();
    let child_style = computed_styles
        .elements
        .iter()
        .find(|style| style.node_id == children[0].id())
        .unwrap();
    assert_eq!(
        child_style.properties.get("visibility").map(String::as_str),
        Some("collapse")
    );
    let output = compute_runtime_style_layout(&snapshot, root, computed_styles, viewport).unwrap();
    let frame = output.layout.frames.get(&children[0].id()).unwrap();
    assert_eq!(
        [frame.x, frame.y, frame.width, frame.height],
        [0.0, 0.0, 100.0, 100.0]
    );
    let current = CurrentLayoutInputs::for_host_document(&snapshot, Default::default(), viewport);
    let render = build_runtime_render_snapshot(&snapshot, root, &output, current).unwrap();
    assert_eq!(render.boxes().len(), 1);
    assert_eq!(render.boxes()[0].node_id(), root.id());
    assert_eq!(
        render
            .hit_test_css_point(50.0, 50.0)
            .map(|target| target.node_id()),
        Some(root.id())
    );
}

#[test]
fn c19_computed_color_profile_is_rejected_by_runtime_renderer() {
    let (document, root, _) = fixture(&["width:51px;height:31px"], false);
    let snapshot = document.snapshot();
    let (mut output, current) = build_fixture(&document, root);
    output.computed_styles.profile = spinon_style::ComputedStyleProfile::C19ComputedColorV1;

    assert!(matches!(
        build_runtime_render_snapshot(&snapshot, root, &output, current),
        Err(StyleRenderError::UnsupportedRuntimeProfile)
    ));
}

#[test]
fn runtime_scene_rejects_stale_document_style_or_environment_revisions() {
    let (document, root, _) = fixture(&["width:51px;height:31px;background-color:#3366ff"], false);
    let snapshot = document.snapshot();
    let (output, current) = build_fixture(&document, root);
    assert!(build_runtime_render_snapshot(&snapshot, root, &output, current).is_ok());

    let mut stale_style = current;
    stale_style.revision = LayoutInputRevision::new(
        stale_style.revision.source(),
        stale_style.revision.style().checked_next().unwrap(),
        stale_style.revision.environment(),
    );
    assert!(matches!(
        build_runtime_render_snapshot(&snapshot, root, &output, stale_style),
        Err(StyleRenderError::SnapshotMismatch {
            field: "StyleRevision"
        })
    ));

    let mut stale_source = current;
    stale_source.revision = LayoutInputRevision::new(
        LayoutSourceRevision::Tree(Default::default()),
        stale_source.revision.style(),
        stale_source.revision.environment(),
    );
    assert!(matches!(
        build_runtime_render_snapshot(&snapshot, root, &output, stale_source),
        Err(StyleRenderError::SnapshotMismatch {
            field: "CurrentLayoutSourceRevision"
        })
    ));

    let mut stale_environment = current;
    stale_environment.revision = LayoutInputRevision::new(
        stale_environment.revision.source(),
        stale_environment.revision.style(),
        stale_environment
            .revision
            .environment()
            .checked_next()
            .unwrap(),
    );
    assert!(matches!(
        build_runtime_render_snapshot(&snapshot, root, &output, stale_environment),
        Err(StyleRenderError::SnapshotMismatch {
            field: "EnvironmentRevision"
        })
    ));

    let stale_intrinsic_size = current
        .with_intrinsic_size_revision(IntrinsicSizeRevision::INITIAL.checked_next().unwrap());
    assert!(matches!(
        build_runtime_render_snapshot(&snapshot, root, &output, stale_intrinsic_size),
        Err(StyleRenderError::SnapshotMismatch {
            field: "LayoutIntrinsicSizeRevision"
        })
    ));
}

#[test]
fn runtime_scene_omits_hidden_nodes_and_allows_an_empty_scene() {
    let (document, root, _) = fixture_with_root_style(
        "display:none;width:301px;height:100px;background-color:#123456",
        &["display:none;width:51px;height:31px;background-color:#3366ff"],
        false,
    );
    let snapshot = document.snapshot();
    let (output, current) = build_fixture(&document, root);
    let render = build_runtime_render_snapshot(&snapshot, root, &output, current).unwrap();
    assert!(render.boxes().is_empty());
}

#[test]
fn runtime_scene_uses_typed_stylo_paint_not_css_string_reparsing() {
    let (document, root, _) = fixture(&["background-color:transparent"], false);
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), root)
        .unwrap();
    let styles = compute_runtime_flex_paint_cascade(&view, viewport(), Default::default()).unwrap();
    assert!(styles.elements.iter().all(|style| matches!(
        style.background_paint,
        Some(
            ComputedBackgroundPaint::Transparent
                | ComputedBackgroundPaint::Opaque(_)
                | ComputedBackgroundPaint::Translucent(_)
        )
    )));
}

#[test]
fn runtime_scene_sorts_flex_paint_order_per_parent_and_keeps_nested_subtrees() {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let item_a = document.reserve_node_handle().unwrap();
    let item_b = document.reserve_node_handle().unwrap();
    let nested_a = document.reserve_node_handle().unwrap();
    let nested_b = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(411).unwrap(), document.document_revision());
    for (node, parent, style) in [
        (
            root,
            HostParent::Root,
            "display:flex;box-sizing:border-box;width:120px;height:40px;flex-direction:row;align-items:flex-start;justify-content:flex-start;background-color:#101827",
        ),
        (
            item_a,
            HostParent::Node(root),
            "display:flex;box-sizing:border-box;width:60px;height:40px;order:2;flex-direction:row;align-items:flex-start;justify-content:flex-start;background-color:#243047",
        ),
        (
            item_b,
            HostParent::Node(root),
            "display:block;box-sizing:border-box;width:60px;height:40px;order:-1;background-color:#a855f7",
        ),
        (
            nested_a,
            HostParent::Node(item_a),
            "display:block;box-sizing:border-box;width:20px;height:20px;order:2;background-color:#e5484d",
        ),
        (
            nested_b,
            HostParent::Node(item_a),
            "display:block;box-sizing:border-box;width:20px;height:20px;order:-1;background-color:#28a745",
        ),
    ] {
        create_element(&mut batch, node, style);
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), root)
        .unwrap();
    let css_viewport = viewport();
    let styles =
        compute_runtime_flex_paint_cascade(&view, css_viewport, Default::default()).unwrap();
    let output = compute_runtime_style_layout(&snapshot, root, styles, css_viewport).unwrap();
    let current =
        CurrentLayoutInputs::for_host_document(&snapshot, Default::default(), css_viewport);
    let render = build_runtime_render_snapshot(&snapshot, root, &output, current).unwrap();

    assert_eq!(
        snapshot.children(root).unwrap().collect::<Vec<_>>(),
        [item_a, item_b],
        "렌더 순서를 만들 때 원본 HostDocument 자식 벡터는 재정렬되지 않습니다"
    );
    assert_eq!(
        snapshot.children(item_a).unwrap().collect::<Vec<_>>(),
        [nested_a, nested_b],
        "중첩 Flex item의 원본 순서도 보존됩니다"
    );
    assert_eq!(output.layout.frames[&item_a.id()].x, 60.0);
    assert_eq!(output.layout.frames[&item_b.id()].x, 0.0);
    assert_eq!(output.layout.frames[&nested_a.id()].x, 80.0);
    assert_eq!(output.layout.frames[&nested_b.id()].x, 60.0);
    assert_eq!(
        render
            .boxes()
            .iter()
            .map(|box_| box_.node_id())
            .collect::<Vec<_>>(),
        [
            root.id(),
            item_b.id(),
            item_a.id(),
            nested_b.id(),
            nested_a.id()
        ]
    );
    assert_eq!(
        render
            .boxes()
            .iter()
            .map(|box_| box_.paint_order())
            .collect::<Vec<_>>(),
        [0, 1, 2, 3, 4]
    );
}

#[test]
fn runtime_scene_ignores_order_for_non_flex_siblings() {
    let (document, root, children) = fixture_with_root_style(
        "display:block;box-sizing:border-box;width:301px;height:100px;background-color:#123456",
        &[
            "display:block;box-sizing:border-box;width:51px;height:31px;order:2;background-color:#3366ff",
            "display:block;box-sizing:border-box;width:41px;height:31px;order:-1;background-color:#12b981",
        ],
        false,
    );
    let snapshot = document.snapshot();
    let (output, current) = build_fixture(&document, root);
    let render = build_runtime_render_snapshot(&snapshot, root, &output, current).unwrap();
    assert_eq!(render.boxes()[1].node_id(), children[0].id());
    assert_eq!(render.boxes()[2].node_id(), children[1].id());
}

#[test]
fn runtime_scene_rejects_missing_or_invalid_flex_item_order() {
    let (document, root, children) = fixture_with_root_style(
        "display:flex;box-sizing:border-box;width:301px;height:100px;background-color:#123456",
        &[
            "display:block;box-sizing:border-box;width:51px;height:31px;order:2;background-color:#3366ff",
            "display:block;box-sizing:border-box;width:41px;height:31px;order:-1;background-color:#12b981",
        ],
        false,
    );
    let snapshot = document.snapshot();
    let (output, current) = build_fixture(&document, root);

    let mut missing_order = output.clone();
    let mut elements = missing_order.computed_styles.elements.to_vec();
    let child = elements
        .iter_mut()
        .find(|style| style.node_id == children[0].id())
        .unwrap();
    child.properties.remove("order");
    missing_order.computed_styles.elements = elements.into();
    assert!(matches!(
        build_runtime_render_snapshot(&snapshot, root, &missing_order, current),
        Err(StyleRenderError::MissingComputedProperty {
            node,
            property: "order"
        }) if node == children[0].id()
    ));

    let mut invalid_order = output;
    let mut elements = invalid_order.computed_styles.elements.to_vec();
    let child = elements
        .iter_mut()
        .find(|style| style.node_id == children[1].id())
        .unwrap();
    child
        .properties
        .insert("order".to_owned(), "not-an-integer".to_owned());
    invalid_order.computed_styles.elements = elements.into();
    assert!(matches!(
        build_runtime_render_snapshot(&snapshot, root, &invalid_order, current),
        Err(StyleRenderError::UnsupportedComputedValue {
            node,
            property: "order",
            value
        }) if node == children[1].id() && value == "not-an-integer"
    ));
}

#[test]
fn runtime_scene_accepts_custom_properties_paint_profile() {
    let (document, root, children) = fixture_with_root_style(
        "display:flex;box-sizing:border-box;width:100vw;height:100vh;--panel:#123456;background-color:var(--panel);--space:11px;gap:var(--space)",
        &[
            "width:20px;height:10px;--surface:#3366ff;background-color:var(--surface)",
            "width:30px;height:10px;--detail:#ffcc33;background-color:var(--detail)",
        ],
        false,
    );
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), root)
        .unwrap();
    let css_viewport = CssViewport {
        width_css_px: 390.0,
        height_css_px: 844.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    };
    let styles = compute_runtime_flex_custom_properties_paint_cascade(
        &view,
        css_viewport,
        Default::default(),
    )
    .unwrap();
    let output = compute_runtime_style_layout(&snapshot, root, styles, css_viewport).unwrap();
    let current =
        CurrentLayoutInputs::for_host_document(&snapshot, Default::default(), css_viewport);
    let render = build_runtime_render_snapshot(&snapshot, root, &output, current).unwrap();

    assert_eq!(render.boxes().len(), 3);
    assert_eq!(render.boxes()[0].node_id(), root.id());
    assert_eq!(render.boxes()[1].node_id(), children[0].id());
    assert_eq!(render.boxes()[2].node_id(), children[1].id());

    let reference_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/fixtures/css/references/c05-runtime-custom-properties-v1.json"
    );
    let reference: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(reference_path).unwrap()).unwrap();
    let reference_root = &reference["observations"]["initial"]["gap-parent"]["rect"];
    let actual_root = render.boxes()[0].frame_css_px();
    for (render_box, node) in render
        .boxes()
        .iter()
        .zip(["gap-parent", "gap-first", "gap-second"])
    {
        let actual = render_box.frame_css_px();
        let expected = &reference["observations"]["initial"][node]["rect"];
        let actual_values = [
            f64::from(actual.x() - actual_root.x()),
            f64::from(actual.y() - actual_root.y()),
            f64::from(actual.width()),
            f64::from(actual.height()),
        ];
        let expected_values = [
            expected["x"].as_f64().unwrap() - reference_root["x"].as_f64().unwrap(),
            expected["y"].as_f64().unwrap() - reference_root["y"].as_f64().unwrap(),
            expected["width"].as_f64().unwrap(),
            expected["height"].as_f64().unwrap(),
        ];
        for (actual, expected) in actual_values.into_iter().zip(expected_values) {
            assert!(
                (actual - expected).abs() <= 0.5,
                "{node} geometry differs from pinned Chromium: actual={actual}, expected={expected}"
            );
        }
    }
    assert_eq!(
        render.boxes()[0].paint(),
        spinon_render::RuntimePaint::Opaque(spinon_render::OpaqueCssSrgb::new(18, 52, 86))
    );
    assert_eq!(
        render.boxes()[1].paint(),
        spinon_render::RuntimePaint::Opaque(spinon_render::OpaqueCssSrgb::new(51, 102, 255))
    );
    assert_eq!(
        render.boxes()[2].paint(),
        spinon_render::RuntimePaint::Opaque(spinon_render::OpaqueCssSrgb::new(255, 204, 51))
    );
}

#[path = "runtime_positioned_flex.rs"]
mod positioned_flex;
