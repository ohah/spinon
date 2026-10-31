use spinon_core::NodeId;

use crate::{GridContainerInput, GridItemInput, GridPlacementInput, LayoutError};

pub(super) fn validate_shape(
    node: NodeId,
    container: &GridContainerInput,
    layout_participating_child_count: usize,
    item: Option<&GridItemInput>,
) -> Result<(), LayoutError> {
    let Some(item) = item.filter(|_| layout_participating_child_count == 1) else {
        return Err(invalid_pair_shape(node, "grid-column-start"));
    };
    let columns_pair = is_positive_implicit_line_pair(
        &item.column_start,
        &item.column_end,
        container.columns.len(),
    );
    let rows_pair =
        is_positive_implicit_line_pair(&item.row_start, &item.row_end, container.rows.len());
    if columns_pair == rows_pair {
        return Err(invalid_pair_shape(node, "grid-column-start"));
    }
    let (opposite_start, opposite_end, opposite_track_count, property) = if columns_pair {
        (
            &item.row_start,
            &item.row_end,
            container.rows.len(),
            "grid-row-start",
        )
    } else {
        (
            &item.column_start,
            &item.column_end,
            container.columns.len(),
            "grid-column-start",
        )
    };
    if !is_explicit_definite_axis(opposite_start, opposite_end, opposite_track_count) {
        return Err(invalid_pair_shape(node, property));
    }
    Ok(())
}

fn is_positive_implicit_line_pair(
    start: &GridPlacementInput,
    end: &GridPlacementInput,
    track_count: usize,
) -> bool {
    let Some(last_explicit_line) = i64::try_from(track_count)
        .ok()
        .and_then(|count| count.checked_add(1))
    else {
        return false;
    };
    matches!(
        (start, end),
        (GridPlacementInput::Line(start), GridPlacementInput::Line(end))
            if i64::from(*start) > last_explicit_line
                && i64::from(*end) > last_explicit_line
    )
}

fn is_explicit_definite_axis(
    start: &GridPlacementInput,
    end: &GridPlacementInput,
    track_count: usize,
) -> bool {
    let GridPlacementInput::Line(start) = start else {
        return false;
    };
    is_explicit_line(*start, track_count)
        && match end {
            GridPlacementInput::Auto => true,
            GridPlacementInput::Line(end) => is_explicit_line(*end, track_count),
            GridPlacementInput::Span(_)
            | GridPlacementInput::NamedLine { .. }
            | GridPlacementInput::SpanNamed { .. } => false,
        }
}

fn is_explicit_line(line: i32, track_count: usize) -> bool {
    if line == 0 {
        return false;
    }
    let Some(last_explicit_line) = i64::try_from(track_count)
        .ok()
        .and_then(|count| count.checked_add(1))
    else {
        return false;
    };
    let line = i64::from(line);
    let normalized = if line < 0 {
        line.checked_add(last_explicit_line)
            .and_then(|line| line.checked_add(1))
    } else {
        Some(line)
    };
    normalized.is_some_and(|line| line >= 1 && line <= last_explicit_line)
}

fn invalid_pair_shape(node: NodeId, property: &'static str) -> LayoutError {
    LayoutError::InvalidGridPlacement {
        node,
        property,
        reason: "C11.3.3 positive implicit named-line pair mode requires one item, one trailing pair, and a definite opposite axis",
    }
}
