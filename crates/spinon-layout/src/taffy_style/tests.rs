use taffy::style::AlignItems;

use crate::LayoutAlignItems;

use super::to_taffy_align_items;

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
