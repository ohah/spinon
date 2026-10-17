use super::{EXPECTED_FRAMES, HTML_FIXTURE, SCREENSHOT, validate_frames};
use serde_json::Value;
use sha2::{Digest, Sha256};
use spinon_render::RuntimeRenderSnapshot;
use spinon_runtime::RuntimeLayoutFrame;

const CHROME_GRID_ORACLE: &str = include_str!(
    "../../../../../../spec/internal/evidence/c13-1-stacking-app-full-viewport-hit-2026-10-18/chrome-full-viewport-hit.json"
);
const GRID_WIDTH: usize = 540;
const GRID_HEIGHT: usize = 128;
const GRID_POINT_COUNT: usize = GRID_WIDTH * GRID_HEIGHT;
const GRID_ROW_COUNT: usize = 128;
const GRID_RUN_COUNT: usize = 1_136;
const NODE_COUNT: usize = 13;
const MISMATCH_PREVIEW_LIMIT: usize = 16;
const MAX_SUMMARY_BYTES: usize = 8_192;

const EXPECTED_NODE_IDS: [&str; NODE_COUNT] = [
    "root",
    "visible-parent",
    "visible-layer",
    "visible-target",
    "visible-overlay",
    "clip-parent",
    "clip-layer",
    "clip-target",
    "clip-overlay",
    "hidden-parent",
    "hidden-layer",
    "hidden-target",
    "hidden-overlay",
];
const EXPECTED_GRID_FILE_SHA256: &str =
    "279364905336ef53d225c781bae68ec26d3fe4d50cfc7d0cfce689ab5354eee2";
const EXPECTED_FIXTURE_SHA256: &str =
    "6b691ddf5afd44543b438b47200f9a6f85981fa3d3c30e1619185a65d320791e";
const EXPECTED_SCREENSHOT_SHA256: &str =
    "a9727d516837df6c8a5d77e624aee3463b08ebfb991d8eb622d52d026c64585c";
const EXPECTED_IDENTITY_SHA256: &str =
    "866073e0c6f092d4b2459fa3b5cfd2ca38dfb8935f582ee3c9614dba3bfc7483";
const EXPECTED_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/155.0.0.0 Safari/537.36";

#[derive(Clone, Debug, PartialEq, Eq)]
struct HitRun {
    start_x: usize,
    end_x: usize,
    node_index: usize,
}

#[derive(Clone, Debug)]
struct ValidatedGrid {
    node_ids: Vec<String>,
    rows: Vec<Vec<HitRun>>,
    identity_sha256: String,
}

#[derive(Clone, Debug)]
struct ActualHit {
    node_index: Option<usize>,
    node_id: Option<String>,
}

#[derive(Clone, Debug)]
struct GridComparison {
    matched_count: usize,
    mismatch_count: usize,
    actual_identity_sha256: String,
    mismatch_preview: Vec<Value>,
}

