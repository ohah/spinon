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
    if !is_negative_implicit_line_pair(
        &item.column_start,
        &item.column_end,
        container.columns.len(),
    ) {
        return Err(invalid_pair_shape(node, "grid-column-start"));
    }
    if !is_negative_implicit_line_pair(&item.row_start, &item.row_end, container.rows.len()) {
        return Err(invalid_pair_shape(node, "grid-row-start"));
    }
    Ok(())
}

fn is_negative_implicit_line_pair(
    start: &GridPlacementInput,
    end: &GridPlacementInput,
    track_count: usize,
) -> bool {
    let Some(first_implicit_line) = i64::try_from(track_count)
        .ok()
        .and_then(|count| count.checked_add(1))
        .and_then(i64::checked_neg)
    else {
        return false;
    };
    matches!(
        (start, end),
        (GridPlacementInput::Line(start), GridPlacementInput::Line(end))
            if i64::from(*start) < first_implicit_line
                && i64::from(*end) < first_implicit_line
    )
}

fn invalid_pair_shape(node: NodeId, property: &'static str) -> LayoutError {
    LayoutError::InvalidGridPlacement {
        node,
        property,
        reason: "C11.3.3 양 축 음수 named-line pair mode는 단일 item과 두 축의 leading pair만 허용합니다",
    }
}
