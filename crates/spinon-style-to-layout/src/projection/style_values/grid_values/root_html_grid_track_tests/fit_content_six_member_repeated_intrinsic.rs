use super::{grid, project_grid_layout};
use spinon_core::NodeId;
use spinon_layout::{GridAutoRows, GridPlacementMode, GridTrackLengthPercentage, GridTrackSize};
use spinon_style::{
    ComputedCssGridAutoRows, ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize,
    ComputedStyleProfile,
};

fn computed_tracks(
    cap: ComputedCssGridTrackLengthPercentage,
    tail: &[ComputedCssGridTrackSize],
) -> Vec<ComputedCssGridTrackSize> {
    let mut tracks = vec![ComputedCssGridTrackSize::FitContent(cap)];
    tracks.extend(tail.iter().cloned());
    tracks
}

fn layout_tracks(
    cap: GridTrackLengthPercentage,
    tail: &[ComputedCssGridTrackSize],
) -> Vec<GridTrackSize> {
    let mut tracks = vec![GridTrackSize::FitContent(cap)];
    tracks.extend(tail.iter().map(|track| match track {
        ComputedCssGridTrackSize::MinContent => GridTrackSize::MinContent,
        ComputedCssGridTrackSize::MaxContent => GridTrackSize::MaxContent,
        ComputedCssGridTrackSize::Auto => GridTrackSize::Auto,
        _ => panic!("repeated intrinsic tail contains only min/max/auto tracks"),
    }));
    tracks
}

fn input_with_rows(rows: Vec<ComputedCssGridTrackSize>) -> spinon_style::ComputedCssGridLayout {
    let mut input = grid(
        vec![ComputedCssGridTrackSize::LengthPx(40.0)],
        vec![ComputedCssGridTrackSize::LengthPx(10.0)],
    );
    input.auto_rows = ComputedCssGridAutoRows::TrackList(rows);
    input
}

fn repeated_tail_orders() -> Vec<Vec<ComputedCssGridTrackSize>> {
    fn visit(
        remaining: &mut [usize; 3],
        current: &mut Vec<ComputedCssGridTrackSize>,
        orders: &mut Vec<Vec<ComputedCssGridTrackSize>>,
    ) {
        if current.len() == 5 {
            orders.push(current.clone());
            return;
        }

        for kind in 0..remaining.len() {
            if remaining[kind] == 0 {
                continue;
            }
            remaining[kind] -= 1;
            current.push(match kind {
                0 => ComputedCssGridTrackSize::MinContent,
                1 => ComputedCssGridTrackSize::MaxContent,
                _ => ComputedCssGridTrackSize::Auto,
            });
            visit(remaining, current, orders);
            current.pop();
            remaining[kind] += 1;
        }
    }

    let mut orders = Vec::new();
    visit(&mut [2, 2, 1], &mut Vec::new(), &mut orders);
    orders
}

#[test]
fn root_html_repeated_intrinsic_auto_row_list_preserves_all_30_orders_and_caps() {
    let node = NodeId::new(11_294).unwrap();
    let orders = repeated_tail_orders();
    assert_eq!(orders.len(), 30);
    assert_eq!(
        orders
            .iter()
            .map(|tail| format!("{tail:?}"))
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        30,
        "the repeated-tail generator must produce 30 unique orders"
    );
    assert!(orders.contains(&vec![
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::MaxContent,
        ComputedCssGridTrackSize::Auto,
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::MaxContent,
    ]));

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
        for tail in &orders {
            let projected = project_grid_layout(
                node,
                &input_with_rows(computed_tracks(computed_cap.clone(), tail)),
                ComputedStyleProfile::RuntimeRootHtmlTextPaintV1,
            )
            .unwrap_or_else(|error| panic!("tail={tail:?} cap={computed_cap:?}: {error:?}"));
            assert_eq!(
                projected.placement_mode,
                GridPlacementMode::RootHtmlFitContentAutoRowsV1
            );
            assert_eq!(
                projected.auto_rows,
                GridAutoRows::TrackList(layout_tracks(layout_cap, tail)),
                "the repeated min/max/auto order must remain typed and unchanged"
            );
        }
    }
}

#[test]
fn root_html_repeated_intrinsic_auto_row_list_rejects_nearby_multiplicities_positions_and_caps() {
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
    let fit_not_first = vec![
        ComputedCssGridTrackSize::MinContent,
        fit(20.0),
        ComputedCssGridTrackSize::MaxContent,
        ComputedCssGridTrackSize::Auto,
        ComputedCssGridTrackSize::MinContent,
        ComputedCssGridTrackSize::MaxContent,
    ];

    for rows in [
        wrong_duplicate,
        short_list,
        long_list,
        fit_not_first,
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
