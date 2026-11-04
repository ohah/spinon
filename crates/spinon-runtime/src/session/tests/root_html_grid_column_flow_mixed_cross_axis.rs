use super::*;
use serde_json::Value;
use spinon_core::{AttributeName, HostNodeKind};

const SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-column-flow-mixed-cross-axis-runtime.html"
);
const IMPLICIT_BOUNDARY_SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-column-flow-mixed-cross-axis-implicit-boundary-runtime.html"
);
const EXPECTED: &str = include_str!(
    "../../../../../spec/internal/evidence/c11-root-html-grid-column-flow-mixed-precomparison-2026-11-04/expected.json"
);

fn scene_source(scene_id: &str) -> String {
    let start = format!("<!-- scene:{scene_id}:start -->");
    let end = format!("<!-- scene:{scene_id}:end -->");
    let fragment = SOURCE
        .split_once(&start)
        .unwrap_or_else(|| panic!("fixture is missing {start}"))
        .1
        .split_once(&end)
        .unwrap_or_else(|| panic!("fixture is missing {end}"))
        .0;
    format!(
        "<!doctype html><html lang=\"ko\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"></head><body style=\"margin:0;padding:0;background-color:#ffffff\">{fragment}</body></html>"
    )
}

fn run(source: &str) -> (RuntimeSession, crate::session::RuntimeLayoutSnapshot) {
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(source)
        .expect("parsed Root HTML should initialize");
    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(980.0, 1_000.0, 1.0, media)
        .expect("Root HTML metadata should configure the CSS environment");
    let layout = session.wait_for_layout_snapshot(Duration::from_secs(5));
    if layout
        .error
        .as_ref()
        .is_some_and(|error| error.code == "cascade_failed")
    {
        panic!(
            "Root HTML cascade failed: {:?}",
            session.ua_cascade_snapshot()
        );
    }
    (session, layout)
}

fn node_ids_by_html_id(session: &RuntimeSession) -> std::collections::BTreeMap<String, u64> {
    let document = session
        .snapshot_document_for_runtime_fixture()
        .expect("parsed Root HTML document should be snapshotable");
    let id_attribute = AttributeName::new(None, "id").unwrap();
    let mut pending = document.root_children().collect::<Vec<_>>();
    let mut node_ids = std::collections::BTreeMap::new();
    while let Some(handle) = pending.pop() {
        let node = document
            .node(handle)
            .expect("document handle should resolve");
        if let HostNodeKind::Element(element) = node.kind()
            && let Some(id) = element.attribute(&id_attribute)
        {
            let id = id.to_string_lossy();
            assert!(
                node_ids.insert(id.clone(), handle.id().get()).is_none(),
                "duplicate id {id}"
            );
        }
        pending.extend(
            document
                .children(handle)
                .expect("element children should resolve"),
        );
    }
    node_ids
}

fn frame_for_html_id<'a>(
    completed: &'a crate::session::RuntimeLayoutCompleted,
    node_ids: &std::collections::BTreeMap<String, u64>,
    html_id: &str,
) -> Option<&'a crate::session::RuntimeLayoutFrame> {
    let node_id = node_ids.get(html_id)?;
    completed
        .frames
        .iter()
        .find(|frame| frame.node_id == *node_id)
}

fn assert_frame(frame: &crate::session::RuntimeLayoutFrame, expected: [f32; 4], html_id: &str) {
    let actual = [frame.x, frame.y, frame.width, frame.height];
    assert_eq!(actual, expected, "{html_id}");
}

