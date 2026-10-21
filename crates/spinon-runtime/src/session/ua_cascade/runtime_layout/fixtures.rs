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
#[path = "fixtures/c11_4_image.rs"]
mod c11_4_image;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "fixtures/c11_4_mixed.rs"]
mod c11_4_mixed;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub(super) struct C11_4TextInput {
    pub(super) paragraph_styles: BTreeMap<NodeId, ParagraphStyle>,
    pub(super) engine: TextLayoutEngine,
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub(super) use c11_4_text::prepare as prepare_c11_4_text;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub(super) struct C11_4ImageInput {
    pub(super) intrinsic_sizes: spinon_layout::LayoutIntrinsicSizeSnapshot,
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
pub(super) use c11_4_mixed::prepare as prepare_c11_4_mixed;
