use std::collections::{BTreeMap, BTreeSet};

mod style_values;
mod typed_math;
use style_values::project_styles;

use spinon_core::{
    HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, NodeId, StyleRevision,
};
use spinon_layout::{
    LayoutDisplay, LayoutEdges, LayoutEngine, LayoutError, LayoutFrame, LayoutInput,
    LayoutIntrinsicSizeSnapshot, LayoutOutput, LayoutSourceRevision, LayoutStyle,
    TaffyLayoutEngine, Viewport,
};
use spinon_style::{
    ComputedStyleProfile, ComputedStyleSnapshot, CssViewport, StylesheetSource, StyloDocumentView,
    compute_flex_alignment_cascade, compute_flex_alignment_layers_cascade,
    compute_flex_layout_cascade, compute_flex_margin_cascade,
    compute_runtime_block_flex_grid_auto_placement_cascade_with_stylesheets,
    compute_runtime_block_flex_grid_cascade_with_stylesheets,
    compute_runtime_block_flex_grid_dense_auto_placement_cascade_with_stylesheets,
    compute_runtime_block_flex_grid_placement_cascade_with_stylesheets,
    compute_runtime_block_flex_grid_template_cascade_with_stylesheets,
    compute_runtime_flex_paint_cascade, compute_s04_flex_paint_cascade,
    first_unsupported_runtime_block_flex_grid_auto_placement_inline_property,
    first_unsupported_runtime_block_flex_grid_dense_auto_placement_inline_property,
    first_unsupported_runtime_block_flex_grid_inline_property,
    first_unsupported_runtime_block_flex_grid_placement_inline_property,
    first_unsupported_runtime_block_flex_grid_template_inline_property,
};

use crate::StyleLayoutError;
use validation::{assert_matching_revision, assert_no_inline_style, assert_view_matches_snapshot};

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

mod fixed_containing_block;
mod validation;

/// 같은 document snapshot에서 계산한 제한 CSS style과 Taffy layout 결과입니다.
#[derive(Clone, Debug)]
pub struct StyleLayoutOutput {
    pub computed_styles: ComputedStyleSnapshot,
    pub layout: LayoutOutput,
}

/// C11.1 inline CSS preflight, Stylo cascade, typed Grid projection과 Taffy layout을 연결합니다.
pub fn compute_runtime_block_flex_grid_style_layout(
    snapshot: &HostDocumentSnapshot,
    view: &StyloDocumentView,
    root: HostNodeHandle,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    assert_view_matches_snapshot(snapshot, view)?;
    if view.mount_root_element().handle() != Some(root) {
        return Err(StyleLayoutError::SnapshotMismatch { field: "root" });
    }
    if let Some((node, property)) = first_unsupported_runtime_block_flex_grid_inline_property(view)
    {
        return Err(StyleLayoutError::UnsupportedInlineProperty { node, property });
    }
    let computed_styles = compute_runtime_block_flex_grid_cascade_with_stylesheets(
        view,
        author_stylesheets,
        viewport,
        style_revision,
    )?;
    compute_runtime_style_layout(snapshot, root, computed_styles, viewport)
}

/// C11.2.1 numeric Grid placement profile을 cascade부터 Taffy까지 연결합니다.
pub fn compute_runtime_block_flex_grid_placement_style_layout(
    snapshot: &HostDocumentSnapshot,
    view: &StyloDocumentView,
    root: HostNodeHandle,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    assert_view_matches_snapshot(snapshot, view)?;
    if view.mount_root_element().handle() != Some(root) {
        return Err(StyleLayoutError::SnapshotMismatch { field: "root" });
    }
    if let Some((node, property)) =
        first_unsupported_runtime_block_flex_grid_placement_inline_property(view)
    {
        return Err(StyleLayoutError::UnsupportedInlineProperty { node, property });
    }
    let computed_styles = compute_runtime_block_flex_grid_placement_cascade_with_stylesheets(
        view,
        author_stylesheets,
        viewport,
        style_revision,
    )?;
    compute_runtime_style_layout(snapshot, root, computed_styles, viewport)
}

/// C11.2.2 sparse row auto-placement profile을 cascade부터 Taffy까지 연결합니다.
pub fn compute_runtime_block_flex_grid_auto_placement_style_layout(
    snapshot: &HostDocumentSnapshot,
    view: &StyloDocumentView,
    root: HostNodeHandle,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    assert_view_matches_snapshot(snapshot, view)?;
    if view.mount_root_element().handle() != Some(root) {
        return Err(StyleLayoutError::SnapshotMismatch { field: "root" });
    }
    if let Some((node, property)) =
        first_unsupported_runtime_block_flex_grid_auto_placement_inline_property(view)
    {
        return Err(StyleLayoutError::UnsupportedInlineProperty { node, property });
    }
    let computed_styles = compute_runtime_block_flex_grid_auto_placement_cascade_with_stylesheets(
        view,
        author_stylesheets,
        viewport,
        style_revision,
    )?;
    compute_runtime_style_layout(snapshot, root, computed_styles, viewport)
}

