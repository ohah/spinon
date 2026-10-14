use spinon_core::NodeId;
use style::values::computed::GridTemplateAreas;

use super::super::{ComputedCssGridArea, ComputedCssGridTemplateAreas, CssCascadeError};

const PROPERTY: &str = "grid-template-areas";

#[cfg(test)]
#[path = "grid_areas/tests.rs"]
mod tests;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::stylo_dom::cascade) enum GridTemplateAreasReservationSite {
    Areas,
    AreaNameText,
}

#[cfg(test)]
thread_local! {
    static FAILURE_SITE: std::cell::Cell<Option<GridTemplateAreasReservationSite>> = const { std::cell::Cell::new(None) };
}

#[cfg(test)]
struct FailureGuard;

#[cfg(test)]
impl Drop for FailureGuard {
    fn drop(&mut self) {
        FAILURE_SITE.with(|site| site.set(None));
    }
}

#[cfg(test)]
pub(in crate::stylo_dom::cascade) fn inject_grid_template_areas_reservation_failure(
    site: GridTemplateAreasReservationSite,
) -> impl Drop {
    let was_armed = FAILURE_SITE.with(|current| current.replace(Some(site)).is_some());
    assert!(
        !was_armed,
        "grid-template-areas 예약 failpoint은 중첩할 수 없습니다"
    );
    FailureGuard
}

pub(super) fn computed_grid_template_areas(
    value: GridTemplateAreas,
    node: NodeId,
    serialized: &str,
) -> Result<Option<ComputedCssGridTemplateAreas>, CssCascadeError> {
    let GridTemplateAreas::Areas(value) = value else {
        return Ok(None);
    };
    let template = &value.0;
    let rows = template.strings.len();
    let columns =
        usize::try_from(template.width).map_err(|_| unsupported_grid_areas(node, serialized))?;
    let mut areas = Vec::new();
    reserve_areas(&mut areas, template.areas.len(), node)?;

    for source in template.areas.iter() {
        let name = source.name.as_str();
        let mut owned_name = String::new();
        reserve_area_name(&mut owned_name, name.len(), node)?;
        owned_name.push_str(name);

        let row_start = cell_range_start(source.rows.start, rows)
            .ok_or_else(|| unsupported_grid_areas(node, serialized))?;
        let row_end = cell_range_end(source.rows.end, rows)
            .ok_or_else(|| unsupported_grid_areas(node, serialized))?;
        let column_start = cell_range_start(source.columns.start, columns)
            .ok_or_else(|| unsupported_grid_areas(node, serialized))?;
        let column_end = cell_range_end(source.columns.end, columns)
            .ok_or_else(|| unsupported_grid_areas(node, serialized))?;
        if row_start >= row_end || column_start >= column_end {
            return Err(unsupported_grid_areas(node, serialized));
        }
        areas.push(ComputedCssGridArea {
            name: owned_name,
            row_start,
            row_end,
            column_start,
            column_end,
        });
    }

    Ok(Some(ComputedCssGridTemplateAreas {
        rows,
        columns,
        areas,
    }))
}

fn cell_range_start(value: u32, dimension: usize) -> Option<usize> {
    let zero_based = usize::try_from(value.checked_sub(1)?).ok()?;
    (zero_based < dimension).then_some(zero_based)
}

fn cell_range_end(value: u32, dimension: usize) -> Option<usize> {
    let zero_based_exclusive = usize::try_from(value.checked_sub(1)?).ok()?;
    (zero_based_exclusive <= dimension).then_some(zero_based_exclusive)
}

fn reserve_areas(
    values: &mut Vec<ComputedCssGridArea>,
    count: usize,
    node: NodeId,
) -> Result<(), CssCascadeError> {
    fail_if_requested(GridTemplateAreasReservationSite::Areas)
        .map_err(|_| allocation_failure(node))?;
    values
        .try_reserve_exact(count)
        .map_err(|_| allocation_failure(node))
}

fn reserve_area_name(
    value: &mut String,
    count: usize,
    node: NodeId,
) -> Result<(), CssCascadeError> {
    fail_if_requested(GridTemplateAreasReservationSite::AreaNameText)
        .map_err(|_| allocation_failure(node))?;
    value
        .try_reserve_exact(count)
        .map_err(|_| allocation_failure(node))
}

#[cfg(test)]
fn fail_if_requested(site: GridTemplateAreasReservationSite) -> Result<(), ()> {
    let should_fail = FAILURE_SITE
        .try_with(|current| {
            if current.get() == Some(site) {
                current.set(None);
                true
            } else {
                false
            }
        })
        .unwrap_or(false);
    if should_fail { Err(()) } else { Ok(()) }
}

#[cfg(not(test))]
fn fail_if_requested(_site: GridTemplateAreasReservationSite) -> Result<(), ()> {
    Ok(())
}

fn allocation_failure(node: NodeId) -> CssCascadeError {
    CssCascadeError::GridAllocationFailure {
        node,
        property: PROPERTY,
    }
}

fn unsupported_grid_areas(node: NodeId, serialized: &str) -> CssCascadeError {
    CssCascadeError::UnsupportedComputedGridValue {
        node,
        property: PROPERTY,
        value: serialized.to_owned(),
    }
}
