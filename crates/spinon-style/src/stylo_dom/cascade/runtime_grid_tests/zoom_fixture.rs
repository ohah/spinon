use super::*;

#[test]
fn zoom_is_rejected_by_default_and_fixture_route_preserves_scaled_typed_lengths() {
    let fixture = fixture_with_base_url(
        "display:grid;box-sizing:border-box;width:60px;height:300px;zoom:2;grid-template-columns:60px;grid-template-rows:none;grid-auto-rows:fit-content(40px);margin:0;padding:0",
        "chrome://spinon.invalid/c11-4/fit-content-zoom.html",
    );
    assert_eq!(
        super::super::first_unsupported_runtime_block_flex_grid_track_sizing_inline_property(
            &fixture.view
        )
        .map(|(_, property)| property),
        Some("zoom".to_owned()),
    );
    assert_eq!(
        super::super::first_unsupported_runtime_block_flex_grid_track_sizing_zoom_fixture_test_inline_property(
            &fixture.view
        ),
        None,
    );
    let unsupported = fixture_with_base_url(
        "display:grid;box-sizing:border-box;width:60px;height:300px;zoom:2;float:left;grid-template-columns:60px;grid-template-rows:none;grid-auto-rows:fit-content(40px);margin:0;padding:0",
        "chrome://spinon.invalid/c11-4/fit-content-zoom.html",
    );
    assert_eq!(
        super::super::first_unsupported_runtime_block_flex_grid_track_sizing_zoom_fixture_test_inline_property(
            &unsupported.view
        )
        .map(|(_, property)| property),
        Some("float".to_owned()),
    );
    assert!(
        compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &fixture.view,
            &[],
            viewport(),
            Default::default(),
        )
        .is_err()
    );
    let snapshot = super::super::compute_runtime_block_flex_grid_track_sizing_zoom_fixture_test_cascade_with_stylesheets(
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
    let child = snapshot
        .elements
        .iter()
        .find(|style| style.node_id != fixture.root)
        .unwrap();
    assert_eq!(root.c11_4_zoom.unwrap().specified, 2.0);
    assert_eq!(root.c11_4_zoom.unwrap().effective, 2.0);
    assert_eq!(child.c11_4_zoom.unwrap().specified, 1.0);
    assert_eq!(child.c11_4_zoom.unwrap().effective, 2.0);
    assert_eq!(
        root.layout_dimensions.width,
        crate::ComputedCssDimension::LengthPx(120.0)
    );
    assert_eq!(
        root.layout_dimensions.height,
        crate::ComputedCssDimension::LengthPx(600.0)
    );
    assert_eq!(
        root.grid_layout.as_ref().unwrap().auto_rows,
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::FitContent(
            ComputedCssGridTrackLengthPercentage::LengthPx(80.0),
        )]),
    );
    assert_eq!(
        child.layout_dimensions.height,
        crate::ComputedCssDimension::LengthPx(20.0)
    );
}
