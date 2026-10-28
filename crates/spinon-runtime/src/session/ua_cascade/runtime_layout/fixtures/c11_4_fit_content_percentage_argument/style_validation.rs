use spinon_core::NodeId;
use spinon_layout::{
    GridAutoFlow, GridAutoRows, GridContainerInput, GridPlacementMode, GridTrackLengthPercentage,
    GridTrackSize, LayoutCalcId, LayoutCssMath, LayoutCssMathProperty, LayoutCssMathValue,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize,
    ComputedCssMath, ComputedCssMaxSize, ComputedElementStyle,
};

use super::super::super::{RuntimeLayoutFailure, layout_failure};
use super::super::c11_4_nested_grid_replaced_item::{spacing_is, unsupported, zero_edges};
use super::{FitContentArgumentScene, failure};

pub(super) fn validate_root_style(
    style: &ComputedElementStyle,
    scene: &FitContentArgumentScene,
) -> Result<(), RuntimeLayoutFailure> {
    let grid = style.grid_layout.as_ref().ok_or_else(|| {
        layout_failure(
            "missing_computed_grid_layout",
            Some(style.node_id.get()),
            None,
        )
    })?;
    let expected_auto_rows = match scene.scene_id {
        "fit-content-percentage-row-wpt" => {
            ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::FitContent(
                ComputedCssGridTrackLengthPercentage::Percentage(0.5),
            )])
        }
        "fit-content-calc-percentage-row-wpt" => {
            ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::FitContent(
                ComputedCssGridTrackLengthPercentage::Calc(ComputedCssMath::Sum(vec![
                    ComputedCssMath::Percentage(0.1),
                    ComputedCssMath::LengthPx(5.0),
                ])),
            )])
        }
        _ => return Err(failure("unknown fit-content computed-style case")),
    };
    if style.properties.get("display").map(String::as_str) != Some("grid")
        || style.properties.get("width").map(String::as_str) != Some("60px")
        || style.properties.get("height").map(String::as_str) != Some("300px")
        || style
            .properties
            .get("grid-template-columns")
            .map(String::as_str)
            != Some("60px")
        || style
            .properties
            .get("grid-template-rows")
            .map(String::as_str)
            != Some("none")
        || style.properties.get("grid-auto-rows").map(String::as_str) != Some(scene.auto_rows_css)
        || style.properties.get("align-content").map(String::as_str) != Some("normal")
        || style.properties.get("grid-auto-flow").map(String::as_str) != Some("row")
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(60.0)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(300.0)
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_edges(style.layout_spacing.padding)
        || !zero_edges(style.layout_spacing.margin)
        || !spacing_is(style.layout_spacing.row_gap, 0.0)
        || !spacing_is(style.layout_spacing.column_gap, 0.0)
        || style.layout_border != spinon_style::ComputedLayoutBorder::default()
        || grid.columns != [ComputedCssGridTrackSize::LengthPx(60.0)]
        || !grid.rows.is_empty()
        || grid.auto_rows != expected_auto_rows
        || grid.auto_columns != ComputedCssGridAutoColumns::Auto
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || !empty_line_name_groups(&grid.columns_line_names, 2)
        || !grid.rows_line_names.is_empty()
    {
        return Err(unsupported(
            style.node_id,
            scene.scene_id,
            "computed root style changed",
        ));
    }
    Ok(())
}

pub(super) fn validate_item_style(
    style: &ComputedElementStyle,
    scene: &FitContentArgumentScene,
) -> Result<(), RuntimeLayoutFailure> {
    if style.properties.get("display").map(String::as_str) != Some("block")
        || style.properties.get("min-height").map(String::as_str) != Some("0px")
        || style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::LengthPx(0.0)
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_overflow_x != Some(spinon_style::ComputedCssOverflow::Visible)
        || style.layout_overflow_y != Some(spinon_style::ComputedCssOverflow::Visible)
        || !zero_edges(style.layout_spacing.padding)
        || !zero_edges(style.layout_spacing.margin)
        || style.layout_border != spinon_style::ComputedLayoutBorder::default()
        || !matches!(
            style.grid_item_placement.as_ref(),
            Some(placement)
                if placement.row_start == spinon_style::ComputedCssGridPlacement::Auto
                    && placement.row_end == spinon_style::ComputedCssGridPlacement::Auto
                    && placement.column_start == spinon_style::ComputedCssGridPlacement::Auto
                    && placement.column_end == spinon_style::ComputedCssGridPlacement::Auto
        )
    {
        return Err(unsupported(
            style.node_id,
            scene.scene_id,
            "grid item style changed",
        ));
    }
    Ok(())
}

