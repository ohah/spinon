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
    Px(f32),
}

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    width: Width,
    reverse: bool,
}

const SCENES: [Scene; 4] = [
    Scene {
        id: "cyclic-minmax-calc-cross-axis-span-groups-intrinsic-forward",
        width: Width::MinContent,
        reverse: false,
    },
    Scene {
        id: "cyclic-minmax-calc-cross-axis-span-groups-intrinsic-reverse",
        width: Width::MinContent,
        reverse: true,
    },
    Scene {
        id: "definite-minmax-calc-cross-axis-span-groups-forward",
        width: Width::Px(40.0),
        reverse: false,
    },
    Scene {
        id: "definite-minmax-calc-cross-axis-span-groups-reverse",
        width: Width::Px(40.0),
        reverse: true,
    },
];

const ITEM_A_TEXT: &str = "AA A A";
const ITEM_B_TEXT: &str = "XXXX";

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

    validate_grid_style(style_for(styles, root.id())?, scene)?;
    let grid_children = children(snapshot, root)?;
    if grid_children.len() != 2 {
        return Err(unsupported(
            root,
            "exactly two direct spanning text items required",
        ));
    }
    let expected_order = if scene.reverse {
        ["b", "a"]
    } else {
        ["a", "b"]
    };
    let mut paragraph_styles = BTreeMap::new();
    for (item, expected_key) in grid_children.into_iter().zip(expected_order) {
        let (item_tag, item_id) = element_identity(snapshot, item)?;
        if item_tag != "div"
            || item_id != format!("c114-{}-item-{expected_key}", scene.id)
            || snapshot.parent(item) != Some(HostParent::Node(root))
            || !has_empty_attribute(snapshot, item, "data-node")
        {
            return Err(unsupported(
                item,
                "direct Grid item identity, order or owner differs",
            ));
        }
        let (text, style_key) = match expected_key {
            "a" => (ITEM_A_TEXT, "a"),
            "b" => (ITEM_B_TEXT, "b"),
            _ => unreachable!("fixed item order"),
        };
        validate_item_text(snapshot, item, text)?;
        validate_item_style(style_for(styles, item.id())?, scene.id, style_key)?;
        paragraph_styles.insert(
            item.id(),
            ParagraphStyle {
                overflow_wrap: Default::default(),
                writing_mode: spinon_core::WritingMode::HorizontalTb,
                text_orientation: spinon_core::TextOrientation::Mixed,
                font_family: "Ahem".to_owned(),
                font_size: 10.0,
                line_height: 10.0,
                white_space: WhiteSpaceMode::Normal,
                word_break: WordBreakMode::Normal,
                ..ParagraphStyle::default()
            },
        );
    }

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).map_err(|error| {
        layout_failure(
            "c114_cross_axis_span_groups_font_registration",
            Some(root.id().get()),
            Some(error.to_string()),
        )
    })?;
    Ok(Some(C11_4TextInput {
        paragraph_styles,
        engine,
        two_axis_nested_grid: false,
        content_box_ratio_node: None,
        aspect_ratio_minmax_node: None,
        definite_axis_ratio_node: None,
    }))
}

fn validate_item_text(
    snapshot: &HostDocumentSnapshot,
    item: HostNodeHandle,
    expected: &str,
) -> Result<(), RuntimeLayoutFailure> {
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
    if text != expected || snapshot.parent(text_node) != Some(HostParent::Node(item)) {
        return Err(unsupported(text_node, "Ahem text content or owner differs"));
    }
    Ok(())
}

fn validate_grid_style(
    style: &ComputedElementStyle,
    scene: Scene,
) -> Result<(), RuntimeLayoutFailure> {
    let expected_width = match scene.width {
        Width::MinContent => ComputedCssDimension::MinContent,
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
        || style.properties.get("font-weight").map(String::as_str) != Some("400")
        || style.properties.get("font-style").map(String::as_str) != Some("normal")
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
    if !tracks_match(&grid.columns, 3)
        || !tracks_match(&grid.rows, 2)
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
            "typed three-column/two-row tracks differ",
        ));
    }
    Ok(())
}

fn validate_item_style(
    style: &ComputedElementStyle,
    scene_id: &str,
    item: &str,
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
            return Err(unsupported_node(
                style.node_id.get(),
                &format!("{scene_id} item {item}: {name} expected {expected}"),
            ));
        }
    }
    let (column_start, column_end, width, height) = if item == "a" {
        (1, 3, ComputedCssDimension::Auto, ComputedCssDimension::Auto)
    } else {
        (
            2,
            4,
            ComputedCssDimension::LengthPx(40.0),
            ComputedCssDimension::LengthPx(10.0),
        )
    };
    let placement_matches = style.grid_item_placement.as_ref().is_some_and(|placement| {
        placement.column_start == ComputedCssGridPlacement::Line(column_start)
            && placement.column_end == ComputedCssGridPlacement::Line(column_end)
            && placement.row_start == ComputedCssGridPlacement::Line(1)
            && placement.row_end == ComputedCssGridPlacement::Line(3)
    });
    if !placement_matches
        || (style.font_size_css_px - 10.0).abs() > f32::EPSILON
        || style.layout_dimensions.width != width
        || style.layout_dimensions.height != height
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
            &format!("{scene_id} item {item}: spanning text style differs"),
        ));
    }
    Ok(())
}

fn tracks_match(actual: &[ComputedCssGridTrackSize], expected_len: usize) -> bool {
    actual.len() == expected_len
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
        "unsupported_c114_cross_axis_span_groups_fixture",
        Some(node),
        Some(detail.to_owned()),
    )
}

#[cfg(test)]
mod tests {
    use super::SCENES;
    use std::collections::BTreeSet;

    #[test]
    fn exposes_only_four_fixed_cross_axis_span_group_scenes() {
        let ids = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 4);
        assert_eq!(ids.len(), SCENES.len());
        assert!(ids.contains("cyclic-minmax-calc-cross-axis-span-groups-intrinsic-forward"));
        assert!(ids.contains("cyclic-minmax-calc-cross-axis-span-groups-intrinsic-reverse"));
        assert!(ids.contains("definite-minmax-calc-cross-axis-span-groups-forward"));
        assert!(ids.contains("definite-minmax-calc-cross-axis-span-groups-reverse"));
    }
}
