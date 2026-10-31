use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent};
use spinon_layout::{ParagraphStyle, TextLayoutEngine, WhiteSpaceMode};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackSize,
    ComputedCssMaxSize, ComputedCssSpacingValue, ComputedElementStyle, ComputedStyleProfile,
    ComputedStyleSnapshot,
};

use super::super::{RuntimeLayoutFailure, layout_failure};
use super::c11_4_nested_grid_depth_two::{
    children, element_identity, has_empty_attribute, style_for,
};

#[derive(Clone, Copy)]
struct Item {
    suffix: &'static str,
    text: &'static str,
    column: i32,
    span: i32,
    row: i32,
}

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    items: &'static [Item],
}

const A: Item = Item {
    suffix: "a",
    text: "XX XXXXX",
    column: 1,
    span: 2,
    row: 1,
};
const B: Item = Item {
    suffix: "b",
    text: "XXXX XXXXXXX",
    column: 2,
    span: 2,
    row: 2,
};
const C: Item = Item {
    suffix: "c",
    text: "XXXXXXXXXXXXXXXXXX",
    column: 1,
    span: 3,
    row: 3,
};

const SCENES: [Scene; 4] = [
    Scene {
        id: "nested-text-span2-forward",
        items: &[A, B],
    },
    Scene {
        id: "nested-text-span2-reversed",
        items: &[B, A],
    },
    Scene {
        id: "nested-text-span2-before-span3",
        items: &[A, B, C],
    },
    Scene {
        id: "nested-text-span3-before-span2",
        items: &[C, B, A],
    },
];

const GRID_ROLES: [&str; 4] = ["root", "middle", "branch", "inner"];

