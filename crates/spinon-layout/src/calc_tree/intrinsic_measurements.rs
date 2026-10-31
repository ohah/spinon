use std::collections::{BTreeMap, BTreeSet};

use spinon_core::NodeId as SpinonNodeId;
use taffy::{AvailableSpace, NodeId as TaffyNodeId, Size};

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
use crate::{GridPlacementMode, LayoutAlignItems, LayoutAlignSelf};
use crate::{
    LayoutDisplay, LayoutError, LayoutGridItemIntrinsicInput, LayoutGridItemIntrinsicSource,
    LayoutInput, LayoutInputRevision, LayoutIntrinsicMeasureAvailableSpace,
    LayoutIntrinsicMeasureKnownDimensions, LayoutIntrinsicMeasureParentSize,
    LayoutIntrinsicMeasureSize, LayoutIntrinsicMeasurement, LayoutIntrinsicMeasurementSource,
    LayoutPosition, LayoutStyle, TextParagraphInput,
};

use super::CalcLayoutTree;

#[derive(Clone)]
pub(super) struct GridItemIntrinsicMetadata {
    pub(super) container: SpinonNodeId,
    pub(super) source_order: usize,
    pub(super) is_leaf: bool,
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    pub(super) is_track_sizing: bool,
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    pub(super) first_baseline_aligned: bool,
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    pub(super) last_baseline_aligned: bool,
    pub(super) style: LayoutStyle,
    pub(super) text_sources: Vec<crate::LayoutGridItemTextSource>,
}

pub(super) fn grid_item_metadata(
    input: &LayoutInput,
    index: &BTreeMap<SpinonNodeId, usize>,
    flow_only: bool,
) -> BTreeMap<SpinonNodeId, GridItemIntrinsicMetadata> {
    if flow_only {
        return BTreeMap::new();
    }

    let hidden = hidden_layout_nodes(input, index);
    input
        .nodes
        .iter()
        .filter(|node| node.style.display == LayoutDisplay::Grid)
        .filter(|container| !hidden.contains(&container.id))
        .flat_map(|container| {
            container
                .children
                .iter()
                .copied()
                .enumerate()
                .filter(|(_, child)| {
                    !hidden.contains(child)
                        && !input.positioning.get(child).is_some_and(|positioning| {
                            matches!(
                                positioning.position,
                                LayoutPosition::Absolute | LayoutPosition::Fixed
                            )
                        })
                })
                .map(|(source_order, child)| {
                    let child_node = &input.nodes[index[&child]];
                    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
                    let container_node = &input.nodes[index[&container.id]];
                    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
                    let is_track_sizing =
                        input
                            .grid_containers
                            .get(&container.id)
                            .is_some_and(|grid| {
                                grid.placement_mode == GridPlacementMode::TrackSizing
                                    || grid.track_sizing_trace_enabled
                            });
                    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
                    let (first_baseline_aligned, last_baseline_aligned) =
                        match child_node.style.align_self {
                            LayoutAlignSelf::FirstBaseline => (true, false),
                            LayoutAlignSelf::LastBaseline => (false, true),
                            LayoutAlignSelf::Auto => (
                                container_node.style.align_items == LayoutAlignItems::FirstBaseline,
                                container_node.style.align_items == LayoutAlignItems::LastBaseline,
                            ),
                            _ => (false, false),
                        };
                    let text_sources = match input.text_paragraphs.get(&child) {
                        Some(TextParagraphInput::AnonymousGridText {
                            grid_container,
                            source_nodes,
                            ..
                        }) if *grid_container == container.id => source_nodes.clone(),
                        _ => Vec::new(),
                    };
                    (
                        child,
                        GridItemIntrinsicMetadata {
                            container: container.id,
                            source_order,
                            is_leaf: child_node.children.is_empty(),
                            #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
                            is_track_sizing,
                            #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
                            first_baseline_aligned,
                            #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
                            last_baseline_aligned,
                            style: child_node.style,
                            text_sources,
                        },
                    )
                })
        })
        .collect()
}

fn hidden_layout_nodes(
    input: &LayoutInput,
    index: &BTreeMap<SpinonNodeId, usize>,
) -> BTreeSet<SpinonNodeId> {
    let mut hidden_nodes = BTreeSet::new();
    let mut pending = vec![(input.root(), false)];
    while let Some((node_id, ancestor_hidden)) = pending.pop() {
        let node = &input.nodes[index[&node_id]];
        let hidden = ancestor_hidden || node.style.display == LayoutDisplay::None;
        if hidden {
            hidden_nodes.insert(node_id);
        }
        pending.extend(node.children.iter().map(|child| (*child, hidden)));
    }
    hidden_nodes
}