pub(in crate::runtime_gpu) fn summary_json(
    scene: &RuntimeRenderSnapshot,
    frames: &[RuntimeLayoutFrame],
) -> Result<String, String> {
    let oracle = parse_pinned_oracle(CHROME_GRID_ORACLE)?;
    let grid = validate_grid_oracle(&oracle)?;
    validate_frames(scene, frames)?;
    validate_node_order_contract()?;
    validate_edge_controls(&oracle, scene, frames, &grid.node_ids)?;

    let comparison = compare_grid(&grid, |x, y| {
        let hit = scene.hit_test_css_point(x as f32, y as f32);
        let node_index = hit.and_then(|hit| {
            frames
                .iter()
                .position(|frame| frame.node_id == hit.node_id().get())
        });
        let node_id = match (hit, node_index) {
            (Some(_), Some(index)) => Some(grid.node_ids[index].clone()),
            (Some(hit), None) => Some(format!("unknown-runtime-node-{}", hit.node_id())),
            (None, _) => None,
        };
        ActualHit {
            node_index,
            node_id,
        }
    })?;
    let summary = serde_json::json!({
        "status": if comparison.mismatch_count == 0 { "matched" } else { "mismatch" },
        "point_count": GRID_POINT_COUNT,
        "matched_count": comparison.matched_count,
        "mismatch_count": comparison.mismatch_count,
        "mismatch_preview_limit": MISMATCH_PREVIEW_LIMIT,
        "mismatch_preview": comparison.mismatch_preview,
        "edge_control_count": 11,
        "edge_controls_matched": 11,
        "expected_identity_sha256": grid.identity_sha256,
        "actual_identity_sha256": comparison.actual_identity_sha256,
        "oracle_sha256": EXPECTED_GRID_FILE_SHA256,
    });
    let summary = serde_json::to_string(&summary)
        .map_err(|error| format!("C13.1 stacking full viewport summary JSON 변환 실패: {error}"))?;
    if summary.len() > MAX_SUMMARY_BYTES {
        return Err(format!(
            "C13.1 stacking full viewport summary가 제한을 넘었습니다: bytes={} max={MAX_SUMMARY_BYTES}",
            summary.len()
        ));
    }
    Ok(summary)
}

fn parse_pinned_oracle(source: &str) -> Result<Value, String> {
    if sha256_hex(source.as_bytes()) != EXPECTED_GRID_FILE_SHA256 {
        return Err(
            "C13.1 stacking full viewport oracle 파일 SHA-256이 고정값과 다릅니다".to_owned(),
        );
    }
    serde_json::from_str(source)
        .map_err(|error| format!("C13.1 stacking full viewport oracle JSON 읽기 실패: {error}"))
}

fn validate_grid_oracle(oracle: &Value) -> Result<ValidatedGrid, String> {
    validate_oracle_metadata(oracle)?;
    let node_ids = validate_node_ids(oracle)?;
    let rows = oracle["rows"]
        .as_array()
        .filter(|rows| rows.len() == GRID_ROW_COUNT)
        .ok_or_else(|| "C13.1 stacking full viewport oracle row 수가 128이 아닙니다".to_owned())?;
    let mut decoded_rows = Vec::with_capacity(GRID_ROW_COUNT);
    let mut identity_bytes = Vec::with_capacity(GRID_POINT_COUNT);
    let mut run_count = 0usize;
    for (y, row) in rows.iter().enumerate() {
        let encoded_runs = row
            .as_array()
            .filter(|runs| !runs.is_empty())
            .ok_or_else(|| format!("C13.1 stacking full viewport row {y}에 run이 없습니다"))?;
        let mut decoded_runs = Vec::with_capacity(encoded_runs.len());
        let mut next_x = 0usize;
        let mut previous_node = None;
        for (run_index, encoded_run) in encoded_runs.iter().enumerate() {
            let fields = encoded_run
                .as_array()
                .filter(|fields| fields.len() == 2)
                .ok_or_else(|| {
                    format!("C13.1 stacking full viewport row {y} run {run_index} shape이 잘못되었습니다")
                })?;
            let node_index = as_usize(&fields[0], "node index", y, run_index)?;
            let width = as_usize(&fields[1], "run width", y, run_index)?;
            if node_index >= NODE_COUNT {
                return Err(format!(
                    "C13.1 stacking full viewport row {y} unknown node index={node_index}"
                ));
            }
            if width == 0 {
                return Err(format!(
                    "C13.1 stacking full viewport row {y} run {run_index} 폭이 0입니다"
                ));
            }
            if previous_node == Some(node_index) {
                return Err(format!(
                    "C13.1 stacking full viewport row {y} adjacent run이 canonical 형태가 아닙니다"
                ));
            }
            let end_x = next_x.checked_add(width).ok_or_else(|| {
                format!("C13.1 stacking full viewport row {y} run 폭 계산이 넘쳤습니다")
            })?;
            if end_x > GRID_WIDTH {
                return Err(format!(
                    "C13.1 stacking full viewport row {y} viewport 폭을 초과했습니다: {end_x}"
                ));
            }
            identity_bytes.extend(std::iter::repeat_n(node_index as u8, width));
            decoded_runs.push(HitRun {
                start_x: next_x,
                end_x,
                node_index,
            });
            next_x = end_x;
            previous_node = Some(node_index);
            run_count += 1;
        }
        if next_x != GRID_WIDTH {
            return Err(format!(
                "C13.1 stacking full viewport row {y}가 viewport 폭을 덮지 않습니다: {next_x}"
            ));
        }
        decoded_rows.push(decoded_runs);
    }
    if run_count != GRID_RUN_COUNT || identity_bytes.len() != GRID_POINT_COUNT {
        return Err(format!(
            "C13.1 stacking full viewport expanded count가 다릅니다: runs={run_count} points={}",
            identity_bytes.len()
        ));
    }
    let identity_sha256 = sha256_hex(&identity_bytes);
    if identity_sha256 != EXPECTED_IDENTITY_SHA256
        || oracle["expandedNodeIndexSha256"].as_str() != Some(identity_sha256.as_str())
    {
        return Err("C13.1 stacking full viewport expanded identity SHA-256이 다릅니다".to_owned());
    }
    Ok(ValidatedGrid {
        node_ids,
        rows: decoded_rows,
        identity_sha256,
    })
}

