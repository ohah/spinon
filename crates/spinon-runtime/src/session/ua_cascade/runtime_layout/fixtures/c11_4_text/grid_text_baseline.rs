use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind};
use spinon_layout::TextLayoutEngine;
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackSize,
    ComputedCssMaxSize, ComputedCssPosition, ComputedCssSpacingValue, ComputedStyleProfile,
    ComputedStyleSnapshot,
};

use super::super::super::{RuntimeLayoutFailure, layout_failure};
use super::super::C11_4TextInput;
use super::{AHEM, generic_direct_text};

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

#[derive(Clone, Copy)]
enum FixtureProfile {
    AutoTrackFirstBaseline,
    FixedTrackFirstLastBaseline,
}

fn fixture_profile(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
) -> Option<FixtureProfile> {
    let Ok((_, root_id)) = super::element_identity(snapshot, root) else {
        return None;
    };
    match root_id.as_str() {
        "c114-grid-text-first-baseline-align-items"
        | "c114-grid-text-first-baseline-start-control"
        | "c114-grid-text-first-baseline-align-self"
        | "c114-grid-text-first-baseline-reversed" => Some(FixtureProfile::AutoTrackFirstBaseline),
        "c114-grid-text-last-baseline-align-items-multiline"
        | "c114-grid-text-last-baseline-align-self-multiline"
        | "c114-grid-text-last-baseline-reversed-multiline"
        | "c114-grid-text-last-baseline-first-control"
        | "c114-grid-text-last-baseline-start-control" => {
            Some(FixtureProfile::FixedTrackFirstLastBaseline)
        }
        _ => None,
    }
}

pub(super) fn is_fixture_root(snapshot: &HostDocumentSnapshot, root: HostNodeHandle) -> bool {
    fixture_profile(snapshot, root).is_some()
}

