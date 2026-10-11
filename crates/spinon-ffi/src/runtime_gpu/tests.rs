use super::{
    PresentationScenes, RUNTIME_CSS_C12_2_ABSOLUTE_BLOCK_FIXTURE_SOURCE, RuntimeGpuHost,
    c12_3_fixed_runtime_script, c12_3_fixed_supported_nodes, scene_matches_layout_key,
};
use spinon_core::{EnvironmentRevision, HostDocument, StyleRevision};
use spinon_render::{CssSize, RuntimeRenderKey, RuntimeRenderSnapshot};
use spinon_runtime::RuntimeUaCascadeKey;
use std::sync::{Arc, Barrier};
use std::thread;

fn empty_scene() -> RuntimeRenderSnapshot {
    let document = HostDocument::new().unwrap();
    let snapshot = document.snapshot();
    RuntimeRenderSnapshot::new(
        RuntimeRenderKey::new(
            snapshot.generation(),
            snapshot.document_revision(),
            snapshot.render_tree_revision(),
            StyleRevision::INITIAL,
            EnvironmentRevision::INITIAL,
        ),
        CssSize::new(301.0, 100.0).unwrap(),
        Vec::new(),
    )
    .unwrap()
}

#[test]
fn render_scene_must_match_the_complete_runtime_key() {
    let document = HostDocument::new().unwrap();
    let snapshot = document.snapshot();
    let scene = RuntimeRenderSnapshot::new(
        RuntimeRenderKey::new(
            snapshot.generation(),
            snapshot.document_revision(),
            snapshot.render_tree_revision(),
            StyleRevision::INITIAL,
            EnvironmentRevision::INITIAL,
        ),
        CssSize::new(301.0, 100.0).unwrap(),
        Vec::new(),
    )
    .unwrap();
    let key = RuntimeUaCascadeKey {
        generation: snapshot.generation().get(),
        document_revision: snapshot.document_revision().get(),
        render_tree_revision: snapshot.render_tree_revision().get(),
        style_revision: StyleRevision::INITIAL.get(),
        environment_revision: EnvironmentRevision::INITIAL.get(),
    };
    assert!(scene_matches_layout_key(&scene, key));
    assert!(!scene_matches_layout_key(
        &scene,
        RuntimeUaCascadeKey {
            environment_revision: 1,
            ..key
        }
    ));
}

#[test]
fn new_presentation_sequence_hides_the_old_scene_and_accepts_only_current_publish() {
    let presentation = PresentationScenes::new();
    let first = presentation.invalidate().unwrap();
    assert!(presentation.publish(first, empty_scene()));

    let (draw_sequence, current) = presentation.lock_current();
    assert_eq!(draw_sequence, first);
    assert!(current.is_some());
    drop(current);

    let second = presentation.invalidate().unwrap();
    assert!(!presentation.publish(first, empty_scene()));
    let (draw_sequence, current) = presentation.lock_current();
    assert_eq!(draw_sequence, second);
    assert!(current.is_none());
    drop(current);

    assert!(presentation.publish(second, empty_scene()));
    assert_eq!(presentation.sequence(), second);
}

