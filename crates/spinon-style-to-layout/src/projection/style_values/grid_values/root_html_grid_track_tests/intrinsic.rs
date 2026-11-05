use super::{grid, project_grid_layout};
use spinon_core::NodeId;
use spinon_layout::{GridTrackMaxSizingFunction, GridTrackMinSizingFunction, GridTrackSize};
use spinon_style::{
    ComputedCssGridAutoRows, ComputedCssGridTrackBreadth, ComputedCssGridTrackSize,
    ComputedStyleProfile,
};

fn minimum(breadth: &ComputedCssGridTrackBreadth) -> GridTrackMinSizingFunction {
    match breadth {
        ComputedCssGridTrackBreadth::LengthPx(value) => {
            GridTrackMinSizingFunction::LengthPx(*value)
        }
        ComputedCssGridTrackBreadth::Percentage(value) => {
            GridTrackMinSizingFunction::Percentage(*value)
        }
        ComputedCssGridTrackBreadth::Auto => GridTrackMinSizingFunction::Auto,
        ComputedCssGridTrackBreadth::MinContent => GridTrackMinSizingFunction::MinContent,
        ComputedCssGridTrackBreadth::MaxContent => GridTrackMinSizingFunction::MaxContent,
        ComputedCssGridTrackBreadth::Calc(_) | ComputedCssGridTrackBreadth::Fraction(_) => {
            panic!("the minimum breadth matrix contains only CSS-valid supported values")
        }
    }
}

fn maximum(breadth: &ComputedCssGridTrackBreadth) -> GridTrackMaxSizingFunction {
    match breadth {
        ComputedCssGridTrackBreadth::LengthPx(value) => {
            GridTrackMaxSizingFunction::LengthPx(*value)
        }
        ComputedCssGridTrackBreadth::Percentage(value) => {
            GridTrackMaxSizingFunction::Percentage(*value)
        }
        ComputedCssGridTrackBreadth::Fraction(value) => {
            GridTrackMaxSizingFunction::Fraction(*value)
        }
        ComputedCssGridTrackBreadth::Auto => GridTrackMaxSizingFunction::Auto,
        ComputedCssGridTrackBreadth::MinContent => GridTrackMaxSizingFunction::MinContent,
        ComputedCssGridTrackBreadth::MaxContent => GridTrackMaxSizingFunction::MaxContent,
        ComputedCssGridTrackBreadth::Calc(_) => {
            panic!("the maximum breadth matrix contains no calc values")
        }
    }
}

#[test]
fn root_html_profile_projects_direct_intrinsic_tracks_on_both_axes() {
    let node = NodeId::new(11_280).unwrap();
    let direct = [
        ComputedCssGridTrackSize::Auto,
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::MaxContent,
    ];
    let projected = project_grid_layout(
        node,
        &grid(direct.to_vec(), direct.to_vec()),
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("direct intrinsic explicit tracks are supported in the Root HTML profile");

    let expected = vec![
        GridTrackSize::Auto,
        GridTrackSize::MinContent,
        GridTrackSize::MaxContent,
    ];
    assert_eq!(projected.columns, expected);
    assert_eq!(projected.rows, expected);
}

#[test]
fn root_html_profile_projects_min_content_implicit_rows_into_intrinsic_mode() {
    let node = NodeId::new(11_287).unwrap();
    let mut computed = grid(
        vec![ComputedCssGridTrackSize::LengthPx(40.0)],
        vec![ComputedCssGridTrackSize::LengthPx(10.0)],
    );
    computed.auto_rows =
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::MinContent]);

    let projected = project_grid_layout(
        node,
        &computed,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("the bounded min-content implicit row is supported");

    assert_eq!(
        projected.placement_mode,
        spinon_layout::GridPlacementMode::RootHtmlIntrinsicTrackSizingV1
    );
    assert_eq!(
        projected.auto_rows,
        spinon_layout::GridAutoRows::TrackList(vec![GridTrackSize::MinContent])
    );
}

