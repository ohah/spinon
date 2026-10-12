use super::*;

#[test]
fn grid_profile_copies_explicit_px_percentage_fraction_and_flow_as_typed_values() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:60px 25% 1fr;grid-template-rows:40px 1fr;grid-auto-flow:row;column-gap:10px;row-gap:5px;margin:0;padding:0",
    );
    let snapshot = compute_runtime_block_flex_grid_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let style = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == fixture.root)
        .unwrap();
    let grid = style.grid_layout.as_ref().unwrap();

    assert_eq!(
        snapshot.profile,
        ComputedStyleProfile::RuntimeBlockFlexGridV1
    );
    assert_eq!(
        grid.columns,
        [
            ComputedCssGridTrackSize::LengthPx(60.0),
            ComputedCssGridTrackSize::Percentage(0.25),
            ComputedCssGridTrackSize::Fraction(1.0),
        ]
    );
    assert_eq!(
        grid.rows,
        [
            ComputedCssGridTrackSize::LengthPx(40.0),
            ComputedCssGridTrackSize::Fraction(1.0),
        ]
    );
    assert_eq!(grid.auto_flow, ComputedCssGridAutoFlow::Row);
}

#[test]
fn grid_profile_preserves_dense_column_flow_for_layout_to_reject_explicitly() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:1fr;grid-template-rows:1fr;grid-auto-flow:column dense;margin:0;padding:0",
    );
    let snapshot = compute_runtime_block_flex_grid_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let grid = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == fixture.root)
        .unwrap()
        .grid_layout
        .as_ref()
        .unwrap();
    assert_eq!(grid.auto_flow, ComputedCssGridAutoFlow::ColumnDense);
}

#[test]
fn grid_profile_computes_author_stylesheets_with_typed_tracks() {
    let fixture = fixture("box-sizing:border-box;width:300px;height:200px;margin:0;padding:0");
    let snapshot = compute_runtime_block_flex_grid_cascade_with_stylesheets(
        &fixture.view,
        &[StylesheetSource {
            id: "grid-author".to_owned(),
            base_url: "https://spinon.invalid/c11/grid.css".to_owned(),
            origin: CssOrigin::Author,
            css: "#root { display:grid; grid-template-columns:80px 1fr; grid-template-rows:40px 1fr; column-gap:8px; row-gap:6px }".to_owned(),
        }],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let grid = snapshot
        .elements
        .iter()
        .find(|style| style.node_id == fixture.root)
        .unwrap()
        .grid_layout
        .as_ref()
        .unwrap();

    assert_eq!(grid.columns[0], ComputedCssGridTrackSize::LengthPx(80.0));
    assert_eq!(grid.columns[1], ComputedCssGridTrackSize::Fraction(1.0));
    assert_eq!(grid.rows[0], ComputedCssGridTrackSize::LengthPx(40.0));
    assert_eq!(grid.rows[1], ComputedCssGridTrackSize::Fraction(1.0));
    assert!(snapshot.diagnostics.is_empty());
}

#[test]
fn unsupported_track_syntax_fails_during_typed_cascade_extraction() {
    for tracks in ["none", "repeat(2, 50px)", "minmax(40px, 1fr)", "auto"] {
        let fixture = fixture(&format!(
            "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:{tracks};grid-template-rows:1fr;margin:0;padding:0"
        ));
        let error = compute_runtime_block_flex_grid_cascade_with_stylesheets(
            &fixture.view,
            &[],
            viewport(),
            Default::default(),
        )
        .unwrap_err();
        assert!(
            matches!(
                error,
                crate::CssCascadeError::UnsupportedComputedGridValue {
                    node,
                    property: "grid-template-columns",
                    ..
                } if node == fixture.root
            ),
            "{tracks}: {error}"
        );
    }
}

#[test]
fn inline_grid_placement_and_aspect_ratio_are_outside_the_c11_1_profile() {
    for (unsupported, property) in [
        ("grid-column:2", "grid-column-start"),
        ("grid-row:span 2", "grid-row-start"),
        ("aspect-ratio:2", "aspect-ratio"),
    ] {
        let fixture = fixture(&format!(
            "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:1fr;grid-template-rows:1fr;margin:0;padding:0;{unsupported}"
        ));
        assert_eq!(
            first_unsupported_runtime_block_flex_grid_inline_property(&fixture.view),
            Some((fixture.root, property.to_owned()))
        );
    }
}
