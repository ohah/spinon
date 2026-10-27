use serde_json::Value;

use super::super::{SpanStageExpectation, StageSnapshotExpectation};
use super::{CASES, ChromeRowAxisCase, ROOT_SIZE, TRACE_TOLERANCE, assert_close};

pub(super) fn assert_expected_scene_registry(expected_json: &Value) {
    let context = "row-axis expected.json";
    assert_eq!(
        json_string(json_required(expected_json, "axis", context), "axis"),
        "Rows",
        "{context}: axis"
    );
    assert_eq!(
        json_usize(
            json_required(expected_json, "track_count", context),
            "track_count"
        ),
        4,
        "{context}: CSS track count"
    );
    assert_close(
        json_number(
            json_required(expected_json, "available_block_size_px", context),
            "available_block_size_px",
        ),
        ROOT_SIZE,
        TRACE_TOLERANCE,
        context,
        "available block size",
    );
    let results = json_array(json_required(expected_json, "results", context), "results");
    assert_eq!(
        results.len(),
        CASES.len(),
        "{context}: exactly one expectation per Rust scene"
    );

    let expected_ids = CASES.iter().map(|case| case.id).collect::<Vec<_>>();
    let mut expected_ids_seen = std::collections::BTreeSet::new();
    assert!(
        expected_ids
            .iter()
            .all(|scene_id| expected_ids_seen.insert(*scene_id)),
        "Rust row-axis scene IDs must be unique"
    );
    let actual_ids = results
        .iter()
        .map(|result| {
            json_string(
                json_required(result, "id", "row-axis expected result"),
                "result.id",
            )
        })
        .collect::<Vec<_>>();
    let mut actual_ids_seen = std::collections::BTreeSet::new();
    assert!(
        actual_ids
            .iter()
            .all(|scene_id| actual_ids_seen.insert(*scene_id)),
        "{context}: duplicate scene ID"
    );
    assert_eq!(
        actual_ids, expected_ids,
        "{context}: exact scene IDs and authored result order"
    );
}

