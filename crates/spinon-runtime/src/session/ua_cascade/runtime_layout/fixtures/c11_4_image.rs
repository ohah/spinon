use spinon_core::{
    AttributeName, HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent,
    IntrinsicSizeRevision,
};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridItemPlacement, ComputedCssGridPlacement,
    ComputedCssGridTrackBreadth, ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize,
    ComputedCssMaxSize, ComputedCssSpacingValue, ComputedElementStyle, ComputedStyleProfile,
    ComputedStyleSnapshot,
};

use super::super::{RuntimeLayoutFailure, layout_failure};

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
const IMAGE_URL: &str = "https://spinon.invalid/c14-1-png-basic.png";
const PNG: &[u8] =
    include_bytes!("../../../../../../../tests/fixtures/resources/c14-1-png-basic.png");
const NATURAL_WIDTH: u32 = 24;
const NATURAL_HEIGHT: u32 = 16;

#[path = "c11_4_image/cases.rs"]
mod cases;
use cases::{IMAGE_CASES, ImageCase, ImageSizing, Placement, Track};

pub(in crate::session::ua_cascade::runtime_layout) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<super::C11_4ImageInput>, RuntimeLayoutFailure> {
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 {
        return Err(layout_failure(
            "c114_image_profile_mismatch",
            Some(root.id().get()),
            None,
        ));
    }

    let image_nodes = images_in_subtree(snapshot, root)?;
    if image_nodes.is_empty() {
        return Ok(None);
    }

    let (root_namespace, root_tag, root_id) = element_identity(snapshot, root)?;
    let case = IMAGE_CASES
        .iter()
        .find(|case| case.root_id == root_id)
        .ok_or_else(|| {
            layout_failure(
                "unsupported_c114_image_fixture_root",
                Some(root.id().get()),
                Some(root_id.clone()),
            )
        })?;
    if root_namespace != HTML_NAMESPACE
        || root_tag != "div"
        || image_nodes.len() != 1
        || snapshot.parent(root) != Some(HostParent::Root)
    {
        return Err(unsupported_case(
            root.id().get(),
            case,
            "one direct HTML img required",
        ));
    }

    let mut root_children = snapshot
        .children(root)
        .ok_or_else(|| layout_failure("missing_node", Some(root.id().get()), None))?;
    let item = root_children
        .next()
        .ok_or_else(|| unsupported_case(root.id().get(), case, "direct image child missing"))?;
    if root_children.next().is_some() || item != image_nodes[0] {
        return Err(unsupported_case(
            root.id().get(),
            case,
            "one direct HTML img required",
        ));
    }
    let (item_namespace, item_tag, item_id) = element_identity(snapshot, item)?;
    if item_namespace != HTML_NAMESPACE || item_tag != "img" || item_id != case.item_id {
        return Err(unsupported_case(
            item.id().get(),
            case,
            "image tag or id mismatch",
        ));
    }
    if snapshot
        .children(item)
        .is_none_or(|mut children| children.next().is_some())
    {
        return Err(unsupported_case(
            item.id().get(),
            case,
            "replaced leaf required",
        ));
    }
    if image_src(snapshot, item)?.as_deref() != Some(IMAGE_URL) {
        return Err(unsupported_case(
            item.id().get(),
            case,
            "fixed PNG source required",
        ));
    }

    let grid_style = styles
        .elements
        .iter()
        .find(|style| style.node_id == root.id())
        .ok_or_else(|| layout_failure("missing_computed_style", Some(root.id().get()), None))?;
    validate_grid_style(grid_style, case)?;
    let item_style = styles
        .elements
        .iter()
        .find(|style| style.node_id == item.id())
        .ok_or_else(|| layout_failure("missing_computed_style", Some(item.id().get()), None))?;
    validate_item_style(item_style, case)?;

    let completion = ImageRequest::from_document(snapshot, item)
        .map_err(|error| {
            layout_failure(
                "c114_image_request",
                Some(item.id().get()),
                Some(error.to_string()),
            )
        })?
        .decode(PNG)
        .map_err(|error| {
            layout_failure(
                "c114_image_decode",
                Some(item.id().get()),
                Some(error.to_string()),
            )
        })?;
    let revision = IntrinsicSizeRevision::INITIAL
        .checked_next()
        .ok_or_else(|| {
            layout_failure("c114_image_revision_exhausted", Some(item.id().get()), None)
        })?;
    let resources =
        ImageResourceSnapshot::new(snapshot, revision, [completion]).map_err(|error| {
            layout_failure(
                "c114_image_resource_snapshot",
                Some(item.id().get()),
                Some(error.to_string()),
            )
        })?;
    let decoded = resources.images().get(&item.id()).ok_or_else(|| {
        layout_failure("c114_image_metadata_missing", Some(item.id().get()), None)
    })?;
    let dimensions = decoded.dimensions();
    if dimensions.width() != NATURAL_WIDTH || dimensions.height() != NATURAL_HEIGHT {
        return Err(layout_failure(
            "c114_image_metadata_mismatch",
            Some(item.id().get()),
            Some(format!("{}x{}", dimensions.width(), dimensions.height())),
        ));
    }
    let intrinsic_sizes = resources
        .to_layout_intrinsic_size_snapshot()
        .map_err(|error| {
            layout_failure(
                "c114_image_intrinsic_snapshot",
                Some(item.id().get()),
                Some(error.to_string()),
            )
        })?;

    Ok(Some(super::C11_4ImageInput { intrinsic_sizes }))
}

