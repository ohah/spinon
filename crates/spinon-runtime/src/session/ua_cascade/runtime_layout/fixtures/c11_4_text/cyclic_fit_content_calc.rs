use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent};
use spinon_layout::{ParagraphStyle, TextLayoutEngine, WhiteSpaceMode};
use spinon_render::RuntimeSrgbColor;
use spinon_style::{
    ComputedCssDimension, ComputedCssGridPlacement, ComputedCssGridTrackBreadth,
    ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize, ComputedCssMath,
    ComputedCssMaxSize, ComputedCssSpacingValue, ComputedElementStyle, ComputedLayoutBorder,
    ComputedStyleProfile, ComputedStyleSnapshot,
};

use super::super::C11_4TextInput;
use super::super::c11_4_nested_grid_depth_two::{
    children, element_identity, has_empty_attribute, style_for,
};
use super::{AHEM, RuntimeLayoutFailure, layout_failure};

#[path = "cyclic_fit_content_calc/scene.rs"]
mod scene;
use scene::{BoxSizing, Dimension, SCENES, Scene, Track};

pub(super) fn is_scene_root(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
) -> Result<bool, RuntimeLayoutFailure> {
    let node = snapshot
        .node(root)
        .ok_or_else(|| layout_failure("missing_node", Some(root.id().get()), None))?;
    let HostNodeKind::Element(element) = node.kind() else {
        return Ok(false);
    };
    if element.namespace() != "http://www.w3.org/1999/xhtml" {
        return Ok(false);
    }
    let root_id = element
        .attributes()
        .iter()
        .find_map(|(name, value)| {
            (name.namespace().is_none() && name.local_name() == "id")
                .then(|| value.to_string_lossy())
        })
        .unwrap_or_default();
    Ok(SCENES
        .iter()
        .any(|scene| root_id == format!("c114-{}-grid", scene.id)))
}

pub(super) fn prepare_render_input(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    owner: spinon_core::NodeId,
    engine: &TextLayoutEngine,
) -> Result<super::super::super::root_html_default_render::Input, RuntimeLayoutFailure> {
    if !is_scene_root(snapshot, root)? {
        return Err(unsupported(
            root,
            "not an allowlisted cyclic calc track scene",
        ));
    }
    let owner_style = style_for(styles, owner)?;
    if owner_style.properties.get("color").map(String::as_str) != Some("rgb(229, 72, 77)") {
        return Err(layout_failure(
            "unsupported_c114_cyclic_calc_track_text_color",
            Some(owner.get()),
            None,
        ));
    }
    let color =
        RuntimeSrgbColor::new(229.0 / 255.0, 72.0 / 255.0, 77.0 / 255.0, 1.0).map_err(|error| {
            layout_failure(
                "c114_cyclic_calc_track_text_color_invalid",
                Some(owner.get()),
                Some(error.to_string()),
            )
        })?;
    super::super::super::root_html_default_render::prepare_fixture_text_owner(
        snapshot, root, styles, owner, engine, color,
    )
}

