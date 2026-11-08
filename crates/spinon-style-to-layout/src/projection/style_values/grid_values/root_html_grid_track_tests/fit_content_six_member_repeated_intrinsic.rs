use super::{grid, project_grid_layout};
use spinon_core::NodeId;
use spinon_layout::{GridAutoRows, GridPlacementMode, GridTrackLengthPercentage, GridTrackSize};
use spinon_style::{
    ComputedCssGridAutoRows, ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize,
    ComputedStyleProfile,
};

fn computed_tracks(cap: ComputedCssGridTrackLengthPercentage) -> Vec<ComputedCssGridTrackSize> {
    vec![
        ComputedCssGridTrackSize::FitContent(cap),
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::MaxContent,
        ComputedCssGridTrackSize::Auto,
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::MaxContent,
    ]
}

fn layout_tracks(cap: GridTrackLengthPercentage) -> Vec<GridTrackSize> {
    vec![
        GridTrackSize::FitContent(cap),
        GridTrackSize::MinContent,
        GridTrackSize::MaxContent,
        GridTrackSize::Auto,
        GridTrackSize::MinContent,
        GridTrackSize::MaxContent,
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
fn root_html_repeated_intrinsic_auto_row_list_preserves_member_identity_order_and_caps() {
    let node = NodeId::new(11_294).unwrap();
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
        let projected = project_grid_layout(
            node,
            &input_with_rows(computed_tracks(computed_cap.clone())),
            ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
        )
        .unwrap_or_else(|error| panic!("cap={computed_cap:?}: {error:?}"));
        assert_eq!(
            projected.placement_mode,
            GridPlacementMode::RootHtmlFitContentAutoRowsV1
        );
        assert_eq!(
            projected.auto_rows,
            GridAutoRows::TrackList(layout_tracks(layout_cap)),
            "each repeated min/max member must remain ordered and typed"
        );
    }
}

#[test]
fn root_html_repeated_intrinsic_auto_row_list_rejects_nearby_orders_and_caps() {
    let node = NodeId::new(11_295).unwrap();
    let fit = |cap| {
        ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(cap))
    };
    let valid_tail = [
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::MaxContent,
        ComputedCssGridTrackSize::Auto,
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::MaxContent,
    ];
    let mut wrong_order = vec![fit(20.0)];
    wrong_order.extend([
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::MaxContent,
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::Auto,
        ComputedCssGridTrackSize::MaxContent,
    ]);
    let mut wrong_duplicate = vec![fit(20.0)];
    wrong_duplicate.extend([
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::MaxContent,
        ComputedCssGridTrackSize::Auto,
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::MinContent,
    ]);
    let mut short_list = vec![fit(20.0)];
    short_list.extend(valid_tail[..4].iter().cloned());
    let mut long_list = vec![fit(20.0)];
    long_list.extend(valid_tail.iter().cloned());
    long_list.push(ComputedCssGridTrackSize::Auto);

    for rows in [
        wrong_order,
        wrong_duplicate,
        short_list,
        long_list,
        vec![
            fit(-1.0),
            valid_tail[0].clone(),
            valid_tail[1].clone(),
            valid_tail[2].clone(),
            valid_tail[3].clone(),
            valid_tail[4].clone(),
        ],
        vec![
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::Percentage(
                f32::NAN,
            )),
            valid_tail[0].clone(),
            valid_tail[1].clone(),
            valid_tail[2].clone(),
            valid_tail[3].clone(),
            valid_tail[4].clone(),
        ],
        vec![
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
                f32::INFINITY,
            )),
            valid_tail[0].clone(),
            valid_tail[1].clone(),
            valid_tail[2].clone(),
            valid_tail[3].clone(),
            valid_tail[4].clone(),
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
