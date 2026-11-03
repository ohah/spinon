use super::{grid, project_grid_layout};
use spinon_core::NodeId;
use spinon_layout::{
    GridPlacementMode, GridTrackMaxSizingFunction, GridTrackMinSizingFunction, GridTrackSize,
};
use spinon_style::{ComputedCssGridTrackBreadth, ComputedCssGridTrackSize, ComputedStyleProfile};

fn auto_minimum_fraction_track(factor: f32) -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::MinMax {
        min: ComputedCssGridTrackBreadth::Auto,
        max: ComputedCssGridTrackBreadth::Fraction(factor),
    }
}

#[test]
fn root_html_profile_projects_auto_minimum_fraction_tracks_to_implicit_rows_mode() {
    let node = NodeId::new(11_252).unwrap();
    let input = grid(
        vec![
            auto_minimum_fraction_track(1.0),
            ComputedCssGridTrackSize::LengthPx(40.0),
        ],
        vec![ComputedCssGridTrackSize::LengthPx(40.0)],
    );

    let projected = project_grid_layout(
        node,
        &input,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("auto-minimum flex maximum is part of the bounded Root HTML profile");

    assert_eq!(
        projected.columns,
        vec![
            GridTrackSize::MinMax {
                min: GridTrackMinSizingFunction::Auto,
                max: GridTrackMaxSizingFunction::Fraction(1.0),
            },
            GridTrackSize::LengthPx(40.0),
        ]
    );
    assert_eq!(projected.rows, vec![GridTrackSize::LengthPx(40.0)]);
    assert_eq!(
        projected.placement_mode,
        GridPlacementMode::RootHtmlImplicitRowsV1
    );
    assert_eq!(projected.auto_rows, spinon_layout::GridAutoRows::Auto);
}

#[test]
fn root_html_profile_preserves_zero_fraction_auto_minimum_shape() {
    let node = NodeId::new(11_253).unwrap();
    let input = grid(
        vec![
            auto_minimum_fraction_track(0.0),
            ComputedCssGridTrackSize::LengthPx(40.0),
        ],
        vec![ComputedCssGridTrackSize::LengthPx(40.0)],
    );

    let projected = project_grid_layout(
        node,
        &input,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("zero is a valid non-negative flex factor");

    assert_eq!(
        projected.columns[0],
        GridTrackSize::MinMax {
            min: GridTrackMinSizingFunction::Auto,
            max: GridTrackMaxSizingFunction::Fraction(0.0),
        }
    );
}

#[test]
fn root_html_profile_rejects_auto_minimum_without_fraction_maximum() {
    let node = NodeId::new(11_254).unwrap();
    for track in [
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::Auto,
            max: ComputedCssGridTrackBreadth::LengthPx(40.0),
        },
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::Auto,
            max: ComputedCssGridTrackBreadth::Percentage(0.5),
        },
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::Auto,
            max: ComputedCssGridTrackBreadth::Auto,
        },
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::Auto,
            max: ComputedCssGridTrackBreadth::Fraction(-1.0),
        },
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::Auto,
            max: ComputedCssGridTrackBreadth::Fraction(f32::NAN),
        },
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::Auto,
            max: ComputedCssGridTrackBreadth::Fraction(f32::INFINITY),
        },
    ] {
        let input = grid(vec![track], vec![ComputedCssGridTrackSize::LengthPx(40.0)]);
        let error = project_grid_layout(
            node,
            &input,
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
        )
        .expect_err("only finite non-negative fraction maximum is admitted with auto minimum");
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
