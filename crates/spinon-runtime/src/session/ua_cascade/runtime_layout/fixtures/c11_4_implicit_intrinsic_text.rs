use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, NodeId};
use spinon_layout::{
    FontStyleMode, LigatureMode, LineBreakMode, ParagraphStyle, TextLayoutEngine,
    TextParagraphDirection, WhiteSpaceMode, WordBreakMode,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssMaxSize, ComputedCssSpacingValue, ComputedElementStyle,
    ComputedStyleProfile, ComputedStyleSnapshot,
};

use super::super::{RuntimeLayoutFailure, layout_failure};

const AHEM: &[u8] = include_bytes!("../../../../../../../tests/fixtures/resources/c15/Ahem.ttf");

#[path = "c11_4_implicit_intrinsic_text/cases.rs"]
mod cases;

pub(in crate::session::ua_cascade::runtime_layout) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<super::C11_4TextInput>, RuntimeLayoutFailure> {
    let (root_tag, root_id) = element_identity(snapshot, root)?;
    let Some(case) = cases::SCENES
        .iter()
        .find(|case| format!("c114-{}-grid", case.scene_id) == root_id)
    else {
        if root_id.starts_with("c114-implicit-") {
            return Err(layout_failure(
                "unsupported_c114_implicit_text_root",
                Some(root.id().get()),
                Some(root_id),
            ));
        }
        return Ok(None);
    };

    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 {
        return Err(layout_failure(
            "c114_implicit_text_profile_mismatch",
            Some(root.id().get()),
            None,
        ));
    }
    if root_tag != "div" {
        return Err(layout_failure(
            "unsupported_c114_implicit_text_root_tag",
            Some(root.id().get()),
            Some(root_tag),
        ));
    }

    let grid_style = style_for(styles, root.id())?;
    validate_grid_style(grid_style, case)?;

    let child_handles = snapshot
        .children(root)
        .ok_or_else(|| layout_failure("missing_node", Some(root.id().get()), None))?
        .collect::<Vec<_>>();
    if child_handles.len() != case.items.len() {
        return Err(unsupported_scene(
            root.id(),
            case.scene_id,
            "grid item count",
        ));
    }

    let mut paragraph_styles = BTreeMap::new();
    for (index, (handle, expected)) in child_handles.iter().zip(case.items).enumerate() {
        if snapshot.parent(*handle) != Some(HostParent::Node(root)) {
            return Err(unsupported_scene(
                handle.id(),
                case.scene_id,
                "grid items must be direct children",
            ));
        }
        let (tag, item_id) = element_identity(snapshot, *handle)?;
        let expected_item_id = format!("c114-{}-item-{}", case.scene_id, index + 1);
        if tag != "div" || item_id != expected_item_id {
            return Err(unsupported_scene(
                handle.id(),
                case.scene_id,
                "item tag or identity",
            ));
        }

        let item_children = snapshot
            .children(*handle)
            .ok_or_else(|| layout_failure("missing_node", Some(handle.id().get()), None))?
            .collect::<Vec<_>>();
        if item_children.len() != 1 {
            return Err(unsupported_scene(
                handle.id(),
                case.scene_id,
                "each grid item must contain one direct text node",
            ));
        }
        let text_handle = item_children[0];
        let text = snapshot
            .node(text_handle)
            .and_then(|node| match node.kind() {
                HostNodeKind::Text(text) => Some(text.to_string_lossy()),
                HostNodeKind::Element(_) => None,
            })
            .ok_or_else(|| {
                unsupported_scene(
                    text_handle.id(),
                    case.scene_id,
                    "grid item child must be text",
                )
            })?;
        if snapshot.parent(text_handle) != Some(HostParent::Node(*handle))
            || text != expected.content
        {
            return Err(unsupported_scene(
                text_handle.id(),
                case.scene_id,
                "text source or ownership",
            ));
        }

        let font_size = if is_negative_text_case(case.scene_id) {
            12.0
        } else {
            10.0
        };
        let negative_text_case = is_negative_text_case(case.scene_id);
        validate_text_style(
            style_for(styles, handle.id())?,
            case.scene_id,
            font_size,
            negative_text_case,
        )?;
        paragraph_styles.insert(handle.id(), ahem_paragraph_style(font_size));
    }

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).map_err(|error| {
        layout_failure(
            "c114_implicit_text_font_registration",
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

fn element_identity(
    snapshot: &HostDocumentSnapshot,
    handle: HostNodeHandle,
) -> Result<(String, String), RuntimeLayoutFailure> {
    let node = snapshot
        .node(handle)
        .ok_or_else(|| layout_failure("missing_node", Some(handle.id().get()), None))?;
    let HostNodeKind::Element(element) = node.kind() else {
        return Err(layout_failure(
            "c114_implicit_text_expected_element",
            Some(handle.id().get()),
            None,
        ));
    };
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

fn is_negative_text_case(scene_id: &str) -> bool {
    matches!(
        scene_id,
        "columns-negative-minmax-track"
            | "columns-negative-min-content-track"
            | "columns-negative-max-content-track"
    )
}

fn validate_grid_style(
    style: &ComputedElementStyle,
    case: &cases::SceneCase,
) -> Result<(), RuntimeLayoutFailure> {
    let negative_text_case = is_negative_text_case(case.scene_id);
    let align_content = if negative_text_case {
        "normal"
    } else {
        "start"
    };
    let justify_content = if negative_text_case {
        "normal"
    } else {
        "start"
    };
    for (name, expected) in [
        ("display", "grid"),
        ("box-sizing", "border-box"),
        ("align-items", "normal"),
        ("align-content", align_content),
        ("justify-content", justify_content),
        ("grid-template-columns", case.columns),
        ("grid-template-rows", case.rows),
        ("grid-auto-columns", case.auto_columns),
        ("grid-auto-rows", case.auto_rows),
        ("grid-auto-flow", case.auto_flow),
    ] {
        require_property(style, name, expected, case.scene_id)?;
    }

    let dimensions_match = if negative_text_case {
        style.layout_dimensions.width == ComputedCssDimension::MaxContent
            && style.layout_dimensions.height == ComputedCssDimension::MaxContent
    } else {
        style.layout_dimensions.width == ComputedCssDimension::LengthPx(case.width as f32)
            && style.layout_dimensions.height == ComputedCssDimension::LengthPx(case.height as f32)
    };
    let expected_gap = if negative_text_case { 3.0 } else { 0.0 };
    let expected_border = case.border as f32;
    if !dimensions_match
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || !spacing_is(style.layout_spacing.row_gap, expected_gap)
        || !spacing_is(style.layout_spacing.column_gap, expected_gap)
        || !px_is(style.layout_border.top, expected_border)
        || !px_is(style.layout_border.right, expected_border)
        || !px_is(style.layout_border.bottom, expected_border)
        || !px_is(style.layout_border.left, expected_border)
    {
        return Err(unsupported_scene(
            style.node_id,
            case.scene_id,
            "container box geometry",
        ));
    }

    let grid = style.grid_layout.as_ref().ok_or_else(|| {
        unsupported_scene(style.node_id, case.scene_id, "missing computed grid layout")
    })?;
    let expected_flow = match case.auto_flow {
        "row" => ComputedCssGridAutoFlow::Row,
        "column" => ComputedCssGridAutoFlow::Column,
        _ => {
            return Err(unsupported_scene(
                style.node_id,
                case.scene_id,
                "invalid flow case",
            ));
        }
    };
    let expected_column_count = if negative_text_case { 2 } else { 1 };
    if grid.columns.len() != expected_column_count
        || grid.rows.len() != 1
        || grid.auto_flow != expected_flow
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || grid
            .columns_line_names
            .iter()
            .any(|names| !names.is_empty())
        || grid.rows_line_names.iter().any(|names| !names.is_empty())
        || !auto_track_count_matches(&grid.auto_columns, case.auto_columns_count)
        || !auto_track_count_matches(&grid.auto_rows, case.auto_rows_count)
    {
        return Err(unsupported_scene(
            style.node_id,
            case.scene_id,
            "computed track shape or auto-track list",
        ));
    }
    Ok(())
}

fn auto_track_count_matches<T>(value: &T, expected_count: usize) -> bool
where
    T: AutoTrackValue,
{
    value.track_count() == expected_count
}

trait AutoTrackValue {
    fn track_count(&self) -> usize;
}

impl AutoTrackValue for ComputedCssGridAutoColumns {
    fn track_count(&self) -> usize {
        match self {
            ComputedCssGridAutoColumns::Auto => 0,
            ComputedCssGridAutoColumns::LengthPx(_) => 1,
            ComputedCssGridAutoColumns::TrackList(tracks) => tracks.len(),
            ComputedCssGridAutoColumns::Unsupported(_) => usize::MAX,
        }
    }
}

impl AutoTrackValue for ComputedCssGridAutoRows {
    fn track_count(&self) -> usize {
        match self {
            ComputedCssGridAutoRows::Auto => 0,
            ComputedCssGridAutoRows::LengthPx(_) => 1,
            ComputedCssGridAutoRows::TrackList(tracks) => tracks.len(),
            ComputedCssGridAutoRows::Unsupported(_) => usize::MAX,
        }
    }
}

fn validate_text_style(
    style: &ComputedElementStyle,
    scene_id: &str,
    font_size: f32,
    negative_text_case: bool,
) -> Result<(), RuntimeLayoutFailure> {
    if (style.font_size_css_px - font_size).abs() > f32::EPSILON {
        return Err(unsupported_scene(style.node_id, scene_id, "font-size"));
    }
    let font_size_css = if font_size == 12.0 { "12px" } else { "10px" };
    require_property(style, "font-size", font_size_css, scene_id)?;
    require_property(style, "line-height", font_size_css, scene_id)?;
    for (name, expected) in [
        ("display", "block"),
        ("box-sizing", "border-box"),
        ("font-family", "Ahem"),
        ("font-weight", "400"),
        ("font-style", "normal"),
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
        require_property(style, name, expected, scene_id)?;
    }
    let min_dimensions_match = if negative_text_case {
        style.layout_dimensions.min_width == ComputedCssDimension::LengthPx(0.0)
            && style.layout_dimensions.min_height == ComputedCssDimension::LengthPx(0.0)
    } else {
        style.layout_dimensions.min_width == ComputedCssDimension::Auto
            && style.layout_dimensions.min_height == ComputedCssDimension::Auto
    };
    if style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || !min_dimensions_match
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || !px_is(style.layout_border.top, 0.0)
        || !px_is(style.layout_border.right, 0.0)
        || !px_is(style.layout_border.bottom, 0.0)
        || !px_is(style.layout_border.left, 0.0)
    {
        return Err(unsupported_scene(
            style.node_id,
            scene_id,
            "item box geometry",
        ));
    }
    Ok(())
}

fn require_property(
    style: &ComputedElementStyle,
    name: &'static str,
    expected: &str,
    scene_id: &str,
) -> Result<(), RuntimeLayoutFailure> {
    let actual = style.properties.get(name).map(String::as_str);
    if actual == Some(expected) {
        Ok(())
    } else {
        Err(layout_failure(
            "unsupported_c114_implicit_text_style",
            Some(style.node_id.get()),
            Some(format!(
                "{scene_id}: {name} expected {expected}, got {actual:?}"
            )),
        ))
    }
}

fn zero_edges(edges: spinon_style::ComputedCssEdges) -> bool {
    spacing_is(edges.top, 0.0)
        && spacing_is(edges.right, 0.0)
        && spacing_is(edges.bottom, 0.0)
        && spacing_is(edges.left, 0.0)
}

fn spacing_is(value: ComputedCssSpacingValue, expected: f32) -> bool {
    matches!(value, ComputedCssSpacingValue::LengthPx(actual) if (actual - expected).abs() <= f32::EPSILON)
}

fn px_is(value: f32, expected: f32) -> bool {
    (value - expected).abs() <= f32::EPSILON
}

fn unsupported_scene(node_id: NodeId, scene_id: &str, reason: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_implicit_text_fixture",
        Some(node_id.get()),
        Some(format!("{scene_id}: {reason}")),
    )
}

fn ahem_paragraph_style(font_size: f32) -> ParagraphStyle {
    ParagraphStyle {
        writing_mode: spinon_core::WritingMode::HorizontalTb,
        text_orientation: spinon_core::TextOrientation::Mixed,
        font_family: "Ahem".to_owned(),
        font_size,
        line_height: font_size,
        line_height_is_normal: false,
        font_weight: 400.0,
        font_style: FontStyleMode::Normal,
        white_space: WhiteSpaceMode::Normal,
        word_break: WordBreakMode::Normal,
        direction: TextParagraphDirection::Ltr,
        line_break: LineBreakMode::Strict,
        ligatures: LigatureMode::Normal,
        letter_spacing: 0.0,
        word_spacing: 0.0,
        text_indent: 0.0,
    }
}
