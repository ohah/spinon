use spinon_core::NodeId;

use crate::LayoutError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum GridReservationSite {
    VisibleItemList,
    RowLockedCursor,
    ExpandedRows,
    ExpandedColumns,
    OccupancyRows,
    OccupancyIntervals,
}

#[cfg(test)]
thread_local! {
    static FAILURE_SITE: std::cell::Cell<Option<GridReservationSite>> = const { std::cell::Cell::new(None) };
}

#[cfg(test)]
pub(crate) struct GridReservationFailureGuard;

#[cfg(test)]
impl Drop for GridReservationFailureGuard {
    fn drop(&mut self) {
        FAILURE_SITE.with(|site| site.set(None));
        crate::test_allocator::reset();
    }
}

#[cfg(test)]
pub(crate) fn inject_failure_once(site: GridReservationSite) -> GridReservationFailureGuard {
    crate::test_allocator::reset();
    let was_armed = FAILURE_SITE.with(|current| current.replace(Some(site)).is_some());
    assert!(
        !was_armed,
        "Grid reservation failpoint은 중첩해서 arm할 수 없습니다"
    );
    GridReservationFailureGuard
}

fn arm_allocator_failure(site: GridReservationSite) {
    #[cfg(test)]
    {
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
        if should_fail {
            crate::test_allocator::fail_next_allocation();
        }
    }
    #[cfg(not(test))]
    let _ = site;
}

pub(super) fn reserve<T>(
    values: &mut Vec<T>,
    additional: usize,
    node: NodeId,
    property: &'static str,
    site: GridReservationSite,
) -> Result<(), LayoutError> {
    arm_allocator_failure(site);
    values
        .try_reserve(additional)
        .map_err(|_| allocation_failure(node, property))
}

pub(super) fn reserve_exact<T>(
    values: &mut Vec<T>,
    additional: usize,
    node: NodeId,
    property: &'static str,
    site: GridReservationSite,
) -> Result<(), LayoutError> {
    arm_allocator_failure(site);
    values
        .try_reserve_exact(additional)
        .map_err(|_| allocation_failure(node, property))
}

fn allocation_failure(node: NodeId, property: &'static str) -> LayoutError {
    LayoutError::GridAllocationFailure { node, property }
}
