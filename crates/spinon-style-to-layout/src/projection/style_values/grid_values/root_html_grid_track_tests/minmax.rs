use super::{grid, project_grid_layout};
use spinon_core::NodeId;
use spinon_layout::{GridTrackMaxSizingFunction, GridTrackMinSizingFunction, GridTrackSize};
use spinon_style::{ComputedCssGridTrackBreadth, ComputedCssGridTrackSize, ComputedStyleProfile};

#[test]
fn root_html_profile_projects_bounded_minmax_explicit_tracks() {
    let node = NodeId::new(11_250).unwrap();
    let input = grid(
        vec![
            ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::LengthPx(20.0),
                max: ComputedCssGridTrackBreadth::LengthPx(80.0),
            },
            ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::Percentage(0.25),
                max: ComputedCssGridTrackBreadth::Fraction(1.0),
            },
        ],
        vec![ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::LengthPx(10.0),
            max: ComputedCssGridTrackBreadth::Percentage(0.5),
        }],
    );

    let projected = project_grid_layout(
        node,
        &input,
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
    )
    .expect("bounded px/percentage/fraction minmax tracks are in scope");

    assert_eq!(
        projected.columns,
        vec![
            GridTrackSize::MinMax {
                min: GridTrackMinSizingFunction::LengthPx(20.0),
                max: GridTrackMaxSizingFunction::LengthPx(80.0),
            },
            GridTrackSize::MinMax {
                min: GridTrackMinSizingFunction::Percentage(0.25),
                max: GridTrackMaxSizingFunction::Fraction(1.0),
            },
        ]
    );
    assert_eq!(
        projected.rows,
        vec![GridTrackSize::MinMax {
            min: GridTrackMinSizingFunction::LengthPx(10.0),
            max: GridTrackMaxSizingFunction::Percentage(0.5),
        }]
    );
}

#[test]
fn root_html_profile_rejects_invalid_minmax_breadths() {
    let node = NodeId::new(11_251).unwrap();
    for track in [
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::LengthPx(-1.0),
            max: ComputedCssGridTrackBreadth::LengthPx(10.0),
        },
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::LengthPx(1.0),
            max: ComputedCssGridTrackBreadth::Percentage(f32::INFINITY),
        },
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::LengthPx(1.0),
            max: ComputedCssGridTrackBreadth::Fraction(-1.0),
        },
    ] {
        let input = grid(vec![track], vec![ComputedCssGridTrackSize::LengthPx(40.0)]);
        let error = project_grid_layout(
            node,
            &input,
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
        )
        .expect_err("unsupported or invalid minmax breadths must fail closed");
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
