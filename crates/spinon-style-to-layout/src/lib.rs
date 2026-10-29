//! Stylo computed-style snapshot을 Taffy layout 입력으로 연결하는 내부 adapter입니다.

mod error;
mod projection;

#[cfg(test)]
mod absolute_length_tests;
#[cfg(test)]
mod c07_2_border_layout_tests;
#[cfg(test)]
mod c07_3_aspect_ratio_tests;
#[cfg(test)]
mod c10_3_5_positioned_tests;
#[cfg(test)]
mod c10_flex_alignment_tests;
#[cfg(test)]
mod c10_flex_baseline_tests;
#[cfg(test)]
mod c10_flex_order_tests;
#[cfg(test)]
mod c10_flex_wrap_tests;
#[cfg(test)]
mod c11_2_1_grid_placement_tests;
#[cfg(test)]
mod c11_2_2_grid_auto_placement_tests;
#[cfg(test)]
mod c11_2_3_grid_auto_placement_tests;
#[cfg(test)]
mod c11_3_1_grid_repeat_tests;
#[cfg(test)]
mod c11_3_2_grid_named_line_tests;
#[cfg(test)]
mod c11_3_3_grid_template_areas_tests;
#[cfg(test)]
mod c11_4_track_sizing_areas_tests;
#[cfg(test)]
mod c11_grid_tests;
#[cfg(test)]
mod c12_1_position_tests;
#[cfg(test)]
mod c12_2_absolute_tests;
#[cfg(test)]
mod c12_3_fixed_tests;
#[cfg(test)]
mod c13_1_overflow_clip_tests;
#[cfg(test)]
mod c13_3_scroll_gesture_style_layout_tests;
#[cfg(test)]
mod c14_intrinsic_contributions_tests;
#[cfg(test)]
mod c14_intrinsic_size_tests;
#[cfg(test)]
mod c14_png_resource_tests;
#[cfg(test)]
mod cascade_layers_tests;
#[cfg(test)]
mod flex_alignment_tests;
#[cfg(test)]
mod font_relative_units_tests;
#[cfg(test)]
mod margin_tests;
#[cfg(test)]
mod min_max_sizing_tests;
#[cfg(test)]
mod percentage_tests;
#[cfg(test)]
mod runtime_layout_tests;
#[cfg(test)]
mod spacing_percentage_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod typed_css_math_tests;
#[cfg(test)]
mod viewport_units_tests;

pub use error::StyleLayoutError;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub use projection::compute_runtime_block_flex_grid_track_sizing_style_layout_with_anonymous_text_and_intrinsic_sizes;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub use projection::compute_runtime_block_flex_grid_track_sizing_style_layout_with_anonymous_text_and_intrinsic_sizes_and_two_axis_nested_grid;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub use projection::compute_runtime_block_flex_grid_track_sizing_style_layout_with_overflow_automatic_minimum_fixture;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub use projection::compute_runtime_block_flex_grid_track_sizing_style_layout_with_overflow_automatic_minimum_fixture_and_intrinsic_sizes;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub use projection::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_and_intrinsic_sizes;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub use projection::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_and_intrinsic_sizes_and_two_axis_grid_fixture;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub use projection::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_first_baseline_fixture;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub use projection::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub use projection::compute_runtime_block_flex_grid_track_sizing_style_layout_with_two_axis_nested_grid_text_paragraphs;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub use projection::compute_runtime_style_layout_with_intrinsic_size_snapshot_for_c11_4_two_axis_grid_fixture;
#[cfg(feature = "c04-12-root-html-text-paint-fixture-test")]
pub use projection::compute_runtime_style_layout_with_text_paragraphs;
pub use projection::compute_runtime_style_layout_with_text_profiles;
pub use projection::{
    StyleLayoutOutput, compute_flex_alignment_layers_style_layout,
    compute_flex_alignment_style_layout, compute_flex_margin_style_layout,
    compute_runtime_block_flex_grid_auto_placement_style_layout,
    compute_runtime_block_flex_grid_dense_auto_placement_style_layout,
    compute_runtime_block_flex_grid_placement_style_layout,
    compute_runtime_block_flex_grid_style_layout,
    compute_runtime_block_flex_grid_template_implicit_named_lines_style_layout,
    compute_runtime_block_flex_grid_template_style_layout,
    compute_runtime_block_flex_grid_track_sizing_style_layout, compute_runtime_style_layout,
    compute_runtime_style_layout_with_implicit_named_lines,
    compute_runtime_style_layout_with_intrinsic_size_snapshot, compute_s04_style_layout,
    compute_style_layout,
};
