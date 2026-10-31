use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, NodeId};
use spinon_layout::{ParagraphStyle, TextLayoutEngine, WhiteSpaceMode, WordBreakMode};
use spinon_render::RuntimeSrgbColor;
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
enum Dimension {
    MaxContent,
    Px(f32),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Axis {
    Columns,
    Rows,
}

#[derive(Clone, Copy)]
struct Item {
    id: &'static str,
    text: &'static str,
    column_start: i32,
    column_end: i32,
    row_start: i32,
    row_end: i32,
    span: u16,
    break_all: bool,
}

const LEFT: Item = Item {
    id: "span2-left",
    text: "aaaaaaa",
    column_start: 1,
    column_end: 3,
    row_start: 1,
    row_end: 2,
    span: 2,
    break_all: false,
};
const RIGHT: Item = Item {
    id: "span2-right",
    text: "aaaaaaaa",
    column_start: 2,
    column_end: 4,
    row_start: 1,
    row_end: 2,
    span: 2,
    break_all: false,
};
const SPAN3: Item = Item {
    id: "span3",
    text: "aaaaaaaaaaaaaa",
    column_start: 1,
    column_end: 4,
    row_start: 1,
    row_end: 2,
    span: 3,
    break_all: false,
};

const LEFT_ROW: Item = Item {
    id: "span2-left",
    text: "aaaaaaa",
    column_start: 1,
    column_end: 2,
    row_start: 1,
    row_end: 3,
    span: 2,
    break_all: true,
};
const RIGHT_ROW: Item = Item {
    id: "span2-right",
    text: "aaaaaaaa",
    column_start: 1,
    column_end: 2,
    row_start: 2,
    row_end: 4,
    span: 2,
    break_all: true,
};
const SPAN3_ROW: Item = Item {
    id: "span3",
    text: "aaaaaaaaaaaaaa",
    column_start: 1,
    column_end: 2,
    row_start: 1,
    row_end: 4,
    span: 3,
    break_all: true,
};

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    axis: Axis,
    width: Dimension,
    height: Dimension,
    items: [Item; 3],
}

