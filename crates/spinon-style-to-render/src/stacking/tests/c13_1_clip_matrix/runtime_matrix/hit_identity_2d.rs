use super::{EXPECTED_USER_AGENT, NodeId, target};
use serde_json::Value;
use spinon_render::RuntimeRenderSnapshot;

const CHROME_HIT_IDENTITY_2D_OBSERVATION: &str = include_str!(
    "../../../../../../../spec/internal/evidence/c13-1-overflow-clip-hit-identity-2d-2026-10-14/chrome155-target-frame-hit-identity-2d.json"
);

pub(super) fn assert_matches_chrome(
    scene: &RuntimeRenderSnapshot,
    target_node: NodeId,
    root_node: NodeId,
    clip_node: NodeId,
    nested_inner_node: Option<NodeId>,
    case_id: &str,
) {
    let observation: Value = serde_json::from_str(CHROME_HIT_IDENTITY_2D_OBSERVATION)
        .expect("agent-browser Chrome 155 target-frame 2D hit identity observation JSON");
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
        observation["browser"]["viewport"]["width"].as_u64(),
        Some(1280)
    );
    assert_eq!(
        observation["browser"]["viewport"]["height"].as_u64(),
        Some(1100)
    );
    assert_eq!(
        observation["browser"]["viewport"]["dpr"].as_f64(),
        Some(1.0)
    );
    assert_eq!(
        observation["fixture"]["sha256"].as_str(),
        Some("b7acd7e63f79355b28cd480511b9be82e0f2bef478f1aadf685eb3b0bf4ef16a")
    );
    assert_eq!(
        observation["fixture"]["parentHitIdentitySha256"].as_str(),
        Some("55682451470f701400ad29f0d86494a0811207fc589ec71b1e62d1a921186db5")
    );
    assert_eq!(
        observation["scan"]["mode"].as_str(),
        Some("target-frame-integer-grid-2d")
    );
    assert_eq!(observation["scan"]["caseCount"].as_u64(), Some(13));
    assert_eq!(observation["scan"]["pointCount"].as_u64(), Some(187_320));

    let cases = observation["cases"]
        .as_array()
        .expect("fixed Chrome 2D hit identity cases");
    assert_eq!(cases.len(), 13, "Chrome 2D identity row count");
    let case = cases
        .iter()
        .find(|case| case["id"].as_str() == Some(case_id))
        .unwrap_or_else(|| panic!("missing Chrome 2D hit identity case {case_id}"));
    assert_eq!(
        case["targetId"].as_str(),
        Some(format!("{case_id}-target").as_str()),
        "Chrome fixture target identity for {case_id}"
    );

    let render_box = target(scene, target_node);
    let frame = render_box.frame_css_px();
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
            "Chrome {case_id} target frame {field}: runtime={actual}, oracle={expected}"
        );
    }

    let x_start = case["xStart"]
        .as_i64()
        .unwrap_or_else(|| panic!("Chrome {case_id} xStart")) as i32;
    let x_end = case["xEnd"]
        .as_i64()
        .unwrap_or_else(|| panic!("Chrome {case_id} xEnd")) as i32;
    let y_start = case["yStart"]
        .as_i64()
        .unwrap_or_else(|| panic!("Chrome {case_id} yStart")) as i32;
    let y_end = case["yEnd"]
        .as_i64()
        .unwrap_or_else(|| panic!("Chrome {case_id} yEnd")) as i32;
    assert!(x_end >= x_start, "Chrome {case_id} inverted x range");
    assert!(y_end >= y_start, "Chrome {case_id} inverted y range");
    assert_eq!(
        x_start,
        case["target"]["x"].as_f64().unwrap().floor() as i32,
        "Chrome {case_id} x origin"
    );
    assert_eq!(
        x_end,
        case["target"]["right"].as_f64().unwrap().ceil() as i32 - 1,
        "Chrome {case_id} x end"
    );
    assert_eq!(
        y_start,
        case["target"]["y"].as_f64().unwrap().floor() as i32,
        "Chrome {case_id} y origin"
    );
    assert_eq!(
        y_end,
        case["target"]["bottom"].as_f64().unwrap().ceil() as i32 - 1,
        "Chrome {case_id} y end"
    );

    let rows = case["rows"]
        .as_array()
        .unwrap_or_else(|| panic!("Chrome {case_id} 2D identity rows"));
    assert_eq!(
        rows.len(),
        (y_end - y_start + 1) as usize,
        "Chrome {case_id} y rows must cover the target frame"
    );
    let mut point_count = 0usize;
    for (row_index, row) in rows.iter().enumerate() {
        let y = y_start + row_index as i32;
        assert_eq!(
            row["y"].as_i64(),
            Some(y as i64),
            "Chrome {case_id} y row gap"
        );
        let runs = row["runs"]
            .as_array()
            .unwrap_or_else(|| panic!("Chrome {case_id} y={y} identity runs"));
        let mut next_x = x_start;
        for run in runs {
            let from = run["from"].as_i64().unwrap() as i32;
            let to = run["to"].as_i64().unwrap() as i32;
            assert_eq!(from, next_x, "Chrome {case_id} y={y} x run gap");
            assert!(to >= from, "Chrome {case_id} y={y} inverted x run");
            let expected_node = match run["hit"].as_str().unwrap() {
                "target" => Some(target_node),
                "box" => Some(clip_node),
                "inner" => Some(nested_inner_node.unwrap_or_else(|| {
                    panic!("Chrome {case_id} reported an unmodeled nested inner hit")
                })),
                "root" | "main" | "body" | "html" => Some(root_node),
                other => panic!("unmodeled Chrome {case_id} hit identity {other}"),
            };
            for x in from..=to {
                let actual_node = scene
                    .hit_test_css_point(x as f32, y as f32)
                    .map(|hit| hit.node_id());
                assert_eq!(
                    actual_node, expected_node,
                    "Chrome {case_id} hit identity at ({x}, {y})"
                );
                point_count += 1;
            }
            next_x = to + 1;
        }
        assert_eq!(
            next_x,
            x_end + 1,
            "Chrome {case_id} y={y} runs end before target frame"
        );
    }
    assert_eq!(
        point_count,
        (x_end - x_start + 1) as usize * (y_end - y_start + 1) as usize,
        "Chrome {case_id} target-frame 2D sample count"
    );
}
