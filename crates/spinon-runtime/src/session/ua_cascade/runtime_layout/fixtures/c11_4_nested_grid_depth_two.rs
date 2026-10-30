use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, NodeId};
use spinon_layout::{ParagraphStyle, TextLayoutEngine, WhiteSpaceMode};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackSize,
    ComputedCssMaxSize, ComputedCssSpacingValue, ComputedElementStyle, ComputedStyleProfile,
    ComputedStyleSnapshot,
};

use super::super::{RuntimeLayoutFailure, layout_failure};

pub(super) const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
pub(super) const AHEM_TEXT: &str = "XX XXXXXXX";

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    depth: usize,
    constraint: Constraint,
    two_axis: bool,
}

#[derive(Clone, Copy)]
enum Constraint {
    MinContent,
    MaxContent,
}

impl Constraint {
    fn dimension(self) -> ComputedCssDimension {
        match self {
            Self::MinContent => ComputedCssDimension::MinContent,
            Self::MaxContent => ComputedCssDimension::MaxContent,
        }
    }

    fn track(self) -> ComputedCssGridTrackSize {
        match self {
            Self::MinContent => ComputedCssGridTrackSize::MinContent,
            Self::MaxContent => ComputedCssGridTrackSize::MaxContent,
        }
    }
}

const SCENES: [Scene; 10] = [
    Scene {
        id: "nested-depth-one-min-content",
        depth: 1,
        constraint: Constraint::MinContent,
        two_axis: false,
    },
    Scene {
        id: "nested-depth-two-min-content",
        depth: 2,
        constraint: Constraint::MinContent,
        two_axis: false,
    },
    Scene {
        id: "nested-depth-one-max-content",
        depth: 1,
        constraint: Constraint::MaxContent,
        two_axis: false,
    },
    Scene {
        id: "nested-depth-two-max-content",
        depth: 2,
        constraint: Constraint::MaxContent,
        two_axis: false,
    },
    Scene {
        id: "nested-depth-one-min-content-two-axis",
        depth: 1,
        constraint: Constraint::MinContent,
        two_axis: true,
    },
    Scene {
        id: "nested-depth-two-min-content-two-axis",
        depth: 2,
        constraint: Constraint::MinContent,
        two_axis: true,
    },
    Scene {
        id: "nested-depth-one-max-content-two-axis",
        depth: 1,
        constraint: Constraint::MaxContent,
        two_axis: true,
    },
    Scene {
        id: "nested-depth-two-max-content-two-axis",
        depth: 2,
        constraint: Constraint::MaxContent,
        two_axis: true,
    },
    Scene {
        id: "nested-depth-three-min-content-two-axis",
        depth: 3,
        constraint: Constraint::MinContent,
        two_axis: true,
    },
    Scene {
        id: "nested-depth-three-max-content-two-axis",
        depth: 3,
        constraint: Constraint::MaxContent,
        two_axis: true,
    },
];

