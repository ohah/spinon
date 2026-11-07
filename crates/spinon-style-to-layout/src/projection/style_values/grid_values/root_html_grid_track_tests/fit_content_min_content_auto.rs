use super::{grid, project_grid_layout};
use spinon_core::NodeId;
use spinon_layout::{GridTrackLengthPercentage, GridTrackSize};
use spinon_style::{
    ComputedCssGridAutoRows, ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize,
    ComputedStyleProfile,
};

fn fit_content_percentage(value: f32) -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::Percentage(value))
}

fn fit_content_px(value: f32) -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(value))
}

fn implicit_rows_grid(rows: Vec<ComputedCssGridTrackSize>) -> spinon_style::ComputedCssGridLayout {
    let mut input = grid(
        vec![ComputedCssGridTrackSize::LengthPx(40.0)],
        vec![ComputedCssGridTrackSize::LengthPx(10.0)],
    );
    input.auto_rows = ComputedCssGridAutoRows::TrackList(rows);
    input
}

#[test]
fn root_html_fit_content_min_content_auto_lists_preserve_all_six_exact_orders() {
    let node = NodeId::new(11_290).unwrap();
    for (rows, expected) in [
        (
            vec![
                fit_content_percentage(0.2),
                ComputedCssGridTrackSize::MinContent,
                ComputedCssGridTrackSize::Auto,
            ],
            vec![
                GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.2)),
                GridTrackSize::MinContent,
                GridTrackSize::Auto,
            ],
        ),
        (
            vec![
                fit_content_px(20.0),
                ComputedCssGridTrackSize::MinContent,
                ComputedCssGridTrackSize::Auto,
            ],
            vec![
                GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(20.0)),
                GridTrackSize::MinContent,
                GridTrackSize::Auto,
            ],
        ),
        (
            vec![
                fit_content_percentage(0.2),
                ComputedCssGridTrackSize::Auto,
                ComputedCssGridTrackSize::MinContent,
            ],
            vec![
                GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.2)),
                GridTrackSize::Auto,
                GridTrackSize::MinContent,
            ],
        ),
        (
            vec![
                ComputedCssGridTrackSize::MinContent,
                fit_content_percentage(0.2),
                ComputedCssGridTrackSize::Auto,
            ],
            vec![
                GridTrackSize::MinContent,
                GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.2)),
                GridTrackSize::Auto,
            ],
        ),
        (
            vec![
                ComputedCssGridTrackSize::MinContent,
                fit_content_px(20.0),
                ComputedCssGridTrackSize::Auto,
            ],
            vec![
                GridTrackSize::MinContent,
                GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(20.0)),
                GridTrackSize::Auto,
            ],
        ),
        (
            vec![
                ComputedCssGridTrackSize::MinContent,
                ComputedCssGridTrackSize::Auto,
                fit_content_percentage(0.2),
            ],
            vec![
                GridTrackSize::MinContent,
                GridTrackSize::Auto,
                GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.2)),
            ],
        ),
        (
            vec![
                ComputedCssGridTrackSize::MinContent,
                ComputedCssGridTrackSize::Auto,
                fit_content_px(20.0),
            ],
            vec![
                GridTrackSize::MinContent,
                GridTrackSize::Auto,
                GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(20.0)),
            ],
        ),
        (
            vec![
                ComputedCssGridTrackSize::Auto,
                fit_content_percentage(0.2),
                ComputedCssGridTrackSize::MinContent,
            ],
            vec![
                GridTrackSize::Auto,
                GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.2)),
                GridTrackSize::MinContent,
            ],
        ),
        (
            vec![
                ComputedCssGridTrackSize::Auto,
                fit_content_px(20.0),
                ComputedCssGridTrackSize::MinContent,
            ],
            vec![
                GridTrackSize::Auto,
                GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(20.0)),
                GridTrackSize::MinContent,
            ],
        ),
        (
            vec![
                ComputedCssGridTrackSize::Auto,
                ComputedCssGridTrackSize::MinContent,
                fit_content_percentage(0.2),
            ],
            vec![
                GridTrackSize::Auto,
                GridTrackSize::MinContent,
                GridTrackSize::FitContent(GridTrackLengthPercentage::Percentage(0.2)),
            ],
        ),
        (
            vec![
                ComputedCssGridTrackSize::Auto,
                ComputedCssGridTrackSize::MinContent,
                fit_content_px(20.0),
            ],
            vec![
                GridTrackSize::Auto,
                GridTrackSize::MinContent,
                GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(20.0)),
            ],
        ),
    ] {
        let projected = project_grid_layout(
            node,
            &implicit_rows_grid(rows),
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
        )
        .expect("all six exact min-content/auto orders should project");

        assert_eq!(
            projected.placement_mode,
            spinon_layout::GridPlacementMode::RootHtmlFitContentAutoRowsV1
        );
        assert_eq!(
            projected.auto_rows,
            spinon_layout::GridAutoRows::TrackList(expected)
        );
    }
}

