use super::{
    SystemReserve, build_runtime_render_snapshot_with_reserve,
    overflow_runtime_inputs_with_viewport,
};
use serde_json::{Value, json};
use spinon_core::NodeId;
use spinon_render::{CssClip, RuntimeRenderSnapshot};

const CHROME_MATRIX_OBSERVATION: &str = include_str!(
    "../../../../../spec/internal/evidence/c13-1-overflow-clip-matrix-2026-10-14/chrome155-selected-observation.json"
);
const EXPECTED_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/155.0.0.0 Safari/537.36";

fn scene_for(styles: &[&str], parents: &[Option<usize>]) -> (RuntimeRenderSnapshot, Vec<NodeId>) {
    let (input, nodes) = overflow_runtime_inputs_with_viewport(styles, parents, 1280.0, 633.0);
    let scene = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut SystemReserve,
    )
    .unwrap();
    (scene, nodes)
}

fn target(scene: &RuntimeRenderSnapshot, node: NodeId) -> &spinon_render::RuntimeRenderBox {
    scene
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == node)
        .unwrap()
}

fn axis_bounds(clip: CssClip, x_axis: bool) -> Option<(f32, f32)> {
    let axis = if x_axis { clip.x() } else { clip.y() }?;
    Some((axis.min(), axis.max()))
}

fn chrome_case(case_id: &str) -> Value {
    let observation: Value = serde_json::from_str(CHROME_MATRIX_OBSERVATION)
        .expect("agent-browser Chrome 155 matrix observation JSON");
    assert_eq!(
        observation["browser"]["version"].as_str(),
        Some("155.0.8059.40")
    );
    assert_eq!(
        observation["browser"]["userAgent"].as_str(),
        Some(EXPECTED_USER_AGENT)
    );
    assert_eq!(
        observation["fixture"]["sha256"].as_str(),
        Some("b7acd7e63f79355b28cd480511b9be82e0f2bef478f1aadf685eb3b0bf4ef16a")
    );
    let case = observation["cases"]
        .as_array()
        .expect("fixed Chrome matrix rows")
        .iter()
        .find(|case| case["id"].as_str() == Some(case_id))
        .cloned()
        .unwrap_or_else(|| panic!("missing Chrome matrix case {case_id}"));
    let expected = match case_id {
        "clip-y" => json!({
            "overflowX": "visible",
            "overflowY": "clip",
            "overflowClipMargin": "0px",
            "target": {"x": 430, "y": 98, "width": 140, "height": 120, "right": 570, "bottom": 218},
            "hitRanges": {
                "x": {"fixedAt": 139, "ranges": [[430, 569]]},
                "y": {"fixedAt": 500, "ranges": [[98, 179]]}
            }
        }),
        "clip-margin-border" => json!({
            "overflowX": "clip",
            "overflowY": "clip",
            "overflowClipMargin": "border-box 8px",
            "target": {"x": 34, "y": 434, "width": 140, "height": 120, "right": 174, "bottom": 554},
            "hitRanges": {
                "x": {"fixedAt": 475, "ranges": [[34, 147]]},
                "y": {"fixedAt": 87, "ranges": [[434, 523]]}
            }
        }),
        "clip-margin-hidden" => json!({
            "overflowX": "hidden",
            "overflowY": "hidden",
            "overflowClipMargin": "border-box 8px",
            "target": {"x": 430, "y": 434, "width": 140, "height": 120, "right": 570, "bottom": 554},
            "hitRanges": {
                "x": {"fixedAt": 475, "ranges": [[430, 535]]},
                "y": {"fixedAt": 483, "ranges": [[434, 515]]}
            }
        }),
        other => panic!("unexpected Chrome matrix case {other}"),
    };
    for field in [
        "overflowX",
        "overflowY",
        "overflowClipMargin",
        "target",
        "hitRanges",
    ] {
        assert_eq!(
            case[field], expected[field],
            "pinned Chrome case {case_id} {field}"
        );
    }
    case
}

fn assert_frame_matches_chrome(scene: &RuntimeRenderSnapshot, node: NodeId, case_id: &str) {
    let case = chrome_case(case_id);
    let frame = target(scene, node).frame_css_px();
    for (field, actual) in [
        ("x", frame.x()),
        ("y", frame.y()),
        ("width", frame.width()),
        ("height", frame.height()),
    ] {
        let expected = case["target"][field]
            .as_f64()
            .unwrap_or_else(|| panic!("Chrome {case_id} target {field}"))
            as f32;
        assert!(
            (actual - expected).abs() <= 0.5,
            "Chrome {case_id} target {field}: runtime={actual}, oracle={expected}"
        );
    }
}

fn chrome_range_contains(ranges: &Value, coordinate: i32) -> bool {
    ranges.as_array().unwrap().iter().any(|range| {
        let min = range[0].as_i64().unwrap() as i32;
        let max = range[1].as_i64().unwrap() as i32;
        (min..=max).contains(&coordinate)
    })
}