/// C11.2.3 dense·column 배치 profile을 cascade부터 Taffy까지 연결합니다.
pub fn compute_runtime_block_flex_grid_dense_auto_placement_style_layout(
    snapshot: &HostDocumentSnapshot,
    view: &StyloDocumentView,
    root: HostNodeHandle,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    assert_view_matches_snapshot(snapshot, view)?;
    if view.mount_root_element().handle() != Some(root) {
        return Err(StyleLayoutError::SnapshotMismatch { field: "root" });
    }
    if let Some((node, property)) =
        first_unsupported_runtime_block_flex_grid_dense_auto_placement_inline_property(view)
    {
        return Err(StyleLayoutError::UnsupportedInlineProperty { node, property });
    }
    let computed_styles =
        compute_runtime_block_flex_grid_dense_auto_placement_cascade_with_stylesheets(
            view,
            author_stylesheets,
            viewport,
            style_revision,
        )?;
    compute_runtime_style_layout(snapshot, root, computed_styles, viewport)
}

/// C11.3.1 정수 `repeat()` profile을 cascade부터 Taffy까지 연결합니다.
pub fn compute_runtime_block_flex_grid_template_style_layout(
    snapshot: &HostDocumentSnapshot,
    view: &StyloDocumentView,
    root: HostNodeHandle,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    assert_view_matches_snapshot(snapshot, view)?;
    if view.mount_root_element().handle() != Some(root) {
        return Err(StyleLayoutError::SnapshotMismatch { field: "root" });
    }
    if let Some((node, property)) =
        first_unsupported_runtime_block_flex_grid_template_inline_property(view)
    {
        return Err(StyleLayoutError::UnsupportedInlineProperty { node, property });
    }
    let computed_styles = compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
        view,
        author_stylesheets,
        viewport,
        style_revision,
    )?;
    compute_runtime_style_layout(snapshot, root, computed_styles, viewport)
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
            | ComputedStyleProfile::RuntimeBlockFlexStackingV1
            | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
            | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
            | ComputedStyleProfile::RuntimeBlockFlexGridV1
            | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
    );
    compute_layout_from_styles(
        snapshot,
        root,
        computed_styles,
        viewport,
        reject_cascade_diagnostics,
        None,
    )
}

/// revision-stamped HTML image natural size를 runtime style/layout 계산에 연결합니다.
pub fn compute_runtime_style_layout_with_intrinsic_size_snapshot(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    intrinsic_sizes: LayoutIntrinsicSizeSnapshot,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    let reject_cascade_diagnostics = matches!(
        computed_styles.profile,
        ComputedStyleProfile::RuntimeBlockPositioningV1
            | ComputedStyleProfile::RuntimeBlockFlexStackingV1
            | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
            | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
            | ComputedStyleProfile::RuntimeBlockFlexGridV1
            | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
    );
    compute_layout_from_styles(
        snapshot,
        root,
        computed_styles,
        viewport,
        reject_cascade_diagnostics,
        Some(intrinsic_sizes),
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
        | ComputedStyleProfile::RuntimeBlockPositioningV1
        | ComputedStyleProfile::RuntimeBlockFlexStackingV1
        | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
        | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
        | ComputedStyleProfile::RuntimeBlockFlexGridV1
        | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
        | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
        | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
        | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1 => {
            return Err(StyleLayoutError::UnsupportedProfile {
                profile: format!("{profile:?}"),
            });
        }
    };
    compute_layout_from_styles(snapshot, root, computed_styles, viewport, true, None)
}

