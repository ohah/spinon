use sha2::{Digest, Sha256};

use super::FRAME_FIELD_COUNT;
use super::{CASES_PLACEHOLDER, EXPECTED_PNG_SHA256, PNG};

#[derive(Clone, Debug)]
pub(super) struct OracleCase {
    pub(super) id: String,
    pub(super) expected_size: [f32; 2],
    pub(super) expected_frame: [f32; FRAME_FIELD_COUNT],
    pub(super) tolerance: f32,
    pub(super) expected_parent_size: Option<[f32; 2]>,
    pub(super) expected_align_items: Option<String>,
}

pub(super) fn oracle_cases(
    mapping: &[serde_json::Value],
    inventory: &serde_json::Value,
    observation: &serde_json::Value,
    flex: bool,
) -> Result<Vec<OracleCase>, String> {
    verify_oracle_header(inventory, observation)?;
    let inventory_cases = inventory["cases"]
        .as_array()
        .ok_or_else(|| "C14.2 inventory case list missing".to_owned())?;
    let observation_cases = observation["cases"]
        .as_array()
        .ok_or_else(|| "C14.2 observation case list missing".to_owned())?;
    let expected_inventory_ids = inventory_cases
        .iter()
        .filter(|case| flex || case.get("layoutStage").is_none())
        .map(|case| json_string(case, "id"))
        .collect::<Result<Vec<_>, _>>()?;
    let mapping_ids = mapping
        .iter()
        .map(|case| json_string(case, "id"))
        .collect::<Result<Vec<_>, _>>()?;
    if expected_inventory_ids != mapping_ids {
        return Err(format!(
            "C14.2 {} live-V8 case IDs/order differ from inventory",
            if flex { "Flex" } else { "Block" }
        ));
    }
    let tolerance = json_number(&inventory["comparison"], "toleranceCssPx")?;
    let mut cases = Vec::with_capacity(mapping.len());
    for mapped in mapping {
        let id = json_string(mapped, "id")?.to_owned();
        let expected = find_case(inventory_cases, &id)?;
        let observed = find_case(observation_cases, &id)?;
        if !flex
            && observed
                .get("complete")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
        {
            return Err(format!("{id} Chrome PNG image was not complete"));
        }
        let expected_size = json_pair(
            expected.get("expectedCssPx"),
            &format!("{id} expectedCssPx"),
        )?;
        let expected_natural = json_size(
            inventory["resource"]
                .get("naturalSizeCssPx")
                .ok_or_else(|| "C14.2 expected PNG natural size missing".to_owned())?,
            "C14.2 expected PNG natural size",
        )?;
        if json_number(observed, "naturalWidth")? != expected_natural[0]
            || json_number(observed, "naturalHeight")? != expected_natural[1]
        {
            return Err(format!(
                "{id} Chrome observation natural size differs from PNG metadata"
            ));
        }
        let observed_frame = json_frame(observed.get("frame"), &format!("{id} oracle frame"))?;
        if (expected_size[0] - observed_frame[2]).abs() > 0.000_001
            || (expected_size[1] - observed_frame[3]).abs() > 0.000_001
        {
            return Err(format!(
                "{id} inventory size differs from Chrome observation"
            ));
        }
        let case_tolerance = expected
            .get("toleranceCssPx")
            .and_then(serde_json::Value::as_f64)
            .map(|number| number as f32)
            .unwrap_or(tolerance);
        if !case_tolerance.is_finite() || case_tolerance < 0.0 {
            return Err(format!("{id} has invalid CSS px tolerance"));
        }
        let (expected_parent_size, expected_align_items) = if flex {
            let parent = observed
                .get("parent")
                .ok_or_else(|| format!("{id} Flex parent observation missing"))?;
            (
                Some(json_size(parent, &format!("{id} parent size"))?),
                Some(json_string(parent, "alignItems")?.to_owned()),
            )
        } else {
            (None, None)
        };
        cases.push(OracleCase {
            id,
            expected_size,
            expected_frame: observed_frame,
            tolerance: case_tolerance,
            expected_parent_size,
            expected_align_items,
        });
    }
    Ok(cases)
}

pub(super) fn verify_oracle_header(
    inventory: &serde_json::Value,
    observation: &serde_json::Value,
) -> Result<(), String> {
    if !json_string(inventory, "browser")?.contains("155.0.8059.40") {
        return Err("C14.2 inventory is not pinned to Chrome 155.0.8059.40".to_owned());
    }
    let observed_browser = observation
        .get("browser")
        .or_else(|| observation.get("userAgent"))
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| "C14.2 observation browser metadata missing".to_owned())?;
    if !observed_browser.contains("155.0.8059.40") {
        return Err("C14.2 observation is not pinned to Chrome 155.0.8059.40".to_owned());
    }
    if json_number(inventory, "devicePixelRatio")? != 1.0 {
        return Err("C14.2 inventory DPR is not 1".to_owned());
    }
    let viewport = inventory
        .get("viewportCssPx")
        .ok_or_else(|| "C14.2 oracle viewport missing".to_owned())?;
    if json_number(viewport, "width")? != 1280.0 || json_number(viewport, "height")? != 633.0 {
        return Err("C14.2 oracle viewport is not 1280×633 CSS px".to_owned());
    }
    if json_number(observation, "devicePixelRatio")? != 1.0 {
        return Err("C14.2 observation DPR is not 1".to_owned());
    }
    let observed_viewport = observation
        .get("viewport")
        .ok_or_else(|| "C14.2 observation viewport missing".to_owned())?;
    if json_number(observed_viewport, "width")? != 1280.0
        || json_number(observed_viewport, "height")? != 633.0
    {
        return Err("C14.2 observation viewport is not 1280×633 CSS px".to_owned());
    }
    Ok(())
}