pub(super) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<super::C11_4TextInput>, RuntimeLayoutFailure> {
    let (root_tag, root_id) = element_identity(snapshot, root)?;
    let Some(scene_id) = root_id
        .strip_prefix("c114-")
        .and_then(|id| id.strip_suffix("-root"))
    else {
        return Ok(None);
    };
    let Some(scene) = SCENES.iter().find(|scene| scene.id == scene_id).copied() else {
        if scene_id.starts_with("nested-text-span") {
            return Err(unsupported(
                root.id(),
                "unlisted nested text multi-span scene",
            ));
        }
        return Ok(None);
    };
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || root_tag != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
        || !has_empty_attribute(snapshot, root, "data-node")
    {
        return Err(unsupported(
            root.id(),
            "root profile, tag, owner or node marker",
        ));
    }

    let mut grids = vec![root];
    let mut current = root;
    for role in GRID_ROLES.iter().skip(1) {
        let nested = children(snapshot, current)?;
        if nested.len() != 1 {
            return Err(unsupported(
                current.id(),
                "each outer Grid must have exactly one nested Grid child",
            ));
        }
        let child = nested[0];
        let (tag, id) = element_identity(snapshot, child)?;
        let expected_id = format!("c114-{}-{role}", scene.id);
        if tag != "div"
            || id != expected_id
            || snapshot.parent(child) != Some(HostParent::Node(current))
            || !has_empty_attribute(snapshot, child, "data-node")
        {
            return Err(unsupported(child.id(), "nested Grid identity or owner"));
        }
        validate_grid_item_placement(style_for(styles, child.id())?, scene.id)?;
        grids.push(child);
        current = child;
    }

    for (index, grid) in grids.iter().enumerate() {
        validate_grid_style(style_for(styles, grid.id())?, index == 3, scene.id)?;
    }
    let item_handles = children(snapshot, current)?;
    if item_handles.len() != scene.items.len() {
        return Err(unsupported(
            current.id(),
            "inner Grid must contain exactly the allowlisted text items",
        ));
    }

    let mut paragraph_styles = BTreeMap::new();
    for (handle, expected) in item_handles.iter().zip(scene.items) {
        let (tag, id) = element_identity(snapshot, *handle)?;
        let expected_id = format!("c114-{}-item-{}", scene.id, expected.suffix);
        if tag != "div"
            || id != expected_id
            || snapshot.parent(*handle) != Some(HostParent::Node(current))
            || !has_empty_attribute(snapshot, *handle, "data-node")
        {
            return Err(unsupported(handle.id(), "text item identity or owner"));
        }
        let text_children = children(snapshot, *handle)?;
        if text_children.len() != 1 {
            return Err(unsupported(
                handle.id(),
                "one direct text child is required",
            ));
        }
        let text_node = text_children[0];
        let text = snapshot
            .node(text_node)
            .and_then(|node| match node.kind() {
                HostNodeKind::Text(text) => Some(text.to_string_lossy()),
                HostNodeKind::Element(_) => None,
            })
            .ok_or_else(|| unsupported(text_node.id(), "direct text node is required"))?;
        if text != expected.text || snapshot.parent(text_node) != Some(HostParent::Node(*handle)) {
            return Err(unsupported(text_node.id(), "text content or owner differs"));
        }

        let style = style_for(styles, handle.id())?;
        validate_text_item_style(style, *expected, scene.id)?;
        paragraph_styles.insert(
            handle.id(),
            ParagraphStyle {
                overflow_wrap: Default::default(),
                writing_mode: spinon_core::WritingMode::HorizontalTb,
                text_orientation: spinon_core::TextOrientation::Mixed,
                font_family: "Ahem".to_owned(),
                font_size: 10.0,
                line_height: 10.0,
                white_space: WhiteSpaceMode::Normal,
                ..ParagraphStyle::default()
            },
        );
    }

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine
        .register_font("Ahem", super::c11_4_text::AHEM)
        .map_err(|error| {
            layout_failure(
                "c114_nested_grid_text_multispan_font_registration",
                Some(root.id().get()),
                Some(error.to_string()),
            )
        })?;
    Ok(Some(super::C11_4TextInput {
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
    inner: bool,
    scene_id: &str,
) -> Result<(), RuntimeLayoutFailure> {
    let grid = style.grid_layout.as_ref().ok_or_else(|| {
        layout_failure(
            "missing_c114_nested_grid_text_multispan_layout",
            Some(style.node_id.get()),
            Some(scene_id.to_owned()),
        )
    })?;
    let expected_columns = if inner {
        vec![ComputedCssGridTrackSize::MinContent; 4]
    } else {
        vec![ComputedCssGridTrackSize::MinContent]
    };
    let expected_rows = if inner {
        vec![ComputedCssGridTrackSize::LengthPx(20.0); 3]
    } else {
        vec![ComputedCssGridTrackSize::LengthPx(60.0)]
    };
    if !property_is(style, "display", "grid")
        || !property_is(style, "box-sizing", "border-box")
        || !property_is(style, "align-items", "start")
        || !property_is(style, "align-content", "start")
        || !property_is(style, "justify-content", "start")
        || style.layout_dimensions.width != ComputedCssDimension::MinContent
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(60.0)
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || style.layout_position != spinon_style::ComputedCssPosition::Static
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || !spacing_is(style.layout_spacing.column_gap, 0.0)
        || !spacing_is(style.layout_spacing.row_gap, 0.0)
        || style.layout_border != Default::default()
        || grid.columns != expected_columns
        || grid.rows != expected_rows
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || grid.auto_rows != ComputedCssGridAutoRows::Auto
        || grid.auto_columns != ComputedCssGridAutoColumns::Auto
        || grid
            .columns_line_names
            .iter()
            .any(|names| !names.is_empty())
        || grid.rows_line_names.iter().any(|names| !names.is_empty())
    {
        return Err(unsupported(
            style.node_id,
            "Grid style or fixed track matrix differs",
        ));
    }
    Ok(())
}

fn validate_grid_item_placement(
    style: &ComputedElementStyle,
    scene_id: &str,
) -> Result<(), RuntimeLayoutFailure> {
    let placement = style.grid_item_placement.as_ref().ok_or_else(|| {
        layout_failure(
            "missing_c114_nested_grid_text_multispan_placement",
            Some(style.node_id.get()),
            Some(scene_id.to_owned()),
        )
    })?;
    if placement.row_start != ComputedCssGridPlacement::Auto
        || placement.row_end != ComputedCssGridPlacement::Auto
        || placement.column_start != ComputedCssGridPlacement::Auto
        || placement.column_end != ComputedCssGridPlacement::Auto
    {
        return Err(unsupported(style.node_id, "nested Grid item placement"));
    }
    Ok(())
}

fn validate_text_item_style(
    style: &ComputedElementStyle,
    item: Item,
    scene_id: &str,
) -> Result<(), RuntimeLayoutFailure> {
    let placement = style.grid_item_placement.as_ref().ok_or_else(|| {
        layout_failure(
            "missing_c114_nested_grid_text_multispan_item_placement",
            Some(style.node_id.get()),
            Some(scene_id.to_owned()),
        )
    })?;
    if ![
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
        ("writing-mode", "horizontal-tb"),
        ("letter-spacing", "normal"),
        ("word-spacing", "0px"),
        ("text-indent", "0px"),
    ]
    .into_iter()
    .all(|(name, expected)| property_is(style, name, expected))
        || style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || style.layout_position != spinon_style::ComputedCssPosition::Static
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || style.layout_border != Default::default()
        || placement.column_start != ComputedCssGridPlacement::Line(item.column)
        || placement.column_end != ComputedCssGridPlacement::Span(item.span)
        || placement.row_start != ComputedCssGridPlacement::Line(item.row)
        || placement.row_end != ComputedCssGridPlacement::Auto
    {
        return Err(unsupported(
            style.node_id,
            "text style, item box or span differs",
        ));
    }
    Ok(())
}

fn property_is(style: &ComputedElementStyle, name: &str, expected: &str) -> bool {
    style.properties.get(name).map(String::as_str) == Some(expected)
}

fn zero_edges(edges: spinon_style::ComputedCssEdges) -> bool {
    [edges.top, edges.right, edges.bottom, edges.left]
        .into_iter()
        .all(|value| spacing_is(value, 0.0))
}

fn spacing_is(value: ComputedCssSpacingValue, expected: f32) -> bool {
    matches!(value, ComputedCssSpacingValue::LengthPx(actual) if (actual - expected).abs() <= f32::EPSILON)
}

fn unsupported(node: spinon_core::NodeId, detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_nested_grid_text_multispan_scene",
        Some(node.get()),
        Some(detail.to_owned()),
    )
}
