use super::root_html_column::{self, CssMathIndex};
use crate::{GridAutoRows, GridTrackSize, LayoutCssMathProperty, LayoutDimension, LayoutError};

const MATH_PROPERTY: LayoutCssMathProperty = LayoutCssMathProperty::GridTemplateRows;

pub(crate) fn has_calc_auto_rows(auto_rows: &GridAutoRows) -> bool {
    matches!(
        auto_rows,
        GridAutoRows::TrackList(tracks)
            if tracks.iter().any(|track| matches!(track, GridTrackSize::Calc(_)))
    )
}

pub(crate) fn validate_auto_row_calc(
    node: spinon_core::NodeId,
    id: crate::LayoutCalcId,
    css_math: &CssMathIndex<'_>,
) -> Result<(), LayoutError> {
    let value = bound_math(node, id, css_math)?;
    if !root_html_column::is_supported_expression(&value.expression) {
        return Err(unsupported_calc(node));
    }
    Ok(())
}

pub(crate) fn calc_track_extent(
    node: spinon_core::NodeId,
    track: GridTrackSize,
    basis: LayoutDimension,
    css_math: &CssMathIndex<'_>,
) -> Result<f64, LayoutError> {
    let GridTrackSize::Calc(id) = track else {
        return Err(LayoutError::InvalidGridTrack {
            node,
            property: "grid-auto-rows",
        });
    };
    let value = bound_math(node, id, css_math)?;
    if !root_html_column::is_supported_expression(&value.expression) {
        return Err(unsupported_calc(node));
    }
    let LayoutDimension::Fixed(basis) = basis else {
        return Err(LayoutError::InvalidGridTrack {
            node,
            property: "grid-auto-rows",
        });
    };
    if !basis.is_finite() || basis < 0.0 {
        return Err(LayoutError::InvalidGridTrack {
            node,
            property: "grid-auto-rows",
        });
    }
    let resolved = value.resolve(basis).map_err(|reason| value.error(reason))?;
    if !resolved.is_finite() || resolved < 0.0 {
        return Err(LayoutError::InvalidGridTrack {
            node,
            property: "grid-auto-rows",
        });
    }
    Ok(f64::from(resolved))
}

fn bound_math<'a>(
    node: spinon_core::NodeId,
    id: crate::LayoutCalcId,
    css_math: &CssMathIndex<'a>,
) -> Result<&'a crate::LayoutCssMathValue, LayoutError> {
    let Some(value) = css_math.get(&id).copied() else {
        return Err(LayoutError::MissingCssMath {
            node,
            property: "grid-auto-rows",
            id,
        });
    };
    if value.node_id != node || value.property != MATH_PROPERTY {
        return Err(LayoutError::CssMathBindingMismatch {
            node,
            property: "grid-auto-rows",
            id,
        });
    }
    Ok(value)
}