#[test]
fn parsed_root_html_column_mixed_cross_axis_matches_all_precomparison_frames() {
    let expected: Value = serde_json::from_str(EXPECTED)
        .expect("column-flow mixed cross-axis expected frames should be valid JSON");
    let expected_scenes = expected["scenes"]
        .as_array()
        .expect("expected scene list should exist");
    assert_eq!(expected_scenes.len(), 9);

    let mut expected_frame_count = 0;

    for expected_scene in expected_scenes {
        if expected_scene["productDisposition"].as_str() != Some("positive") {
            continue;
        }
        let scene_id = expected_scene["id"]
            .as_str()
            .expect("scene id should be present");
        let source = scene_source(scene_id);
        let (session, layout) = run(&source);
        let node_ids = node_ids_by_html_id(&session);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "{scene_id}: layout error={:?}; HTML ids={node_ids:?}",
            layout.error
        );
        let completed = layout
            .completed
            .as_ref()
            .expect("supported positive scene should complete");
        let render_scene = completed
            .render_snapshot
            .as_ref()
            .expect("Root HTML runtime should publish its render snapshot");
        let root_id = format!("grid-{scene_id}");
        let root = frame_for_html_id(completed, &node_ids, &root_id)
            .unwrap_or_else(|| panic!("{scene_id}: Grid root frame is missing"));
        let root_css = expected_scene["rootCssPx"]
            .as_array()
            .expect("scene size should exist");
        let width = root_css[0].as_f64().unwrap() as f32;
        let height = root_css[1].as_f64().unwrap() as f32;
        assert_frame(root, [0.0, 0.0, width, height], &root_id);

        let frames = expected_scene["framesCssPx"]
            .as_object()
            .expect("positive scene frames should exist");
        for (item_id, values) in frames {
            let html_id = format!("{scene_id}--{item_id}");
            let frame = frame_for_html_id(completed, &node_ids, &html_id)
                .unwrap_or_else(|| panic!("{scene_id}/{item_id}: layout frame is missing"));
            let values = values.as_array().expect("frame should be an array");
            let local = [
                values[0].as_f64().unwrap() as f32,
                values[1].as_f64().unwrap() as f32,
                values[2].as_f64().unwrap() as f32,
                values[3].as_f64().unwrap() as f32,
            ];
            assert_frame(frame, local, &html_id);
            expected_frame_count += 1;
        }

        if let Some(non_participating) = expected_scene["nonParticipating"].as_array() {
            for item_id in non_participating {
                let item_id = item_id.as_str().expect("item id should be a string");
                let html_id = format!("{scene_id}--{item_id}");
                assert!(
                    frame_for_html_id(completed, &node_ids, &html_id).is_none(),
                    "{scene_id}/{item_id}: display:none must not have a layout frame"
                );
            }
        }
        let hidden_item_ids: &[&str] = match scene_id {
            "hidden-controls" => &["hidden-row", "hidden-column"],
            "hidden-row-lock-qualification" => &["hidden-row-lock"],
            "hidden-column-lock-qualification" => &["hidden-column-lock"],
            _ => &[],
        };
        for item_id in hidden_item_ids {
            let html_id = format!("{scene_id}--{item_id}");
            let node_id = node_ids[&html_id];
            assert!(
                !render_scene
                    .boxes()
                    .iter()
                    .any(|render_box| render_box.node_id().get() == node_id),
                "{html_id}: visibility:hidden must retain its layout frame but suppress paint"
            );
        }
    }
    assert_eq!(expected_frame_count, 31);
}

#[test]
fn parsed_root_html_column_mixed_cross_axis_rejects_row_and_column_implicit_extent_atomically() {
    let (session, layout) = run(IMPLICIT_BOUNDARY_SOURCE);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Failed,
        "the row and column implicit-boundary control must fail: {:?}",
        layout.error
    );
    assert!(
        layout.completed.is_none(),
        "failure must publish no partial scene"
    );
    let row_error = layout
        .error
        .as_ref()
        .expect("row rejection needs a diagnostic");
    assert!(
        row_error
            .property
            .as_deref()
            .is_some_and(|property| property.starts_with("grid-row-start")),
        "unexpected row diagnostic: {row_error:?}"
    );
    let row_node_id = node_ids_by_html_id(&session)["implicit-boundary-control--row-four"];
    assert_eq!(row_error.node_id, Some(row_node_id));

    let column_only = IMPLICIT_BOUNDARY_SOURCE
        .replace("grid-row:4;grid-column:auto", "grid-row:2;grid-column:auto");
    let (session, layout) = run(&column_only);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Failed,
        "the isolated column extent control must fail: {:?}",
        layout.error
    );
    assert!(
        layout.completed.is_none(),
        "failure must publish no partial scene"
    );
    let column_error = layout
        .error
        .as_ref()
        .expect("column rejection needs a diagnostic");
    assert!(
        column_error
            .property
            .as_deref()
            .is_some_and(|property| property.starts_with("grid-column-start")),
        "unexpected column diagnostic: {column_error:?}"
    );
    let column_node_id = node_ids_by_html_id(&session)["implicit-boundary-control--column-five"];
    assert_eq!(column_error.node_id, Some(column_node_id));
}
