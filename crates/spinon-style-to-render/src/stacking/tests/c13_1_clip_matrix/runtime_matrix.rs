use super::{EXPECTED_USER_AGENT, axis_bounds, chrome_range_contains, target};
use serde_json::{Value, json};
use spinon_core::NodeId;
use spinon_render::RuntimeRenderSnapshot;

const CHROME_RUNTIME_MATRIX_OBSERVATION: &str = include_str!(
    "../../../../../../spec/internal/evidence/c13-1-overflow-clip-runtime-matrix-2026-10-14/chrome155-runtime-matrix-observation.json"
);

fn assert_clip_bounds(
    scene: &RuntimeRenderSnapshot,
    node: NodeId,
    case_id: &str,
    paint_x: Option<(f32, f32)>,
    paint_y: Option<(f32, f32)>,
    hit_x: Option<(f32, f32)>,
    hit_y: Option<(f32, f32)>,
) {
    let render_box = target(scene, node);
    assert_eq!(
        axis_bounds(render_box.clip_css_px(), true),
        paint_x,
        "{case_id} paint x"
    );
    assert_eq!(
        axis_bounds(render_box.clip_css_px(), false),
        paint_y,
        "{case_id} paint y"
    );
    assert_eq!(
        axis_bounds(render_box.hit_test_clip_css_px(), true),
        hit_x,
        "{case_id} hit x"
    );
    assert_eq!(
        axis_bounds(render_box.hit_test_clip_css_px(), false),
        hit_y,
        "{case_id} hit y"
    );
}
fn runtime_matrix_chrome_case(case_id: &str) -> Value {
    let observation: Value = serde_json::from_str(CHROME_RUNTIME_MATRIX_OBSERVATION)
        .expect("agent-browser Chrome 155 runtime matrix observation JSON");
    assert_eq!(
        observation["browser"]["version"].as_str(),
        Some("155.0.8059.40")
    );
    assert_eq!(
        observation["browser"]["revision"].as_str(),
        Some("cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}")
    );
    assert_eq!(
        observation["browser"]["userAgent"].as_str(),
        Some(EXPECTED_USER_AGENT)
    );
    assert_eq!(
        observation["browser"]["viewportCssPx"]["width"].as_u64(),
        Some(1280)
    );
    assert_eq!(
        observation["browser"]["viewportCssPx"]["height"].as_u64(),
        Some(1100)
    );
    assert_eq!(
        observation["browser"]["viewportCssPx"]["dpr"].as_f64(),
        Some(1.0)
    );
    assert_eq!(
        observation["fixture"]["sha256"].as_str(),
        Some("b7acd7e63f79355b28cd480511b9be82e0f2bef478f1aadf685eb3b0bf4ef16a")
    );
    assert_eq!(
        observation["fixture"]["sourceObservationSha256"].as_str(),
        Some("fd6d9e66825bc453609245950635f4fc56054ae9011b14d2f8901d490ecde4b5")
    );
    let case = observation["cases"]
        .as_array()
        .expect("fixed Chrome runtime matrix rows")
        .iter()
        .find(|case| case["id"].as_str() == Some(case_id))
        .cloned()
        .unwrap_or_else(|| panic!("missing Chrome runtime matrix case {case_id}"));
    let expected = match case_id {
        "visible" => json!({
            "overflowX": "visible",
            "overflowY": "visible",
            "overflowClipMargin": "0px",
            "target": {"x": 34, "y": 98, "width": 140, "height": 120, "right": 174, "bottom": 218},
            "hitRanges": {
                "x": {"fixedAt": 158, "ranges": [[34, 173]]},
                "y": {"fixedAt": 104, "ranges": [[98, 217]]}
            }
        }),
        "clip-x" => json!({
            "overflowX": "clip",
            "overflowY": "visible",
            "overflowClipMargin": "0px",
            "target": {"x": 232, "y": 98, "width": 140, "height": 120, "right": 372, "bottom": 218},
            "hitRanges": {
                "x": {"fixedAt": 158, "ranges": [[232, 337]]},
                "y": {"fixedAt": 285, "ranges": [[98, 217]]}
            }
        }),
        "clip-both" => json!({
            "overflowX": "clip",
            "overflowY": "clip",
            "overflowClipMargin": "0px",
            "target": {"x": 34, "y": 266, "width": 140, "height": 120, "right": 174, "bottom": 386},
            "hitRanges": {
                "x": {"fixedAt": 307, "ranges": [[34, 139]]},
                "y": {"fixedAt": 87, "ranges": [[266, 347]]}
            }
        }),
        "clip-margin-padding" => json!({
            "overflowX": "clip",
            "overflowY": "clip",
            "overflowClipMargin": "8px",
            "target": {"x": 232, "y": 266, "width": 140, "height": 120, "right": 372, "bottom": 386},
            "hitRanges": {
                "x": {"fixedAt": 307, "ranges": [[232, 341]]},
                "y": {"fixedAt": 285, "ranges": [[266, 351]]}
            }
        }),
        "clip-margin-content" => json!({
            "overflowX": "clip",
            "overflowY": "clip",
            "overflowClipMargin": "content-box 8px",
            "target": {"x": 430, "y": 266, "width": 140, "height": 120, "right": 570, "bottom": 386},
            "hitRanges": {
                "x": {"fixedAt": 307, "ranges": [[430, 535]]},
                "y": {"fixedAt": 483, "ranges": [[266, 347]]}
            }
        }),
        "mixed-visible-hidden" => json!({
            "overflowX": "auto",
            "overflowY": "hidden",
            "overflowClipMargin": "0px",
            "target": {"x": 34, "y": 602, "width": 140, "height": 120, "right": 174, "bottom": 722},
            "hitRanges": {
                "x": {"fixedAt": 643, "ranges": [[34, 139]]},
                "y": {"fixedAt": 87, "ranges": [[602, 683]]}
            }
        }),
        "mixed-clip-auto" => json!({
            "overflowX": "hidden",
            "overflowY": "auto",
            "overflowClipMargin": "border-box 8px",
            "target": {"x": 232, "y": 602, "width": 140, "height": 120, "right": 372, "bottom": 722},
            "hitRanges": {
                "x": {"fixedAt": 643, "ranges": [[232, 337]]},
                "y": {"fixedAt": 285, "ranges": [[602, 683]]}
            }
        }),
        "positioned-contained" => json!({
            "overflowX": "clip",
            "overflowY": "clip",
            "overflowClipMargin": "0px",
            "target": {"x": 504, "y": 822, "width": 42, "height": 30, "right": 546, "bottom": 852},
            "hitRanges": {
                "x": {"fixedAt": 837, "ranges": [[504, 531]]},
                "y": {"fixedAt": 520, "ranges": [[822, 851]]}
            }
        }),
        "positioned-escape" => json!({
            "overflowX": "clip",
            "overflowY": "clip",
            "overflowClipMargin": "0px",
            "target": {"x": 114, "y": 968, "width": 42, "height": 30, "right": 156, "bottom": 998},
            "hitRanges": {
                "x": {"fixedAt": 983, "ranges": [[114, 155]]},
                "y": {"fixedAt": 127, "ranges": [[968, 997]]}
            }
        }),
        "nested-clip" => json!({
            "overflowX": "clip",
            "overflowY": "clip",
            "overflowClipMargin": "0px",
            "target": {"x": 253, "y": 961, "width": 140, "height": 120, "right": 393, "bottom": 1081},
            "hitRanges": {
                "x": {"fixedAt": 990.5, "ranges": [[253, 323]]},
                "y": {"fixedAt": 288.5, "ranges": [[961, 1015]]}
            }
        }),
        _ => panic!("unexpected Chrome runtime matrix case {case_id}"),
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
            "pinned Chrome runtime matrix case {case_id} {field}"
        );
    }
    case
}
pub(super) fn assert_frame_matches_observation(
    scene: &RuntimeRenderSnapshot,
    node: NodeId,
    case_id: &str,
    case: &Value,
) {
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
pub(super) fn assert_hit_ranges_match_observation(
    scene: &RuntimeRenderSnapshot,
    node: NodeId,
    case_id: &str,
    case: &Value,
) {
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

#[path = "runtime_matrix/axis_margin.rs"]
mod axis_margin;
#[path = "runtime_matrix/complex.rs"]
mod complex;
