use super::*;
use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind};
use std::collections::BTreeMap;

const SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-direct-text-aspect-ratio-values-precomparison.html"
);
const MIN_MAX_SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-direct-text-aspect-ratio-min-max-precomparison.html"
);

fn element_handles_by_id(snapshot: &HostDocumentSnapshot) -> BTreeMap<String, HostNodeHandle> {
    let mut handles = BTreeMap::new();
    let mut pending = snapshot.root_children().collect::<Vec<_>>();
    while let Some(handle) = pending.pop() {
        let Some(node) = snapshot.node(handle) else {
            continue;
        };
        if let HostNodeKind::Element(element) = node.kind()
            && let Some((_, value)) = element
                .attributes()
                .iter()
                .find(|(name, _)| name.namespace().is_none() && name.local_name() == "id")
        {
            handles.insert(value.to_string_lossy(), handle);
        }
        if let Some(children) = snapshot.children(handle) {
            pending.extend(children);
        }
    }
    handles
}

fn child_with_class(
    snapshot: &HostDocumentSnapshot,
    parent: HostNodeHandle,
    class_name: &str,
) -> HostNodeHandle {
    snapshot
        .children(parent)
        .expect("fixture element should have a child list")
        .find(|handle| {
            let Some(node) = snapshot.node(*handle) else {
                return false;
            };
            let HostNodeKind::Element(element) = node.kind() else {
                return false;
            };
            element.attributes().iter().any(|(name, value)| {
                name.namespace().is_none()
                    && name.local_name() == "class"
                    && value
                        .to_string_lossy()
                        .split_ascii_whitespace()
                        .any(|class| class == class_name)
            })
        })
        .unwrap_or_else(|| {
            panic!(
                "fixture child class .{class_name} missing below {:?}",
                parent.id()
            )
        })
}

#[test]
fn c11_1_default_root_html_runtime_projects_ratio_value_matrix() {
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(SOURCE)
        .expect("ratio value fixture should initialize the default Root HTML runtime");
    let canvas = session.resolve_root_html_canvas(
        1080.0,
        2209.0,
        3.0,
        spinon_style::CssMediaEnvironment::MOBILE,
        Duration::from_secs(5),
    );
    assert!(
        canvas.is_ok(),
        "canvas={canvas:?}; cascade={:?}; layout={:?}",
        session.ua_cascade_snapshot(),
        session.layout_snapshot()
    );
    let layout = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "{layout:?}; cascade={:?}",
        session.ua_cascade_snapshot()
    );
    let completed = layout
        .completed
        .expect("the ratio value fixture should publish its default layout");

    let parsed = HostDocumentBridge::new_with_root_html(SOURCE)
        .expect("the same source should produce a parsed HostDocument snapshot");
    let parsed = parsed.snapshot();
    let handles = element_handles_by_id(&parsed);
    let frame = |handle: HostNodeHandle| {
        let node_id = handle.id().get();
        let frame = completed
            .frames
            .iter()
            .find(|frame| frame.node_id == node_id)
            .unwrap_or_else(|| panic!("layout frame missing for node {node_id}"));
        [frame.x, frame.y, frame.width, frame.height]
    };
    let assert_near = |actual: f32, expected: f32, label: &str| {
        assert!(
            (actual - expected).abs() <= 0.5,
            "{label}: expected {expected}, got {actual}"
        );
    };

    let cases = [
        ("ratio-half", 120.0, Some(60.0), 120.0),
        ("ratio-four-thirds", 45.0, Some(60.0), 45.0),
        ("ratio-three-halves", 40.0, Some(60.0), 40.0),
        ("ratio-five-halves", 24.0, Some(60.0), 24.0),
        ("ratio-three-halves-pair", 40.0, Some(60.0), 40.0),
        ("ratio-ten", 10.0, Some(60.0), 10.0),
        ("ratio-auto-control", 10.0, Some(60.0), 10.0),
        ("height-half", 80.0, Some(40.0), 80.0),
        ("height-four-thirds", 30.0, Some(40.0), 30.0),
        ("height-three-halves", 30.0, Some(45.0), 30.0),
        ("height-five-halves", 30.0, Some(75.0), 30.0),
        ("width-two", 30.0, Some(60.0), 30.0),
        ("height-two", 30.0, Some(60.0), 30.0),
        ("height-auto-control", 30.0, None, 30.0),
    ];
    for (scene_id, row_height, target_width, target_height) in cases {
        let section = handles
            .get(scene_id)
            .copied()
            .unwrap_or_else(|| panic!("fixture section #{scene_id} is missing"));
        let grid = child_with_class(&parsed, section, "grid");
        let target_class = if scene_id.starts_with("height-") {
            "target-height"
        } else {
            "target"
        };
        let target = child_with_class(&parsed, grid, target_class);
        let grid_frame = frame(grid);
        let target_frame = frame(target);
        assert_near(grid_frame[2], 70.0, &format!("#{scene_id} Grid width"));
        assert_near(grid_frame[3], row_height, &format!("#{scene_id} auto row"));
        assert_near(
            target_frame[0] - grid_frame[0],
            10.0,
            &format!("#{scene_id} target x relative to Grid"),
        );
        assert_near(
            target_frame[1] - grid_frame[1],
            0.0,
            &format!("#{scene_id} target y relative to Grid"),
        );
        assert_near(
            target_frame[3],
            target_height,
            &format!("#{scene_id} target height"),
        );
        if let Some(target_width) = target_width {
            assert_near(
                target_frame[2],
                target_width,
                &format!("#{scene_id} target width"),
            );
        } else {
            assert!(
                target_frame[2] > 0.0 && target_frame[2] < 60.0,
                "platform monospace intrinsic width should remain unconstrained: {target_frame:?}"
            );
        }
        if scene_id == "height-five-halves" {
            assert_near(
                target_frame[0] + target_frame[2] - grid_frame[0] - grid_frame[2],
                15.0,
                "height-locked 5/2 visible overflow beyond the Grid",
            );
        }
    }
}

