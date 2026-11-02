use crate::{
    GridTrackSize, LayoutCssMathProperty, LayoutCssMathValue, LayoutDimension, LayoutError,
    LayoutLengthPercentage, LayoutStyle,
};

pub fn resolve_root_html_gap_px(
    node: spinon_core::NodeId,
    property: &'static str,
    gap: LayoutLengthPercentage,
    style: LayoutStyle,
    css_math: &[LayoutCssMathValue],
) -> Result<f32, LayoutError> {
    let (basis, min_size, max_size, min_property, max_property) = match property {
        "row-gap" => (
            style.height,
            style.min_height,
            style.max_height,
            "min-height",
            "max-height",
        ),
        "column-gap" => (
            style.width,
            style.min_width,
            style.max_width,
            "min-width",
            "max-width",
        ),
        _ => return Err(LayoutError::InvalidGridTrack { node, property }),
    };
    let math_property = match property {
        "row-gap" => LayoutCssMathProperty::RowGap,
        "column-gap" => LayoutCssMathProperty::ColumnGap,
        _ => return Err(LayoutError::InvalidGridTrack { node, property }),
    };
    let value = match gap {
        LayoutLengthPercentage::LengthPx(value) if value.is_finite() && value >= 0.0 => value,
        LayoutLengthPercentage::Percentage(value) if value.is_finite() && value >= 0.0 => {
            let LayoutDimension::Fixed(basis) = basis else {
                return Err(LayoutError::InvalidGridTrack { node, property });
            };
            if !basis.is_finite() || basis < 0.0 {
                return Err(LayoutError::InvalidGridTrack { node, property });
            }
            if min_size != LayoutDimension::Auto {
                return Err(LayoutError::UnsupportedGridContainerStyle {
                    node,
                    property: min_property,
                });
            }
            if max_size != LayoutDimension::Auto {
                return Err(LayoutError::UnsupportedGridContainerStyle {
                    node,
                    property: max_property,
                });
            }
            let resolved = f64::from(value) * f64::from(basis);
            if !resolved.is_finite() || resolved > f64::from(f32::MAX) {
                return Err(LayoutError::InvalidGridTrack { node, property });
            }
            resolved as f32
        }
        LayoutLengthPercentage::Calc(id) => {
            let Some(value) = css_math.iter().find(|value| value.id == id) else {
                return Err(LayoutError::MissingCssMath { node, property, id });
            };
            if value.node_id != node || value.property != math_property {
                return Err(LayoutError::CssMathBindingMismatch { node, property, id });
            }
            let contains_percentage = value
                .contains_percentage()
                .map_err(|reason| value.error(reason))?;
            let calculation_basis = if contains_percentage {
                if min_size != LayoutDimension::Auto {
                    return Err(LayoutError::UnsupportedGridContainerStyle {
                        node,
                        property: min_property,
                    });
                }
                if max_size != LayoutDimension::Auto {
                    return Err(LayoutError::UnsupportedGridContainerStyle {
                        node,
                        property: max_property,
                    });
                }
                let LayoutDimension::Fixed(basis) = basis else {
                    return Err(LayoutError::InvalidGridTrack { node, property });
                };
                if !basis.is_finite() || basis < 0.0 {
                    return Err(LayoutError::InvalidGridTrack { node, property });
                }
                basis
            } else {
                // Length-only CSS math has no percentage basis requirement.
                0.0
            };
            value
                .resolve(calculation_basis)
                .map_err(|reason| value.error(reason))?
        }
        _ => return Err(LayoutError::InvalidGridTrack { node, property }),
    };
    Ok(value)
}

pub(super) fn track_extent(
    node: spinon_core::NodeId,
    property: &'static str,
    track: GridTrackSize,
    basis: LayoutDimension,
) -> Result<f64, LayoutError> {
    match track {
        GridTrackSize::LengthPx(value) if value.is_finite() && value >= 0.0 => Ok(f64::from(value)),
        GridTrackSize::Percentage(value) if value.is_finite() && value >= 0.0 => {
            let LayoutDimension::Fixed(basis) = basis else {
                return Err(LayoutError::InvalidGridTrack { node, property });
            };
            if !basis.is_finite() || basis < 0.0 {
                return Err(LayoutError::InvalidGridTrack { node, property });
            }
            let resolved = f64::from(value) * f64::from(basis);
            if !resolved.is_finite() || resolved > f64::from(f32::MAX) {
                return Err(LayoutError::InvalidGridTrack { node, property });
            }
            Ok(resolved)
        }
        GridTrackSize::Fraction(value) if value.is_finite() && value >= 0.0 => Ok(0.0),
        _ => Err(LayoutError::InvalidGridTrack { node, property }),
    }
}

