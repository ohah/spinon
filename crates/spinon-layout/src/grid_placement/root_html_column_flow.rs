use std::collections::BTreeMap;

use spinon_core::NodeId;

use crate::{
    GridAutoColumns, GridAutoFlow, GridAutoRows, GridContainerInput, GridItemInput,
    GridPlacementInput, GridPlacementMode, GridTrackSize, LayoutError, LayoutInput,
    grid_auto_placement::DenseGridPlacement,
};

pub(super) fn validate_items(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
    hidden: &BTreeMap<NodeId, bool>,
    container_node: &crate::LayoutNode,
    container: &GridContainerInput,
) -> Result<(), LayoutError> {
    let node = container_node.id;
    if container.placement_mode != GridPlacementMode::RootHtmlColumnFlowV1 {
        return Ok(());
    }
    validate_container(node, container)?;
    let visible_count = visible_item_count(hidden, container_node);
    let required_columns = required_column_count(container, visible_count);
    if required_columns > crate::GRID_MAX_TRACKS {
        return Err(LayoutError::GridTrackLimitExceeded {
            node,
            property: "grid-template-columns",
            count: required_columns,
            maximum: crate::GRID_MAX_TRACKS,
        });
    }
    if required_columns > container.columns.len()
        && matches!(container.auto_columns, GridAutoColumns::Auto)
    {
        return Err(LayoutError::GridImplicitTrackRequired(node));
    }

    for child in container_node
        .children
        .iter()
        .copied()
        .filter(|child| !hidden[child])
    {
        let child_node = &input.nodes[index[&child]];
        if child_node.style.order != 0 {
            return Err(LayoutError::UnsupportedGridOrder(child));
        }
        if input
            .grid_items
            .get(&child)
            .is_some_and(|placement| !placement.is_auto())
        {
            return Err(super::invalid_placement(
                child,
                "grid-row-start",
                "Root HTML column-flow mode requires all four placement longhands to be auto",
            ));
        }
        if input.grid_containers.contains_key(&child)
            || !crate::grid::root_html_fixed_empty_item_is_supported(input, child_node)
        {
            return Err(LayoutError::UnsupportedGridItemSize(child));
        }
    }
    Ok(())
}

pub(super) fn normalize(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
    hidden: &BTreeMap<NodeId, bool>,
    container_node: &crate::LayoutNode,
    container: &GridContainerInput,
) -> Result<DenseGridPlacement, LayoutError> {
    validate_items(input, index, hidden, container_node, container)?;
    let placement = crate::grid_auto_placement::normalize_dense_grid_container(
        input,
        hidden,
        container_node,
        container,
    )?;
    let node = container_node.id;
    let visible_count = visible_item_count(hidden, container_node);
    let required_columns = required_column_count(container, visible_count);
    let positions_fit_tracks = |item: &GridItemInput| {
        fn axis_fits(start: &GridPlacementInput, end: &GridPlacementInput, tracks: usize) -> bool {
            matches!(
                (start, end),
                (GridPlacementInput::Line(start), GridPlacementInput::Line(end))
                    if *start >= 1
                        && *end > *start
                        && usize::try_from(*end - 1).is_ok_and(|last_line| last_line <= tracks)
            )
        }
        axis_fits(&item.row_start, &item.row_end, container.rows.len())
            && axis_fits(&item.column_start, &item.column_end, required_columns)
    };
    if placement.expanded_rows.is_some()
        || !expanded_columns_match(
            container,
            placement.expanded_columns.as_deref(),
            required_columns,
        )
        || placement.item_placements.len() != visible_count
        || placement.auto_placed_items.len() != visible_count
        || placement
            .item_placements
            .values()
            .any(|item| !positions_fit_tracks(item))
    {
        return Err(LayoutError::GridImplicitTrackRequired(node));
    }
    Ok(placement)
}