pub(super) fn verify_png(
    block_inventory: &serde_json::Value,
    flex_inventory: &serde_json::Value,
) -> Result<String, String> {
    for inventory in [block_inventory, flex_inventory] {
        let resource = inventory
            .get("resource")
            .ok_or_else(|| "C14.2 PNG resource metadata missing".to_owned())?;
        let natural_size = resource
            .get("naturalSizeCssPx")
            .ok_or_else(|| "C14.2 PNG naturalSizeCssPx missing".to_owned())?;
        if json_string(resource, "path")? != "../../resources/c14-1-png-basic.png"
            || json_string(resource, "sha256")? != EXPECTED_PNG_SHA256
            || json_size(natural_size, "PNG naturalSizeCssPx")? != [24.0, 16.0]
        {
            return Err(
                "C14.2 inventory PNG metadata differs from the fixed 24×16 resource".to_owned(),
            );
        }
    }
    let actual = Sha256::digest(PNG)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    if actual != EXPECTED_PNG_SHA256 {
        return Err(format!(
            "C14.2 PNG SHA-256 mismatch: expected={EXPECTED_PNG_SHA256} actual={actual}"
        ));
    }
    Ok(actual)
}

pub(super) fn fixture_array<'a>(
    fixture: &'a serde_json::Value,
    key: &str,
) -> Result<&'a [serde_json::Value], String> {
    fixture
        .get(key)
        .and_then(serde_json::Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| format!("C14.2 app fixture array missing: {key}"))
}

pub(super) fn script_with_cases(
    template: &str,
    cases: &[serde_json::Value],
) -> Result<String, String> {
    if template.matches(CASES_PLACEHOLDER).count() != 1 {
        return Err("C14.2 live-V8 document script must contain one case placeholder".to_owned());
    }
    let cases = serde_json::to_string(cases).map_err(|error| format!("case JSON: {error}"))?;
    Ok(template.replacen(CASES_PLACEHOLDER, &cases, 1))
}

pub(super) fn find_case<'a>(
    cases: &'a [serde_json::Value],
    id: &str,
) -> Result<&'a serde_json::Value, String> {
    let mut matches = cases
        .iter()
        .filter(|case| case.get("id").and_then(serde_json::Value::as_str) == Some(id));
    let case = matches
        .next()
        .ok_or_else(|| format!("oracle case missing: {id}"))?;
    if matches.next().is_some() {
        return Err(format!("oracle case ID duplicated: {id}"));
    }
    Ok(case)
}

pub(super) fn json_string<'a>(value: &'a serde_json::Value, key: &str) -> Result<&'a str, String> {
    value
        .get(key)
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| format!("JSON string missing: {key}"))
}

pub(super) fn json_number(value: &serde_json::Value, key: &str) -> Result<f32, String> {
    value
        .get(key)
        .and_then(serde_json::Value::as_f64)
        .map(|number| number as f32)
        .filter(|number| number.is_finite())
        .ok_or_else(|| format!("JSON finite number missing: {key}"))
}

pub(super) fn json_pair(
    value: Option<&serde_json::Value>,
    label: &str,
) -> Result<[f32; 2], String> {
    let values = value
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| format!("{label} pair missing"))?;
    if values.len() == 2 {
        return Ok([
            values[0]
                .as_f64()
                .map(|number| number as f32)
                .filter(|number| number.is_finite())
                .ok_or_else(|| format!("{label}[0] invalid"))?,
            values[1]
                .as_f64()
                .map(|number| number as f32)
                .filter(|number| number.is_finite())
                .ok_or_else(|| format!("{label}[1] invalid"))?,
        ]);
    }
    Err(format!("{label} must contain two numbers"))
}

pub(super) fn json_size(value: &serde_json::Value, label: &str) -> Result<[f32; 2], String> {
    Ok([
        json_number(value, "width").map_err(|_| format!("{label}.width missing"))?,
        json_number(value, "height").map_err(|_| format!("{label}.height missing"))?,
    ])
}

pub(super) fn json_frame(
    value: Option<&serde_json::Value>,
    label: &str,
) -> Result<[f32; 4], String> {
    let value = value.ok_or_else(|| format!("{label} missing"))?;
    let frame = [
        json_number(value, "x")?,
        json_number(value, "y")?,
        json_number(value, "width")?,
        json_number(value, "height")?,
    ];
    if frame.iter().all(|number| number.is_finite()) {
        Ok(frame)
    } else {
        Err(format!("{label} contains non-finite geometry"))
    }
}

pub(super) fn within_frame(actual: [f32; 4], expected: [f32; 4], tolerance: f32) -> bool {
    actual
        .iter()
        .zip(expected)
        .all(|(actual, expected)| actual.is_finite() && (*actual - expected).abs() <= tolerance)
}
