use std::collections::BTreeMap;

use spinon_core::NodeId;
use spinon_layout::{ParagraphStyle, TextLayoutEngine};

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "fixtures/c11_4_text.rs"]
mod c11_4_text;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "fixtures/c11_4_implicit_intrinsic_text.rs"]
mod c11_4_implicit_intrinsic_text;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "fixtures/c11_4_nested_grid_intrinsic.rs"]
mod c11_4_nested_grid_intrinsic;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "fixtures/c11_4_nested_grid_depth_two.rs"]
mod c11_4_nested_grid_depth_two;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "fixtures/c11_4_nested_grid_depth_three_anonymous_image.rs"]
mod c11_4_nested_grid_depth_three_anonymous_image;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "fixtures/c11_4_nested_grid_text_multispan.rs"]
mod c11_4_nested_grid_text_multispan;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "fixtures/c11_4_nested_grid_replaced_item.rs"]
mod c11_4_nested_grid_replaced_item;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "fixtures/c11_4_nested_grid_rows.rs"]
mod c11_4_nested_grid_rows;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "fixtures/c11_4_nested_grid_implicit_rows.rs"]
mod c11_4_nested_grid_implicit_rows;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "fixtures/c11_4_image.rs"]
mod c11_4_image;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "fixtures/c11_4_mixed.rs"]
mod c11_4_mixed;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "fixtures/c11_4_anonymous_text.rs"]
mod c11_4_anonymous_text;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "fixtures/c11_4_anonymous_text_image.rs"]
mod c11_4_anonymous_text_image;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "fixtures/c11_4_anonymous_text_multi_image.rs"]
mod c11_4_anonymous_text_multi_image;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "fixtures/c11_4_anonymous_grid_border_overflow.rs"]
mod c11_4_anonymous_grid_border_overflow;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub(super) struct C11_4TextInput {
    pub(super) paragraph_styles: BTreeMap<NodeId, ParagraphStyle>,
    pub(super) engine: TextLayoutEngine,
    pub(super) two_axis_nested_grid: bool,
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub(super) use c11_4_text::prepare as prepare_c11_4_text;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub(super) use c11_4_text::is_cyclic_fit_content_calc_scene_root;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub(super) use c11_4_text::prepare_cyclic_fit_content_calc_render_input;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub(super) struct C11_4ImageInput {
    pub(super) intrinsic_sizes: spinon_layout::LayoutIntrinsicSizeSnapshot,
    pub(super) allow_two_axis_root_intrinsic: bool,
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub(super) use c11_4_image::prepare as prepare_c11_4_image;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub(super) struct C11_4MixedInput {
    pub(super) paragraph_styles: BTreeMap<NodeId, ParagraphStyle>,
    pub(super) engine: TextLayoutEngine,
    pub(super) intrinsic_sizes: spinon_layout::LayoutIntrinsicSizeSnapshot,
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub(super) struct C11_4NestedAnonymousImageInput {
    pub(super) paragraph_styles: BTreeMap<NodeId, ParagraphStyle>,
    pub(super) engine: TextLayoutEngine,
    pub(super) intrinsic_sizes: spinon_layout::LayoutIntrinsicSizeSnapshot,
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub(super) use c11_4_mixed::prepare as prepare_c11_4_mixed;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub(super) use c11_4_nested_grid_replaced_item::prepare as prepare_c11_4_nested_grid_replaced_item;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub(super) use c11_4_nested_grid_depth_three_anonymous_image::prepare as prepare_c11_4_nested_grid_depth_three_anonymous_image;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub(super) use c11_4_anonymous_text::prepare as prepare_c11_4_anonymous_text;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub(super) use c11_4_anonymous_text_image::prepare as prepare_c11_4_anonymous_text_image;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub(super) use c11_4_anonymous_text_multi_image::prepare as prepare_c11_4_anonymous_text_multi_image;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub(super) use c11_4_anonymous_grid_border_overflow::prepare as prepare_c11_4_anonymous_grid_border_overflow;
