use super::{
    SystemReserve, build_runtime_render_snapshot_with_reserve,
    overflow_runtime_inputs_with_reordered_child, overflow_runtime_inputs_with_viewport,
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

#[test]
fn positioned_auto_background_precedes_simple_in_flow_children_for_each_overflow_mode() {
    for (case_id, overflow) in [
        ("visible", "overflow:visible;"),
        ("clip", "overflow:clip;"),
        ("hidden", "overflow:hidden;"),
        ("mixed-clip-auto", "overflow-x:clip;overflow-y:auto;"),
    ] {
        let positioned_style = format!(
            "position:absolute;left:0;top:1px;width:64px;height:14px;background-color:#344256;{overflow}"
        );
        let styles = [
            "position:relative;display:block;width:300px;height:100px;background-color:#101827",
            "position:absolute;left:0;top:0;width:96px;height:18px;background-color:#101827",
            positioned_style.as_str(),
            "width:88px;height:16px;background-color:#57a6d9",
        ];
        let parents = [None, Some(0), Some(1), Some(2)];
        let (input, node_ids) =
            overflow_runtime_inputs_with_viewport(&styles, &parents, 300.0, 100.0);
        let scene = build_runtime_render_snapshot_with_reserve(
            &input.document,
            input.root,
            &input.output,
            input.current,
            &mut SystemReserve,
        )
        .unwrap();
        let positioned_box = scene
            .boxes()
            .iter()
            .find(|render_box| render_box.node_id() == node_ids[2])
            .unwrap();
        let static_target = scene
            .boxes()
            .iter()
            .find(|render_box| render_box.node_id() == node_ids[3])
            .unwrap();

        assert!(
            positioned_box.paint_order() < static_target.paint_order(),
            "{case_id}: positioned background must precede its in-flow child"
        );
        assert!(
            static_target.hit_test_order() > positioned_box.hit_test_order(),
            "{case_id}: descendant hit testing must remain above its parent"
        );
    }
}

#[test]
fn positioned_auto_parent_keeps_static_box_before_its_positioned_escape_sibling() {
    let styles = [
        "position:relative;display:block;width:300px;height:100px;background-color:#101827",
        "position:absolute;left:200px;top:60px;width:96px;height:18px;background-color:#101827",
        "position:static;box-sizing:border-box;width:64px;height:14px;padding:2px;border:1px solid #f0c674;overflow:clip;background-color:#344256",
        "position:absolute;left:58px;top:8px;box-sizing:border-box;width:30px;height:12px;background-color:#e3544e",
    ];
    let parents = [None, Some(0), Some(1), Some(1)];
    let (input, node_ids) = overflow_runtime_inputs_with_viewport(&styles, &parents, 300.0, 100.0);
    let scene = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut SystemReserve,
    )
    .unwrap();
    let section = scene
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == node_ids[1])
        .unwrap();
    let static_box = scene
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == node_ids[2])
        .unwrap();
    let escape_target = scene
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == node_ids[3])
        .unwrap();

    assert!(
        section.paint_order() < static_box.paint_order(),
        "positioned parent background must precede its in-flow static box"
    );
    assert!(
        static_box.paint_order() < escape_target.paint_order(),
        "the positioned sibling must retain its later auto-positioned paint phase"
    );
    assert!(
        escape_target.hit_test_order() > static_box.hit_test_order(),
        "paint grouping must not change topmost hit-test order"
    );
}

#[test]
fn positioned_escape_subtrees_keep_their_own_static_paint_groups() {
    let styles = [
        "position:relative;display:block;width:300px;height:100px;background-color:#101827",
        "position:absolute;left:0;top:0;width:96px;height:40px;overflow:visible;background-color:#101827",
        "position:static;width:20px;height:10px;overflow:clip;background-color:#344256",
        "position:absolute;left:30px;top:0;width:24px;height:20px;background-color:#e3544e",
        "position:static;width:12px;height:8px;background-color:#57a6d9",
        "position:absolute;z-index:1;left:60px;top:0;width:24px;height:20px;background-color:#8a45ad",
        "position:static;width:12px;height:8px;background-color:#f0c674",
    ];
    let parents = [None, Some(0), Some(1), Some(1), Some(3), Some(1), Some(5)];
    let (input, node_ids) = overflow_runtime_inputs_with_viewport(&styles, &parents, 300.0, 100.0);
    let scene = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut SystemReserve,
    )
    .unwrap();
    let render_box = |node| {
        scene
            .boxes()
            .iter()
            .find(|render_box| render_box.node_id() == node_ids[node])
            .unwrap()
    };
    let outer = render_box(1);
    let in_flow = render_box(2);
    let escaped = render_box(3);
    let escaped_child = render_box(4);
    let stacking_context = render_box(5);
    let context_child = render_box(6);

    assert!(outer.paint_order() < in_flow.paint_order());
    assert!(in_flow.paint_order() < escaped.paint_order());
    assert!(escaped.paint_order() < escaped_child.paint_order());
    assert!(escaped_child.paint_order() < stacking_context.paint_order());
    assert!(stacking_context.paint_order() < context_child.paint_order());
    assert!(escaped.hit_test_order() < escaped_child.hit_test_order());
    assert!(stacking_context.hit_test_order() < context_child.hit_test_order());
    assert!(
        escaped.clip_css_px().x().is_none(),
        "a static sibling's overflow clip must not attach to a positioned escape sibling"
    );
    assert!(
        stacking_context.paint_order() > in_flow.paint_order(),
        "a non-auto stacking context must remain in its separate paint phase"
    );
}

#[test]
fn moving_static_descendants_before_a_positioned_sibling_preserves_tree_order() {
    let styles = [
        "position:relative;display:block;width:300px;height:100px;background-color:#101827",
        "position:absolute;left:0;top:0;width:96px;height:40px;background-color:#101827",
        "position:static;width:12px;height:8px;background-color:#344256",
        "position:static;width:12px;height:8px;background-color:#57a6d9",
        "position:absolute;left:30px;top:0;width:24px;height:20px;background-color:#e3544e",
    ];
    let parents = [None, Some(0), Some(1), Some(1), Some(1)];
    let (input, node_ids) = overflow_runtime_inputs_with_reordered_child(
        &styles,
        &parents,
        300.0,
        100.0,
        Some((1, 3, 2)),
    );
    let scene = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut SystemReserve,
    )
    .unwrap();
    let paint_order = |node| {
        scene
            .boxes()
            .iter()
            .find(|render_box| render_box.node_id() == node_ids[node])
            .unwrap()
            .paint_order()
    };

    assert!(paint_order(1) < paint_order(3));
    assert!(paint_order(3) < paint_order(2));
    assert!(paint_order(2) < paint_order(4));
}

#[test]
fn positioned_relative_auto_parent_paints_its_static_child_after_its_background() {
    let styles = [
        "position:relative;display:block;width:300px;height:100px;background-color:#101827",
        "position:relative;width:96px;height:40px;background-color:#344256",
        "position:static;width:64px;height:20px;background-color:#57a6d9",
    ];
    let parents = [None, Some(0), Some(1)];
    let (input, node_ids) = overflow_runtime_inputs_with_viewport(&styles, &parents, 300.0, 100.0);
    let scene = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut SystemReserve,
    )
    .unwrap();
    let parent = scene
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == node_ids[1])
        .unwrap();
    let child = scene
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == node_ids[2])
        .unwrap();

    assert!(
        parent.paint_order() < child.paint_order(),
        "relative z-index:auto parent background must precede its in-flow child"
    );
}
