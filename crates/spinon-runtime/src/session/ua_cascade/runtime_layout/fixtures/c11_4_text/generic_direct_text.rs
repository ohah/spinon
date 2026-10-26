use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, NodeId};
use spinon_layout::{
    FontStyleMode, LigatureMode, LineBreakMode, ParagraphStyle, TextLayoutEngine,
    TextParagraphDirection, WhiteSpaceMode, WordBreakMode,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackSize,
    ComputedCssMaxSize, ComputedCssPosition, ComputedCssSpacingValue, ComputedElementStyle,
    ComputedStyleProfile, ComputedStyleSnapshot,
};

use super::super::super::{RuntimeLayoutFailure, layout_failure};
use super::super::C11_4TextInput;
use super::AHEM;

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

/// 기존 고정 scene adapter 다음에 실행하는 direct element-text fixture collector입니다.
pub(super) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<C11_4TextInput>, RuntimeLayoutFailure> {
    validate_snapshot_revision(snapshot, root.id(), styles)?;
    let root_style = style_for(styles, root.id())?;
    if property(root_style, "display") != Some("grid") {
        return Ok(None);
    }

    let root_children = snapshot
        .children(root)
        .ok_or_else(|| source_failure(root.id(), "missing Grid root children"))?
        .collect::<Vec<_>>();
    let mut candidates = Vec::new();
    for child in root_children {
        let node = snapshot
            .node(child)
            .ok_or_else(|| source_failure(child.id(), "missing direct Grid child"))?;
        match node.kind() {
            HostNodeKind::Text(text) if !text.is_only_collapsible_html_whitespace() => {
                return Err(source_failure(
                    child.id(),
                    "anonymous direct Grid text is unsupported",
                ));
            }
            HostNodeKind::Element(_) if subtree_has_meaningful_text(snapshot, child)? => {
                candidates.push(child);
            }
            HostNodeKind::Element(_) | HostNodeKind::Text(_) => {}
        }
    }

    if candidates.len() < 2 {
        // Preserve the legacy one-item TEXT_CASES path, including its own validation and errors.
        return Ok(None);
    }
    if candidates.len() > 2 {
        return Err(source_failure(
            candidates[2].id(),
            "more than two direct text-item candidates",
        ));
    }

    validate_grid_root(snapshot, root, styles, root_style)?;
    let mut paragraph_styles = BTreeMap::new();
    for item in candidates {
        let text_node = direct_text_source(snapshot, root, item)?;
        let text = snapshot
            .node(text_node)
            .and_then(|node| match node.kind() {
                HostNodeKind::Text(text) => Some(text.to_string_lossy()),
                HostNodeKind::Element(_) => None,
            })
            .ok_or_else(|| source_failure(text_node.id(), "direct Text source is missing"))?;
        if !text
            .chars()
            .all(|character| character == ' ' || character.is_ascii_uppercase())
        {
            return Err(source_failure(
                text_node.id(),
                "text must contain only ASCII A-Z and U+0020",
            ));
        }
        let style = style_for(styles, item.id())?;
        let paragraph_style = paragraph_style_for(styles, style)?;
        paragraph_styles.insert(item.id(), paragraph_style);
    }

    if paragraph_styles.len() != 2 {
        return Err(source_failure(
            root.id(),
            "candidate paragraph ownership is not unique",
        ));
    }

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).map_err(|error| {
        layout_failure(
            "c114_text_font_registration",
            Some(root.id().get()),
            Some(error.to_string()),
        )
    })?;
    Ok(Some(C11_4TextInput {
        paragraph_styles,
        engine,
        two_axis_nested_grid: false,
    }))
}

