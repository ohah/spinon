use spinon_core::{NodeId, StyleRevision};
use style::properties::LonghandId;

#[path = "runtime_grid/author_preflight.rs"]
mod author_preflight;
pub(super) use author_preflight::{
    TextOrientationScan, scan_text_orientation_declaration_list, scan_text_orientation_stylesheet,
};

use super::{
    ComputedStyleProfile, CssCascadeError, CssViewport, StyloDocumentView, compute_cascade,
    device::enable_orthogonal_writing_mode_preferences,
    runtime_layout::{RUNTIME_FLEX_LAYOUT_AUTHOR_PROPERTIES, runtime_flex_wrap_properties},
    runtime_paint::{
        RUNTIME_BLOCK_POSITIONING_AUTHOR_PROPERTIES, runtime_block_positioning_properties,
    },
};
use crate::{CssOrigin, StylesheetSource};

/// C11.1에서 혼합 Block/Flex/Grid cascade에 허용하는 CSS 선언입니다.
pub(super) fn runtime_block_flex_grid_author_properties() -> Vec<&'static str> {
    let mut properties = Vec::new();
    for property in RUNTIME_FLEX_LAYOUT_AUTHOR_PROPERTIES
        .iter()
        .chain(RUNTIME_BLOCK_POSITIONING_AUTHOR_PROPERTIES)
        .copied()
        .chain([
            "grid-template-columns",
            "grid-template-rows",
            "grid-auto-flow",
        ])
    {
        if property == "aspect-ratio" {
            continue;
        }
        if !properties.contains(&property) {
            properties.push(property);
        }
    }
    properties
}

pub(super) fn runtime_block_flex_grid_properties() -> Vec<(&'static str, LonghandId)> {
    let mut properties = runtime_flex_wrap_properties();
    for property in runtime_block_positioning_properties() {
        if !properties.iter().any(|(name, _)| *name == property.0) {
            properties.push(property);
        }
    }
    properties.extend([
        ("grid-template-columns", LonghandId::GridTemplateColumns),
        ("grid-template-rows", LonghandId::GridTemplateRows),
        ("grid-auto-flow", LonghandId::GridAutoFlow),
    ]);
    properties
}

pub(super) fn runtime_block_flex_grid_placement_author_properties() -> Vec<&'static str> {
    let mut properties = runtime_block_flex_grid_author_properties();
    for property in [
        "grid-row-start",
        "grid-row-end",
        "grid-column-start",
        "grid-column-end",
        "grid-row",
        "grid-column",
        "grid-area",
    ] {
        if !properties.contains(&property) {
            properties.push(property);
        }
    }
    properties
}

pub(super) fn runtime_block_flex_grid_placement_properties() -> Vec<(&'static str, LonghandId)> {
    let mut properties = runtime_block_flex_grid_properties();
    properties.extend([
        ("grid-row-start", LonghandId::GridRowStart),
        ("grid-row-end", LonghandId::GridRowEnd),
        ("grid-column-start", LonghandId::GridColumnStart),
        ("grid-column-end", LonghandId::GridColumnEnd),
    ]);
    properties
}

pub(super) fn runtime_block_flex_grid_auto_placement_author_properties() -> Vec<&'static str> {
    let mut properties = runtime_block_flex_grid_placement_author_properties();
    properties.push("grid-auto-rows");
    properties
}

pub(super) fn runtime_block_flex_grid_auto_placement_properties() -> Vec<(&'static str, LonghandId)>
{
    let mut properties = runtime_block_flex_grid_placement_properties();
    properties.push(("grid-auto-rows", LonghandId::GridAutoRows));
    properties
}

pub(super) fn runtime_block_flex_grid_dense_auto_placement_author_properties() -> Vec<&'static str>
{
    let mut properties = runtime_block_flex_grid_auto_placement_author_properties();
    properties.push("grid-auto-columns");
    properties
}

pub(super) fn runtime_block_flex_grid_dense_auto_placement_properties()
-> Vec<(&'static str, LonghandId)> {
    let mut properties = runtime_block_flex_grid_auto_placement_properties();
    properties.push(("grid-auto-columns", LonghandId::GridAutoColumns));
    properties
}

pub(super) fn runtime_block_flex_grid_template_author_properties() -> Vec<&'static str> {
    let mut properties = runtime_block_flex_grid_dense_auto_placement_author_properties();
    properties.extend(["grid-template", "grid", "grid-template-areas"]);
    properties
}

