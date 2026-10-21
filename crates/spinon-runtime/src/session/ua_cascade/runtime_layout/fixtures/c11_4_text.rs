use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent};
use spinon_layout::{
    FontStyleMode, LigatureMode, LineBreakMode, ParagraphStyle, TextLayoutEngine,
    TextParagraphDirection, WhiteSpaceMode, WordBreakMode,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackLengthPercentage,
    ComputedCssGridTrackSize, ComputedCssMath, ComputedCssMaxSize, ComputedCssSpacingValue,
    ComputedElementStyle, ComputedStyleProfile, ComputedStyleSnapshot,
};

use super::super::{RuntimeLayoutFailure, layout_failure};

pub(super) const AHEM: &[u8] =
    include_bytes!("../../../../../../../tests/fixtures/resources/c15/Ahem.ttf");

#[path = "c11_4_text/cases.rs"]
mod cases;
#[path = "c11_4_text/flex_span.rs"]
mod flex_span;
use cases::{TEXT_CASES, TextCase, TextTrack};

pub(in crate::session::ua_cascade::runtime_layout) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<super::C11_4TextInput>, RuntimeLayoutFailure> {
    if let Some(input) = super::c11_4_nested_grid_text_multispan::prepare(snapshot, root, styles)? {
        return Ok(Some(input));
    }
    if let Some(input) = super::c11_4_nested_grid_depth_two::prepare(snapshot, root, styles)? {
        return Ok(Some(input));
    }
    if let Some(input) = super::c11_4_nested_grid_intrinsic::prepare(snapshot, root, styles)? {
        return Ok(Some(input));
    }
    if let Some(input) = super::c11_4_implicit_intrinsic_text::prepare(snapshot, root, styles)? {
        return Ok(Some(input));
    }
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 {
        return Err(layout_failure(
            "c114_text_profile_mismatch",
            Some(root.id().get()),
            None,
        ));
    }
    if let Some(input) = prepare_typed_auto_repeat_text(snapshot, root, styles)? {
        return Ok(Some(input));
    }
    if let Some(input) = prepare_multispan_text(snapshot, root, styles)? {
        return Ok(Some(input));
    }
    if let Some(input) = flex_span::prepare(snapshot, root, styles)? {
        return Ok(Some(input));
    }

    let mut pending = vec![root];
    let mut text_nodes = Vec::new();
    while let Some(handle) = pending.pop() {
        let node = snapshot
            .node(handle)
            .ok_or_else(|| layout_failure("missing_node", Some(handle.id().get()), None))?;
        if let HostNodeKind::Text(text) = node.kind() {
            if !text.is_only_collapsible_html_whitespace() {
                if !text_nodes.is_empty() {
                    return Err(layout_failure(
                        "unsupported_c114_text_node_count",
                        Some(handle.id().get()),
                        Some("2+".to_owned()),
                    ));
                }
                text_nodes.push(handle);
            }
            continue;
        }
        pending.extend(snapshot.children(handle).into_iter().flatten());
    }

    if text_nodes.is_empty() {
        return Ok(None);
    }
    let text_node = text_nodes[0];
    let text = snapshot
        .node(text_node)
        .and_then(|node| match node.kind() {
            HostNodeKind::Text(text) => Some(text.to_string_lossy()),
            HostNodeKind::Element(_) => None,
        })
        .ok_or_else(|| {
            layout_failure("missing_c114_text_node", Some(text_node.id().get()), None)
        })?;

    let item = match snapshot.parent(text_node) {
        Some(HostParent::Node(parent)) => parent,
        _ => {
            return Err(layout_failure(
                "invalid_c114_text_owner",
                Some(text_node.id().get()),
                None,
            ));
        }
    };
    let (item_tag, item_id) = element_identity(snapshot, item)?;
    let mut item_children = snapshot
        .children(item)
        .ok_or_else(|| layout_failure("missing_node", Some(item.id().get()), None))?;
    if item_tag != "div"
        || item_children.next() != Some(text_node)
        || item_children.next().is_some()
    {
        return Err(layout_failure(
            "invalid_c114_text_owner",
            Some(item.id().get()),
            Some("allowlisted direct-leaf grid item required".to_owned()),
        ));
    }

    let grid = match snapshot.parent(item) {
        Some(HostParent::Node(parent)) => parent,
        _ => {
            return Err(layout_failure(
                "invalid_c114_text_grid_owner",
                Some(item.id().get()),
                None,
            ));
        }
    };
    if grid.id() != root.id() || snapshot.parent(item) != Some(HostParent::Node(root)) {
        return Err(layout_failure(
            "invalid_c114_text_grid_owner",
            Some(grid.id().get()),
            Some("root grid direct child required".to_owned()),
        ));
    }

    let (root_tag, root_id) = element_identity(snapshot, root)?;
    let mut grid_children = snapshot
        .children(root)
        .ok_or_else(|| layout_failure("missing_node", Some(root.id().get()), None))?;
    if grid_children.next() != Some(item) || grid_children.next().is_some() {
        return Err(layout_failure(
            "invalid_c114_text_grid_owner",
            Some(root.id().get()),
            Some("one direct text item required".to_owned()),
        ));
    }
    let case = TEXT_CASES
        .iter()
        .find(|case| case.root_id == root_id)
        .ok_or_else(|| {
            layout_failure(
                "unsupported_c114_text_fixture_root",
                Some(root.id().get()),
                Some(root_id.clone()),
            )
        })?;
    if root_tag != "div" || case.item_id != item_id || case.content != text {
        return Err(layout_failure(
            "unsupported_c114_text_fixture_case",
            Some(item.id().get()),
            Some(case.scene_id.to_owned()),
        ));
    }

    let grid_style = styles
        .elements
        .iter()
        .find(|style| style.node_id == grid.id())
        .ok_or_else(|| layout_failure("missing_computed_style", Some(grid.id().get()), None))?;
    validate_grid_style(grid_style, case)?;

    let item_style = styles
        .elements
        .iter()
        .find(|style| style.node_id == item.id())
        .ok_or_else(|| layout_failure("missing_computed_style", Some(item.id().get()), None))?;
    validate_text_style(item_style, case)?;

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).map_err(|error| {
        layout_failure(
            "c114_text_font_registration",
            Some(item.id().get()),
            Some(error.to_string()),
        )
    })?;
    Ok(Some(super::C11_4TextInput {
        paragraph_styles: BTreeMap::from([(item.id(), ahem_paragraph_style(case))]),
        engine,
        two_axis_nested_grid: false,
    }))
}