#[test]
fn c11_1_default_root_html_runtime_projects_ratio_min_max_matrix() {
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(MIN_MAX_SOURCE)
        .expect("ratio min/max fixture should initialize the default Root HTML runtime");
    let canvas = session.resolve_root_html_canvas(
        1080.0,
        2209.0,
        3.0,
        spinon_style::CssMediaEnvironment::MOBILE,
        Duration::from_secs(5),
    );
    assert!(
        canvas.is_ok(),
        "canvas={canvas:?}; cascade={:?}; layout={:?}",
        session.ua_cascade_snapshot(),
        session.layout_snapshot()
    );
    let layout = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "{layout:?}; cascade={:?}",
        session.ua_cascade_snapshot()
    );
    let completed = layout
        .completed
        .expect("the ratio min/max fixture should publish its default layout");

    let parsed = HostDocumentBridge::new_with_root_html(MIN_MAX_SOURCE)
        .expect("the same source should produce a parsed HostDocument snapshot");
    let parsed = parsed.snapshot();
    let handles = element_handles_by_id(&parsed);
    let frame = |handle: HostNodeHandle| {
        let node_id = handle.id().get();
        let frame = completed
            .frames
            .iter()
            .find(|frame| frame.node_id == node_id)
            .unwrap_or_else(|| panic!("layout frame missing for node {node_id}"));
        [frame.x, frame.y, frame.width, frame.height]
    };
    let assert_near = |actual: f32, expected: f32, label: &str| {
        assert!(
            (actual - expected).abs() <= 0.5,
            "{label}: expected {expected}, got {actual}"
        );
    };

    let cases = [
        ("width-ratio-baseline", 30.0, Some(60.0), 30.0),
        ("width-ratio-min-height", 40.0, Some(60.0), 40.0),
        ("width-ratio-max-height", 20.0, Some(60.0), 20.0),
        ("width-ratio-minmax-height", 40.0, Some(60.0), 40.0),
        ("width-control-min-height", 40.0, Some(60.0), 40.0),
        ("height-ratio-baseline", 30.0, Some(60.0), 30.0),
        ("height-ratio-min-width", 30.0, Some(70.0), 30.0),
        ("height-ratio-max-width", 30.0, Some(50.0), 30.0),
        ("height-ratio-minmax-width", 30.0, Some(70.0), 30.0),
        ("height-control-max-width", 30.0, None, 30.0),
    ];
    let mut expected_y = 0.0;
    for (scene_id, row_height, target_width, target_height) in cases {
        let grid = handles
            .get(scene_id)
            .copied()
            .unwrap_or_else(|| panic!("fixture Grid #{scene_id} is missing"));
        let target_id = format!("{scene_id}-target");
        let target = handles
            .get(&target_id)
            .copied()
            .unwrap_or_else(|| panic!("fixture target #{target_id} is missing"));
        let grid_frame = frame(grid);
        let target_frame = frame(target);
        assert_near(grid_frame[0], 0.0, &format!("#{scene_id} Grid x"));
        assert_near(grid_frame[1], expected_y, &format!("#{scene_id} Grid y"));
        assert_near(grid_frame[2], 70.0, &format!("#{scene_id} Grid width"));
        assert_near(grid_frame[3], row_height, &format!("#{scene_id} auto row"));
        assert_near(
            target_frame[0] - grid_frame[0],
            10.0,
            &format!("#{scene_id} target x relative to Grid"),
        );
        assert_near(
            target_frame[1] - grid_frame[1],
            0.0,
            &format!("#{scene_id} target y relative to Grid"),
        );
        assert_near(
            target_frame[3],
            target_height,
            &format!("#{scene_id} target height"),
        );
        if let Some(target_width) = target_width {
            assert_near(
                target_frame[2],
                target_width,
                &format!("#{scene_id} target width"),
            );
        } else {
            assert!(
                target_frame[2] > 0.0 && target_frame[2] <= 50.0,
                "system monospace intrinsic width should remain bounded by max-width: {target_frame:?}"
            );
        }
        expected_y += row_height;
    }
}
