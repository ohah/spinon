use super::{
    SystemReserve, build_runtime_render_snapshot_with_reserve,
    overflow_runtime_inputs_with_viewport,
};
use serde_json::Value;
use spinon_core::NodeId;
use spinon_render::RuntimeRenderSnapshot;

const BORDER_FIXTURE_HTML: &str =
    include_str!("../../../../../tests/fixtures/css/c13/overflow-clip-hit-border-c13-1.html");
const BORDER_FIXTURE_OBSERVATION: &str = include_str!(
    "../../../../../spec/internal/evidence/c13-1-overflow-clip-paint-hit-border-2026-10-14/oracle-fixture-observation.json"
);

fn runtime_border_fixture() -> (RuntimeRenderSnapshot, [NodeId; 5]) {
    let styles = [
        "position:relative;display:flow-root;width:380px;height:120px;background-color:#101827",
        "position:static;margin-left:10px;margin-top:10px;box-sizing:border-box;width:120px;height:96px;padding:10px;border:4px solid #f0c674;overflow-x:clip;overflow-y:visible;background-color:#344256",
        "width:140px;height:120px;background-color:#57a6d9",
        "position:absolute;left:200px;top:10px;box-sizing:border-box;width:120px;height:96px;padding:10px;border:4px solid #f0c674;overflow-x:clip;overflow-y:visible;background-color:#344256",
        "width:140px;height:120px;background-color:#57a6d9",
    ];
    let parents = [None, Some(0), Some(1), Some(0), Some(3)];
    let (input, node_ids) = overflow_runtime_inputs_with_viewport(&styles, &parents, 400.0, 160.0);
    let scene = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut SystemReserve,
    )
    .unwrap();
    let ids = node_ids.try_into().expect("five border-fixture nodes");
    (scene, ids)
}

#[test]
fn static_and_positioned_clip_border_strips_match_chrome_without_changing_paint_clips() {
    assert!(BORDER_FIXTURE_HTML.contains("position: static;"));
    assert!(BORDER_FIXTURE_HTML.contains("position: absolute;"));
    assert!(BORDER_FIXTURE_HTML.contains("overflow-x: clip;"));
    assert!(BORDER_FIXTURE_HTML.contains("overflow-y: visible;"));

    let observation: Value = serde_json::from_str(BORDER_FIXTURE_OBSERVATION)
        .expect("pinned Chrome border-strip hit observation");
    assert_eq!(
        observation["browser"]["userAgent"].as_str(),
        Some(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/155.0.0.0 Safari/537.36"
        )
    );
    let (scene, nodes) = runtime_border_fixture();

    let static_target = scene
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == nodes[2])
        .unwrap();
    let positioned_target = scene
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == nodes[4])
        .unwrap();
    let positioned_clip = scene
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == nodes[3])
        .unwrap();
    assert_eq!(
        static_target
            .clip_css_px()
            .x()
            .map(|clip| (clip.min(), clip.max())),
        Some((14.0, 126.0)),
        "static clip paint edge remains the padding box"
    );
    assert_eq!(
        positioned_target
            .clip_css_px()
            .x()
            .map(|clip| (clip.min(), clip.max())),
        Some((204.0, 316.0)),
        "positioned clip paint edge remains the padding box"
    );
    assert!(
        positioned_clip.paint_order() < positioned_target.paint_order(),
        "positioned parent background is painted before its in-flow child"
    );
    assert_eq!(positioned_clip.border().right().width_css_px(), 4.0);
    assert_eq!(
        positioned_clip.border().right().paint(),
        spinon_render::RuntimePaint::Opaque(spinon_render::OpaqueCssSrgb::new(240, 198, 116))
    );
    assert!(
        positioned_target.hit_test_order() > positioned_clip.hit_test_order(),
        "hit testing considers the descendant before its parent"
    );

    let samples = observation["samples"]
        .as_array()
        .expect("Chrome border-strip samples");
    assert_eq!(samples.len(), 6);
    for sample in samples {
        assert_eq!(sample["hitMatches"].as_bool(), Some(true));
        let sample_id = sample["id"].as_str().expect("sample id");
        let x = sample["xCssPx"].as_f64().expect("sample x") as f32;
        let y = sample["yCssPx"].as_f64().expect("sample y") as f32;
        let expected_id = sample["expectedElementId"].as_str().expect("expected ID");
        let expected_node = match expected_id {
            "root" => nodes[0],
            "static-clip" => nodes[1],
            "static-target" => nodes[2],
            "positioned-clip" => nodes[3],
            "positioned-target" => nodes[4],
            other => panic!("unexpected Chrome border hit target {other}"),
        };
        assert_eq!(
            scene
                .hit_test_css_point(x, y)
                .map(|render_box| render_box.node_id()),
            Some(expected_node),
            "Chrome 155 sample {sample_id} at ({x},{y}) should hit {expected_id}; runtime boxes: {:?}",
            scene.boxes()
        );
    }
}
