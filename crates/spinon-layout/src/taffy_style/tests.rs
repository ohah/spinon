use taffy::style::AlignItems;

use crate::{LayoutAlignItems, LayoutAlignSelf};

use super::{to_taffy_align_items, to_taffy_grid_align_items, to_taffy_grid_align_self};

#[test]
fn align_items_preserves_normal_and_stretch_keywords() {
    assert_eq!(
        to_taffy_align_items(LayoutAlignItems::Normal),
        AlignItems::NORMAL
    );
    assert_eq!(
        to_taffy_align_items(LayoutAlignItems::Stretch),
        AlignItems::STRETCH
    );
}

#[test]
fn last_baseline_marker_is_limited_to_grid_alignment_mapping() {
    assert_eq!(
        to_taffy_align_items(LayoutAlignItems::LastBaseline),
        AlignItems::BASELINE
    );
    assert_eq!(
        to_taffy_grid_align_items(LayoutAlignItems::LastBaseline),
        AlignItems::LAST_BASELINE
    );
    assert_eq!(
        to_taffy_grid_align_self(LayoutAlignSelf::LastBaseline),
        Some(AlignItems::LAST_BASELINE)
    );
}
