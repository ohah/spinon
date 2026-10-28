use std::collections::BTreeMap;

use spinon_core::{
    HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, IntrinsicSizeRevision,
    TextOrientation, WritingMode,
};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_layout::{LineBreakMode, TextLayoutEngine, WhiteSpaceMode};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackBreadth,
    ComputedCssGridTrackSize, ComputedCssMaxSize, ComputedCssPosition, ComputedCssSpacingValue,
    ComputedElementStyle, ComputedStyleSnapshot,
};

use super::generic_direct_text_image::{has_mixed_signature, validate_snapshot_revision};
use super::{
    HTML_NAMESPACE, RuntimeLayoutFailure, ahem_paragraph_style, element_identity, failure,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Axis {
    Columns,
    Rows,
}

/// Routes a direct text/image/text fixture through the generic Grid and intrinsic-size paths.
pub(super) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<super::super::C11_4MixedInput>, RuntimeLayoutFailure> {
    prepare_with_png(snapshot, root, styles, super::PNG)
}

/// 고정 adapter를 유지하면서 PNG decode 입력을 바꿔 fail-closed 경계를 검증한다.
fn prepare_with_png(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    png: &[u8],
) -> Result<Option<super::super::C11_4MixedInput>, RuntimeLayoutFailure> {
    if let Some(input) = hidden::prepare(snapshot, root, styles, png)? {
        return Ok(Some(input));
    }
    if !has_mixed_signature(snapshot, root)? {
        return Ok(None);
    }
    validate_snapshot_revision(snapshot, root.id(), styles)?;
    let root_style = super::style_for(styles, root)?;
    let Some(axis) = track_axis(root_style) else {
        return Ok(None);
    };
    validate_root_style(root_style, styles, root, axis)?;

    let (namespace, tag, _) = element_identity(snapshot, root)?;
    if namespace != HTML_NAMESPACE
        || tag != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
    {
        return Err(reject(root.id(), "root identity or parent differs"));
    }
    let children = snapshot
        .children(root)
        .ok_or_else(|| fail(root.id(), "root children unavailable"))?
        .collect::<Vec<_>>();
    if children.len() != 3 {
        return Err(reject(
            root.id(),
            "exactly three direct Grid items required",
        ));
    }

    let mut source_tracks = Vec::with_capacity(3);
    let mut text_items = Vec::with_capacity(2);
    let mut image_item = None;
    for child in children {
        let (child_namespace, child_tag, _) = element_identity(snapshot, child)?;
        if child_namespace != HTML_NAMESPACE
            || snapshot.parent(child) != Some(HostParent::Node(root))
        {
            return Err(reject(child.id(), "item must be a direct HTML element"));
        }
        let style = super::style_for(styles, child)?;
        let track = item_track(style, axis)
            .ok_or_else(|| reject(child.id(), "item placement must occupy one explicit track"))?;
        source_tracks.push(track);
        match child_tag.as_str() {
            "div" => {
                validate_item_style(style, styles, axis, track, false)?;
                let text = direct_text(snapshot, child)?;
                text_items.push((track, child, text));
            }
            "img" => {
                if image_item.replace(child).is_some() || track != 2 {
                    return Err(reject(
                        child.id(),
                        "one image in the fixed middle track required",
                    ));
                }
                validate_item_style(style, styles, axis, track, true)?;
                if snapshot
                    .children(child)
                    .is_none_or(|mut descendants| descendants.next().is_some())
                    || super::image_src(snapshot, child)?.as_deref() != Some(super::IMAGE_URL)
                {
                    return Err(reject(child.id(), "fixed leaf image source required"));
                }
            }
            _ => {
                return Err(reject(
                    child.id(),
                    "only text divs and one image are supported",
                ));
            }
        }
    }

    if source_tracks != [1, 2, 3] && source_tracks != [3, 2, 1] {
        return Err(reject(
            root.id(),
            "source order must be forward or fully reversed",
        ));
    }
    if text_items.len() != 2 {
        return Err(reject(root.id(), "exactly two text items required"));
    }
    let first_text = text_items
        .iter()
        .find(|(track, _, _)| *track == 1)
        .ok_or_else(|| reject(root.id(), "first intrinsic track text is missing"))?;
    let third_text = text_items
        .iter()
        .find(|(track, _, _)| *track == 3)
        .ok_or_else(|| reject(root.id(), "third track text is missing"))?;
    if ![4, 8, 10].contains(&first_text.2.len())
        || first_text.2 != "X".repeat(first_text.2.len())
        || third_text.2 != "XXXX"
    {
        return Err(reject(root.id(), "fixed Ahem text contributions differ"));
    }

    let image_item = image_item.ok_or_else(|| reject(root.id(), "image item missing"))?;
    let completion = ImageRequest::from_document(snapshot, image_item)
        .map_err(|error| fail(image_item.id(), format!("image request: {error}")))?
        .decode(png)
        .map_err(|error| fail(image_item.id(), format!("image decode: {error}")))?;
    let revision = IntrinsicSizeRevision::INITIAL
        .checked_next()
        .ok_or_else(|| fail(root.id(), "intrinsic-size revision exhausted"))?;
    let resources = ImageResourceSnapshot::new(snapshot, revision, vec![completion])
        .map_err(|error| fail(root.id(), format!("image snapshot: {error}")))?;
    let dimensions = resources
        .images()
        .get(&image_item.id())
        .ok_or_else(|| fail(image_item.id(), "image intrinsic metadata missing"))?
        .dimensions();
    if dimensions.width() != super::NATURAL_WIDTH || dimensions.height() != super::NATURAL_HEIGHT {
        return Err(fail(
            image_item.id(),
            format!(
                "image natural size is {}x{}",
                dimensions.width(),
                dimensions.height()
            ),
        ));
    }
    let intrinsic_sizes = resources
        .to_layout_intrinsic_size_snapshot()
        .map_err(|error| fail(root.id(), format!("intrinsic-size snapshot: {error}")))?;

    let mut paragraph_styles = BTreeMap::new();
    for (_, handle, _) in text_items {
        let mut paragraph = ahem_paragraph_style();
        paragraph.white_space = WhiteSpaceMode::NoWrap;
        if axis == Axis::Rows {
            paragraph.writing_mode = WritingMode::VerticalRl;
            paragraph.line_break = LineBreakMode::Normal;
        }
        paragraph_styles.insert(handle.id(), paragraph);
    }
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine
        .register_font("Ahem", super::AHEM)
        .map_err(|error| fail(root.id(), format!("Ahem font registration: {error}")))?;

    Ok(Some(super::super::C11_4MixedInput {
        paragraph_styles,
        engine,
        intrinsic_sizes,
        use_extended_fixture_projection: false,
    }))
}

#[path = "generic_intrinsic_flex_fixed/hidden.rs"]
mod hidden;

fn track_axis(style: &ComputedElementStyle) -> Option<Axis> {
    let grid = style.grid_layout.as_ref()?;
    let tracks = mixed_flex_tracks();
    if grid.columns == tracks && grid.rows == vec![ComputedCssGridTrackSize::LengthPx(120.0)] {
        Some(Axis::Columns)
    } else if grid.rows == tracks && grid.columns == vec![ComputedCssGridTrackSize::LengthPx(120.0)]
    {
        Some(Axis::Rows)
    } else {
        None
    }
}

fn mixed_flex_tracks() -> Vec<ComputedCssGridTrackSize> {
    vec![
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::MinContent,
            max: ComputedCssGridTrackBreadth::Fraction(1.0),
        },
        ComputedCssGridTrackSize::LengthPx(40.0),
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::LengthPx(0.0),
            max: ComputedCssGridTrackBreadth::Fraction(2.0),
        },
    ]
}

