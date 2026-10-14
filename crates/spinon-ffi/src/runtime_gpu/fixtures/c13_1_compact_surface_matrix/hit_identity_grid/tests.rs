use super::{
    ActualHit, CHROME_ORACLE, EXPECTED_GRID_FILE_SHA256, EXPECTED_IDENTITY_SHA256,
    GRID_POINT_COUNT, HIT_IDENTITY_GRID_ORACLE, NODE_COUNT, compare_grid, parse_pinned_grid_oracle,
    validate_grid_oracle, validate_runtime_node_order,
};
use serde_json::Value;
use spinon_runtime::RuntimeLayoutFrame;

fn pinned_inputs() -> (Value, Value) {
    (
        serde_json::from_str(HIT_IDENTITY_GRID_ORACLE).unwrap(),
        serde_json::from_str(CHROME_ORACLE).unwrap(),
    )
}

fn expected_index_at(grid: &super::ValidatedGrid, x: usize, y: usize) -> Option<usize> {
    grid.rows[y]
        .iter()
        .find(|run| x >= run.start_x && x < run.end_x)
        .map(|run| run.node_index)
}

fn expected_hit(grid: &super::ValidatedGrid, x: usize, y: usize) -> ActualHit {
    let node_index = expected_index_at(grid, x, y);
    ActualHit {
        node_index,
        node_id: node_index.map(|index| grid.node_ids[index].clone()),
    }
}

fn runtime_frame(node_id: u64) -> RuntimeLayoutFrame {
    RuntimeLayoutFrame {
        node_id,
        x: 0.0,
        y: 0.0,
        width: 1.0,
        height: 1.0,
    }
}

#[test]
fn rejects_frame_order_that_could_remap_node_ids_to_different_chrome_ids() {
    let ordered = (1..=NODE_COUNT as u64)
        .map(runtime_frame)
        .collect::<Vec<_>>();
    assert!(validate_runtime_node_order(&ordered).is_ok());

    let mut reordered = ordered;
    reordered.swap(7, 8);
    assert!(validate_runtime_node_order(&reordered).is_err());
}

#[test]
fn full_grid_pins_source_browser_rows_histogram_fixed_samples_and_digest() {
    let (grid, chrome) = pinned_inputs();
    let validated = validate_grid_oracle(&grid, &chrome).unwrap();
    assert_eq!(validated.node_ids.len(), NODE_COUNT);
    assert_eq!(validated.rows.len(), 100);
    assert_eq!(validated.rows.iter().map(Vec::len).sum::<usize>(), 870);
    assert_eq!(
        validated
            .rows
            .iter()
            .flatten()
            .map(|run| run.end_x - run.start_x)
            .sum::<usize>(),
        GRID_POINT_COUNT
    );
    assert_eq!(validated.identity_sha256, EXPECTED_IDENTITY_SHA256);
}

#[test]
fn app_comparison_mapping_queries_each_integer_point_once_and_matches_full_digest() {
    let (grid_value, chrome) = pinned_inputs();
    let grid = validate_grid_oracle(&grid_value, &chrome).unwrap();
    let mut queries = 0usize;
    let result = compare_grid(&grid, |x, y| {
        queries += 1;
        expected_hit(&grid, x, y)
    })
    .unwrap();
    assert_eq!(queries, GRID_POINT_COUNT);
    assert_eq!(result.matched_count, GRID_POINT_COUNT);
    assert_eq!(result.mismatch_count, 0);
    assert!(result.mismatch_preview.is_empty());
    assert_eq!(result.actual_identity_sha256, EXPECTED_IDENTITY_SHA256);
}

