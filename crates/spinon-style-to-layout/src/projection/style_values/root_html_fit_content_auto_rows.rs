use spinon_core::{HostDocumentSnapshot, HostNodeKind};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoRows, ComputedCssGridPlacement,
    ComputedCssGridTrackSize, ComputedCssMaxSize, ComputedCssPosition, ComputedCssSpacingValue,
    ComputedElementStyle, ComputedStyleSnapshot,
};

use super::{RuntimeGridNodes, StyleLayoutError, grid_values};

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
#[path = "root_html_fit_content_auto_rows/repeated_text.rs"]
mod repeated_text;
#[path = "root_html_fit_content_auto_rows/validation_helpers.rs"]
mod validation_helpers;

use validation_helpers::{
    auto_placement, automatic_limits, direct_element_children, explicit_empty_span_item_placement,
    fit_content_item, fixed_item, repeated_span_fit_positions_are_supported,
    repeated_span_item_kind, repeated_span_item_placement, repeated_span_topology_implicit_rows,
    repeated_span_writing_context, validate_payload, zero_border, zero_edges,
};

pub(super) fn validate(
    tree: &HostDocumentSnapshot,
    snapshot: &ComputedStyleSnapshot,
    nodes: &RuntimeGridNodes,
) -> Result<(), StyleLayoutError> {
    let candidates = snapshot
        .elements
        .iter()
        .filter(|element| {
            nodes.grid_containers.contains(&element.node_id)
                && element
                    .grid_layout
                    .as_ref()
                    .is_some_and(grid_values::root_html_fit_content_implicit_rows_candidate)
        })
        .collect::<Vec<_>>();
    if candidates.is_empty() {
        return Ok(());
    }

    let node = candidates[0].node_id;
    let unsupported =
        |node, property, value: &'static str| StyleLayoutError::UnsupportedComputedValue {
            node,
            property,
            value: value.to_owned(),
        };
    if candidates.len() != 1 || nodes.grid_containers.len() != 1 {
        return Err(unsupported(
            node,
            "grid-auto-rows",
            "Root HTML fit-content rows require exactly one visible Grid owner",
        ));
    }

    let grid_style = candidates[0];
    let grid = grid_style
        .grid_layout
        .as_ref()
        .expect("fit-content candidate has computed Grid tracks");
    let four_member_list =
        grid_values::root_html_fit_content_auto_rows_with_four_member_intrinsic_tracks(grid);
    let six_member_list =
        grid_values::root_html_fit_content_auto_rows_with_six_member_intrinsic_tracks(grid);
    let six_member_auto_placement_list = grid_values::root_html_fit_content_auto_rows_with_six_member_auto_placement_intrinsic_tracks(grid);
    let six_member_span_list =
        grid_values::root_html_fit_content_auto_rows_with_six_member_span_intrinsic_tracks(grid);
    let six_member_candidate = six_member_auto_placement_list || six_member_span_list;
    let span_fit_positions = match &grid.auto_rows {
        ComputedCssGridAutoRows::TrackList(tracks) => tracks
            .iter()
            .enumerate()
            .filter_map(|(position, track)| {
                matches!(track, ComputedCssGridTrackSize::FitContent(_)).then_some(position + 1)
            })
            .collect::<Vec<_>>(),
        _ => Vec::new(),
    };
    let two_fit_span_track_list = six_member_span_list && span_fit_positions.len() == 2;
    let align_content = grid_style
        .properties
        .get("align-content")
        .map(String::as_str);
    let align_content_is_supported = align_content == Some("start")
        || (grid_values::root_html_fit_content_auto_rows_with_intrinsic_tracks(grid)
            && align_content == Some("stretch"));
    if !align_content_is_supported {
        return Err(unsupported(
            node,
            "align-content",
            "Root HTML fit-content rows support only start or auto-track stretch alignment",
        ));
    }
    if grid.columns.as_slice() != [ComputedCssGridTrackSize::LengthPx(40.0)]
        || grid.rows.as_slice() != [ComputedCssGridTrackSize::LengthPx(10.0)]
        || grid
            .columns_line_names
            .iter()
            .any(|group| !group.is_empty())
        || grid.rows_line_names.iter().any(|group| !group.is_empty())
    {
        return Err(unsupported(
            node,
            "grid-template-rows",
            "Root HTML fit-content rows require the fixed 40px column and 10px explicit row shape",
        ));
    }
    if grid_style.layout_dimensions.width != ComputedCssDimension::LengthPx(40.0)
        || !automatic_limits(grid_style)
        || grid_style.layout_aspect_ratio.is_some()
        || grid_style.layout_position != ComputedCssPosition::Static
        || !zero_edges(grid_style.layout_spacing.margin)
        || !zero_edges(grid_style.layout_spacing.padding)
        || grid_style.layout_spacing.row_gap != ComputedCssSpacingValue::LengthPx(0.0)
        || grid_style.layout_spacing.column_gap != ComputedCssSpacingValue::LengthPx(0.0)
        || !zero_border(grid_style)
        || grid_style.properties.get("box-sizing").map(String::as_str) != Some("border-box")
        || grid_style.properties.get("align-items").map(String::as_str) != Some("normal")
        || grid_style
            .properties
            .get("justify-content")
            .map(String::as_str)
            != Some("start")
        || grid_style
            .properties
            .get("justify-items")
            .map(String::as_str)
            != Some("normal")
        || grid_style.properties.get("overflow-x").map(String::as_str) != Some("visible")
        || grid_style.properties.get("overflow-y").map(String::as_str) != Some("visible")
    {
        return Err(unsupported(
            node,
            "grid-auto-rows",
            "Root HTML fit-content rows require positive definite px axes, zero spacing and gaps, static positioning, start alignment, and visible overflow",
        ));
    }

    let Some(owner) = nodes.grid_container_handles.get(&node).copied() else {
        return Err(unsupported(
            node,
            "grid-auto-rows",
            "Root HTML fit-content rows require an inspectable parsed Grid owner",
        ));
    };
    let children = tree
        .children(owner)
        .ok_or_else(|| unsupported(node, "grid-auto-rows", "Grid item children are unavailable"))?
        .collect::<Vec<_>>();
    let mut items = Vec::new();
    for child in children {
        if nodes.hidden.contains(&child.id()) {
            continue;
        }
        let child_node = tree
            .node(child)
            .ok_or_else(|| unsupported(node, "grid-auto-rows", "Grid item is unavailable"))?;
        match child_node.kind() {
            HostNodeKind::Text(text) if text.to_string_lossy().trim().is_empty() => {}
            HostNodeKind::Element(element)
                if element.namespace() == HTML_NAMESPACE
                    && element.local_name() == "div"
                    && !nodes.grid_containers.contains(&child.id()) =>
            {
                items.push(child);
            }
            _ => {
                return Err(unsupported(
                    child.id(),
                    "grid-auto-rows",
                    "Root HTML fit-content rows require three or five items, six for the four-member list, or nine for the repeated six-member list",
                ));
            }
        }
    }
    let styles = snapshot
        .elements
        .iter()
        .map(|style| (style.node_id, style))
        .collect::<std::collections::BTreeMap<_, _>>();
    let repeated_text_profile = six_member_auto_placement_list
        && items.len() == 9
        && items.iter().copied().skip(1).all(|item| {
            let Ok(children) = direct_element_children(tree, nodes, item) else {
                return false;
            };
            let [payload] = children.as_slice() else {
                return false;
            };
            repeated_text::payload_has_supported_text(tree, *payload)
        });
    let repeated_text_auto_minimum_profile = repeated_text_profile
        && items
            .first()
            .and_then(|item| styles.get(&item.id()).copied())
            .is_some_and(|style| {
                style.layout_dimensions.min_height == ComputedCssDimension::LengthPx(0.0)
            })
        && items.iter().copied().skip(1).all(|item| {
            styles.get(&item.id()).copied().is_some_and(|style| {
                style.layout_dimensions.min_height == ComputedCssDimension::Auto
            })
        });
    let repeated_text_span_profile = six_member_span_list
        && repeated_span_writing_context(grid_style)
        && items.iter().copied().all(|item| {
            styles
                .get(&item.id())
                .copied()
                .is_some_and(repeated_span_writing_context)
        })
        && items
            .first()
            .and_then(|item| styles.get(&item.id()).copied())
            .is_some_and(explicit_empty_span_item_placement)
        && items.iter().copied().skip(1).all(|item| {
            let Some(style) = styles.get(&item.id()).copied() else {
                return false;
            };
            let Ok(children) = direct_element_children(tree, nodes, item) else {
                return false;
            };
            let [payload] = children.as_slice() else {
                return false;
            };
            repeated_span_item_placement(style)
                && repeated_text::payload_has_supported_text(tree, *payload)
        })
        && {
            let placements = items
                .iter()
                .copied()
                .skip(1)
                .map(|item| {
                    styles
                        .get(&item.id())
                        .copied()
                        .and_then(repeated_span_item_kind)
                })
                .collect::<Option<Vec<_>>>();
            placements
                .as_deref()
                .and_then(repeated_span_topology_implicit_rows)
                .is_some_and(|implicit_rows| {
                    repeated_span_fit_positions_are_supported(implicit_rows, &span_fit_positions)
                })
        };
    if (four_member_list && items.len() != 6)
        || (six_member_candidate && items.len() != 9 && !repeated_text_span_profile)
        || (two_fit_span_track_list && !repeated_text_span_profile && !repeated_text_profile)
        || (!four_member_list && !six_member_candidate && !matches!(items.len(), 3 | 5))
        || nodes
            .grid_item_owners
            .values()
            .filter(|owner| **owner == node)
            .count()
            != items.len()
    {
        return Err(unsupported(
            node,
            "grid-auto-rows",
            "Root HTML fit-content rows require three or five direct items, six for the four-member list, nine for the repeated auto-placement profile, or the bounded explicit row-span profile",
        ));
    }
    let grid_height_is_supported = grid_style.layout_dimensions.height
        == ComputedCssDimension::LengthPx(100.0)
        || ((repeated_text_profile || repeated_text_span_profile)
            && grid_style.layout_dimensions.height == ComputedCssDimension::LengthPx(220.0));
    if !grid_height_is_supported {
        return Err(unsupported(
            node,
            "grid-auto-rows",
            "Root HTML fit-content rows require a supported definite grid block size",
        ));
    }
    let Some(first) = styles.get(&items[0].id()).copied() else {
        return Err(unsupported(
            node,
            "grid-auto-rows",
            "First Grid item style is missing",
        ));
    };
    let first_placement_is_supported = if repeated_text_span_profile {
        explicit_empty_span_item_placement(first)
    } else {
        auto_placement(first)
    };
    if !fixed_item(
        first,
        10.0,
        repeated_text_profile || repeated_text_span_profile,
    ) || !first_placement_is_supported
    {
        return Err(unsupported(
            node,
            "grid-auto-rows",
            "Root HTML fit-content rows require a 10px explicit-row item",
        ));
    }
    const LIST_PAYLOAD_HEIGHTS: &[f32] = &[6.0, 8.0, 10.0, 25.0, 30.0, 34.0, 48.0, 50.0];
    const FOUR_MEMBER_LIST_PAYLOAD_HEIGHTS: &[f32] = &[20.0, 10.0, 12.0, 6.0, 20.0];
    let six_member_payload_height_profiles = if six_member_list {
        let ComputedCssGridAutoRows::TrackList(tracks) = &grid.auto_rows else {
            return Err(unsupported(
                node,
                "grid-auto-rows",
                "Repeated intrinsic list requires typed implicit rows",
            ));
        };
        let Some(fit_position) = tracks
            .iter()
            .position(|track| matches!(track, ComputedCssGridTrackSize::FitContent(_)))
        else {
            return Err(unsupported(
                node,
                "grid-auto-rows",
                "Repeated intrinsic list requires one fit-content track",
            ));
        };
        // Keep the numeric fixture bounded to its historical first-position profile.
        (fit_position == 0)
            .then(|| repeated_intrinsic_payload_height_profiles(fit_position, tracks.len()))
    } else {
        None
    };
    let mut matching_six_member_payload_profiles = [true; 2];
    const FIT_CONTENT_OVERFLOW_HEIGHT: f32 = 60.0;
    if !direct_element_children(tree, nodes, items[0])?.is_empty() {
        return Err(unsupported(
            items[0].id(),
            "grid-auto-rows",
            "The 10px explicit-row item must not contain descendants",
        ));
    }
    for (position, item) in items.iter().enumerate().skip(1) {
        let Some(style) = styles.get(&item.id()).copied() else {
            return Err(unsupported(
                item.id(),
                "grid-auto-rows",
                "Implicit-row item style is missing",
            ));
        };
        let allow_minimum_floor = !repeated_text_span_profile && items.len() == 3 && position == 1;
        let allow_auto_minimum = (two_fit_span_track_list && repeated_text_span_profile)
            || (six_member_auto_placement_list && repeated_text_auto_minimum_profile);
        let item_placement_is_supported = if repeated_text_span_profile {
            repeated_span_item_placement(style)
        } else {
            auto_placement(style)
        };
        if fit_content_item(style, allow_minimum_floor, allow_auto_minimum)
            && item_placement_is_supported
        {
            let children = direct_element_children(tree, nodes, *item)?;
            if children.len() != 1 {
                return Err(unsupported(
                    item.id(),
                    "grid-auto-rows",
                    "Each intrinsic list item must contain one fixed-px payload div",
                ));
            }
            let profile_payload_heights = if four_member_list {
                Some(FOUR_MEMBER_LIST_PAYLOAD_HEIGHTS)
            } else {
                None
            };
            if repeated_text_profile || repeated_text_span_profile {
                if !repeated_text::payload_is_supported(tree, nodes, snapshot, &styles, children[0])
                {
                    return Err(unsupported(
                        children[0].id(),
                        "grid-auto-rows",
                        "Repeated intrinsic text payload must match the fixed monospace text profile",
                    ));
                }
            } else if six_member_list {
                let Some(all_profiles) = six_member_payload_height_profiles.as_ref() else {
                    return Err(unsupported(
                        item.id(),
                        "grid-auto-rows",
                        "Repeated intrinsic list payload position is unavailable",
                    ));
                };
                let Some(profiles) = all_profiles
                    .iter()
                    .map(|heights| heights.get(position - 1))
                    .collect::<Option<Vec<_>>>()
                else {
                    return Err(unsupported(
                        item.id(),
                        "grid-auto-rows",
                        "Repeated intrinsic list payload position is unavailable",
                    ));
                };
                let allowed_heights = profiles.iter().map(|height| **height).collect::<Vec<_>>();
                validate_payload(tree, nodes, &styles, children[0], &allowed_heights)?;
                let payload_style = styles.get(&children[0].id()).copied().ok_or_else(|| {
                    unsupported(
                        children[0].id(),
                        "height",
                        "Nested payload style is missing",
                    )
                })?;
                for (profile_index, profile) in all_profiles.iter().enumerate() {
                    matching_six_member_payload_profiles[profile_index] &=
                        fixed_item(payload_style, profile[position - 1], false);
                }
                if !matching_six_member_payload_profiles
                    .iter()
                    .any(|matches| *matches)
                {
                    return Err(unsupported(
                        children[0].id(),
                        "height",
                        "Repeated intrinsic items must match one complete fixed-px payload profile",
                    ));
                }
            } else if let Some(profile_payload_heights) = profile_payload_heights {
                let expected_height = profile_payload_heights[position - 1];
                let ComputedCssGridAutoRows::TrackList(tracks) = &grid.auto_rows else {
                    return Err(unsupported(
                        item.id(),
                        "grid-auto-rows",
                        "Intrinsic list requires typed implicit rows",
                    ));
                };
                let track_index = (position - 1) % tracks.len();
                if matches!(tracks[track_index], ComputedCssGridTrackSize::FitContent(_)) {
                    let fit_content_payload_heights =
                        [expected_height, FIT_CONTENT_OVERFLOW_HEIGHT];
                    validate_payload(
                        tree,
                        nodes,
                        &styles,
                        children[0],
                        &fit_content_payload_heights,
                    )?;
                } else {
                    validate_payload(
                        tree,
                        nodes,
                        &styles,
                        children[0],
                        std::slice::from_ref(&expected_height),
                    )?;
                }
            } else {
                validate_payload(tree, nodes, &styles, children[0], LIST_PAYLOAD_HEIGHTS)?;
            }
        } else if items.len() == 3
            && position == 2
            && fixed_item(style, 6.0, false)
            && auto_placement(style)
            && direct_element_children(tree, nodes, *item)?.is_empty()
        {
            // The original singleton fixture uses a fixed 6px item as a following probe.
        } else {
            return Err(unsupported(
                item.id(),
                "grid-auto-rows",
                "Implicit list items require auto size and one payload, except for the fixed 6px singleton probe",
            ));
        }
    }
    Ok(())
}

fn repeated_intrinsic_payload_height_profiles(
    fit_position: usize,
    track_count: usize,
) -> [[f32; 8]; 2] {
    let mut legacy = [20.0, 10.0, 12.0, 6.0, 6.0, 6.0, 6.0, 6.0];
    let mut compact = [1.0; 8];
    for heights in [&mut legacy, &mut compact] {
        heights[fit_position] = 20.0;
        if let Some(repeated_fit_position) = fit_position.checked_add(track_count)
            && repeated_fit_position < heights.len()
        {
            heights[repeated_fit_position] = 60.0;
        }
    }
    [legacy, compact]
}
