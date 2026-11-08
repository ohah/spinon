use super::{grid, project_grid_layout};
use spinon_core::NodeId;
use spinon_layout::{GridTrackLengthPercentage, GridTrackSize};
use spinon_style::{
    ComputedCssGridAutoRows, ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize,
    ComputedStyleProfile,
};

const ORDERS: [[usize; 4]; 24] = [
    [0, 1, 2, 3],
    [0, 1, 3, 2],
    [0, 2, 1, 3],
    [0, 2, 3, 1],
    [0, 3, 1, 2],
    [0, 3, 2, 1],
    [1, 0, 2, 3],
    [1, 0, 3, 2],
    [1, 2, 0, 3],
    [1, 2, 3, 0],
    [1, 3, 0, 2],
    [1, 3, 2, 0],
    [2, 0, 1, 3],
    [2, 0, 3, 1],
    [2, 1, 0, 3],
    [2, 1, 3, 0],
    [2, 3, 0, 1],
    [2, 3, 1, 0],
    [3, 0, 1, 2],
    [3, 0, 2, 1],
    [3, 1, 0, 2],
    [3, 1, 2, 0],
    [3, 2, 0, 1],
    [3, 2, 1, 0],
];

fn typed_members(cap: ComputedCssGridTrackLengthPercentage) -> [ComputedCssGridTrackSize; 4] {
    [
        ComputedCssGridTrackSize::FitContent(cap),
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::MaxContent,
        ComputedCssGridTrackSize::Auto,
    ]
}

fn layout_members(cap: GridTrackLengthPercentage) -> [GridTrackSize; 4] {
    [
        GridTrackSize::FitContent(cap),
        GridTrackSize::MinContent,
        GridTrackSize::MaxContent,
        GridTrackSize::Auto,
    ]
}

fn input_with_rows(rows: Vec<ComputedCssGridTrackSize>) -> spinon_style::ComputedCssGridLayout {
    let mut input = grid(
        vec![ComputedCssGridTrackSize::LengthPx(40.0)],
        vec![ComputedCssGridTrackSize::LengthPx(10.0)],
    );
    input.auto_rows = ComputedCssGridAutoRows::TrackList(rows);
    input
}

#[test]
fn root_html_fit_content_min_max_content_auto_lists_preserve_all_24_orders_and_typed_caps() {
    let node = NodeId::new(11_292).unwrap();
    for (computed_cap, layout_cap) in [
        (
            ComputedCssGridTrackLengthPercentage::LengthPx(20.0),
            GridTrackLengthPercentage::LengthPx(20.0),
        ),
        (
            ComputedCssGridTrackLengthPercentage::Percentage(0.2),
            GridTrackLengthPercentage::Percentage(0.2),
        ),
        (
            ComputedCssGridTrackLengthPercentage::Percentage(0.0),
            GridTrackLengthPercentage::Percentage(0.0),
        ),
        (
            ComputedCssGridTrackLengthPercentage::Percentage(1.2),
            GridTrackLengthPercentage::Percentage(1.2),
        ),
    ] {
        let computed_members = typed_members(computed_cap.clone());
        let layout_members = layout_members(layout_cap);
        for order in ORDERS {
            let computed = order
                .map(|member| computed_members[member].clone())
                .to_vec();
            let expected = order.map(|member| layout_members[member]).to_vec();
            let projected = project_grid_layout(
                node,
                &input_with_rows(computed),
                ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
            )
            .unwrap_or_else(|error| panic!("order={order:?} cap={computed_cap:?}: {error:?}"));
            assert_eq!(
                projected.placement_mode,
                spinon_layout::GridPlacementMode::RootHtmlFitContentAutoRowsV1,
                "order={order:?}"
            );
            assert_eq!(
                projected.auto_rows,
                spinon_layout::GridAutoRows::TrackList(expected),
                "typed track order must be preserved for {order:?}"
            );
        }
    }
}

#[test]
fn root_html_fit_content_min_max_content_auto_lists_reject_nearby_grammars() {
    let node = NodeId::new(11_293).unwrap();
    let fit = |cap| {
        ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(cap))
    };
    for rows in [
        vec![
            fit(20.0),
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::MaxContent,
        ],
        vec![
            fit(20.0),
            ComputedCssGridTrackSize::MaxContent,
            ComputedCssGridTrackSize::Auto,
        ],
        vec![
            fit(20.0),
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::Auto,
        ],
        vec![
            fit(20.0),
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::MaxContent,
            ComputedCssGridTrackSize::Auto,
            ComputedCssGridTrackSize::LengthPx(8.0),
        ],
        vec![
            fit(-1.0),
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::MaxContent,
            ComputedCssGridTrackSize::Auto,
        ],
        vec![
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::Percentage(
                f32::NAN,
            )),
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::MaxContent,
            ComputedCssGridTrackSize::Auto,
        ],
        vec![
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
                f32::INFINITY,
            )),
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::MaxContent,
            ComputedCssGridTrackSize::Auto,
        ],
        vec![
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::Percentage(
                f32::INFINITY,
            )),
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::MaxContent,
            ComputedCssGridTrackSize::Auto,
        ],
    ] {
        assert!(matches!(
            project_grid_layout(
                node,
                &input_with_rows(rows),
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
