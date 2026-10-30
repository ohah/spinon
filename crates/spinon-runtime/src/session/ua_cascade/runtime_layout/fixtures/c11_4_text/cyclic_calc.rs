use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent};
use spinon_layout::{ParagraphStyle, TextLayoutEngine, WhiteSpaceMode};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridPlacement, ComputedCssGridTrackSize, ComputedCssMath,
    ComputedCssMaxSize, ComputedCssSpacingValue, ComputedElementStyle, ComputedStyleProfile,
    ComputedStyleSnapshot,
};

use super::super::C11_4TextInput;
use super::super::c11_4_nested_grid_depth_two::{
    children, element_identity, has_empty_attribute, style_for,
};
use super::{AHEM, RuntimeLayoutFailure, layout_failure};

#[derive(Clone, Copy)]
enum Dimension {
    MaxContent,
    Px(f32),
}

#[derive(Clone, Copy)]
enum Track {
    Calc { percentage: f32, length_px: f32 },
    Px(f32),
}

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    width: Dimension,
    height: Dimension,
    columns: Track,
    rows: Track,
}

const CALC_COLUMN: Track = Track::Calc {
    percentage: 0.5,
    length_px: 4.0,
};
const CALC_ROW: Track = Track::Calc {
    percentage: 0.5,
    length_px: 2.0,
};

const SCENES: [Scene; 4] = [
    Scene {
        id: "cyclic-calc-column",
        width: Dimension::MaxContent,
        height: Dimension::MaxContent,
        columns: CALC_COLUMN,
        rows: Track::Px(16.0),
    },
    Scene {
        id: "cyclic-calc-row",
        width: Dimension::Px(24.0),
        height: Dimension::MaxContent,
        columns: Track::Px(24.0),
        rows: CALC_ROW,
    },
    Scene {
        id: "cyclic-calc-two-axis",
        width: Dimension::MaxContent,
        height: Dimension::MaxContent,
        columns: CALC_COLUMN,
        rows: CALC_ROW,
    },
    Scene {
        id: "definite-calc-control",
        width: Dimension::Px(40.0),
        height: Dimension::Px(20.0),
        columns: CALC_COLUMN,
        rows: CALC_ROW,
    },
];

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
        if root_id.starts_with("c114-cyclic-calc-") {
            return Err(unsupported(root, "unregistered cyclic calc scene"));
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
    if text != "XX" || snapshot.parent(text_node) != Some(HostParent::Node(item)) {
        return Err(unsupported(text_node, "Ahem text content or owner differs"));
    }
    validate_grid_style(style_for(styles, root.id())?, scene)?;
    let item_style = style_for(styles, item.id())?;
    validate_item_style(item_style, scene.id)?;
    validate_placement(item_style, scene.id)?;

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).map_err(|error| {
        layout_failure(
            "c114_cyclic_calc_font_registration",
            Some(item.id().get()),
            Some(error.to_string()),
        )
    })?;
    Ok(Some(C11_4TextInput {
        paragraph_styles: BTreeMap::from([(item.id(), ahem_paragraph_style())]),
        engine,
        two_axis_nested_grid: false,
        content_box_ratio_node: None,
        aspect_ratio_minmax_node: None,
        definite_axis_ratio_node: None,
    }))
}

