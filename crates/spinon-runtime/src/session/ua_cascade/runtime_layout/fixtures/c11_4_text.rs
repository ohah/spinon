use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent};
use spinon_layout::{
    FontStyleMode, LigatureMode, LineBreakMode, ParagraphStyle, TextLayoutEngine,
    TextParagraphDirection,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize,
    ComputedCssMath, ComputedCssMaxSize, ComputedCssSpacingValue, ComputedElementStyle,
    ComputedStyleProfile, ComputedStyleSnapshot,
};

use super::super::{RuntimeLayoutFailure, layout_failure};

const AHEM: &[u8] = include_bytes!("../../../../../../../tests/fixtures/resources/c15/Ahem.ttf");

#[path = "c11_4_text/cases.rs"]
mod cases;
use cases::{TEXT_CASES, TextCase, TextTrack};

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
    }))
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
