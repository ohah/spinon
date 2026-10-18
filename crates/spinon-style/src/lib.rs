//! 앱 바이너리에 포함하는 Spinon 기본 스타일 자원입니다.

use std::ffi::CStr;

mod c10_flex_baseline_css;
mod computed_css_color;
mod container_relative_units;
mod font_relative_units;
mod opaque_css_srgb;
mod s04_color_syntax;
mod stylesheet_registry;
mod stylo_dom;
mod wcag_contrast;

pub use computed_css_color::{ComputedCssColor, ComputedCssColorSpace};
pub use opaque_css_srgb::{ComputedBackgroundPaint, ComputedSrgbColor, OpaqueCssSrgb};
pub use stylesheet_registry::{
    CssOrigin, CssParseDiagnostic, RegisteredStylesheet, StylesheetRegistry,
    StylesheetRegistryError, StylesheetSource,
};
pub use stylo_dom::{
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
    StyloDocument, StyloDocumentView, StyloDomError, StyloElement, StyloNode,
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
pub use wcag_contrast::{FinalOpaqueSrgb, FinalOpaqueSrgbError, wcag_22_contrast_ratio};

/// 지원 HTML 요소 기본 스타일 프로필의 초안 식별자입니다.
pub const UA_STYLESHEET_PROFILE_ID: &CStr = c"spinon-html-ua/0.1.0-draft";

/// 컴파일 시 앱 바이너리에 포함하는 기본 스타일 규칙입니다.
pub const UA_STYLESHEET: &str = include_str!("../resources/ua/supported-elements-v0.css");