fn validate_grid_style(
    style: &ComputedElementStyle,
    scene: Scene,
) -> Result<(), RuntimeLayoutFailure> {
    if style.properties.get("display").map(String::as_str) != Some("grid")
        || style.properties.get("box-sizing").map(String::as_str) != Some("border-box")
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
        || !zero_spacing(style.layout_spacing.row_gap)
        || !zero_spacing(style.layout_spacing.column_gap)
        || style.layout_border != Default::default()
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
    if !tracks_match(&grid.columns, scene.columns)
        || !tracks_match(&grid.rows, scene.rows)
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
            "typed calc track matrix differs",
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
        ("font-size", "12px"),
        ("font-weight", "400"),
        ("font-style", "normal"),
        ("line-height", "16px"),
        ("white-space-collapse", "collapse"),
        ("text-wrap-mode", "wrap"),
        ("word-break", "normal"),
        ("direction", "ltr"),
        ("line-break", "auto"),
        ("text-align", "start"),
        ("overflow-wrap", "normal"),
        ("text-transform", "none"),
        ("letter-spacing", "normal"),
        ("word-spacing", "0px"),
        ("text-indent", "0px"),
    ] {
        if style.properties.get(name).map(String::as_str) != Some(expected) {
            return Err(layout_failure(
                "unsupported_c114_cyclic_calc_style",
                Some(style.node_id.get()),
                Some(format!("{scene_id}: {name} expected {expected}")),
            ));
        }
    }
    if (style.font_size_css_px - 12.0).abs() > f32::EPSILON
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(24.0)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(16.0)
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
            "fixed Ahem item style differs",
        ));
    }
    Ok(())
}

fn validate_placement(
    style: &ComputedElementStyle,
    scene_id: &str,
) -> Result<(), RuntimeLayoutFailure> {
    let valid = style.grid_item_placement.as_ref().is_some_and(|placement| {
        placement.column_start == ComputedCssGridPlacement::Line(1)
            && placement.column_end == ComputedCssGridPlacement::Auto
            && placement.row_start == ComputedCssGridPlacement::Line(1)
            && placement.row_end == ComputedCssGridPlacement::Auto
    });
    if valid {
        Ok(())
    } else {
        Err(layout_failure(
            "unsupported_c114_cyclic_calc_placement",
            Some(style.node_id.get()),
            Some(scene_id.to_owned()),
        ))
    }
}

fn tracks_match(actual: &[ComputedCssGridTrackSize], expected: Track) -> bool {
    actual.len() == 1
        && match (actual.first(), expected) {
            (
                Some(ComputedCssGridTrackSize::Calc(ComputedCssMath::Sum(terms))),
                Track::Calc {
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
            (Some(ComputedCssGridTrackSize::LengthPx(actual)), Track::Px(expected)) => {
                (*actual - expected).abs() <= f32::EPSILON
            }
            _ => false,
        }
}

fn dimension_matches(actual: ComputedCssDimension, expected: Dimension) -> bool {
    match (actual, expected) {
        (ComputedCssDimension::MaxContent, Dimension::MaxContent) => true,
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
    matches!(value, ComputedCssSpacingValue::LengthPx(actual) if actual.abs() <= f32::EPSILON)
}

fn ahem_paragraph_style() -> ParagraphStyle {
    ParagraphStyle {
        writing_mode: spinon_core::WritingMode::HorizontalTb,
        text_orientation: spinon_core::TextOrientation::Mixed,
        font_family: "Ahem".to_owned(),
        font_size: 12.0,
        line_height: 16.0,
        line_height_is_normal: false,
        white_space: WhiteSpaceMode::Normal,
        ..ParagraphStyle::default()
    }
}

fn unsupported(node: HostNodeHandle, detail: &str) -> RuntimeLayoutFailure {
    unsupported_node(node.id().get(), detail)
}

fn unsupported_node(node: u64, detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_cyclic_calc_fixture",
        Some(node),
        Some(detail.to_owned()),
    )
}

#[cfg(test)]
mod tests {
    use super::SCENES;
    use std::collections::BTreeSet;

    #[test]
    fn exposes_only_four_fixed_cyclic_calc_scenes() {
        let scene_ids = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 4);
        assert_eq!(scene_ids.len(), SCENES.len());
        assert!(scene_ids.contains("cyclic-calc-column"));
        assert!(scene_ids.contains("cyclic-calc-row"));
        assert!(scene_ids.contains("cyclic-calc-two-axis"));
        assert!(scene_ids.contains("definite-calc-control"));
    }
}