fn unsupported_calc(node: spinon_core::NodeId) -> LayoutError {
    LayoutError::UnsupportedGridAutoRows {
        node,
        reason: "Root HTML implicit row calc() requires one finite px term and one finite percentage term",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LayoutCalcId, LayoutCssMath, LayoutCssMathValue};

    fn math(expression: LayoutCssMath) -> LayoutCssMathValue {
        LayoutCssMathValue {
            id: LayoutCalcId(17),
            node_id: spinon_core::NodeId::new(1).unwrap(),
            property: MATH_PROPERTY,
            expression,
        }
    }

    #[test]
    fn resolves_signed_px_percentage_sum_with_grid_row_range_clamping() {
        let node = spinon_core::NodeId::new(1).unwrap();
        for (expression, height, expected) in [
            (
                LayoutCssMath::Sum(vec![
                    LayoutCssMath::Percentage(0.1),
                    LayoutCssMath::LengthPx(2.0),
                ]),
                100.0,
                12.0,
            ),
            (
                LayoutCssMath::Sum(vec![
                    LayoutCssMath::LengthPx(1.0),
                    LayoutCssMath::Negate(Box::new(LayoutCssMath::Percentage(0.1))),
                ]),
                100.0,
                0.0,
            ),
            (
                LayoutCssMath::Sum(vec![
                    LayoutCssMath::LengthPx(1.0),
                    LayoutCssMath::Percentage(1.2),
                ]),
                100.0,
                121.0,
            ),
            (
                LayoutCssMath::Sum(vec![
                    LayoutCssMath::Percentage(0.1),
                    LayoutCssMath::LengthPx(5.0),
                ]),
                0.0,
                5.0,
            ),
        ] {
            let value = math(expression);
            let css_math = root_html_column::index_css_math(std::slice::from_ref(&value)).unwrap();
            let resolved = calc_track_extent(
                node,
                GridTrackSize::Calc(value.id),
                LayoutDimension::Fixed(height),
                &css_math,
            )
            .unwrap();
            assert!((resolved - expected).abs() < 0.001);
        }
    }

    #[test]
    fn rejects_wrong_property_owner_and_missing_row_math_bindings() {
        let node = spinon_core::NodeId::new(1).unwrap();
        let value = math(LayoutCssMath::Sum(vec![
            LayoutCssMath::LengthPx(2.0),
            LayoutCssMath::Percentage(0.1),
        ]));
        let missing = root_html_column::index_css_math(&[]).unwrap();
        assert!(matches!(
            validate_auto_row_calc(node, value.id, &missing),
            Err(LayoutError::MissingCssMath {
                property: "grid-auto-rows",
                ..
            })
        ));

        let mut wrong_property = value.clone();
        wrong_property.property = LayoutCssMathProperty::GridTemplateColumns;
        let wrong_property_values = [wrong_property];
        let wrong_property = root_html_column::index_css_math(&wrong_property_values).unwrap();
        assert!(matches!(
            validate_auto_row_calc(node, value.id, &wrong_property),
            Err(LayoutError::CssMathBindingMismatch {
                property: "grid-auto-rows",
                ..
            })
        ));

        let mut wrong_owner = value.clone();
        wrong_owner.node_id = spinon_core::NodeId::new(2).unwrap();
        let wrong_owner_values = [wrong_owner];
        let wrong_owner = root_html_column::index_css_math(&wrong_owner_values).unwrap();
        assert!(matches!(
            validate_auto_row_calc(node, value.id, &wrong_owner),
            Err(LayoutError::CssMathBindingMismatch {
                property: "grid-auto-rows",
                ..
            })
        ));
    }

    #[test]
    fn rejects_unsupported_or_non_finite_row_calc_shapes() {
        let node = spinon_core::NodeId::new(1).unwrap();
        for expression in [
            LayoutCssMath::LengthPx(5.0),
            LayoutCssMath::Sum(vec![
                LayoutCssMath::LengthPx(1.0),
                LayoutCssMath::LengthPx(2.0),
            ]),
            LayoutCssMath::Sum(vec![
                LayoutCssMath::Percentage(0.1),
                LayoutCssMath::LengthPx(2.0),
                LayoutCssMath::LengthPx(3.0),
            ]),
            LayoutCssMath::Sum(vec![
                LayoutCssMath::Product(vec![LayoutCssMath::LengthPx(2.0)]),
                LayoutCssMath::Percentage(0.1),
            ]),
            LayoutCssMath::Sum(vec![
                LayoutCssMath::LengthPx(f32::NAN),
                LayoutCssMath::Percentage(0.1),
            ]),
        ] {
            let value = math(expression);
            let css_math = root_html_column::index_css_math(std::slice::from_ref(&value)).unwrap();
            assert!(matches!(
                validate_auto_row_calc(node, value.id, &css_math),
                Err(LayoutError::UnsupportedGridAutoRows { .. })
            ));
        }
    }

    #[test]
    fn rejects_indefinite_height_basis_and_duplicate_math_ids() {
        let node = spinon_core::NodeId::new(1).unwrap();
        let value = math(LayoutCssMath::Sum(vec![
            LayoutCssMath::LengthPx(2.0),
            LayoutCssMath::Percentage(0.1),
        ]));
        let css_math = root_html_column::index_css_math(std::slice::from_ref(&value)).unwrap();
        assert!(matches!(
            calc_track_extent(
                node,
                GridTrackSize::Calc(value.id),
                LayoutDimension::Auto,
                &css_math,
            ),
            Err(LayoutError::InvalidGridTrack {
                property: "grid-auto-rows",
                ..
            })
        ));
        assert!(matches!(
            root_html_column::index_css_math(&[value.clone(), value]),
            Err(LayoutError::DuplicateCssMathId)
        ));
    }
}
