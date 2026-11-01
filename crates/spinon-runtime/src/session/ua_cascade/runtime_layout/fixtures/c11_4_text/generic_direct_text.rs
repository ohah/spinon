use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, NodeId};
use spinon_layout::{
    FontStyleMode, LigatureMode, LineBreakMode, ParagraphStyle, TextLayoutEngine,
    TextParagraphDirection, WhiteSpaceMode, WordBreakMode,
};
use spinon_style::{
    ComputedAspectRatioSizingBox, ComputedCssDimension, ComputedCssGridAutoColumns,
    ComputedCssGridAutoFlow, ComputedCssGridAutoRows, ComputedCssGridPlacement,
    ComputedCssGridTrackSize, ComputedCssMaxSize, ComputedCssPosition, ComputedCssSpacingValue,
    ComputedElementStyle, ComputedStyleProfile, ComputedStyleSnapshot,
};

use super::super::super::{RuntimeLayoutFailure, layout_failure};
use super::super::C11_4TextInput;
use super::AHEM;

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

#[path = "generic_direct_text/aspect_ratio_definite_axis.rs"]
mod aspect_ratio_definite_axis;
#[path = "generic_direct_text/aspect_ratio_multispan.rs"]
mod aspect_ratio_multispan;

#[derive(Clone, Copy, Eq, PartialEq)]
enum DirectTextProfile {
    Intrinsic,
    AspectRatio,
    AspectRatioMinMax,
}

/// 기존 고정 scene adapter 다음에 실행하는 direct element-text fixture collector입니다.
pub(super) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<C11_4TextInput>, RuntimeLayoutFailure> {
    prepare_with_profile(snapshot, root, styles, DirectTextProfile::Intrinsic)
}