fn validate_root_style(
    style: &ComputedElementStyle,
    styles: &ComputedStyleSnapshot,
    root: HostNodeHandle,
    axis: Axis,
) -> Result<(), RuntimeLayoutFailure> {
    let (width, height, columns, rows, column_gap, row_gap) = match axis {
        Axis::Columns => (
            300.0,
            120.0,
            3,
            1,
            ComputedCssSpacingValue::LengthPx(10.0),
            ComputedCssSpacingValue::LengthPx(0.0),
        ),
        Axis::Rows => (
            120.0,
            300.0,
            1,
            3,
            ComputedCssSpacingValue::LengthPx(0.0),
            ComputedCssSpacingValue::LengthPx(10.0),
        ),
    };
    for (name, expected) in [
        ("display", "grid"),
        ("box-sizing", "border-box"),
        ("align-items", "normal"),
        ("align-content", "start"),
        ("justify-items", "normal"),
        ("justify-content", "start"),
        ("direction", "ltr"),
        ("writing-mode", "horizontal-tb"),
    ] {
        require_property(style, name, expected)?;
    }
    let grid = style
        .grid_layout
        .as_ref()
        .ok_or_else(|| fail(root.id(), "computed Grid layout missing"))?;
    let mixed_tracks = mixed_flex_tracks();
    let fixed_track = vec![ComputedCssGridTrackSize::LengthPx(120.0)];
    let (expected_columns, expected_rows) = match axis {
        Axis::Columns => (mixed_tracks, fixed_track),
        Axis::Rows => (fixed_track, mixed_tracks),
    };
    if styles.profile != spinon_style::ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || styles.writing_style_for(root.id())
            != Some((WritingMode::HorizontalTb, TextOrientation::Mixed))
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(width)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(height)
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || style.layout_position != ComputedCssPosition::Static
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || style.layout_spacing.column_gap != column_gap
        || style.layout_spacing.row_gap != row_gap
        || style.layout_border != Default::default()
        || grid.columns.len() != columns
        || grid.rows.len() != rows
        || grid.columns != expected_columns
        || grid.rows != expected_rows
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || grid.auto_rows != ComputedCssGridAutoRows::Auto
        || grid.auto_columns != ComputedCssGridAutoColumns::Auto
        || grid.columns_line_names.len() != columns + 1
        || grid.rows_line_names.len() != rows + 1
        || grid
            .columns_line_names
            .iter()
            .chain(grid.rows_line_names.iter())
            .any(|names| !names.is_empty())
    {
        return Err(reject(
            root.id(),
            "fixed mixed intrinsic/flex Grid style differs",
        ));
    }
    Ok(())
}

