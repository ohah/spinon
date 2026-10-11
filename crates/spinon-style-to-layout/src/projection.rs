use std::collections::BTreeMap;

mod style_values;
mod typed_math;
use style_values::project_styles;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, NodeId, StyleRevision};
use spinon_layout::{
    LayoutDisplay, LayoutEdges, LayoutEngine, LayoutFrame, LayoutInput, LayoutOutput, LayoutStyle,
    TaffyLayoutEngine, Viewport,
};
use spinon_style::{
    ComputedStyleProfile, ComputedStyleSnapshot, CssViewport, StylesheetSource, StyloDocumentView,
    compute_flex_alignment_cascade, compute_flex_alignment_layers_cascade,
    compute_flex_layout_cascade, compute_flex_margin_cascade, compute_runtime_flex_paint_cascade,
    compute_s04_flex_paint_cascade,
};

use crate::StyleLayoutError;
use validation::{assert_matching_revision, assert_no_inline_style, assert_view_matches_snapshot};

mod fixed_containing_block;
mod validation;

/// 같은 document snapshot에서 계산한 제한 CSS style과 Taffy layout 결과입니다.
#[derive(Clone, Debug)]
pub struct StyleLayoutOutput {
    pub computed_styles: ComputedStyleSnapshot,
    pub layout: LayoutOutput,
}

/// C04.2 profile로 Stylo cascade를 계산하고 HostDocument subtree를 Taffy에 전달합니다.
pub fn compute_style_layout(
    snapshot: &HostDocumentSnapshot,
    view: &StyloDocumentView,
    root: HostNodeHandle,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    compute_profile_layout(
        snapshot,
        view,
        root,
        author_stylesheets,
        viewport,
        style_revision,
        ComputedStyleProfile::FlexLayoutV1,
    )
}

/// C04.6 profile로 네 방향 CSS margin을 계산하고 Taffy Flex에 전달합니다.
pub fn compute_flex_margin_style_layout(
    snapshot: &HostDocumentSnapshot,
    view: &StyloDocumentView,
    root: HostNodeHandle,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    compute_profile_layout(
        snapshot,
        view,
        root,
        author_stylesheets,
        viewport,
        style_revision,
        ComputedStyleProfile::FlexMarginV1,
    )
}

/// C04.3 profile로 제한된 Flex 정렬 값과 레이아웃을 연결합니다.
pub fn compute_flex_alignment_style_layout(
    snapshot: &HostDocumentSnapshot,
    view: &StyloDocumentView,
    root: HostNodeHandle,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    compute_profile_layout(
        snapshot,
        view,
        root,
        author_stylesheets,
        viewport,
        style_revision,
        ComputedStyleProfile::FlexAlignmentV1,
    )
}

/// C04.4 profile로 CSS Cascade Layers를 포함한 제한 Flex 정렬과 레이아웃을 계산합니다.
pub fn compute_flex_alignment_layers_style_layout(
    snapshot: &HostDocumentSnapshot,
    view: &StyloDocumentView,
    root: HostNodeHandle,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    compute_profile_layout(
        snapshot,
        view,
        root,
        author_stylesheets,
        viewport,
        style_revision,
        ComputedStyleProfile::FlexAlignmentCascadeLayersV1,
    )
}

/// S04 새 paint profile만 대상으로 계산 style과 Taffy layout을 연결합니다.
pub fn compute_s04_style_layout(
    snapshot: &HostDocumentSnapshot,
    view: &StyloDocumentView,
    root: HostNodeHandle,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    compute_profile_layout(
        snapshot,
        view,
        root,
        author_stylesheets,
        viewport,
        style_revision,
        ComputedStyleProfile::S04FlexPaintV1,
    )
}

/// 이미 계산한 runtime profile을 Stylo를 다시 호출하지 않고 Taffy에 전달합니다.
pub fn compute_runtime_style_layout(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    let reject_cascade_diagnostics = matches!(
        computed_styles.profile,
        ComputedStyleProfile::RuntimeBlockPositioningV1
    );
    compute_layout_from_styles(
        snapshot,
        root,
        computed_styles,
        viewport,
        reject_cascade_diagnostics,
    )
}