fn assert_hit_ranges_match_chrome(scene: &RuntimeRenderSnapshot, node: NodeId, case_id: &str) {
    let case = chrome_case(case_id);
    let frame = target(scene, node).frame_css_px();
    for (axis_name, start, end, expected_start, expected_end) in [
        (
            "x",
            frame.x(),
            frame.x() + frame.width(),
            case["target"]["x"].as_f64().unwrap() as f32,
            case["target"]["right"].as_f64().unwrap() as f32,
        ),
        (
            "y",
            frame.y(),
            frame.y() + frame.height(),
            case["target"]["y"].as_f64().unwrap() as f32,
            case["target"]["bottom"].as_f64().unwrap() as f32,
        ),
    ] {
        assert!(
            (start - expected_start).abs() <= 0.5,
            "Chrome {case_id} {axis_name} start"
        );
        assert!(
            (end - expected_end).abs() <= 0.5,
            "Chrome {case_id} {axis_name} end"
        );
        let fixed = case["hitRanges"][axis_name]["fixedAt"].as_f64().unwrap() as f32;
        let ranges = &case["hitRanges"][axis_name]["ranges"];
        for coordinate in (start.floor() as i32)..=(end.ceil() as i32 - 1) {
            let actual = if axis_name == "x" {
                scene.hit_test_css_point(coordinate as f32, fixed)
            } else {
                scene.hit_test_css_point(fixed, coordinate as f32)
            }
            .is_some_and(|hit| hit.node_id() == node);
            assert_eq!(
                actual,
                chrome_range_contains(ranges, coordinate),
                "Chrome {case_id} {axis_name} elementFromPoint at {coordinate}"
            );
        }
    }
}

#[test]
fn y_only_clip_matches_chrome_without_restricting_x_hit_range() {
    let (scene, nodes) = scene_for(
        &[
            "display:block;position:relative;width:1280px;height:633px;background-color:#111111",
            "position:absolute;left:416px;top:84px;box-sizing:border-box;width:120px;height:96px;padding:10px;border:4px solid;overflow-x:visible;overflow-y:clip;background-color:#333333",
            "width:140px;height:120px;background-color:#ff0000",
        ],
        &[None, Some(0), Some(1)],
    );
    let clipped_target = target(&scene, nodes[2]);

    assert_eq!(axis_bounds(clipped_target.clip_css_px(), true), None);
    assert_eq!(
        axis_bounds(clipped_target.clip_css_px(), false),
        Some((88.0, 176.0))
    );
    assert_eq!(
        axis_bounds(clipped_target.hit_test_clip_css_px(), true),
        None
    );
    assert_eq!(
        axis_bounds(clipped_target.hit_test_clip_css_px(), false),
        Some((84.0, 180.0))
    );
    assert_frame_matches_chrome(&scene, nodes[2], "clip-y");
    assert_hit_ranges_match_chrome(&scene, nodes[2], "clip-y");
}

#[test]
fn border_box_clip_margin_matches_chrome_hit_ranges_on_both_axes() {
    let (scene, nodes) = scene_for(
        &[
            "display:block;position:relative;width:1280px;height:633px;background-color:#111111",
            "position:absolute;left:20px;top:420px;box-sizing:border-box;width:120px;height:96px;padding:10px;border:4px solid;overflow:clip;overflow-clip-margin:border-box 8px;background-color:#333333",
            "width:140px;height:120px;background-color:#ff0000",
        ],
        &[None, Some(0), Some(1)],
    );
    let clipped_target = target(&scene, nodes[2]);

    assert_eq!(
        axis_bounds(clipped_target.clip_css_px(), true),
        Some((12.0, 148.0))
    );
    assert_eq!(
        axis_bounds(clipped_target.clip_css_px(), false),
        Some((412.0, 524.0))
    );
    assert_eq!(
        axis_bounds(clipped_target.hit_test_clip_css_px(), true),
        Some((12.0, 148.0))
    );
    assert_eq!(
        axis_bounds(clipped_target.hit_test_clip_css_px(), false),
        Some((412.0, 524.0))
    );
    assert_frame_matches_chrome(&scene, nodes[2], "clip-margin-border");
    assert_hit_ranges_match_chrome(&scene, nodes[2], "clip-margin-border");
}

#[test]
fn hidden_ignores_clip_margin_and_matches_chrome_padding_clip_border_hit_ranges() {
    let (scene, nodes) = scene_for(
        &[
            "display:block;position:relative;width:1280px;height:633px;background-color:#111111",
            "position:absolute;left:416px;top:420px;box-sizing:border-box;width:120px;height:96px;padding:10px;border:4px solid;overflow:hidden;overflow-clip-margin:border-box 8px;background-color:#333333",
            "width:140px;height:120px;background-color:#ff0000",
        ],
        &[None, Some(0), Some(1)],
    );
    let clipped_target = target(&scene, nodes[2]);

    assert_eq!(
        axis_bounds(clipped_target.clip_css_px(), true),
        Some((420.0, 532.0))
    );
    assert_eq!(
        axis_bounds(clipped_target.clip_css_px(), false),
        Some((424.0, 512.0))
    );
    assert_eq!(
        axis_bounds(clipped_target.hit_test_clip_css_px(), true),
        Some((416.0, 536.0))
    );
    assert_eq!(
        axis_bounds(clipped_target.hit_test_clip_css_px(), false),
        Some((420.0, 516.0))
    );
    assert_frame_matches_chrome(&scene, nodes[2], "clip-margin-hidden");
    assert_hit_ranges_match_chrome(&scene, nodes[2], "clip-margin-hidden");
}