#[test]
fn root_html_profile_rejects_other_intrinsic_implicit_row_shapes() {
    let node = NodeId::new(11_288).unwrap();
    for tracks in [
        vec![ComputedCssGridTrackSize::MaxContent],
        vec![
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::LengthPx(12.0),
        ],
    ] {
        let mut computed = grid(
            vec![ComputedCssGridTrackSize::LengthPx(40.0)],
            vec![ComputedCssGridTrackSize::LengthPx(10.0)],
        );
        computed.auto_rows = ComputedCssGridAutoRows::TrackList(tracks);

        assert!(matches!(
            project_grid_layout(
                node,
                &computed,
                ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
            ),
            Err(crate::StyleLayoutError::UnsupportedComputedValue {
                node: error_node,
                property: "grid-auto-rows",
                ..
            }) if error_node == node
        ));
    }

    let computed = grid(
        vec![
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::LengthPx(20.0),
        ],
        vec![ComputedCssGridTrackSize::LengthPx(10.0)],
    );
    let mut computed = computed;
    computed.auto_rows =
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::MinContent]);
    assert!(matches!(
        project_grid_layout(
            node,
            &computed,
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
        ),
        Err(crate::StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-auto-rows",
            ..
        }) if error_node == node
    ));
}

#[test]
fn root_html_profile_projects_css_valid_intrinsic_minmax_breadth_matrix() {
    let node = NodeId::new(11_281).unwrap();
    let minima = [
        ComputedCssGridTrackBreadth::LengthPx(20.0),
        ComputedCssGridTrackBreadth::Percentage(0.25),
        ComputedCssGridTrackBreadth::Auto,
        ComputedCssGridTrackBreadth::MinContent,
        ComputedCssGridTrackBreadth::MaxContent,
    ];
    let maxima = [
        ComputedCssGridTrackBreadth::LengthPx(80.0),
        ComputedCssGridTrackBreadth::Percentage(0.5),
        ComputedCssGridTrackBreadth::Auto,
        ComputedCssGridTrackBreadth::MinContent,
        ComputedCssGridTrackBreadth::MaxContent,
        ComputedCssGridTrackBreadth::Fraction(1.0),
    ];

    for min in &minima {
        for max in &maxima {
            let computed = ComputedCssGridTrackSize::MinMax {
                min: min.clone(),
                max: max.clone(),
            };
            let projected = project_grid_layout(
                node,
                &grid(vec![computed], vec![ComputedCssGridTrackSize::Auto]),
                ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
            )
            .unwrap_or_else(|error| panic!("valid minmax({min:?}, {max:?}) rejected: {error:?}"));

            assert_eq!(
                projected.columns,
                vec![GridTrackSize::MinMax {
                    min: minimum(min),
                    max: maximum(max),
                }],
                "minmax({min:?}, {max:?}) must preserve the typed breadths"
            );
            assert_eq!(projected.rows, vec![GridTrackSize::Auto]);
        }
    }
}

#[test]
fn root_html_profile_rejects_invalid_or_out_of_scope_intrinsic_breadths() {
    let node = NodeId::new(11_282).unwrap();
    let cases = [
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::Fraction(1.0),
            max: ComputedCssGridTrackBreadth::MaxContent,
        },
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::LengthPx(-1.0),
            max: ComputedCssGridTrackBreadth::MaxContent,
        },
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::Percentage(f32::INFINITY),
            max: ComputedCssGridTrackBreadth::MaxContent,
        },
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::MinContent,
            max: ComputedCssGridTrackBreadth::Fraction(f32::NAN),
        },
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::Auto,
            max: ComputedCssGridTrackBreadth::LengthPx(f32::NEG_INFINITY),
        },
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::MinContent,
            max: ComputedCssGridTrackBreadth::Calc(spinon_style::ComputedCssMath::LengthPx(10.0)),
        },
    ];

    for track in cases {
        let error = project_grid_layout(
            node,
            &grid(vec![track], vec![ComputedCssGridTrackSize::Auto]),
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
        )
        .expect_err("invalid and out-of-scope minmax breadths must fail closed");
        assert!(matches!(
            error,
            crate::StyleLayoutError::UnsupportedComputedValue {
                node: error_node,
                property: "grid-template-columns",
                ..
            } if error_node == node
        ));
    }
}
