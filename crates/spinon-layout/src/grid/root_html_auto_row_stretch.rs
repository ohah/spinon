use super::{GridAutoColumns, GridAutoFlow, GridAutoRows, GridContainerInput, GridTrackSize};
use crate::{LayoutDimension, LayoutLengthPercentage, LayoutStyle};

const GRID_BLOCK_SIZE_PX: f64 = 40.0;
const BASE_ROW_EXTENT_PX: f64 = 30.0;
const IMPLICIT_ROW_GUTTER_COUNT: f64 = 2.0;

pub(super) fn layout_profile_is_supported(
    style: LayoutStyle,
    container: &GridContainerInput,
    visible_item_count: usize,
    all_visible_item_placements_auto: bool,
) -> bool {
    container.placement_mode == super::GridPlacementMode::RootHtmlAutoRowStretchV1
        && matches!(style.width, LayoutDimension::Fixed(value) if value == 40.0)
        && matches!(style.height, LayoutDimension::Fixed(value) if value == 40.0)
        && matches!(
            container.columns.as_slice(),
            [GridTrackSize::LengthPx(value)] if *value == 40.0
        )
        && matches!(
            container.rows.as_slice(),
            [GridTrackSize::LengthPx(value)] if *value == 10.0
        )
        && container.auto_rows == GridAutoRows::Auto
        && container.auto_columns == GridAutoColumns::Auto
        && container.auto_flow == GridAutoFlow::Row
        && container.columns_auto_repeat.is_none()
        && container.rows_auto_repeat.is_none()
        && container.template_areas.is_none()
        && matches!(visible_item_count, 2 | 3)
        && all_visible_item_placements_auto
}

pub(super) fn row_gap_is_supported(gap: LayoutLengthPercentage) -> bool {
    match gap {
        LayoutLengthPercentage::LengthPx(value) => {
            value.is_finite() && (0.0..=7.5).contains(&value)
        }
        LayoutLengthPercentage::Percentage(fraction) if fraction.is_finite() && fraction >= 0.0 => {
            let used_gap = f64::from(fraction) * GRID_BLOCK_SIZE_PX;
            let grid_extent = used_gap * IMPLICIT_ROW_GUTTER_COUNT + BASE_ROW_EXTENT_PX;
            grid_extent.is_finite() && grid_extent <= f64::from(f32::MAX)
        }
        LayoutLengthPercentage::Percentage(_) | LayoutLengthPercentage::Calc(_) => false,
        LayoutLengthPercentage::Auto => false,
    }
}

#[cfg(test)]
mod tests {
    use super::layout_profile_is_supported;
    use super::row_gap_is_supported;
    use crate::{GridPlacementMode, GridTrackSize, LayoutLengthPercentage, LayoutStyle};

    fn supported_profile() -> (LayoutStyle, super::GridContainerInput) {
        let style = LayoutStyle {
            width: crate::LayoutDimension::Fixed(40.0),
            height: crate::LayoutDimension::Fixed(40.0),
            ..LayoutStyle::default()
        };
        let container = super::GridContainerInput {
            columns: vec![GridTrackSize::LengthPx(40.0)],
            rows: vec![GridTrackSize::LengthPx(10.0)],
            placement_mode: GridPlacementMode::RootHtmlAutoRowStretchV1,
            auto_rows: super::GridAutoRows::Auto,
            auto_columns: super::GridAutoColumns::Auto,
            ..super::GridContainerInput::default()
        };
        (style, container)
    }

    #[test]
    fn layout_profile_requires_the_definite_gap_basis_and_bounded_items() {
        let (style, container) = supported_profile();
        assert!(layout_profile_is_supported(style, &container, 2, true));
        assert!(layout_profile_is_supported(style, &container, 3, true));
        assert!(!layout_profile_is_supported(style, &container, 4, true));
        assert!(!layout_profile_is_supported(style, &container, 3, false));

        let mut invalid_style = style;
        invalid_style.height = crate::LayoutDimension::Fixed(2.0e38);
        assert!(!layout_profile_is_supported(
            invalid_style,
            &container,
            3,
            true
        ));

        let mut invalid_container = container.clone();
        invalid_container.rows = vec![GridTrackSize::LengthPx(20.0)];
        assert!(!layout_profile_is_supported(
            style,
            &invalid_container,
            3,
            true
        ));
    }

    #[test]
    fn row_gap_accepts_bounded_px_and_definite_percentages() {
        for px in [0.0, 2.5, 5.0, 7.5] {
            assert!(row_gap_is_supported(LayoutLengthPercentage::LengthPx(px)));
        }
        for fraction in [0.0, 0.0625, 0.125, 0.1875, 0.2, 1.0, 1.2] {
            assert!(row_gap_is_supported(LayoutLengthPercentage::Percentage(
                fraction
            )));
        }
    }

    #[test]
    fn row_gap_rejects_invalid_values_and_unrepresentable_percentage_extent() {
        for px in [-0.5, 7.5001, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert!(!row_gap_is_supported(LayoutLengthPercentage::LengthPx(px)));
        }
        for fraction in [-0.01, f32::NAN, f32::INFINITY, f32::NEG_INFINITY, f32::MAX] {
            assert!(!row_gap_is_supported(LayoutLengthPercentage::Percentage(
                fraction
            )));
        }
        assert!(!row_gap_is_supported(LayoutLengthPercentage::Auto));
        assert!(!row_gap_is_supported(LayoutLengthPercentage::Calc(
            crate::LayoutCalcId(1)
        )));
    }

    #[test]
    fn row_gap_accepts_the_last_f32_representable_grid_extent() {
        let maximum_fraction = ((f64::from(f32::MAX) - 30.0) / 80.0) as f32;
        assert!(row_gap_is_supported(LayoutLengthPercentage::Percentage(
            maximum_fraction
        )));
        let next_fraction = f32::from_bits(maximum_fraction.to_bits() + 1);
        assert!(!row_gap_is_supported(LayoutLengthPercentage::Percentage(
            next_fraction
        )));
    }
}
