use super::*;
#[cfg(feature = "c14-1-live-v8-fixture-test")]
use serde_json::Value;
#[cfg(feature = "c14-1-live-v8-fixture-test")]
use spinon_core::{AttributeName, HostDocumentSnapshot, HostNodeHandle, HostNodeKind};

#[cfg(feature = "c14-1-live-v8-fixture-test")]
const ROOT_HTML_DENSE_MATRIX: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-dense-auto-placement-runtime-matrix-v1.json"
);

const ROW_DENSE: &str = r#"<!doctype html><html><head><meta name="viewport" content="width=device-width, initial-scale=1"><style>
html,body{margin:0;padding:0}*{box-sizing:content-box}
#grid{display:grid;width:120px;height:40px;grid-template-columns:40px 40px 40px;grid-template-rows:20px 20px;grid-auto-flow:row dense;grid-auto-rows:auto;grid-auto-columns:auto}
.wide{width:84px;height:18px;grid-column:span 2}.small{width:18px;height:18px}
#a{background-color:#2866f6}#b{background-color:#8758ff}#c{background-color:#19a974}
</style></head><body><div id="grid"><div id="a" class="wide"></div><div id="b" class="wide"></div><div id="c" class="small"></div></div></body></html>"#;

const COLUMN_DENSE: &str = r#"<!doctype html><html><head><meta name="viewport" content="width=device-width, initial-scale=1"><style>
html,body{margin:0;padding:0}*{box-sizing:content-box}
#grid{display:grid;width:80px;height:60px;grid-template-columns:40px 40px;grid-template-rows:20px 20px 20px;grid-auto-flow:column dense;grid-auto-rows:auto;grid-auto-columns:auto}
.wide{width:18px;height:40px;grid-row:span 2}.small{width:18px;height:18px}
#a{background-color:#2866f6}#b{background-color:#8758ff}#c{background-color:#19a974}
</style></head><body><div id="grid"><div id="a" class="wide"></div><div id="b" class="wide"></div><div id="c" class="small"></div></div></body></html>"#;

const NESTED_DENSE: &str = r#"<!doctype html><html><head><meta name="viewport" content="width=device-width, initial-scale=1"><style>
html,body{margin:0;padding:0}*{box-sizing:content-box}
#outer{display:grid;width:120px;height:40px;grid-template-columns:120px;grid-template-rows:40px;grid-auto-flow:row}
#inner{display:grid;width:120px;height:40px;grid-template-columns:40px 40px 40px;grid-template-rows:20px 20px;grid-auto-flow:row dense;grid-auto-rows:auto;grid-auto-columns:auto}
.wide{width:84px;height:18px;grid-column:span 2}.small{width:18px;height:18px}
#a{background-color:#2866f6}#b{background-color:#8758ff}#c{background-color:#19a974}
</style></head><body><div id="outer"><div id="inner"><div id="a" class="wide"></div><div id="b" class="wide"></div><div id="c" class="small"></div></div></div></body></html>"#;

