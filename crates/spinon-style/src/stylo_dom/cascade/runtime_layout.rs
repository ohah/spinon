use super::{
    ComputedStyleProfile, ComputedStyleSnapshot, CssCascadeError, CssViewport, StyloDocumentView,
    compute_cascade,
};
use spinon_core::{NodeId, StyleRevision};
use style::properties::LonghandId;

const RUNTIME_FLEX_LAYOUT_PROPERTIES: &[(&str, LonghandId)] = &[
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

    let guard = view.shared_lock().read();
    let mut pending = vec![view.root_handle()];
    while let Some(handle) = pending.pop() {
        if let Some(data) = view.element_data(handle)
            && let Some(inline_style) = data.inline_style.as_ref()
        {
            let block = inline_style.read_with(&guard);
            for declaration in block.declarations() {
                let property = declaration.id().name().into_owned();
                if !ALLOWED_PROPERTIES.contains(&property.as_str()) {
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
