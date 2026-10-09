use super::{
    ComputedStyleProfile, ComputedStyleSnapshot, CssCascadeError, CssViewport, StyloDocumentView,
    compute_cascade,
};
use crate::StylesheetSource;
use spinon_core::{NodeId, StyleRevision};
use style::properties::LonghandId;

pub(super) const RUNTIME_FLEX_LAYOUT_PROPERTIES: &[(&str, LonghandId)] = &[
    ("display", LonghandId::Display),
    ("list-style-type", LonghandId::ListStyleType),
    ("margin-block-start", LonghandId::MarginBlockStart),
    ("margin-block-end", LonghandId::MarginBlockEnd),
    ("margin-inline-start", LonghandId::MarginInlineStart),
    ("margin-inline-end", LonghandId::MarginInlineEnd),
    ("padding-inline-start", LonghandId::PaddingInlineStart),
    ("box-sizing", LonghandId::BoxSizing),
    ("width", LonghandId::Width),
    ("height", LonghandId::Height),
    ("flex-direction", LonghandId::FlexDirection),
    ("flex-grow", LonghandId::FlexGrow),
    ("flex-shrink", LonghandId::FlexShrink),
    ("flex-basis", LonghandId::FlexBasis),
    ("direction", LonghandId::Direction),
    ("align-items", LonghandId::AlignItems),
    ("justify-content", LonghandId::JustifyContent),
    ("row-gap", LonghandId::RowGap),
    ("column-gap", LonghandId::ColumnGap),
    ("margin-top", LonghandId::MarginTop),
    ("margin-right", LonghandId::MarginRight),
    ("margin-bottom", LonghandId::MarginBottom),
    ("margin-left", LonghandId::MarginLeft),
    ("padding-top", LonghandId::PaddingTop),
    ("padding-right", LonghandId::PaddingRight),
    ("padding-bottom", LonghandId::PaddingBottom),
    ("padding-left", LonghandId::PaddingLeft),
];

pub(super) const RUNTIME_FLEX_LAYOUT_AUTHOR_PROPERTIES: &[&str] = &[
    "display",
    "box-sizing",
    "width",
    "height",
    "flex",
    "flex-direction",
    "flex-grow",
    "flex-shrink",
    "flex-basis",
    "direction",
    "align-items",
    "justify-content",
    "gap",
    "row-gap",
    "column-gap",
    "margin",
    "margin-top",
    "margin-right",
    "margin-bottom",
    "margin-left",
    "margin-block",
    "margin-block-start",
    "margin-block-end",
    "margin-inline",
    "margin-inline-start",
    "margin-inline-end",
    "padding",
    "padding-top",
    "padding-right",
    "padding-bottom",
    "padding-left",
    "padding-block",
    "padding-block-start",
    "padding-block-end",
    "padding-inline",
    "padding-inline-start",
    "padding-inline-end",
];

/// C04 runtime에서 UA 속성과 제한 layout 입력을 한 번의 cascade로 계산합니다.
pub fn compute_runtime_flex_layout_cascade(
    view: &StyloDocumentView,
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<ComputedStyleSnapshot, CssCascadeError> {
    compute_cascade(
        view,
        &[],
        viewport,
        style_revision,
        RUNTIME_FLEX_LAYOUT_PROPERTIES,
        ComputedStyleProfile::RuntimeFlexLayoutV1,
    )
}

/// C05.1 사용자 지정 속성과 `var()`를 포함하는 제한 runtime layout snapshot을 계산합니다.
pub fn compute_runtime_flex_custom_properties_cascade(
    view: &StyloDocumentView,
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<ComputedStyleSnapshot, CssCascadeError> {
    compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        view,
        &[],
        viewport,
        style_revision,
    )
}

/// C04.11에서 수집한 author stylesheet와 C05.1 사용자 지정 속성을 runtime layout에서 계산합니다.
pub fn compute_runtime_flex_custom_properties_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<ComputedStyleSnapshot, CssCascadeError> {
    compute_cascade(
        view,
        author_stylesheets,
        viewport,
        style_revision,
        RUNTIME_FLEX_LAYOUT_PROPERTIES,
        ComputedStyleProfile::RuntimeFlexCustomPropertiesV1,
    )
}

/// Runtime CSS layout에서 계산 가능한 inline declaration만 통과시킵니다.
///
/// CSS parser가 확장한 longhand를 검사하므로 shorthand와 논리 속성도 원래 규칙에 따라
/// 처리됩니다. 원문 CSS는 반환하지 않습니다.
pub fn first_unsupported_runtime_layout_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    const ALLOWED_PROPERTIES: &[&str] = &[
        "display",
        "box-sizing",
        "width",
        "height",
        "flex-direction",
        "flex-grow",
        "flex-shrink",
        "flex-basis",
        "direction",
        "align-items",
        "justify-content",
        "row-gap",
        "column-gap",
        "margin-top",
        "margin-right",
        "margin-bottom",
        "margin-left",
        "margin-block-start",
        "margin-block-end",
        "margin-inline-start",
        "margin-inline-end",
        "padding-top",
        "padding-right",
        "padding-bottom",
        "padding-left",
        "padding-block-start",
        "padding-block-end",
        "padding-inline-start",
        "padding-inline-end",
    ];

    first_unsupported_inline_property(view, |property| ALLOWED_PROPERTIES.contains(&property))
}

/// C05.1 runtime layout에서 허용하지 않은 inline CSS 속성을 반환합니다.
pub fn first_unsupported_runtime_custom_properties_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    first_unsupported_runtime_custom_properties_inline_property_with_paint(view, false)
}

/// C05.1 paint profile에서 허용하지 않은 inline CSS 속성을 반환합니다.
pub fn first_unsupported_runtime_custom_properties_paint_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    first_unsupported_runtime_custom_properties_inline_property_with_paint(view, true)
}

fn first_unsupported_runtime_custom_properties_inline_property_with_paint(
    view: &StyloDocumentView,
    allow_background_color: bool,
) -> Option<(NodeId, String)> {
    first_unsupported_inline_property(view, |property| {
        property.starts_with("--")
            || RUNTIME_FLEX_LAYOUT_AUTHOR_PROPERTIES.contains(&property)
            || (allow_background_color && property == "background-color")
    })
}

pub(super) fn first_unsupported_inline_property(
    view: &StyloDocumentView,
    is_allowed: impl Fn(&str) -> bool,
) -> Option<(NodeId, String)> {
    let guard = view.shared_lock().read();
    let mut pending = vec![view.root_handle()];
    while let Some(handle) = pending.pop() {
        if let Some(data) = view.element_data(handle)
            && let Some(inline_style) = data.inline_style.as_ref()
        {
            let block = inline_style.read_with(&guard);
            for declaration in block.declarations() {
                let property = declaration.id().name().into_owned();
                if !is_allowed(&property) {
                    return Some((handle.id(), property));
                }
            }
        }
        if let Some(children) = view.snapshot().children(handle) {
            let children = children
                .filter(|child| view.is_member(*child))
                .collect::<Vec<_>>();
            pending.extend(children.into_iter().rev());
        }
    }
    None
}
