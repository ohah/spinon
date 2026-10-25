use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackSize,
    ComputedCssMaxSize, ComputedCssPosition, ComputedCssSpacingValue, ComputedElementStyle,
};

use super::{
    Item, ItemKind, RuntimeLayoutFailure, Scene, SceneGrid, failure, layout_failure, unsupported,
};

pub(super) fn validate_grid_style(
    style: &ComputedElementStyle,
    scene: Scene,
) -> Result<(), RuntimeLayoutFailure> {
    for (name, expected) in [
        ("display", "grid"),
        ("box-sizing", "border-box"),
        ("align-items", "start"),
        ("align-content", "start"),
        ("justify-content", "start"),
    ] {
        require_property(style, name, expected)?;
    }
    if matches!(
        scene.grid,
        SceneGrid::DirectMinContentRow | SceneGrid::DirectMaxContentRow
    ) {
        require_property(style, "direction", "ltr")?;
        require_property(style, "writing-mode", "horizontal-tb")?;
        require_property(style, "justify-items", "normal")?;
    }
    let grid = style.grid_layout.as_ref().ok_or_else(|| {
        failure(
            "missing_c114_mixed_grid_layout",
            style.node_id,
            Some(scene.id.to_owned()),
        )
    })?;
    let (root_width, root_height, columns, rows) = match scene.grid {
        SceneGrid::SpanningMultiTrack => (
            ComputedCssDimension::LengthPx(220.0),
            ComputedCssDimension::LengthPx(180.0),
            vec![ComputedCssGridTrackSize::MinContent; 4],
            vec![ComputedCssGridTrackSize::LengthPx(60.0); 3],
        ),
        SceneGrid::DirectMinContent => (
            ComputedCssDimension::LengthPx(120.0),
            ComputedCssDimension::LengthPx(48.0),
            vec![ComputedCssGridTrackSize::MinContent],
            vec![ComputedCssGridTrackSize::LengthPx(24.0); 2],
        ),
        SceneGrid::DirectMaxContent => (
            ComputedCssDimension::LengthPx(120.0),
            ComputedCssDimension::LengthPx(48.0),
            vec![ComputedCssGridTrackSize::MaxContent],
            vec![ComputedCssGridTrackSize::LengthPx(24.0); 2],
        ),
        SceneGrid::DirectMinContentRow => (
            ComputedCssDimension::LengthPx(74.0),
            ComputedCssDimension::Auto,
            vec![
                ComputedCssGridTrackSize::LengthPx(50.0),
                ComputedCssGridTrackSize::LengthPx(24.0),
            ],
            vec![ComputedCssGridTrackSize::MinContent],
        ),
        SceneGrid::DirectMaxContentRow => (
            ComputedCssDimension::LengthPx(74.0),
            ComputedCssDimension::Auto,
            vec![
                ComputedCssGridTrackSize::LengthPx(50.0),
                ComputedCssGridTrackSize::LengthPx(24.0),
            ],
            vec![ComputedCssGridTrackSize::MaxContent],
        ),
    };
    if style.layout_dimensions.width != root_width
        || style.layout_dimensions.height != root_height
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
        || style.layout_border != Default::default()
        || grid.columns != columns
        || grid.rows != rows
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || grid.auto_rows != ComputedCssGridAutoRows::Auto
        || grid.auto_columns != ComputedCssGridAutoColumns::Auto
        || grid.columns_line_names.len() != columns.len() + 1
        || grid.rows_line_names.len() != rows.len() + 1
        || grid
            .columns_line_names
            .iter()
            .any(|names| !names.is_empty())
        || grid.rows_line_names.iter().any(|names| !names.is_empty())
    {
        return Err(unsupported(
            style.node_id,
            scene,
            "scene-specific fixed grid style required",
        ));
    }
    Ok(())
}

