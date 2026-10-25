use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, NodeId};
use spinon_layout::{ParagraphStyle, TextLayoutEngine, WhiteSpaceMode};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackBreadth,
    ComputedCssGridTrackSize, ComputedCssMaxSize, ComputedCssSpacingValue, ComputedElementStyle,
    ComputedStyleProfile, ComputedStyleSnapshot,
};

use super::super::{RuntimeLayoutFailure, layout_failure};

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

const SPAN_TWO: Scene = Scene {
    id: "nested-grid-span-two",
    root_width: RootWidth::MaxContent,
    outer_columns: OuterColumns::MinContentPair,
    nested_column_end: ComputedCssGridPlacement::Span(2),
};
const ONE_TRACK: Scene = Scene {
    id: "nested-grid-one-track",
    root_width: RootWidth::MaxContent,
    outer_columns: OuterColumns::MinContentPair,
    nested_column_end: ComputedCssGridPlacement::Line(2),
};
const DEFINITE_FLEX_CONTROL: Scene = Scene {
    id: "nested-grid-definite-flex-control",
    root_width: RootWidth::Fixed120,
    outer_columns: OuterColumns::AutoMinFlexPair,
    nested_column_end: ComputedCssGridPlacement::Span(2),
};
const SCENES: [Scene; 3] = [SPAN_TWO, ONE_TRACK, DEFINITE_FLEX_CONTROL];

const ITEMS: [Item; 2] = [
    Item {
        id: "item-a",
        text: "XX XXXXXXX",
        column: 1,
    },
    Item {
        id: "item-b",
        text: "XXXXXX",
        column: 2,
    },
];

#[derive(Clone)]
struct Scene {
    id: &'static str,
    root_width: RootWidth,
    outer_columns: OuterColumns,
    nested_column_end: ComputedCssGridPlacement,
}

#[derive(Clone, Copy)]
struct Item {
    id: &'static str,
    text: &'static str,
    column: i32,
}

#[derive(Clone, Copy)]
enum RootWidth {
    MaxContent,
    Fixed120,
}

#[derive(Clone, Copy)]
enum OuterColumns {
    MinContentPair,
    AutoMinFlexPair,
}

pub(super) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<super::C11_4TextInput>, RuntimeLayoutFailure> {
    let (root_tag, root_id) = element_identity(snapshot, root)?;
    let Some(scene) = SCENES
        .iter()
        .find(|scene| format!("c114-{}-grid", scene.id) == root_id)
        .cloned()
    else {
        return Ok(None);
    };
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || root_tag != "div"
    {
        return Err(unsupported(root.id(), scene.id, "root profile or tag"));
    }

    let root_children = children(snapshot, root)?;
    if root_children.len() != 1 {
        return Err(unsupported(
            root.id(),
            scene.id,
            "one direct nested Grid required",
        ));
    }
    let nested = root_children[0];
    let (nested_tag, nested_id) = element_identity(snapshot, nested)?;
    if nested_tag != "div"
        || nested_id != format!("c114-{}-nested", scene.id)
        || snapshot.parent(nested) != Some(HostParent::Node(root))
    {
        return Err(unsupported(
            nested.id(),
            scene.id,
            "nested Grid identity or owner",
        ));
    }

    let nested_children = children(snapshot, nested)?;
    if nested_children.len() != ITEMS.len() {
        return Err(unsupported(
            nested.id(),
            scene.id,
            "two direct text items required",
        ));
    }

    validate_grid_style(style_for(styles, root.id())?, &scene, true)?;
    validate_grid_style(style_for(styles, nested.id())?, &scene, false)?;
    validate_placement(
        style_for(styles, nested.id())?,
        scene.id,
        ComputedCssGridPlacement::Line(1),
        scene.nested_column_end.clone(),
    )?;

    let mut paragraph_styles = BTreeMap::new();
    for (handle, expected) in nested_children.iter().zip(ITEMS) {
        let (tag, id) = element_identity(snapshot, *handle)?;
        if tag != "div"
            || id != format!("c114-{}-{}", scene.id, expected.id)
            || snapshot.parent(*handle) != Some(HostParent::Node(nested))
        {
            return Err(unsupported(
                handle.id(),
                scene.id,
                "inner text item identity or owner",
            ));
        }
        let text_nodes = children(snapshot, *handle)?;
        if text_nodes.len() != 1 {
            return Err(unsupported(
                handle.id(),
                scene.id,
                "one direct text node required",
            ));
        }
        let text_node = text_nodes[0];
        let text = snapshot
            .node(text_node)
            .and_then(|node| match node.kind() {
                HostNodeKind::Text(text) => Some(text.to_string_lossy()),
                HostNodeKind::Element(_) => None,
            })
            .ok_or_else(|| unsupported(text_node.id(), scene.id, "direct text node required"))?;
        if text != expected.text || snapshot.parent(text_node) != Some(HostParent::Node(*handle)) {
            return Err(unsupported(
                text_node.id(),
                scene.id,
                "text content or owner",
            ));
        }
        let style = style_for(styles, handle.id())?;
        validate_text_style(style, scene.id)?;
        validate_placement(
            style,
            scene.id,
            ComputedCssGridPlacement::Line(expected.column),
            ComputedCssGridPlacement::Auto,
        )?;
        paragraph_styles.insert(
            handle.id(),
            ParagraphStyle {
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
                "c114_nested_grid_font_registration",
                Some(root.id().get()),
                Some(error.to_string()),
            )
        })?;
    Ok(Some(super::C11_4TextInput {
        paragraph_styles,
        engine,
        two_axis_nested_grid: false,
    }))
}