#[cfg(test)]
thread_local! {
    static FAIL_NEXT_INTRINSIC_MEASUREMENT_RESERVATION: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static FAIL_NEXT_GRID_ITEM_INTRINSIC_INPUT_RESERVATION: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
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

#[cfg(test)]
struct GridItemIntrinsicInputReservationFailureGuard;

#[cfg(test)]
impl Drop for GridItemIntrinsicInputReservationFailureGuard {
    fn drop(&mut self) {
        FAIL_NEXT_GRID_ITEM_INTRINSIC_INPUT_RESERVATION.with(|armed| armed.set(false));
        crate::test_allocator::reset();
    }
}

#[cfg(test)]
pub(crate) fn fail_next_grid_item_intrinsic_input_reservation() -> impl Drop {
    crate::test_allocator::reset();
    FAIL_NEXT_GRID_ITEM_INTRINSIC_INPUT_RESERVATION.with(|armed| {
        assert!(
            !armed.replace(true),
            "Grid item intrinsic input failpoint은 중첩할 수 없습니다"
        );
    });
    GridItemIntrinsicInputReservationFailureGuard
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
        let Some(item_metadata) = self.grid_item_intrinsic_metadata.get(&node) else {
            return;
        };
        let item_style = item_metadata.style;

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

    pub(crate) fn collect_grid_item_intrinsic_inputs(
        &self,
        revision: LayoutInputRevision,
    ) -> Result<Vec<LayoutGridItemIntrinsicInput>, LayoutError> {
        let mut inputs = Vec::new();
        for (&node, item_metadata) in &self.grid_item_intrinsic_metadata {
            if !item_metadata.is_leaf {
                continue;
            }
            let engine_id = self.engine_ids[&node];
            let calc_node = self.node(engine_id);
            let source = if let Some(paragraph) = self.text_paragraphs.get(&engine_id) {
                if paragraph.style.writing_mode == spinon_core::WritingMode::VerticalRl {
                    let sizes = self.text_intrinsic_logical_sizes(engine_id, node)?;
                    LayoutGridItemIntrinsicSource::OrthogonalText {
                        min_content: sizes.min_content(),
                        max_content: sizes.max_content(),
                        child_writing_mode: sizes.writing_mode(),
                        parent_writing_mode: spinon_core::WritingMode::HorizontalTb,
                        text_orientation: sizes.text_orientation(),
                        font_registry_revision: sizes.font_registry_revision(),
                        font_registry_identity: sizes.clone(),
                    }
                } else {
                    LayoutGridItemIntrinsicSource::Text {
                        sizes: self.text_intrinsic_inline_sizes(engine_id, node)?,
                    }
                }
            } else if let Some(replaced) = calc_node.replaced_sizing {
                LayoutGridItemIntrinsicSource::Replaced {
                    natural: replaced.natural,
                }
            } else {
                continue;
            };

            #[cfg(test)]
            FAIL_NEXT_GRID_ITEM_INTRINSIC_INPUT_RESERVATION.with(|armed| {
                if armed.replace(false) {
                    crate::test_allocator::fail_next_allocation();
                }
            });
            if inputs.try_reserve(1).is_err() {
                return Err(LayoutError::IntrinsicMeasurementAllocationFailure { node });
            }
            inputs.push(LayoutGridItemIntrinsicInput {
                grid_container: item_metadata.container,
                node,
                source_order: item_metadata.source_order,
                revision,
                source,
                text_sources: item_metadata.text_sources.clone(),
                item_style: item_metadata.style,
            });
        }
        inputs.sort_unstable_by_key(|input| (input.grid_container, input.source_order));
        Ok(inputs)
    }
}

fn measure_space(value: AvailableSpace) -> crate::LayoutIntrinsicMeasureSpace {
    match value {
        AvailableSpace::MinContent => crate::LayoutIntrinsicMeasureSpace::MinContent,
        AvailableSpace::MaxContent => crate::LayoutIntrinsicMeasureSpace::MaxContent,
        AvailableSpace::Definite(value) => crate::LayoutIntrinsicMeasureSpace::Definite(value),
    }
}