fn validate_snapshot_revision(
    snapshot: &HostDocumentSnapshot,
    root: NodeId,
    styles: &ComputedStyleSnapshot,
) -> Result<(), RuntimeLayoutFailure> {
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 {
        return Err(style_failure(
            root.get(),
            "computed-style profile must be RuntimeBlockFlexGridTrackSizingV1",
        ));
    }
    if styles.generation != snapshot.generation()
        || styles.document_revision != snapshot.document_revision()
        || styles.render_tree_revision != snapshot.render_tree_revision()
    {
        return Err(style_failure(
            root.get(),
            "computed-style source revision does not match HostDocumentSnapshot",
        ));
    }
    Ok(())
}

fn validate_grid_root(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    style: &ComputedElementStyle,
) -> Result<(), RuntimeLayoutFailure> {
    let element = snapshot
        .node(root)
        .and_then(|node| match node.kind() {
            HostNodeKind::Element(element) => Some(element),
            HostNodeKind::Text(_) => None,
        })
        .ok_or_else(|| source_failure(root.id(), "Grid root is not an element"))?;
    if element.namespace() != HTML_NAMESPACE {
        return Err(source_failure(
            root.id(),
            "Grid root must use HTML namespace",
        ));
    }
    require_grid_property(style, "display", "grid")?;
    require_grid_property(style, "align-items", "start")?;
    require_grid_property(style, "align-content", "normal")?;
    require_grid_property(style, "justify-content", "normal")?;
    require_grid_property(style, "justify-items", "normal")?;
    require_grid_property(style, "direction", "ltr")?;
    if styles.writing_style_for(root.id())
        != Some((
            spinon_core::WritingMode::HorizontalTb,
            spinon_core::TextOrientation::Mixed,
        ))
    {
        return Err(grid_style_failure(style.node_id, "Grid root writing style"));
    }

    let grid = style.grid_layout.as_ref().ok_or_else(|| {
        grid_style_failure(style.node_id, "grid_layout is missing from computed style")
    })?;
    let dimensions = style.layout_dimensions;
    let spacing = style.layout_spacing;
    if grid.columns
        != [
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::MaxContent,
        ]
        || grid.rows != [ComputedCssGridTrackSize::Auto]
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || grid.auto_rows != ComputedCssGridAutoRows::Auto
        || grid.auto_columns != ComputedCssGridAutoColumns::Auto
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || grid
            .columns_line_names
            .iter()
            .any(|names| !names.is_empty())
        || grid.rows_line_names.iter().any(|names| !names.is_empty())
    {
        return Err(grid_style_failure(
            style.node_id,
            "explicit Grid track profile",
        ));
    }
    if dimensions.width != ComputedCssDimension::LengthPx(100.0)
        || dimensions.height != ComputedCssDimension::Auto
        || dimensions.min_width != ComputedCssDimension::Auto
        || dimensions.max_width != ComputedCssMaxSize::None
        || dimensions.min_height != ComputedCssDimension::Auto
        || dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_aspect_ratio.is_some()
        || style.layout_position != ComputedCssPosition::Static
        || !zero_edges(spacing.margin)
        || !zero_edges(spacing.padding)
        || (spacing.row_gap != ComputedCssSpacingValue::Normal
            && spacing.row_gap != ComputedCssSpacingValue::LengthPx(0.0))
        || spacing.column_gap != ComputedCssSpacingValue::LengthPx(10.0)
        || style.layout_border.top != 0.0
        || style.layout_border.right != 0.0
        || style.layout_border.bottom != 0.0
        || style.layout_border.left != 0.0
    {
        return Err(grid_style_failure(style.node_id, "Grid root box geometry"));
    }
    Ok(())
}

