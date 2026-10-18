mod cascade;
mod document;
mod element;
mod node;
mod selectors;
mod style;

pub(crate) use cascade::{acquire_standard_stylo_preferences, initialize_stylo_preferences};

#[cfg(test)]
mod tests;

pub use cascade::{
    CascadeDiagnostic, ComputedBorderPaint, ComputedCssDimension, ComputedCssEdges,
    ComputedCssGridArea, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridItemPlacement, ComputedCssGridLayout,
    ComputedCssGridPlacement, ComputedCssGridTemplateAreas, ComputedCssGridTrackSize,
    ComputedCssMath, ComputedCssMaxSize, ComputedCssOpacity, ComputedCssOverflow,
    ComputedCssOverflowClipMargin, ComputedCssOverflowVisualBox, ComputedCssOverscrollBehavior,
    ComputedCssPosition, ComputedCssSpacingValue, ComputedCssTouchAction, ComputedCssVisibility,
    ComputedCssZIndex, ComputedElementStyle, ComputedLayoutBorder, ComputedLayoutDimensions,
    ComputedLayoutInsets, ComputedLayoutSpacing, ComputedStyleProfile, ComputedStyleSnapshot,
    CssCascadeError, CssColorScheme, CssMediaEnvironment, CssPointerCapabilities,
    CssPrimaryPointer, CssViewport, FixedContainingBlockEffect, RuntimeCascadeReuseStats,
    compute_c19_box_paint_opacity_cascade_with_stylesheets,
    compute_c19_computed_color_cascade_with_stylesheets, compute_flex_alignment_cascade,
    compute_flex_alignment_layers_cascade, compute_flex_layout_cascade,
    compute_flex_margin_cascade, compute_flex_media_environment_cascade,
    compute_runtime_block_flex_grid_auto_placement_cascade_with_stylesheets,
    compute_runtime_block_flex_grid_cascade_with_stylesheets,
    compute_runtime_block_flex_grid_dense_auto_placement_cascade_with_stylesheets,
    compute_runtime_block_flex_grid_placement_cascade_with_stylesheets,
    compute_runtime_block_flex_grid_template_cascade_with_stylesheets,
    compute_runtime_block_flex_overflow_clip_cascade_with_stylesheets,
    compute_runtime_block_flex_scroll_gesture_cascade_with_stylesheets,
    compute_runtime_block_flex_stacking_cascade_with_stylesheets,
    compute_runtime_block_formatting_cascade_with_stylesheets,
    compute_runtime_block_image_cascade_with_stylesheets, compute_runtime_block_paint_cascade,
    compute_runtime_block_paint_cascade_with_stylesheets,
    compute_runtime_block_positioning_cascade_with_stylesheets,
    compute_runtime_flex_custom_properties_cascade,
    compute_runtime_flex_custom_properties_cascade_with_stylesheets,
    compute_runtime_flex_custom_properties_media_environment_cascade_with_stylesheets,
    compute_runtime_flex_custom_properties_paint_cascade,
    compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets,
    compute_runtime_flex_layout_cascade, compute_runtime_flex_paint_cascade,
    compute_runtime_flex_registered_properties_cascade_with_stylesheets,
    compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets,
    compute_runtime_incremental_cascade_with_stylesheets, compute_s04_flex_paint_cascade,
    compute_supported_elements_ua_cascade, first_unsupported_runtime_block_display_inline_value,
    first_unsupported_runtime_block_display_stylesheet_value,
    first_unsupported_runtime_block_flex_grid_auto_placement_inline_property,
    first_unsupported_runtime_block_flex_grid_dense_auto_placement_inline_property,
    first_unsupported_runtime_block_flex_grid_inline_property,
    first_unsupported_runtime_block_flex_grid_placement_inline_property,
    first_unsupported_runtime_block_flex_grid_template_inline_property,
    first_unsupported_runtime_block_flex_overflow_clip_inline_property,
    first_unsupported_runtime_block_flex_scroll_gesture_inline_property,
    first_unsupported_runtime_block_flex_stacking_inline_property,
    first_unsupported_runtime_block_formatting_inline_property,
    first_unsupported_runtime_block_image_inline_property,
    first_unsupported_runtime_block_paint_inline_property,
    first_unsupported_runtime_block_positioning_inline_property,
    first_unsupported_runtime_custom_properties_inline_property,
    first_unsupported_runtime_custom_properties_paint_inline_property,
    first_unsupported_runtime_flex_paint_inline_property,
    first_unsupported_runtime_layout_inline_property,
    first_unsupported_runtime_root_html_inline_property,
};
pub use document::{StyloDocument, StyloDocumentView, StyloDomError};
pub use element::StyloElement;
pub use node::StyloNode;