fn validate_oracle_metadata(oracle: &Value) -> Result<(), String> {
    const ORACLE_KEYS: [&str; 13] = [
        "browser",
        "edgeControls",
        "expandedNodeIndexSha256",
        "fixtureSha256",
        "nodeIds",
        "pointCount",
        "queryCoordinateConvention",
        "rowCount",
        "rows",
        "schemaVersion",
        "screenshot",
        "source",
        "viewport",
    ];
    let Some(object) = oracle.as_object() else {
        return Err("C13.1 stacking full viewport oracle root가 object가 아닙니다".to_owned());
    };
    if object.len() != ORACLE_KEYS.len() || ORACLE_KEYS.iter().any(|key| !object.contains_key(*key))
    {
        return Err("C13.1 stacking full viewport oracle schema가 다릅니다".to_owned());
    }
    let expected_viewport = serde_json::json!({
        "width": GRID_WIDTH,
        "height": GRID_HEIGHT,
        "devicePixelRatio": 1,
        "scrollX": 0,
        "scrollY": 0,
    });
    let chrome_versions = oracle["browser"]["fullVersionList"].as_array();
    let has_pinned_chrome = chrome_versions.is_some_and(|versions| {
        versions.iter().any(|version| {
            version["brand"].as_str() == Some("Google Chrome")
                && version["version"].as_str() == Some("155.0.8059.85")
        })
    });
    if oracle["schemaVersion"].as_u64() != Some(1)
        || oracle["source"].as_str()
            != Some("Agent Browser Chrome elementFromPoint at integer CSS coordinates")
        || oracle["fixtureSha256"].as_str() != Some(EXPECTED_FIXTURE_SHA256)
        || oracle["pointCount"].as_u64() != Some(GRID_POINT_COUNT as u64)
        || oracle["rowCount"].as_u64() != Some(GRID_ROW_COUNT as u64)
        || oracle["queryCoordinateConvention"].as_str()
            != Some("integer CSS points (x,y), not pixel centers")
        || oracle["viewport"] != expected_viewport
        || oracle["browser"]["userAgent"].as_str() != Some(EXPECTED_USER_AGENT)
        || oracle["browser"]["platform"].as_str() != Some("macOS")
        || !has_pinned_chrome
        || sha256_hex(HTML_FIXTURE.as_bytes()) != EXPECTED_FIXTURE_SHA256
        || sha256_hex(SCREENSHOT) != EXPECTED_SCREENSHOT_SHA256
        || oracle["screenshot"]["path"].as_str() != Some("chrome155-stacking-full-viewport.png")
        || oracle["screenshot"]["width"].as_u64() != Some(GRID_WIDTH as u64)
        || oracle["screenshot"]["height"].as_u64() != Some(GRID_HEIGHT as u64)
        || oracle["screenshot"]["sha256"].as_str() != Some(EXPECTED_SCREENSHOT_SHA256)
        || oracle["expandedNodeIndexSha256"].as_str() != Some(EXPECTED_IDENTITY_SHA256)
    {
        return Err(
            "C13.1 stacking full viewport fixture/Chrome/viewport/screenshot metadata가 다릅니다"
                .to_owned(),
        );
    }
    validate_edge_control_metadata(oracle)?;
    Ok(())
}