pub(super) fn validate_item_style(
    style: &ComputedElementStyle,
    scene: Scene,
    item: Item,
) -> Result<(), RuntimeLayoutFailure> {
    for (name, expected) in [("display", "block"), ("box-sizing", "border-box")] {
        require_property(style, name, expected)?;
    }
    let placement = style.grid_item_placement.as_ref().is_some_and(|placement| {
        placement.column_start == ComputedCssGridPlacement::Line(item.column_start)
            && placement.column_end == ComputedCssGridPlacement::Span(item.column_span)
            && placement.row_start == ComputedCssGridPlacement::Line(item.row)
            && placement.row_end == ComputedCssGridPlacement::Auto
    });
    let (justify_self, align_self) = match scene.grid {
        SceneGrid::SpanningMultiTrack
        | SceneGrid::DirectMinContent
        | SceneGrid::DirectMaxContent
        | SceneGrid::DirectMinContentRow
        | SceneGrid::DirectMaxContentRow => ("auto", "auto"),
    };
    if !placement
        || style.layout_position != ComputedCssPosition::Static
        || !property_is(style, "justify-self", justify_self)
        || !property_is(style, "align-self", align_self)
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || style.layout_border != Default::default()
    {
        return Err(unsupported(
            style.node_id,
            scene,
            "item sizing or placement differs",
        ));
    }
    match item.kind {
        ItemKind::Text(_) => {
            if style.layout_dimensions.width != ComputedCssDimension::Auto
                || style.layout_dimensions.height != ComputedCssDimension::Auto
                || (style.font_size_css_px - 10.0).abs() > f32::EPSILON
            {
                return Err(unsupported(
                    style.node_id,
                    scene,
                    "Ahem text box sizing differs",
                ));
            }
            for (name, expected) in [
                ("font-family", "Ahem"),
                ("font-size", "10px"),
                ("font-weight", "400"),
                ("font-style", "normal"),
                ("line-height", "10px"),
                ("white-space-collapse", "collapse"),
                ("text-wrap-mode", "wrap"),
                ("word-break", "normal"),
                ("direction", "ltr"),
                ("line-break", "auto"),
                ("text-align", "start"),
                ("overflow-wrap", "normal"),
                ("text-transform", "none"),
                ("writing-mode", "horizontal-tb"),
                ("letter-spacing", "normal"),
                ("word-spacing", "0px"),
                ("text-indent", "0px"),
            ] {
                require_property(style, name, expected)?;
            }
        }
        ItemKind::Image { height } => {
            let height_matches = match height {
                Some(height) => {
                    style.layout_dimensions.height == ComputedCssDimension::LengthPx(height)
                }
                None => style.layout_dimensions.height == ComputedCssDimension::Auto,
            };
            if style.layout_dimensions.width != ComputedCssDimension::Auto
                || !height_matches
                || style.layout_aspect_ratio.is_some()
            {
                return Err(unsupported(
                    style.node_id,
                    scene,
                    "image transfer sizing differs",
                ));
            }
        }
    }
    Ok(())
}

fn require_property(
    style: &ComputedElementStyle,
    name: &str,
    expected: &str,
) -> Result<(), RuntimeLayoutFailure> {
    if style.properties.get(name).map(String::as_str) == Some(expected) {
        Ok(())
    } else {
        Err(layout_failure(
            "unsupported_c114_mixed_style",
            Some(style.node_id.get()),
            Some(format!("{name}: expected {expected}")),
        ))
    }
}

fn property_is(style: &ComputedElementStyle, name: &str, expected: &str) -> bool {
    style.properties.get(name).map(String::as_str) == Some(expected)
}

fn zero_edges(edges: spinon_style::ComputedCssEdges) -> bool {
    [edges.top, edges.right, edges.bottom, edges.left]
        .into_iter()
        .all(zero_spacing)
}

fn zero_spacing(value: ComputedCssSpacingValue) -> bool {
    matches!(value, ComputedCssSpacingValue::LengthPx(actual) if actual.abs() <= f32::EPSILON)
}
