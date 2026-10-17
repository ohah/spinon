use serde_json::Value;
use sha2::{Digest, Sha256};
use spinon_render::RuntimeRenderSnapshot;
use spinon_runtime::RuntimeLayoutFrame;
use std::collections::HashSet;

const HTML_FIXTURE: &str = include_str!(
    "../../../../../tests/fixtures/css/c13/overflow-clip-stacking-descendants-c13-1.html"
);
const CHROME_OBSERVATION: &str = include_str!(
    "../../../../../spec/internal/evidence/c13-1-stacking-descendant-clip-plan-2026-10-18/chrome-observation.json"
);
const PAINT_HIT_SAMPLES: &str = include_str!(
    "../../../../../spec/internal/evidence/c13-1-stacking-descendant-clip-plan-2026-10-18/chrome-paint-hit-samples.json"
);
const SCREENSHOT: &[u8] = include_bytes!(
    "../../../../../spec/internal/evidence/c13-1-stacking-descendant-clip-plan-2026-10-18/chrome155-nested-stacking-descendants.png"
);
const EXPECTED_FIXTURE_SHA256: &str =
    "6b691ddf5afd44543b438b47200f9a6f85981fa3d3c30e1619185a65d320791e";
const EXPECTED_SCREENSHOT_SHA256: &str =
    "a9727d516837df6c8a5d77e624aee3463b08ebfb991d8eb622d52d026c64585c";
const EXPECTED_OBSERVATION_SHA256: &str =
    "8d33e5a9acb674eb4927f75615d766612167b8026d2d0093280011ea25d2a633";
const EXPECTED_SAMPLE_JSON_SHA256: &str =
    "9c15d0ab584fef0135f3be3e6942eee23a0edf71656f8d66984072a68855271a";
const EXPECTED_CASES: [&str; 3] = ["visible", "clip", "hidden"];
const EXPECTED_POINT_NAMES: [&str; 6] = [
    "parent-only",
    "target-inside",
    "clip-edge-inside",
    "clip-edge-outside",
    "escape-before-overlay",
    "stacking-overlay",
];