fn validate_edge_control_metadata(
    oracle: &Value,
) -> Result<Vec<(Option<String>, f32, f32)>, String> {
    const EXPECTED_CONTROLS: [(Option<&str>, f64, f64); 11] = [
        (Some("root"), 538.0, 64.0),
        (Some("root"), 539.0, 64.0),
        (Some("root"), 0.0, 127.0),
        (Some("root"), 539.0, 127.0),
        (None, 539.5, 0.5),
        (None, 0.5, 127.5),
        (None, 539.5, 127.5),
        (None, -0.5, 64.0),
        (None, 540.0, 64.0),
        (None, 269.0, -0.5),
        (None, 269.0, 128.0),
    ];
    let controls = oracle["edgeControls"]
        .as_array()
        .filter(|controls| controls.len() == EXPECTED_CONTROLS.len())
        .ok_or_else(|| "C13.1 stacking full viewport edge control 수가 다릅니다".to_owned())?;
    let mut validated = Vec::with_capacity(controls.len());
    for (index, (control, (expected_id, expected_x, expected_y))) in
        controls.iter().zip(EXPECTED_CONTROLS).enumerate()
    {
        let Some(object) = control.as_object().filter(|object| object.len() == 3) else {
            return Err(format!(
                "C13.1 stacking full viewport edge control {index} schema가 다릅니다"
            ));
        };
        let x = control["x"].as_f64().filter(|value| value.is_finite());
        let y = control["y"].as_f64().filter(|value| value.is_finite());
        let hit_id = match control.get("hitId") {
            Some(Value::Null) => None,
            Some(Value::String(id)) => Some(id.as_str()),
            _ => {
                return Err(format!(
                    "C13.1 stacking full viewport edge control {index} hitId가 잘못되었습니다"
                ));
            }
        };
        if object
            .keys()
            .any(|key| !["hitId", "x", "y"].contains(&key.as_str()))
            || hit_id != expected_id
            || x != Some(expected_x)
            || y != Some(expected_y)
        {
            return Err(format!(
                "C13.1 stacking full viewport edge control {index}가 고정 기준과 다릅니다"
            ));
        }
        validated.push((
            hit_id.map(str::to_owned),
            expected_x as f32,
            expected_y as f32,
        ));
    }
    Ok(validated)
}

