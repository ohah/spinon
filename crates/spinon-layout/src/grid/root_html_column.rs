use std::collections::BTreeMap;

use crate::{
    GridAutoColumns, GridTrackSize, LayoutCalcId, LayoutCssMath, LayoutCssMathProperty,
    LayoutCssMathValue, LayoutDimension, LayoutError,
};

const MATH_PROPERTY: LayoutCssMathProperty = LayoutCssMathProperty::GridTemplateColumns;

pub(crate) type CssMathIndex<'a> = BTreeMap<LayoutCalcId, &'a LayoutCssMathValue>;

pub(crate) fn has_calc_auto_columns(auto_columns: &GridAutoColumns) -> bool {
    matches!(
        auto_columns,
        GridAutoColumns::TrackList(tracks)
            if tracks.iter().any(|track| matches!(track, GridTrackSize::Calc(_)))
    )
}

pub(crate) fn index_css_math(
    css_math: &[LayoutCssMathValue],
) -> Result<CssMathIndex<'_>, LayoutError> {
    let mut index = BTreeMap::new();
    for value in css_math {
        if index.insert(value.id, value).is_some() {
            return Err(LayoutError::DuplicateCssMathId);
        }
    }
    Ok(index)
}

pub(crate) fn validate_auto_column_calc(
    node: spinon_core::NodeId,
    id: crate::LayoutCalcId,
    css_math: &CssMathIndex<'_>,
) -> Result<(), LayoutError> {
    let value = bound_math(node, id, css_math)?;
    if !is_supported_expression(&value.expression) {
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
            property: "grid-auto-columns",
        });
    };
    let value = bound_math(node, id, css_math)?;
    if !is_supported_expression(&value.expression) {
        return Err(unsupported_calc(node));
    }
    let LayoutDimension::Fixed(basis) = basis else {
        return Err(LayoutError::InvalidGridTrack {
            node,
            property: "grid-auto-columns",
        });
    };
    if !basis.is_finite() || basis < 0.0 {
        return Err(LayoutError::InvalidGridTrack {
            node,
            property: "grid-auto-columns",
        });
    }
    let resolved = value.resolve(basis).map_err(|reason| value.error(reason))?;
    if !resolved.is_finite() || resolved < 0.0 {
        return Err(LayoutError::InvalidGridTrack {
            node,
            property: "grid-auto-columns",
        });
    }
    Ok(f64::from(resolved))
}

fn bound_math<'a>(
    node: spinon_core::NodeId,
    id: crate::LayoutCalcId,
    css_math: &CssMathIndex<'a>,
) -> Result<&'a LayoutCssMathValue, LayoutError> {
    let Some(value) = css_math.get(&id).copied() else {
        return Err(LayoutError::MissingCssMath {
            node,
            property: "grid-auto-columns",
            id,
        });
    };
    if value.node_id != node || value.property != MATH_PROPERTY {
        return Err(LayoutError::CssMathBindingMismatch {
            node,
            property: "grid-auto-columns",
            id,
        });
    }
    Ok(value)
}

fn is_supported_expression(expression: &LayoutCssMath) -> bool {
    let LayoutCssMath::Sum(terms) = expression else {
        return false;
    };
    if terms.len() != 2 {
        return false;
    }
    let mut has_length = false;
    let mut has_percentage = false;
    for term in terms {
        match signed_term(term) {
            Some(SignedTerm::Length(value)) if value.is_finite() && !has_length => {
                has_length = true;
            }
            Some(SignedTerm::Percentage(value)) if value.is_finite() && !has_percentage => {
                has_percentage = true;
            }
            _ => return false,
        }
    }
    has_length && has_percentage
}

enum SignedTerm {
    Length(f32),
    Percentage(f32),
}

fn signed_term(term: &LayoutCssMath) -> Option<SignedTerm> {
    match term {
        LayoutCssMath::LengthPx(value) => Some(SignedTerm::Length(*value)),
        LayoutCssMath::Percentage(value) => Some(SignedTerm::Percentage(*value)),
        LayoutCssMath::Negate(value) => match value.as_ref() {
            LayoutCssMath::LengthPx(value) => Some(SignedTerm::Length(-value)),
            LayoutCssMath::Percentage(value) => Some(SignedTerm::Percentage(-value)),
            _ => None,
        },
        _ => None,
    }
}

fn unsupported_calc(node: spinon_core::NodeId) -> LayoutError {
    LayoutError::UnsupportedGridAutoColumns {
        node,
        reason: "Root HTML column-flow calc() requires one finite px term and one finite percentage term",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn math(expression: LayoutCssMath) -> LayoutCssMathValue {
        LayoutCssMathValue {
            id: crate::LayoutCalcId(7),
            node_id: spinon_core::NodeId::new(1).unwrap(),
            property: MATH_PROPERTY,
            expression,
        }
    }

    #[test]
    fn accepts_two_term_signed_length_percentage_sums_only() {
        for expression in [
            LayoutCssMath::Sum(vec![
                LayoutCssMath::Percentage(0.1),
                LayoutCssMath::LengthPx(5.0),
            ]),
            LayoutCssMath::Sum(vec![
                LayoutCssMath::Negate(Box::new(LayoutCssMath::LengthPx(1.0))),
                LayoutCssMath::Percentage(0.1),
            ]),
        ] {
            assert!(is_supported_expression(&expression));
        }
        for expression in [
            LayoutCssMath::LengthPx(5.0),
            LayoutCssMath::Sum(vec![LayoutCssMath::LengthPx(1.0); 2]),
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
            assert!(!is_supported_expression(&expression));
        }
    }

    #[test]
    fn resolves_calc_track_against_definite_width_and_checks_binding() {
        let node = spinon_core::NodeId::new(1).unwrap();
        let value = math(LayoutCssMath::Sum(vec![
            LayoutCssMath::Percentage(0.1),
            LayoutCssMath::LengthPx(5.0),
        ]));
        let css_math = index_css_math(std::slice::from_ref(&value)).unwrap();
        assert_eq!(
            calc_track_extent(
                node,
                GridTrackSize::Calc(value.id),
                LayoutDimension::Fixed(240.0),
                &css_math,
            ),
            Ok(29.0)
        );
        assert!(matches!(
            calc_track_extent(
                node,
                GridTrackSize::Calc(value.id),
                LayoutDimension::Auto,
                &css_math,
            ),
            Err(LayoutError::InvalidGridTrack {
                property: "grid-auto-columns",
                ..
            })
        ));
        let wrong_owner = LayoutCssMathValue {
            node_id: spinon_core::NodeId::new(2).unwrap(),
            ..value.clone()
        };
        let wrong_owner_values = [wrong_owner];
        let wrong_owner = index_css_math(&wrong_owner_values).unwrap();
        assert!(matches!(
            validate_auto_column_calc(node, value.id, &wrong_owner),
            Err(LayoutError::CssMathBindingMismatch { .. })
        ));
    }

    #[test]
    fn calc_track_uses_the_existing_css_layout_value_ceiling() {
        let node = spinon_core::NodeId::new(1).unwrap();
        let value = math(LayoutCssMath::Sum(vec![
            LayoutCssMath::LengthPx(1.0e30),
            LayoutCssMath::Percentage(1.0e30),
        ]));
        let css_math = index_css_math(std::slice::from_ref(&value)).unwrap();
        assert_eq!(
            calc_track_extent(
                node,
                GridTrackSize::Calc(value.id),
                LayoutDimension::Fixed(240.0),
                &css_math,
            ),
            Ok(33_554_428.0)
        );
    }
}