fn direct_text_source(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    item: HostNodeHandle,
) -> Result<HostNodeHandle, RuntimeLayoutFailure> {
    let element = snapshot
        .node(item)
        .and_then(|node| match node.kind() {
            HostNodeKind::Element(element) => Some(element),
            HostNodeKind::Text(_) => None,
        })
        .ok_or_else(|| source_failure(item.id(), "Grid text item is not an element"))?;
    if element.namespace() != HTML_NAMESPACE
        || snapshot.parent(item) != Some(spinon_core::HostParent::Node(root))
    {
        return Err(source_failure(
            item.id(),
            "text item must be a direct HTML element child of the Grid root",
        ));
    }

    let mut direct_texts = Vec::new();
    for child in snapshot
        .children(item)
        .ok_or_else(|| source_failure(item.id(), "missing item children"))?
    {
        let node = snapshot
            .node(child)
            .ok_or_else(|| source_failure(child.id(), "missing item child"))?;
        match node.kind() {
            HostNodeKind::Text(text) if !text.is_only_collapsible_html_whitespace() => {
                direct_texts.push(child);
            }
            HostNodeKind::Text(_) => {}
            HostNodeKind::Element(_) => {
                return Err(source_failure(
                    child.id(),
                    "descendant or replaced content in a text item is unsupported",
                ));
            }
        }
    }
    if direct_texts.len() != 1 {
        return Err(source_failure(
            item.id(),
            "each text item must own exactly one meaningful direct Text node",
        ));
    }
    Ok(direct_texts[0])
}

fn subtree_has_meaningful_text(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
) -> Result<bool, RuntimeLayoutFailure> {
    let mut pending = vec![root];
    while let Some(handle) = pending.pop() {
        let node = snapshot
            .node(handle)
            .ok_or_else(|| source_failure(handle.id(), "missing candidate node"))?;
        match node.kind() {
            HostNodeKind::Element(_) => {
                pending.extend(
                    snapshot
                        .children(handle)
                        .ok_or_else(|| source_failure(handle.id(), "missing candidate children"))?,
                );
            }
            HostNodeKind::Text(text) if !text.is_only_collapsible_html_whitespace() => {
                return Ok(true);
            }
            HostNodeKind::Text(_) => {}
        }
    }
    Ok(false)
}

fn paragraph_style_for(
    styles: &ComputedStyleSnapshot,
    style: &ComputedElementStyle,
) -> Result<ParagraphStyle, RuntimeLayoutFailure> {
    require_style_property(style, "display", "block")?;
    require_style_property(style, "position", "static")?;
    require_style_property(style, "order", "0")?;
    require_style_property(style, "align-self", "auto")?;
    require_style_property(style, "justify-self", "auto")?;
    if style.layout_position != ComputedCssPosition::Static {
        return Err(style_failure(style.node_id.get(), "position"));
    }
    require_style_property(style, "font-family", "Ahem")?;
    require_style_property(style, "font-size", "10px")?;
    require_style_property(style, "font-weight", "400")?;
    require_style_property(style, "font-style", "normal")?;
    require_style_property(style, "line-height", "10px")?;
    require_style_property(style, "white-space-collapse", "collapse")?;
    require_style_property(style, "direction", "ltr")?;
    require_style_property(style, "line-break", "auto")?;
    require_style_property(style, "text-align", "start")?;
    require_style_property(style, "overflow-wrap", "normal")?;
    require_style_property(style, "text-transform", "none")?;
    require_style_property(style, "writing-mode", "horizontal-tb")?;
    require_style_property(style, "letter-spacing", "normal")?;
    require_style_property(style, "word-spacing", "0px")?;
    require_style_property(style, "text-indent", "0px")?;

    let (writing_mode, text_orientation) = styles
        .writing_style_for(style.node_id)
        .ok_or_else(|| style_failure(style.node_id.get(), "writing-mode ownership"))?;
    if (style.font_size_css_px - 10.0).abs() > f32::EPSILON
        || writing_mode != spinon_core::WritingMode::HorizontalTb
        || text_orientation != spinon_core::TextOrientation::Mixed
    {
        return Err(style_failure(
            style.node_id.get(),
            "computed font or writing style",
        ));
    }

    let white_space = match property(style, "text-wrap-mode") {
        Some("wrap") => WhiteSpaceMode::Normal,
        Some("nowrap") => WhiteSpaceMode::NoWrap,
        _ => return Err(style_failure(style.node_id.get(), "text-wrap-mode")),
    };
    let word_break = match property(style, "word-break") {
        Some("normal") => WordBreakMode::Normal,
        Some("break-all") => WordBreakMode::BreakAll,
        _ => return Err(style_failure(style.node_id.get(), "word-break")),
    };
    if style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_aspect_ratio.is_some()
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || style.layout_border.top != 0.0
        || style.layout_border.right != 0.0
        || style.layout_border.bottom != 0.0
        || style.layout_border.left != 0.0
    {
        return Err(style_failure(
            style.node_id.get(),
            "item width/height/min/max/ratio/edges",
        ));
    }
    let placement = style
        .grid_item_placement
        .as_ref()
        .ok_or_else(|| style_failure(style.node_id.get(), "Grid item placement is missing"))?;
    if placement.row_start != ComputedCssGridPlacement::Auto
        || placement.row_end != ComputedCssGridPlacement::Auto
        || placement.column_start != ComputedCssGridPlacement::Auto
        || placement.column_end != ComputedCssGridPlacement::Auto
    {
        return Err(style_failure(style.node_id.get(), "Grid item placement"));
    }

    Ok(ParagraphStyle {
        writing_mode,
        text_orientation,
        font_family: "Ahem".to_owned(),
        font_size: style.font_size_css_px,
        line_height: 10.0,
        line_height_is_normal: false,
        font_weight: 400.0,
        font_style: FontStyleMode::Normal,
        white_space,
        word_break,
        direction: TextParagraphDirection::Ltr,
        line_break: LineBreakMode::Strict,
        ligatures: LigatureMode::Normal,
    })
}