#[test]
fn app_comparison_reports_a_single_edge_mismatch_and_bounds_the_preview() {
    let (grid_value, chrome) = pinned_inputs();
    let grid = validate_grid_oracle(&grid_value, &chrome).unwrap();
    let one_mismatch = compare_grid(&grid, |x, y| {
        if (x, y) == (299, 99) {
            ActualHit {
                node_index: None,
                node_id: None,
            }
        } else {
            expected_hit(&grid, x, y)
        }
    })
    .unwrap();
    assert_eq!(one_mismatch.matched_count, GRID_POINT_COUNT - 1);
    assert_eq!(one_mismatch.mismatch_count, 1);
    assert_eq!(one_mismatch.mismatch_preview.len(), 1);
    assert_eq!(one_mismatch.mismatch_preview[0]["xCssPx"], 299);
    assert_eq!(one_mismatch.mismatch_preview[0]["yCssPx"], 99);

    let all_missing = compare_grid(&grid, |_, _| ActualHit {
        node_index: None,
        node_id: None,
    })
    .unwrap();
    assert_eq!(all_missing.matched_count, 0);
    assert_eq!(all_missing.mismatch_count, GRID_POINT_COUNT);
    assert_eq!(all_missing.mismatch_preview.len(), 16);
    assert_ne!(all_missing.actual_identity_sha256, EXPECTED_IDENTITY_SHA256);
}

#[test]
fn rejects_source_browser_grid_file_and_identity_digest_mutations() {
    let (original_grid, original_chrome) = pinned_inputs();
    let mut changed_source = original_chrome.clone();
    changed_source["sources"]["htmlSha256"] = Value::from("changed");
    assert!(validate_grid_oracle(&original_grid, &changed_source).is_err());

    let mut changed_browser = original_grid.clone();
    changed_browser["browser"]["userAgent"] = Value::from("different Chrome");
    assert!(validate_grid_oracle(&changed_browser, &original_chrome).is_err());

    let mut changed_grid_digest = original_grid;
    changed_grid_digest["pixelGridIdentitySha256"] = Value::from("changed");
    assert!(validate_grid_oracle(&changed_grid_digest, &original_chrome).is_err());
    assert_eq!(EXPECTED_GRID_FILE_SHA256.len(), 64);
    let mutated_source = HIT_IDENTITY_GRID_ORACLE.replacen("captured", "changed", 1);
    assert!(parse_pinned_grid_oracle(&mutated_source).is_err());
}

#[test]
fn rejects_row_gaps_overlaps_empty_rows_and_out_of_bounds_runs() {
    let (original, chrome) = pinned_inputs();
    let mut gap = original.clone();
    gap["rows"][0][0][0] = Value::from(1);
    assert!(validate_grid_oracle(&gap, &chrome).is_err());

    let mut overlap = original.clone();
    overlap["rows"][0][1][0] = Value::from(95);
    assert!(validate_grid_oracle(&overlap, &chrome).is_err());

    let mut empty = original.clone();
    empty["rows"][0] = Value::Array(Vec::new());
    assert!(validate_grid_oracle(&empty, &chrome).is_err());

    let mut out_of_bounds = original;
    out_of_bounds["rows"][0][0][1] = Value::from(301);
    assert!(validate_grid_oracle(&out_of_bounds, &chrome).is_err());
}

#[test]
fn rejects_unknown_or_duplicate_node_identities() {
    let (original, chrome) = pinned_inputs();
    let mut unknown = original.clone();
    unknown["rows"][0][0][2] = Value::from(NODE_COUNT);
    assert!(validate_grid_oracle(&unknown, &chrome).is_err());

    let mut duplicate = original;
    duplicate["nodeIds"][1] = duplicate["nodeIds"][0].clone();
    assert!(validate_grid_oracle(&duplicate, &chrome).is_err());
}

#[test]
fn rejects_histogram_fixed_subset_and_viewport_mutations() {
    let (original, chrome) = pinned_inputs();
    let mut histogram = original.clone();
    histogram["histogram"][0] = Value::from(0);
    assert!(validate_grid_oracle(&histogram, &chrome).is_err());

    let mut viewport = original.clone();
    viewport["viewport"]["width"] = Value::from(301);
    assert!(validate_grid_oracle(&viewport, &chrome).is_err());

    let mut sample_mismatch = original;
    sample_mismatch["rows"][0][0][2] = Value::from(2);
    assert!(validate_grid_oracle(&sample_mismatch, &chrome).is_err());
}
