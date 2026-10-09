//! Stylo computed-style snapshot을 Taffy layout 입력으로 연결하는 내부 adapter입니다.

mod error;
mod projection;

#[cfg(test)]
mod cascade_layers_tests;
#[cfg(test)]
mod flex_alignment_tests;
#[cfg(test)]
mod margin_tests;
#[cfg(test)]
mod tests;

pub use error::StyleLayoutError;
pub use projection::{
    StyleLayoutOutput, compute_flex_alignment_layers_style_layout,
    compute_flex_alignment_style_layout, compute_flex_margin_style_layout,
    compute_s04_style_layout, compute_style_layout,
};
