use super::*;
use serde_json::Value;
use spinon_core::{AttributeName, HostNodeKind};
use std::collections::BTreeMap;

const SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-tracks-runtime.html"
);
const EXPECTED: &str = include_str!(
    "../../../../../spec/internal/evidence/c11-root-html-intrinsic-tracks-precomparison-2026-11-05/expected.json"
);

fn run(source: &str) -> (RuntimeSession, crate::session::RuntimeLayoutSnapshot) {
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(source)
        .expect("parsed Root HTML should initialize the default runtime");
    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(390.0, 844.0, 1.0, media)
        .expect("root metadata should configure the CSS environment");
    let layout = session.wait_for_layout_snapshot(Duration::from_secs(5));
    (session, layout)
}

fn grid_and_item_nodes_by_scene_id(
    session: &RuntimeSession,
) -> BTreeMap<String, (u64, u64, Vec<u64>)> {
    let document = session
        .snapshot_document_for_runtime_fixture()
        .expect("parsed Root HTML document should be snapshotable");
    let id_attribute = AttributeName::new(None, "id").unwrap();
    let mut pending = document.root_children().collect::<Vec<_>>();
    let mut scenes = BTreeMap::new();
    while let Some(handle) = pending.pop() {
        let node = document
            .node(handle)
            .expect("document handle should resolve");
        if let HostNodeKind::Element(element) = node.kind()
            && let Some(id) = element.attribute(&id_attribute)
        {
            let id = id.to_string_lossy();
            let children = document
                .children(handle)
                .expect("scene children should resolve")
                .collect::<Vec<_>>();
            assert_eq!(children.len(), 1, "scene {id} should have one Grid root");
            let grid = children[0];
            let items = document
                .children(grid)
                .expect("Grid item children should resolve")
                .collect::<Vec<_>>()
                .into_iter()
                .map(|item| item.id().get())
                .collect::<Vec<_>>();
            assert!(
                scenes
                    .insert(id.clone(), (handle.id().get(), grid.id().get(), items))
                    .is_none(),
                "duplicate scene id {id}"
            );
        }
        pending.extend(
            document
                .children(handle)
                .expect("document children should resolve"),
        );
    }
    scenes
}

fn frame_for_node(
    completed: &crate::session::RuntimeLayoutCompleted,
    node_id: u64,
) -> Option<&crate::session::RuntimeLayoutFrame> {
    completed
        .frames
        .iter()
        .find(|frame| frame.node_id == node_id)
}

fn expected_frame(value: &Value) -> [f32; 4] {
    let values = value.as_array().expect("expected frame should be an array");
    assert_eq!(values.len(), 4);
    std::array::from_fn(|index| values[index].as_f64().unwrap() as f32)
}

fn assert_frame_near(actual: [f32; 4], expected: [f32; 4], context: &str) {
    for (component, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert!(
            actual.is_finite() && (actual - expected).abs() <= 0.5,
            "{context} component {component}: actual={actual:?}, expected={expected:?} ±0.5 CSS px"
        );
    }
}

#[test]
fn parsed_root_html_intrinsic_grid_tracks_match_all_reference_frames() {
    let expected: Value = serde_json::from_str(EXPECTED)
        .expect("intrinsic Grid expected values should be valid JSON");
    let expected_scenes = expected["cases"]
        .as_array()
        .expect("expected intrinsic Grid scenes should exist")
        .iter()
        .filter(|scene| scene["runtimeSupported"] != false)
        .collect::<Vec<_>>();
    assert_eq!(expected_scenes.len(), 17);

    let (session, layout) = run(SOURCE);
    let nodes = grid_and_item_nodes_by_scene_id(&session);
    assert_eq!(nodes.len(), expected_scenes.len());
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout error={:?}; node map={:?}; cascade state={:?}, error={:?}",
        layout.error,
        nodes,
        session.ua_cascade_snapshot().state,
        session.ua_cascade_snapshot().error
    );
    let completed = layout
        .completed
        .as_ref()
        .expect("all supported intrinsic Grid scenes should complete atomically");
    let render_scene = completed
        .render_snapshot
        .as_ref()
        .expect("default Root HTML runtime should publish its render snapshot");
    let cascade = session.ua_cascade_snapshot();
    let cascade_completed = cascade
        .completed
        .as_ref()
        .expect("computed styles should complete for the same document revision");
    assert_eq!(completed.key, cascade_completed.key);

    let mut checked_frames = 0;
    for expected_scene in expected_scenes {
        let scene_id = expected_scene["id"]
            .as_str()
            .expect("scene id should exist");
        let (scene_node, grid_node, item_nodes) = &nodes[scene_id];
        assert!(!item_nodes.is_empty(), "{scene_id} should have a Grid item");
        let grid = frame_for_node(completed, *grid_node)
            .unwrap_or_else(|| panic!("{scene_id}: Grid frame is missing"));
        let item = frame_for_node(completed, item_nodes[0])
            .unwrap_or_else(|| panic!("{scene_id}: visible Grid item frame is missing"));
        assert_frame_near(
            [0.0, 0.0, item.width, item.height],
            expected_frame(&expected_scene["itemFrameInGridPx"]),
            &format!("{scene_id} item size"),
        );
        assert_frame_near(
            [item.x - grid.x, item.y - grid.y, item.width, item.height],
            expected_frame(&expected_scene["itemFrameInGridPx"]),
            &format!("{scene_id} item frame relative to Grid"),
        );
        checked_frames += 1;

        if let Some(expected_grid_frame) = expected_scene.get("gridFrameInScenePx") {
            let scene = frame_for_node(completed, *scene_node)
                .unwrap_or_else(|| panic!("{scene_id}: scene frame is missing"));
            assert_frame_near(
                [grid.x - scene.x, grid.y - scene.y, grid.width, grid.height],
                expected_frame(expected_grid_frame),
                &format!("{scene_id} Grid frame"),
            );
        }

        assert!(
            render_scene
                .boxes()
                .iter()
                .any(|render_box| render_box.node_id().get() == item_nodes[0]),
            "{scene_id}: visible intrinsic Grid item should be included in the published paint scene"
        );

        if scene_id == "hidden-item" {
            assert_eq!(item_nodes.len(), 2);
            assert!(
                frame_for_node(completed, item_nodes[1]).is_none(),
                "display:none intrinsic content must not create a layout frame"
            );
            assert!(
                !render_scene
                    .boxes()
                    .iter()
                    .any(|render_box| render_box.node_id().get() == item_nodes[1]),
                "display:none intrinsic content must not contribute paint"
            );
        }
    }
    assert_eq!(checked_frames, 17);
}

