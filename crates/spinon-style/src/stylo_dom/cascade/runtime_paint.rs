use spinon_core::{NodeId, StyleRevision};
use style::properties::{ComputedValues, LonghandId};

use crate::{ComputedBackgroundPaint, OpaqueCssSrgb, StylesheetSource};

use super::{
    ComputedStyleProfile, CssCascadeError, CssViewport, StyloDocumentView, compute_cascade,
    runtime_layout::{RUNTIME_FLEX_LAYOUT_AUTHOR_PROPERTIES, RUNTIME_FLEX_LAYOUT_PROPERTIES},
};

pub(super) fn computed_background_for_profile(
    profile: ComputedStyleProfile,
    computed: &ComputedValues,
    node: NodeId,
) -> Result<(Option<OpaqueCssSrgb>, Option<ComputedBackgroundPaint>), CssCascadeError> {
    match profile {
        ComputedStyleProfile::S04FlexPaintV1 => {
            let color = super::s04::computed_background_color(computed, node)?;
            Ok((Some(color), Some(ComputedBackgroundPaint::Opaque(color))))
        }
        ComputedStyleProfile::RuntimeFlexPaintV1
        | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
        | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1 => {
            let paint = computed_runtime_background_paint(computed, node)?;
            let color = match paint {
                ComputedBackgroundPaint::Transparent => None,
                ComputedBackgroundPaint::Opaque(color) => Some(color),
            };
            Ok((color, Some(paint)))
        }
        _ => Ok((None, None)),
    }
}

/// C04.10 runtime Flex와 단색 배경 paint를 같은 Stylo cascade에서 계산합니다.
pub fn compute_runtime_flex_paint_cascade(
    view: &StyloDocumentView,
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    let mut properties = RUNTIME_FLEX_LAYOUT_PROPERTIES.to_vec();
    properties.push(("background-color", LonghandId::BackgroundColor));
    compute_cascade(
        view,
        &[],
        viewport,
        style_revision,
        &properties,
        super::ComputedStyleProfile::RuntimeFlexPaintV1,
    )
}

/// C05.1 사용자 지정 속성과 `var()`를 포함하는 runtime layout·paint snapshot을 계산합니다.
pub fn compute_runtime_flex_custom_properties_paint_cascade(
    view: &StyloDocumentView,
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets(
        view,
        &[],
        viewport,
        style_revision,
    )
}

/// C04.11 author stylesheet와 C05.1 사용자 지정 속성을 runtime layout·paint에서 계산합니다.
pub fn compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    let mut properties = RUNTIME_FLEX_LAYOUT_PROPERTIES.to_vec();
    properties.push(("background-color", LonghandId::BackgroundColor));
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &properties,
        super::ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1,
    )
}

/// C05.2 stylesheet `@property` 등록을 포함하는 runtime layout·paint snapshot을 계산합니다.
pub fn compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    let mut properties = RUNTIME_FLEX_LAYOUT_PROPERTIES.to_vec();
    properties.push(("background-color", LonghandId::BackgroundColor));
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        &properties,
        super::ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1,
    )
}

pub(super) fn computed_runtime_background_paint(
    computed: &ComputedValues,
    node: NodeId,
) -> Result<ComputedBackgroundPaint, CssCascadeError> {
    match computed.clone_background_color() {
        style::values::computed::Color::Absolute(color) => {
            OpaqueCssSrgb::runtime_paint_from_absolute_color(color).map_err(|reason| {
                CssCascadeError::UnsupportedComputedBackgroundColor {
                    node,
                    reason: reason.to_owned(),
                }
            })
        }
        _ => Err(CssCascadeError::UnsupportedComputedBackgroundColor {
            node,
            reason: "Stylo 계산값이 절대 색상이 아닙니다".to_owned(),
        }),
    }
}

pub fn first_unsupported_runtime_flex_paint_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    super::runtime_layout::first_unsupported_inline_property(view, |property| {
        property == "background-color" || RUNTIME_FLEX_LAYOUT_AUTHOR_PROPERTIES.contains(&property)
    })
}
