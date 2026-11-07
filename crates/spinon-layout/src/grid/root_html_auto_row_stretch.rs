use super::{
    GridAutoColumns, GridAutoFlow, GridAutoRows, GridContainerInput, GridTrackSize,
    root_html_column::{self, CssMathIndex},
};
use crate::{
    LayoutCssMathProperty, LayoutDimension, LayoutError, LayoutLengthPercentage, LayoutStyle,
};

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

pub(crate) fn row_gap_is_supported(
    node: spinon_core::NodeId,
    gap: LayoutLengthPercentage,
    css_math: &CssMathIndex<'_>,
) -> Result<bool, LayoutError> {
    match gap {
        LayoutLengthPercentage::LengthPx(value) => {
            Ok(value.is_finite() && (0.0..=7.5).contains(&value))
        }
        LayoutLengthPercentage::Percentage(fraction) if fraction.is_finite() && fraction >= 0.0 => {
            let used_gap = f64::from(fraction) * GRID_BLOCK_SIZE_PX;
            Ok(grid_extent_is_supported(used_gap))
        }
        LayoutLengthPercentage::Calc(id) => {
            let Some(value) = css_math.get(&id).copied() else {
                return Err(LayoutError::MissingCssMath {
                    node,
                    property: "row-gap",
                    id,
                });
            };
            if value.node_id != node || value.property != LayoutCssMathProperty::RowGap {
                return Err(LayoutError::CssMathBindingMismatch {
                    node,
                    property: "row-gap",
                    id,
                });
            }
            if !root_html_column::is_supported_expression(&value.expression) {
                return Ok(false);
            }
            let resolved = value
                .resolve(GRID_BLOCK_SIZE_PX as f32)
                .map_err(|reason| value.error(reason))?;
            Ok(resolved.is_finite()
                && resolved >= 0.0
                && grid_extent_is_supported(f64::from(resolved)))
        }
        LayoutLengthPercentage::Percentage(_) | LayoutLengthPercentage::Auto => Ok(false),
    }
}

fn grid_extent_is_supported(gap: f64) -> bool {
    let grid_extent = gap * IMPLICIT_ROW_GUTTER_COUNT + BASE_ROW_EXTENT_PX;
    gap.is_finite() && gap >= 0.0 && grid_extent.is_finite() && grid_extent <= f64::from(f32::MAX)
}

#[cfg(test)]
mod tests {
    use super::layout_profile_is_supported;
    use super::row_gap_is_supported;
    use crate::{
        GridPlacementMode, GridTrackSize, LayoutCssMath, LayoutCssMathProperty, LayoutCssMathValue,
        LayoutLengthPercentage, LayoutStyle,
    };
    use std::collections::BTreeMap;