#[test]
#[ignore = "호스트 V8 바인딩이 초기화되지 않는 환경에서는 Android·iOS fixture 실행으로 검증합니다"]
fn c12_2_absolute_runtime_frames_match_the_pinned_chromium_geometry() {
    let inventory: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../tests/fixtures/css/c12/position-absolute-block-inventory.json"
    ))
    .unwrap();
    let reference: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../tests/fixtures/css/references/c12-2-position-absolute-block-v1.json"
    ))
    .unwrap();
    let expected_nodes = inventory["nodes"].as_array().unwrap();
    let reference_nodes = reference["observations"][0]["nodes"].as_array().unwrap();
    assert_eq!(expected_nodes.len(), 82);
    assert_eq!(reference_nodes.len(), expected_nodes.len());
    for (expected, observed) in expected_nodes.iter().zip(reference_nodes) {
        assert_eq!(expected["id"], observed["id"]);
    }

    let (host, _) = RuntimeGpuHost::new_c12_2_absolute_block_fixture().unwrap();
    host.set_environment(360.0, 800.0, 1.0, false, 10_000)
        .unwrap();
    let report = host
        .eval(RUNTIME_CSS_C12_2_ABSOLUTE_BLOCK_FIXTURE_SOURCE, 10_000)
        .unwrap();
    let actual_nodes = parse_node_frames(&report);
    assert_eq!(actual_nodes.len(), expected_nodes.len() + 1, "{report}");
    let style_element = actual_nodes[1];
    assert_eq!(
        (
            style_element.2,
            style_element.3,
            style_element.4,
            style_element.5
        ),
        (0.0, 0.0, 0.0, 0.0)
    );
    let actual_nodes = actual_nodes
        .iter()
        .enumerate()
        .filter_map(|(index, frame)| (index != 1).then_some(*frame))
        .collect::<Vec<_>>();
    assert_eq!(actual_nodes.len(), expected_nodes.len(), "{report}");

    let tolerance = reference["comparison"]["maximumAbsoluteRectErrorCssPx"]
        .as_f64()
        .unwrap() as f32;
    for (((expected, observed), actual), index) in expected_nodes
        .iter()
        .zip(reference_nodes)
        .zip(actual_nodes)
        .zip(0..)
    {
        let node_id = expected["id"].as_str().unwrap();
        assert_eq!(
            actual.0,
            if index > 0 { index + 1 } else { index },
            "node {index} ({node_id}) runtime source-order index"
        );
        let rect = &observed["rect"];
        for (field, value) in [
            ("x", actual.2),
            ("y", actual.3),
            ("width", actual.4),
            ("height", actual.5),
        ] {
            let target = rect[field].as_f64().unwrap() as f32;
            assert!(
                (value - target).abs() <= tolerance,
                "node {index} ({node_id}) {field}: runtime={value} Chromium={target}; {report}"
            );
        }
    }
}

#[test]
fn c12_3_runtime_fixture_is_a_parent_closed_inventory_subset() {
    let nodes = c12_3_fixed_supported_nodes().unwrap();
    let ids = nodes
        .iter()
        .map(|node| node["id"].as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(nodes.len(), 28);
    assert_eq!(nodes[0]["id"], "c12-root");
    assert!(ids.contains("viewport-aspect-ratio"));
    for node in nodes.iter().skip(1) {
        assert!(ids.contains(node["parentId"].as_str().unwrap()));
        assert!(matches!(
            node["caseId"].as_str().unwrap(),
            "viewport-insets"
                | "unpositioned-ancestor"
                | "positioned-ancestors"
                | "fixed-nesting"
                | "hidden-subtree"
        ));
    }
    let source = c12_3_fixed_runtime_script().unwrap();
    assert!(source.contains("globalThis.__spinonC123FixtureNodes="));
    assert!(source.contains("spinonC123NodeRefs"));
    assert!(!source.contains("offset-path"));
    assert!(!source.contains("content-visibility"));
    assert!(!source.contains("will-change:"));
}

#[test]
#[ignore = "C12.3 V8 runtime frame comparison runs on the Android device and iOS Simulator"]
fn c12_3_fixed_runtime_frames_match_chromium_across_dpr_and_resize() {
    let inventory: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../tests/fixtures/css/c12/position-fixed-inventory.json"
    ))
    .unwrap();
    let reference: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../tests/fixtures/css/references/c12-3-position-fixed-v1.json"
    ))
    .unwrap();
    let expected_nodes = c12_3_fixed_supported_nodes().unwrap();
    let (host, _) = RuntimeGpuHost::new_c12_3_fixed_fixture().unwrap();
    let script = c12_3_fixed_runtime_script().unwrap();

    let matrix = [
        (360.0, 800.0, 1.0, "matrix-360x800-dpr-1"),
        (360.0, 800.0, 2.0, "matrix-360x800-dpr-2"),
        (360.0, 800.0, 2.625, "matrix-360x800-dpr-2.625"),
        (360.0, 800.0, 3.0, "matrix-360x800-dpr-3"),
        (390.0, 844.0, 1.0, "matrix-390x844-dpr-1"),
        (390.0, 844.0, 2.0, "matrix-390x844-dpr-2"),
        (390.0, 844.0, 2.625, "matrix-390x844-dpr-2.625"),
        (390.0, 844.0, 3.0, "matrix-390x844-dpr-3"),
    ];
    let mut environment_revision = EnvironmentRevision::INITIAL.get();
    for (index, (width, height, scale, observation_id)) in matrix.into_iter().enumerate() {
        let environment = host
            .set_environment(width, height, scale, false, 10_000)
            .unwrap();
        if index != 0 {
            environment_revision += 1;
        }
        assert_eq!(
            reported_environment_revision(&environment),
            environment_revision,
            "{environment}"
        );
        let report = if index == 0 {
            host.eval(&script, 10_000).unwrap()
        } else {
            environment
        };
        assert_eq!(
            reported_environment_revision(&report),
            environment_revision,
            "{report}"
        );
        assert_c12_3_report_matches(&report, &expected_nodes, &reference, observation_id);
    }

    let resize_sequence = [
        (360.0, 800.0, 1.0, "resize-start"),
        (390.0, 844.0, 1.0, "resize-outbound"),
        (390.0, 844.0, 1.0, "resize-noop"),
        (360.0, 800.0, 1.0, "resize-return"),
    ];
    for (index, (width, height, scale, observation_id)) in resize_sequence.into_iter().enumerate() {
        let report = host
            .set_environment(width, height, scale, false, 10_000)
            .unwrap();
        if index != 2 {
            environment_revision += 1;
        }
        assert_eq!(
            reported_environment_revision(&report),
            environment_revision,
            "{report}"
        );
        assert_c12_3_report_matches(&report, &expected_nodes, &reference, observation_id);
    }

    assert_eq!(
        inventory["comparison"]["maximumAbsoluteRectErrorCssPx"],
        reference["comparison"]["maximumAbsoluteRectErrorCssPx"]
    );
}