#[test]
fn parsed_root_html_intrinsic_grid_rejects_implicit_tracks_without_partial_publish() {
    let valid = "#min-content .grid { grid-template-columns: min-content; }";
    let invalid = "#min-content .grid { grid-template-columns: min-content; grid-auto-rows: 4px; align-content: normal; justify-content: normal; }";
    let source = SOURCE.replacen(valid, invalid, 1);
    assert_ne!(
        source, SOURCE,
        "the rejection mutation should match the fixture"
    );

    let (session, layout) = run(&source);
    assert_eq!(layout.state, crate::session::RuntimeLayoutState::Failed);
    let failure = layout
        .error
        .as_ref()
        .expect("intrinsic tracks with implicit rows should report a bounded diagnostic");
    assert_eq!(failure.code, "unsupported_computed_value");
    assert_eq!(failure.property.as_deref(), Some("grid-auto-rows"));
    assert!(
        layout.completed.is_none(),
        "an unsupported intrinsic composition must not publish partial layout frames or paint"
    );

    let cascade = session.ua_cascade_snapshot();
    assert_eq!(cascade.state, crate::session::RuntimeUaCascadeState::Ready);
    assert!(cascade.error.is_none());
}

#[test]
fn parsed_root_html_intrinsic_grid_rejects_axes_gaps_and_box_edges() {
    let valid = "#min-content .grid { grid-template-columns: min-content; }";
    let cases = [
        (
            "#min-content .grid { grid-template-columns: min-content; height: auto; }",
            "height",
            "layout_failed",
        ),
        (
            "#min-content .grid { grid-template-columns: min-content; width: auto; }",
            "width",
            "layout_failed",
        ),
        (
            "#min-content .grid { grid-template-columns: min-content; width: 50%; }",
            "width",
            "layout_failed",
        ),
        (
            "#min-content .grid { grid-template-columns: min-content; width: 0; }",
            "width",
            "layout_failed",
        ),
        (
            "#min-content .grid { grid-template-columns: min-content; height: 0; }",
            "height",
            "layout_failed",
        ),
        (
            "#min-content .grid { grid-template-columns: min-content; gap: 1px; }",
            "row-gap",
            "layout_failed",
        ),
        (
            "#min-content .grid { grid-template-columns: min-content; margin: 1px; }",
            "margin",
            "unsupported_computed_value",
        ),
        (
            "#min-content .grid { grid-template-columns: min-content; padding: 1px; }",
            "padding",
            "unsupported_computed_value",
        ),
        (
            "#min-content .grid { grid-template-columns: min-content; border-width: 1px; border-style: solid; }",
            "border",
            "unsupported_computed_value",
        ),
    ];

    for (invalid, property, code) in cases {
        let source = SOURCE.replacen(valid, invalid, 1);
        assert_ne!(
            source, SOURCE,
            "the {property} mutation should match the fixture"
        );

        let (session, layout) = run(&source);
        assert_eq!(layout.state, crate::session::RuntimeLayoutState::Failed);
        let failure = layout
            .error
            .as_ref()
            .unwrap_or_else(|| panic!("unsupported {property} should report a diagnostic"));
        assert_eq!(failure.code, code, "{property}: {failure:?}");
        assert!(failure.property.as_deref().is_some_and(|diagnostic| {
            diagnostic.contains(property)
                && (code == "unsupported_computed_value" || diagnostic.contains("Grid container"))
        }));
        assert!(
            layout.completed.is_none(),
            "unsupported {property} must not publish partial frames or paint"
        );
        assert_eq!(
            session.ua_cascade_snapshot().state,
            crate::session::RuntimeUaCascadeState::Ready
        );
    }
}

#[test]
fn parsed_root_html_intrinsic_prefix_does_not_hide_later_track_overflow() {
    let valid = "#min-content .grid { grid-template-columns: min-content; }";
    let invalid =
        "#min-content .grid { grid-template-columns: min-content 100% 3e38px; width: 3e38px; }";
    let source = SOURCE.replacen(valid, invalid, 1);
    assert_ne!(
        source, SOURCE,
        "the overflow mutation should match the fixture"
    );

    let (session, layout) = run(&source);
    assert_eq!(layout.state, crate::session::RuntimeLayoutState::Failed);
    let failure = layout
        .error
        .as_ref()
        .expect("aggregate explicit track overflow should be diagnosed");
    assert_eq!(failure.code, "unsupported_computed_value");
    assert_eq!(failure.property.as_deref(), Some("grid-template-columns"));
    assert!(
        layout.completed.is_none(),
        "an aggregate track overflow must not publish partial layout frames or paint"
    );

    assert_eq!(
        session.ua_cascade_snapshot().state,
        crate::session::RuntimeUaCascadeState::Ready
    );
}
