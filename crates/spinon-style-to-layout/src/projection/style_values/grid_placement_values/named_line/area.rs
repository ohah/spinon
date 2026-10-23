use spinon_core::NodeId;

use crate::StyleLayoutError;

use super::{GridAxis, LineMetadata, unsupported_named_line, validate_line_metadata};

#[derive(Clone, Copy)]
enum AreaEdge {
    Start,
    End,
}

pub(super) fn resolve_bare_custom_ident(
    node: NodeId,
    property: &'static str,
    name: &str,
    is_start: bool,
    line_metadata: &LineMetadata<'_>,
    allow_implicit_named_lines: bool,
) -> Result<(i32, bool), StyleLayoutError> {
    let start_exists = has_area_edge_on_axis(line_metadata.names, name, AreaEdge::Start);
    let end_exists = has_area_edge_on_axis(line_metadata.names, name, AreaEdge::End);
    let opposite_start_exists =
        has_area_edge_on_axis(line_metadata.opposite_names, name, AreaEdge::Start);
    let opposite_end_exists =
        has_area_edge_on_axis(line_metadata.opposite_names, name, AreaEdge::End);
    let explicit_area = line_metadata
        .areas
        .is_some_and(|areas| areas.areas.iter().any(|area| area.name == name));
    let implicit_area = start_exists && end_exists && opposite_start_exists && opposite_end_exists;
    if !explicit_area && !implicit_area {
        if !allow_implicit_named_lines {
            return super::find_named_occurrence(node, property, name, 1, line_metadata)
                .map(|line| (line, false));
        }
        if let Some(line) =
            super::find_first_named_occurrence_if_present(node, property, name, line_metadata)?
        {
            return Ok((line, false));
        }
        let line = line_metadata
            .track_count
            .checked_add(2)
            .and_then(|line| i32::try_from(line).ok())
            .ok_or_else(|| {
                unsupported_named_line(
                    node,
                    property,
                    name,
                    "generated implicit named line exceeds the supported range",
                )
            })?;
        return Ok((line, true));
    }

    validate_area_line_metadata(node, property, name, line_metadata)?;
    let edge = if is_start {
        AreaEdge::Start
    } else {
        AreaEdge::End
    };
    for (index, group) in line_metadata.names.iter().enumerate() {
        let authored_alias = group
            .iter()
            .any(|candidate| is_area_edge_alias(candidate, name, edge));
        let generated_alias = line_metadata.areas.is_some_and(|areas| {
            areas.areas.iter().any(|area| {
                area.name == name && area_edge_line(area, line_metadata.axis, edge) == index
            })
        });
        if authored_alias || generated_alias {
            let line = i32::try_from(index + 1).map_err(|_| {
                unsupported_named_line(
                    node,
                    property,
                    name,
                    "resolved area line exceeds the supported range",
                )
            })?;
            return Ok((line, false));
        }
    }
    Err(unsupported_named_line(
        node,
        property,
        name,
        "named area edge requires an implicit grid line outside the explicit grid",
    ))
}

fn validate_area_line_metadata(
    node: NodeId,
    property: &'static str,
    name: &str,
    line_metadata: &LineMetadata<'_>,
) -> Result<(), StyleLayoutError> {
    validate_line_metadata(node, property, name, line_metadata)?;
    let Some(areas) = line_metadata.areas else {
        let opposite_track_count = line_metadata.opposite_names.len().saturating_sub(1);
        if opposite_track_count.checked_add(1) != Some(line_metadata.opposite_names.len()) {
            return Err(unsupported_named_line(
                node,
                property,
                name,
                "opposite grid line-name metadata is missing or has the wrong length",
            ));
        }
        return Ok(());
    };

    let axis_track_count = match line_metadata.axis {
        GridAxis::Rows => areas.rows,
        GridAxis::Columns => areas.columns,
    };
    if axis_track_count != line_metadata.track_count {
        return Err(unsupported_named_line(
            node,
            property,
            name,
            "template-area dimensions do not match the explicit grid",
        ));
    }
    Ok(())
}

fn has_area_edge_on_axis(names: &[Vec<String>], name: &str, edge: AreaEdge) -> bool {
    names.iter().any(|group| {
        group
            .iter()
            .any(|candidate| is_area_edge_alias(candidate, name, edge))
    })
}

fn is_area_edge_alias(candidate: &str, name: &str, edge: AreaEdge) -> bool {
    let suffix = match edge {
        AreaEdge::Start => "-start",
        AreaEdge::End => "-end",
    };
    candidate.strip_suffix(suffix) == Some(name)
}

fn area_edge_line(
    area: &spinon_style::ComputedCssGridArea,
    axis: GridAxis,
    edge: AreaEdge,
) -> usize {
    match (axis, edge) {
        (GridAxis::Rows, AreaEdge::Start) => area.row_start,
        (GridAxis::Rows, AreaEdge::End) => area.row_end,
        (GridAxis::Columns, AreaEdge::Start) => area.column_start,
        (GridAxis::Columns, AreaEdge::End) => area.column_end,
    }
}

pub(super) fn line_has_name(line_metadata: &LineMetadata<'_>, index: usize, name: &str) -> bool {
    if line_metadata
        .names
        .get(index)
        .is_some_and(|group| group.iter().any(|candidate| candidate == name))
    {
        return true;
    }
    let Some(areas) = line_metadata.areas else {
        return false;
    };
    areas.areas.iter().any(|area| {
        [AreaEdge::Start, AreaEdge::End].into_iter().any(|edge| {
            is_area_edge_alias(name, &area.name, edge)
                && area_edge_line(area, line_metadata.axis, edge) == index
        })
    })
}