pub(super) fn validate_child_style(
    style: &ComputedElementStyle,
    scene: &FitContentArgumentScene,
) -> Result<(), RuntimeLayoutFailure> {
    if style.properties.get("display").map(String::as_str) != Some("block")
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(30.0)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(400.0)
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_edges(style.layout_spacing.padding)
        || !zero_edges(style.layout_spacing.margin)
        || style.layout_border != spinon_style::ComputedLayoutBorder::default()
    {
        return Err(unsupported(
            style.node_id,
            scene.scene_id,
            "nested child style changed",
        ));
    }
    Ok(())
}

pub(super) fn validate_grid_input(
    root: NodeId,
    input: &GridContainerInput,
    scene: &FitContentArgumentScene,
) -> Result<(), RuntimeLayoutFailure> {
    let expected_auto_rows = match scene.scene_id {
        "fit-content-percentage-row-wpt" => {
            Some(GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
                GridTrackLengthPercentage::Percentage(0.5),
            )]))
        }
        "fit-content-calc-percentage-row-wpt" => None,
        _ => return Err(failure("unknown fit-content Grid input case")),
    };
    let auto_rows_match = expected_auto_rows
        .as_ref()
        .is_some_and(|expected| input.auto_rows == *expected)
        || (scene.scene_id == "fit-content-calc-percentage-row-wpt"
            && matches!(
                &input.auto_rows,
                GridAutoRows::TrackList(tracks)
                    if matches!(tracks.as_slice(), [GridTrackSize::FitContent(
                        GridTrackLengthPercentage::Calc(_)
                    )])
            ));
    if input.columns != [GridTrackSize::LengthPx(60.0)]
        || !input.rows.is_empty()
        || !auto_rows_match
        || input.auto_flow != GridAutoFlow::Row
        || input.placement_mode != GridPlacementMode::TrackSizing
        || input.columns_auto_repeat.is_some()
        || input.rows_auto_repeat.is_some()
        || input.template_areas.is_some()
        || !empty_line_name_groups(&input.columns_line_names, 2)
        || !input.rows_line_names.is_empty()
    {
        return Err(unsupported(
            root,
            scene.scene_id,
            "typed LayoutInput Grid container changed",
        ));
    }
    Ok(())
}

pub(super) fn validate_layout_math(
    root: NodeId,
    input: &GridContainerInput,
    css_math: &[LayoutCssMathValue],
    scene: &FitContentArgumentScene,
) -> Result<(), RuntimeLayoutFailure> {
    let matches = match scene.scene_id {
        "fit-content-percentage-row-wpt" => css_math.is_empty(),
        "fit-content-calc-percentage-row-wpt" => {
            let Some(id) = fit_content_calc_id(input) else {
                return Err(unsupported(
                    root,
                    scene.scene_id,
                    "fit-content calc handle is missing",
                ));
            };
            css_math
                == [LayoutCssMathValue {
                    id,
                    node_id: root,
                    property: LayoutCssMathProperty::GridTemplateRows,
                    expression: LayoutCssMath::Sum(vec![
                        LayoutCssMath::Percentage(0.1),
                        LayoutCssMath::LengthPx(5.0),
                    ]),
                }]
        }
        _ => return Err(failure("unknown fit-content calc registry case")),
    };
    if !matches {
        return Err(unsupported(
            root,
            scene.scene_id,
            "calc handle, owner, property, or expression changed",
        ));
    }
    Ok(())
}

fn fit_content_calc_id(input: &GridContainerInput) -> Option<LayoutCalcId> {
    let GridAutoRows::TrackList(tracks) = &input.auto_rows else {
        return None;
    };
    let [GridTrackSize::FitContent(GridTrackLengthPercentage::Calc(id))] = tracks.as_slice() else {
        return None;
    };
    Some(*id)
}

fn empty_line_name_groups(groups: &[Vec<String>], expected_count: usize) -> bool {
    groups.len() == expected_count && groups.iter().all(Vec::is_empty)
}