fn validate_item_style(
    style: &ComputedElementStyle,
    styles: &ComputedStyleSnapshot,
    axis: Axis,
    track: i32,
    image: bool,
) -> Result<(), RuntimeLayoutFailure> {
    let placement = style.grid_item_placement.as_ref().is_some_and(|placement| {
        let (column, row) = match axis {
            Axis::Columns => (track, 1),
            Axis::Rows => (1, track),
        };
        placement.column_start == ComputedCssGridPlacement::Line(column)
            && placement.column_end == ComputedCssGridPlacement::Line(column + 1)
            && placement.row_start == ComputedCssGridPlacement::Line(row)
            && placement.row_end == ComputedCssGridPlacement::Line(row + 1)
    });
    let writing_mode = if !image && axis == Axis::Rows {
        WritingMode::VerticalRl
    } else {
        WritingMode::HorizontalTb
    };
    for (name, expected) in [
        ("display", "block"),
        ("box-sizing", "border-box"),
        ("position", "static"),
        ("justify-self", "auto"),
        ("align-self", "auto"),
    ] {
        require_property(style, name, expected)?;
    }
    if !placement
        || style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || style.layout_position != ComputedCssPosition::Static
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || style.layout_border != Default::default()
        || style.layout_aspect_ratio.is_some()
        || styles.writing_style_for(style.node_id) != Some((writing_mode, TextOrientation::Mixed))
    {
        return Err(reject(
            style.node_id,
            "item dimensions, writing mode, or placement differs",
        ));
    }
    if !image {
        for (name, expected) in [
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
            ("letter-spacing", "normal"),
            ("word-spacing", "0px"),
            ("text-indent", "0px"),
        ] {
            require_property(style, name, expected)?;
        }
        if (style.font_size_css_px - 10.0).abs() > f32::EPSILON {
            return Err(reject(style.node_id, "Ahem font size differs"));
        }
    }
    Ok(())
}

fn item_track(style: &ComputedElementStyle, axis: Axis) -> Option<i32> {
    let placement = style.grid_item_placement.as_ref()?;
    let (start, end, other_start, other_end) = match axis {
        Axis::Columns => (
            &placement.column_start,
            &placement.column_end,
            &placement.row_start,
            &placement.row_end,
        ),
        Axis::Rows => (
            &placement.row_start,
            &placement.row_end,
            &placement.column_start,
            &placement.column_end,
        ),
    };
    let ComputedCssGridPlacement::Line(start) = start else {
        return None;
    };
    if !(1..=3).contains(start)
        || end != &ComputedCssGridPlacement::Line(start + 1)
        || other_start != &ComputedCssGridPlacement::Line(1)
        || other_end != &ComputedCssGridPlacement::Line(2)
    {
        return None;
    }
    Some(*start)
}

fn direct_text(
    snapshot: &HostDocumentSnapshot,
    item: HostNodeHandle,
) -> Result<String, RuntimeLayoutFailure> {
    let children = snapshot
        .children(item)
        .ok_or_else(|| fail(item.id(), "text item children unavailable"))?
        .collect::<Vec<_>>();
    if children.len() != 1 || snapshot.parent(children[0]) != Some(HostParent::Node(item)) {
        return Err(reject(item.id(), "one direct text node required"));
    }
    snapshot
        .node(children[0])
        .and_then(|node| match node.kind() {
            HostNodeKind::Text(value) => Some(value.to_string_lossy()),
            HostNodeKind::Element(_) => None,
        })
        .ok_or_else(|| reject(children[0].id(), "text source must be a text node"))
}

fn require_property(
    style: &ComputedElementStyle,
    name: &str,
    expected: &str,
) -> Result<(), RuntimeLayoutFailure> {
    if style.properties.get(name).map(String::as_str) == Some(expected) {
        Ok(())
    } else {
        Err(reject(style.node_id, &format!("{name} must be {expected}")))
    }
}

fn zero_edges(edges: spinon_style::ComputedCssEdges) -> bool {
    [edges.top, edges.right, edges.bottom, edges.left]
        .into_iter()
        .all(|value| value == ComputedCssSpacingValue::LengthPx(0.0))
}

fn reject(node: spinon_core::NodeId, reason: &str) -> RuntimeLayoutFailure {
    fail(node, reason.to_owned())
}

fn fail(node: spinon_core::NodeId, reason: impl Into<String>) -> RuntimeLayoutFailure {
    failure(
        "unsupported_c114_generic_intrinsic_flex_fixed",
        node,
        Some(reason.into()),
    )
}

#[cfg(test)]
#[path = "generic_intrinsic_flex_fixed/tests.rs"]
mod tests;