const SCENES: [Scene; 12] = [
    Scene {
        id: "cyclic-minmax-calc-overlap-forward",
        axis: Axis::Columns,
        width: Dimension::MaxContent,
        height: Dimension::Px(10.0),
        items: [LEFT, RIGHT, SPAN3],
    },
    Scene {
        id: "cyclic-minmax-calc-overlap-reverse",
        axis: Axis::Columns,
        width: Dimension::MaxContent,
        height: Dimension::Px(10.0),
        items: [SPAN3, RIGHT, LEFT],
    },
    Scene {
        id: "definite-minmax-calc-overlap-forward",
        axis: Axis::Columns,
        width: Dimension::Px(140.0),
        height: Dimension::Px(10.0),
        items: [LEFT, RIGHT, SPAN3],
    },
    Scene {
        id: "definite-minmax-calc-overlap-reverse",
        axis: Axis::Columns,
        width: Dimension::Px(140.0),
        height: Dimension::Px(10.0),
        items: [SPAN3, RIGHT, LEFT],
    },
    Scene {
        id: "definite-minmax-calc-overlap-small-forward",
        axis: Axis::Columns,
        width: Dimension::Px(100.0),
        height: Dimension::Px(10.0),
        items: [LEFT, RIGHT, SPAN3],
    },
    Scene {
        id: "definite-minmax-calc-overlap-small-reverse",
        axis: Axis::Columns,
        width: Dimension::Px(100.0),
        height: Dimension::Px(10.0),
        items: [SPAN3, RIGHT, LEFT],
    },
    Scene {
        id: "cyclic-minmax-calc-overlap-rows-forward",
        axis: Axis::Rows,
        width: Dimension::Px(10.0),
        height: Dimension::MaxContent,
        items: [LEFT_ROW, RIGHT_ROW, SPAN3_ROW],
    },
    Scene {
        id: "cyclic-minmax-calc-overlap-rows-reverse",
        axis: Axis::Rows,
        width: Dimension::Px(10.0),
        height: Dimension::MaxContent,
        items: [SPAN3_ROW, RIGHT_ROW, LEFT_ROW],
    },
    Scene {
        id: "definite-minmax-calc-overlap-rows-forward",
        axis: Axis::Rows,
        width: Dimension::Px(10.0),
        height: Dimension::Px(140.0),
        items: [LEFT_ROW, RIGHT_ROW, SPAN3_ROW],
    },
    Scene {
        id: "definite-minmax-calc-overlap-rows-reverse",
        axis: Axis::Rows,
        width: Dimension::Px(10.0),
        height: Dimension::Px(140.0),
        items: [SPAN3_ROW, RIGHT_ROW, LEFT_ROW],
    },
    Scene {
        id: "definite-minmax-calc-overlap-rows-small-forward",
        axis: Axis::Rows,
        width: Dimension::Px(10.0),
        height: Dimension::Px(100.0),
        items: [LEFT_ROW, RIGHT_ROW, SPAN3_ROW],
    },
    Scene {
        id: "definite-minmax-calc-overlap-rows-small-reverse",
        axis: Axis::Rows,
        width: Dimension::Px(10.0),
        height: Dimension::Px(100.0),
        items: [SPAN3_ROW, RIGHT_ROW, LEFT_ROW],
    },
];

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
    owners: &[NodeId],
    engine: &TextLayoutEngine,
) -> Result<super::super::super::root_html_default_render::Input, RuntimeLayoutFailure> {
    if !is_scene_root(snapshot, root)? || owners.len() != 3 {
        return Err(unsupported(root, "scene root or three text owners differ"));
    }
    let color =
        RuntimeSrgbColor::new(229.0 / 255.0, 72.0 / 255.0, 77.0 / 255.0, 1.0).map_err(|error| {
            layout_failure(
                "c114_cyclic_calc_track_text_color_invalid",
                Some(root.id().get()),
                Some(error.to_string()),
            )
        })?;
    super::super::super::root_html_default_render::prepare_fixture_text_owners(
        snapshot, root, styles, owners, engine, color,
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
        if root_id.starts_with("c114-cyclic-minmax-calc-overlap-")
            || root_id.starts_with("c114-definite-minmax-calc-overlap-")
        {
            return Err(unsupported(root, "unregistered overlapping span scene"));
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
    validate_grid_style(style_for(styles, root.id())?, scene)?;

    let grid_children = children(snapshot, root)?;
    if grid_children.len() != scene.items.len() {
        return Err(unsupported(
            root,
            "exactly three direct text items required",
        ));
    }
    let mut paragraph_styles = BTreeMap::new();
    for (item, item_handle) in scene.items.into_iter().zip(grid_children) {
        let (tag, id) = element_identity(snapshot, item_handle)?;
        if tag != "div"
            || id != format!("c114-{}-item-{}", scene.id, item.id)
            || snapshot.parent(item_handle) != Some(HostParent::Node(root))
            || !has_empty_attribute(snapshot, item_handle, "data-node")
        {
            return Err(unsupported(
                item_handle,
                "item identity or DOM order differs",
            ));
        }
        validate_item_style(style_for(styles, item_handle.id())?, scene, item)?;
        let text_children = children(snapshot, item_handle)?;
        if text_children.len() != 1 {
            return Err(unsupported(
                item_handle,
                "one direct Ahem text node required",
            ));
        }
        let text_node = text_children[0];
        let text = snapshot
            .node(text_node)
            .and_then(|node| match node.kind() {
                HostNodeKind::Text(text) => Some(text.to_string_lossy()),
                HostNodeKind::Element(_) => None,
            })
            .ok_or_else(|| unsupported(text_node, "direct text node required"))?;
        if text != item.text || snapshot.parent(text_node) != Some(HostParent::Node(item_handle)) {
            return Err(unsupported(text_node, "Ahem text content or owner differs"));
        }
        paragraph_styles.insert(item_handle.id(), ahem_paragraph_style(item));
    }

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).map_err(|error| {
        layout_failure(
            "c114_cyclic_calc_track_font_registration",
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

fn validate_grid_style(
    style: &ComputedElementStyle,
    scene: Scene,
) -> Result<(), RuntimeLayoutFailure> {
    let expected_dimension = |dimension| match dimension {
        Dimension::MaxContent => ComputedCssDimension::MaxContent,
        Dimension::Px(value) => ComputedCssDimension::LengthPx(value),
    };
    let expected_column_gap = if scene.axis == Axis::Columns {
        2.0
    } else {
        0.0
    };
    let expected_row_gap = if scene.axis == Axis::Rows { 2.0 } else { 0.0 };
    let expected_border = ComputedLayoutBorder {
        top: 2.0,
        right: 2.0,
        bottom: 2.0,
        left: 2.0,
    };
    if style.properties.get("display").map(String::as_str) != Some("grid")
        || style.properties.get("box-sizing").map(String::as_str) != Some("content-box")
        || style.properties.get("align-content").map(String::as_str) != Some("start")
        || style.properties.get("justify-content").map(String::as_str) != Some("start")
        || style.layout_dimensions.width != expected_dimension(scene.width)
        || style.layout_dimensions.height != expected_dimension(scene.height)
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
    let (calc_tracks, fixed_tracks) = if scene.axis == Axis::Columns {
        (&grid.columns, &grid.rows)
    } else {
        (&grid.rows, &grid.columns)
    };
    if calc_tracks.len() != 3
        || !calc_tracks.iter().all(calc_minmax_track_matches)
        || fixed_tracks.as_slice() != [ComputedCssGridTrackSize::LengthPx(10.0)]
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

fn calc_minmax_track_matches(track: &ComputedCssGridTrackSize) -> bool {
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
}

fn validate_item_style(
    style: &ComputedElementStyle,
    scene: Scene,
    item: Item,
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
        (
            "text-wrap-mode",
            if item.break_all { "wrap" } else { "nowrap" },
        ),
        (
            "word-break",
            if item.break_all {
                "break-all"
            } else {
                "normal"
            },
        ),
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
                "unsupported_c114_cyclic_calc_overlapping_span_style",
                Some(style.node_id.get()),
                Some(format!("{}: {name} expected {expected}", scene.id)),
            ));
        }
    }
    let placement_matches = style.grid_item_placement.as_ref().is_some_and(|placement| {
        placement.column_start == ComputedCssGridPlacement::Line(item.column_start)
            && placement.column_end == ComputedCssGridPlacement::Line(item.column_end)
            && placement.row_start == ComputedCssGridPlacement::Line(item.row_start)
            && placement.row_end == ComputedCssGridPlacement::Line(item.row_end)
    });
    let placement_span = if scene.axis == Axis::Rows {
        item.row_end - item.row_start
    } else {
        item.column_end - item.column_start
    };
    if !placement_matches
        || placement_span as u16 != item.span
        || (style.font_size_css_px - 10.0).abs() > f32::EPSILON
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
        return Err(layout_failure(
            "unsupported_c114_cyclic_calc_overlapping_span_item",
            Some(style.node_id.get()),
            Some(format!(
                "{}: {} placement or box differs",
                scene.id, item.id
            )),
        ));
    }
    Ok(())
}

fn ahem_paragraph_style(item: Item) -> ParagraphStyle {
    ParagraphStyle {
        overflow_wrap: Default::default(),
        writing_mode: spinon_core::WritingMode::HorizontalTb,
        text_orientation: spinon_core::TextOrientation::Mixed,
        font_family: "Ahem".to_owned(),
        font_size: 10.0,
        line_height: 10.0,
        white_space: if item.break_all {
            WhiteSpaceMode::Normal
        } else {
            WhiteSpaceMode::NoWrap
        },
        word_break: if item.break_all {
            WordBreakMode::BreakAll
        } else {
            WordBreakMode::Normal
        },
        ..ParagraphStyle::default()
    }
}

fn zero_edges(edges: spinon_style::ComputedCssEdges) -> bool {
    [edges.top, edges.right, edges.bottom, edges.left]
        .into_iter()
        .all(|value| spacing_matches(value, 0.0))
}

fn spacing_matches(value: ComputedCssSpacingValue, expected: f32) -> bool {
    matches!(value, ComputedCssSpacingValue::LengthPx(actual) if (actual - expected).abs() <= f32::EPSILON)
}

fn unsupported(node: HostNodeHandle, detail: &str) -> RuntimeLayoutFailure {
    unsupported_node(node.id().get(), detail)
}

fn unsupported_node(node: u64, detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_cyclic_calc_overlapping_span_fixture",
        Some(node),
        Some(detail.to_owned()),
    )
}

#[cfg(test)]
mod tests {
    use super::{Axis, SCENES};
    use std::collections::BTreeSet;

    #[test]
    fn exposes_only_fixed_overlapping_cyclic_calc_span_scenes() {
        let unique = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 12);
        assert_eq!(unique.len(), SCENES.len());
        assert_eq!(
            SCENES
                .iter()
                .filter(|scene| scene.axis == Axis::Columns)
                .count(),
            6
        );
        assert_eq!(
            SCENES
                .iter()
                .filter(|scene| scene.axis == Axis::Rows)
                .count(),
            6
        );
        for scene in SCENES {
            assert_eq!(scene.items.len(), 3);
            assert!(scene.items.iter().any(|item| item.span == 2));
            assert!(scene.items.iter().any(|item| item.span == 3));
            assert!(
                scene
                    .items
                    .iter()
                    .all(|item| item.break_all == (scene.axis == Axis::Rows))
            );
        }
    }
}