#[test]
fn root_html_fit_content_min_content_auto_lists_reject_nearby_patterns_and_invalid_caps() {
    let node = NodeId::new(11_291).unwrap();
    let invalid_lists = [
        vec![
            ComputedCssGridTrackSize::MinContent,
            fit_content_percentage(0.2),
            ComputedCssGridTrackSize::MaxContent,
        ],
        vec![
            fit_content_percentage(0.2),
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::MinContent,
        ],
        vec![
            fit_content_percentage(0.2),
            ComputedCssGridTrackSize::Auto,
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::LengthPx(8.0),
        ],
        vec![
            fit_content_percentage(0.2),
            ComputedCssGridTrackSize::MaxContent,
            ComputedCssGridTrackSize::Auto,
        ],
        vec![
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::Auto,
        ],
    ];

    for rows in invalid_lists {
        assert!(matches!(
            project_grid_layout(
                node,
                &implicit_rows_grid(rows),
                ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
            ),
            Err(crate::StyleLayoutError::UnsupportedComputedValue {
                node: error_node,
                property: "grid-auto-rows",
                ..
            }) if error_node == node
        ));
    }

    for cap in [-0.01, f32::NAN, f32::INFINITY] {
        assert!(matches!(
            project_grid_layout(
                node,
                &implicit_rows_grid(vec![
                    fit_content_percentage(cap),
                    ComputedCssGridTrackSize::MinContent,
                    ComputedCssGridTrackSize::Auto,
                ]),
                ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
            ),
            Err(crate::StyleLayoutError::UnsupportedComputedValue {
                node: error_node,
                property: "grid-auto-rows",
                ..
            }) if error_node == node
        ));
    }
    for cap in [-0.01, f32::NAN, f32::INFINITY] {
        assert!(matches!(
            project_grid_layout(
                node,
                &implicit_rows_grid(vec![
                    ComputedCssGridTrackSize::MinContent,
                    ComputedCssGridTrackSize::Auto,
                    fit_content_percentage(cap),
                ]),
                ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
            ),
            Err(crate::StyleLayoutError::UnsupportedComputedValue {
                node: error_node,
                property: "grid-auto-rows",
                ..
            }) if error_node == node
        ));
    }
    for cap in [-0.01, f32::NAN, f32::INFINITY] {
        assert!(matches!(
            project_grid_layout(
                node,
                &implicit_rows_grid(vec![
                    fit_content_px(cap),
                    ComputedCssGridTrackSize::MinContent,
                    ComputedCssGridTrackSize::Auto,
                ]),
                ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
            ),
            Err(crate::StyleLayoutError::UnsupportedComputedValue {
                node: error_node,
                property: "grid-auto-rows",
                ..
            }) if error_node == node
        ));
    }
    for cap in [-0.01, f32::NAN, f32::INFINITY] {
        assert!(matches!(
            project_grid_layout(
                node,
                &implicit_rows_grid(vec![
                    ComputedCssGridTrackSize::Auto,
                    ComputedCssGridTrackSize::MinContent,
                    fit_content_px(cap),
                ]),
                ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
            ),
            Err(crate::StyleLayoutError::UnsupportedComputedValue {
                node: error_node,
                property: "grid-auto-rows",
                ..
            }) if error_node == node
        ));
    }
}