#[derive(Clone, Copy)]
struct MultispanTextItem {
    text: &'static str,
    column_start: u8,
    column_span: u8,
    row: u8,
}

#[derive(Clone, Copy)]
struct MultispanTextScene {
    id: &'static str,
    items: &'static [MultispanTextItem],
}

const MULTISPAN_TEXT_A: MultispanTextItem = MultispanTextItem {
    text: "XX XXXXX",
    column_start: 1,
    column_span: 2,
    row: 1,
};
const MULTISPAN_TEXT_B: MultispanTextItem = MultispanTextItem {
    text: "XXXX XXXXXXX",
    column_start: 2,
    column_span: 2,
    row: 2,
};
const MULTISPAN_TEXT_C: MultispanTextItem = MultispanTextItem {
    text: "XXXXXXXXXXXXXXXXXX",
    column_start: 1,
    column_span: 3,
    row: 3,
};

const MULTISPAN_TEXT_SCENES: [MultispanTextScene; 4] = [
    MultispanTextScene {
        id: "text-span2-forward",
        items: &[MULTISPAN_TEXT_A, MULTISPAN_TEXT_B],
    },
    MultispanTextScene {
        id: "text-span2-reversed",
        items: &[MULTISPAN_TEXT_B, MULTISPAN_TEXT_A],
    },
    MultispanTextScene {
        id: "text-span2-before-span3",
        items: &[MULTISPAN_TEXT_A, MULTISPAN_TEXT_B, MULTISPAN_TEXT_C],
    },
    MultispanTextScene {
        id: "text-span3-before-span2",
        items: &[MULTISPAN_TEXT_C, MULTISPAN_TEXT_B, MULTISPAN_TEXT_A],
    },
];

