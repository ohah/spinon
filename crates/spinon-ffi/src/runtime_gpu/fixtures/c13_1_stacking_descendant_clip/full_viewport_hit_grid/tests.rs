use super::{
    ActualHit, CHROME_GRID_ORACLE, GRID_POINT_COUNT, MISMATCH_PREVIEW_LIMIT, ValidatedGrid,
    compare_grid, parse_pinned_oracle, validate_grid_oracle,
};
use serde_json::{Value, json};

fn pinned_grid() -> (Value, ValidatedGrid) {
    let oracle = parse_pinned_oracle(CHROME_GRID_ORACLE).expect("pinned oracle JSON is valid");
    let grid = validate_grid_oracle(&oracle).expect("pinned grid contract is valid");
    (oracle, grid)
}

fn expected_node_at(grid: &ValidatedGrid, x: usize, y: usize) -> usize {
    grid.rows[y]
        .iter()
        .find(|run| (run.start_x..run.end_x).contains(&x))
        .expect("every row covers the viewport")
        .node_index
}

fn expected_hit(grid: &ValidatedGrid, x: usize, y: usize) -> ActualHit {
    let node_index = expected_node_at(grid, x, y);
    ActualHit {
        node_index: Some(node_index),
        node_id: Some(grid.node_ids[node_index].clone()),
    }
}

#[test]
fn pinned_oracle_compares_every_point_and_emits_a_bounded_success_summary() {
    let (_, grid) = pinned_grid();
    let comparison = compare_grid(&grid, |x, y| expected_hit(&grid, x, y))
        .expect("all fixed grid points are accounted for");
    assert_eq!(comparison.matched_count, GRID_POINT_COUNT);
    assert_eq!(comparison.mismatch_count, 0);
    assert!(comparison.mismatch_preview.is_empty());
    assert_eq!(comparison.actual_identity_sha256, grid.identity_sha256);
}

#[test]
fn mismatch_preview_is_bounded_and_each_failed_point_is_counted_once() {
    let (_, grid) = pinned_grid();
    let comparison = compare_grid(&grid, |x, y| {
        if x < MISMATCH_PREVIEW_LIMIT + 3 && y == 0 {
            ActualHit {
                node_index: None,
                node_id: None,
            }
        } else {
            expected_hit(&grid, x, y)
        }
    })
    .expect("mismatched grid points remain fully accounted for");
    assert_eq!(
        comparison.matched_count + comparison.mismatch_count,
        GRID_POINT_COUNT
    );
    assert_eq!(comparison.mismatch_count, MISMATCH_PREVIEW_LIMIT + 3);
    assert_eq!(comparison.mismatch_preview.len(), MISMATCH_PREVIEW_LIMIT);
    assert_eq!(comparison.mismatch_preview[0]["xCssPx"], json!(0));
    assert_eq!(comparison.mismatch_preview[0]["yCssPx"], json!(0));
    assert_eq!(comparison.mismatch_preview[0]["actualNodeId"], Value::Null);
}

#[test]
fn oracle_rejects_missing_or_extra_rows_runs_and_schema_fields() {
    let (oracle, _) = pinned_grid();

    let mut missing_row = oracle.clone();
    missing_row["rows"].as_array_mut().unwrap().pop();
    assert!(validate_grid_oracle(&missing_row).is_err());

    let mut extra_row = oracle.clone();
    extra_row["rows"]
        .as_array_mut()
        .unwrap()
        .push(json!([[0, 540]]));
    assert!(validate_grid_oracle(&extra_row).is_err());

    let mut missing_schema_field = oracle.clone();
    missing_schema_field
        .as_object_mut()
        .unwrap()
        .remove("source");
    assert!(validate_grid_oracle(&missing_schema_field).is_err());

    let mut extra_schema_field = oracle.clone();
    extra_schema_field["unreviewed"] = json!(true);
    assert!(validate_grid_oracle(&extra_schema_field).is_err());
}

#[test]
fn oracle_rejects_malformed_runs_and_incomplete_or_overlapping_row_coverage() {
    let (oracle, _) = pinned_grid();

    let mut wrong_run_shape = oracle.clone();
    wrong_run_shape["rows"][0][0] = json!([0, 540, 1]);
    assert!(validate_grid_oracle(&wrong_run_shape).is_err());

    let mut zero_width = oracle.clone();
    zero_width["rows"][0][0][1] = json!(0);
    assert!(validate_grid_oracle(&zero_width).is_err());

    let mut negative_width = oracle.clone();
    negative_width["rows"][0][0][1] = json!(-1);
    assert!(validate_grid_oracle(&negative_width).is_err());

    let mut unknown_node = oracle.clone();
    unknown_node["rows"][0][0][0] = json!(13);
    assert!(validate_grid_oracle(&unknown_node).is_err());

    let mut adjacent_duplicate_runs = oracle.clone();
    adjacent_duplicate_runs["rows"][0]
        .as_array_mut()
        .unwrap()
        .insert(0, json!([0, 1]));
    assert!(validate_grid_oracle(&adjacent_duplicate_runs).is_err());

    let mut short_row = oracle.clone();
    short_row["rows"][0][0][1] = json!(539);
    assert!(validate_grid_oracle(&short_row).is_err());

    let mut long_row = oracle.clone();
    long_row["rows"][0][0][1] = json!(541);
    assert!(validate_grid_oracle(&long_row).is_err());
}

#[test]
fn oracle_rejects_metadata_digest_node_order_and_edge_control_mutations() {
    let (oracle, _) = pinned_grid();

    let mut wrong_viewport = oracle.clone();
    wrong_viewport["viewport"]["width"] = json!(541);
    assert!(validate_grid_oracle(&wrong_viewport).is_err());

    let mut wrong_point_count = oracle.clone();
    wrong_point_count["pointCount"] = json!(69_119);
    assert!(validate_grid_oracle(&wrong_point_count).is_err());

    let mut wrong_node_order = oracle.clone();
    wrong_node_order["nodeIds"]
        .as_array_mut()
        .unwrap()
        .swap(1, 2);
    assert!(validate_grid_oracle(&wrong_node_order).is_err());

    let mut wrong_identity_digest = oracle.clone();
    wrong_identity_digest["expandedNodeIndexSha256"] = json!("0".repeat(64));
    assert!(validate_grid_oracle(&wrong_identity_digest).is_err());

    let mut wrong_edge_hit = oracle.clone();
    wrong_edge_hit["edgeControls"][0]["hitId"] = json!(null);
    assert!(validate_grid_oracle(&wrong_edge_hit).is_err());

    let mut wrong_edge_coordinate = oracle.clone();
    wrong_edge_coordinate["edgeControls"][4]["x"] = json!(539);
    assert!(validate_grid_oracle(&wrong_edge_coordinate).is_err());

    let mut extra_edge_field = oracle.clone();
    extra_edge_field["edgeControls"][0]["note"] = json!("unexpected");
    assert!(validate_grid_oracle(&extra_edge_field).is_err());
}

#[test]
fn raw_oracle_must_match_its_pinned_file_digest() {
    let changed = CHROME_GRID_ORACLE.replacen("\"rowCount\": 128", "\"rowCount\": 127", 1);
    assert!(parse_pinned_oracle(&changed).is_err());
}