pub(super) fn validate_extent(
    node: spinon_core::NodeId,
    property: &'static str,
    extent: f64,
) -> Result<(), LayoutError> {
    if extent.is_finite() && extent <= f64::from(f32::MAX) {
        Ok(())
    } else {
        Err(LayoutError::InvalidGridTrack { node, property })
    }
}

pub(super) fn validate_track_extent(
    node: spinon_core::NodeId,
    property: &'static str,
    gap_property: &'static str,
    tracks: &[GridTrackSize],
    gap: f32,
    required_tracks: usize,
    basis: LayoutDimension,
) -> Result<(), LayoutError> {
    let gap_extent = f64::from(gap) * required_tracks.saturating_sub(1) as f64;
    validate_extent(node, gap_property, gap_extent)?;
    let mut extent = gap_extent;
    for track in tracks {
        extent += track_extent(node, property, *track, basis)?;
        validate_extent(node, property, extent)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LayoutCalcId, LayoutCssMath};

    #[test]
    fn pure_length_math_gap_does_not_need_a_definite_axis_size() {
        let node = spinon_core::NodeId::new(1).unwrap();
        let math = LayoutCssMathValue {
            id: LayoutCalcId(1),
            node_id: node,
            property: LayoutCssMathProperty::RowGap,
            expression: LayoutCssMath::Sum(vec![
                LayoutCssMath::LengthPx(5.0),
                LayoutCssMath::LengthPx(7.0),
            ]),
        };

        let gap = resolve_root_html_gap_px(
            node,
            "row-gap",
            LayoutLengthPercentage::Calc(math.id),
            LayoutStyle::default(),
            &[math],
        );

        assert_eq!(gap, Ok(12.0));
    }

    #[test]
    fn percentage_math_gap_requires_a_definite_axis_size() {
        let node = spinon_core::NodeId::new(1).unwrap();
        let math = LayoutCssMathValue {
            id: LayoutCalcId(1),
            node_id: node,
            property: LayoutCssMathProperty::RowGap,
            expression: LayoutCssMath::Sum(vec![
                LayoutCssMath::Percentage(0.05),
                LayoutCssMath::LengthPx(2.0),
            ]),
        };

        let gap = resolve_root_html_gap_px(
            node,
            "row-gap",
            LayoutLengthPercentage::Calc(math.id),
            LayoutStyle::default(),
            &[math],
        );

        assert!(matches!(
            gap,
            Err(LayoutError::InvalidGridTrack {
                node: error_node,
                property: "row-gap"
            }) if error_node == node
        ));
    }

    #[test]
    fn math_gap_requires_a_matching_id_node_and_property() {
        let node = spinon_core::NodeId::new(1).unwrap();
        let other_node = spinon_core::NodeId::new(2).unwrap();
        let id = LayoutCalcId(1);
        let missing = resolve_root_html_gap_px(
            node,
            "row-gap",
            LayoutLengthPercentage::Calc(id),
            LayoutStyle::default(),
            &[],
        );
        assert!(matches!(
            missing,
            Err(LayoutError::MissingCssMath {
                node: error_node,
                property: "row-gap",
                id: error_id,
            }) if error_node == node && error_id == id
        ));

        for (owner, property) in [
            (other_node, LayoutCssMathProperty::RowGap),
            (node, LayoutCssMathProperty::ColumnGap),
        ] {
            let math = LayoutCssMathValue {
                id,
                node_id: owner,
                property,
                expression: LayoutCssMath::LengthPx(12.0),
            };
            let mismatch = resolve_root_html_gap_px(
                node,
                "row-gap",
                LayoutLengthPercentage::Calc(id),
                LayoutStyle::default(),
                &[math],
            );
            assert!(matches!(
                mismatch,
                Err(LayoutError::CssMathBindingMismatch {
                    node: error_node,
                    property: "row-gap",
                    id: error_id,
                }) if error_node == node && error_id == id
            ));
        }
    }
}