fn prepare_multispan_text(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<super::C11_4TextInput>, RuntimeLayoutFailure> {
    let (root_tag, root_id) = element_identity(snapshot, root)?;
    let Some(scene_id) = root_id
        .strip_prefix("c114-")
        .and_then(|id| id.strip_suffix("-grid"))
    else {
        return Ok(None);
    };
    let Some(scene) = MULTISPAN_TEXT_SCENES
        .iter()
        .find(|scene| scene.id == scene_id)
        .copied()
    else {
        return Ok(None);
    };
    if root_tag != "div" {
        return Err(layout_failure(
            "unsupported_c114_text_multispan_root",
            Some(root.id().get()),
            Some(root_tag),
        ));
    }

    let item_handles = snapshot
        .children(root)
        .ok_or_else(|| layout_failure("missing_node", Some(root.id().get()), None))?
        .collect::<Vec<_>>();
    if item_handles.len() != scene.items.len() {
        return Err(layout_failure(
            "unsupported_c114_text_multispan_items",
            Some(root.id().get()),
            Some(format!(
                "expected={} actual={}",
                scene.items.len(),
                item_handles.len()
            )),
        ));
    }

    let grid_style = styles
        .elements
        .iter()
        .find(|style| style.node_id == root.id())
        .ok_or_else(|| layout_failure("missing_computed_style", Some(root.id().get()), None))?;
    validate_multispan_text_grid_style(grid_style, scene.id)?;

    let mut paragraph_styles = BTreeMap::new();
    for (index, (item, expected)) in item_handles.iter().zip(scene.items).enumerate() {
        let (item_tag, item_id) = element_identity(snapshot, *item)?;
        let expected_item_id = format!("c114-{scene_id}-item-{}", index + 1);
        if item_tag != "div"
            || item_id != expected_item_id
            || snapshot.parent(*item) != Some(HostParent::Node(root))
        {
            return Err(layout_failure(
                "unsupported_c114_text_multispan_item",
                Some(item.id().get()),
                Some(format!(
                    "expected={expected_item_id} actual={item_tag}#{item_id}"
                )),
            ));
        }

        let text_nodes = snapshot
            .children(*item)
            .ok_or_else(|| layout_failure("missing_node", Some(item.id().get()), None))?
            .collect::<Vec<_>>();
        if text_nodes.len() != 1 {
            return Err(layout_failure(
                "unsupported_c114_text_multispan_topology",
                Some(item.id().get()),
                Some(format!(
                    "expected one direct text child, got {}",
                    text_nodes.len()
                )),
            ));
        }
        let text_node = text_nodes[0];
        let text = snapshot
            .node(text_node)
            .and_then(|node| match node.kind() {
                HostNodeKind::Text(text) => Some(text.to_string_lossy()),
                HostNodeKind::Element(_) => None,
            })
            .ok_or_else(|| {
                layout_failure(
                    "unsupported_c114_text_multispan_topology",
                    Some(text_node.id().get()),
                    Some("direct text child required".to_owned()),
                )
            })?;
        let item_style = styles
            .elements
            .iter()
            .find(|style| style.node_id == item.id())
            .ok_or_else(|| layout_failure("missing_computed_style", Some(item.id().get()), None))?;
        let Some(placement) = item_style.grid_item_placement.as_ref() else {
            return Err(layout_failure(
                "unsupported_c114_text_multispan_case",
                Some(item.id().get()),
                Some("computed grid placement required".to_owned()),
            ));
        };
        if text != expected.text
            || snapshot.parent(text_node) != Some(HostParent::Node(*item))
            || placement.column_start
                != ComputedCssGridPlacement::Line(expected.column_start as i32)
            || placement.column_end != ComputedCssGridPlacement::Span(expected.column_span as i32)
            || placement.row_start != ComputedCssGridPlacement::Line(expected.row as i32)
            || placement.row_end != ComputedCssGridPlacement::Auto
        {
            return Err(layout_failure(
                "unsupported_c114_text_multispan_case",
                Some(item.id().get()),
                Some(format!(
                    "{} item {} topology, text, or placement differs",
                    scene.id,
                    index + 1
                )),
            ));
        }
        validate_multispan_text_item_style(item_style)?;
        paragraph_styles.insert(item.id(), ahem_paragraph_style(&TEXT_CASES[0]));
    }

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).map_err(|error| {
        layout_failure(
            "c114_text_multispan_font_registration",
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

fn validate_multispan_text_grid_style(
    style: &ComputedElementStyle,
    scene_id: &str,
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
    let grid = style.grid_layout.as_ref().ok_or_else(|| {
        layout_failure(
            "missing_c114_text_grid_layout",
            Some(style.node_id.get()),
            Some(scene_id.to_owned()),
        )
    })?;
    if style.layout_dimensions.width != ComputedCssDimension::LengthPx(220.0)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(60.0)
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || !zero_spacing(style.layout_spacing.row_gap)
        || !zero_spacing(style.layout_spacing.column_gap)
        || style.layout_border != Default::default()
        || grid.columns != vec![ComputedCssGridTrackSize::MinContent; 4]
        || grid.rows != vec![ComputedCssGridTrackSize::LengthPx(20.0); 3]
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || grid.auto_rows != ComputedCssGridAutoRows::Auto
        || grid.auto_columns != ComputedCssGridAutoColumns::Auto
        || grid.columns_line_names.len() != 5
        || grid.rows_line_names.len() != 4
        || grid
            .columns_line_names
            .iter()
            .any(|names| !names.is_empty())
        || grid.rows_line_names.iter().any(|names| !names.is_empty())
    {
        return Err(layout_failure(
            "unsupported_c114_text_grid_style",
            Some(style.node_id.get()),
            Some(format!(
                "{scene_id}: fixed 4x3 min-content text matrix required"
            )),
        ));
    }
    Ok(())
}

fn validate_multispan_text_item_style(
    style: &ComputedElementStyle,
) -> Result<(), RuntimeLayoutFailure> {
    if (style.font_size_css_px - 10.0).abs() > f32::EPSILON {
        return unsupported_text_style(style.node_id.get(), "multi-span font-size");
    }
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
        require_property(style, name, expected)?;
    }
    if style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || style.layout_border != Default::default()
    {
        return unsupported_text_style(style.node_id.get(), "multi-span item box geometry");
    }
    Ok(())
}

fn prepare_typed_auto_repeat_text(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<super::C11_4TextInput>, RuntimeLayoutFailure> {
    let (root_tag, root_id) = element_identity(snapshot, root)?;
    let Some(scene_id) = root_id
        .strip_prefix("c114-")
        .and_then(|id| id.strip_suffix("-grid"))
    else {
        return Ok(None);
    };
    let Some((expected_item_count, text_item_index)) = typed_auto_repeat_text_case(scene_id) else {
        return Ok(None);
    };
    if root_tag != "div" {
        return Err(layout_failure(
            "unsupported_c114_typed_auto_repeat_root",
            Some(root.id().get()),
            Some(root_tag),
        ));
    }

    let item_handles = snapshot
        .children(root)
        .ok_or_else(|| layout_failure("missing_node", Some(root.id().get()), None))?
        .collect::<Vec<_>>();
    if item_handles.len() != expected_item_count {
        return Err(layout_failure(
            "unsupported_c114_typed_auto_repeat_items",
            Some(root.id().get()),
            Some(format!(
                "expected={expected_item_count} actual={}",
                item_handles.len()
            )),
        ));
    }

    let mut text_owner = None;
    for (index, item) in item_handles.iter().enumerate() {
        let (tag, item_id) = element_identity(snapshot, *item)?;
        let expected_id = format!("c114-{scene_id}-item-{}", index + 1);
        if tag != "div"
            || item_id != expected_id
            || snapshot.parent(*item) != Some(HostParent::Node(root))
        {
            return Err(layout_failure(
                "unsupported_c114_typed_auto_repeat_item",
                Some(item.id().get()),
                Some(format!("expected={expected_id} actual={tag}#{item_id}")),
            ));
        }

        let children = snapshot
            .children(*item)
            .ok_or_else(|| layout_failure("missing_node", Some(item.id().get()), None))?
            .collect::<Vec<_>>();
        if index == text_item_index {
            if children.len() != 1 {
                return Err(layout_failure(
                    "unsupported_c114_typed_auto_repeat_text_node",
                    Some(item.id().get()),
                    Some(format!(
                        "expected one direct text node, got {}",
                        children.len()
                    )),
                ));
            }
            let text_handle = children[0];
            let text = snapshot
                .node(text_handle)
                .and_then(|node| match node.kind() {
                    HostNodeKind::Text(text) => Some(text.to_string_lossy()),
                    HostNodeKind::Element(_) => None,
                });
            if snapshot.parent(text_handle) != Some(HostParent::Node(*item))
                || text.as_deref() != Some("aa bbbb c")
            {
                return Err(layout_failure(
                    "unsupported_c114_typed_auto_repeat_text_content",
                    Some(text_handle.id().get()),
                    Some(text.unwrap_or_else(|| "not a text node".to_owned())),
                ));
            }
            let item_style = styles
                .elements
                .iter()
                .find(|style| style.node_id == item.id())
                .ok_or_else(|| {
                    layout_failure("missing_computed_style", Some(item.id().get()), None)
                })?;
            validate_typed_auto_repeat_text_style(
                item_style,
                !matches!(scene_id, "typed-mixed-cycle" | "typed-minmax-max-content"),
            )?;
            text_owner = Some(*item);
        } else if !children.is_empty() {
            return Err(layout_failure(
                "unsupported_c114_typed_auto_repeat_text_owner",
                Some(item.id().get()),
                Some("unexpected child under non-text item".to_owned()),
            ));
        }
    }

    let text_owner = text_owner.ok_or_else(|| {
        layout_failure(
            "missing_c114_typed_auto_repeat_text_owner",
            Some(root.id().get()),
            Some(scene_id.to_owned()),
        )
    })?;
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).map_err(|error| {
        layout_failure(
            "c114_typed_auto_repeat_font_registration",
            Some(text_owner.id().get()),
            Some(error.to_string()),
        )
    })?;
    Ok(Some(super::C11_4TextInput {
        paragraph_styles: BTreeMap::from([(text_owner.id(), typed_auto_repeat_paragraph_style())]),
        engine,
        two_axis_nested_grid: false,
    }))
}

fn typed_auto_repeat_text_case(scene_id: &str) -> Option<(usize, usize)> {
    match scene_id {
        "typed-mixed-cycle" => Some((3, 0)),
        "typed-auto"
        | "typed-min-content"
        | "typed-max-content"
        | "typed-minmax-max-content"
        | "typed-fit-content" => Some((1, 0)),
        "typed-auto-fit-occupied" | "typed-auto-fit-gutter" => Some((3, 1)),
        _ => None,
    }
}

fn validate_typed_auto_repeat_text_style(
    style: &ComputedElementStyle,
    min_height_auto: bool,
) -> Result<(), RuntimeLayoutFailure> {
    if (style.font_size_css_px - 12.0).abs() > f32::EPSILON {
        return unsupported_text_style(style.node_id.get(), "typed auto-repeat font-size");
    }
    for (name, expected) in [
        ("display", "block"),
        ("box-sizing", "border-box"),
        ("font-family", "Ahem"),
        ("font-size", "12px"),
        ("font-weight", "400"),
        ("font-style", "normal"),
        ("line-height", "12px"),
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
    if style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height
            != if min_height_auto {
                ComputedCssDimension::Auto
            } else {
                ComputedCssDimension::LengthPx(0.0)
            }
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || style.layout_border.top != 0.0
        || style.layout_border.right != 0.0
        || style.layout_border.bottom != 0.0
        || style.layout_border.left != 0.0
    {
        return unsupported_text_style(style.node_id.get(), "typed auto-repeat text box geometry");
    }
    Ok(())
}

fn typed_auto_repeat_paragraph_style() -> ParagraphStyle {
    ParagraphStyle {
        font_family: "Ahem".to_owned(),
        font_size: 12.0,
        line_height: 12.0,
        line_height_is_normal: false,
        font_weight: 400.0,
        font_style: FontStyleMode::Normal,
        white_space: WhiteSpaceMode::Normal,
        word_break: WordBreakMode::Normal,
        direction: TextParagraphDirection::Ltr,
        line_break: LineBreakMode::Strict,
        ligatures: LigatureMode::Normal,
    }
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
            "invalid_c114_text_owner",
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

fn validate_grid_style(
    style: &ComputedElementStyle,
    case: &TextCase,
) -> Result<(), RuntimeLayoutFailure> {
    require_property(style, "display", "grid")?;
    require_property(style, "box-sizing", "border-box")?;
    require_property(style, "align-items", "start")?;
    require_property(style, "align-content", "normal")?;
    if style.layout_dimensions.width != ComputedCssDimension::LengthPx(case.grid_width)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(case.grid_height)
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || !zero_spacing(style.layout_spacing.row_gap)
        || !zero_spacing(style.layout_spacing.column_gap)
        || style.layout_border != Default::default()
    {
        return unsupported_grid_style(style.node_id.get(), case, "container box geometry");
    }

    let grid = style.grid_layout.as_ref().ok_or_else(|| {
        layout_failure(
            "missing_c114_text_grid_layout",
            Some(style.node_id.get()),
            Some(case.scene_id.to_owned()),
        )
    })?;
    if grid.columns.len() != 1
        || !track_matches(&grid.columns[0], case.track)
        || grid.rows != [ComputedCssGridTrackSize::Auto]
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || grid
            .columns_line_names
            .iter()
            .any(|names| !names.is_empty())
        || grid.rows_line_names.iter().any(|names| !names.is_empty())
    {
        return unsupported_grid_style(style.node_id.get(), case, "track functions");
    }
    Ok(())
}

fn track_matches(track: &ComputedCssGridTrackSize, expected: TextTrack) -> bool {
    match (track, expected) {
        (ComputedCssGridTrackSize::Auto, TextTrack::Auto)
        | (ComputedCssGridTrackSize::MinContent, TextTrack::MinContent)
        | (ComputedCssGridTrackSize::MaxContent, TextTrack::MaxContent) => true,
        (
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
                actual,
            )),
            TextTrack::FitContent40,
        ) => (*actual - 40.0).abs() <= f32::EPSILON,
        (
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::Calc(
                ComputedCssMath::Sum(terms),
            )),
            TextTrack::FitContentCalc,
        ) => {
            let percentage = terms.iter().find_map(|term| match term {
                ComputedCssMath::Percentage(value) => Some(*value),
                _ => None,
            });
            let length = terms.iter().find_map(|term| match term {
                ComputedCssMath::LengthPx(value) => Some(*value),
                _ => None,
            });
            terms.len() == 2
                && percentage.is_some_and(|value| (value - 0.1).abs() <= f32::EPSILON)
                && length.is_some_and(|value| (value - 5.0).abs() <= f32::EPSILON)
        }
        _ => false,
    }
}

fn validate_text_style(
    style: &ComputedElementStyle,
    case: &TextCase,
) -> Result<(), RuntimeLayoutFailure> {
    if (style.font_size_css_px - 10.0).abs() > f32::EPSILON {
        return unsupported_text_style(style.node_id.get(), "font-size");
    }
    for (name, expected) in [
        ("display", "block"),
        ("box-sizing", case.box_sizing),
        ("font-family", "Ahem"),
        ("font-size", "10px"),
        ("font-weight", "400"),
        ("font-style", "normal"),
        ("line-height", "10px"),
        ("white-space-collapse", "collapse"),
        ("text-wrap-mode", case.white_space_mode),
        ("word-break", case.word_break_mode),
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

    let expected_min_width = case
        .min_width
        .map(ComputedCssDimension::LengthPx)
        .unwrap_or(ComputedCssDimension::Auto);
    let expected_max_width = case
        .max_width
        .map(ComputedCssMaxSize::LengthPx)
        .unwrap_or(ComputedCssMaxSize::None);
    if style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_width != expected_min_width
        || style.layout_dimensions.max_width != expected_max_width
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_edges(style.layout_spacing.margin)
        || !expected_padding(style, case.inline_padding)
        || style.layout_border.top != 0.0
        || style.layout_border.bottom != 0.0
        || style.layout_border.left != case.inline_border
        || style.layout_border.right != case.inline_border
    {
        return unsupported_text_style(
            style.node_id.get(),
            &format!(
                "box geometry or min/max width: dimensions={:?}, spacing={:?}, border={:?}",
                style.layout_dimensions, style.layout_spacing, style.layout_border
            ),
        );
    }
    Ok(())
}

fn expected_padding(style: &ComputedElementStyle, inline: f32) -> bool {
    let padding = style.layout_spacing.padding;
    spacing_is(padding.top, 0.0)
        && spacing_is(padding.right, inline)
        && spacing_is(padding.bottom, 0.0)
        && spacing_is(padding.left, inline)
}

fn zero_edges(edges: spinon_style::ComputedCssEdges) -> bool {
    spacing_is(edges.top, 0.0)
        && spacing_is(edges.right, 0.0)
        && spacing_is(edges.bottom, 0.0)
        && spacing_is(edges.left, 0.0)
}

fn zero_spacing(value: ComputedCssSpacingValue) -> bool {
    spacing_is(value, 0.0)
}

fn spacing_is(value: ComputedCssSpacingValue, expected: f32) -> bool {
    matches!(value, ComputedCssSpacingValue::LengthPx(actual) if (actual - expected).abs() <= f32::EPSILON)
}

fn unsupported_grid_style<T>(
    node: u64,
    case: &TextCase,
    detail: &str,
) -> Result<T, RuntimeLayoutFailure> {
    Err(layout_failure(
        "unsupported_c114_text_grid_style",
        Some(node),
        Some(format!("{}: {detail}", case.scene_id)),
    ))
}

fn unsupported_text_style<T>(node: u64, detail: &str) -> Result<T, RuntimeLayoutFailure> {
    Err(layout_failure(
        "unsupported_c114_text_style",
        Some(node),
        Some(detail.to_owned()),
    ))
}

fn require_property(
    style: &ComputedElementStyle,
    name: &'static str,
    expected: &str,
) -> Result<(), RuntimeLayoutFailure> {
    let value = style.properties.get(name).map(String::as_str);
    if value == Some(expected) {
        Ok(())
    } else {
        Err(layout_failure(
            "unsupported_c114_text_style",
            Some(style.node_id.get()),
            Some(format!("{name}: expected {expected}, got {value:?}")),
        ))
    }
}

fn ahem_paragraph_style(case: &TextCase) -> ParagraphStyle {
    ParagraphStyle {
        font_family: "Ahem".to_owned(),
        font_size: 10.0,
        line_height: 10.0,
        line_height_is_normal: false,
        font_weight: 400.0,
        font_style: FontStyleMode::Normal,
        white_space: case.white_space,
        word_break: case.word_break,
        direction: TextParagraphDirection::Ltr,
        line_break: LineBreakMode::Strict,
        ligatures: LigatureMode::Normal,
    }
}

#[cfg(test)]
mod tests {
    use super::{TEXT_CASES, TextTrack};
    use std::collections::BTreeSet;

    #[test]
    fn fixed_text_matrix_has_ten_unique_direct_leaf_cases() {
        let scene_ids = TEXT_CASES
            .iter()
            .map(|case| case.scene_id)
            .collect::<BTreeSet<_>>();
        let root_ids = TEXT_CASES
            .iter()
            .map(|case| case.root_id)
            .collect::<BTreeSet<_>>();
        let item_ids = TEXT_CASES
            .iter()
            .map(|case| case.item_id)
            .collect::<BTreeSet<_>>();

        assert_eq!(TEXT_CASES.len(), 10);
        assert_eq!(scene_ids.len(), TEXT_CASES.len());
        assert_eq!(root_ids.len(), TEXT_CASES.len());
        assert_eq!(item_ids.len(), TEXT_CASES.len());
        assert!(
            TEXT_CASES
                .iter()
                .any(|case| matches!(case.track, TextTrack::FitContentCalc))
        );
        assert!(TEXT_CASES.iter().any(|case| case.min_width == Some(90.0)));
        assert!(TEXT_CASES.iter().any(|case| case.max_width == Some(40.0)));
    }
}