fn style_for(
    styles: &ComputedStyleSnapshot,
    node: NodeId,
) -> Result<&ComputedElementStyle, RuntimeLayoutFailure> {
    let mut matches = styles.elements.iter().filter(|style| style.node_id == node);
    let style = matches
        .next()
        .ok_or_else(|| style_failure(node.get(), "computed style owner missing"))?;
    if matches.next().is_some() {
        return Err(style_failure(
            node.get(),
            "computed style owner is ambiguous",
        ));
    }
    Ok(style)
}

fn require_style_property(
    style: &ComputedElementStyle,
    name: &'static str,
    expected: &'static str,
) -> Result<(), RuntimeLayoutFailure> {
    if property(style, name) == Some(expected) {
        Ok(())
    } else {
        Err(style_failure(
            style.node_id.get(),
            &format!(
                "{name}: expected {expected}, got {:?}",
                property(style, name)
            ),
        ))
    }
}

fn require_grid_property(
    style: &ComputedElementStyle,
    name: &'static str,
    expected: &'static str,
) -> Result<(), RuntimeLayoutFailure> {
    if property(style, name) == Some(expected) {
        Ok(())
    } else {
        Err(grid_style_failure(
            style.node_id,
            &format!(
                "{name}: expected {expected}, got {:?}",
                property(style, name)
            ),
        ))
    }
}

fn property<'a>(style: &'a ComputedElementStyle, name: &str) -> Option<&'a str> {
    style.properties.get(name).map(String::as_str)
}

fn zero_edges(edges: spinon_style::ComputedCssEdges) -> bool {
    [edges.top, edges.right, edges.bottom, edges.left]
        .into_iter()
        .all(|value| value == ComputedCssSpacingValue::LengthPx(0.0))
}

fn source_failure(node: NodeId, detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_text_source",
        Some(node.get()),
        Some(detail.to_owned()),
    )
}

fn style_failure(node: u64, detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_text_style",
        Some(node),
        Some(detail.to_owned()),
    )
}

fn grid_style_failure(node: NodeId, detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_text_grid_style",
        Some(node.get()),
        Some(detail.to_owned()),
    )
}
