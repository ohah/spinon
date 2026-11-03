use taffy::style::{
    AlignItems, AlignItemsKeyword, AlignSelf, AlignmentSafety as TaffyAlignmentSafety,
};

use crate::{ItemAlignmentPosition, LayoutAlignItems, LayoutAlignSelf};

use super::{
    to_taffy_align_items, to_taffy_grid_align_items, to_taffy_grid_align_self,
    to_taffy_justify_self,
};

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

#[test]
fn justify_self_auto_keeps_replaced_start_fallback_but_explicit_value_wins() {
    assert_eq!(to_taffy_justify_self(LayoutAlignSelf::Auto, false), None);
    assert_eq!(
        to_taffy_justify_self(LayoutAlignSelf::Auto, true),
        Some(AlignSelf::START)
    );
    assert_eq!(
        to_taffy_justify_self(LayoutAlignSelf::Center, true),
        Some(AlignSelf::CENTER)
    );
    assert_eq!(
        to_taffy_justify_self(
            LayoutAlignSelf::Position {
                position: ItemAlignmentPosition::End,
                safety: None,
            },
            false,
        ),
        Some(AlignSelf {
            keyword: AlignItemsKeyword::End,
            safety: TaffyAlignmentSafety::Unsafe,
        })
    );
}