fn validate_node_ids(oracle: &Value) -> Result<Vec<String>, String> {
    let node_ids = oracle["nodeIds"]
        .as_array()
        .filter(|ids| ids.len() == NODE_COUNT)
        .ok_or_else(|| "C13.1 stacking full viewport node ID 수가 다릅니다".to_owned())?
        .iter()
        .map(|id| {
            id.as_str().map(str::to_owned).ok_or_else(|| {
                "C13.1 stacking full viewport node ID가 문자열이 아닙니다".to_owned()
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if node_ids.iter().map(String::as_str).collect::<Vec<_>>() != EXPECTED_NODE_IDS
        || node_ids
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
            != NODE_COUNT
        || EXPECTED_FRAMES
            .iter()
            .map(|frame| frame.id)
            .collect::<Vec<_>>()
            != EXPECTED_NODE_IDS
    {
        return Err("C13.1 stacking full viewport node ID 순서가 fixture와 다릅니다".to_owned());
    }
    Ok(node_ids)
}

fn validate_node_order_contract() -> Result<(), String> {
    if EXPECTED_FRAMES.len() != NODE_COUNT {
        return Err("C13.1 stacking full viewport expected frame 수가 다릅니다".to_owned());
    }
    Ok(())
}

fn validate_edge_controls(
    oracle: &Value,
    scene: &RuntimeRenderSnapshot,
    frames: &[RuntimeLayoutFrame],
    node_ids: &[String],
) -> Result<(), String> {
    let controls = validate_edge_control_metadata(oracle)?;
    for (index, (expected_id, x, y)) in controls.iter().enumerate() {
        let actual_id = match scene.hit_test_css_point(*x, *y) {
            None => None,
            Some(hit) => Some(
                frames
                    .iter()
                    .position(|frame| frame.node_id == hit.node_id().get())
                    .and_then(|node_index| node_ids.get(node_index))
                    .ok_or_else(|| {
                        format!(
                            "C13.1 stacking full viewport edge control {index}가 fixture 밖 node에 닿았습니다"
                        )
                    })?
                    .as_str(),
            ),
        };
        if actual_id != expected_id.as_deref() {
            return Err(format!(
                "C13.1 stacking full viewport edge control {index} hit mismatch at ({x}, {y}): expected={expected_id:?} actual={actual_id:?}"
            ));
        }
    }
    Ok(())
}

fn compare_grid<F>(grid: &ValidatedGrid, mut hit_test: F) -> Result<GridComparison, String>
where
    F: FnMut(usize, usize) -> ActualHit,
{
    let mut actual_identity = Vec::with_capacity(GRID_POINT_COUNT);
    let mut matched_count = 0usize;
    let mut mismatch_count = 0usize;
    let mut mismatch_preview = Vec::with_capacity(MISMATCH_PREVIEW_LIMIT);
    for (y, runs) in grid.rows.iter().enumerate() {
        for run in runs {
            for x in run.start_x..run.end_x {
                let actual = hit_test(x, y);
                actual_identity.push(actual.node_index.map_or(u8::MAX, |index| index as u8));
                if actual.node_index == Some(run.node_index) {
                    matched_count += 1;
                    continue;
                }
                mismatch_count += 1;
                if mismatch_preview.len() < MISMATCH_PREVIEW_LIMIT {
                    mismatch_preview.push(serde_json::json!({
                        "xCssPx": x,
                        "yCssPx": y,
                        "expectedNodeId": grid.node_ids[run.node_index],
                        "actualNodeId": actual.node_id,
                    }));
                }
            }
        }
    }
    if actual_identity.len() != GRID_POINT_COUNT
        || matched_count + mismatch_count != GRID_POINT_COUNT
        || mismatch_preview.len() != mismatch_count.min(MISMATCH_PREVIEW_LIMIT)
    {
        return Err(format!(
            "C13.1 stacking full viewport query accounting이 다릅니다: points={} matched={} mismatches={} preview={}",
            actual_identity.len(),
            matched_count,
            mismatch_count,
            mismatch_preview.len()
        ));
    }
    Ok(GridComparison {
        matched_count,
        mismatch_count,
        actual_identity_sha256: sha256_hex(&actual_identity),
        mismatch_preview,
    })
}

fn as_usize(value: &Value, label: &str, row: usize, run: usize) -> Result<usize, String> {
    value
        .as_u64()
        .and_then(|number| usize::try_from(number).ok())
        .ok_or_else(|| {
            format!("C13.1 stacking full viewport row {row} run {run} {label}가 unsigned integer가 아닙니다")
        })
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests;