fn children(
    snapshot: &HostDocumentSnapshot,
    node: HostNodeHandle,
) -> Result<Vec<HostNodeHandle>, RuntimeLayoutFailure> {
    snapshot
        .children(node)
        .map(|children| children.collect())
        .ok_or_else(|| layout_failure("missing_node", Some(node.id().get()), None))
}

fn element_identity(
    snapshot: &HostDocumentSnapshot,
    handle: HostNodeHandle,
) -> Result<(String, String), RuntimeLayoutFailure> {
    let node = snapshot
        .node(handle)
        .ok_or_else(|| layout_failure("missing_node", Some(handle.id().get()), None))?;
    let HostNodeKind::Element(element) = node.kind() else {
        return Err(unsupported(handle.id(), "nested-grid", "expected element"));
    };
    if element.namespace() != HTML_NAMESPACE {
        return Err(unsupported(
            handle.id(),
            "nested-grid",
            "expected HTML namespace",
        ));
    }
    let id = element
        .attributes()
        .iter()
        .find_map(|(name, value)| {
            (name.namespace().is_none() && name.local_name() == "id")
                .then(|| value.to_string_lossy())
        })
        .unwrap_or_default();
    Ok((element.local_name().to_owned(), id))
}

fn style_for(
    styles: &ComputedStyleSnapshot,
    node_id: NodeId,
) -> Result<&ComputedElementStyle, RuntimeLayoutFailure> {
    styles
        .elements
        .iter()
        .find(|style| style.node_id == node_id)
        .ok_or_else(|| layout_failure("missing_computed_style", Some(node_id.get()), None))
}

fn validate_grid_style(
    style: &ComputedElementStyle,
    scene: &Scene,
    is_outer: bool,
) -> Result<(), RuntimeLayoutFailure> {
    let grid = style.grid_layout.as_ref().ok_or_else(|| {
        layout_failure(
            "missing_c114_nested_grid_layout",
            Some(style.node_id.get()),
            Some(scene.id.to_owned()),
        )
    })?;
    let expected_width = if is_outer {
        match scene.root_width {
            RootWidth::MaxContent => ComputedCssDimension::MaxContent,
            RootWidth::Fixed120 => ComputedCssDimension::LengthPx(120.0),
        }
    } else {
        ComputedCssDimension::Auto
    };
    let expected_columns = if is_outer {
        match scene.outer_columns {
            OuterColumns::MinContentPair => vec![
                ComputedCssGridTrackSize::MinContent,
                ComputedCssGridTrackSize::MinContent,
            ],
            OuterColumns::AutoMinFlexPair => vec![
                ComputedCssGridTrackSize::MinMax {
                    min: ComputedCssGridTrackBreadth::Auto,
                    max: ComputedCssGridTrackBreadth::Fraction(1.0),
                },
                ComputedCssGridTrackSize::MinMax {
                    min: ComputedCssGridTrackBreadth::Auto,
                    max: ComputedCssGridTrackBreadth::Fraction(1.0),
                },
            ],
        }
    } else {
        vec![
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::MaxContent,
        ]
    };
    if style.properties.get("display").map(String::as_str) != Some("grid")
        || style.properties.get("box-sizing").map(String::as_str) != Some("border-box")
        || style.layout_dimensions.width != expected_width
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(20.0)
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || !spacing_is(style.layout_spacing.column_gap, 4.0)
        || !spacing_is(style.layout_spacing.row_gap, 0.0)
        || style.layout_border != Default::default()
        || grid.columns != expected_columns
        || grid.rows != [ComputedCssGridTrackSize::Auto]
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
            scene.id,
            if is_outer {
                "outer Grid style"
            } else {
                "nested Grid style"
            },
        ));
    }
    Ok(())
}