#[derive(Clone, Copy)]
struct ExpectedFrame {
    id: &'static str,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

const EXPECTED_FRAMES: [ExpectedFrame; 13] = [
    ExpectedFrame {
        id: "root",
        x: 0.0,
        y: 0.0,
        width: 540.0,
        height: 128.0,
    },
    ExpectedFrame {
        id: "visible-parent",
        x: 20.0,
        y: 28.0,
        width: 120.0,
        height: 72.0,
    },
    ExpectedFrame {
        id: "visible-layer",
        x: 76.0,
        y: 40.0,
        width: 80.0,
        height: 48.0,
    },
    ExpectedFrame {
        id: "visible-target",
        x: 108.0,
        y: 40.0,
        width: 72.0,
        height: 48.0,
    },
    ExpectedFrame {
        id: "visible-overlay",
        x: 148.0,
        y: 40.0,
        width: 16.0,
        height: 48.0,
    },
    ExpectedFrame {
        id: "clip-parent",
        x: 190.0,
        y: 28.0,
        width: 120.0,
        height: 72.0,
    },
    ExpectedFrame {
        id: "clip-layer",
        x: 246.0,
        y: 40.0,
        width: 80.0,
        height: 48.0,
    },
    ExpectedFrame {
        id: "clip-target",
        x: 278.0,
        y: 40.0,
        width: 72.0,
        height: 48.0,
    },
    ExpectedFrame {
        id: "clip-overlay",
        x: 318.0,
        y: 40.0,
        width: 16.0,
        height: 48.0,
    },
    ExpectedFrame {
        id: "hidden-parent",
        x: 360.0,
        y: 28.0,
        width: 120.0,
        height: 72.0,
    },
    ExpectedFrame {
        id: "hidden-layer",
        x: 416.0,
        y: 40.0,
        width: 80.0,
        height: 48.0,
    },
    ExpectedFrame {
        id: "hidden-target",
        x: 448.0,
        y: 40.0,
        width: 72.0,
        height: 48.0,
    },
    ExpectedFrame {
        id: "hidden-overlay",
        x: 488.0,
        y: 40.0,
        width: 16.0,
        height: 48.0,
    },
];

pub(crate) fn hit_test_samples_json(
    scene: &RuntimeRenderSnapshot,
    frames: &[RuntimeLayoutFrame],
) -> Result<String, String> {
    let observation: Value = serde_json::from_str(CHROME_OBSERVATION).map_err(|error| {
        format!("C13.1 stacking clip Chrome observation JSON 읽기 실패: {error}")
    })?;
    let samples_oracle: Value = serde_json::from_str(PAINT_HIT_SAMPLES)
        .map_err(|error| format!("C13.1 stacking clip paint/hit JSON 읽기 실패: {error}"))?;
    validate_pinned_inputs(&observation, &samples_oracle)?;
    validate_frames(scene, frames)?;

    let samples = samples_oracle["samples"]
        .as_array()
        .filter(|samples| samples.len() == 18)
        .ok_or_else(|| "C13.1 stacking clip oracle 표본 수가 18이 아닙니다".to_owned())?;
    let expected_ids = EXPECTED_FRAMES
        .iter()
        .enumerate()
        .map(|(index, frame)| (frame.id, index))
        .collect::<std::collections::HashMap<_, _>>();
    let mut seen_ids = HashSet::with_capacity(18);
    let mut seen_pixels = HashSet::with_capacity(18);
    let mut result = Vec::with_capacity(18);

    for (index, sample) in samples.iter().enumerate() {
        let case_id = required_string(sample, "case", index)?;
        let point_name = required_string(sample, "name", index)?;
        let sample_id = format!("{case_id}:{point_name}");
        if case_id != EXPECTED_CASES[index / 6] || point_name != EXPECTED_POINT_NAMES[index % 6] {
            return Err(format!(
                "C13.1 stacking clip sample 순서가 잘못되었습니다: {sample_id}"
            ));
        }
        if !seen_ids.insert(sample_id.clone()) {
            return Err(format!(
                "C13.1 stacking clip sample ID가 중복되었습니다: {sample_id}"
            ));
        }
        let x = required_number(sample, "xCssPx", &sample_id)?;
        let y = required_number(sample, "yCssPx", &sample_id)?;
        let pixel_x = x.floor() as u32;
        let pixel_y = y.floor() as u32;
        if x.fract() != 0.5 || y.fract() != 0.5 || pixel_x >= 540 || pixel_y >= 128 {
            return Err(format!(
                "C13.1 stacking clip sample 좌표가 고정 CSS pixel center가 아닙니다: {sample_id} ({x}, {y})"
            ));
        }
        if !seen_pixels.insert((pixel_x, pixel_y)) {
            return Err(format!(
                "C13.1 stacking clip pixel 좌표가 중복되었습니다: {sample_id} ({pixel_x}, {pixel_y})"
            ));
        }
        let element_id = required_string(sample, "hitId", index)?;
        let expected_node_index = *expected_ids.get(element_id.as_str()).ok_or_else(|| {
            format!("C13.1 stacking clip sample {sample_id}의 hit ID가 고정 frame에 없습니다")
        })?;
        let actual_node_index = scene
            .hit_test_css_point(x as f32, y as f32)
            .and_then(|hit| {
                frames
                    .iter()
                    .position(|frame| frame.node_id == hit.node_id().get())
            });
        if actual_node_index != Some(expected_node_index) {
            return Err(format!(
                "C13.1 stacking clip sample {sample_id} hit frame={actual_node_index:?}, expected={expected_node_index} ({element_id})"
            ));
        }
        let rgba = required_rgba(sample, &sample_id)?;
        result.push(serde_json::json!({
            "id": sample_id,
            "caseId": case_id,
            "role": point_name,
            "xCssPx": x,
            "yCssPx": y,
            "pixelX": pixel_x,
            "pixelY": pixel_y,
            "elementId": element_id,
            "nodeIndex": actual_node_index,
            "rgba": rgba,
            "status": "matched",
        }));
    }

    if seen_ids.len() != 18 || seen_pixels.len() != 18 {
        return Err("C13.1 stacking clip 표본 inventory가 불완전합니다".to_owned());
    }
    serde_json::to_string(&result)
        .map_err(|error| format!("C13.1 stacking clip hit sample JSON 변환 실패: {error}"))
}

fn validate_pinned_inputs(observation: &Value, samples: &Value) -> Result<(), String> {
    let viewport = &observation["viewport"];
    if viewport["width"].as_u64() != Some(540)
        || viewport["height"].as_u64() != Some(128)
        || viewport["devicePixelRatio"].as_u64() != Some(1)
        || viewport["scrollX"].as_i64() != Some(0)
        || viewport["scrollY"].as_i64() != Some(0)
        || samples["browserVersion"].as_str() != Some("155.0.8059.85")
        || samples["fixtureSha256"].as_str() != Some(EXPECTED_FIXTURE_SHA256)
        || samples["sampleCount"].as_u64() != Some(18)
        || samples["screenshot"]["width"].as_u64() != Some(540)
        || samples["screenshot"]["height"].as_u64() != Some(128)
        || samples["screenshot"]["sha256"].as_str() != Some(EXPECTED_SCREENSHOT_SHA256)
        || sha256_hex(HTML_FIXTURE.as_bytes()) != EXPECTED_FIXTURE_SHA256
        || sha256_hex(CHROME_OBSERVATION.as_bytes()) != EXPECTED_OBSERVATION_SHA256
        || sha256_hex(PAINT_HIT_SAMPLES.as_bytes()) != EXPECTED_SAMPLE_JSON_SHA256
        || sha256_hex(SCREENSHOT) != EXPECTED_SCREENSHOT_SHA256
    {
        return Err("C13.1 stacking clip pinned Chrome 입력·hash가 일치하지 않습니다".to_owned());
    }
    let cases = observation["cases"]
        .as_array()
        .filter(|cases| cases.len() == EXPECTED_CASES.len())
        .ok_or_else(|| "C13.1 stacking clip Chrome case inventory가 3개가 아닙니다".to_owned())?;
    let pinned_samples = samples["samples"]
        .as_array()
        .filter(|samples| samples.len() == 18)
        .ok_or_else(|| {
            "C13.1 stacking clip Chrome sample inventory가 18개가 아닙니다".to_owned()
        })?;
    for (index, case) in cases.iter().enumerate() {
        if case["id"].as_str() != Some(EXPECTED_CASES[index]) {
            return Err(format!(
                "C13.1 stacking clip Chrome case 순서가 잘못되었습니다: {index}"
            ));
        }
        let expected_overflow = match EXPECTED_CASES[index] {
            "visible" => "visible",
            "clip" => "clip",
            "hidden" => "hidden",
            _ => unreachable!(),
        };
        if case["overflowX"].as_str() != Some(expected_overflow)
            || case["overflowY"].as_str() != Some(expected_overflow)
            || case["zIndex"]["parent"].as_str() != Some("0")
            || case["zIndex"]["layer"].as_str() != Some("900")
            || case["zIndex"]["target"].as_str() != Some("10")
            || case["zIndex"]["overlay"].as_str() != Some("1")
        {
            return Err(format!(
                "C13.1 stacking clip Chrome case metadata 불일치: {expected_overflow}"
            ));
        }
        let points = case["points"]
            .as_array()
            .filter(|points| points.len() == EXPECTED_POINT_NAMES.len())
            .ok_or_else(|| {
                format!("C13.1 stacking clip Chrome points 수가 다릅니다: {expected_overflow}")
            })?;
        for (point_index, point) in points.iter().enumerate() {
            if point["name"].as_str() != Some(EXPECTED_POINT_NAMES[point_index]) {
                return Err(format!(
                    "C13.1 stacking clip Chrome point 순서가 잘못되었습니다: {expected_overflow}"
                ));
            }
            let sample_index = index * EXPECTED_POINT_NAMES.len() + point_index;
            let sample = &pinned_samples[sample_index];
            if sample["case"].as_str() != Some(expected_overflow)
                || sample["name"].as_str() != Some(EXPECTED_POINT_NAMES[point_index])
                || point["hit"].as_str() != sample["hitId"].as_str()
                || point["x"].as_f64() != sample["xCssPx"].as_f64()
                || point["y"].as_f64() != sample["yCssPx"].as_f64()
            {
                return Err(format!(
                    "C13.1 stacking clip Chrome observation/sample JSON 불일치: {}:{}",
                    expected_overflow, EXPECTED_POINT_NAMES[point_index]
                ));
            }
            let expected_hit_id = match EXPECTED_POINT_NAMES[point_index] {
                "parent-only" => format!("{expected_overflow}-parent"),
                "target-inside" | "clip-edge-inside" => format!("{expected_overflow}-target"),
                "clip-edge-outside" | "escape-before-overlay" if expected_overflow == "visible" => {
                    "visible-target".to_owned()
                }
                "clip-edge-outside" | "escape-before-overlay" => "root".to_owned(),
                "stacking-overlay" => format!("{expected_overflow}-overlay"),
                _ => unreachable!(),
            };
            let expected_rgba = match expected_hit_id.as_str() {
                "root" => [240, 244, 247, 255],
                id if id.ends_with("-parent") => [240, 198, 116, 255],
                id if id.ends_with("-target") => [87, 166, 217, 255],
                id if id.ends_with("-overlay") => [228, 81, 79, 255],
                _ => unreachable!(),
            };
            if sample["hitId"].as_str() != Some(expected_hit_id.as_str())
                || required_rgba(
                    sample,
                    &format!("{expected_overflow}:{}", EXPECTED_POINT_NAMES[point_index]),
                )? != expected_rgba
            {
                return Err(format!(
                    "C13.1 stacking clip expected hit/color control 불일치: {}:{}",
                    expected_overflow, EXPECTED_POINT_NAMES[point_index]
                ));
            }
        }
    }
    Ok(())
}

fn validate_frames(
    scene: &RuntimeRenderSnapshot,
    frames: &[RuntimeLayoutFrame],
) -> Result<(), String> {
    if scene.viewport_css_px().width() != 540.0 || scene.viewport_css_px().height() != 128.0 {
        return Err(format!(
            "C13.1 stacking clip viewport가 다릅니다: actual={}x{} expected=540x128",
            scene.viewport_css_px().width(),
            scene.viewport_css_px().height()
        ));
    }
    if frames.len() != EXPECTED_FRAMES.len() || scene.boxes().len() != EXPECTED_FRAMES.len() {
        return Err(format!(
            "C13.1 stacking clip frame 수가 다릅니다: frames={} boxes={} expected={}",
            frames.len(),
            scene.boxes().len(),
            EXPECTED_FRAMES.len()
        ));
    }
    let frame_node_ids = frames
        .iter()
        .map(|frame| frame.node_id)
        .collect::<HashSet<_>>();
    let box_node_ids = scene
        .boxes()
        .iter()
        .map(|render_box| render_box.node_id().get())
        .collect::<HashSet<_>>();
    if frame_node_ids.len() != frames.len() || frame_node_ids != box_node_ids {
        return Err("C13.1 stacking clip frame/render box node inventory가 다릅니다".to_owned());
    }
    for (index, expected) in EXPECTED_FRAMES.iter().enumerate() {
        let actual = frames[index];
        let delta = [
            (actual.x as f64 - expected.x).abs(),
            (actual.y as f64 - expected.y).abs(),
            (actual.width as f64 - expected.width).abs(),
            (actual.height as f64 - expected.height).abs(),
        ];
        if delta.into_iter().any(|difference| difference > 0.5) {
            return Err(format!(
                "C13.1 stacking clip frame {} index={} actual=({}, {}, {}, {}) expected=({}, {}, {}, {})",
                expected.id,
                index,
                actual.x,
                actual.y,
                actual.width,
                actual.height,
                expected.x,
                expected.y,
                expected.width,
                expected.height
            ));
        }
    }
    Ok(())
}

fn required_string(value: &Value, field: &str, index: usize) -> Result<String, String> {
    value[field]
        .as_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| format!("C13.1 stacking clip sample {index}에 {field}가 없습니다"))
}