pub(super) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<C11_4TextInput>, RuntimeLayoutFailure> {
    let (root_tag, root_id) = element_identity(snapshot, root)?;
    let Some(scene) = SCENES
        .iter()
        .find(|scene| root_id == format!("c114-{}-grid", scene.id))
        .copied()
    else {
        if root_id.starts_with("c114-cyclic-fit-content-calc-")
            || root_id.starts_with("c114-definite-fit-content-calc-")
            || root_id.starts_with("c114-cyclic-minmax-calc-")
            || root_id.starts_with("c114-definite-minmax-calc-")
            || root_id.starts_with("c114-definite-column-cyclic-minmax-calc-")
        {
            return Err(unsupported(root, "unregistered cyclic calc track scene"));
        }
        return Ok(None);
    };
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || root_tag != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
        || !has_empty_attribute(snapshot, root, "data-node")
    {
        return Err(unsupported(
            root,
            "root profile, identity, owner or node marker",
        ));
    }
    let grid_children = children(snapshot, root)?;
    if grid_children.len() != 1 {
        return Err(unsupported(root, "exactly one direct text item required"));
    }
    let item = grid_children[0];
    let (item_tag, item_id) = element_identity(snapshot, item)?;
    if item_tag != "div"
        || item_id != format!("c114-{}-item", scene.id)
        || snapshot.parent(item) != Some(HostParent::Node(root))
        || !has_empty_attribute(snapshot, item, "data-node")
    {
        return Err(unsupported(
            item,
            "direct Grid item identity or owner differs",
        ));
    }
    let text_children = children(snapshot, item)?;
    if text_children.len() != 1 {
        return Err(unsupported(item, "one direct Ahem text node required"));
    }
    let text_node = text_children[0];
    let text = snapshot
        .node(text_node)
        .and_then(|node| match node.kind() {
            HostNodeKind::Text(text) => Some(text.to_string_lossy()),
            HostNodeKind::Element(_) => None,
        })
        .ok_or_else(|| unsupported(text_node, "direct text node required"))?;
    if text != scene.text || snapshot.parent(text_node) != Some(HostParent::Node(item)) {
        return Err(unsupported(text_node, "Ahem text content or owner differs"));
    }

    validate_grid_style(style_for(styles, root.id())?, scene)?;
    validate_item_style(style_for(styles, item.id())?, scene.id)?;

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).map_err(|error| {
        layout_failure(
            "c114_cyclic_calc_track_font_registration",
            Some(item.id().get()),
            Some(error.to_string()),
        )
    })?;
    Ok(Some(C11_4TextInput {
        paragraph_styles: BTreeMap::from([(item.id(), ahem_paragraph_style())]),
        engine,
        two_axis_nested_grid: false,
        content_box_ratio_node: None,
    }))
}

fn validate_grid_style(
    style: &ComputedElementStyle,
    scene: Scene,
) -> Result<(), RuntimeLayoutFailure> {
    let expected_box_sizing = match scene.box_sizing {
        BoxSizing::ContentBox => "content-box",
        BoxSizing::BorderBox => "border-box",
    };
    let expected_border = ComputedLayoutBorder {
        top: scene.border_width,
        right: scene.border_width,
        bottom: scene.border_width,
        left: scene.border_width,
    };
    let expected_column_gap = track_gap(scene.columns);
    let expected_row_gap = track_gap(scene.rows);
    if style.properties.get("display").map(String::as_str) != Some("grid")
        || style.properties.get("box-sizing").map(String::as_str) != Some(expected_box_sizing)
        || style.properties.get("align-content").map(String::as_str) != Some("start")
        || style.properties.get("justify-content").map(String::as_str) != Some("start")
        || !dimension_matches(style.layout_dimensions.width, scene.width)
        || !dimension_matches(style.layout_dimensions.height, scene.height)
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || !spacing_matches(style.layout_spacing.row_gap, expected_row_gap)
        || !spacing_matches(style.layout_spacing.column_gap, expected_column_gap)
        || style.layout_border != expected_border
    {
        return Err(unsupported_node(
            style.node_id.get(),
            "root Grid box differs",
        ));
    }
    let grid = style
        .grid_layout
        .as_ref()
        .ok_or_else(|| unsupported_node(style.node_id.get(), "computed Grid tracks are missing"))?;
    if !track_matches(grid.columns.as_slice(), scene.columns)
        || !track_matches(grid.rows.as_slice(), scene.rows)
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || grid
            .columns_line_names
            .iter()
            .any(|names| !names.is_empty())
        || grid.rows_line_names.iter().any(|names| !names.is_empty())
    {
        return Err(unsupported_node(
            style.node_id.get(),
            "typed cyclic calc track matrix differs",
        ));
    }
    Ok(())
}

