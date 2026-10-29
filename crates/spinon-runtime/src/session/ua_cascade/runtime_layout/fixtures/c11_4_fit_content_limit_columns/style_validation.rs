use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize,
    ComputedCssMaxSize, ComputedCssOverflow, ComputedCssPosition, ComputedElementStyle,
};

use super::super::super::c11_4_nested_grid_replaced_item::{spacing_is, unsupported, zero_edges};
use super::{CHILD_WIDTH, Case, ITEM_HEIGHT, ROOT_HEIGHT, RuntimeLayoutFailure, layout_failure};

pub(super) fn validate_root_style(
    style: &ComputedElementStyle,
    case: Case,
) -> Result<(), RuntimeLayoutFailure> {
    let grid = style.grid_layout.as_ref().ok_or_else(|| {
        layout_failure(
            "missing_computed_grid_layout",
            Some(style.node_id.get()),
            None,
        )
    })?;
    let fit_limit = ComputedCssGridTrackSize::FitContent(
        ComputedCssGridTrackLengthPercentage::LengthPx(case.limit),
    );
    let width = format!("{}px", case.root_width as u32);
    let auto_columns = if case.limit == 30.0 {
        "fit-content(30px)"
    } else if case.limit == 40.0 {
        "fit-content(40px)"
    } else {
        "fit-content(200px)"
    };
    if style.properties.get("display").map(String::as_str) != Some("grid")
        || style.properties.get("width").map(String::as_str) != Some(width.as_str())
        || style.properties.get("height").map(String::as_str) != Some("60px")
        || style
            .properties
            .get("grid-template-columns")
            .map(String::as_str)
            != Some("none")
        || style
            .properties
            .get("grid-template-rows")
            .map(String::as_str)
            != Some("60px")
        || style
            .properties
            .get("grid-auto-columns")
            .map(String::as_str)
            != Some(auto_columns)
        || style.properties.get("grid-auto-rows").map(String::as_str) != Some("auto")
        || style.properties.get("grid-auto-flow").map(String::as_str) != Some("column")
        || style.properties.get("align-content").map(String::as_str) != Some("normal")
        || style.layout_position != ComputedCssPosition::Static
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(case.root_width)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(ROOT_HEIGHT)
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_overflow_x != Some(ComputedCssOverflow::Visible)
        || style.layout_overflow_y != Some(ComputedCssOverflow::Visible)
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || !spacing_is(style.layout_spacing.row_gap, 0.0)
        || !spacing_is(style.layout_spacing.column_gap, 0.0)
        || style.layout_border != spinon_style::ComputedLayoutBorder::default()
        || !grid.columns.is_empty()
        || grid.rows != [ComputedCssGridTrackSize::LengthPx(ROOT_HEIGHT)]
        || grid.auto_columns != ComputedCssGridAutoColumns::TrackList(vec![fit_limit])
        || grid.auto_rows != ComputedCssGridAutoRows::Auto
        || grid.auto_flow != ComputedCssGridAutoFlow::Column
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || grid
            .columns_line_names
            .iter()
            .any(|names| !names.is_empty())
        || grid.rows_line_names.len() != 2
        || grid.rows_line_names.iter().any(|names| !names.is_empty())
    {
        return Err(unsupported(
            style.node_id,
            case.scene_id,
            "computed root style changed",
        ));
    }
    Ok(())
}

pub(super) fn validate_item_style(
    style: &ComputedElementStyle,
    case: Case,
) -> Result<(), RuntimeLayoutFailure> {
    let expected_min_width = if case.automatic_minimum {
        ComputedCssDimension::Auto
    } else {
        ComputedCssDimension::LengthPx(0.0)
    };
    if style.properties.get("display").map(String::as_str) != Some("block")
        || style.layout_position != ComputedCssPosition::Static
        || style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_width != expected_min_width
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_overflow_x != Some(ComputedCssOverflow::Visible)
        || style.layout_overflow_y != Some(ComputedCssOverflow::Visible)
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || style.layout_border != spinon_style::ComputedLayoutBorder::default()
    {
        return Err(unsupported(
            style.node_id,
            case.scene_id,
            "grid item style changed",
        ));
    }
    Ok(())
}

pub(super) fn validate_child_style(
    style: &ComputedElementStyle,
    case: Case,
) -> Result<(), RuntimeLayoutFailure> {
    if style.properties.get("display").map(String::as_str) != Some("block")
        || style.layout_position != ComputedCssPosition::Static
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(CHILD_WIDTH)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(ITEM_HEIGHT)
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || style.layout_border != spinon_style::ComputedLayoutBorder::default()
    {
        return Err(unsupported(
            style.node_id,
            case.scene_id,
            "nested child style changed",
        ));
    }
    Ok(())
}
