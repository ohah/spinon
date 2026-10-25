use spinon_core::{HostNodeHandle, TextOrientation, WritingMode};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackSize,
    ComputedCssMaxSize, ComputedCssPosition, ComputedCssSpacingValue, ComputedElementStyle,
    ComputedLayoutBorder, ComputedStyleSnapshot,
};

use super::super::super::super::RuntimeLayoutFailure;
use super::helpers::{fail, unsupported};
use super::scene::{Item, ItemKind, Scene};

pub(super) fn validate_grid_style(
    style: &ComputedElementStyle,
    styles: &ComputedStyleSnapshot,
    root: HostNodeHandle,
    scene: Scene,
) -> Result<(), RuntimeLayoutFailure> {
    for (name, expected) in [
        ("display", "grid"),
        ("box-sizing", "border-box"),
        ("direction", "ltr"),
        ("writing-mode", "horizontal-tb"),
        ("text-orientation", "mixed"),
        ("align-items", "normal"),
        ("align-content", "start"),
        ("justify-items", "normal"),
        ("justify-content", "start"),
    ] {
        require_property(style, name, expected, scene)?;
    }
    if styles.writing_style_for(root.id())
        != Some((WritingMode::HorizontalTb, TextOrientation::Mixed))
    {
        return Err(unsupported(root.id(), "root writing style differs"));
    }
    let grid = style
        .grid_layout
        .as_ref()
        .ok_or_else(|| fail("c114_orthogonal_grid_layout_missing", root.id(), scene.id))?;
    if style.layout_dimensions.width != ComputedCssDimension::LengthPx(74.0)
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || style.layout_position != ComputedCssPosition::Static
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || !zero_spacing(style.layout_spacing.row_gap)
        || !zero_spacing(style.layout_spacing.column_gap)
        || style.layout_border != ComputedLayoutBorder::default()
        || grid.columns.len() != 2
        || grid.columns[0] != ComputedCssGridTrackSize::LengthPx(50.0)
        || grid.columns[1] != ComputedCssGridTrackSize::LengthPx(24.0)
        || grid.rows.len() != 1
        || grid.rows[0] != scene.track.style_value()
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || grid.auto_rows != ComputedCssGridAutoRows::Auto
        || grid.auto_columns != ComputedCssGridAutoColumns::Auto
        || grid.columns_line_names.len() != 3
        || grid.rows_line_names.len() != 2
        || grid
            .columns_line_names
            .iter()
            .chain(grid.rows_line_names.iter())
            .any(|names| !names.is_empty())
    {
        return Err(unsupported(
            root.id(),
            "computed Grid sizing or box style differs",
        ));
    }
    Ok(())
}

pub(super) fn validate_item_style(
    style: &ComputedElementStyle,
    styles: &ComputedStyleSnapshot,
    scene: Scene,
    item: Item,
) -> Result<(), RuntimeLayoutFailure> {
    for (name, expected) in [
        ("display", "block"),
        ("box-sizing", "border-box"),
        ("align-self", "auto"),
        ("justify-self", "auto"),
    ] {
        require_property(style, name, expected, scene)?;
    }
    let placement = style.grid_item_placement.as_ref().is_some_and(|placement| {
        placement.column_start == ComputedCssGridPlacement::Line(item.column)
            && placement.column_end == ComputedCssGridPlacement::Span(1)
            && placement.row_start == ComputedCssGridPlacement::Line(1)
            && placement.row_end == ComputedCssGridPlacement::Auto
    });
    if !placement
        || style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || style.layout_position != ComputedCssPosition::Static
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || style.layout_border != ComputedLayoutBorder::default()
        || style.layout_aspect_ratio.is_some()
    {
        return Err(unsupported(
            style.node_id,
            "item dimensions, edges, or placement differ",
        ));
    }
    match item.kind {
        ItemKind::Text => {
            if (style.font_size_css_px - 10.0).abs() > f32::EPSILON
                || styles.writing_style_for(style.node_id)
                    != Some((WritingMode::VerticalRl, TextOrientation::Mixed))
            {
                return Err(unsupported(style.node_id, "vertical Ahem style differs"));
            }
            for (name, expected) in [
                ("direction", "ltr"),
                ("writing-mode", "vertical-rl"),
                ("text-orientation", "mixed"),
                ("font-family", "Ahem"),
                ("font-size", "10px"),
                ("font-weight", "400"),
                ("font-style", "normal"),
                ("line-height", "10px"),
                ("white-space-collapse", "collapse"),
                ("text-wrap-mode", "wrap"),
                ("word-break", "normal"),
                ("line-break", "auto"),
                ("text-align", "start"),
                ("overflow-wrap", "normal"),
                ("text-transform", "none"),
                ("letter-spacing", "normal"),
                ("word-spacing", "0px"),
                ("text-indent", "0px"),
            ] {
                require_property(style, name, expected, scene)?;
            }
        }
        ItemKind::Image => {
            if styles.writing_style_for(style.node_id)
                != Some((WritingMode::HorizontalTb, TextOrientation::Mixed))
            {
                return Err(unsupported(
                    style.node_id,
                    "inherited image writing style differs",
                ));
            }
            for (name, expected) in [
                ("direction", "ltr"),
                ("writing-mode", "horizontal-tb"),
                ("text-orientation", "mixed"),
            ] {
                require_property(style, name, expected, scene)?;
            }
        }
    }
    Ok(())
}

fn require_property(
    style: &ComputedElementStyle,
    name: &str,
    expected: &str,
    scene: Scene,
) -> Result<(), RuntimeLayoutFailure> {
    if style.properties.get(name).map(String::as_str) == Some(expected) {
        Ok(())
    } else {
        Err(fail(
            "unsupported_c114_orthogonal_style",
            style.node_id,
            format!("{}: expected {name}:{expected}", scene.id),
        ))
    }
}

fn zero_edges(edges: spinon_style::ComputedCssEdges) -> bool {
    [edges.top, edges.right, edges.bottom, edges.left]
        .into_iter()
        .all(zero_spacing)
}

fn zero_spacing(value: ComputedCssSpacingValue) -> bool {
    matches!(value, ComputedCssSpacingValue::LengthPx(actual) if actual.abs() <= f32::EPSILON)
}
