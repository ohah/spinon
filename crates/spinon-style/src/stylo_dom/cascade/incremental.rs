use spinon_core::{NodeId, StyleRevision};
use style::properties::{ComputedValues, LonghandId};

use super::{
    ComputedCssDimension, ComputedCssSpacingValue, ComputedStyleProfile, ComputedStyleSnapshot,
    CssCascadeError, CssViewport, StyloDocumentView, UA_STYLESHEET_ID, runtime_layout,
    runtime_paint,
};
use crate::StylesheetSource;

fn computed_font_size_css_px(computed: &ComputedValues) -> f32 {
    computed.get_font().clone_font_size().computed_size().px()
}

fn validate_synthetic_document_box(
    computed: &ComputedValues,
    element: &'static str,
) -> Result<(), CssCascadeError> {
    let dimensions = dimensions::computed_layout_dimensions(computed);
    for (property, value) in [
        ("width", dimensions.width),
        ("height", dimensions.height),
        ("flex-basis", dimensions.flex_basis),
    ] {
        if value != ComputedCssDimension::Auto {
            return Err(CssCascadeError::UnsupportedSyntheticDocumentStyle {
                element,
                property,
                value: format!("{value:?}"),
            });
        }
    }

    let spacing = spacing::computed_layout_spacing(computed);
    for (property, value) in [
        ("margin-top", spacing.margin.top),
        ("margin-right", spacing.margin.right),
        ("margin-bottom", spacing.margin.bottom),
        ("margin-left", spacing.margin.left),
        ("padding-top", spacing.padding.top),
        ("padding-right", spacing.padding.right),
        ("padding-bottom", spacing.padding.bottom),
        ("padding-left", spacing.padding.left),
    ] {
        if value != ComputedCssSpacingValue::LengthPx(0.0) {
            return Err(CssCascadeError::UnsupportedSyntheticDocumentStyle {
                element,
                property,
                value: format!("{value:?}"),
            });
        }
    }

    let display = computed.computed_value_to_string(
        style::properties::PropertyDeclarationId::Longhand(LonghandId::Display),
    );
    if display != "block" {
        return Err(CssCascadeError::UnsupportedSyntheticDocumentStyle {
            element,
            property: "display",
            value: display,
        });
    }

    let background_color = computed.clone_background_color();
    let background_alpha = match background_color {
        style::values::computed::Color::Absolute(color) => {
            color.into_srgb_legacy().raw_components()[3]
        }
        _ => 1.0,
    };
    if background_alpha != 0.0 {
        return Err(CssCascadeError::UnsupportedSyntheticDocumentStyle {
            element,
            property: "background-color",
            value: computed.computed_value_to_string(
                style::properties::PropertyDeclarationId::Longhand(LonghandId::BackgroundColor),
            ),
        });
    }

    Ok(())
}

#[path = "incremental/aspect_ratio.rs"]
mod aspect_ratio;
#[path = "incremental/border.rs"]
mod border;
#[path = "incremental/compute.rs"]
mod compute;
#[path = "incremental/dimensions.rs"]
mod dimensions;
#[path = "incremental/element.rs"]
mod element;
#[path = "incremental/reuse.rs"]
mod reuse;
#[path = "incremental/source_border.rs"]
mod source_border;
#[path = "incremental/source_math.rs"]
mod source_math;
#[path = "incremental/spacing.rs"]
mod spacing;
pub(super) use aspect_ratio::computed_layout_aspect_ratio;
pub(super) use border::computed_layout_border;
pub(super) use compute::compute_cascade_with_reuse;

/// 안전한 runtime subtree 변경에서 이전 직렬화 스타일 출력을 재사용합니다.
/// `None`은 입력이 지원 범위 밖이므로 호출자가 전체 cascade로 되돌려야 함을 뜻합니다.
pub fn compute_runtime_incremental_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
    profile: ComputedStyleProfile,
    previous: &ComputedStyleSnapshot,
    dirty_root_ids: &[NodeId],
) -> Result<Option<(ComputedStyleSnapshot, RuntimeCascadeReuseStats)>, CssCascadeError> {
    let Some(properties) = runtime_properties_for_profile(profile) else {
        return Ok(None);
    };
    if !author_stylesheets.is_empty()
        || previous.profile != profile
        || !same_viewport(previous.viewport, viewport)
        || previous.generation != view.snapshot().generation()
        || previous.document_revision >= view.document_revision()
        || previous.render_tree_revision > view.render_tree_revision()
        || previous.style_revision != style_revision
        || UA_STYLESHEET_ID != "spinon-ua-supported-elements-v0"
    {
        return Ok(None);
    }

    compute_cascade_with_reuse(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &properties,
        profile,
        Some((previous, dirty_root_ids)),
    )
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RuntimeCascadeReuseStats {
    /// 현재 요청에서 Stylo cascade 함수를 호출하고 새 출력을 만든 요소 수입니다.
    pub recomputed_style_elements: u64,
    /// 이전 직렬화 출력 항목을 현재 snapshot에 재사용한 요소 수입니다.
    pub reused_style_elements: u64,
    /// 상속 context 계산 후 이전 출력 항목을 재사용한 조상 수입니다.
    pub context_style_elements: u64,
}

fn runtime_properties_for_profile(
    profile: ComputedStyleProfile,
) -> Option<Vec<(&'static str, LonghandId)>> {
    if profile == ComputedStyleProfile::RuntimeBlockPaintV1 {
        return Some(runtime_paint::runtime_block_paint_properties());
    }
    if profile == ComputedStyleProfile::RuntimeBlockFormattingV1 {
        return Some(runtime_paint::runtime_block_formatting_properties());
    }
    let has_paint = matches!(
        profile,
        ComputedStyleProfile::RuntimeFlexPaintV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
    );
    if matches!(
        profile,
        ComputedStyleProfile::RuntimeFlexLayoutV1
            | ComputedStyleProfile::RuntimeFlexPaintV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
    ) {
        let mut properties = runtime_layout::runtime_flex_wrap_properties();
        if has_paint {
            properties.push(("background-color", LonghandId::BackgroundColor));
        }
        return Some(properties);
    }
    None
}

fn same_viewport(left: CssViewport, right: CssViewport) -> bool {
    left.width_css_px.to_bits() == right.width_css_px.to_bits()
        && left.height_css_px.to_bits() == right.height_css_px.to_bits()
        && left.device_scale_factor.to_bits() == right.device_scale_factor.to_bits()
        && left.environment_revision == right.environment_revision
        && left.media_environment == right.media_environment
}

fn is_runtime_layout_profile(profile: ComputedStyleProfile) -> bool {
    matches!(
        profile,
        ComputedStyleProfile::RuntimeFlexLayoutV1
            | ComputedStyleProfile::RuntimeFlexPaintV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
            | ComputedStyleProfile::RuntimeBlockPaintV1
            | ComputedStyleProfile::RuntimeBlockFormattingV1
    )
}
