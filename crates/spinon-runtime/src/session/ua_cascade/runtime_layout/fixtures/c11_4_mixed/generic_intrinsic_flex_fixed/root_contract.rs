use spinon_core::{HostNodeHandle, TextOrientation, WritingMode};
use spinon_style::{
    ComputedBackgroundPaint, ComputedBorderPaint, ComputedCssDimension, ComputedCssGridAutoColumns,
    ComputedCssGridAutoFlow, ComputedCssGridAutoRows, ComputedCssGridTrackSize, ComputedCssMaxSize,
    ComputedCssOverflow, ComputedCssPosition, ComputedCssSpacingValue, ComputedElementStyle,
    ComputedLayoutBorder, ComputedStyleSnapshot, OpaqueCssSrgb,
};

use super::Axis;
use super::{RuntimeLayoutFailure, fail, reject};
use super::{mixed_flex_tracks, require_property, zero_edges};

pub(super) fn validate(
    style: &ComputedElementStyle,
    styles: &ComputedStyleSnapshot,
    root: HostNodeHandle,
    axis: Axis,
    border_overflow: bool,
) -> Result<(), RuntimeLayoutFailure> {
    let (width, height, fixed_cross_track, columns, rows, column_gap, row_gap) =
        match (axis, border_overflow) {
            (Axis::Columns, true) => (
                100.0,
                120.0,
                116.0,
                3,
                1,
                ComputedCssSpacingValue::LengthPx(10.0),
                ComputedCssSpacingValue::LengthPx(0.0),
            ),
            (Axis::Columns, false) => (
                300.0,
                120.0,
                120.0,
                3,
                1,
                ComputedCssSpacingValue::LengthPx(10.0),
                ComputedCssSpacingValue::LengthPx(0.0),
            ),
            (Axis::Rows, true) => (
                120.0,
                100.0,
                116.0,
                1,
                3,
                ComputedCssSpacingValue::LengthPx(0.0),
                ComputedCssSpacingValue::LengthPx(10.0),
            ),
            (Axis::Rows, false) => (
                120.0,
                300.0,
                120.0,
                1,
                3,
                ComputedCssSpacingValue::LengthPx(0.0),
                ComputedCssSpacingValue::LengthPx(10.0),
            ),
        };
    for (name, expected) in [
        ("display", "grid"),
        ("box-sizing", "border-box"),
        ("align-items", "normal"),
        ("align-content", "start"),
        ("justify-items", "normal"),
        ("justify-content", "start"),
        ("direction", "ltr"),
        ("writing-mode", "horizontal-tb"),
    ] {
        require_property(style, name, expected)?;
    }
    let grid = style
        .grid_layout
        .as_ref()
        .ok_or_else(|| fail(root.id(), "computed Grid layout missing"))?;
    let mixed_tracks = mixed_flex_tracks();
    let fixed_track = vec![ComputedCssGridTrackSize::LengthPx(fixed_cross_track)];
    let (expected_columns, expected_rows) = match axis {
        Axis::Columns => (mixed_tracks, fixed_track),
        Axis::Rows => (fixed_track, mixed_tracks),
    };
    if styles.profile != spinon_style::ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || styles.writing_style_for(root.id())
            != Some((WritingMode::HorizontalTb, TextOrientation::Mixed))
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(width)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(height)
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || style.layout_position != ComputedCssPosition::Static
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || style.layout_spacing.column_gap != column_gap
        || style.layout_spacing.row_gap != row_gap
        || style.layout_border
            != if border_overflow {
                border_layout()
            } else {
                Default::default()
            }
        || style.layout_overflow_x != Some(ComputedCssOverflow::Visible)
        || style.layout_overflow_y != Some(ComputedCssOverflow::Visible)
        || (border_overflow && style.layout_border_paint != Some(border_paint()))
        || grid.columns.len() != columns
        || grid.rows.len() != rows
        || grid.columns != expected_columns
        || grid.rows != expected_rows
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || grid.auto_rows != ComputedCssGridAutoRows::Auto
        || grid.auto_columns != ComputedCssGridAutoColumns::Auto
        || grid.columns_line_names.len() != columns + 1
        || grid.rows_line_names.len() != rows + 1
        || grid
            .columns_line_names
            .iter()
            .chain(grid.rows_line_names.iter())
            .any(|names| !names.is_empty())
    {
        return Err(reject(
            root.id(),
            "fixed mixed intrinsic/flex Grid style differs",
        ));
    }
    Ok(())
}

pub(super) fn border_layout() -> ComputedLayoutBorder {
    ComputedLayoutBorder {
        top: 2.0,
        right: 2.0,
        bottom: 2.0,
        left: 2.0,
    }
}

pub(super) fn border_paint() -> ComputedBorderPaint {
    let paint = ComputedBackgroundPaint::Opaque(OpaqueCssSrgb {
        red: 240,
        green: 198,
        blue: 116,
    });
    ComputedBorderPaint {
        top: paint,
        right: paint,
        bottom: paint,
        left: paint,
    }
}
