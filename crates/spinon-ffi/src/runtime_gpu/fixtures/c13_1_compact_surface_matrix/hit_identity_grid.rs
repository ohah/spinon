use super::{
    CHROME_ORACLE, EXPECTED_CASE_IDS, ExpectedFrame, expected_frames, validate_oracle_metadata,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use spinon_render::RuntimeRenderSnapshot;
use spinon_runtime::RuntimeLayoutFrame;
use std::collections::HashSet;

const HIT_IDENTITY_GRID_ORACLE: &str = include_str!(
    "../../../../../../spec/internal/evidence/c13-1-android-app-hit-identity-grid-2026-10-14/chrome155-hit-identity-grid.json"
);
const COMPACT_SURFACE_HTML: &[u8] =
    include_bytes!("../../../../../../tests/fixtures/css/c13/c13-1-compact-surface-matrix.html");
const COMPACT_SURFACE_JS: &[u8] =
    include_bytes!("../../../../../../tests/fixtures/css/c13/c13-1-compact-surface-matrix.js");

const EXPECTED_GRID_FILE_SHA256: &str =
    "35e88e57edc16e526e389317b0d18ef227e35d1356a2353b92476c75ef6414f6";
const EXPECTED_CHROME_OBSERVATION_SHA256: &str =
    "515fc95f811399fda9e336a13e7583f2d22555fce4727f1fe7f734981abd0299";
const EXPECTED_HTML_SHA256: &str =
    "431f4b4452969adcae80c4e5f89e5e45b1e8756fc3c4d8995b7a5eccfbc6f355";
const EXPECTED_SCRIPT_SHA256: &str =
    "d36bc1f7d3c791cdc91f6d878c3961e067efc2f88ca1486db8a065c5fd69c760";
const EXPECTED_IDENTITY_SHA256: &str =
    "93cccd042e82d8cd30cb7a8ab34a5154073d4a9eccbc4038294d3ca5b9dfce87";
const GRID_WIDTH: usize = 300;
const GRID_HEIGHT: usize = 100;
const GRID_POINT_COUNT: usize = GRID_WIDTH * GRID_HEIGHT;
const NODE_COUNT: usize = 41;
const GRID_RUN_COUNT: usize = 870;
const MISMATCH_PREVIEW_LIMIT: usize = 16;
const MAX_SUMMARY_BYTES: usize = 8192;

#[derive(Clone, Debug)]
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

pub(super) fn summary_json(
    scene: &RuntimeRenderSnapshot,
    frames: &[RuntimeLayoutFrame],
) -> Result<String, String> {
    let chrome_oracle: Value = serde_json::from_str(CHROME_ORACLE)
        .map_err(|error| format!("compact C13.1 Chrome observation JSON 읽기 실패: {error}"))?;
    let grid_oracle = parse_pinned_grid_oracle(HIT_IDENTITY_GRID_ORACLE)?;
    let grid = validate_grid_oracle(&grid_oracle, &chrome_oracle)?;
    validate_runtime_scene(scene, frames, &chrome_oracle)?;

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
        "expected_identity_sha256": grid.identity_sha256,
        "actual_identity_sha256": comparison.actual_identity_sha256,
        "chrome_observation_sha256": EXPECTED_CHROME_OBSERVATION_SHA256,
        "grid_oracle_sha256": EXPECTED_GRID_FILE_SHA256,
    });
    let summary = serde_json::to_string(&summary)
        .map_err(|error| format!("compact C13.1 hit grid summary JSON 변환 실패: {error}"))?;
    if summary.len() > MAX_SUMMARY_BYTES {
        return Err(format!(
            "compact C13.1 hit grid summary가 제한을 넘었습니다: bytes={} max={MAX_SUMMARY_BYTES}",
            summary.len()
        ));
    }
    Ok(summary)
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
            "compact C13.1 hit grid query accounting이 다릅니다: points={} matched={} mismatches={} preview={}",
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

fn parse_pinned_grid_oracle(source: &str) -> Result<Value, String> {
    if sha256_hex(source.as_bytes()) != EXPECTED_GRID_FILE_SHA256 {
        return Err("compact C13.1 hit grid oracle 파일 SHA-256이 고정값과 다릅니다".to_owned());
    }
    serde_json::from_str(source)
        .map_err(|error| format!("compact C13.1 hit grid JSON 읽기 실패: {error}"))
}