pub(super) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<C11_4TextInput>, RuntimeLayoutFailure> {
    generic_direct_text::validate_snapshot_revision(snapshot, root.id(), styles)?;
    let Some(profile) = fixture_profile(snapshot, root) else {
        return Ok(None);
    };
    let (_, root_id) = super::element_identity(snapshot, root)?;
    let root_style = generic_direct_text::style_for(styles, root.id())?;
    if generic_direct_text::property(root_style, "display") != Some("grid") {
        return Ok(None);
    }
    if !matches_baseline_fixture_profile(root_style, profile) {
        return Ok(None);
    }
    validate_root(snapshot, root, styles, root_style, profile)?;

    let children = snapshot
        .children(root)
        .ok_or_else(|| generic_direct_text::source_failure(root.id(), "missing Grid children"))?
        .collect::<Vec<_>>();
    let mut items = Vec::new();
    for child in children {
        let node = snapshot
            .node(child)
            .ok_or_else(|| generic_direct_text::source_failure(child.id(), "missing Grid child"))?;
        match node.kind() {
            HostNodeKind::Text(text) if text.is_only_collapsible_html_whitespace() => {}
            HostNodeKind::Text(_) => {
                return Err(generic_direct_text::source_failure(
                    child.id(),
                    "anonymous text is unsupported in the direct text baseline fixture",
                ));
            }
            HostNodeKind::Element(_) => items.push(child),
        }
    }
    if items.len() != 2 {
        return Err(generic_direct_text::source_failure(
            root.id(),
            "baseline fixture requires exactly two direct element items",
        ));
    }

    let align_items = match generic_direct_text::property(root_style, "align-items") {
        // Stylo serializes the computed first-baseline keyword as `baseline`, matching the
        // computed-value behavior observed in pinned Chromium.
        Some("baseline" | "first baseline") => "first baseline",
        Some("last baseline") => "last baseline",
        Some("start") => "start",
        _ => {
            return Err(generic_direct_text::grid_style_failure(
                root.id(),
                "only first/last baseline and start control are supported",
            ));
        }
    };

    let mut paragraph_styles = BTreeMap::new();
    let mut font_sizes = Vec::with_capacity(items.len());
    for item in items {
        if !generic_direct_text::subtree_has_meaningful_text(snapshot, item)? {
            return Err(generic_direct_text::source_failure(
                item.id(),
                "baseline fixture item must contain direct text",
            ));
        }
        let text_node = generic_direct_text::direct_text_source(snapshot, root, item)?;
        let text = snapshot
            .node(text_node)
            .and_then(|node| match node.kind() {
                HostNodeKind::Text(text) => Some(text.to_string_lossy()),
                HostNodeKind::Element(_) => None,
            })
            .ok_or_else(|| {
                generic_direct_text::source_failure(text_node.id(), "direct Text source is missing")
            })?;
        let style = generic_direct_text::style_for(styles, item.id())?;
        let font_size = style.font_size_css_px;
        if !matches!(font_size, 10.0 | 20.0) {
            return Err(generic_direct_text::style_failure(
                style.node_id.get(),
                "baseline fixture font size must be 10px or 20px",
            ));
        }
        if matches!(profile, FixtureProfile::FixedTrackFirstLastBaseline) {
            let placement = style.grid_item_placement.as_ref().ok_or_else(|| {
                generic_direct_text::grid_style_failure(style.node_id, "grid placement")
            })?;
            let expected_column = match font_size {
                10.0 => 1,
                20.0 => 2,
                _ => unreachable!("font size was validated above"),
            };
            if placement.row_start != ComputedCssGridPlacement::Line(1)
                || placement.row_end != ComputedCssGridPlacement::Line(2)
                || placement.column_start != ComputedCssGridPlacement::Line(expected_column)
                || placement.column_end != ComputedCssGridPlacement::Line(expected_column + 1)
            {
                return Err(generic_direct_text::grid_style_failure(
                    style.node_id,
                    "explicit baseline fixture column placement",
                ));
            }
        }
        let is_multiline_scene = matches!(
            root_id.as_str(),
            "c114-grid-text-last-baseline-align-items-multiline"
                | "c114-grid-text-last-baseline-align-self-multiline"
                | "c114-grid-text-last-baseline-reversed-multiline"
                | "c114-grid-text-last-baseline-start-control"
        );
        let expected_text = if is_multiline_scene && font_size == 10.0 {
            "XX XX"
        } else {
            "XX"
        };
        if text != expected_text {
            return Err(generic_direct_text::source_failure(
                text_node.id(),
                "baseline fixture text does not match the pinned scene",
            ));
        }
        let align_self = match generic_direct_text::property(style, "align-self") {
            Some("auto") => "auto",
            Some("baseline" | "first baseline") => "first baseline",
            Some("last baseline") => "last baseline",
            _ => {
                return Err(generic_direct_text::style_failure(
                    style.node_id.get(),
                    "align-self must be auto, first baseline, or last baseline",
                ));
            }
        };
        let paragraph_style = generic_direct_text::paragraph_style_for_baseline_fixture(
            styles,
            style,
            font_size,
            align_self,
            matches!(profile, FixtureProfile::FixedTrackFirstLastBaseline),
        )?;
        font_sizes.push((font_size, align_self));
        paragraph_styles.insert(item.id(), paragraph_style);
    }

    if font_sizes.iter().filter(|(size, _)| *size == 10.0).count() != 1
        || font_sizes.iter().filter(|(size, _)| *size == 20.0).count() != 1
    {
        return Err(generic_direct_text::source_failure(
            root.id(),
            "baseline fixture requires one 10px and one 20px text item",
        ));
    }
    let (expected_align_items, expected_align_self) = match root_id.as_str() {
        "c114-grid-text-first-baseline-align-items" | "c114-grid-text-first-baseline-reversed" => {
            ("first baseline", "auto")
        }
        "c114-grid-text-first-baseline-start-control" => ("start", "auto"),
        "c114-grid-text-first-baseline-align-self" => ("start", "first baseline"),
        "c114-grid-text-last-baseline-align-items-multiline"
        | "c114-grid-text-last-baseline-reversed-multiline" => ("last baseline", "auto"),
        "c114-grid-text-last-baseline-align-self-multiline" => ("start", "last baseline"),
        "c114-grid-text-last-baseline-first-control" => ("first baseline", "auto"),
        "c114-grid-text-last-baseline-start-control" => ("start", "auto"),
        _ => return Ok(None),
    };
    if align_items != expected_align_items
        || font_sizes
            .iter()
            .any(|(_, align_self)| *align_self != expected_align_self)
    {
        return Err(generic_direct_text::grid_style_failure(
            root.id(),
            "baseline fixture alignment does not match its scene ID",
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
        content_box_ratio_node: None,
        aspect_ratio_minmax_node: None,
        definite_axis_ratio_node: None,
    }))
}

