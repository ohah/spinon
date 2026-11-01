use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent};
use spinon_layout::{
    FontStyleMode, LigatureMode, LineBreakMode, ParagraphStyle, TextLayoutEngine,
    TextParagraphDirection, WhiteSpaceMode, WordBreakMode,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackBreadth,
    ComputedCssGridTrackSize, ComputedCssMaxSize, ComputedCssSpacingValue, ComputedStyleProfile,
    ComputedStyleSnapshot,
};

use super::{AHEM, RuntimeLayoutFailure, layout_failure};

const SCENE_ID: &str = "spanning-auto-min-flex-tracks";
const ROOT_ID: &str = "c114-spanning-auto-min-flex-tracks-grid";
const ITEM_ID: &str = "c114-spanning-auto-min-flex-tracks-item";
const TEXT: &str = "XXXXXXXXXXXXXXXXXXXX";

pub(super) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<super::super::C11_4TextInput>, RuntimeLayoutFailure> {
    let (root_tag, root_id) = super::element_identity(snapshot, root)?;
    if root_id != ROOT_ID {
        return Ok(None);
    }
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || root_tag != "div"
    {
        return Err(layout_failure(
            "unsupported_c114_flex_span_root",
            Some(root.id().get()),
            Some(format!(
                "{SCENE_ID}: expected the fixed Grid profile and a div root"
            )),
        ));
    }

    let item_handles = snapshot
        .children(root)
        .ok_or_else(|| layout_failure("missing_node", Some(root.id().get()), None))?
        .collect::<Vec<_>>();
    if item_handles.len() != 1 {
        return Err(layout_failure(
            "unsupported_c114_flex_span_items",
            Some(root.id().get()),
            Some(format!("{SCENE_ID}: expected one direct Grid item")),
        ));
    }
    let item = item_handles[0];
    let (item_tag, item_id) = super::element_identity(snapshot, item)?;
    if item_tag != "div"
        || item_id != ITEM_ID
        || snapshot.parent(item) != Some(HostParent::Node(root))
    {
        return Err(layout_failure(
            "unsupported_c114_flex_span_item",
            Some(item.id().get()),
            Some(format!(
                "{SCENE_ID}: expected the allowlisted direct-leaf item"
            )),
        ));
    }

    let text_nodes = snapshot
        .children(item)
        .ok_or_else(|| layout_failure("missing_node", Some(item.id().get()), None))?
        .collect::<Vec<_>>();
    if text_nodes.len() != 1 {
        return Err(layout_failure(
            "unsupported_c114_flex_span_text",
            Some(item.id().get()),
            Some(format!("{SCENE_ID}: expected one direct text node")),
        ));
    }
    let text_node = text_nodes[0];
    let actual_text = snapshot
        .node(text_node)
        .and_then(|node| match node.kind() {
            HostNodeKind::Text(text) => Some(text.to_string_lossy()),
            HostNodeKind::Element(_) => None,
        })
        .ok_or_else(|| {
            layout_failure(
                "unsupported_c114_flex_span_text",
                Some(text_node.id().get()),
                Some("direct text child required".to_owned()),
            )
        })?;
    if actual_text != TEXT || snapshot.parent(text_node) != Some(HostParent::Node(item)) {
        return Err(layout_failure(
            "unsupported_c114_flex_span_text",
            Some(text_node.id().get()),
            Some(format!(
                "{SCENE_ID}: text or parent differs from the fixed oracle input"
            )),
        ));
    }

    let root_style = styles
        .elements
        .iter()
        .find(|style| style.node_id == root.id())
        .ok_or_else(|| layout_failure("missing_computed_style", Some(root.id().get()), None))?;
    validate_grid_style(root_style)?;

    let item_style = styles
        .elements
        .iter()
        .find(|style| style.node_id == item.id())
        .ok_or_else(|| layout_failure("missing_computed_style", Some(item.id().get()), None))?;
    validate_item_style(item_style)?;

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).map_err(|error| {
        layout_failure(
            "c114_flex_span_font_registration",
            Some(item.id().get()),
            Some(error.to_string()),
        )
    })?;

    Ok(Some(super::super::C11_4TextInput {
        paragraph_styles: BTreeMap::from([(
            item.id(),
            ParagraphStyle {
                overflow_wrap: Default::default(),
                writing_mode: spinon_core::WritingMode::HorizontalTb,
                text_orientation: spinon_core::TextOrientation::Mixed,
                font_family: "Ahem".to_owned(),
                font_size: 10.0,
                line_height: 10.0,
                line_height_is_normal: false,
                font_weight: 400.0,
                font_style: FontStyleMode::Normal,
                white_space: WhiteSpaceMode::NoWrap,
                word_break: WordBreakMode::Normal,
                hyphens: None,
                locale: None,
                direction: TextParagraphDirection::Ltr,
                line_break: LineBreakMode::Strict,
                ligatures: LigatureMode::Normal,
                letter_spacing: 0.0,
                word_spacing: 0.0,
                text_indent: 0.0,
                text_indent_flags: spinon_layout::TextIndentFlags::default(),
                text_transform: Default::default(),
            },
        )]),
        engine,
        two_axis_nested_grid: false,
        content_box_ratio_node: None,
        aspect_ratio_minmax_node: None,
        definite_axis_ratio_node: None,
    }))
}