fn validate_grid_oracle(grid: &Value, chrome: &Value) -> Result<ValidatedGrid, String> {
    validate_oracle_metadata(chrome)?;
    validate_source_provenance(chrome)?;
    if grid["status"].as_str() != Some("captured")
        || grid["queryConvention"].as_str()
            != Some("elementFromPoint(xCssPx, yCssPx), x=0..299, y=0..99")
        || grid["queryCount"].as_u64() != Some(GRID_POINT_COUNT as u64)
        || grid["rowCount"].as_u64() != Some(GRID_HEIGHT as u64)
        || grid["runCount"].as_u64() != Some(GRID_RUN_COUNT as u64)
        || grid["pixelGridIdentitySha256"].as_str() != Some(EXPECTED_IDENTITY_SHA256)
        || grid["browser"]["userAgent"] != chrome["browser"]["userAgent"]
        || grid["viewport"]
            != serde_json::json!({
                "width": GRID_WIDTH,
                "height": GRID_HEIGHT,
                "devicePixelRatio": 1,
                "scrollX": 0,
                "scrollY": 0,
            })
        || grid["root"]
            != serde_json::json!({
                "x": 0,
                "y": 0,
                "width": GRID_WIDTH,
                "height": GRID_HEIGHT,
                "top": 0,
                "right": GRID_WIDTH,
                "bottom": GRID_HEIGHT,
                "left": 0,
            })
        || grid["unexpected"]
            .as_array()
            .is_none_or(|items| !items.is_empty())
    {
        return Err(
            "compact C13.1 hit grid source/browser/viewport/count metadata가 다릅니다".to_owned(),
        );
    }

    let expected_frames = expected_frames(chrome)?;
    if expected_frames.len() != NODE_COUNT {
        return Err(format!(
            "compact C13.1 Chrome node 수가 다릅니다: {}",
            expected_frames.len()
        ));
    }
    let expected_node_ids = expected_frames
        .iter()
        .map(|frame| frame.id.as_str())
        .collect::<Vec<_>>();
    let node_ids = grid["nodeIds"]
        .as_array()
        .filter(|ids| ids.len() == NODE_COUNT)
        .ok_or_else(|| "compact C13.1 hit grid node ID 목록 길이가 다릅니다".to_owned())?
        .iter()
        .map(|id| {
            id.as_str()
                .map(str::to_owned)
                .ok_or_else(|| "compact C13.1 hit grid node ID가 문자열이 아닙니다".to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?;
    if node_ids.iter().map(String::as_str).collect::<Vec<_>>() != expected_node_ids
        || node_ids.iter().collect::<HashSet<_>>().len() != NODE_COUNT
    {
        return Err("compact C13.1 hit grid node ID 순서가 Chrome fixture와 다릅니다".to_owned());
    }

    let rows = grid["rows"]
        .as_array()
        .filter(|rows| rows.len() == GRID_HEIGHT)
        .ok_or_else(|| "compact C13.1 hit grid row 수가 다릅니다".to_owned())?;
    let mut decoded_rows = Vec::with_capacity(GRID_HEIGHT);
    let mut identity_bytes = Vec::with_capacity(GRID_POINT_COUNT);
    let mut histogram = vec![0usize; NODE_COUNT];
    let mut run_count = 0usize;
    for (y, row) in rows.iter().enumerate() {
        let runs = row
            .as_array()
            .filter(|runs| !runs.is_empty())
            .ok_or_else(|| format!("compact C13.1 hit grid row {y}에 run이 없습니다"))?;
        let mut decoded_runs = Vec::with_capacity(runs.len());
        let mut next_x = 0usize;
        for run in runs {
            let fields = run
                .as_array()
                .filter(|fields| fields.len() == 3)
                .ok_or_else(|| {
                    format!("compact C13.1 hit grid row {y} run shape이 잘못되었습니다")
                })?;
            let start_x = as_usize(&fields[0], "run start")?;
            let end_x = as_usize(&fields[1], "run end")?;
            let node_index = as_usize(&fields[2], "node index")?;
            if start_x != next_x || end_x <= start_x || end_x > GRID_WIDTH {
                return Err(format!(
                    "compact C13.1 hit grid row {y} run coverage가 잘못되었습니다: [{start_x},{end_x}) next={next_x}"
                ));
            }
            if node_index >= NODE_COUNT {
                return Err(format!(
                    "compact C13.1 hit grid row {y} unknown node index={node_index}"
                ));
            }
            let width = end_x - start_x;
            histogram[node_index] += width;
            identity_bytes.extend(std::iter::repeat_n(node_index as u8, width));
            decoded_runs.push(HitRun {
                start_x,
                end_x,
                node_index,
            });
            next_x = end_x;
            run_count += 1;
        }
        if next_x != GRID_WIDTH {
            return Err(format!(
                "compact C13.1 hit grid row {y}가 viewport 폭을 덮지 않습니다: {next_x}"
            ));
        }
        decoded_rows.push(decoded_runs);
    }
    if run_count != GRID_RUN_COUNT || identity_bytes.len() != GRID_POINT_COUNT {
        return Err(format!(
            "compact C13.1 hit grid expanded count가 다릅니다: runs={run_count} points={}",
            identity_bytes.len()
        ));
    }
    let declared_histogram = grid["histogram"]
        .as_array()
        .filter(|items| items.len() == NODE_COUNT)
        .ok_or_else(|| "compact C13.1 hit grid histogram 길이가 다릅니다".to_owned())?
        .iter()
        .map(|value| as_usize(value, "histogram count"))
        .collect::<Result<Vec<_>, _>>()?;
    if histogram != declared_histogram || histogram.iter().sum::<usize>() != GRID_POINT_COUNT {
        return Err("compact C13.1 hit grid histogram이 row data와 다릅니다".to_owned());
    }
    let identity_sha256 = sha256_hex(&identity_bytes);
    if identity_sha256 != EXPECTED_IDENTITY_SHA256
        || grid["pixelGridIdentitySha256"].as_str() != Some(identity_sha256.as_str())
    {
        return Err("compact C13.1 hit grid expanded identity SHA-256이 다릅니다".to_owned());
    }
    validate_fixed_sample_subset(&decoded_rows, &node_ids, chrome)?;
    Ok(ValidatedGrid {
        node_ids,
        rows: decoded_rows,
        identity_sha256,
    })
}

fn validate_source_provenance(chrome: &Value) -> Result<(), String> {
    let sources = &chrome["sources"];
    if sha256_hex(CHROME_ORACLE.as_bytes()) != EXPECTED_CHROME_OBSERVATION_SHA256
        || sources["html"].as_str()
            != Some("tests/fixtures/css/c13/c13-1-compact-surface-matrix.html")
        || sources["htmlSha256"].as_str() != Some(EXPECTED_HTML_SHA256)
        || sha256_hex(COMPACT_SURFACE_HTML) != EXPECTED_HTML_SHA256
        || sources["fixture"].as_str()
            != Some("tests/fixtures/css/c13/c13-1-compact-surface-matrix.js")
        || sources["fixtureSha256"].as_str() != Some(EXPECTED_SCRIPT_SHA256)
        || sha256_hex(COMPACT_SURFACE_JS) != EXPECTED_SCRIPT_SHA256
    {
        return Err(
            "compact C13.1 Chrome source/observation SHA-256이 고정값과 다릅니다".to_owned(),
        );
    }
    Ok(())
}

fn validate_fixed_sample_subset(
    rows: &[Vec<HitRun>],
    node_ids: &[String],
    chrome: &Value,
) -> Result<(), String> {
    let cases = chrome["cases"]
        .as_array()
        .filter(|cases| cases.len() == EXPECTED_CASE_IDS.len())
        .ok_or_else(|| "compact C13.1 fixed sample case 수가 다릅니다".to_owned())?;
    let mut sample_count = 0usize;
    for (case, expected_case_id) in cases.iter().zip(EXPECTED_CASE_IDS) {
        if case["id"].as_str() != Some(expected_case_id) {
            return Err("compact C13.1 fixed sample case 순서가 다릅니다".to_owned());
        }
        let samples = case["samples"]
            .as_array()
            .filter(|samples| samples.len() == 4)
            .ok_or_else(|| {
                format!("compact C13.1 fixed sample {expected_case_id} 수가 다릅니다")
            })?;
        for sample in samples {
            let x = as_usize(&sample["x"], "fixed sample x")?;
            let y = as_usize(&sample["y"], "fixed sample y")?;
            if x >= GRID_WIDTH || y >= GRID_HEIGHT {
                return Err("compact C13.1 fixed sample이 viewport 밖입니다".to_owned());
            }
            let run = rows[y]
                .iter()
                .find(|run| x >= run.start_x && x < run.end_x)
                .ok_or_else(|| {
                    format!("compact C13.1 fixed sample ({x},{y})에 grid run이 없습니다")
                })?;
            let expected_id = sample["hitId"]
                .as_str()
                .ok_or_else(|| "compact C13.1 fixed sample Chrome hit ID가 없습니다".to_owned())?;
            if node_ids[run.node_index] != expected_id {
                return Err(format!(
                    "compact C13.1 fixed sample ({x},{y}) identity가 다릅니다"
                ));
            }
            sample_count += 1;
        }
    }
    if sample_count != 52 {
        return Err(format!(
            "compact C13.1 fixed sample subset 수가 다릅니다: {sample_count}"
        ));
    }
    Ok(())
}

fn validate_runtime_scene(
    scene: &RuntimeRenderSnapshot,
    frames: &[RuntimeLayoutFrame],
    chrome: &Value,
) -> Result<Vec<ExpectedFrame>, String> {
    let viewport = scene.viewport_css_px();
    if viewport.width() != GRID_WIDTH as f32 || viewport.height() != GRID_HEIGHT as f32 {
        return Err(format!(
            "compact C13.1 hit grid viewport가 다릅니다: actual={}x{} expected={}x{}",
            viewport.width(),
            viewport.height(),
            GRID_WIDTH,
            GRID_HEIGHT
        ));
    }
    let expected = expected_frames(chrome)?;
    if frames.len() != NODE_COUNT
        || scene.boxes().len() != NODE_COUNT
        || expected.len() != NODE_COUNT
    {
        return Err(format!(
            "compact C13.1 hit grid frame 수가 다릅니다: layout={} scene={} oracle={}",
            frames.len(),
            scene.boxes().len(),
            expected.len()
        ));
    }
    validate_runtime_node_order(frames)?;
    let frame_node_ids = frames
        .iter()
        .map(|frame| frame.node_id)
        .collect::<HashSet<_>>();
    let scene_node_ids = scene
        .boxes()
        .iter()
        .map(|render_box| render_box.node_id().get())
        .collect::<HashSet<_>>();
    if frame_node_ids.len() != NODE_COUNT
        || scene_node_ids.len() != NODE_COUNT
        || scene_node_ids != frame_node_ids
    {
        return Err("compact C13.1 hit grid render box와 layout NodeId가 다릅니다".to_owned());
    }
    for (index, expected_frame) in expected.iter().enumerate() {
        let actual = frames[index];
        let differences = [
            (actual.x as f64 - expected_frame.x).abs(),
            (actual.y as f64 - expected_frame.y).abs(),
            (actual.width as f64 - expected_frame.width).abs(),
            (actual.height as f64 - expected_frame.height).abs(),
        ];
        if differences.into_iter().any(|difference| difference > 0.5) {
            return Err(format!(
                "compact C13.1 hit grid frame {} index={} differs from Chrome",
                expected_frame.id, index
            ));
        }
    }
    Ok(expected)
}

fn validate_runtime_node_order(frames: &[RuntimeLayoutFrame]) -> Result<(), String> {
    // fixture script의 생성 순서가 oracle ID 순서이고 runtime frame도 DOM preorder로 나온다.
    if let Some(pair) = frames
        .windows(2)
        .find(|pair| pair[0].node_id >= pair[1].node_id)
    {
        return Err(format!(
            "compact C13.1 hit grid frame NodeId 순서가 fixture 생성 순서와 다릅니다: {} 다음 {}",
            pair[0].node_id, pair[1].node_id
        ));
    }
    Ok(())
}

fn as_usize(value: &Value, label: &str) -> Result<usize, String> {
    value
        .as_u64()
        .and_then(|value| usize::try_from(value).ok())
        .ok_or_else(|| format!("compact C13.1 {label}가 유효한 unsigned integer가 아닙니다"))
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests;