fn reported_environment_revision(report: &str) -> u64 {
    report
        .split_once("environment_revision=")
        .and_then(|(_, suffix)| suffix.split_whitespace().next())
        .unwrap()
        .parse()
        .unwrap()
}

fn assert_c12_3_report_matches(
    report: &str,
    expected_nodes: &[serde_json::Value],
    reference: &serde_json::Value,
    observation_id: &str,
) {
    let observation = reference["observations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|observation| observation["id"].as_str() == Some(observation_id))
        .unwrap();
    let expected_ids = expected_nodes
        .iter()
        .map(|node| node["id"].as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    let reference_nodes = observation["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|node| expected_ids.contains(node["id"].as_str().unwrap()))
        .map(|node| (node["id"].as_str().unwrap(), node))
        .collect::<std::collections::BTreeMap<_, _>>();
    let actual_nodes = parse_node_frames(report);
    assert_eq!(actual_nodes.len(), expected_nodes.len(), "{report}");
    assert_eq!(
        reference_nodes.len(),
        expected_nodes.len(),
        "{observation_id}"
    );
    let actual_by_node_id = actual_nodes
        .iter()
        .map(|frame| (frame.1, frame))
        .collect::<std::collections::BTreeMap<_, _>>();
    assert_eq!(actual_by_node_id.len(), expected_nodes.len(), "{report}");
    let mut frame_indexes = actual_nodes.iter().map(|frame| frame.0).collect::<Vec<_>>();
    frame_indexes.sort_unstable();
    assert_eq!(frame_indexes, (0..expected_nodes.len()).collect::<Vec<_>>());

    let tolerance = reference["comparison"]["maximumAbsoluteRectErrorCssPx"]
        .as_f64()
        .unwrap() as f32;
    for (index, expected) in expected_nodes.iter().enumerate() {
        let node_id = expected["id"].as_str().unwrap();
        let observed = reference_nodes.get(node_id).unwrap();
        let runtime_node_id = index as u64 + 1;
        let (_, _, x, y, width, height) = actual_by_node_id
            .get(&runtime_node_id)
            .unwrap_or_else(|| panic!("runtime NodeId {runtime_node_id} 누락: {report}"));
        assert_eq!(expected["id"], observed["id"]);
        assert_eq!(expected["parentId"], observed["parentId"]);
        assert_eq!(expected["expectedOwner"], observed["owner"]);
        assert_eq!(
            expected["expectedPosition"],
            observed["properties"]["position"]
        );
        let has_layout_box = expected["hasLayoutBox"].as_bool().unwrap_or(true);
        if !has_layout_box {
            assert_eq!((*x, *y, *width, *height), (0.0, 0.0, 0.0, 0.0));
        }
        let rect = &observed["rect"];
        for (field, value) in [("x", *x), ("y", *y), ("width", *width), ("height", *height)] {
            let target = rect[field].as_f64().unwrap() as f32;
            assert!(
                (value - target).abs() <= tolerance,
                "node {node_id} {field}: runtime={value} Chromium={target} observation={observation_id}; {report}"
            );
        }
    }
}

fn parse_node_frames(report: &str) -> Vec<(usize, u64, f32, f32, f32, f32)> {
    let Some(frames) = report
        .split_once(" node_frames_css_px=[")
        .and_then(|(_, value)| value.split_once(']'))
        .map(|(frames, _)| frames)
    else {
        panic!("runtime frame 보고가 없습니다: {report}");
    };
    frames
        .split(';')
        .map(|frame| {
            let (index, values) = frame.split_once(':').unwrap();
            let fields = values
                .split(',')
                .map(|field| field.split_once('=').unwrap())
                .collect::<Vec<_>>();
            assert_eq!(fields.len(), 5);
            assert_eq!(fields[0].0, "node");
            (
                index.parse().unwrap(),
                fields[0].1.parse().unwrap(),
                fields[1].1.parse().unwrap(),
                fields[2].1.parse().unwrap(),
                fields[3].1.parse().unwrap(),
                fields[4].1.parse().unwrap(),
            )
        })
        .collect()
}

#[test]
fn invalidation_winning_a_publish_race_rejects_the_stale_scene() {
    let presentation = Arc::new(PresentationScenes::new());
    let stale_sequence = presentation.invalidate().unwrap();
    let barrier = Arc::new(Barrier::new(2));
    let publisher = {
        let presentation = Arc::clone(&presentation);
        let barrier = Arc::clone(&barrier);
        thread::spawn(move || {
            barrier.wait();
            presentation.publish(stale_sequence, empty_scene())
        })
    };

    presentation.invalidate().unwrap();
    barrier.wait();
    assert!(!publisher.join().unwrap());
    let (_, current) = presentation.lock_current();
    assert!(current.is_none());
}

#[cfg(target_os = "macos")]
#[test]
fn runtime_scene_readback_matches_the_fixed_chromium_color_samples() {
    use spinon_core::{HostDocument, NodeId, StyleRevision};
    use spinon_render::{
        CssRect, CssSize, OpaqueCssSrgb, RuntimePaint, RuntimeRenderBox, RuntimeRenderKey,
        RuntimeRenderSnapshot,
    };
    let document = HostDocument::new().unwrap();
    let document_snapshot = document.snapshot();
    let scene = RuntimeRenderSnapshot::new(
        RuntimeRenderKey::new(
            document_snapshot.generation(),
            document_snapshot.document_revision(),
            document_snapshot.render_tree_revision(),
            StyleRevision::INITIAL,
            spinon_core::EnvironmentRevision::INITIAL,
        ),
        CssSize::new(301.0, 100.0).unwrap(),
        vec![
            RuntimeRenderBox::new(
                NodeId::new(1).unwrap(),
                CssRect::new(0.0, 0.0, 301.0, 100.0).unwrap(),
                RuntimePaint::Opaque(OpaqueCssSrgb::new(18, 52, 86)),
                0,
            ),
            RuntimeRenderBox::new(
                NodeId::new(2).unwrap(),
                CssRect::new(0.0, 0.0, 51.0, 31.0).unwrap(),
                RuntimePaint::Opaque(OpaqueCssSrgb::new(51, 102, 255)),
                1,
            ),
        ],
    )
    .unwrap();
    let samples = spinon_render_wgpu::readback_runtime_scene_samples(
        &scene,
        301,
        100,
        &[(25, 15), (150, 50), (82, 15)],
    )
    .expect("실제 WGPU offscreen readback");
    assert_eq!(
        samples,
        [[51, 102, 255, 255], [18, 52, 86, 255], [18, 52, 86, 255],]
    );

    let empty_scene = RuntimeRenderSnapshot::new(
        RuntimeRenderKey::new(
            document_snapshot.generation(),
            document_snapshot.document_revision(),
            document_snapshot.render_tree_revision(),
            StyleRevision::INITIAL,
            spinon_core::EnvironmentRevision::INITIAL,
        ),
        CssSize::new(301.0, 100.0).unwrap(),
        Vec::new(),
    )
    .unwrap();
    let clear_sample =
        spinon_render_wgpu::readback_runtime_scene_samples(&empty_scene, 2, 2, &[(0, 0)])
            .expect("빈 runtime root의 WGPU clear readback");
    assert_eq!(clear_sample, [[0, 0, 0, 255]]);
}