fn validate_grid_style(
    style: &spinon_style::ComputedElementStyle,
) -> Result<(), RuntimeLayoutFailure> {
    for (name, expected) in [
        ("display", "grid"),
        ("box-sizing", "border-box"),
        ("align-items", "start"),
        ("align-content", "start"),
        ("justify-content", "start"),
    ] {
        super::require_property(style, name, expected)?;
    }
    let grid = style.grid_layout.as_ref().ok_or_else(|| {
        layout_failure(
            "missing_c114_flex_span_grid_layout",
            Some(style.node_id.get()),
            Some(SCENE_ID.to_owned()),
        )
    })?;
    let expected_track = ComputedCssGridTrackSize::MinMax {
        min: ComputedCssGridTrackBreadth::LengthPx(0.0),
        max: ComputedCssGridTrackBreadth::Fraction(1.0),
    };
    if style.layout_dimensions.width != ComputedCssDimension::LengthPx(100.0)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(30.0)
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || !super::zero_edges(style.layout_spacing.margin)
        || !super::zero_edges(style.layout_spacing.padding)
        || !super::zero_spacing(style.layout_spacing.row_gap)
        || !matches!(style.layout_spacing.column_gap, ComputedCssSpacingValue::LengthPx(value) if (value - 4.0).abs() <= f32::EPSILON)
        || style.layout_border != Default::default()
        || grid.columns != [expected_track.clone(), expected_track]
        || grid.rows != [ComputedCssGridTrackSize::Auto]
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || grid.auto_rows != ComputedCssGridAutoRows::Auto
        || grid.auto_columns != ComputedCssGridAutoColumns::Auto
        || grid.columns_line_names.len() != 3
        || grid.rows_line_names.len() != 2
        || grid
            .columns_line_names
            .iter()
            .any(|names| !names.is_empty())
        || grid.rows_line_names.iter().any(|names| !names.is_empty())
    {
        return Err(layout_failure(
            "unsupported_c114_flex_span_grid_style",
            Some(style.node_id.get()),
            Some(format!("{SCENE_ID}: fixed 2x1 minmax(0,1fr) grid required")),
        ));
    }
    Ok(())
}

fn validate_item_style(
    style: &spinon_style::ComputedElementStyle,
) -> Result<(), RuntimeLayoutFailure> {
    if (style.font_size_css_px - 10.0).abs() > f32::EPSILON {
        return super::unsupported_text_style(style.node_id.get(), "flex-span font-size");
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
        ("text-wrap-mode", "nowrap"),
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
        super::require_property(style, name, expected)?;
    }
    let placement = style.grid_item_placement.as_ref().ok_or_else(|| {
        layout_failure(
            "missing_c114_flex_span_placement",
            Some(style.node_id.get()),
            Some(SCENE_ID.to_owned()),
        )
    })?;
    if placement.column_start != ComputedCssGridPlacement::Line(1)
        || placement.column_end != ComputedCssGridPlacement::Span(2)
        || placement.row_start != ComputedCssGridPlacement::Auto
        || placement.row_end != ComputedCssGridPlacement::Auto
        || style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || !super::zero_edges(style.layout_spacing.margin)
        || !super::zero_edges(style.layout_spacing.padding)
        || style.layout_border != Default::default()
    {
        return super::unsupported_text_style(
            style.node_id.get(),
            "flex-span item geometry or placement",
        );
    }
    Ok(())
}
