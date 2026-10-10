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
mod c10_flex_alignment_tests;
#[cfg(test)]
mod c10_flex_baseline_tests;
#[cfg(test)]
mod c10_flex_order_tests;
#[cfg(test)]
mod c10_flex_wrap_tests;
#[cfg(test)]
mod c12_1_position_tests;
#[cfg(test)]
mod c12_2_absolute_tests;
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
pub use projection::{
    StyleLayoutOutput, compute_flex_alignment_layers_style_layout,
    compute_flex_alignment_style_layout, compute_flex_margin_style_layout,
    compute_runtime_style_layout, compute_s04_style_layout, compute_style_layout,
};
