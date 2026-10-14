use serde_json::Value;
use spinon_render::RuntimeRenderSnapshot;
use spinon_runtime::RuntimeLayoutFrame;
use std::collections::{HashMap, HashSet};

mod hit_identity_grid;

const CHROME_ORACLE: &str = include_str!(
    "../../../../../spec/internal/evidence/c13-1-compact-surface-matrix-2026-10-14/chrome155-observation.json"
);
const EXPECTED_CASE_IDS: [&str; 13] = [
    "visible",
    "clip-x",
    "clip-y",
    "clip-both",
    "clip-margin-padding",
    "clip-margin-content",
    "clip-margin-border",
    "clip-margin-hidden",
    "mixed-visible-hidden",
    "mixed-clip-auto",
    "positioned-contained",
    "positioned-escape",
    "nested-clip",
];

#[derive(Clone, Debug)]
struct ExpectedFrame {
    id: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

pub(crate) fn hit_test_samples_json(
    scene: &RuntimeRenderSnapshot,
    frames: &[RuntimeLayoutFrame],
) -> Result<String, String> {
    let oracle: Value = serde_json::from_str(CHROME_ORACLE)
        .map_err(|error| format!("compact C13.1 Chrome oracle JSON 읽기 실패: {error}"))?;
    validate_oracle_metadata(&oracle)?;
    let viewport = scene.viewport_css_px();
    if viewport.width() != 300.0 || viewport.height() != 100.0 {
        return Err(format!(
            "compact C13.1 viewport가 다릅니다: actual={}x{} expected=300x100",
            viewport.width(),
            viewport.height()
        ));
    }

    let expected_frames = expected_frames(&oracle)?;
    if frames.len() != 41 || scene.boxes().len() != 41 || expected_frames.len() != 41 {
        return Err(format!(
            "compact C13.1 frame 수가 다릅니다: layout={} scene={} oracle={}",
            frames.len(),
            scene.boxes().len(),
            expected_frames.len()
        ));
    }
    let actual_node_ids = frames
        .iter()
        .map(|frame| frame.node_id)
        .collect::<HashSet<_>>();
    if actual_node_ids.len() != frames.len() {
        return Err("compact C13.1 layout node ID가 중복되었습니다".to_owned());
    }
    let scene_node_ids = scene
        .boxes()
        .iter()
        .map(|render_box| render_box.node_id().get())
        .collect::<HashSet<_>>();
    if scene_node_ids.len() != scene.boxes().len() || scene_node_ids != actual_node_ids {
        return Err("compact C13.1 render box와 layout frame node 집합이 다릅니다".to_owned());
    }

    for (index, expected) in expected_frames.iter().enumerate() {
        let actual = frames[index];
        let differences = [
            (actual.x as f64 - expected.x).abs(),
            (actual.y as f64 - expected.y).abs(),
            (actual.width as f64 - expected.width).abs(),
            (actual.height as f64 - expected.height).abs(),
        ];
        if differences.into_iter().any(|difference| difference > 0.5) {
            return Err(format!(
                "compact C13.1 frame {} index={} actual=({}, {}, {}, {}) expected=({}, {}, {}, {})",
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

    let frame_indices = expected_frames
        .iter()
        .enumerate()
        .map(|(index, frame)| (frame.id.as_str(), index))
        .collect::<HashMap<_, _>>();
    let cases = oracle["cases"]
        .as_array()
        .ok_or_else(|| "compact C13.1 Chrome oracle case 목록이 없습니다".to_owned())?;
    let mut observations = Vec::with_capacity(52);
    let mut sample_ids = HashSet::with_capacity(52);
    let mut coordinates = HashSet::with_capacity(52);
    for case in cases {
        let case_id = required_string(case, "id", "case ID")?;
        let samples = case["samples"]
            .as_array()
            .filter(|samples| samples.len() == 4)
            .ok_or_else(|| format!("compact C13.1 case {case_id}는 표본 4개가 아닙니다"))?;
        for sample in samples {
            let role = required_string(sample, "role", "sample role")?;
            let id = format!("{case_id}:{role}");
            if !sample_ids.insert(id.clone()) {
                return Err(format!("compact C13.1 sample ID가 중복되었습니다: {id}"));
            }
            let x = required_number(sample, "x", &id)?;
            let y = required_number(sample, "y", &id)?;
            if !(0.0..300.0).contains(&x) || !(0.0..100.0).contains(&y) {
                return Err(format!(
                    "compact C13.1 sample {id}가 viewport 밖입니다: ({x}, {y})"
                ));
            }
            if !coordinates.insert((x as i32, y as i32)) {
                return Err(format!(
                    "compact C13.1 sample 좌표가 중복되었습니다: ({x}, {y})"
                ));
            }
            let element_id = required_string(sample, "hitId", &id)?;
            let expected_node_index = frame_indices.get(element_id).copied().ok_or_else(|| {
                format!(
                    "compact C13.1 sample {id}의 oracle hit node가 frame에 없습니다: {element_id}"
                )
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
                    "compact C13.1 sample {id} hit index={actual_node_index:?}, oracle index={expected_node_index} ({element_id})"
                ));
            }
            let rgba = oracle_rgba(&sample["rgba"], &id)?;
            observations.push(serde_json::json!({
                "id": id,
                "caseId": case_id,
                "role": role,
                "xCssPx": x,
                "yCssPx": y,
                "elementId": element_id,
                "nodeIndex": actual_node_index,
                "rgba": rgba,
                "status": "matched",
            }));
        }
    }
    if observations.len() != 52 || sample_ids.len() != 52 || coordinates.len() != 52 {
        return Err(format!(
            "compact C13.1 sample 수가 다릅니다: output={} ids={} coordinates={}",
            observations.len(),
            sample_ids.len(),
            coordinates.len()
        ));
    }
    serde_json::to_string(&observations)
        .map_err(|error| format!("compact C13.1 hit sample JSON 변환 실패: {error}"))
}

pub(crate) fn hit_identity_grid_summary_json(
    scene: &RuntimeRenderSnapshot,
    frames: &[RuntimeLayoutFrame],
) -> Result<String, String> {
    hit_identity_grid::summary_json(scene, frames)
}

fn validate_oracle_metadata(oracle: &Value) -> Result<(), String> {
    if oracle["browser"]["chromeVersion"].as_str() != Some("Google Chrome 155.0.8059.40")
        || oracle["browser"]["revision"].as_str()
            != Some("cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}")
        || !oracle["browser"]["userAgent"]
            .as_str()
            .is_some_and(|value| value.contains("HeadlessChrome/155.0.0.0"))
    {
        return Err("compact C13.1 Chrome oracle version/revision이 고정값과 다릅니다".to_owned());
    }
    if oracle["viewport"]["width"].as_u64() != Some(300)
        || oracle["viewport"]["height"].as_u64() != Some(100)
        || oracle["viewport"]["devicePixelRatio"].as_u64() != Some(1)
        || oracle["root"]["x"].as_f64() != Some(0.0)
        || oracle["root"]["y"].as_f64() != Some(0.0)
        || oracle["root"]["width"].as_f64() != Some(300.0)
        || oracle["root"]["height"].as_f64() != Some(100.0)
        || oracle["nodeCount"].as_u64() != Some(41)
        || oracle["sampleCount"].as_u64() != Some(52)
    {
        return Err("compact C13.1 Chrome oracle viewport/root/count가 다릅니다".to_owned());
    }
    let actual_case_ids = oracle["caseIds"]
        .as_array()
        .map(|ids| ids.iter().filter_map(Value::as_str).collect::<Vec<_>>())
        .ok_or_else(|| "compact C13.1 Chrome oracle ID 목록이 없습니다".to_owned())?;
    if actual_case_ids != EXPECTED_CASE_IDS {
        return Err("compact C13.1 Chrome oracle case ID 목록이 다릅니다".to_owned());
    }
    Ok(())
}

fn expected_frames(oracle: &Value) -> Result<Vec<ExpectedFrame>, String> {
    let mut frames = Vec::with_capacity(41);
    frames.push(frame_from_rect("c131-surface-root", &oracle["root"])?);
    let cases = oracle["cases"]
        .as_array()
        .filter(|cases| cases.len() == EXPECTED_CASE_IDS.len())
        .ok_or_else(|| "compact C13.1 Chrome oracle frame case 수가 다릅니다".to_owned())?;
    for (case, expected_id) in cases.iter().zip(EXPECTED_CASE_IDS) {
        let case_id = required_string(case, "id", "case ID")?;
        if case_id != expected_id {
            return Err(format!(
                "compact C13.1 Chrome oracle case 순서가 다릅니다: {case_id}"
            ));
        }
        let frame = &case["frame"];
        frames.push(frame_from_rect(
            &format!("c131-surface-case-{case_id}"),
            &frame["section"],
        )?);
        frames.push(frame_from_rect(
            &format!("c131-surface-box-{case_id}"),
            &frame["box"],
        )?);
        if frame["inner"].is_object() {
            frames.push(frame_from_rect(
                "c131-surface-inner-nested-clip",
                &frame["inner"],
            )?);
        }
        frames.push(frame_from_rect(
            &format!("c131-surface-target-{case_id}"),
            &frame["target"],
        )?);
    }
    Ok(frames)
}

fn frame_from_rect(id: &str, rect: &Value) -> Result<ExpectedFrame, String> {
    Ok(ExpectedFrame {
        id: id.to_owned(),
        x: required_number(rect, "x", id)?,
        y: required_number(rect, "y", id)?,
        width: required_number(rect, "width", id)?,
        height: required_number(rect, "height", id)?,
    })
}

fn required_string<'a>(value: &'a Value, key: &str, owner: &str) -> Result<&'a str, String> {
    value[key]
        .as_str()
        .ok_or_else(|| format!("compact C13.1 {owner} {key}가 없습니다"))
}

fn required_number(value: &Value, key: &str, owner: &str) -> Result<f64, String> {
    value[key]
        .as_f64()
        .filter(|number| number.is_finite())
        .ok_or_else(|| format!("compact C13.1 {owner} {key}가 유효한 숫자가 아닙니다"))
}

fn oracle_rgba(value: &Value, sample_id: &str) -> Result<String, String> {
    let channels = value
        .as_array()
        .filter(|channels| channels.len() == 4)
        .ok_or_else(|| format!("compact C13.1 sample {sample_id} RGBA가 없습니다"))?;
    let channels = channels
        .iter()
        .map(|channel| {
            channel
                .as_u64()
                .filter(|value| *value <= 255)
                .map(|value| value.to_string())
                .ok_or_else(|| format!("compact C13.1 sample {sample_id} RGBA 값이 잘못되었습니다"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(channels.join(","))
}

#[cfg(test)]
mod tests {
    use super::{CHROME_ORACLE, EXPECTED_CASE_IDS, expected_frames, validate_oracle_metadata};
    use serde_json::Value;
    use std::collections::HashSet;

    #[test]
    fn pinned_oracle_has_41_ordered_frames_and_52_unique_hit_samples() {
        let oracle: Value = serde_json::from_str(CHROME_ORACLE).unwrap();
        validate_oracle_metadata(&oracle).unwrap();
        let frames = expected_frames(&oracle).unwrap();
        assert_eq!(frames.len(), 41);
        assert_eq!(frames.first().unwrap().id, "c131-surface-root");
        assert_eq!(frames.last().unwrap().id, "c131-surface-target-nested-clip");
        assert_eq!(
            oracle["cases"]
                .as_array()
                .unwrap()
                .iter()
                .filter_map(|case| case["id"].as_str())
                .collect::<Vec<_>>(),
            EXPECTED_CASE_IDS
        );

        let frame_ids = frames
            .iter()
            .map(|frame| frame.id.as_str())
            .collect::<HashSet<_>>();
        let mut sample_ids = HashSet::new();
        let mut coordinates = HashSet::new();
        for case in oracle["cases"].as_array().unwrap() {
            let case_id = case["id"].as_str().unwrap();
            for sample in case["samples"].as_array().unwrap() {
                let role = sample["role"].as_str().unwrap();
                assert!(sample_ids.insert(format!("{case_id}:{role}")));
                let coordinate = (sample["x"].as_u64().unwrap(), sample["y"].as_u64().unwrap());
                assert!(coordinates.insert(coordinate));
                assert!(frame_ids.contains(sample["hitId"].as_str().unwrap()));
                assert_eq!(sample["rgba"].as_array().unwrap().len(), 4);
            }
        }
        assert_eq!(sample_ids.len(), 52);
        assert_eq!(coordinates.len(), 52);
    }

    #[test]
    fn oracle_metadata_rejects_viewport_and_chrome_revision_mutations() {
        let original: Value = serde_json::from_str(CHROME_ORACLE).unwrap();
        let mut wrong_viewport = original.clone();
        wrong_viewport["viewport"]["width"] = Value::from(320);
        assert!(validate_oracle_metadata(&wrong_viewport).is_err());
        let mut wrong_revision = original;
        wrong_revision["browser"]["revision"] = Value::from("changed");
        assert!(validate_oracle_metadata(&wrong_revision).is_err());
    }
}