fn compute_layout_from_styles(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    reject_cascade_diagnostics: bool,
    intrinsic_sizes: Option<LayoutIntrinsicSizeSnapshot>,
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
            | ComputedStyleProfile::RuntimeBlockFlexStackingV1
            | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
            | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
            | ComputedStyleProfile::RuntimeBlockFlexGridV1
            | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
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
                ComputedStyleProfile::RuntimeBlockFlexStackingV1 => {
                    matches!(display.as_str(), "block" | "flow-root" | "flex" | "none")
                }
                ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
                | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1 => {
                    matches!(display.as_str(), "block" | "flow-root" | "flex" | "none")
                }
                ComputedStyleProfile::RuntimeBlockFlexGridV1
                | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
                | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
                | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
                | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1 => {
                    matches!(
                        display.as_str(),
                        "block" | "flow-root" | "flex" | "grid" | "none"
                    )
                }
                _ => matches!(display.as_str(), "block" | "none"),
            };
            if !supported_display {
                return Err(
                    if matches!(
                        computed_styles.profile,
                        ComputedStyleProfile::RuntimeBlockFlexStackingV1
                            | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
                            | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
                            | ComputedStyleProfile::RuntimeBlockFlexGridV1
                            | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
                            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
                            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
                            | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
                    ) {
                        StyleLayoutError::UnsupportedComputedValue {
                            node: element.node_id,
                            property: "display",
                            value: display.clone(),
                        }
                    } else {
                        StyleLayoutError::UnsupportedBlockDisplay {
                            node: element.node_id,
                            value: display.clone(),
                        }
                    },
                );
            }
        }
    }
    let mut projected = project_styles(snapshot, root, &computed_styles)?;
    validate_intrinsic_image_inputs(
        snapshot,
        root,
        &mut projected.styles,
        intrinsic_sizes.as_ref(),
    )?;
    let styles = projected.styles;
    let positioning = projected.positioning;
    let grid_containers = projected.grid_containers;
    let grid_items = projected.grid_items;
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
            | ComputedStyleProfile::RuntimeBlockFlexStackingV1
            | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
            | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
            | ComputedStyleProfile::RuntimeBlockFlexGridV1
            | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
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
    .with_positioning(positioning)
    .with_grid_layout(grid_containers, grid_items);
    let mut layout = if let Some(intrinsic_sizes) = intrinsic_sizes {
        TaffyLayoutEngine.compute_with_intrinsic_size_snapshot(&input, &intrinsic_sizes)?
    } else {
        TaffyLayoutEngine.compute(&input)?
    };
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
            | ComputedStyleProfile::RuntimeBlockFlexStackingV1
            | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
            | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
            | ComputedStyleProfile::RuntimeBlockFlexGridV1
            | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
    ) {
        zero_display_none_frames(snapshot, root, &styles, &mut layout);
    }
    Ok(StyleLayoutOutput {
        computed_styles,
        layout,
    })
}

fn validate_intrinsic_image_inputs(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &mut BTreeMap<NodeId, LayoutStyle>,
    intrinsic_sizes: Option<&LayoutIntrinsicSizeSnapshot>,
) -> Result<(), StyleLayoutError> {
    let expected_source = LayoutSourceRevision::HostDocument {
        generation: snapshot.generation(),
        document: snapshot.document_revision(),
        render_tree: snapshot.render_tree_revision(),
    };
    if let Some(intrinsic_sizes) = intrinsic_sizes
        && intrinsic_sizes.source() != expected_source
    {
        return Err(spinon_layout::LayoutError::IntrinsicSizeSourceMismatch {
            expected: expected_source,
            actual: intrinsic_sizes.source(),
        }
        .into());
    }
    let mut layout_nodes = BTreeSet::new();
    let mut images = BTreeSet::new();
    let mut required_images = BTreeSet::new();
    let mut pending = vec![(root, false)];
    while let Some((handle, ancestor_hidden)) = pending.pop() {
        let node = snapshot
            .node(handle)
            .ok_or(StyleLayoutError::SnapshotMismatch { field: "HostNode" })?;
        if matches!(node.kind(), HostNodeKind::Text(_)) {
            continue;
        }
        layout_nodes.insert(node.id());
        let Some(style) = styles.get(&node.id()).copied() else {
            return Err(LayoutError::MissingStyle(node.id()).into());
        };
        let hidden = ancestor_hidden || style.display == LayoutDisplay::None;
        let is_html_image = matches!(node.kind(), HostNodeKind::Element(element)
            if element.namespace() == HTML_NAMESPACE && element.local_name() == "img");
        let has_intrinsic_keyword = matches!(
            style.width,
            spinon_layout::LayoutDimension::MinContent
                | spinon_layout::LayoutDimension::MaxContent
                | spinon_layout::LayoutDimension::FitContent
        ) || matches!(
            style.height,
            spinon_layout::LayoutDimension::MinContent
                | spinon_layout::LayoutDimension::MaxContent
                | spinon_layout::LayoutDimension::FitContent
        );
        if has_intrinsic_keyword && !is_html_image {
            let supported_image = if style.width == spinon_layout::LayoutDimension::MinContent {
                is_supported_cyclic_min_content_container(snapshot, handle, style, styles)
            } else {
                None
            };
            let Some((image, fraction)) = supported_image else {
                return Err(LayoutError::UnsupportedIntrinsicSizing {
                    node: node.id(),
                    reason: "C14.3 intrinsic sizing keyword는 HTML img와 제한된 cyclic parent만 지원합니다",
                }
                .into());
            };
            let image_style = styles
                .get_mut(&image)
                .ok_or(LayoutError::MissingStyle(image))?;
            image_style.width = spinon_layout::LayoutDimension::CyclicMinContentPercent(fraction);
        }
        if is_html_image {
            images.insert(node.id());
            let requires_intrinsic_size = has_intrinsic_keyword
                || style.width == spinon_layout::LayoutDimension::Auto
                || style.height == spinon_layout::LayoutDimension::Auto;
            if !hidden
                && requires_intrinsic_size
                && snapshot
                    .children(handle)
                    .is_some_and(|mut children| children.next().is_some())
            {
                return Err(LayoutError::UnsupportedIntrinsicSizing {
                    node: node.id(),
                    reason: "C14.3 HTML img intrinsic sizing은 자식 없는 leaf만 지원합니다",
                }
                .into());
            }
            if !hidden && requires_intrinsic_size {
                required_images.insert(node.id());
            }
        }
        if let Some(children) = snapshot.children(handle) {
            pending.extend(
                children
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .map(|child| (child, hidden)),
            );
        }
    }
    if let Some(intrinsic_sizes) = intrinsic_sizes {
        for node in intrinsic_sizes.sizes().keys() {
            if !layout_nodes.contains(node) {
                return Err(LayoutError::UnknownIntrinsicSizeNode(*node).into());
            }
            if !images.contains(node) {
                return Err(LayoutError::UnexpectedIntrinsicSizeNode(*node).into());
            }
        }
    }
    for node in required_images {
        if !intrinsic_sizes.is_some_and(|input| input.sizes().contains_key(&node)) {
            return Err(LayoutError::MissingIntrinsicSize(node).into());
        }
    }
    Ok(())
}

