pub(crate) mod data;
mod host;

#[cfg(feature = "c04-12-root-html-block-paint-fixture-test")]
pub(super) mod c04_12_root_html_block_paint;
#[cfg(feature = "c04-12-root-html-text-paint-fixture-test")]
pub(super) mod c04_12_root_html_text_paint;

#[cfg(feature = "c15-3-wgpu-glyph-paint-fixture-test")]
pub(super) mod c15_3;
#[cfg(feature = "c15-3-wgpu-glyph-paint-fixture-test")]
pub(super) mod c15_3_hostdocument_text_paint;
#[cfg(feature = "c18-1-image-paint-fixture-test")]
pub(super) mod c18_1;
#[cfg(feature = "c18-2-alpha-image-compositing-fixture-test")]
pub(super) mod c18_2;
#[cfg(feature = "c18-3-object-fit-position-fixture-test")]
pub(super) mod c18_3;
#[cfg(feature = "c18-4-image-fixture-test")]
pub(super) mod c18_4;
#[cfg(feature = "c19-2-box-paint-opacity-fixture-test")]
pub(super) mod c19_2;

pub(crate) mod c11;
pub(crate) mod c11_2_2;
#[cfg(feature = "c11-2-3-grid-fixture-test")]
pub(crate) mod c11_2_3;
pub(crate) mod c11_3_1;
pub(crate) mod c11_3_2;
pub(crate) mod c11_3_3;
#[cfg(feature = "c13-1-overflow-clip-fixture-test")]
pub(crate) mod c13_1;
#[cfg(feature = "c13-1-overflow-clip-fixture-test")]
pub(crate) mod c13_1_compact_surface_matrix;
#[cfg(feature = "c13-1-overflow-clip-fixture-test")]
pub(crate) mod c13_1_stacking_descendant_clip;
#[cfg(feature = "c13-2-scroll-fixture-test")]
pub(crate) mod c13_2_scroll;
#[cfg(feature = "c13-2-scroll-fixture-test")]
mod c13_2_scroll_host;
#[cfg(feature = "c13-3-gesture-fixture-test")]
pub(crate) mod c13_3_scroll_gesture;
#[cfg(feature = "c13-3-gesture-fixture-test")]
pub(super) mod c13_3_scroll_gesture_host;
