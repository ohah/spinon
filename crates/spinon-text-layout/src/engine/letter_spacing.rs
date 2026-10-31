use parley::layout::LineMetrics;

/// Computes the line advance with CSS Text's line-edge letter-spacing suppression.
///
/// Parley stores letter spacing after each atom. For the current horizontal LTR text
/// profile, removing one trailing spacing from each non-empty visible line is equivalent
/// to keeping only the spacing between adjacent typographic character units.
pub(super) fn visible_line_advance(metrics: &LineMetrics, letter_spacing: f32) -> f32 {
    let visible_advance = metrics.advance - metrics.hanging_advance;
    if visible_advance > 0.0 {
        visible_advance - letter_spacing
    } else {
        visible_advance
    }
}