    fn node_id(value: u64) -> spinon_core::NodeId {
        spinon_core::NodeId::new(value).unwrap()
    }

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
        let math = BTreeMap::new();
        for px in [0.0, 2.5, 5.0, 7.5] {
            assert!(
                row_gap_is_supported(node_id(1), LayoutLengthPercentage::LengthPx(px), &math)
                    .unwrap()
            );
        }
        for fraction in [0.0, 0.0625, 0.125, 0.1875, 0.2, 1.0, 1.2] {
            assert!(
                row_gap_is_supported(
                    node_id(1),
                    LayoutLengthPercentage::Percentage(fraction),
                    &math
                )
                .unwrap()
            );
        }
    }

    #[test]
    fn row_gap_rejects_invalid_values_and_unrepresentable_percentage_extent() {
        let math = BTreeMap::new();
        for px in [-0.5, 7.5001, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert!(
                !row_gap_is_supported(node_id(1), LayoutLengthPercentage::LengthPx(px), &math)
                    .unwrap()
            );
        }
        for fraction in [-0.01, f32::NAN, f32::INFINITY, f32::NEG_INFINITY, f32::MAX] {
            assert!(
                !row_gap_is_supported(
                    node_id(1),
                    LayoutLengthPercentage::Percentage(fraction),
                    &math
                )
                .unwrap()
            );
        }
        assert!(!row_gap_is_supported(node_id(1), LayoutLengthPercentage::Auto, &math).unwrap());
        assert!(matches!(
            row_gap_is_supported(node_id(1), LayoutLengthPercentage::Calc(crate::LayoutCalcId(1)), &math),
            Err(crate::LayoutError::MissingCssMath { node, property: "row-gap", .. }) if node == node_id(1)
        ));
    }

    #[test]
    fn row_gap_accepts_the_last_f32_representable_grid_extent() {
        let math = BTreeMap::new();
        let maximum_fraction = ((f64::from(f32::MAX) - 30.0) / 80.0) as f32;
        assert!(
            row_gap_is_supported(
                node_id(1),
                LayoutLengthPercentage::Percentage(maximum_fraction),
                &math
            )
            .unwrap()
        );
        let next_fraction = f32::from_bits(maximum_fraction.to_bits() + 1);
        assert!(
            !row_gap_is_supported(
                node_id(1),
                LayoutLengthPercentage::Percentage(next_fraction),
                &math
            )
            .unwrap()
        );
    }

    #[test]
    fn row_gap_accepts_bound_two_term_length_percentage_math_and_clamps_negative_result() {
        let node = node_id(1);
        for (expression, expected) in [
            (
                LayoutCssMath::Sum(vec![
                    LayoutCssMath::Percentage(0.0625),
                    LayoutCssMath::LengthPx(2.0),
                ]),
                4.5,
            ),
            (
                LayoutCssMath::Sum(vec![
                    LayoutCssMath::LengthPx(-2.0),
                    LayoutCssMath::Percentage(0.25),
                ]),
                8.0,
            ),
            (
                LayoutCssMath::Sum(vec![
                    LayoutCssMath::LengthPx(1.0),
                    LayoutCssMath::Negate(Box::new(LayoutCssMath::Percentage(0.1))),
                ]),
                0.0,
            ),
        ] {
            let value = LayoutCssMathValue {
                id: crate::LayoutCalcId(1),
                node_id: node,
                property: LayoutCssMathProperty::RowGap,
                expression,
            };
            let math = BTreeMap::from([(value.id, &value)]);
            assert!(
                row_gap_is_supported(node, LayoutLengthPercentage::Calc(value.id), &math).unwrap()
            );
            assert_eq!(value.resolve(40.0).unwrap(), expected);
        }
    }

    #[test]
    fn row_gap_applies_existing_css_math_range_before_checking_grid_extent() {
        let node = node_id(1);
        let value = LayoutCssMathValue {
            id: crate::LayoutCalcId(1),
            node_id: node,
            property: LayoutCssMathProperty::RowGap,
            expression: LayoutCssMath::Sum(vec![
                LayoutCssMath::LengthPx(f32::MAX),
                LayoutCssMath::Percentage(0.0),
            ]),
        };
        let math = BTreeMap::from([(value.id, &value)]);

        assert_eq!(value.resolve(40.0).unwrap(), 33_554_428.0);
        assert!(row_gap_is_supported(node, LayoutLengthPercentage::Calc(value.id), &math).unwrap());
    }

    #[test]
    fn row_gap_rejects_wrong_node_property_and_unsupported_math_shape() {
        let node = node_id(1);
        let value = LayoutCssMathValue {
            id: crate::LayoutCalcId(1),
            node_id: node_id(2),
            property: LayoutCssMathProperty::RowGap,
            expression: LayoutCssMath::Sum(vec![
                LayoutCssMath::Percentage(0.1),
                LayoutCssMath::LengthPx(1.0),
            ]),
        };
        let math = BTreeMap::from([(value.id, &value)]);
        assert!(matches!(
            row_gap_is_supported(node, LayoutLengthPercentage::Calc(value.id), &math),
            Err(crate::LayoutError::CssMathBindingMismatch { node: error_node, property: "row-gap", .. }) if error_node == node
        ));

        let wrong_property = LayoutCssMathValue {
            node_id: node,
            property: LayoutCssMathProperty::ColumnGap,
            ..value.clone()
        };
        let math = BTreeMap::from([(wrong_property.id, &wrong_property)]);
        assert!(matches!(
            row_gap_is_supported(node, LayoutLengthPercentage::Calc(wrong_property.id), &math),
            Err(crate::LayoutError::CssMathBindingMismatch {
                property: "row-gap",
                ..
            })
        ));

        let unsupported = LayoutCssMathValue {
            node_id: node,
            expression: LayoutCssMath::Sum(vec![
                LayoutCssMath::LengthPx(1.0),
                LayoutCssMath::LengthPx(2.0),
            ]),
            ..value
        };
        let math = BTreeMap::from([(unsupported.id, &unsupported)]);
        assert!(
            !row_gap_is_supported(node, LayoutLengthPercentage::Calc(unsupported.id), &math)
                .unwrap()
        );
    }
}
