use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent};
use spinon_layout::{ParagraphStyle, TextLayoutEngine, WhiteSpaceMode, WordBreakMode};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridPlacement, ComputedCssGridTrackBreadth,
    ComputedCssGridTrackSize, ComputedCssMath, ComputedCssMaxSize, ComputedCssSpacingValue,
    ComputedElementStyle, ComputedLayoutBorder, ComputedStyleProfile, ComputedStyleSnapshot,
};

use super::super::C11_4TextInput;
use super::super::c11_4_nested_grid_depth_two::{
    children, element_identity, has_empty_attribute, style_for,
};
use super::{AHEM, RuntimeLayoutFailure, layout_failure};

#[derive(Clone, Copy)]
enum Width {
    MinContent,
    MaxContent,
    Px(f32),
}

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    width: Width,
}

const SCENES: [Scene; 4] = [
    Scene {
        id: "cyclic-minmax-calc-two-axis-span-text-min-content",
        width: Width::MinContent,
    },
    Scene {
        id: "cyclic-minmax-calc-two-axis-span-text-max-content",
        width: Width::MaxContent,
    },
    Scene {
        id: "definite-minmax-calc-two-axis-span-text-min-content",
        width: Width::Px(10.0),
    },
    Scene {
        id: "definite-minmax-calc-two-axis-span-text-max-content",
        width: Width::Px(60.0),
    },
];

const TEXT: &str = "aaaaaa";

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
        return Err(unsupported(
            root,
            "exactly one direct spanning text item required",
        ));
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
    if text != TEXT || snapshot.parent(text_node) != Some(HostParent::Node(item)) {
        return Err(unsupported(text_node, "Ahem text content or owner differs"));
    }

    validate_grid_style(style_for(styles, root.id())?, scene)?;
    let item_style = style_for(styles, item.id())?;
    validate_item_style(item_style, scene.id)?;

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).map_err(|error| {
        layout_failure(
            "c114_two_axis_span_text_font_registration",
            Some(item.id().get()),
            Some(error.to_string()),
        )
    })?;
    Ok(Some(C11_4TextInput {
        paragraph_styles: BTreeMap::from([(
            item.id(),
            ParagraphStyle {
                font_family: "Ahem".to_owned(),
                font_size: 10.0,
                line_height: 10.0,
                white_space: WhiteSpaceMode::Normal,
                word_break: WordBreakMode::BreakAll,
                ..ParagraphStyle::default()
            },
        )]),
        engine,
        two_axis_nested_grid: false,
    }))
}

fn validate_grid_style(
    style: &ComputedElementStyle,
    scene: Scene,
) -> Result<(), RuntimeLayoutFailure> {
    let expected_width = match scene.width {
        Width::MinContent => ComputedCssDimension::MinContent,
        Width::MaxContent => ComputedCssDimension::MaxContent,
        Width::Px(width) => ComputedCssDimension::LengthPx(width),
    };
    let expected_border = ComputedLayoutBorder {
        top: 2.0,
        right: 2.0,
        bottom: 2.0,
        left: 2.0,
    };
    let invalid_box = style.properties.get("display").map(String::as_str) != Some("grid")
        || style.properties.get("box-sizing").map(String::as_str) != Some("content-box")
        || style.properties.get("align-content").map(String::as_str) != Some("start")
        || style.properties.get("justify-content").map(String::as_str) != Some("start")
        || style.properties.get("font-family").map(String::as_str) != Some("Ahem")
        || style.properties.get("font-size").map(String::as_str) != Some("10px")
        || style.properties.get("line-height").map(String::as_str) != Some("10px")
        || style.layout_dimensions.width != expected_width
        || style.layout_dimensions.height != ComputedCssDimension::MaxContent
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || !spacing_matches(style.layout_spacing.column_gap, 2.0)
        || !spacing_matches(style.layout_spacing.row_gap, 2.0)
        || style.layout_border != expected_border;
    if invalid_box {
        return Err(unsupported_node(
            style.node_id.get(),
            "root Grid box differs",
        ));
    }

    let grid = style
        .grid_layout
        .as_ref()
        .ok_or_else(|| unsupported_node(style.node_id.get(), "computed Grid tracks are missing"))?;
    if !two_tracks_match(&grid.columns)
        || !two_tracks_match(&grid.rows)
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
            "typed two-axis tracks differ",
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
        ("word-break", "break-all"),
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
            return Err(unsupported_node(
                style.node_id.get(),
                &format!("{scene_id}: {name} expected {expected}"),
            ));
        }
    }
    let placement_matches = style.grid_item_placement.as_ref().is_some_and(|placement| {
        placement.column_start == ComputedCssGridPlacement::Line(1)
            && placement.column_end == ComputedCssGridPlacement::Line(3)
            && placement.row_start == ComputedCssGridPlacement::Line(1)
            && placement.row_end == ComputedCssGridPlacement::Line(3)
    });
    if !placement_matches
        || (style.font_size_css_px - 10.0).abs() > f32::EPSILON
        || style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || style.layout_border != Default::default()
    {
        return Err(unsupported_node(
            style.node_id.get(),
            &format!("{scene_id}: spanning text item style differs"),
        ));
    }
    Ok(())
}

fn two_tracks_match(actual: &[ComputedCssGridTrackSize]) -> bool {
    actual.len() == 2
        && actual.iter().all(|track| {
            matches!(
                track,
                ComputedCssGridTrackSize::MinMax {
                    min: ComputedCssGridTrackBreadth::Calc(ComputedCssMath::Sum(terms)),
                    max: ComputedCssGridTrackBreadth::MaxContent,
                } if terms.as_slice() == [
                    ComputedCssMath::Percentage(0.1),
                    ComputedCssMath::LengthPx(2.0),
                ]
            )
        })
}

fn dimensionless_spacing(value: ComputedCssSpacingValue) -> bool {
    matches!(value, ComputedCssSpacingValue::LengthPx(value) if value.abs() <= f32::EPSILON)
}

fn zero_edges(edges: spinon_style::ComputedCssEdges) -> bool {
    [edges.top, edges.right, edges.bottom, edges.left]
        .into_iter()
        .all(dimensionless_spacing)
}

fn spacing_matches(value: ComputedCssSpacingValue, expected: f32) -> bool {
    matches!(value, ComputedCssSpacingValue::LengthPx(value) if (value - expected).abs() <= f32::EPSILON)
}

fn unsupported(node: HostNodeHandle, detail: &str) -> RuntimeLayoutFailure {
    unsupported_node(node.id().get(), detail)
}

fn unsupported_node(node: u64, detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_two_axis_span_text_fixture",
        Some(node),
        Some(detail.to_owned()),
    )
}

#[cfg(test)]
mod tests {
    use super::SCENES;
    use std::collections::BTreeSet;

    #[test]
    fn exposes_only_the_four_two_axis_span_text_scenes() {
        let ids = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 4);
        assert_eq!(ids.len(), SCENES.len());
        assert!(ids.contains("cyclic-minmax-calc-two-axis-span-text-min-content"));
        assert!(ids.contains("cyclic-minmax-calc-two-axis-span-text-max-content"));
        assert!(ids.contains("definite-minmax-calc-two-axis-span-text-min-content"));
        assert!(ids.contains("definite-minmax-calc-two-axis-span-text-max-content"));
    }
}
