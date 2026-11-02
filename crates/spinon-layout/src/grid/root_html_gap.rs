use crate::{GridTrackSize, LayoutDimension, LayoutError, LayoutLengthPercentage, LayoutStyle};

pub fn resolve_root_html_gap_px(
    node: spinon_core::NodeId,
    property: &'static str,
    gap: LayoutLengthPercentage,
    style: LayoutStyle,
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
