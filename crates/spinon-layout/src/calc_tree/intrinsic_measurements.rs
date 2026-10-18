use taffy::{AvailableSpace, NodeId as TaffyNodeId, Size};

use crate::{
    LayoutError, LayoutInputRevision, LayoutIntrinsicMeasureAvailableSpace,
    LayoutIntrinsicMeasureKnownDimensions, LayoutIntrinsicMeasureParentSize,
    LayoutIntrinsicMeasureSize, LayoutIntrinsicMeasurement, LayoutIntrinsicMeasurementSource,
};

use super::CalcLayoutTree;

#[cfg(test)]
thread_local! {
    static FAIL_NEXT_INTRINSIC_MEASUREMENT_RESERVATION: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(test)]
struct IntrinsicMeasurementReservationFailureGuard;

#[cfg(test)]
impl Drop for IntrinsicMeasurementReservationFailureGuard {
    fn drop(&mut self) {
        FAIL_NEXT_INTRINSIC_MEASUREMENT_RESERVATION.with(|armed| armed.set(false));
        crate::test_allocator::reset();
    }
}

#[cfg(test)]
pub(crate) fn fail_next_intrinsic_measurement_reservation() -> impl Drop {
    crate::test_allocator::reset();
    FAIL_NEXT_INTRINSIC_MEASUREMENT_RESERVATION.with(|armed| {
        assert!(
            !armed.replace(true),
            "intrinsic measurement failpoint은 중첩할 수 없습니다"
        );
    });
    IntrinsicMeasurementReservationFailureGuard
}

impl CalcLayoutTree {
    pub(super) fn record_intrinsic_measurement(
        &self,
        taffy_node: TaffyNodeId,
        source: LayoutIntrinsicMeasurementSource,
        available_space: Size<AvailableSpace>,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        content_size: Size<f32>,
    ) {
        if !self.record_intrinsic_measurements {
            return;
        }
        let Some(node) = self.node(taffy_node).spinon_node else {
            return;
        };
        if !self.grid_item_nodes.contains(&node) {
            return;
        }
        let item_style = self.grid_item_styles[&node];

        let mut measurements = self.intrinsic_measurements.borrow_mut();
        #[cfg(test)]
        FAIL_NEXT_INTRINSIC_MEASUREMENT_RESERVATION.with(|armed| {
            if armed.replace(false) {
                crate::test_allocator::fail_next_allocation();
            }
        });
        if measurements.try_reserve(1).is_err() {
            self.record_error(LayoutError::IntrinsicMeasurementAllocationFailure { node });
            return;
        }
        measurements.push(LayoutIntrinsicMeasurement {
            node,
            revision: LayoutInputRevision::default(),
            source,
            item_style,
            available_space: LayoutIntrinsicMeasureAvailableSpace {
                width: measure_space(available_space.width),
                height: measure_space(available_space.height),
            },
            known_dimensions: LayoutIntrinsicMeasureKnownDimensions {
                width: known_dimensions.width,
                height: known_dimensions.height,
            },
            parent_size: LayoutIntrinsicMeasureParentSize {
                width: parent_size.width,
                height: parent_size.height,
            },
            content_size: LayoutIntrinsicMeasureSize {
                width: content_size.width,
                height: content_size.height,
            },
        });
    }

    pub(crate) fn take_intrinsic_measurements(
        &self,
        revision: LayoutInputRevision,
    ) -> Vec<LayoutIntrinsicMeasurement> {
        let mut measurements = std::mem::take(&mut *self.intrinsic_measurements.borrow_mut());
        for measurement in &mut measurements {
            measurement.revision = revision;
        }
        measurements
    }
}

fn measure_space(value: AvailableSpace) -> crate::LayoutIntrinsicMeasureSpace {
    match value {
        AvailableSpace::MinContent => crate::LayoutIntrinsicMeasureSpace::MinContent,
        AvailableSpace::MaxContent => crate::LayoutIntrinsicMeasureSpace::MaxContent,
        AvailableSpace::Definite(value) => crate::LayoutIntrinsicMeasureSpace::Definite(value),
    }
}
