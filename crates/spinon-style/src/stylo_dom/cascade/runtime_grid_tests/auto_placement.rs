use super::*;

#[test]
fn auto_placement_profile_preserves_typed_implicit_row_sizing() {
    for (authored, expected) in [
        ("auto", ComputedCssGridAutoRows::Auto),
        ("12px", ComputedCssGridAutoRows::LengthPx(12.0)),
    ] {
        let fixture = fixture(&format!(
            "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:80px 80px;grid-template-rows:40px;grid-auto-rows:{authored};margin:0;padding:0"
        ));
        let snapshot = compute_runtime_block_flex_grid_auto_placement_cascade_with_stylesheets(
            &fixture.view,
            &[],
            viewport(),
            Default::default(),
        )
        .unwrap();
        let root = snapshot
            .elements
            .iter()
            .find(|style| style.node_id == fixture.root)
            .unwrap();

        assert_eq!(
            snapshot.profile,
            ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
        );
        assert_eq!(root.grid_layout.as_ref().unwrap().auto_rows, expected);
        assert_eq!(root.properties.get("grid-auto-rows").unwrap(), authored);
        assert!(snapshot.diagnostics.is_empty());
    }
}

#[test]
fn auto_placement_profile_preserves_unsupported_auto_row_pattern_for_layout_rejection() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:80px 80px;grid-template-rows:40px;grid-auto-rows:12px 16px;margin:0;padding:0",
    );
    let snapshot = compute_runtime_block_flex_grid_auto_placement_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let root = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == fixture.root)
        .unwrap();

    assert_eq!(
        root.grid_layout.as_ref().unwrap().auto_rows,
        ComputedCssGridAutoRows::Unsupported("12px 16px".to_owned())
    );
}

#[test]
fn auto_placement_inline_gate_allows_grid_auto_rows_and_rejects_auto_columns() {
    let allowed_fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:80px 80px;grid-template-rows:40px;grid-auto-rows:12px;margin:0;padding:0",
    );
    assert_eq!(
        first_unsupported_runtime_block_flex_grid_auto_placement_inline_property(
            &allowed_fixture.view
        ),
        None
    );

    let rejected_fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:80px 80px;grid-template-rows:40px;grid-auto-columns:12px;margin:0;padding:0",
    );
    assert_eq!(
        first_unsupported_runtime_block_flex_grid_auto_placement_inline_property(
            &rejected_fixture.view
        ),
        Some((rejected_fixture.root, "grid-auto-columns".to_owned()))
    );
}
