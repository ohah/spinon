use super::*;

#[test]
fn track_sizing_profile_preserves_intrinsic_track_functions_and_calc_values() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:auto min-content max-content minmax(20px,max-content) minmax(min-content,2fr) fit-content(80px) calc(30px + 5%);grid-template-rows:fit-content(50%);margin:0;padding:0",
    );
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
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
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
    );
    assert_eq!(
        grid.columns,
        [
            ComputedCssGridTrackSize::Auto,
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::MaxContent,
            ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::LengthPx(20.0),
                max: ComputedCssGridTrackBreadth::MaxContent,
            },
            ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::MinContent,
                max: ComputedCssGridTrackBreadth::Fraction(2.0),
            },
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
                80.0
            )),
            ComputedCssGridTrackSize::Calc(ComputedCssMath::Sum(vec![
                ComputedCssMath::Percentage(0.05),
                ComputedCssMath::LengthPx(30.0),
            ])),
        ]
    );
    assert_eq!(
        grid.rows,
        [ComputedCssGridTrackSize::FitContent(
            ComputedCssGridTrackLengthPercentage::Percentage(0.5)
        )]
    );
}

#[test]
fn track_sizing_profile_preserves_grid_auto_track_lists_on_both_axes() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:160px;height:80px;grid-template-columns:24px;grid-template-rows:12px;grid-auto-columns:min-content max-content 25% 1fr minmax(20px,40px) fit-content(30px) calc(10% + 5px);grid-auto-rows:max-content 10px min-content;margin:0;padding:0",
    );
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();
    let grid = snapshot.elements[0].grid_layout.as_ref().unwrap();

    assert_eq!(
        grid.auto_columns,
        crate::ComputedCssGridAutoColumns::TrackList(vec![
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::MaxContent,
            ComputedCssGridTrackSize::Percentage(0.25),
            ComputedCssGridTrackSize::Fraction(1.0),
            ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::LengthPx(20.0),
                max: ComputedCssGridTrackBreadth::LengthPx(40.0),
            },
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
                30.0
            ),),
            ComputedCssGridTrackSize::Calc(ComputedCssMath::Sum(vec![
                ComputedCssMath::Percentage(0.1),
                ComputedCssMath::LengthPx(5.0),
            ])),
        ])
    );
    assert_eq!(
        grid.auto_rows,
        ComputedCssGridAutoRows::TrackList(vec![
            ComputedCssGridTrackSize::MaxContent,
            ComputedCssGridTrackSize::LengthPx(10.0),
            ComputedCssGridTrackSize::MinContent,
        ])
    );
    assert!(snapshot.diagnostics.is_empty());
}

#[test]
fn track_sizing_auto_track_list_reservation_failures_keep_axis_context() {
    use super::super::incremental::{
        IntegerRepeatReservationSite, inject_integer_repeat_reservation_failure,
    };

    for (property, site, declaration) in [
        (
            "grid-auto-rows",
            IntegerRepeatReservationSite::AutoRows,
            "grid-auto-rows:10px max-content",
        ),
        (
            "grid-auto-columns",
            IntegerRepeatReservationSite::AutoColumns,
            "grid-auto-columns:min-content max-content",
        ),
    ] {
        let fixture = fixture(&format!(
            "display:grid;grid-template-columns:40px;grid-template-rows:20px;{declaration}"
        ));
        let error = {
            let _guard = inject_integer_repeat_reservation_failure(site);
            compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
                &fixture.view,
                &[],
                viewport(),
                Default::default(),
            )
            .unwrap_err()
        };
        assert!(matches!(
            error,
            crate::CssCascadeError::GridAllocationFailure {
                node,
                property: error_property,
            } if node == fixture.root && error_property == property
        ));
    }
}

#[test]
fn previous_template_profile_rejects_intrinsic_track_functions() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:minmax(20px,1fr);grid-template-rows:40px;margin:0;padding:0",
    );

    let error = compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap_err();

    assert!(matches!(
        error,
        crate::stylo_dom::cascade::CssCascadeError::UnsupportedComputedGridValue {
            property: "grid-template-columns",
            ..
        }
    ));
}

#[test]
fn track_sizing_preserves_zero_fraction_in_minmax_maximum() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:minmax(auto,0fr);grid-template-rows:40px;margin:0;padding:0",
    );

    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();

    let grid = snapshot.elements[0].grid_layout.as_ref().unwrap();
    assert_eq!(
        grid.columns,
        [ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::Auto,
            max: ComputedCssGridTrackBreadth::Fraction(0.0),
        }]
    );
}

#[test]
fn track_sizing_carries_aspect_ratio_for_transferred_size_suggestions() {
    let fixture = fixture(
        "display:grid;aspect-ratio:2 / 1;grid-template-columns:100px;grid-template-rows:40px",
    );
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
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
    assert_eq!(style.layout_aspect_ratio, Some(2.0));
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn overflow_fixture_cascade_keeps_axis_conversion_out_of_the_base_profile() {
    for (authored_x, expected_x, expected_y) in [
        (
            "visible",
            crate::ComputedCssOverflow::Visible,
            crate::ComputedCssOverflow::Visible,
        ),
        (
            "clip",
            crate::ComputedCssOverflow::Clip,
            crate::ComputedCssOverflow::Visible,
        ),
        (
            "hidden",
            crate::ComputedCssOverflow::Hidden,
            crate::ComputedCssOverflow::Auto,
        ),
        (
            "auto",
            crate::ComputedCssOverflow::Auto,
            crate::ComputedCssOverflow::Auto,
        ),
        (
            "scroll",
            crate::ComputedCssOverflow::Scroll,
            crate::ComputedCssOverflow::Auto,
        ),
    ] {
        let fixture = fixture(&format!(
            "display:grid;box-sizing:border-box;width:80px;height:20px;grid-template-columns:minmax(auto,max-content) minmax(auto,max-content);grid-template-rows:20px;overflow-x:{authored_x};overflow-y:visible;margin:0;padding:0"
        ));
        let authored = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &fixture.view,
            &[],
            viewport(),
            Default::default(),
        )
        .unwrap_err();
        assert!(matches!(
            authored,
            crate::CssCascadeError::UnsupportedInlineAuthorProperty { .. }
        ));

        let snapshot =
            crate::compute_runtime_block_flex_grid_track_sizing_overflow_fixture_test_cascade_with_stylesheets(
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
        assert_eq!(style.layout_overflow_x, Some(expected_x), "{authored_x}");
        assert_eq!(style.layout_overflow_y, Some(expected_y), "{authored_x}");
    }
}

#[test]
fn track_sizing_profile_rejects_unsupported_visibility_collapse_in_author_stylesheets() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:60px;height:20px;grid-template-columns:60px;grid-template-rows:20px;margin:0;padding:0",
    );
    let stylesheet = StylesheetSource {
        id: "visibility-collapse".to_owned(),
        base_url: "https://spinon.invalid/visibility-collapse.css".to_owned(),
        origin: CssOrigin::Author,
        css: "#root { visibility: collapse; }".to_owned(),
    };

    let error = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
        &[stylesheet],
        viewport(),
        Default::default(),
    )
    .unwrap_err();

    assert!(matches!(
        error,
        crate::CssCascadeError::UnsupportedAuthorCss {
            stylesheet_id,
            feature,
        } if stylesheet_id == "visibility-collapse"
            && feature == "지원하지 않는 CSS 선언 visibility 값"
    ));
}