fn validate_container(node: NodeId, container: &GridContainerInput) -> Result<(), LayoutError> {
    if container.auto_flow != GridAutoFlow::Column
        || container.columns.is_empty()
        || container.rows.is_empty()
        || container.template_areas.is_some()
        || container.columns_auto_repeat.is_some()
        || container.rows_auto_repeat.is_some()
        || container
            .columns_line_names
            .iter()
            .any(|group| !group.is_empty())
        || container
            .rows_line_names
            .iter()
            .any(|group| !group.is_empty())
    {
        return Err(LayoutError::GridImplicitTrackRequired(node));
    }
    if container.columns.len() > crate::GRID_MAX_TRACKS {
        return Err(LayoutError::GridTrackLimitExceeded {
            node,
            property: "grid-template-columns",
            count: container.columns.len(),
            maximum: crate::GRID_MAX_TRACKS,
        });
    }
    if container.rows.len() > crate::GRID_MAX_TRACKS {
        return Err(LayoutError::GridTrackLimitExceeded {
            node,
            property: "grid-template-rows",
            count: container.rows.len(),
            maximum: crate::GRID_MAX_TRACKS,
        });
    }
    if container
        .columns
        .iter()
        .any(|track| !is_nonnegative_px(track))
    {
        return Err(LayoutError::InvalidGridTrackList {
            node,
            property: "grid-template-columns",
        });
    }
    if container.rows.iter().any(|track| !is_nonnegative_px(track)) {
        return Err(LayoutError::InvalidGridTrackList {
            node,
            property: "grid-template-rows",
        });
    }
    if !matches!(container.auto_rows, GridAutoRows::Auto) {
        return Err(LayoutError::UnsupportedGridAutoRows {
            node,
            reason: "Root HTML column-flow mode only supports grid-auto-rows:auto",
        });
    }
    validate_auto_columns(node, &container.auto_columns)
}

fn validate_auto_columns(node: NodeId, auto_columns: &GridAutoColumns) -> Result<(), LayoutError> {
    match auto_columns {
        GridAutoColumns::Auto => Ok(()),
        GridAutoColumns::LengthPx(value) if value.is_finite() && *value >= 0.0 => Ok(()),
        GridAutoColumns::LengthPx(_) => Err(LayoutError::InvalidGridTrack {
            node,
            property: "grid-auto-columns",
        }),
        GridAutoColumns::TrackList(tracks) if tracks.is_empty() => {
            Err(LayoutError::InvalidGridTrackList {
                node,
                property: "grid-auto-columns",
            })
        }
        GridAutoColumns::TrackList(tracks) if tracks.len() > crate::GRID_MAX_TRACKS => {
            Err(LayoutError::GridTrackLimitExceeded {
                node,
                property: "grid-auto-columns",
                count: tracks.len(),
                maximum: crate::GRID_MAX_TRACKS,
            })
        }
        GridAutoColumns::TrackList(tracks) => {
            for track in tracks {
                let value = match track {
                    GridTrackSize::LengthPx(value)
                    | GridTrackSize::Percentage(value)
                    | GridTrackSize::Fraction(value) => value,
                    _ => {
                        return Err(LayoutError::UnsupportedGridAutoColumns {
                            node,
                            reason: "Root HTML column-flow mode requires direct px, percentage, or fraction auto-column tracks",
                        });
                    }
                };
                if !value.is_finite() || *value < 0.0 {
                    return Err(LayoutError::InvalidGridTrack {
                        node,
                        property: "grid-auto-columns",
                    });
                }
            }
            Ok(())
        }
    }
}

fn is_nonnegative_px(track: &GridTrackSize) -> bool {
    matches!(track, GridTrackSize::LengthPx(value) if value.is_finite() && *value >= 0.0)
}

fn visible_item_count(
    hidden: &BTreeMap<NodeId, bool>,
    container_node: &crate::LayoutNode,
) -> usize {
    container_node
        .children
        .iter()
        .filter(|child| !hidden[child])
        .count()
}

fn required_column_count(container: &GridContainerInput, visible_items: usize) -> usize {
    visible_items
        .div_ceil(container.rows.len())
        .max(container.columns.len())
}

fn expanded_columns_match(
    container: &GridContainerInput,
    expanded_columns: Option<&[GridTrackSize]>,
    required_columns: usize,
) -> bool {
    if required_columns == container.columns.len() {
        return expanded_columns.is_none();
    }
    let Some(expanded_columns) = expanded_columns else {
        return false;
    };
    if expanded_columns.len() != required_columns
        || expanded_columns.get(..container.columns.len()) != Some(container.columns.as_slice())
    {
        return false;
    }
    let implicit_count = required_columns - container.columns.len();
    (0..implicit_count).all(|index| {
        let expected = match &container.auto_columns {
            GridAutoColumns::LengthPx(value) => GridTrackSize::LengthPx(*value),
            GridAutoColumns::TrackList(tracks) => tracks[index % tracks.len()],
            GridAutoColumns::Auto => return false,
        };
        expanded_columns[container.columns.len() + index] == expected
    })
}