fn validate_item_style(
    style: &ComputedElementStyle,
    scene_id: &str,
) -> Result<(), RuntimeLayoutFailure> {
    for (name, expected) in [
        ("display", "block"),
        ("box-sizing", "border-box"),
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
        ("color", "rgb(229, 72, 77)"),
        ("writing-mode", "horizontal-tb"),
        ("letter-spacing", "normal"),
        ("word-spacing", "0px"),
        ("text-indent", "0px"),
    ] {
        if style.properties.get(name).map(String::as_str) != Some(expected) {
            return Err(layout_failure(
                "unsupported_c114_cyclic_calc_track_style",
                Some(style.node_id.get()),
                Some(format!("{scene_id}: {name} expected {expected}")),
            ));
        }
    }
    let placement_matches = match scene_id {
        "cyclic-minmax-calc-min-span-columns" | "definite-minmax-calc-min-span-columns" => {
            style.grid_item_placement.as_ref().is_some_and(|placement| {
                placement.column_start == ComputedCssGridPlacement::Line(1)
                    && placement.column_end == ComputedCssGridPlacement::Span(2)
                    && placement.row_start == ComputedCssGridPlacement::Auto
                    && placement.row_end == ComputedCssGridPlacement::Auto
            })
        }
        "cyclic-minmax-calc-min-span-rows" | "definite-minmax-calc-min-span-rows" => {
            style.grid_item_placement.as_ref().is_some_and(|placement| {
                placement.row_start == ComputedCssGridPlacement::Line(1)
                    && placement.row_end == ComputedCssGridPlacement::Span(2)
                    && placement.column_start == ComputedCssGridPlacement::Auto
                    && placement.column_end == ComputedCssGridPlacement::Auto
            })
        }
        _ => true,
    };
    if !placement_matches {
        return Err(layout_failure(
            "unsupported_c114_cyclic_calc_placement",
            Some(style.node_id.get()),
            Some(scene_id.to_owned()),
        ));
    }
    if (style.font_size_css_px - 10.0).abs() > f32::EPSILON
        || style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_width != ComputedCssDimension::LengthPx(0.0)
        || style.layout_dimensions.min_height != ComputedCssDimension::LengthPx(0.0)
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || style.layout_border != Default::default()
    {
        return Err(unsupported_node(
            style.node_id.get(),
            "Ahem item box differs",
        ));
    }
    Ok(())
}

fn track_matches(actual: &[ComputedCssGridTrackSize], expected: Track) -> bool {
    if let Track::TwoMinMaxCalcMinimum {
        percentage,
        length_px,
        ..
    } = expected
    {
        return actual.len() == 2
            && actual
                .iter()
                .all(|track| minmax_calc_minimum_track_matches(track, percentage, length_px));
    }
    actual.len() == 1
        && match (actual.first(), expected) {
            (Some(ComputedCssGridTrackSize::Auto), Track::Auto) => true,
            (Some(ComputedCssGridTrackSize::LengthPx(actual)), Track::Px(expected)) => {
                (*actual - expected).abs() <= f32::EPSILON
            }
            (
                Some(ComputedCssGridTrackSize::FitContent(
                    ComputedCssGridTrackLengthPercentage::Calc(ComputedCssMath::Sum(terms)),
                )),
                Track::FitContentCalc {
                    percentage,
                    length_px,
                },
            ) => {
                terms.as_slice()
                    == [
                        ComputedCssMath::Percentage(percentage),
                        ComputedCssMath::LengthPx(length_px),
                    ]
            }
            (
                Some(ComputedCssGridTrackSize::MinMax {
                    min: ComputedCssGridTrackBreadth::LengthPx(min),
                    max: ComputedCssGridTrackBreadth::Calc(ComputedCssMath::Sum(terms)),
                }),
                Track::MinMaxCalc {
                    percentage,
                    length_px,
                },
            ) => {
                min.abs() <= f32::EPSILON
                    && terms.as_slice()
                        == [
                            ComputedCssMath::Percentage(percentage),
                            ComputedCssMath::LengthPx(length_px),
                        ]
            }
            (
                Some(track),
                Track::MinMaxCalcMinimum {
                    percentage,
                    length_px,
                },
            ) => minmax_calc_minimum_track_matches(track, percentage, length_px),
            _ => false,
        }
}