fn validate_placement(
    style: &ComputedElementStyle,
    scene_id: &str,
    expected_column_start: ComputedCssGridPlacement,
    expected_column_end: ComputedCssGridPlacement,
) -> Result<(), RuntimeLayoutFailure> {
    let placement = style.grid_item_placement.as_ref().ok_or_else(|| {
        layout_failure(
            "missing_c114_nested_grid_placement",
            Some(style.node_id.get()),
            Some(scene_id.to_owned()),
        )
    })?;
    if placement.row_start != ComputedCssGridPlacement::Auto
        || placement.row_end != ComputedCssGridPlacement::Auto
        || placement.column_start != expected_column_start
        || placement.column_end != expected_column_end
    {
        return Err(unsupported(style.node_id, scene_id, "Grid item placement"));
    }
    Ok(())
}

fn validate_text_style(
    style: &ComputedElementStyle,
    scene_id: &str,
) -> Result<(), RuntimeLayoutFailure> {
    validate_text_profile_properties(style.node_id, &style.properties, scene_id)?;
    if style.layout_dimensions.width != ComputedCssDimension::Auto
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
    {
        return Err(unsupported(
            style.node_id,
            scene_id,
            "text item dimensions or box edges",
        ));
    }
    Ok(())
}

fn validate_text_profile_properties(
    node_id: NodeId,
    properties: &std::collections::BTreeMap<String, String>,
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
        ("writing-mode", "horizontal-tb"),
        ("letter-spacing", "normal"),
        ("word-spacing", "0px"),
        ("text-indent", "0px"),
    ] {
        if properties.get(name).map(String::as_str) != Some(expected) {
            return Err(unsupported(node_id, scene_id, name));
        }
    }
    Ok(())
}

fn zero_edges(edges: spinon_style::ComputedCssEdges) -> bool {
    [edges.top, edges.right, edges.bottom, edges.left]
        .into_iter()
        .all(|value| spacing_is(value, 0.0))
}

fn spacing_is(value: ComputedCssSpacingValue, expected: f32) -> bool {
    matches!(value, ComputedCssSpacingValue::LengthPx(actual) if (actual - expected).abs() <= f32::EPSILON)
}

fn unsupported(node: NodeId, scene: &str, detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_nested_grid_scene",
        Some(node.get()),
        Some(format!("{scene}: {detail}")),
    )
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use spinon_core::NodeId;

    use super::validate_text_profile_properties;

    fn properties() -> BTreeMap<String, String> {
        [
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
        .map(|(name, value)| (name.to_owned(), value.to_owned()))
        .collect()
    }

    #[test]
    fn text_profile_requires_all_intrinsic_sizing_properties() {
        let expected = properties();
        let node = NodeId::new(1).unwrap();
        assert!(validate_text_profile_properties(node, &expected, "nested-grid-span-two").is_ok());

        for (name, changed) in [
            ("font-family", "sans-serif"),
            ("font-size", "11px"),
            ("font-weight", "700"),
            ("font-style", "italic"),
            ("line-height", "normal"),
            ("white-space-collapse", "preserve"),
            ("text-wrap-mode", "nowrap"),
            ("word-break", "break-all"),
            ("direction", "rtl"),
            ("line-break", "strict"),
            ("text-align", "center"),
            ("overflow-wrap", "anywhere"),
            ("text-transform", "uppercase"),
            ("writing-mode", "vertical-rl"),
            ("letter-spacing", "1px"),
            ("word-spacing", "1px"),
            ("text-indent", "1px"),
        ] {
            let mut mutated = expected.clone();
            mutated.insert(name.to_owned(), changed.to_owned());
            assert!(
                validate_text_profile_properties(node, &mutated, "nested-grid-span-two").is_err(),
                "nested Grid fixture must reject changed {name}"
            );
        }
    }
}