fn required_number(value: &Value, field: &str, id: &str) -> Result<f64, String> {
    value[field]
        .as_f64()
        .filter(|number| number.is_finite())
        .ok_or_else(|| format!("C13.1 stacking clip sample {id}에 유한한 {field}가 없습니다"))
}

fn required_rgba(value: &Value, id: &str) -> Result<[u8; 4], String> {
    let channels = value["rgba"]
        .as_array()
        .filter(|channels| channels.len() == 4)
        .ok_or_else(|| format!("C13.1 stacking clip sample {id} RGBA가 잘못되었습니다"))?;
    let mut rgba = [0u8; 4];
    for (index, channel) in channels.iter().enumerate() {
        rgba[index] = channel
            .as_u64()
            .and_then(|value| u8::try_from(value).ok())
            .ok_or_else(|| {
                format!("C13.1 stacking clip sample {id} RGBA channel이 잘못되었습니다")
            })?;
    }
    Ok(rgba)
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{CHROME_OBSERVATION, PAINT_HIT_SAMPLES, Value, validate_pinned_inputs};

    #[test]
    fn chrome_oracle_rejects_case_point_sample_order_and_identity_mutations() {
        let observation: Value = serde_json::from_str(CHROME_OBSERVATION).unwrap();
        let samples: Value = serde_json::from_str(PAINT_HIT_SAMPLES).unwrap();
        validate_pinned_inputs(&observation, &samples).expect("pinned Chrome data is valid");

        let mut reordered_cases = observation.clone();
        reordered_cases["cases"].as_array_mut().unwrap().swap(0, 1);
        assert!(validate_pinned_inputs(&reordered_cases, &samples).is_err());

        let mut reordered_points = observation.clone();
        reordered_points["cases"][0]["points"]
            .as_array_mut()
            .unwrap()
            .swap(0, 1);
        assert!(validate_pinned_inputs(&reordered_points, &samples).is_err());

        let mut reordered_samples = samples.clone();
        reordered_samples["samples"]
            .as_array_mut()
            .unwrap()
            .swap(0, 1);
        assert!(validate_pinned_inputs(&observation, &reordered_samples).is_err());

        let mut missing = samples.clone();
        missing["samples"].as_array_mut().unwrap().pop();
        assert!(validate_pinned_inputs(&observation, &missing).is_err());

        let mut duplicate = samples.clone();
        duplicate["samples"][1]["name"] = Value::from("parent-only");
        assert!(validate_pinned_inputs(&observation, &duplicate).is_err());

        let mut wrong_target_identity = samples.clone();
        wrong_target_identity["samples"][1]["hitId"] = Value::from("root");
        assert!(validate_pinned_inputs(&observation, &wrong_target_identity).is_err());

        let mut wrong_overlay_identity = samples.clone();
        wrong_overlay_identity["samples"][5]["hitId"] = Value::from("visible-target");
        assert!(validate_pinned_inputs(&observation, &wrong_overlay_identity).is_err());
    }

    #[test]
    fn chrome_oracle_rejects_one_byte_mutations_in_each_rgba_channel() {
        let observation: Value = serde_json::from_str(CHROME_OBSERVATION).unwrap();
        let samples: Value = serde_json::from_str(PAINT_HIT_SAMPLES).unwrap();
        validate_pinned_inputs(&observation, &samples).expect("pinned Chrome data is valid");

        for channel in 0..4 {
            let mut changed_channel = samples.clone();
            let value = changed_channel["samples"][0]["rgba"][channel]
                .as_u64()
                .unwrap();
            changed_channel["samples"][0]["rgba"][channel] = Value::from(value ^ 1);
            assert!(validate_pinned_inputs(&observation, &changed_channel).is_err());
        }
    }
}