fn compute_profile_layout(
    snapshot: &HostDocumentSnapshot,
    view: &StyloDocumentView,
    root: HostNodeHandle,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
    profile: ComputedStyleProfile,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    assert_view_matches_snapshot(snapshot, view)?;
    assert_no_inline_style(snapshot, root)?;
    let computed_styles = match profile {
        ComputedStyleProfile::FlexLayoutV1 => {
            compute_flex_layout_cascade(view, author_stylesheets, viewport, style_revision)?
        }
        ComputedStyleProfile::FlexMarginV1 => {
            compute_flex_margin_cascade(view, author_stylesheets, viewport, style_revision)?
        }
        ComputedStyleProfile::FlexAlignmentV1 => {
            compute_flex_alignment_cascade(view, author_stylesheets, viewport, style_revision)?
        }
        ComputedStyleProfile::FlexAlignmentCascadeLayersV1 => {
            compute_flex_alignment_layers_cascade(
                view,
                author_stylesheets,
                viewport,
                style_revision,
            )?
        }
        ComputedStyleProfile::S04FlexPaintV1 => {
            compute_s04_flex_paint_cascade(view, author_stylesheets, viewport, style_revision)?
        }
        ComputedStyleProfile::RuntimeFlexPaintV1 => {
            compute_runtime_flex_paint_cascade(view, viewport, style_revision)?
        }
        ComputedStyleProfile::BasicCascadeV1 | ComputedStyleProfile::FlexMediaEnvironmentV1 => {
            return Err(StyleLayoutError::UnsupportedProfile {
                profile: format!("{profile:?}"),
            });
        }
        ComputedStyleProfile::SupportedElementsUaV1
        | ComputedStyleProfile::RuntimeFlexLayoutV1
        | ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
        | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
        | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
        | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
        | ComputedStyleProfile::RuntimeBlockPaintV1
        | ComputedStyleProfile::RuntimeBlockFormattingV1
        | ComputedStyleProfile::RuntimeBlockPositioningV1 => {
            return Err(StyleLayoutError::UnsupportedProfile {
                profile: format!("{profile:?}"),
            });
        }
    };
    compute_layout_from_styles(snapshot, root, computed_styles, viewport, true)
}