fn images_in_subtree(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
) -> Result<Vec<HostNodeHandle>, RuntimeLayoutFailure> {
    let mut pending = vec![root];
    let mut images = Vec::new();
    while let Some(handle) = pending.pop() {
        let node = snapshot
            .node(handle)
            .ok_or_else(|| layout_failure("missing_node", Some(handle.id().get()), None))?;
        if let HostNodeKind::Element(element) = node.kind()
            && element.namespace() == HTML_NAMESPACE
            && element.local_name() == "img"
        {
            images.push(handle);
        }
        pending.extend(snapshot.children(handle).into_iter().flatten());
    }
    Ok(images)
}

fn element_identity(
    snapshot: &HostDocumentSnapshot,
    handle: HostNodeHandle,
) -> Result<(String, String, String), RuntimeLayoutFailure> {
    let Some(node) = snapshot.node(handle) else {
        return Err(layout_failure(
            "missing_node",
            Some(handle.id().get()),
            None,
        ));
    };
    let HostNodeKind::Element(element) = node.kind() else {
        return Err(layout_failure(
            "expected_element",
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
    Ok((
        element.namespace().to_owned(),
        element.local_name().to_owned(),
        id,
    ))
}

fn image_src(
    snapshot: &HostDocumentSnapshot,
    image: HostNodeHandle,
) -> Result<Option<String>, RuntimeLayoutFailure> {
    let node = snapshot
        .node(image)
        .ok_or_else(|| layout_failure("missing_node", Some(image.id().get()), None))?;
    let HostNodeKind::Element(element) = node.kind() else {
        return Err(layout_failure(
            "expected_element",
            Some(image.id().get()),
            None,
        ));
    };
    let name = AttributeName::new(None, "src")
        .ok_or_else(|| layout_failure("invalid_attribute_name", Some(image.id().get()), None))?;
    Ok(element
        .attribute(&name)
        .map(|value| value.to_string_lossy()))
}

fn validate_grid_style(
    style: &ComputedElementStyle,
    case: &ImageCase,
) -> Result<(), RuntimeLayoutFailure> {
    for (name, value) in [
        ("display", "grid"),
        ("box-sizing", "border-box"),
        ("align-items", "start"),
        ("align-content", "start"),
        ("justify-content", "start"),
    ] {
        require_property(style, name, value)?;
    }
    if style.layout_dimensions.width != ComputedCssDimension::LengthPx(case.grid_width)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(case.grid_height)
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || !spacing_is(style.layout_spacing.row_gap, case.row_gap)
        || !spacing_is(style.layout_spacing.column_gap, case.column_gap)
        || style.layout_border
            != (spinon_style::ComputedLayoutBorder {
                top: case.grid_border,
                right: case.grid_border,
                bottom: case.grid_border,
                left: case.grid_border,
            })
    {
        return Err(unsupported_case(
            style.node_id.get(),
            case,
            "container box geometry",
        ));
    }
    let grid = style.grid_layout.as_ref().ok_or_else(|| {
        layout_failure(
            "missing_c114_image_grid_layout",
            Some(style.node_id.get()),
            Some(case.scene_id.to_owned()),
        )
    })?;
    if grid.columns.len() != case.columns.len()
        || !grid
            .columns
            .iter()
            .zip(case.columns)
            .all(|(actual, expected)| track_matches(actual, *expected))
        || grid.rows.len() != case.rows.len()
        || !grid
            .rows
            .iter()
            .zip(case.rows)
            .all(|(actual, expected)| track_matches(actual, *expected))
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
        return Err(unsupported_case(
            style.node_id.get(),
            case,
            "track functions",
        ));
    }
    Ok(())
}

fn validate_item_style(
    style: &ComputedElementStyle,
    case: &ImageCase,
) -> Result<(), RuntimeLayoutFailure> {
    require_property(style, "display", "block")?;
    let (
        width,
        height,
        min_width,
        min_height,
        max_width,
        max_height,
        aspect_ratio,
        padding,
        border,
        box_sizing,
    ) = match case.sizing {
        ImageSizing::Intrinsic => (
            ComputedCssDimension::Auto,
            ComputedCssDimension::Auto,
            ComputedCssDimension::Auto,
            ComputedCssDimension::Auto,
            ComputedCssMaxSize::None,
            ComputedCssMaxSize::None,
            None,
            0.0,
            0.0,
            "border-box",
        ),
        ImageSizing::ColumnAutoNaturalRatio => (
            ComputedCssDimension::Auto,
            ComputedCssDimension::LengthPx(8.0),
            ComputedCssDimension::Auto,
            ComputedCssDimension::Auto,
            ComputedCssMaxSize::None,
            ComputedCssMaxSize::None,
            None,
            0.0,
            0.0,
            "border-box",
        ),
        ImageSizing::ColumnTargetMaxWidth => (
            ComputedCssDimension::Auto,
            ComputedCssDimension::LengthPx(8.0),
            ComputedCssDimension::Auto,
            ComputedCssDimension::Auto,
            ComputedCssMaxSize::LengthPx(9.0),
            ComputedCssMaxSize::None,
            Some(1.5),
            0.0,
            0.0,
            "border-box",
        ),
        ImageSizing::RowTargetMaxHeight => (
            ComputedCssDimension::LengthPx(12.0),
            ComputedCssDimension::Auto,
            ComputedCssDimension::Auto,
            ComputedCssDimension::Auto,
            ComputedCssMaxSize::None,
            ComputedCssMaxSize::LengthPx(6.0),
            Some(1.5),
            0.0,
            0.0,
            "border-box",
        ),
        ImageSizing::ColumnOppositeMinOverMax => (
            ComputedCssDimension::Auto,
            ComputedCssDimension::LengthPx(8.0),
            ComputedCssDimension::Auto,
            ComputedCssDimension::LengthPx(12.0),
            ComputedCssMaxSize::None,
            ComputedCssMaxSize::LengthPx(4.0),
            Some(1.5),
            0.0,
            0.0,
            "border-box",
        ),
        ImageSizing::ColumnBorderBoxTransfer => (
            ComputedCssDimension::Auto,
            ComputedCssDimension::LengthPx(8.0),
            ComputedCssDimension::Auto,
            ComputedCssDimension::Auto,
            ComputedCssMaxSize::None,
            ComputedCssMaxSize::None,
            Some(1.5),
            1.0,
            1.0,
            "border-box",
        ),
        ImageSizing::ColumnContentBoxTransfer => (
            ComputedCssDimension::Auto,
            ComputedCssDimension::LengthPx(8.0),
            ComputedCssDimension::Auto,
            ComputedCssDimension::Auto,
            ComputedCssMaxSize::None,
            ComputedCssMaxSize::None,
            Some(1.5),
            1.0,
            1.0,
            "content-box",
        ),
        ImageSizing::ColumnTargetMinWidth => (
            ComputedCssDimension::Auto,
            ComputedCssDimension::LengthPx(8.0),
            ComputedCssDimension::LengthPx(18.0),
            ComputedCssDimension::Auto,
            ComputedCssMaxSize::None,
            ComputedCssMaxSize::None,
            Some(1.5),
            0.0,
            0.0,
            "border-box",
        ),
        ImageSizing::RowTransferInputCappedByFixedColumnSpan => (
            ComputedCssDimension::LengthPx(18.0),
            ComputedCssDimension::Auto,
            ComputedCssDimension::Auto,
            ComputedCssDimension::Auto,
            ComputedCssMaxSize::None,
            ComputedCssMaxSize::None,
            Some(1.5),
            0.0,
            0.0,
            "border-box",
        ),
        ImageSizing::ColumnTransferInputCappedByFixedRowSpan => (
            ComputedCssDimension::Auto,
            ComputedCssDimension::LengthPx(18.0),
            ComputedCssDimension::Auto,
            ComputedCssDimension::Auto,
            ComputedCssMaxSize::None,
            ComputedCssMaxSize::None,
            Some(1.5),
            0.0,
            0.0,
            "border-box",
        ),
        ImageSizing::FixedGutterColumnSuggestionClamp => (
            ComputedCssDimension::LengthPx(36.0),
            ComputedCssDimension::Auto,
            ComputedCssDimension::Auto,
            ComputedCssDimension::Auto,
            ComputedCssMaxSize::None,
            ComputedCssMaxSize::None,
            Some(1.5),
            0.0,
            0.0,
            "border-box",
        ),
        ImageSizing::FixedGutterRowSuggestionClamp => (
            ComputedCssDimension::Auto,
            ComputedCssDimension::LengthPx(36.0),
            ComputedCssDimension::Auto,
            ComputedCssDimension::Auto,
            ComputedCssMaxSize::None,
            ComputedCssMaxSize::None,
            Some(1.5),
            0.0,
            0.0,
            "border-box",
        ),
    };
    require_property(style, "box-sizing", box_sizing)?;
    if style.layout_dimensions.width != width
        || style.layout_dimensions.height != height
        || style.layout_dimensions.min_width != min_width
        || style.layout_dimensions.min_height != min_height
        || style.layout_dimensions.max_width != max_width
        || style.layout_dimensions.max_height != max_height
        || style.layout_aspect_ratio != aspect_ratio
        || !zero_edges(style.layout_spacing.margin)
        || !edges_are(style.layout_spacing.padding, padding)
        || style.layout_border
            != (spinon_style::ComputedLayoutBorder {
                top: border,
                right: border,
                bottom: border,
                left: border,
            })
        || style.layout_position != spinon_style::ComputedCssPosition::Static
        || !placement_matches(style.grid_item_placement.as_ref(), case.placement)
    {
        return Err(unsupported_case(
            style.node_id.get(),
            case,
            "replaced item box or placement",
        ));
    }
    Ok(())
}

fn placement_matches(
    placement: Option<&ComputedCssGridItemPlacement>,
    expected: Placement,
) -> bool {
    match (placement, expected) {
        (None, Placement::Auto) => true,
        (Some(value), Placement::Auto) => {
            value.row_start == ComputedCssGridPlacement::Auto
                && value.row_end == ComputedCssGridPlacement::Auto
                && value.column_start == ComputedCssGridPlacement::Auto
                && value.column_end == ComputedCssGridPlacement::Auto
        }
        (Some(value), Placement::ColumnSpan2) => {
            value.row_start == ComputedCssGridPlacement::Auto
                && value.row_end == ComputedCssGridPlacement::Auto
                && value.column_start == ComputedCssGridPlacement::Line(1)
                && value.column_end == ComputedCssGridPlacement::Span(2)
        }
        (Some(value), Placement::RowSpan2) => {
            value.row_start == ComputedCssGridPlacement::Line(1)
                && value.row_end == ComputedCssGridPlacement::Span(2)
                && value.column_start == ComputedCssGridPlacement::Auto
                && value.column_end == ComputedCssGridPlacement::Auto
        }
        (None, Placement::ColumnSpan2 | Placement::RowSpan2) => false,
    }
}

fn track_matches(actual: &ComputedCssGridTrackSize, expected: Track) -> bool {
    match (actual, expected) {
        (ComputedCssGridTrackSize::Auto, Track::Auto)
        | (ComputedCssGridTrackSize::MinContent, Track::MinContent) => true,
        (
            ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::LengthPx(min),
                max: ComputedCssGridTrackBreadth::Auto,
            },
            Track::MinMaxZeroAuto,
        ) => approximately(*min, 0.0),
        (
            ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::Auto,
                max: ComputedCssGridTrackBreadth::Fraction(fraction),
            },
            Track::AutoFraction,
        ) => approximately(*fraction, 1.0),
        (
            ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::MinContent,
                max: ComputedCssGridTrackBreadth::MaxContent,
            },
            Track::MinMaxMinMax,
        ) => true,
        (
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
                actual,
            )),
            Track::FitContent12,
        ) => approximately(*actual, 12.0),
        (
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(
                actual,
            )),
            Track::FitContent40,
        ) => approximately(*actual, 40.0),
        (
            ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::Percentage(
                actual,
            )),
            Track::FitContent50Percent,
        ) => approximately(*actual, 0.5),
        (
            ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::LengthPx(min),
                max: ComputedCssGridTrackBreadth::LengthPx(max),
            },
            Track::Fixed12,
        ) => approximately(*min, 0.0) && approximately(*max, 12.0),
        (
            ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::Auto,
                max: ComputedCssGridTrackBreadth::LengthPx(max),
            },
            Track::FixedEight,
        ) => approximately(*max, 8.0),
        (
            ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::Auto,
                max: ComputedCssGridTrackBreadth::LengthPx(max),
            },
            Track::MinMaxAuto3,
        ) => approximately(*max, 3.0),
        (ComputedCssGridTrackSize::LengthPx(actual), Track::Length100) => {
            approximately(*actual, 100.0)
        }
        (ComputedCssGridTrackSize::LengthPx(actual), Track::Length20) => {
            approximately(*actual, 20.0)
        }
        (ComputedCssGridTrackSize::LengthPx(actual), Track::Length4) => approximately(*actual, 4.0),
        _ => false,
    }
}