pub(super) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<super::C11_4TextInput>, RuntimeLayoutFailure> {
    let (root_tag, root_id) = element_identity(snapshot, root)?;
    let Some(scene) = SCENES
        .iter()
        .find(|scene| format!("c114-{}-root", scene.id) == root_id)
        .copied()
    else {
        if root_id.starts_with("c114-nested-depth-") {
            return Err(unsupported(root.id(), "unlisted depth scene"));
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

    let mut current = root;
    for depth in 0..=scene.depth {
        validate_grid_style(style_for(styles, current.id())?, scene)?;
        let current_children = children(snapshot, current)?;
        if current_children.len() != 1 {
            return Err(unsupported(
                current.id(),
                "each Grid must have exactly one direct child",
            ));
        }
        let child = current_children[0];
        if depth == scene.depth {
            let (tag, id) = element_identity(snapshot, child)?;
            if tag != "div"
                || id != format!("c114-{}-text", scene.id)
                || snapshot.parent(child) != Some(HostParent::Node(current))
                || !has_empty_attribute(snapshot, child, "data-node")
            {
                return Err(unsupported(child.id(), "leaf text item identity or owner"));
            }
            validate_placement(style_for(styles, child.id())?, scene.id)?;
            validate_text(snapshot, child, styles)?;
        } else {
            let role = match (scene.depth, depth) {
                (1, 0) | (2, 1) | (3, 2) => "inner",
                (2, 0) | (3, 0) => "middle",
                (3, 1) => "branch",
                _ => return Err(unsupported(child.id(), "unlisted nested Grid depth")),
            };
            let expected_id = format!("c114-{}-{role}", scene.id);
            let (tag, id) = element_identity(snapshot, child)?;
            if tag != "div"
                || id != expected_id
                || snapshot.parent(child) != Some(HostParent::Node(current))
                || !has_empty_attribute(snapshot, child, "data-node")
            {
                return Err(unsupported(child.id(), "Grid chain identity or owner"));
            }
            validate_placement(style_for(styles, child.id())?, scene.id)?;
            current = child;
        }
    }

    let mut paragraph_styles = BTreeMap::new();
    let leaf = children(snapshot, current)?[0];
    paragraph_styles.insert(
        leaf.id(),
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
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine
        .register_font("Ahem", super::c11_4_text::AHEM)
        .map_err(|error| {
            layout_failure(
                "c114_nested_grid_depth_two_font_registration",
                Some(root.id().get()),
                Some(error.to_string()),
            )
        })?;
    Ok(Some(super::C11_4TextInput {
        paragraph_styles,
        engine,
        two_axis_nested_grid: scene.two_axis,
        content_box_ratio_node: None,
    }))
}

fn validate_text(
    snapshot: &HostDocumentSnapshot,
    text_item: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<(), RuntimeLayoutFailure> {
    let text_nodes = children(snapshot, text_item)?;
    if text_nodes.len() != 1 {
        return Err(unsupported(
            text_item.id(),
            "one direct Ahem text node required",
        ));
    }
    let text_node = text_nodes[0];
    let text = snapshot
        .node(text_node)
        .and_then(|node| match node.kind() {
            HostNodeKind::Text(text) => Some(text.to_string_lossy()),
            HostNodeKind::Element(_) => None,
        })
        .ok_or_else(|| unsupported(text_node.id(), "direct text node required"))?;
    if text != AHEM_TEXT || snapshot.parent(text_node) != Some(HostParent::Node(text_item)) {
        return Err(unsupported(text_node.id(), "Ahem text or owner"));
    }
    let style = style_for(styles, text_item.id())?;
    validate_text_style(style)
}

fn validate_grid_style(
    style: &ComputedElementStyle,
    scene: Scene,
) -> Result<(), RuntimeLayoutFailure> {
    let grid = style.grid_layout.as_ref().ok_or_else(|| {
        layout_failure(
            "missing_c114_nested_grid_depth_two_layout",
            Some(style.node_id.get()),
            Some(scene.id.to_owned()),
        )
    })?;
    let expected_height = if scene.two_axis {
        scene.constraint.dimension()
    } else {
        ComputedCssDimension::Auto
    };
    let expected_rows = if scene.two_axis {
        scene.constraint.track()
    } else {
        ComputedCssGridTrackSize::Auto
    };
    if style.properties.get("display").map(String::as_str) != Some("grid")
        || style.properties.get("align-items").map(String::as_str) != Some("start")
        || style.properties.get("box-sizing").map(String::as_str) != Some("border-box")
        || style.layout_dimensions.width != scene.constraint.dimension()
        || style.layout_dimensions.height != expected_height
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
        || grid.columns != [scene.constraint.track()]
        || grid.rows != [expected_rows]
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
            "Grid style or one-track template",
        ));
    }
    Ok(())
}

fn validate_placement(
    style: &ComputedElementStyle,
    scene_id: &str,
) -> Result<(), RuntimeLayoutFailure> {
    let placement = style.grid_item_placement.as_ref().ok_or_else(|| {
        layout_failure(
            "missing_c114_nested_grid_depth_two_placement",
            Some(style.node_id.get()),
            Some(scene_id.to_owned()),
        )
    })?;
    if placement.row_start != ComputedCssGridPlacement::Auto
        || placement.row_end != ComputedCssGridPlacement::Auto
        || placement.column_start != ComputedCssGridPlacement::Auto
        || placement.column_end != ComputedCssGridPlacement::Auto
    {
        return Err(unsupported(style.node_id, "Grid item placement"));
    }
    Ok(())
}

fn validate_text_style(style: &ComputedElementStyle) -> Result<(), RuntimeLayoutFailure> {
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
        ("letter-spacing", "normal"),
        ("word-spacing", "0px"),
        ("text-indent", "0px"),
    ] {
        if style.properties.get(name).map(String::as_str) != Some(expected) {
            return Err(unsupported(style.node_id, name));
        }
    }
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
            "text item dimensions or box edges",
        ));
    }
    Ok(())
}

pub(super) fn element_identity(
    snapshot: &HostDocumentSnapshot,
    handle: HostNodeHandle,
) -> Result<(String, String), RuntimeLayoutFailure> {
    let node = snapshot
        .node(handle)
        .ok_or_else(|| layout_failure("missing_node", Some(handle.id().get()), None))?;
    let HostNodeKind::Element(element) = node.kind() else {
        return Err(unsupported(handle.id(), "expected HTML element"));
    };
    if element.namespace() != HTML_NAMESPACE {
        return Err(unsupported(handle.id(), "expected HTML namespace"));
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

pub(super) fn has_empty_attribute(
    snapshot: &HostDocumentSnapshot,
    handle: HostNodeHandle,
    attribute_name: &str,
) -> bool {
    let Some(node) = snapshot.node(handle) else {
        return false;
    };
    let HostNodeKind::Element(element) = node.kind() else {
        return false;
    };
    element.attributes().iter().any(|(name, value)| {
        name.namespace().is_none()
            && name.local_name() == attribute_name
            && value.to_string_lossy().is_empty()
    })
}

pub(super) fn children(
    snapshot: &HostDocumentSnapshot,
    node: HostNodeHandle,
) -> Result<Vec<HostNodeHandle>, RuntimeLayoutFailure> {
    snapshot
        .children(node)
        .map(|children| children.collect())
        .ok_or_else(|| layout_failure("missing_node", Some(node.id().get()), None))
}

pub(super) fn style_for(
    styles: &ComputedStyleSnapshot,
    node_id: NodeId,
) -> Result<&ComputedElementStyle, RuntimeLayoutFailure> {
    styles
        .elements
        .iter()
        .find(|style| style.node_id == node_id)
        .ok_or_else(|| layout_failure("missing_computed_style", Some(node_id.get()), None))
}

fn zero_edges(edges: spinon_style::ComputedCssEdges) -> bool {
    [edges.top, edges.right, edges.bottom, edges.left]
        .into_iter()
        .all(|value| spacing_is(value, 0.0))
}

fn spacing_is(value: ComputedCssSpacingValue, expected: f32) -> bool {
    matches!(value, ComputedCssSpacingValue::LengthPx(actual) if (actual - expected).abs() <= f32::EPSILON)
}

fn unsupported(node: NodeId, detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_nested_grid_depth_two_scene",
        Some(node.get()),
        Some(detail.to_owned()),
    )
}
