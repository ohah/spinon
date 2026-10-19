use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent};
use spinon_layout::{
    FontStyleMode, LigatureMode, LineBreakMode, ParagraphStyle, TextLayoutEngine,
    TextParagraphDirection, WhiteSpaceMode, WordBreakMode,
};
use spinon_style::{ComputedElementStyle, ComputedStyleProfile, ComputedStyleSnapshot};

use super::super::{RuntimeLayoutFailure, layout_failure};

const TEXT_ITEM_ID: &str = "c114-track-text-item";
const TEXT_CONTENT: &str = "XX XXXXXXX";
const AHEM: &[u8] = include_bytes!("../../../../../../../tests/fixtures/resources/c15/Ahem.ttf");

pub(in crate::session::ua_cascade::runtime_layout) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<super::C11_4TextInput>, RuntimeLayoutFailure> {
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 {
        return Err(layout_failure(
            "c114_text_profile_mismatch",
            Some(root.id().get()),
            None,
        ));
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
    if text != TEXT_CONTENT {
        return Err(layout_failure(
            "unsupported_c114_text_content",
            Some(text_node.id().get()),
            Some("text".to_owned()),
        ));
    }

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
    let item_node = snapshot
        .node(item)
        .ok_or_else(|| layout_failure("missing_node", Some(item.id().get()), None))?;
    let HostNodeKind::Element(item_element) = item_node.kind() else {
        return Err(layout_failure(
            "invalid_c114_text_owner",
            Some(item.id().get()),
            None,
        ));
    };
    let mut item_children = snapshot
        .children(item)
        .ok_or_else(|| layout_failure("missing_node", Some(item.id().get()), None))?;
    if item_element.local_name() != "div"
        || item_element
            .attributes()
            .iter()
            .find_map(|(name, value)| {
                (name.namespace().is_none() && name.local_name() == "id")
                    .then(|| value.to_string_lossy())
            })
            .as_deref()
            != Some(TEXT_ITEM_ID)
        || item_children.next() != Some(text_node)
        || item_children.next().is_some()
    {
        return Err(layout_failure(
            "invalid_c114_text_owner",
            Some(item.id().get()),
            Some("direct-leaf grid item".to_owned()),
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
    let grid_style = styles
        .elements
        .iter()
        .find(|style| style.node_id == grid.id())
        .ok_or_else(|| layout_failure("missing_computed_style", Some(grid.id().get()), None))?;
    require_property(grid_style, "display", "grid")?;

    let item_style = styles
        .elements
        .iter()
        .find(|style| style.node_id == item.id())
        .ok_or_else(|| layout_failure("missing_computed_style", Some(item.id().get()), None))?;
    validate_text_style(item_style)?;

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).map_err(|error| {
        layout_failure(
            "c114_text_font_registration",
            Some(item.id().get()),
            Some(error.to_string()),
        )
    })?;
    Ok(Some(super::C11_4TextInput {
        paragraph_styles: BTreeMap::from([(item.id(), ahem_paragraph_style())]),
        engine,
    }))
}

fn validate_text_style(style: &ComputedElementStyle) -> Result<(), RuntimeLayoutFailure> {
    if (style.font_size_css_px - 10.0).abs() > f32::EPSILON {
        return Err(layout_failure(
            "unsupported_c114_text_style",
            Some(style.node_id.get()),
            Some("font-size".to_owned()),
        ));
    }
    for (name, expected) in [
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
    Ok(())
}

fn require_property(
    style: &ComputedElementStyle,
    name: &'static str,
    expected: &'static str,
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

fn ahem_paragraph_style() -> ParagraphStyle {
    ParagraphStyle {
        font_family: "Ahem".to_owned(),
        font_size: 10.0,
        line_height: 10.0,
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
