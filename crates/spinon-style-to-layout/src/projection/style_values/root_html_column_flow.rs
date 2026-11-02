use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind};
use spinon_layout::GRID_MAX_TRACKS;
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackSize,
    ComputedStyleSnapshot,
};

use super::{HTML_NAMESPACE, RuntimeGridNodes};
use crate::StyleLayoutError;

pub(super) fn validate_candidate(
    tree: &HostDocumentSnapshot,
    snapshot: &ComputedStyleSnapshot,
    nodes: &RuntimeGridNodes,
) -> Result<(), StyleLayoutError> {
    let mut candidates = snapshot.elements.iter().filter(|element| {
        nodes.grid_containers.contains(&element.node_id)
            && element
                .grid_layout
                .as_ref()
                .is_some_and(|grid| grid.auto_flow == ComputedCssGridAutoFlow::Column)
    });
    let Some(container) = candidates.next() else {
        return Ok(());
    };
    if candidates.next().is_some() || nodes.grid_containers.len() != 1 {
        return Err(unsupported(
            container.node_id,
            "grid-auto-flow",
            "Root HTML column-flow mode supports one non-nested Grid container",
        ));
    }
    let node = container.node_id;
    let grid = container
        .grid_layout
        .as_ref()
        .expect("column-flow candidates were filtered by computed Grid style");
    for (property, groups) in [
        (
            "grid-template-columns",
            grid.columns_fixed_repeat_group_count,
        ),
        ("grid-template-rows", grid.rows_fixed_repeat_group_count),
    ] {
        if groups == 0 {
            continue;
        }
        return Err(unsupported(
            node,
            property,
            "Root HTML column-flow mode excludes repeat() in explicit tracks",
        ));
    }
    if container.properties.get("direction").map(String::as_str) != Some("ltr") {
        return Err(unsupported(
            node,
            "direction",
            "Root HTML column-flow mode requires computed direction:ltr",
        ));
    }
    if container.properties.get("writing-mode").map(String::as_str) != Some("horizontal-tb") {
        return Err(unsupported(
            node,
            "writing-mode",
            "Root HTML column-flow mode requires computed writing-mode:horizontal-tb",
        ));
    }
    if !root_html_auto_columns_supported(&grid.auto_columns) {
        return Err(unsupported(
            node,
            "grid-auto-columns",
            "Root HTML column-flow mode requires auto or a finite non-negative px value/list",
        ));
    }
    if grid.template_areas.is_some()
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || !matches!(grid.auto_rows, ComputedCssGridAutoRows::Auto)
        || grid.columns.is_empty()
        || grid.rows.is_empty()
        || grid.columns.iter().chain(&grid.rows).any(|track| {
            !matches!(
                track,
                ComputedCssGridTrackSize::LengthPx(value)
                    if value.is_finite() && *value >= 0.0
            )
        })
    {
        return Err(unsupported(
            node,
            "grid-auto-flow",
            "Root HTML column-flow mode requires direct px explicit tracks, auto rows, no areas, and no repeat",
        ));
    }
    let container_handle = nodes
        .grid_container_handles
        .get(&node)
        .copied()
        .ok_or_else(|| {
            unsupported(
                node,
                "grid-auto-flow",
                "computed Grid container has no matching HostDocument node",
            )
        })?;
    let visible_item_count = tree
        .children(container_handle)
        .into_iter()
        .flatten()
        .filter(|child| nodes.grid_item_owners.get(&child.id()) == Some(&node))
        .count();
    let required_columns = visible_item_count.div_ceil(grid.rows.len());
    let total_columns = required_columns.max(grid.columns.len());
    if total_columns > GRID_MAX_TRACKS {
        return Err(unsupported(
            node,
            "grid-template-columns",
            "Root HTML column-flow mode exceeds the 10,000 explicit-plus-implicit column limit",
        ));
    }
    if required_columns > grid.columns.len()
        && matches!(grid.auto_columns, ComputedCssGridAutoColumns::Auto)
    {
        return Err(unsupported(
            node,
            "grid-auto-columns",
            "implicit columns require fixed-px grid-auto-columns in the Root HTML column-flow profile",
        ));
    }
    for item in tree
        .children(container_handle)
        .into_iter()
        .flatten()
        .filter(|child| nodes.grid_item_owners.get(&child.id()) == Some(&node))
    {
        validate_item(tree, snapshot, item)?;
    }
    Ok(())
}

fn root_html_auto_columns_supported(auto_columns: &ComputedCssGridAutoColumns) -> bool {
    let valid_px = |track: &ComputedCssGridTrackSize| matches!(track, ComputedCssGridTrackSize::LengthPx(value) if value.is_finite() && *value >= 0.0);
    match auto_columns {
        ComputedCssGridAutoColumns::Auto => true,
        ComputedCssGridAutoColumns::LengthPx(value) => value.is_finite() && *value >= 0.0,
        ComputedCssGridAutoColumns::TrackList(tracks) => {
            !tracks.is_empty() && tracks.len() <= GRID_MAX_TRACKS && tracks.iter().all(valid_px)
        }
        ComputedCssGridAutoColumns::Unsupported(_) => false,
    }
}

fn validate_item(
    tree: &HostDocumentSnapshot,
    snapshot: &ComputedStyleSnapshot,
    item: HostNodeHandle,
) -> Result<(), StyleLayoutError> {
    let node = item.id();
    let host_node = tree
        .node(item)
        .ok_or_else(|| unsupported(node, "grid-item", "HostDocument item is missing"))?;
    let is_empty_html_div = matches!(
        host_node.kind(),
        HostNodeKind::Element(element)
            if element.namespace() == HTML_NAMESPACE && element.local_name() == "div"
    ) && tree
        .children(item)
        .is_none_or(|mut children| children.next().is_none());
    if !is_empty_html_div {
        return Err(unsupported(
            node,
            "grid-item",
            "Root HTML column-flow mode requires direct empty HTML div items",
        ));
    }
    let style = snapshot
        .elements
        .iter()
        .find(|element| element.node_id == node)
        .ok_or_else(|| unsupported(node, "grid-item", "computed item style is missing"))?;
    let Some(placement) = style.grid_item_placement.as_ref() else {
        return Err(unsupported(
            node,
            "grid-row-start",
            "computed four-axis Grid placement is missing",
        ));
    };
    if ![
        &placement.row_start,
        &placement.row_end,
        &placement.column_start,
        &placement.column_end,
    ]
    .into_iter()
    .all(|value| matches!(value, ComputedCssGridPlacement::Auto))
    {
        return Err(unsupported(
            node,
            "grid-row-start",
            "Root HTML column-flow mode requires all four placement longhands to be auto",
        ));
    }
    let fixed_nonnegative = |value| matches!(value, ComputedCssDimension::LengthPx(value) if value.is_finite() && value >= 0.0);
    if !fixed_nonnegative(style.layout_dimensions.width)
        || !fixed_nonnegative(style.layout_dimensions.height)
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != spinon_style::ComputedCssMaxSize::None
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_height != spinon_style::ComputedCssMaxSize::None
        || style.layout_aspect_ratio.is_some()
        || style.layout_math_values.contains_key("width")
        || style.layout_math_values.contains_key("height")
    {
        return Err(unsupported(
            node,
            "width",
            "Root HTML column-flow mode requires fixed finite px item dimensions without min/max, aspect ratio, or CSS math",
        ));
    }
    Ok(())
}

fn unsupported(node: spinon_core::NodeId, property: &'static str, value: &str) -> StyleLayoutError {
    StyleLayoutError::UnsupportedComputedValue {
        node,
        property,
        value: value.to_owned(),
    }
}