fn compute_layout_from_styles(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    reject_cascade_diagnostics: bool,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    assert_matching_revision(snapshot, &computed_styles, computed_styles.profile)?;
    if reject_cascade_diagnostics
        && let Some(diagnostic) = computed_styles.diagnostics.first().cloned()
    {
        return Err(StyleLayoutError::CascadeDiagnostic(diagnostic));
    }
    if matches!(
        computed_styles.profile,
        ComputedStyleProfile::RuntimeBlockPaintV1
            | ComputedStyleProfile::RuntimeBlockFormattingV1
            | ComputedStyleProfile::RuntimeBlockPositioningV1
    ) {
        for element in computed_styles.elements.iter() {
            let display = element.properties.get("display").ok_or(
                StyleLayoutError::MissingComputedProperty {
                    node: element.node_id,
                    property: "display",
                },
            )?;
            let supported_display = match computed_styles.profile {
                ComputedStyleProfile::RuntimeBlockFormattingV1
                | ComputedStyleProfile::RuntimeBlockPositioningV1 => {
                    matches!(display.as_str(), "block" | "flow-root" | "none")
                }
                _ => matches!(display.as_str(), "block" | "none"),
            };
            if !supported_display {
                return Err(StyleLayoutError::UnsupportedBlockDisplay {
                    node: element.node_id,
                    value: display.clone(),
                });
            }
        }
    }
    let projected = project_styles(snapshot, root, &computed_styles)?;
    let styles = projected.styles;
    let positioning = projected.positioning;
    if matches!(
        computed_styles.profile,
        ComputedStyleProfile::FlexMarginV1
            | ComputedStyleProfile::RuntimeFlexLayoutV1
            | ComputedStyleProfile::RuntimeFlexPaintV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
            | ComputedStyleProfile::RuntimeBlockPaintV1
    ) && styles
        .get(&root.id())
        .is_some_and(|style| style.margin != LayoutEdges::default())
    {
        return Err(StyleLayoutError::UnsupportedRootMargin(root.id()));
    }
    let layout_viewport = Viewport {
        width: viewport.width_css_px,
        height: viewport.height_css_px,
    };
    let input = if matches!(
        computed_styles.profile,
        ComputedStyleProfile::RuntimeBlockFormattingV1
            | ComputedStyleProfile::RuntimeBlockPositioningV1
    ) {
        LayoutInput::from_host_document_with_block_formatting_viewport(
            snapshot,
            root,
            layout_viewport,
            &styles,
            computed_styles.style_revision,
            viewport.environment_revision,
        )?
    } else if matches!(
        computed_styles.profile,
        ComputedStyleProfile::RuntimeFlexLayoutV1
            | ComputedStyleProfile::RuntimeFlexPaintV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
            | ComputedStyleProfile::RuntimeBlockPaintV1
            | ComputedStyleProfile::RuntimeBlockFormattingV1
            | ComputedStyleProfile::RuntimeBlockPositioningV1
    ) {
        LayoutInput::from_host_document_with_viewport_containing_block(
            snapshot,
            root,
            layout_viewport,
            &styles,
            computed_styles.style_revision,
            viewport.environment_revision,
        )?
    } else {
        LayoutInput::from_host_document(
            snapshot,
            root,
            layout_viewport,
            &styles,
            computed_styles.style_revision,
            viewport.environment_revision,
        )?
    }
    .with_css_math(projected.css_math)
    .with_positioning(positioning);
    let mut layout = TaffyLayoutEngine.compute(&input)?;
    if matches!(
        computed_styles.profile,
        ComputedStyleProfile::RuntimeFlexLayoutV1
            | ComputedStyleProfile::RuntimeFlexPaintV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
            | ComputedStyleProfile::RuntimeBlockPaintV1
            | ComputedStyleProfile::RuntimeBlockFormattingV1
            | ComputedStyleProfile::RuntimeBlockPositioningV1
    ) {
        zero_display_none_frames(snapshot, root, &styles, &mut layout);
    }
    Ok(StyleLayoutOutput {
        computed_styles,
        layout,
    })
}

fn zero_display_none_frames(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &BTreeMap<NodeId, LayoutStyle>,
    layout: &mut LayoutOutput,
) {
    let mut pending = vec![(root, false)];
    while let Some((handle, ancestor_hidden)) = pending.pop() {
        let hidden = ancestor_hidden
            || styles
                .get(&handle.id())
                .is_some_and(|style| style.display == LayoutDisplay::None);
        if hidden && let Some(frame) = layout.frames.get_mut(&handle.id()) {
            *frame = LayoutFrame {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            };
        }
        if hidden && let Some(frame) = layout.flow_frames.get_mut(&handle.id()) {
            *frame = LayoutFrame {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            };
        }
        if let Some(children) = snapshot.children(handle) {
            pending.extend(children.map(|child| (child, hidden)));
        }
    }
}

#[cfg(test)]
mod ua_profile_boundary_tests {
    use spinon_core::{
        DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId, StyleRevision,
    };
    use spinon_style::{ComputedStyleProfile, CssViewport, StyloDocumentView};
    use style::context::QuirksMode;

    use super::compute_profile_layout;
    use crate::StyleLayoutError;

    #[test]
    fn ua_snapshot_profile_is_rejected_by_the_flex_layout_adapter() {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(860).unwrap(), document.document_revision());
        batch.push(DocumentOperation::CreateElement {
            node: root,
            namespace: "http://www.w3.org/1999/xhtml".to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        });
        document.commit(batch).unwrap();

        let snapshot = document.snapshot();
        let view =
            StyloDocumentView::new(snapshot.clone(), root, true, QuirksMode::NoQuirks).unwrap();
        let error = compute_profile_layout(
            &snapshot,
            &view,
            root,
            &[],
            CssViewport::C04_FIXTURE,
            StyleRevision::default(),
            ComputedStyleProfile::SupportedElementsUaV1,
        )
        .unwrap_err();

        assert!(matches!(
            error,
            StyleLayoutError::UnsupportedProfile { profile }
                if profile == "SupportedElementsUaV1"
        ));
    }
}