fn is_supported_cyclic_min_content_container(
    snapshot: &HostDocumentSnapshot,
    handle: HostNodeHandle,
    style: LayoutStyle,
    styles: &BTreeMap<NodeId, LayoutStyle>,
) -> Option<(NodeId, f32)> {
    if style.display != LayoutDisplay::Block
        || style.width != spinon_layout::LayoutDimension::MinContent
        || style.height != spinon_layout::LayoutDimension::Auto
        || style.min_width != spinon_layout::LayoutDimension::Auto
        || style.max_width != spinon_layout::LayoutDimension::Auto
        || style.min_height != spinon_layout::LayoutDimension::Auto
        || style.max_height != spinon_layout::LayoutDimension::Auto
        || style.aspect_ratio.is_some()
        || style.padding.top != spinon_layout::LayoutLengthPercentage::ZERO
        || style.padding.right != spinon_layout::LayoutLengthPercentage::ZERO
        || style.padding.bottom != spinon_layout::LayoutLengthPercentage::ZERO
        || style.padding.left != spinon_layout::LayoutLengthPercentage::ZERO
        || style.border.top != 0.0
        || style.border.right != 0.0
        || style.border.bottom != 0.0
        || style.border.left != 0.0
    {
        return None;
    }
    if !has_only_block_ancestors(snapshot, handle, styles) {
        return None;
    }
    let children = snapshot.children(handle)?;
    let mut children = children.collect::<Vec<_>>().into_iter();
    let child = children.next()?;
    if children.next().is_some() {
        return None;
    }
    let node = snapshot.node(child)?;
    let is_html_image = matches!(node.kind(), HostNodeKind::Element(element)
        if element.namespace() == HTML_NAMESPACE && element.local_name() == "img");
    if !is_html_image {
        return None;
    }
    let image = styles.get(&node.id())?;
    let spinon_layout::LayoutDimension::Percent(fraction) = image.width else {
        return None;
    };
    (fraction.is_finite()
        && fraction >= 0.0
        && image.height == spinon_layout::LayoutDimension::Auto)
        .then_some((node.id(), fraction))
}

fn has_only_block_ancestors(
    snapshot: &HostDocumentSnapshot,
    handle: HostNodeHandle,
    styles: &BTreeMap<NodeId, LayoutStyle>,
) -> bool {
    let mut parent = snapshot.parent(handle);
    while let Some(HostParent::Node(ancestor)) = parent {
        let Some(style) = styles.get(&ancestor.id()) else {
            return false;
        };
        if style.display != LayoutDisplay::Block {
            return false;
        }
        parent = snapshot.parent(ancestor);
    }
    matches!(parent, Some(HostParent::Root))
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