pub(super) fn row_axis_expectation(
    expected_json: &Value,
    case: &ChromeRowAxisCase,
) -> SpanStageExpectation {
    let context = format!("expected.json.results[{}]", case.id);
    let results = json_array(
        json_required(expected_json, "results", "row-axis expected.json"),
        "results",
    );
    let matching = results
        .iter()
        .filter(|result| {
            json_string(
                json_required(result, "id", "row-axis expected result"),
                "result.id",
            ) == case.id
        })
        .collect::<Vec<_>>();
    assert_eq!(
        matching.len(),
        1,
        "{context}: exactly one JSON result is required"
    );
    let result = matching[0];

    let source_order = json_array(
        json_required(result, "source_order", &context),
        &format!("{context}.source_order"),
    )
    .iter()
    .map(|item| json_string(item, &format!("{context}.source_order[]")))
    .collect::<Vec<_>>();
    let expected_source_order = case
        .items_in_source_order
        .iter()
        .map(|item| item.id)
        .collect::<Vec<_>>();
    assert_eq!(
        source_order, expected_source_order,
        "{context}: source order must match the runtime document"
    );

    let row_contributions = json_required(result, "row_contributions_px", &context)
        .as_object()
        .unwrap_or_else(|| panic!("{context}.row_contributions_px must be an object"));
    assert_eq!(
        row_contributions.len(),
        case.items_in_source_order.len(),
        "{context}.row_contributions_px: exact item set"
    );
    for item in case.items_in_source_order {
        let value = row_contributions
            .get(item.id)
            .unwrap_or_else(|| panic!("{context}.row_contributions_px.{} is missing", item.id));
        assert_close(
            json_number(
                value,
                &format!("{context}.row_contributions_px.{}", item.id),
            ),
            item.height,
            TRACE_TOLERANCE,
            case.id,
            &format!("{} block contribution", item.id),
        );
    }

    let span_batches = json_array(
        json_required(result, "normative_batch_order", &context),
        &format!("{context}.normative_batch_order"),
    )
    .iter()
    .map(|span| parse_span_label(span, &format!("{context}.normative_batch_order[]")))
    .collect::<Vec<_>>();
    assert_eq!(
        span_batches, case.expected_span_batches,
        "{context}: normative span batches must match the fixture input"
    );

    let initialize_json = json_required(result, "initialize", &context);
    let initialize = parse_stage_snapshot(
        initialize_json,
        "base_sizes_px",
        &format!("{context}.initialize"),
    );

    let batch_stages = json_array(
        json_required(result, "intrinsic_batch_stages", &context),
        &format!("{context}.intrinsic_batch_stages"),
    );
    assert_eq!(
        batch_stages.len(),
        span_batches.len(),
        "{context}.intrinsic_batch_stages: one snapshot per batch"
    );
    let intrinsic_batches = batch_stages
        .iter()
        .zip(&span_batches)
        .enumerate()
        .map(|(index, (stage_json, expected_span))| {
            let stage_context = format!("{context}.intrinsic_batch_stages[{index}]");
            let expected_stage_name =
                format!("IntrinsicItemBatch(span={expected_span}, crosses_flexible_track=false)");
            assert_eq!(
                json_string(
                    json_required(stage_json, "stage", &stage_context),
                    &format!("{stage_context}.stage"),
                ),
                expected_stage_name,
                "{stage_context}: stage name"
            );
            parse_stage_snapshot(stage_json, "base_sizes_px", &stage_context)
        })
        .collect::<Vec<_>>();

    let intrinsic_sizing_complete_json =
        json_required(result, "intrinsic_sizing_complete", &context);
    let intrinsic_sizing_complete = parse_stage_snapshot(
        intrinsic_sizing_complete_json,
        "base_sizes_px",
        &format!("{context}.intrinsic_sizing_complete"),
    );

    let maximize_json = json_required(result, "maximize_tracks_stage", &context);
    let maximize_tracks = parse_stage_snapshot(
        maximize_json,
        "row_sizes_px",
        &format!("{context}.maximize_tracks_stage"),
    );
    let available_block_size_px = json_number(
        json_required(
            maximize_json,
            "available_block_size_px",
            &format!("{context}.maximize_tracks_stage"),
        ),
        &format!("{context}.maximize_tracks_stage.available_block_size_px"),
    );
    assert_close(
        available_block_size_px,
        json_number(
            json_required(
                expected_json,
                "available_block_size_px",
                "row-axis expected.json",
            ),
            "available_block_size_px",
        ),
        TRACE_TOLERANCE,
        case.id,
        "oracle available block size",
    );
    let remaining_free_space_px = json_number(
        json_required(
            maximize_json,
            "remaining_free_space_px",
            &format!("{context}.maximize_tracks_stage"),
        ),
        &format!("{context}.maximize_tracks_stage.remaining_free_space_px"),
    );

    SpanStageExpectation {
        span_batches,
        initialize,
        intrinsic_batches,
        intrinsic_sizing_complete,
        maximize_tracks,
        available_block_size_px,
        remaining_free_space_px,
    }
}

fn parse_stage_snapshot(
    value: &Value,
    base_sizes_key: &str,
    context: &str,
) -> StageSnapshotExpectation {
    StageSnapshotExpectation {
        base_sizes: json_float_array(
            json_required(value, base_sizes_key, context),
            &format!("{context}.{base_sizes_key}"),
        ),
        growth_limits: json_growth_array(
            json_required(value, "growth_limits_px", context),
            &format!("{context}.growth_limits_px"),
        ),
    }
}

fn parse_span_label(value: &Value, context: &str) -> u16 {
    let label = json_string(value, context);
    let digits = label
        .strip_prefix("span")
        .unwrap_or_else(|| panic!("{context}: expected spanN, got {label:?}"));
    let span = digits
        .parse::<u16>()
        .unwrap_or_else(|error| panic!("{context}: invalid span label {label:?}: {error}"));
    assert!(span > 0, "{context}: span must be positive");
    assert_eq!(
        label,
        format!("span{span}"),
        "{context}: non-canonical span label"
    );
    span
}

fn json_required<'a>(value: &'a Value, key: &str, context: &str) -> &'a Value {
    value
        .get(key)
        .unwrap_or_else(|| panic!("{context}.{key} is missing"))
}