fn matches_baseline_fixture_profile(
    style: &spinon_style::ComputedElementStyle,
    profile: FixtureProfile,
) -> bool {
    let Some(grid) = style.grid_layout.as_ref() else {
        return false;
    };
    let track_and_width_match = match profile {
        FixtureProfile::AutoTrackFirstBaseline => {
            grid.columns
                == [
                    ComputedCssGridTrackSize::Auto,
                    ComputedCssGridTrackSize::Auto,
                ]
                && style.layout_dimensions.width == ComputedCssDimension::MaxContent
        }
        FixtureProfile::FixedTrackFirstLastBaseline => {
            grid.columns
                == [
                    ComputedCssGridTrackSize::LengthPx(25.0),
                    ComputedCssGridTrackSize::LengthPx(40.0),
                ]
                && style.layout_dimensions.width == ComputedCssDimension::LengthPx(73.0)
        }
    };
    track_and_width_match
        && grid.rows == [ComputedCssGridTrackSize::Auto]
        && grid.auto_flow == ComputedCssGridAutoFlow::Row
        && grid.auto_rows == ComputedCssGridAutoRows::Auto
        && grid.auto_columns == ComputedCssGridAutoColumns::Auto
        && grid.columns_auto_repeat.is_none()
        && grid.rows_auto_repeat.is_none()
        && grid.template_areas.is_none()
        && style.layout_dimensions.height == ComputedCssDimension::Auto
        && style.layout_spacing.column_gap == ComputedCssSpacingValue::LengthPx(8.0)
}

fn validate_root(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    style: &spinon_style::ComputedElementStyle,
    profile: FixtureProfile,
) -> Result<(), RuntimeLayoutFailure> {
    let element = snapshot
        .node(root)
        .and_then(|node| match node.kind() {
            HostNodeKind::Element(element) => Some(element),
            HostNodeKind::Text(_) => None,
        })
        .ok_or_else(|| {
            generic_direct_text::source_failure(root.id(), "Grid root is not element")
        })?;
    if element.namespace() != HTML_NAMESPACE {
        return Err(generic_direct_text::source_failure(
            root.id(),
            "Grid root must use HTML namespace",
        ));
    }
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 {
        return Err(generic_direct_text::grid_style_failure(
            root.id(),
            "computed-style profile",
        ));
    }
    for (name, expected) in [
        ("display", "grid"),
        ("align-content", "normal"),
        ("justify-content", "normal"),
        ("justify-items", "normal"),
        ("direction", "ltr"),
        ("writing-mode", "horizontal-tb"),
    ] {
        generic_direct_text::require_grid_property(style, name, expected)?;
    }
    if styles.writing_style_for(root.id())
        != Some((
            spinon_core::WritingMode::HorizontalTb,
            spinon_core::TextOrientation::Mixed,
        ))
    {
        return Err(generic_direct_text::grid_style_failure(
            root.id(),
            "Grid root writing style",
        ));
    }
    let grid = style.grid_layout.as_ref().ok_or_else(|| {
        generic_direct_text::grid_style_failure(root.id(), "Grid layout is missing")
    })?;
    let (expected_columns, expected_width) = match profile {
        FixtureProfile::AutoTrackFirstBaseline => (
            [
                ComputedCssGridTrackSize::Auto,
                ComputedCssGridTrackSize::Auto,
            ],
            ComputedCssDimension::MaxContent,
        ),
        FixtureProfile::FixedTrackFirstLastBaseline => (
            [
                ComputedCssGridTrackSize::LengthPx(25.0),
                ComputedCssGridTrackSize::LengthPx(40.0),
            ],
            ComputedCssDimension::LengthPx(73.0),
        ),
    };
    if grid.columns != expected_columns
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
        return Err(generic_direct_text::grid_style_failure(
            root.id(),
            "explicit auto-track profile",
        ));
    }
    let dimensions = style.layout_dimensions;
    let spacing = style.layout_spacing;
    if dimensions.width != expected_width
        || dimensions.height != ComputedCssDimension::Auto
        || dimensions.min_width != ComputedCssDimension::Auto
        || dimensions.max_width != ComputedCssMaxSize::None
        || dimensions.min_height != ComputedCssDimension::Auto
        || dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_aspect_ratio.is_some()
        || style.layout_position != ComputedCssPosition::Static
        || !generic_direct_text::zero_edges(spacing.margin)
        || !generic_direct_text::zero_edges(spacing.padding)
        || (spacing.row_gap != ComputedCssSpacingValue::Normal
            && spacing.row_gap != ComputedCssSpacingValue::LengthPx(0.0))
        || spacing.column_gap != ComputedCssSpacingValue::LengthPx(8.0)
        || style.layout_border.top != 0.0
        || style.layout_border.right != 0.0
        || style.layout_border.bottom != 0.0
        || style.layout_border.left != 0.0
    {
        return Err(generic_direct_text::grid_style_failure(
            style.node_id,
            "Grid root box geometry",
        ));
    }
    Ok(())
}