pub(super) fn runtime_block_flex_grid_template_properties() -> Vec<(&'static str, LonghandId)> {
    let mut properties = runtime_block_flex_grid_dense_auto_placement_properties();
    properties.push(("grid-template-areas", LonghandId::GridTemplateAreas));
    properties
}

pub(super) fn runtime_block_flex_grid_track_sizing_author_properties() -> Vec<&'static str> {
    let mut properties = runtime_block_flex_grid_template_author_properties();
    for property in [
        "aspect-ratio",
        "visibility",
        "font-family",
        "font-size",
        "font-weight",
        "font-style",
        "line-height",
        "tab-size",
        "white-space-collapse",
        "text-wrap-mode",
        "word-break",
        "direction",
        "line-break",
        "text-align",
        "overflow-wrap",
        "text-transform",
        "writing-mode",
        "letter-spacing",
        "word-spacing",
        "text-indent",
    ] {
        if !properties.contains(&property) {
            properties.push(property);
        }
    }
    properties
}

pub(super) fn runtime_block_flex_grid_track_sizing_properties() -> Vec<(&'static str, LonghandId)> {
    let mut properties = runtime_block_flex_grid_template_properties();
    properties.extend([
        ("grid-row-start", LonghandId::GridRowStart),
        ("grid-row-end", LonghandId::GridRowEnd),
        ("grid-column-start", LonghandId::GridColumnStart),
        ("grid-column-end", LonghandId::GridColumnEnd),
        ("visibility", LonghandId::Visibility),
    ]);
    properties.extend([
        ("font-family", LonghandId::FontFamily),
        ("font-size", LonghandId::FontSize),
        ("font-weight", LonghandId::FontWeight),
        ("font-style", LonghandId::FontStyle),
        ("line-height", LonghandId::LineHeight),
        ("tab-size", LonghandId::TabSize),
        ("white-space-collapse", LonghandId::WhiteSpaceCollapse),
        ("text-wrap-mode", LonghandId::TextWrapMode),
        ("word-break", LonghandId::WordBreak),
        ("direction", LonghandId::Direction),
        ("line-break", LonghandId::LineBreak),
        ("text-align", LonghandId::TextAlign),
        ("overflow-wrap", LonghandId::OverflowWrap),
        ("text-transform", LonghandId::TextTransform),
        ("writing-mode", LonghandId::WritingMode),
        ("text-orientation", LonghandId::TextOrientation),
        ("letter-spacing", LonghandId::LetterSpacing),
        ("word-spacing", LonghandId::WordSpacing),
        ("text-indent", LonghandId::TextIndent),
    ]);
    properties
}

/// C11.1 explicit Grid tracks와 기존 Block/Flex 입력을 같은 cascade에서 계산합니다.
pub fn compute_runtime_block_flex_grid_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &runtime_block_flex_grid_properties(),
        ComputedStyleProfile::RuntimeBlockFlexGridV1,
    )
}

/// C11.2.1 numeric explicit placement을 위한 독립 computed-style profile입니다.
pub fn compute_runtime_block_flex_grid_placement_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &runtime_block_flex_grid_placement_properties(),
        ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1,
    )
}

/// C11.2.2 sparse row auto-placement를 위한 독립 computed-style profile입니다.
pub fn compute_runtime_block_flex_grid_auto_placement_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &runtime_block_flex_grid_auto_placement_properties(),
        ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1,
    )
}

/// C11.2.3 dense·column auto-placement를 위한 독립 computed-style profile입니다.
pub fn compute_runtime_block_flex_grid_dense_auto_placement_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &runtime_block_flex_grid_dense_auto_placement_properties(),
        ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1,
    )
}

/// C11.3.1의 정수 `repeat()` 확장과 line-name metadata를 계산하는 누적 Grid profile입니다.
pub fn compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &runtime_block_flex_grid_template_properties(),
        ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
    )
}

/// C11.4 intrinsic track-sizing 입력을 포함하는 누적 Grid profile입니다.
pub fn compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    compute_runtime_block_flex_grid_track_sizing_cascade_with_author_properties(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        None,
        None,
        false,
    )
}

/// C11.4 overflow automatic minimum 고정 fixture에서만 overflow 선언을 cascade 합니다.
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub fn compute_runtime_block_flex_grid_track_sizing_overflow_fixture_test_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    let mut author_properties = runtime_block_flex_grid_track_sizing_author_properties();
    author_properties.extend(["overflow-x", "overflow-y"]);
    compute_runtime_block_flex_grid_track_sizing_cascade_with_author_properties(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        Some(&author_properties),
        None,
        false,
    )
}

