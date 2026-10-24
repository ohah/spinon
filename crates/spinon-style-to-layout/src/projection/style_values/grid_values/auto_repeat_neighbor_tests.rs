use super::project_grid_layout;
use crate::StyleLayoutError;
use spinon_core::NodeId;
use spinon_layout::GridTrackSize;
use spinon_style::{
    ComputedCssGridAutoColumns, ComputedCssGridAutoFlow, ComputedCssGridAutoRepeat,
    ComputedCssGridAutoRepeatMode, ComputedCssGridAutoRows, ComputedCssGridLayout,
    ComputedCssGridTrackBreadth, ComputedCssGridTrackSize, ComputedCssMath, ComputedStyleProfile,
};

fn repeated_track() -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::MinMax {
        min: ComputedCssGridTrackBreadth::LengthPx(40.0),
        max: ComputedCssGridTrackBreadth::Fraction(1.0),
    }
}

fn grid_with_column_neighbors(
    prefix: ComputedCssGridTrackSize,
    suffix: ComputedCssGridTrackSize,
) -> ComputedCssGridLayout {
    ComputedCssGridLayout {
        columns: vec![prefix, repeated_track(), suffix],
        rows: vec![ComputedCssGridTrackSize::LengthPx(12.0)],
        columns_auto_repeat: Some(ComputedCssGridAutoRepeat {
            mode: ComputedCssGridAutoRepeatMode::AutoFill,
            start: 1,
            end: 2,
            line_names: Vec::new(),
        }),
        rows_auto_repeat: None,
        columns_line_names: Vec::new(),
        rows_line_names: vec![Vec::new(), Vec::new()],
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: ComputedCssGridAutoColumns::Auto,
    }
}

fn grid_with_row_neighbor(neighbor: ComputedCssGridTrackSize) -> ComputedCssGridLayout {
    ComputedCssGridLayout {
        columns: vec![ComputedCssGridTrackSize::LengthPx(12.0)],
        rows: vec![neighbor, repeated_track()],
        columns_auto_repeat: None,
        rows_auto_repeat: Some(ComputedCssGridAutoRepeat {
            mode: ComputedCssGridAutoRepeatMode::AutoFit,
            start: 1,
            end: 2,
            line_names: Vec::new(),
        }),
        columns_line_names: vec![Vec::new(), Vec::new()],
        rows_line_names: Vec::new(),
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: ComputedCssGridAutoColumns::Auto,
    }
}

fn calc_neighbor(length: ComputedCssMath, percentage: f32) -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::Calc(ComputedCssMath::Sum(vec![
        length,
        ComputedCssMath::Percentage(percentage),
    ]))
}

#[test]
fn track_sizing_projection_accepts_the_supported_auto_repeat_neighbor_shapes() {
    let node = NodeId::new(11_131).unwrap();
    let supported = [
        ComputedCssGridTrackSize::LengthPx(8.0),
        ComputedCssGridTrackSize::Percentage(0.05),
        calc_neighbor(ComputedCssMath::LengthPx(10.0), 0.05),
        ComputedCssGridTrackSize::Calc(ComputedCssMath::Sum(vec![
            ComputedCssMath::Percentage(0.05),
            ComputedCssMath::LengthPx(10.0),
        ])),
        calc_neighbor(
            ComputedCssMath::Negate(Box::new(ComputedCssMath::LengthPx(2.0))),
            0.05,
        ),
    ];
    for neighbor in supported {
        let grid = grid_with_column_neighbors(neighbor.clone(), neighbor);
        let projected = project_grid_layout(
            node,
            &grid,
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
        )
        .unwrap_or_else(|error| panic!("supported auto-repeat neighbor: {error:?}"));
        assert_eq!(projected.columns.len(), 3);
    }
}

#[test]
fn track_sizing_projection_preserves_css_property_context_for_unsupported_neighbors() {
    let node = NodeId::new(11_132).unwrap();
    let unsupported = [
        calc_neighbor(ComputedCssMath::LengthPx(10.0), -0.05),
        ComputedCssGridTrackSize::Calc(ComputedCssMath::Sum(vec![
            ComputedCssMath::LengthPx(10.0),
            ComputedCssMath::Percentage(0.05),
            ComputedCssMath::Clamp {
                min: Box::new(ComputedCssMath::LengthPx(0.0)),
                value: Box::new(ComputedCssMath::Percentage(0.0)),
                max: Box::new(ComputedCssMath::LengthPx(1.0)),
            },
        ])),
        ComputedCssGridTrackSize::MaxContent,
        ComputedCssGridTrackSize::Fraction(1.0),
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::LengthPx(10.0),
            max: ComputedCssGridTrackBreadth::LengthPx(20.0),
        },
    ];
    for neighbor in unsupported {
        let grid = grid_with_column_neighbors(neighbor, ComputedCssGridTrackSize::LengthPx(4.0));
        let error = project_grid_layout(
            node,
            &grid,
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
        )
        .unwrap_err();
        assert!(
            matches!(
                error,
                StyleLayoutError::UnsupportedComputedValue {
                    node: error_node,
                    property: "grid-template-columns",
                    ..
                } if error_node == node
            ),
            "unsupported column neighbor must retain node/property: {error:?}"
        );
    }

    let grid = grid_with_row_neighbor(calc_neighbor(ComputedCssMath::LengthPx(10.0), -0.05));
    assert!(matches!(
        project_grid_layout(
            node,
            &grid,
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
        ),
        Err(StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-template-rows",
            ..
        }) if error_node == node
    ));
}

#[test]
fn track_sizing_projection_rejects_invalid_auto_repeat_ranges_before_slicing() {
    let node = NodeId::new(11_133).unwrap();
    let mut grid = grid_with_column_neighbors(
        ComputedCssGridTrackSize::LengthPx(4.0),
        ComputedCssGridTrackSize::LengthPx(4.0),
    );
    grid.columns_auto_repeat.as_mut().unwrap().end = usize::MAX;

    assert!(matches!(
        project_grid_layout(
            node,
            &grid,
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
        ),
        Err(StyleLayoutError::UnsupportedComputedValue {
            node: error_node,
            property: "grid-template-columns",
            ..
        }) if error_node == node
    ));
}

#[test]
fn supported_auto_repeat_neighbor_projection_keeps_calc_ids_on_their_css_axis() {
    let node = NodeId::new(11_134).unwrap();
    let neighbor = calc_neighbor(ComputedCssMath::LengthPx(10.0), 0.05);
    let grid = grid_with_column_neighbors(neighbor, ComputedCssGridTrackSize::LengthPx(4.0));
    let projected = project_grid_layout(
        node,
        &grid,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
    )
    .unwrap();
    assert!(matches!(&projected.columns[0], GridTrackSize::Calc(_)));
}
