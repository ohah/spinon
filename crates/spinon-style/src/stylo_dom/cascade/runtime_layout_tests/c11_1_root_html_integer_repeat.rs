use super::super::incremental::{
    IntegerRepeatReservationSite, inject_integer_repeat_reservation_failure,
};
use super::super::{CssViewport, compute_runtime_root_html_text_paint_cascade_with_stylesheets};
use super::view_for_inline_style;
use crate::{ComputedCssGridTrackSize, CssCascadeError};

#[test]
fn root_html_integer_repeat_accepts_exact_expanded_track_limit() {
    let (view, root) = view_for_inline_style(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:repeat(10000,1px);grid-template-rows:40px;margin:0;padding:0",
    );
    let snapshot = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .expect("the default Root HTML profile should preserve the exact 10,000-track limit");
    let grid = snapshot
        .elements
        .iter()
        .find(|element| element.node_id == root)
        .and_then(|element| element.grid_layout.as_ref())
        .expect("exact-limit repeat should produce a typed Grid");

    assert_eq!(grid.columns.len(), 10_000);
    assert!(grid.columns.iter().all(|track| {
        matches!(track, ComputedCssGridTrackSize::LengthPx(value) if *value == 1.0)
    }));
    assert_eq!(grid.columns_line_names.len(), 10_001);
    assert!(grid.columns_line_names.iter().all(Vec::is_empty));
}

#[test]
fn root_html_integer_repeat_allocation_failures_are_contextual_and_recover() {
    let (view, root) = view_for_inline_style(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:repeat(2,[start] 30px [middle] 50px [end]);grid-template-rows:40px;margin:0;padding:0",
    );

    for site in [
        IntegerRepeatReservationSite::Tracks,
        IntegerRepeatReservationSite::LineGroups,
        IntegerRepeatReservationSite::LineNames,
        IntegerRepeatReservationSite::LineNameText,
    ] {
        let error = {
            let _guard = inject_integer_repeat_reservation_failure(site);
            compute_runtime_root_html_text_paint_cascade_with_stylesheets(
                &view,
                &[],
                CssViewport::C04_FIXTURE,
                Default::default(),
            )
            .expect_err("injected integer-repeat reservation failure must abort the cascade")
        };
        assert!(
            matches!(
                error,
                CssCascadeError::GridAllocationFailure {
                    node,
                    property: "grid-template-columns"
                } if node == root
            ),
            "reservation failure at {site:?}: {error:?}"
        );

        let snapshot = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
            &view,
            &[],
            CssViewport::C04_FIXTURE,
            Default::default(),
        )
        .unwrap_or_else(|error| panic!("cascade should recover after {site:?}: {error:?}"));
        let grid = snapshot
            .elements
            .iter()
            .find(|element| element.node_id == root)
            .and_then(|element| element.grid_layout.as_ref())
            .expect("recovered cascade should publish the complete computed Grid");
        assert_eq!(grid.columns.len(), 4, "recovered columns after {site:?}");
        assert_eq!(grid.columns_line_names.len(), 5);
        assert_eq!(grid.columns_line_names[0], ["start"]);
        assert_eq!(grid.columns_line_names[1], ["middle"]);
        assert_eq!(grid.columns_line_names[2], ["end", "start"]);
        assert_eq!(grid.columns_line_names[3], ["middle"]);
        assert_eq!(grid.columns_line_names[4], ["end"]);
    }
}
