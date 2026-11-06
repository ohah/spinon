use super::root_html_auto_repeat_cell_capacity;
use crate::{
    GridAutoRepeat, GridAutoRepeatMode, GridContainerInput, GridTrackMaxSizingFunction,
    GridTrackMinSizingFunction, GridTrackSize, LayoutDimension, LayoutGap, LayoutLengthPercentage,
    LayoutStyle,
};
use spinon_core::NodeId;

fn repeated_track(minimum: GridTrackMinSizingFunction) -> GridTrackSize {
    GridTrackSize::MinMax {
        min: minimum,
        max: GridTrackMaxSizingFunction::Fraction(1.0),
    }
}

fn input_for_columns(minima: &[f32], width: f32, gap: f32) -> (LayoutStyle, GridContainerInput) {
    let minima = minima
        .iter()
        .copied()
        .map(GridTrackMinSizingFunction::LengthPx)
        .collect::<Vec<_>>();
    input_for_columns_with_minima(&minima, width, gap)
}

fn input_for_columns_with_minima(
    minima: &[GridTrackMinSizingFunction],
    width: f32,
    gap: f32,
) -> (LayoutStyle, GridContainerInput) {
    let columns = minima
        .iter()
        .copied()
        .map(repeated_track)
        .collect::<Vec<_>>();
    let container = GridContainerInput {
        columns_auto_repeat: Some(GridAutoRepeat {
            mode: GridAutoRepeatMode::AutoFill,
            start: 0,
            end: columns.len(),
            line_names: Vec::new(),
        }),
        columns,
        rows: vec![GridTrackSize::LengthPx(24.0)],
        ..GridContainerInput::default()
    };
    let style = LayoutStyle {
        width: LayoutDimension::Fixed(width),
        height: LayoutDimension::Fixed(24.0),
        gap: LayoutGap {
            column: LayoutLengthPercentage::LengthPx(gap),
            ..LayoutGap::default()
        },
        ..LayoutStyle::default()
    };
    (style, container)
}

fn input_for_rows(minima: &[f32], height: f32, gap: f32) -> (LayoutStyle, GridContainerInput) {
    let minima = minima
        .iter()
        .copied()
        .map(GridTrackMinSizingFunction::LengthPx)
        .collect::<Vec<_>>();
    input_for_rows_with_minima(&minima, height, gap)
}

fn input_for_rows_with_minima(
    minima: &[GridTrackMinSizingFunction],
    height: f32,
    gap: f32,
) -> (LayoutStyle, GridContainerInput) {
    let rows = minima
        .iter()
        .copied()
        .map(repeated_track)
        .collect::<Vec<_>>();
    let container = GridContainerInput {
        rows_auto_repeat: Some(GridAutoRepeat {
            mode: GridAutoRepeatMode::AutoFill,
            start: 0,
            end: rows.len(),
            line_names: Vec::new(),
        }),
        columns: vec![GridTrackSize::LengthPx(24.0)],
        rows,
        ..GridContainerInput::default()
    };
    let style = LayoutStyle {
        width: LayoutDimension::Fixed(24.0),
        height: LayoutDimension::Fixed(height),
        gap: LayoutGap {
            row: LayoutLengthPercentage::LengthPx(gap),
            ..LayoutGap::default()
        },
        ..LayoutStyle::default()
    };
    (style, container)
}

#[test]
fn zero_minimum_capacity_matches_columns_and_repeated_pattern_counts() {
    for (minima, width, gap, expected) in [
        (&[0.0][..], 11.0, 1.0, 6),
        (&[0.0, 0.0][..], 19.0, 1.0, 10),
        (&[0.0, 2.0][..], 19.0, 1.0, 8),
    ] {
        let (style, container) = input_for_columns(minima, width, gap);
        assert_eq!(
            root_html_auto_repeat_cell_capacity(NodeId::new(31_201).unwrap(), style, &container),
            Ok(expected)
        );
    }
}

#[test]
fn zero_minimum_capacity_uses_the_row_axis_size_and_gap() {
    let (style, container) = input_for_rows(&[0.0], 7.0, 1.0);
    assert_eq!(
        root_html_auto_repeat_cell_capacity(NodeId::new(31_205).unwrap(), style, &container),
        Ok(4)
    );
}

#[test]
fn percentage_capacity_resolves_against_the_repeated_axis_and_keeps_pattern_gaps() {
    for (minima, width, gap, expected) in [
        (
            &[GridTrackMinSizingFunction::Percentage(0.1)][..],
            200.0,
            5.0,
            8,
        ),
        (
            &[
                GridTrackMinSizingFunction::Percentage(0.1),
                GridTrackMinSizingFunction::Percentage(0.05),
            ][..],
            200.0,
            2.0,
            10,
        ),
        (
            &[GridTrackMinSizingFunction::Percentage(0.0)][..],
            200.0,
            5.0,
            34,
        ),
        (
            &[GridTrackMinSizingFunction::Percentage(1.2)][..],
            200.0,
            0.0,
            1,
        ),
    ] {
        let (style, container) = input_for_columns_with_minima(minima, width, gap);
        assert_eq!(
            root_html_auto_repeat_cell_capacity(NodeId::new(31_207).unwrap(), style, &container),
            Ok(expected)
        );
    }

    let (style, container) =
        input_for_rows_with_minima(&[GridTrackMinSizingFunction::Percentage(0.2)], 160.0, 4.0);
    assert_eq!(
        root_html_auto_repeat_cell_capacity(NodeId::new(31_208).unwrap(), style, &container),
        Ok(4)
    );
}

#[test]
fn percentage_capacity_rejects_invalid_or_unrepresentable_minimums() {
    for minimum in [f32::NAN, -0.01, f32::MAX] {
        let (style, container) = input_for_columns_with_minima(
            &[GridTrackMinSizingFunction::Percentage(minimum)],
            200.0,
            0.0,
        );
        assert!(matches!(
            root_html_auto_repeat_cell_capacity(NodeId::new(31_209).unwrap(), style, &container),
            Err(crate::LayoutError::InvalidGridTrackList {
                property: "grid-template-columns",
                ..
            })
        ));
    }
}

#[test]
fn capacity_accepts_exact_track_ceiling_and_rejects_overflow_without_clamping() {
    let (style, container) = input_for_columns(&[0.0], 10_000.0, 0.0);
    assert_eq!(
        root_html_auto_repeat_cell_capacity(NodeId::new(31_202).unwrap(), style, &container),
        Ok(crate::GRID_MAX_TRACKS)
    );

    let (style, container) = input_for_columns(&[0.0, 0.0], 10_000.0, 0.0);
    assert_eq!(
        root_html_auto_repeat_cell_capacity(NodeId::new(31_206).unwrap(), style, &container),
        Ok(crate::GRID_MAX_TRACKS)
    );

    let (style, container) = input_for_columns(&[0.0], 10_001.0, 0.0);
    assert!(matches!(
        root_html_auto_repeat_cell_capacity(NodeId::new(31_203).unwrap(), style, &container),
        Err(crate::LayoutError::GridTrackExpansionOverflow(node))
            if node == NodeId::new(31_203).unwrap()
    ));

    let (style, container) = input_for_columns(&[0.0, 0.0], 10_002.0, 0.0);
    assert!(matches!(
        root_html_auto_repeat_cell_capacity(NodeId::new(31_204).unwrap(), style, &container),
        Err(crate::LayoutError::GridTrackExpansionOverflow(node))
            if node == NodeId::new(31_204).unwrap()
    ));
}