/// Ahem direct text의 좁은 preferred aspect-ratio fixture profile입니다.
pub(super) fn prepare_aspect_ratio(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<C11_4TextInput>, RuntimeLayoutFailure> {
    prepare_with_profile(snapshot, root, styles, DirectTextProfile::AspectRatio)
}

/// Ahem direct-text ratio and fixed min/max fixture profile.
pub(super) fn prepare_aspect_ratio_minmax(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<C11_4TextInput>, RuntimeLayoutFailure> {
    prepare_with_profile(snapshot, root, styles, DirectTextProfile::AspectRatioMinMax)
}

/// One direct-text item spanning two equal max-content columns with a ratio/minmax profile.
pub(super) fn prepare_aspect_ratio_multispan(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<C11_4TextInput>, RuntimeLayoutFailure> {
    aspect_ratio_multispan::prepare(snapshot, root, styles)
}

/// Definite item axis transferred through the numeric preferred aspect ratio.
pub(super) fn prepare_aspect_ratio_definite_axis(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<C11_4TextInput>, RuntimeLayoutFailure> {
    aspect_ratio_definite_axis::prepare(snapshot, root, styles)
}

fn prepare_with_profile(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    profile: DirectTextProfile,
) -> Result<Option<C11_4TextInput>, RuntimeLayoutFailure> {
    validate_snapshot_revision(snapshot, root.id(), styles)?;
    let root_style = style_for(styles, root.id())?;
    if property(root_style, "display") != Some("grid") {
        return Ok(None);
    }
    if matches!(
        profile,
        DirectTextProfile::AspectRatio | DirectTextProfile::AspectRatioMinMax
    ) && !is_aspect_ratio_grid_root_candidate(root_style)
    {
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
            HostNodeKind::Element(_) => {
                if subtree_has_meaningful_text(snapshot, child)? {
                    candidates.push(child);
                }
            }
            HostNodeKind::Text(_) => {}
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
    if let Some(image) = first_html_image(snapshot, root)? {
        return Err(source_failure(
            image.id(),
            "replaced image content is unsupported in the generic text profile",
        ));
    }

    validate_grid_root(snapshot, root, styles, root_style, profile)?;
    let mut paragraph_styles = BTreeMap::new();
    let mut short_text_count = 0;
    let mut long_text_count = 0;
    let mut aspect_ratio_count = 0;
    let mut content_box_ratio_node = None;
    let mut aspect_ratio_minmax_node = None;
    let mut saw_minmax_constraints = false;
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
        let item_has_minmax = has_minmax_constraints(style);
        saw_minmax_constraints |= item_has_minmax;
        if item_has_minmax {
            if profile != DirectTextProfile::AspectRatioMinMax
                || text != "AAAA"
                || !supported_minmax_constraints(style)
            {
                return Err(style_failure(
                    style.node_id.get(),
                    "min/max constraints are limited to fixed C11.4 direct-text cases on AAAA",
                ));
            }
            if style.layout_aspect_ratio.is_some() {
                aspect_ratio_minmax_node = Some(style.node_id);
            }
        }
        let paragraph_style = match profile {
            DirectTextProfile::Intrinsic => paragraph_style_for(styles, style, 10.0, "auto")?,
            DirectTextProfile::AspectRatio | DirectTextProfile::AspectRatioMinMax => {
                match text.as_str() {
                    "A" => short_text_count += 1,
                    "AAAA" => long_text_count += 1,
                    _ => {
                        return Err(source_failure(
                            text_node.id(),
                            "aspect-ratio profile requires one A and one AAAA item",
                        ));
                    }
                }
                if let Some(ratio) = style.layout_aspect_ratio {
                    aspect_ratio_count += 1;
                    if text != "AAAA" || !computed_ratio_matches(style, ratio) {
                        return Err(style_failure(
                            style.node_id.get(),
                            "aspect-ratio profile requires a matching finite positive computed ratio on AAAA",
                        ));
                    }
                    if ratio.sizing_box == ComputedAspectRatioSizingBox::ContentBox {
                        content_box_ratio_node = Some(style.node_id);
                    }
                    validate_ratio_geometry(root_style, ratio.ratio, style.node_id.get())?;
                } else if property(style, "aspect-ratio") != Some("auto") {
                    return Err(style_failure(
                        style.node_id.get(),
                        "aspect-ratio profile requires computed auto when typed ratio is absent",
                    ));
                }
                if profile == DirectTextProfile::AspectRatioMinMax {
                    paragraph_style_for_aspect_ratio_minmax(styles, style, 10.0, "auto")?
                } else {
                    paragraph_style_for_aspect_ratio(styles, style, 10.0, "auto")?
                }
            }
        };
        paragraph_styles.insert(item.id(), paragraph_style);
    }

    if paragraph_styles.len() != 2 {
        return Err(source_failure(
            root.id(),
            "candidate paragraph ownership is not unique",
        ));
    }
    if profile == DirectTextProfile::AspectRatioMinMax && !saw_minmax_constraints {
        return Ok(None);
    }
    if matches!(
        profile,
        DirectTextProfile::AspectRatio | DirectTextProfile::AspectRatioMinMax
    ) && (short_text_count != 1 || long_text_count != 1 || aspect_ratio_count > 1)
    {
        return Err(source_failure(
            root.id(),
            "aspect-ratio profile requires A/AAAA and at most one ratio item",
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
        content_box_ratio_node,
        aspect_ratio_minmax_node,
        definite_axis_ratio_node: None,
    }))
}

fn is_aspect_ratio_grid_root_candidate(style: &ComputedElementStyle) -> bool {
    let Some(grid) = style.grid_layout.as_ref() else {
        return false;
    };
    grid.columns
        == [
            ComputedCssGridTrackSize::MaxContent,
            ComputedCssGridTrackSize::MaxContent,
        ]
        && aspect_ratio_rows_supported(&grid.rows)
        && style.layout_dimensions.width == ComputedCssDimension::LengthPx(100.0)
        && style.layout_dimensions.height == ComputedCssDimension::Auto
}

pub(super) fn validate_snapshot_revision(
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
    profile: DirectTextProfile,
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
    require_grid_property(
        style,
        "align-items",
        match profile {
            DirectTextProfile::Intrinsic => "start",
            DirectTextProfile::AspectRatio | DirectTextProfile::AspectRatioMinMax => "normal",
        },
    )?;
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
    let tracks_match = match profile {
        DirectTextProfile::Intrinsic => {
            grid.columns
                == [
                    ComputedCssGridTrackSize::MinContent,
                    ComputedCssGridTrackSize::MaxContent,
                ]
                && grid.rows == [ComputedCssGridTrackSize::Auto]
        }
        DirectTextProfile::AspectRatio | DirectTextProfile::AspectRatioMinMax => {
            grid.columns
                == [
                    ComputedCssGridTrackSize::MaxContent,
                    ComputedCssGridTrackSize::MaxContent,
                ]
                && aspect_ratio_rows_supported(&grid.rows)
        }
    };
    if !tracks_match
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
        || match profile {
            DirectTextProfile::Intrinsic => {
                (spacing.row_gap != ComputedCssSpacingValue::Normal
                    && spacing.row_gap != ComputedCssSpacingValue::LengthPx(0.0))
                    || spacing.column_gap != ComputedCssSpacingValue::LengthPx(10.0)
            }
            DirectTextProfile::AspectRatio | DirectTextProfile::AspectRatioMinMax => {
                spacing.row_gap != ComputedCssSpacingValue::LengthPx(0.0)
                    || spacing.column_gap != ComputedCssSpacingValue::LengthPx(0.0)
            }
        }
        || style.layout_border.top != 0.0
        || style.layout_border.right != 0.0
        || style.layout_border.bottom != 0.0
        || style.layout_border.left != 0.0
    {
        return Err(grid_style_failure(style.node_id, "Grid root box geometry"));
    }
    Ok(())
}

fn aspect_ratio_rows_supported(rows: &[ComputedCssGridTrackSize]) -> bool {
    matches!(rows, [ComputedCssGridTrackSize::Auto])
        || matches!(
            rows,
            [ComputedCssGridTrackSize::LengthPx(value)]
                if [20.0, 30.0, 40.0, 45.0, 60.0].contains(value)
        )
}

fn has_minmax_constraints(style: &ComputedElementStyle) -> bool {
    let dimensions = style.layout_dimensions;
    dimensions.min_width != ComputedCssDimension::Auto
        || dimensions.max_width != ComputedCssMaxSize::None
        || dimensions.min_height != ComputedCssDimension::Auto
        || dimensions.max_height != ComputedCssMaxSize::None
}

fn supported_minmax_constraints(style: &ComputedElementStyle) -> bool {
    fn min_supported(value: ComputedCssDimension) -> bool {
        match value {
            ComputedCssDimension::Auto => true,
            ComputedCssDimension::LengthPx(value) => {
                value.is_finite() && (0.0..=60.0).contains(&value)
            }
            _ => false,
        }
    }
    fn max_supported(value: ComputedCssMaxSize) -> bool {
        match value {
            ComputedCssMaxSize::None => true,
            ComputedCssMaxSize::LengthPx(value) => {
                value.is_finite() && (0.0..=60.0).contains(&value)
            }
            _ => false,
        }
    }

    let dimensions = style.layout_dimensions;
    min_supported(dimensions.min_width)
        && max_supported(dimensions.max_width)
        && min_supported(dimensions.min_height)
        && max_supported(dimensions.max_height)
}

fn computed_ratio_matches(
    style: &ComputedElementStyle,
    ratio: spinon_style::ComputedLayoutAspectRatio,
) -> bool {
    if !ratio.ratio.is_finite() || ratio.ratio <= 0.0 {
        return false;
    }
    let Some(serialized) = property(style, "aspect-ratio") else {
        return false;
    };
    let (has_auto, serialized_ratio) = if let Some(serialized) = serialized.strip_prefix("auto ") {
        (true, serialized)
    } else {
        (false, serialized)
    };
    if has_auto != (ratio.sizing_box == ComputedAspectRatioSizingBox::ContentBox) {
        return false;
    }
    let Some((numerator, denominator)) = serialized_ratio.split_once('/') else {
        return false;
    };
    let (Ok(numerator), Ok(denominator)) = (
        numerator.trim().parse::<f32>(),
        denominator.trim().parse::<f32>(),
    ) else {
        return false;
    };
    if !numerator.is_finite() || !denominator.is_finite() || numerator <= 0.0 || denominator <= 0.0
    {
        return false;
    }
    numerator / denominator == ratio.ratio
}

fn supported_aspect_ratio_edges(style: &ComputedElementStyle, allow_aspect_ratio: bool) -> bool {
    let spacing = style.layout_spacing;
    let border = style.layout_border;
    let no_border =
        border.top == 0.0 && border.right == 0.0 && border.bottom == 0.0 && border.left == 0.0;
    if zero_edges(spacing.padding) && no_border {
        return true;
    }
    allow_aspect_ratio
        && spacing.padding.top == ComputedCssSpacingValue::LengthPx(2.0)
        && spacing.padding.right == ComputedCssSpacingValue::LengthPx(4.0)
        && spacing.padding.bottom == ComputedCssSpacingValue::LengthPx(2.0)
        && spacing.padding.left == ComputedCssSpacingValue::LengthPx(4.0)
        && border.top == 1.0
        && border.right == 1.0
        && border.bottom == 1.0
        && border.left == 1.0
        && matches!(
            property(style, "box-sizing"),
            Some("border-box" | "content-box")
        )
}

fn validate_ratio_geometry(
    root_style: &ComputedElementStyle,
    ratio: f32,
    node: u64,
) -> Result<(), RuntimeLayoutFailure> {
    // The profile's long text has a 40px intrinsic inline size. A ratio can
    // derive its block size from that width, or its inline size from a fixed
    // row track. Reject unrepresentable intermediates before Taffy sees them.
    let derived_block = 40.0_f32 / ratio;
    let fixed_row = root_style
        .grid_layout
        .as_ref()
        .and_then(|grid| grid.rows.first())
        .and_then(|row| match row {
            ComputedCssGridTrackSize::LengthPx(value) => Some(*value),
            _ => None,
        });
    let derived_inline = fixed_row.map(|height| height * ratio);
    if !derived_block.is_finite()
        || derived_block <= 0.0
        || derived_inline.is_some_and(|value| !value.is_finite() || value <= 0.0)
    {
        return Err(style_failure(
            node,
            "aspect-ratio derived geometry is outside the finite positive f32 layout domain",
        ));
    }
    Ok(())
}

pub(super) fn direct_text_source(
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

pub(super) fn subtree_has_meaningful_text(
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

pub(super) fn first_html_image(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
) -> Result<Option<HostNodeHandle>, RuntimeLayoutFailure> {
    let mut pending = vec![root];
    while let Some(handle) = pending.pop() {
        let node = snapshot
            .node(handle)
            .ok_or_else(|| source_failure(handle.id(), "missing image subtree node"))?;
        if let HostNodeKind::Element(element) = node.kind()
            && element.namespace() == HTML_NAMESPACE
            && element.local_name() == "img"
        {
            return Ok(Some(handle));
        }
        pending.extend(
            snapshot
                .children(handle)
                .ok_or_else(|| source_failure(handle.id(), "missing image subtree children"))?,
        );
    }
    Ok(None)
}

#[derive(Clone, Copy)]
enum DirectTextParagraphProfile {
    Plain,
    AspectRatio,
    AspectRatioDefiniteAxis,
    AspectRatioMinMax,
    AspectRatioMinMaxSpan,
    Baseline { allow_explicit_grid_placement: bool },
}

pub(super) fn paragraph_style_for(
    styles: &ComputedStyleSnapshot,
    style: &ComputedElementStyle,
    font_size: f32,
    align_self: &'static str,
) -> Result<ParagraphStyle, RuntimeLayoutFailure> {
    paragraph_style_for_with_baseline_alias(
        styles,
        style,
        font_size,
        align_self,
        DirectTextParagraphProfile::Plain,
    )
}

pub(super) fn paragraph_style_for_aspect_ratio(
    styles: &ComputedStyleSnapshot,
    style: &ComputedElementStyle,
    font_size: f32,
    align_self: &'static str,
) -> Result<ParagraphStyle, RuntimeLayoutFailure> {
    paragraph_style_for_with_baseline_alias(
        styles,
        style,
        font_size,
        align_self,
        DirectTextParagraphProfile::AspectRatio,
    )
}

pub(super) fn paragraph_style_for_aspect_ratio_definite_axis(
    styles: &ComputedStyleSnapshot,
    style: &ComputedElementStyle,
    font_size: f32,
    align_self: &'static str,
) -> Result<ParagraphStyle, RuntimeLayoutFailure> {
    paragraph_style_for_with_baseline_alias(
        styles,
        style,
        font_size,
        align_self,
        DirectTextParagraphProfile::AspectRatioDefiniteAxis,
    )
}

pub(super) fn paragraph_style_for_aspect_ratio_minmax(
    styles: &ComputedStyleSnapshot,
    style: &ComputedElementStyle,
    font_size: f32,
    align_self: &'static str,
) -> Result<ParagraphStyle, RuntimeLayoutFailure> {
    paragraph_style_for_with_baseline_alias(
        styles,
        style,
        font_size,
        align_self,
        DirectTextParagraphProfile::AspectRatioMinMax,
    )
}

pub(super) fn paragraph_style_for_aspect_ratio_minmax_span(
    styles: &ComputedStyleSnapshot,
    style: &ComputedElementStyle,
    font_size: f32,
    align_self: &'static str,
) -> Result<ParagraphStyle, RuntimeLayoutFailure> {
    paragraph_style_for_with_baseline_alias(
        styles,
        style,
        font_size,
        align_self,
        DirectTextParagraphProfile::AspectRatioMinMaxSpan,
    )
}

pub(super) fn paragraph_style_for_baseline_fixture(
    styles: &ComputedStyleSnapshot,
    style: &ComputedElementStyle,
    font_size: f32,
    align_self: &'static str,
    allow_explicit_grid_placement: bool,
) -> Result<ParagraphStyle, RuntimeLayoutFailure> {
    paragraph_style_for_with_baseline_alias(
        styles,
        style,
        font_size,
        align_self,
        DirectTextParagraphProfile::Baseline {
            allow_explicit_grid_placement,
        },
    )
}

fn paragraph_style_for_with_baseline_alias(
    styles: &ComputedStyleSnapshot,
    style: &ComputedElementStyle,
    font_size: f32,
    align_self: &'static str,
    profile: DirectTextParagraphProfile,
) -> Result<ParagraphStyle, RuntimeLayoutFailure> {
    let (allow_explicit_grid_placement, allow_aspect_ratio, allow_minmax_constraints) =
        match profile {
            DirectTextParagraphProfile::Plain => (false, false, false),
            DirectTextParagraphProfile::AspectRatio
            | DirectTextParagraphProfile::AspectRatioDefiniteAxis => (false, true, false),
            DirectTextParagraphProfile::AspectRatioMinMax => (false, true, true),
            DirectTextParagraphProfile::AspectRatioMinMaxSpan => (true, true, true),
            DirectTextParagraphProfile::Baseline {
                allow_explicit_grid_placement,
            } => (allow_explicit_grid_placement, false, false),
        };
    let allow_first_baseline_alias = matches!(profile, DirectTextParagraphProfile::Baseline { .. });
    require_style_property(style, "display", "block")?;
    require_style_property(style, "position", "static")?;
    require_style_property(style, "order", "0")?;
    if !(allow_first_baseline_alias
        && align_self == "first baseline"
        && property(style, "align-self") == Some("baseline"))
    {
        require_style_property(style, "align-self", align_self)?;
    }
    require_style_property(style, "justify-self", "auto")?;
    if style.layout_position != ComputedCssPosition::Static {
        return Err(style_failure(style.node_id.get(), "position"));
    }
    require_style_property(style, "font-family", "Ahem")?;
    let font_size_property = match font_size {
        10.0 => "10px",
        20.0 => "20px",
        _ => return Err(style_failure(style.node_id.get(), "font-size profile")),
    };
    require_style_property(style, "font-size", font_size_property)?;
    require_style_property(style, "font-weight", "400")?;
    require_style_property(style, "font-style", "normal")?;
    require_style_property(style, "line-height", font_size_property)?;
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
    if (style.font_size_css_px - font_size).abs() > f32::EPSILON
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
    let dimensions_supported =
        if matches!(profile, DirectTextParagraphProfile::AspectRatioDefiniteAxis) {
            matches!(
                (
                    style.layout_dimensions.width,
                    style.layout_dimensions.height
                ),
                (ComputedCssDimension::Auto, ComputedCssDimension::Auto)
                    | (
                        ComputedCssDimension::LengthPx(60.0),
                        ComputedCssDimension::Auto
                    )
                    | (
                        ComputedCssDimension::Auto,
                        ComputedCssDimension::LengthPx(30.0)
                    )
            )
        } else {
            style.layout_dimensions.width == ComputedCssDimension::Auto
                && style.layout_dimensions.height == ComputedCssDimension::Auto
        };
    if !dimensions_supported
        || (!allow_minmax_constraints && has_minmax_constraints(style))
        || match style.layout_aspect_ratio {
            None => false,
            Some(ratio) => !allow_aspect_ratio || !computed_ratio_matches(style, ratio),
        }
        || !zero_edges(style.layout_spacing.margin)
        || !supported_aspect_ratio_edges(style, allow_aspect_ratio)
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
    if !allow_explicit_grid_placement
        && (placement.row_start != ComputedCssGridPlacement::Auto
            || placement.row_end != ComputedCssGridPlacement::Auto
            || placement.column_start != ComputedCssGridPlacement::Auto
            || placement.column_end != ComputedCssGridPlacement::Auto)
    {
        return Err(style_failure(style.node_id.get(), "Grid item placement"));
    }

    Ok(ParagraphStyle {
        overflow_wrap: Default::default(),
        writing_mode,
        text_orientation,
        font_family: "Ahem".to_owned(),
        font_size: style.font_size_css_px,
        line_height: font_size,
        line_height_is_normal: false,
        font_weight: 400.0,
        font_style: FontStyleMode::Normal,
        white_space,
        word_break,
        locale: None,
        direction: TextParagraphDirection::Ltr,
        line_break: LineBreakMode::Strict,
        ligatures: LigatureMode::Normal,
        letter_spacing: 0.0,
        word_spacing: 0.0,
        text_indent: 0.0,
        text_indent_flags: spinon_layout::TextIndentFlags::default(),
        text_transform: Default::default(),
    })
}

pub(super) fn style_for(
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

pub(super) fn require_style_property(
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

pub(super) fn require_grid_property(
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

pub(super) fn property<'a>(style: &'a ComputedElementStyle, name: &str) -> Option<&'a str> {
    style.properties.get(name).map(String::as_str)
}

pub(super) fn zero_edges(edges: spinon_style::ComputedCssEdges) -> bool {
    [edges.top, edges.right, edges.bottom, edges.left]
        .into_iter()
        .all(|value| value == ComputedCssSpacingValue::LengthPx(0.0))
}

pub(super) fn source_failure(node: NodeId, detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_text_source",
        Some(node.get()),
        Some(detail.to_owned()),
    )
}

pub(super) fn style_failure(node: u64, detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_text_style",
        Some(node),
        Some(detail.to_owned()),
    )
}

pub(super) fn grid_style_failure(node: NodeId, detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_text_grid_style",
        Some(node.get()),
        Some(detail.to_owned()),
    )
}
