mod oracle;
mod runtime;
#[cfg(test)]
mod tests;

use oracle::{fixture_array, oracle_cases, script_with_cases, verify_png};
use runtime::{create_live_document, layout_document, viewport};

pub(super) const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
pub(super) const IMAGE_URL: &str = "https://spinon.invalid/c14-2-png-basic.png";
pub(super) const EXPECTED_PNG_SHA256: &str =
    "5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2";
pub(super) const BLOCK_HTML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c14/replaced-ratio-sizing-precomparison.html"
));
pub(super) const FLEX_HTML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c14/replaced-ratio-sizing-flex-precomparison.html"
));
pub(super) const CASES: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c14/c14-2-live-v8-cases.json"
));
pub(super) const BLOCK_SCRIPT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c14/c14-2-live-v8-block-document.js"
));
pub(super) const FLEX_SCRIPT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c14/c14-2-live-v8-flex-document.js"
));
pub(super) const BLOCK_INVENTORY: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c14/replaced-ratio-sizing-inventory.json"
));
pub(super) const FLEX_INVENTORY: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c14/replaced-ratio-sizing-flex-inventory.json"
));
pub(super) const BLOCK_ORACLE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../spec/internal/evidence/c14-2-image-ratio-precomparison-2026-10-14/chrome155-observation.json"
));
pub(super) const FLEX_ORACLE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../spec/internal/evidence/c14-2-image-ratio-precomparison-2026-10-14/chrome155-flex-observation.json"
));
pub(super) const PNG: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/resources/c14-1-png-basic.png"
));
pub(super) const CASES_PLACEHOLDER: &str = "__SPINON_C14_2_CASES__";
pub(super) const BLOCK_CASE_COUNT: usize = 21;
pub(super) const FLEX_CASE_COUNT: usize = 8;
pub(super) const FRAME_FIELD_COUNT: usize = 4;

pub(super) fn run_fixture() -> Result<(String, bool), String> {
    let fixture: serde_json::Value = serde_json::from_str(CASES)
        .map_err(|error| format!("C14.2 app case fixture JSON: {error}"))?;
    let block_inventory: serde_json::Value = serde_json::from_str(BLOCK_INVENTORY)
        .map_err(|error| format!("C14.2 Block inventory JSON: {error}"))?;
    let flex_inventory: serde_json::Value = serde_json::from_str(FLEX_INVENTORY)
        .map_err(|error| format!("C14.2 Flex inventory JSON: {error}"))?;
    let block_oracle: serde_json::Value = serde_json::from_str(BLOCK_ORACLE)
        .map_err(|error| format!("C14.2 Block observation JSON: {error}"))?;
    let flex_oracle: serde_json::Value = serde_json::from_str(FLEX_ORACLE)
        .map_err(|error| format!("C14.2 Flex observation JSON: {error}"))?;
    let png_sha256 = verify_png(&block_inventory, &flex_inventory)?;
    let block_mapping = fixture_array(&fixture, "blockCases")?;
    let flex_mapping = fixture_array(&fixture, "flexCases")?;
    let block_cases = oracle_cases(block_mapping, &block_inventory, &block_oracle, false)?;
    let flex_cases = oracle_cases(flex_mapping, &flex_inventory, &flex_oracle, true)?;
    if block_cases.len() != BLOCK_CASE_COUNT || flex_cases.len() != FLEX_CASE_COUNT {
        return Err(format!(
            "C14.2 oracle case count mismatch: Block={} Flex={}",
            block_cases.len(),
            flex_cases.len()
        ));
    }
    let block_viewport = viewport(&block_inventory)?;
    let flex_viewport = viewport(&flex_inventory)?;
    if block_viewport.width_css_px != flex_viewport.width_css_px
        || block_viewport.height_css_px != flex_viewport.height_css_px
        || block_viewport.device_scale_factor != flex_viewport.device_scale_factor
    {
        return Err("C14.2 Block/Flex oracle viewport가 다릅니다".to_owned());
    }

    let block_script = script_with_cases(BLOCK_SCRIPT, block_mapping)?;
    let flex_script = script_with_cases(FLEX_SCRIPT, flex_mapping)?;
    let block_live = create_live_document(&block_script, block_mapping, false)?;
    let flex_live = create_live_document(&flex_script, flex_mapping, true)?;
    let block_report = layout_document(
        "block",
        &block_live,
        &block_cases,
        &block_inventory,
        BLOCK_HTML,
        block_viewport,
    )?;
    let flex_report = layout_document(
        "flex",
        &flex_live,
        &flex_cases,
        &flex_inventory,
        FLEX_HTML,
        flex_viewport,
    )?;
    let mut report_cases = block_report.cases;
    report_cases.extend(flex_report.cases);
    let matched_count = block_report.matched_count + flex_report.matched_count;
    let matches = report_cases.len() == BLOCK_CASE_COUNT + FLEX_CASE_COUNT
        && matched_count == BLOCK_CASE_COUNT + FLEX_CASE_COUNT
        && block_report.source_stamp_matches
        && flex_report.source_stamp_matches;
    let report = serde_json::json!({
        "status": if matches { "ok" } else { "mismatch" },
        "fixture": "c14-2-live-v8-image-ratio-sizing",
        "document_origin": "live-v8",
        "oracle": "Chrome Headless 155.0.8059.40",
        "png_sha256": png_sha256,
        "case_count": BLOCK_CASE_COUNT + FLEX_CASE_COUNT,
        "matched_count": matched_count,
        "documents": [block_report.summary, flex_report.summary],
        "cases": report_cases,
    });
    let serialized =
        serde_json::to_string(&report).map_err(|error| format!("report JSON: {error}"))?;
    Ok((serialized, matches))
}

/// C14.2 고정 PNG를 live V8 문서의 Block 21-case와 Flex 8-case에서 실행합니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트만큼 쓸 수 있는 메모리여야 합니다.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn spinon_c14_2_app_image_ratio_fixture(
    output: *mut std::ffi::c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return -1;
    }
    match run_fixture() {
        Ok((report, matches)) => {
            if !crate::write_report(output, output_capacity, &report) {
                return -3;
            }
            if matches { 0 } else { -4 }
        }
        Err(detail) => {
            let report = serde_json::json!({ "status": "error", "detail": detail }).to_string();
            if !crate::write_report(output, output_capacity, &report) {
                return -3;
            }
            -2
        }
    }
}