/// C11.4 zoomed fit-content 고정 fixture에서만 author `zoom`을 계산합니다.
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub fn compute_runtime_block_flex_grid_track_sizing_zoom_fixture_test_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    let mut author_properties = runtime_block_flex_grid_track_sizing_author_properties();
    author_properties.push("zoom");
    let mut properties = runtime_block_flex_grid_track_sizing_properties();
    properties.push(("zoom", LonghandId::Zoom));
    compute_runtime_block_flex_grid_track_sizing_cascade_with_author_properties(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        Some(&author_properties),
        Some(&properties),
        false,
    )
}

/// C11.4 `visibility:collapse` 고정 Grid fixture에서만 해당 computed 값을 cascade 합니다.
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub fn compute_runtime_block_flex_grid_track_sizing_visibility_collapse_fixture_test_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    let author_properties = runtime_block_flex_grid_track_sizing_author_properties();
    compute_runtime_block_flex_grid_track_sizing_cascade_with_author_properties(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        Some(&author_properties),
        None,
        true,
    )
}

fn compute_runtime_block_flex_grid_track_sizing_cascade_with_author_properties(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
    author_properties_override: Option<&[&str]>,
    properties_override: Option<&[(&'static str, LonghandId)]>,
    allow_visibility_collapse: bool,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    let profile_view = {
        let _preferences = enable_orthogonal_writing_mode_preferences();
        for source in author_stylesheets
            .iter()
            .filter(|source| source.origin == CssOrigin::Author)
        {
            match scan_text_orientation_stylesheet(&source.css) {
                TextOrientationScan::Found => {
                    return Err(CssCascadeError::UnsupportedAuthorCss {
                        stylesheet_id: source.id.clone(),
                        feature: "text-orientation".to_owned(),
                    });
                }
                TextOrientationScan::Incomplete => {
                    return Err(CssCascadeError::UnsupportedAuthorCss {
                        stylesheet_id: source.id.clone(),
                        feature:
                            "C11.4 author-property preflight에서 CSS 전체를 검사하지 못했습니다"
                                .to_owned(),
                    });
                }
                TextOrientationScan::Clean => {}
            }
        }
        let profile_view = view
            .rebuilt_with_current_stylo_preferences()
            .map_err(CssCascadeError::OrthogonalWritingModeViewRebuild)?;
        let default_allowed = runtime_block_flex_grid_track_sizing_author_properties();
        let allowed = author_properties_override.unwrap_or(&default_allowed);
        if let Some((node, property)) = super::runtime_layout::first_unsupported_inline_declaration(
            &profile_view,
            |declaration, property| {
                allowed.contains(&property)
                    && (property != "visibility"
                        || crate::stylesheet_registry::supports_runtime_visibility_value(
                            declaration,
                        )
                        || (allow_visibility_collapse
                            && crate::stylesheet_registry::supports_runtime_visibility_collapse_value(
                                declaration,
                            )))
            },
        ) {
            return Err(CssCascadeError::UnsupportedInlineAuthorProperty { node, property });
        }
        match first_authored_disabled_inline_property(&profile_view) {
            Ok(Some((node, property))) => {
                return Err(CssCascadeError::UnsupportedInlineAuthorProperty { node, property });
            }
            Ok(None) => {}
            Err(node) => {
                return Err(CssCascadeError::InlineAuthorPropertyPreflightFailed { node });
            }
        }
        profile_view
    };
    let default_properties = runtime_block_flex_grid_track_sizing_properties();
    let properties = properties_override.unwrap_or(&default_properties);
    if let Some(author_properties) = author_properties_override {
        super::compute_cascade_with_author_properties_override(
            &profile_view,
            author_stylesheets,
            viewport,
            style_revision,
            properties,
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
            super::CascadeOptions {
                additional_user_agent_stylesheets: &[],
                author_properties_override: Some(author_properties),
                allow_visibility_collapse,
            },
        )
    } else {
        compute_cascade(
            &profile_view,
            author_stylesheets,
            viewport,
            style_revision,
            properties,
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
        )
    }
}

/// C11.1 inline Grid cascade 밖 선언을 찾습니다.
pub fn first_unsupported_runtime_block_flex_grid_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    let allowed = runtime_block_flex_grid_author_properties();
    super::runtime_layout::first_unsupported_inline_property(view, |property| {
        allowed.contains(&property)
    })
}

/// C11.2.1 inline cascade에서 미지원 author 속성을 찾습니다.
pub fn first_unsupported_runtime_block_flex_grid_placement_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    let allowed = runtime_block_flex_grid_placement_author_properties();
    super::runtime_layout::first_unsupported_inline_property(view, |property| {
        allowed.contains(&property)
    })
}