fn require_property(
    style: &ComputedElementStyle,
    name: &str,
    expected: &str,
) -> Result<(), RuntimeLayoutFailure> {
    if style.properties.get(name).map(String::as_str) == Some(expected) {
        Ok(())
    } else {
        Err(layout_failure(
            "unsupported_c114_image_style",
            Some(style.node_id.get()),
            Some(format!("{} expected {expected}", name)),
        ))
    }
}

fn zero_edges(edges: spinon_style::ComputedCssEdges) -> bool {
    edges_are(edges, 0.0)
}

fn edges_are(edges: spinon_style::ComputedCssEdges, expected: f32) -> bool {
    spacing_is(edges.top, expected)
        && spacing_is(edges.right, expected)
        && spacing_is(edges.bottom, expected)
        && spacing_is(edges.left, expected)
}

fn spacing_is(actual: ComputedCssSpacingValue, expected: f32) -> bool {
    matches!(actual, ComputedCssSpacingValue::LengthPx(value) if approximately(value, expected))
}

fn approximately(actual: f32, expected: f32) -> bool {
    (actual - expected).abs() <= f32::EPSILON
}

fn unsupported_case(node_id: u64, case: &ImageCase, detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_image_case",
        Some(node_id),
        Some(format!("{}: {detail}", case.scene_id)),
    )
}

#[cfg(test)]
#[path = "c11_4_image/tests.rs"]
mod tests;
