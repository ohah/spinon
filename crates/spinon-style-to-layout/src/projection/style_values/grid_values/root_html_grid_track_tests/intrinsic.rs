use super::super::root_html_intrinsic_auto_rows_candidate;
use super::{grid, project_grid_layout};
use spinon_core::NodeId;
use spinon_layout::{GridTrackMaxSizingFunction, GridTrackMinSizingFunction, GridTrackSize};
use spinon_style::{
    ComputedCssGridArea, ComputedCssGridAutoColumns, ComputedCssGridAutoRepeat,
    ComputedCssGridAutoRepeatMode, ComputedCssGridAutoRows, ComputedCssGridTemplateAreas,
    ComputedCssGridTrackBreadth, ComputedCssGridTrackSize, ComputedStyleProfile,
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
fn intrinsic_auto_row_candidate_keeps_the_existing_combined_column_boundary() {
    let intrinsic_columns = [
        ComputedCssGridTrackSize::Auto,
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::MaxContent,
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::MinContent,
            max: ComputedCssGridTrackBreadth::Fraction(1.0),
        },
    ];
    for column in intrinsic_columns {
        let mut input = grid(vec![column], vec![ComputedCssGridTrackSize::LengthPx(10.0)]);
        input.auto_rows =
            ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::MinContent]);
        assert!(
            root_html_intrinsic_auto_rows_candidate(&input),
            "existing intrinsic column should remain a combined auto-row candidate: {input:?}"
        );
    }

    let mut intrinsic_row = grid(
        vec![ComputedCssGridTrackSize::MinContent],
        vec![ComputedCssGridTrackSize::MinContent],
    );
    intrinsic_row.auto_rows =
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::MinContent]);
    assert!(!root_html_intrinsic_auto_rows_candidate(&intrinsic_row));

    let mut fixed_column = grid(
        vec![ComputedCssGridTrackSize::LengthPx(40.0)],
        vec![ComputedCssGridTrackSize::LengthPx(10.0)],
    );
    fixed_column.auto_rows =
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::MinContent]);
    assert!(root_html_intrinsic_auto_rows_candidate(&fixed_column));

    let mut unsupported_implicit_row = grid(
        vec![ComputedCssGridTrackSize::MinContent],
        vec![ComputedCssGridTrackSize::LengthPx(10.0)],
    );
    unsupported_implicit_row.auto_rows =
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::MaxContent]);
    assert!(!root_html_intrinsic_auto_rows_candidate(
        &unsupported_implicit_row
    ));

    let mut named_areas = grid(
        vec![ComputedCssGridTrackSize::MinContent],
        vec![ComputedCssGridTrackSize::LengthPx(10.0)],
    );
    named_areas.auto_rows =
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::MinContent]);
    named_areas.template_areas = Some(ComputedCssGridTemplateAreas {
        rows: 1,
        columns: 1,
        areas: vec![ComputedCssGridArea {
            name: "content".to_owned(),
            row_start: 0,
            row_end: 1,
            column_start: 0,
            column_end: 1,
        }],
    });
    assert!(!root_html_intrinsic_auto_rows_candidate(&named_areas));

    let mut column_auto_repeat = grid(
        vec![ComputedCssGridTrackSize::MinContent],
        vec![ComputedCssGridTrackSize::LengthPx(10.0)],
    );
    column_auto_repeat.auto_rows =
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::MinContent]);
    column_auto_repeat.columns_auto_repeat = Some(ComputedCssGridAutoRepeat {
        mode: ComputedCssGridAutoRepeatMode::AutoFill,
        start: 0,
        end: 1,
        line_names: vec![Vec::new(), Vec::new()],
        fixed_repeat_minmax_track_indices: Vec::new(),
    });
    assert!(!root_html_intrinsic_auto_rows_candidate(
        &column_auto_repeat
    ));

    let mut row_auto_repeat = grid(
        vec![ComputedCssGridTrackSize::MinContent],
        vec![ComputedCssGridTrackSize::LengthPx(10.0)],
    );
    row_auto_repeat.auto_rows =
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::MinContent]);
    row_auto_repeat.rows_auto_repeat = Some(ComputedCssGridAutoRepeat {
        mode: ComputedCssGridAutoRepeatMode::AutoFill,
        start: 0,
        end: 1,
        line_names: vec![Vec::new(), Vec::new()],
        fixed_repeat_minmax_track_indices: Vec::new(),
    });
    assert!(!root_html_intrinsic_auto_rows_candidate(&row_auto_repeat));

    let mut multiple_explicit_rows = grid(
        vec![ComputedCssGridTrackSize::MinContent],
        vec![
            ComputedCssGridTrackSize::LengthPx(10.0),
            ComputedCssGridTrackSize::LengthPx(10.0),
        ],
    );
    multiple_explicit_rows.auto_rows =
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::MinContent]);
    assert!(!root_html_intrinsic_auto_rows_candidate(
        &multiple_explicit_rows
    ));

    let mut zero_explicit_row = grid(
        vec![ComputedCssGridTrackSize::MinContent],
        vec![ComputedCssGridTrackSize::LengthPx(0.0)],
    );
    zero_explicit_row.auto_rows =
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::MinContent]);
    assert!(!root_html_intrinsic_auto_rows_candidate(&zero_explicit_row));

    let mut implicit_columns = grid(
        vec![ComputedCssGridTrackSize::MinContent],
        vec![ComputedCssGridTrackSize::LengthPx(10.0)],
    );
    implicit_columns.auto_rows =
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::MinContent]);
    implicit_columns.auto_columns = ComputedCssGridAutoColumns::LengthPx(8.0);
    assert!(!root_html_intrinsic_auto_rows_candidate(&implicit_columns));

    let mut column_flow = grid(
        vec![ComputedCssGridTrackSize::MinContent],
        vec![ComputedCssGridTrackSize::LengthPx(10.0)],
    );
    column_flow.auto_rows =
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::MinContent]);
    column_flow.auto_flow = spinon_style::ComputedCssGridAutoFlow::Column;
    assert!(!root_html_intrinsic_auto_rows_candidate(&column_flow));
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