const AUTO_ROW_EXPANSION_REJECT: &str = r#"<!doctype html><html><head><meta name="viewport" content="width=device-width, initial-scale=1"><style>
html,body{margin:0;padding:0}*{box-sizing:content-box}
#grid{display:grid;width:40px;height:20px;grid-template-columns:40px;grid-template-rows:20px;grid-auto-flow:row dense;grid-auto-rows:auto;grid-auto-columns:auto}
.item{width:18px;height:18px;background-color:#2866f6}
</style></head><body><div id="grid"><div class="item"></div><div class="item"></div></div></body></html>"#;

const SPAN_AT_TRACK_LIMIT: &str = r#"<!doctype html><html><head><meta name="viewport" content="width=device-width, initial-scale=1"><style>
html,body{margin:0;padding:0}*{box-sizing:content-box}
#grid{display:grid;width:360px;height:20px;grid-template-columns:1px;grid-template-rows:1px;grid-auto-flow:row dense;grid-auto-rows:auto;grid-auto-columns:1px}
#item{width:10000px;height:1px;grid-column:span 10000;grid-row:1 / 2;background-color:#2866f6}
</style></head><body><div id="grid"><div id="item"></div></div></body></html>"#;

const SPAN_OVER_TRACK_LIMIT: &str = r#"<!doctype html><html><head><meta name="viewport" content="width=device-width, initial-scale=1"><style>
html,body{margin:0;padding:0}*{box-sizing:content-box}
#grid{display:grid;width:360px;height:20px;grid-template-columns:1px;grid-template-rows:1px;grid-auto-flow:row dense;grid-auto-rows:auto;grid-auto-columns:1px}
#item{width:10001px;height:1px;grid-column:span 10001;grid-row:1 / 2;background-color:#2866f6}
</style></head><body><div id="grid"><div id="item"></div></div></body></html>"#;

fn run(source: &str) -> crate::session::RuntimeLayoutSnapshot {
    run_with_session(source).1
}

#[cfg(feature = "c14-1-live-v8-fixture-test")]
fn run_with_session(source: &str) -> (RuntimeSession, crate::session::RuntimeLayoutSnapshot) {
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(source)
        .expect("parsed Root HTML should initialize");
    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(360.0, 800.0, 1.0, media)
        .expect("root metadata should configure the CSS environment");
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

#[cfg(feature = "c14-1-live-v8-fixture-test")]
fn node_ids_by_html_id(session: &RuntimeSession) -> std::collections::BTreeMap<String, u64> {
    let document = session
        .snapshot_document_for_c14_1_fixture()
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

#[cfg(feature = "c14-1-live-v8-fixture-test")]
fn html_ids_in_document_order(session: &RuntimeSession) -> Vec<String> {
    fn visit(
        document: &HostDocumentSnapshot,
        handle: HostNodeHandle,
        id_attribute: &AttributeName,
        ids: &mut Vec<String>,
    ) {
        let node = document
            .node(handle)
            .expect("document child handle should resolve");
        if let HostNodeKind::Element(element) = node.kind()
            && let Some(id) = element.attribute(id_attribute)
        {
            ids.push(id.to_string_lossy());
        }
        for child in document
            .children(handle)
            .expect("element children should resolve")
        {
            visit(document, child, id_attribute, ids);
        }
    }

    let document = session
        .snapshot_document_for_c14_1_fixture()
        .expect("parsed Root HTML document should be snapshotable");
    let id_attribute = AttributeName::new(None, "id").unwrap();
    let mut ids = Vec::new();
    for root in document.root_children() {
        visit(&document, root, &id_attribute, &mut ids);
    }
    ids
}

fn paint_frame(
    scene: &spinon_render::RuntimeRenderSnapshot,
    color: spinon_render::OpaqueCssSrgb,
) -> [f32; 4] {
    let render_box = scene
        .boxes()
        .iter()
        .find(|render_box| {
            matches!(
                render_box.paint(),
                spinon_render::RuntimePaint::Opaque(actual) if actual == color
            )
        })
        .expect("fixture paint should be present");
    let frame = render_box.frame_css_px();
    [frame.x(), frame.y(), frame.width(), frame.height()]
}

const BLUE: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(40, 102, 246);
const PURPLE: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(135, 88, 255);
const GREEN: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(25, 169, 116);

fn assert_authored_paint_order(scene: &spinon_render::RuntimeRenderSnapshot) {
    let authored_colors = scene
        .boxes()
        .iter()
        .filter_map(|render_box| match render_box.paint() {
            spinon_render::RuntimePaint::Opaque(color)
                if [BLUE, PURPLE, GREEN].contains(&color) =>
            {
                Some(color)
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(authored_colors, [BLUE, PURPLE, GREEN]);
}

#[test]
fn parsed_root_html_dense_row_and_column_fill_holes() {
    for source in [ROW_DENSE, NESTED_DENSE] {
        let layout = run(source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "layout={layout:?}"
        );
        let scene = layout
            .completed
            .as_ref()
            .and_then(|completed| completed.render_snapshot.clone())
            .expect("Root HTML dense scene should be published");
        assert_authored_paint_order(&scene);
        assert_eq!(paint_frame(&scene, BLUE), [0.0, 0.0, 84.0, 18.0]);
        assert_eq!(paint_frame(&scene, PURPLE), [0.0, 20.0, 84.0, 18.0]);
        assert_eq!(paint_frame(&scene, GREEN), [80.0, 0.0, 18.0, 18.0]);
    }

    let layout = run(COLUMN_DENSE);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout={layout:?}"
    );
    let scene = layout
        .completed
        .as_ref()
        .and_then(|completed| completed.render_snapshot.clone())
        .expect("Root HTML column-dense scene should be published");
    assert_authored_paint_order(&scene);
    assert_eq!(paint_frame(&scene, BLUE), [0.0, 0.0, 18.0, 40.0]);
    assert_eq!(paint_frame(&scene, PURPLE), [40.0, 0.0, 18.0, 40.0]);
    assert_eq!(paint_frame(&scene, GREEN), [0.0, 40.0, 18.0, 18.0]);
}

#[test]
fn parsed_root_html_dense_visibility_hidden_item_keeps_its_layout_slot() {
    let source = ROW_DENSE
        .replace(
            "#b{background-color:#8758ff}",
            "#b{background-color:#8758ff;visibility:hidden}",
        )
        .replace("id=\"b\" class=\"wide\"", "id=\"b\" class=\"small\"");
    let (session, layout) = run_with_session(&source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout={layout:?}"
    );
    let completed = layout.completed.as_ref().expect("layout should complete");
    let node_ids = node_ids_by_html_id(&session);
    for (html_id, expected) in [
        ("a", [0.0, 0.0, 84.0, 18.0]),
        ("b", [80.0, 0.0, 18.0, 18.0]),
        ("c", [0.0, 20.0, 18.0, 18.0]),
    ] {
        let node_id = node_ids
            .get(html_id)
            .unwrap_or_else(|| panic!("{html_id}: parsed DOM id missing"));
        let frame = completed
            .frames
            .iter()
            .find(|frame| frame.node_id == *node_id)
            .unwrap_or_else(|| panic!("{html_id}: visibility:hidden layout frame missing"));
        assert_eq!(
            [frame.x, frame.y, frame.width, frame.height],
            expected,
            "{html_id}: visibility:hidden must retain grid occupancy"
        );
    }
    let hidden_node = node_ids["b"];
    assert!(
        !completed
            .render_snapshot
            .as_ref()
            .expect("Root HTML render snapshot should be published")
            .boxes()
            .iter()
            .any(|render_box| render_box.node_id().get() == hidden_node),
        "visibility:hidden must retain layout occupancy and suppress its paint box"
    );
}

#[test]
fn parsed_root_html_auto_implicit_track_is_rejected_when_dense_placement_expands() {
    let layout = run(AUTO_ROW_EXPANSION_REJECT);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Failed,
        "layout={layout:?}"
    );
}

#[test]
fn parsed_root_html_dense_tracks_accept_axis_limit_and_reject_one_overflowing_track() {
    let layout = run(SPAN_AT_TRACK_LIMIT);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout={layout:?}"
    );
    let scene = layout
        .completed
        .as_ref()
        .and_then(|completed| completed.render_snapshot.clone())
        .expect("track-limit scene should be published");
    assert_eq!(paint_frame(&scene, BLUE), [0.0, 0.0, 10000.0, 1.0]);

    let over_limit = run(SPAN_OVER_TRACK_LIMIT);
    assert_eq!(
        over_limit.state,
        crate::session::RuntimeLayoutState::Failed,
        "layout={over_limit:?}"
    );
    assert!(
        over_limit.completed.is_none(),
        "rejection must not publish partial layout"
    );
    assert!(
        over_limit
            .error
            .as_ref()
            .and_then(|error| error.property.as_deref())
            .is_some_and(|property| property.starts_with("grid-template-columns"))
    );
}

#[test]
#[cfg(feature = "c14-1-live-v8-fixture-test")]
fn parsed_root_html_dense_runtime_matrix_matches_independent_chrome_frames() {
    let matrix: Value = serde_json::from_str(ROOT_HTML_DENSE_MATRIX)
        .expect("fixed Root HTML dense matrix should be valid JSON");
    assert_eq!(
        matrix["schema"].as_str(),
        Some("spinon-css-c11-root-html-dense-auto-placement-runtime-matrix/v1")
    );
    let cases = matrix["cases"]
        .as_array()
        .expect("matrix cases should exist");
    assert_eq!(cases.len(), 14);
    assert_eq!(matrix["summary"]["successCount"].as_u64(), Some(12));
    assert_eq!(matrix["summary"]["expectedRejectCount"].as_u64(), Some(2));

    let mut successes = 0;
    let mut expected_rejections = 0;
    for case in cases {
        let scene_id = case["id"].as_str().expect("scene id should exist");
        let source = case["source"].as_str().expect("static source should exist");
        let (session, layout) = run_with_session(source);
        if case["expected"]["outcome"].as_str() == Some("expected-rejection") {
            expected_rejections += 1;
            assert_eq!(
                layout.state,
                crate::session::RuntimeLayoutState::Failed,
                "{scene_id}: layout={layout:?}"
            );
            let expected_property = case["expected"]["property"]
                .as_str()
                .expect("expected property should exist");
            assert!(
                layout
                    .error
                    .as_ref()
                    .and_then(|error| error.property.as_deref())
                    .is_some_and(|property| property.starts_with(expected_property)),
                "{scene_id}: expected {expected_property}, got {:?}",
                layout.error
            );
            continue;
        }
        assert_eq!(
            case["expected"]["outcome"].as_str(),
            Some("success"),
            "{scene_id}: unexpected outcome"
        );
        successes += 1;
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "{scene_id}: layout={layout:?}"
        );
        let completed = layout.completed.as_ref().expect("layout should complete");
        let expected_source_ids = case["nodeIds"]
            .as_array()
            .expect("source node IDs should exist")
            .iter()
            .map(|id| {
                id.as_str()
                    .expect("source node ID should be a string")
                    .to_owned()
            })
            .collect::<Vec<_>>();
        let scene_id_prefix = format!("c1123-{scene_id}-");
        let authored_document_ids = html_ids_in_document_order(&session)
            .into_iter()
            .filter(|id| id.starts_with(&scene_id_prefix))
            .collect::<Vec<_>>();
        assert_eq!(
            authored_document_ids, expected_source_ids,
            "{scene_id}: dense layout must preserve authored document child order"
        );
        let node_ids = node_ids_by_html_id(&session);
        for expected in case["chrome"]["nodes"]
            .as_array()
            .expect("Chrome expected node frames should exist")
        {
            let html_id = expected["id"].as_str().expect("HTML id should exist");
            let runtime_node = node_ids
                .get(html_id)
                .unwrap_or_else(|| panic!("{scene_id}/{html_id}: parsed DOM id missing"));
            let frame = completed
                .frames
                .iter()
                .find(|frame| frame.node_id == *runtime_node);
            if expected["display"].as_str() == Some("none") {
                assert!(
                    frame.is_none(),
                    "{scene_id}/{html_id}: hidden node has a frame"
                );
                continue;
            }
            let frame = frame.unwrap_or_else(|| panic!("{scene_id}/{html_id}: frame missing"));
            let expected_rect = &expected["documentRect"];
            for (property, actual, expected_value) in [
                ("x", frame.x, expected_rect["x"].as_f64().unwrap() as f32),
                ("y", frame.y, expected_rect["y"].as_f64().unwrap() as f32),
                (
                    "width",
                    frame.width,
                    expected_rect["width"].as_f64().unwrap() as f32,
                ),
                (
                    "height",
                    frame.height,
                    expected_rect["height"].as_f64().unwrap() as f32,
                ),
            ] {
                assert!(
                    (actual - expected_value).abs() <= 0.5,
                    "{scene_id}/{html_id}/{property}: runtime={actual}, Chrome={expected_value}"
                );
            }
        }
    }
    assert_eq!(successes, 12);
    assert_eq!(expected_rejections, 2);
}