fn minmax_calc_minimum_track_matches(
    actual: &ComputedCssGridTrackSize,
    percentage: f32,
    length_px: f32,
) -> bool {
    matches!(
        actual,
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::Calc(ComputedCssMath::Sum(terms)),
            max: ComputedCssGridTrackBreadth::MaxContent,
        } if terms.as_slice()
            == [
                ComputedCssMath::Percentage(percentage),
                ComputedCssMath::LengthPx(length_px),
            ]
    )
}

fn track_gap(track: Track) -> f32 {
    match track {
        Track::TwoMinMaxCalcMinimum { gap_px, .. } => gap_px,
        _ => 0.0,
    }
}

fn dimension_matches(actual: ComputedCssDimension, expected: Dimension) -> bool {
    match (actual, expected) {
        (ComputedCssDimension::Auto, Dimension::Auto)
        | (ComputedCssDimension::MaxContent, Dimension::MaxContent) => true,
        (ComputedCssDimension::LengthPx(actual), Dimension::Px(expected)) => {
            (actual - expected).abs() <= f32::EPSILON
        }
        _ => false,
    }
}

fn zero_edges(edges: spinon_style::ComputedCssEdges) -> bool {
    [edges.top, edges.right, edges.bottom, edges.left]
        .into_iter()
        .all(zero_spacing)
}

fn zero_spacing(value: ComputedCssSpacingValue) -> bool {
    spacing_matches(value, 0.0)
}

fn spacing_matches(value: ComputedCssSpacingValue, expected: f32) -> bool {
    matches!(value, ComputedCssSpacingValue::LengthPx(actual) if (actual - expected).abs() <= f32::EPSILON)
}

fn ahem_paragraph_style() -> ParagraphStyle {
    ParagraphStyle {
        writing_mode: spinon_core::WritingMode::HorizontalTb,
        text_orientation: spinon_core::TextOrientation::Mixed,
        font_family: "Ahem".to_owned(),
        font_size: 10.0,
        line_height: 10.0,
        white_space: WhiteSpaceMode::Normal,
        ..ParagraphStyle::default()
    }
}

fn unsupported(node: HostNodeHandle, detail: &str) -> RuntimeLayoutFailure {
    unsupported_node(node.id().get(), detail)
}

fn unsupported_node(node: u64, detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_cyclic_calc_track_fixture",
        Some(node),
        Some(detail.to_owned()),
    )
}

#[cfg(test)]
mod tests {
    use super::SCENES;
    use std::collections::BTreeSet;

    #[test]
    fn exposes_fixed_cyclic_calc_track_scenes_by_function() {
        let scene_ids = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 20);
        assert_eq!(scene_ids.len(), SCENES.len());
        assert!(scene_ids.contains("cyclic-fit-content-calc-column"));
        assert!(scene_ids.contains("cyclic-fit-content-calc-row"));
        assert!(scene_ids.contains("definite-fit-content-calc-column"));
        assert!(scene_ids.contains("definite-fit-content-calc-small-limit"));
        assert!(scene_ids.contains("cyclic-minmax-calc-column"));
        assert!(scene_ids.contains("cyclic-minmax-calc-row"));
        assert!(scene_ids.contains("definite-minmax-calc-column"));
        assert!(scene_ids.contains("definite-minmax-calc-small-limit"));
        assert!(scene_ids.contains("cyclic-minmax-calc-min-column"));
        assert!(scene_ids.contains("cyclic-minmax-calc-min-row"));
        assert!(scene_ids.contains("definite-minmax-calc-min-column"));
        assert!(scene_ids.contains("definite-minmax-calc-min-row"));
        assert!(scene_ids.contains("cyclic-minmax-calc-min-both-axes"));
        assert!(scene_ids.contains("cyclic-minmax-calc-min-column-definite-row"));
        assert!(scene_ids.contains("definite-column-cyclic-minmax-calc-min-row"));
        assert!(scene_ids.contains("definite-minmax-calc-min-both-axes"));
        assert!(scene_ids.contains("cyclic-minmax-calc-min-span-columns"));
        assert!(scene_ids.contains("definite-minmax-calc-min-span-columns"));
        assert!(scene_ids.contains("cyclic-minmax-calc-min-span-rows"));
        assert!(scene_ids.contains("definite-minmax-calc-min-span-rows"));
    }
}