/// C11.2.2 inline cascade에서 지원하지 않는 author 속성을 찾습니다.
pub fn first_unsupported_runtime_block_flex_grid_auto_placement_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    let allowed = runtime_block_flex_grid_auto_placement_author_properties();
    super::runtime_layout::first_unsupported_inline_property(view, |property| {
        allowed.contains(&property)
    })
}

/// C11.2.3 inline cascade에서 지원하지 않는 author 속성을 찾습니다.
pub fn first_unsupported_runtime_block_flex_grid_dense_auto_placement_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    let allowed = runtime_block_flex_grid_dense_auto_placement_author_properties();
    super::runtime_layout::first_unsupported_inline_property(view, |property| {
        allowed.contains(&property)
    })
}

/// C11.3.1 inline cascade에서 지원하지 않는 author 속성을 찾습니다.
pub fn first_unsupported_runtime_block_flex_grid_template_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    let allowed = runtime_block_flex_grid_template_author_properties();
    super::runtime_layout::first_unsupported_inline_property(view, |property| {
        allowed.contains(&property)
    })
}

/// C11.4 inline cascade에서 지원하지 않는 author 속성을 찾습니다.
pub fn first_unsupported_runtime_block_flex_grid_track_sizing_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    first_unsupported_runtime_block_flex_grid_track_sizing_inline_property_with_visibility_collapse(
        view, false,
    )
}

/// C11.4 visibility fixture route에서만 inline `collapse`를 preflight에 허용합니다.
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub fn first_unsupported_runtime_block_flex_grid_track_sizing_visibility_collapse_fixture_test_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    first_unsupported_runtime_block_flex_grid_track_sizing_inline_property_with_visibility_collapse(
        view, true,
    )
}

fn first_unsupported_runtime_block_flex_grid_track_sizing_inline_property_with_visibility_collapse(
    view: &StyloDocumentView,
    allow_visibility_collapse: bool,
) -> Option<(NodeId, String)> {
    let allowed = runtime_block_flex_grid_track_sizing_author_properties();
    super::runtime_layout::first_unsupported_inline_declaration(view, |declaration, property| {
        allowed.contains(&property)
            && (property != "visibility"
                || crate::stylesheet_registry::supports_runtime_visibility_value(declaration)
                || allow_visibility_collapse
                    && crate::stylesheet_registry::supports_runtime_visibility_collapse_value(
                        declaration,
                    ))
    })
    .or_else(|| {
        view.inline_style_sources().find_map(|(node, css)| {
            matches!(
                scan_text_orientation_declaration_list(css),
                TextOrientationScan::Found
            )
            .then_some((node, "text-orientation".to_owned()))
        })
    })
}

/// C11.4 overflow automatic minimum fixture route의 inline 선언을 제한합니다.
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub fn first_unsupported_runtime_block_flex_grid_track_sizing_overflow_fixture_test_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    let mut allowed = runtime_block_flex_grid_track_sizing_author_properties();
    allowed.extend(["overflow-x", "overflow-y"]);
    super::runtime_layout::first_unsupported_inline_declaration(view, |declaration, property| {
        allowed.contains(&property)
            && (property != "visibility"
                || crate::stylesheet_registry::supports_runtime_visibility_value(declaration))
    })
}

/// C11.4 zoomed fit-content 고정 fixture route에서만 author `zoom`을 허용합니다.
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub fn first_unsupported_runtime_block_flex_grid_track_sizing_zoom_fixture_test_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    let mut allowed = runtime_block_flex_grid_track_sizing_author_properties();
    allowed.push("zoom");
    super::runtime_layout::first_unsupported_inline_declaration(view, |declaration, property| {
        allowed.contains(&property)
            && (property != "visibility"
                || crate::stylesheet_registry::supports_runtime_visibility_value(declaration))
    })
}

fn first_authored_disabled_inline_property(
    view: &StyloDocumentView,
) -> Result<Option<(NodeId, String)>, NodeId> {
    for (node, css) in view.inline_style_sources() {
        match scan_text_orientation_declaration_list(css) {
            TextOrientationScan::Found => {
                return Ok(Some((node, "text-orientation".to_owned())));
            }
            TextOrientationScan::Incomplete => return Err(node),
            TextOrientationScan::Clean => {}
        }
    }
    Ok(None)
}