fn json_array<'a>(value: &'a Value, context: &str) -> &'a [Value] {
    value
        .as_array()
        .unwrap_or_else(|| panic!("{context} must be an array"))
}

fn json_string<'a>(value: &'a Value, context: &str) -> &'a str {
    value
        .as_str()
        .unwrap_or_else(|| panic!("{context} must be a string"))
}

fn json_usize(value: &Value, context: &str) -> usize {
    usize::try_from(
        value
            .as_u64()
            .unwrap_or_else(|| panic!("{context} must be an unsigned integer")),
    )
    .unwrap_or_else(|error| panic!("{context} is out of range: {error}"))
}

fn json_number(value: &Value, context: &str) -> f32 {
    let parsed = if let Some(number) = value.as_f64() {
        number
    } else if let Some(rational) = value.as_str() {
        let (numerator, denominator) = rational
            .split_once('/')
            .unwrap_or_else(|| panic!("{context}: unsupported numeric string {rational:?}"));
        assert!(
            !denominator.contains('/'),
            "{context}: malformed rational {rational:?}"
        );
        let numerator = numerator.parse::<f64>().unwrap_or_else(|error| {
            panic!("{context}: invalid rational numerator in {rational:?}: {error}")
        });
        let denominator = denominator.parse::<f64>().unwrap_or_else(|error| {
            panic!("{context}: invalid rational denominator in {rational:?}: {error}")
        });
        assert!(
            numerator.is_finite() && denominator.is_finite() && denominator > 0.0,
            "{context}: non-finite or non-positive rational {rational:?}"
        );
        numerator / denominator
    } else {
        panic!("{context}: expected a JSON number or rational string")
    };
    assert!(
        parsed.is_finite() && parsed.abs() <= f32::MAX as f64,
        "{context}: value is outside finite f32 range"
    );
    let value = parsed as f32;
    assert!(value.is_finite(), "{context}: f32 conversion is non-finite");
    value
}

fn json_float_array(value: &Value, context: &str) -> [f32; 4] {
    let values = json_array(value, context);
    assert_eq!(values.len(), 4, "{context}: expected exactly four tracks");
    std::array::from_fn(|index| json_number(&values[index], &format!("{context}[{index}]")))
}

fn json_growth_array(value: &Value, context: &str) -> [Option<f32>; 4] {
    let values = json_array(value, context);
    assert_eq!(values.len(), 4, "{context}: expected exactly four tracks");
    std::array::from_fn(|index| {
        let value = &values[index];
        if value.is_null() {
            None
        } else {
            Some(json_number(value, &format!("{context}[{index}]")))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{json_float_array, json_growth_array, json_number, parse_span_label};
    use serde_json::Value;

    #[test]
    fn rational_values_are_finite_and_malformed_values_are_rejected() {
        assert_eq!(
            json_number(&Value::String("160/3".to_owned()), "growth limit"),
            160.0_f32 / 3.0
        );

        for invalid in ["missing slash", "160/0", "160/-3", "160/3/4", "1e99/1"] {
            let value = Value::String(invalid.to_owned());
            assert!(
                std::panic::catch_unwind(|| json_number(&value, "growth limit")).is_err(),
                "expected {invalid:?} to be rejected"
            );
        }
    }

    #[test]
    fn stage_arrays_require_exactly_four_tracks() {
        let short_array = Value::Array(vec![Value::from(0), Value::from(1), Value::from(2)]);
        assert!(std::panic::catch_unwind(|| json_float_array(&short_array, "base sizes")).is_err());
        assert!(
            std::panic::catch_unwind(|| json_growth_array(&short_array, "growth limits")).is_err()
        );
    }

    #[test]
    fn span_labels_require_canonical_positive_decimal_form() {
        assert_eq!(parse_span_label(&Value::String("span2".into()), "batch"), 2);
        for invalid in ["span02", "span+2", "span0", "span-2", "column2"] {
            let value = Value::String(invalid.to_owned());
            assert!(
                std::panic::catch_unwind(|| parse_span_label(&value, "batch")).is_err(),
                "expected {invalid:?} to be rejected"
            );
        }
    }
}
