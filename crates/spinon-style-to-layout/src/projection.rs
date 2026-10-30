use std::collections::{BTreeMap, BTreeSet};

mod style_values;
mod typed_math;
use style_values::project_styles;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
use spinon_core::TextOrientation;
use spinon_core::{
    HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, NodeId, StyleRevision,
    WritingMode,
};
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
use spinon_layout::GridContainerInput;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
use spinon_layout::LineBreakMode;
use spinon_layout::{
    LayoutDisplay, LayoutEdges, LayoutEngine, LayoutError, LayoutFrame, LayoutInput,
    LayoutIntrinsicSizeSnapshot, LayoutOutput, LayoutSourceRevision, LayoutStyle, ParagraphStyle,
    TaffyLayoutEngine, TextLayoutEngine, Viewport,
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
    compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets,
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

type TextProjectionProfile<'a> = (
    &'a BTreeMap<NodeId, ParagraphStyle>,
    &'a BTreeMap<NodeId, spinon_layout::InlineRunStyle>,
    &'a TextLayoutEngine,
);

mod fixed_containing_block;
mod validation;

/// 같은 document snapshot에서 계산한 제한 CSS style과 Taffy layout 결과입니다.
#[derive(Clone, Debug)]
pub struct StyleLayoutOutput {
    pub computed_styles: ComputedStyleSnapshot,
    pub layout: LayoutOutput,
    /// C11.4 fixture에서 Taffy에 전달한 실제 typed Grid container 입력입니다.
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    pub grid_container_inputs: BTreeMap<NodeId, GridContainerInput>,
    /// C11.4 fixture에서 calc ID와 node/property가 결합된 실제 layout registry입니다.
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    pub css_math: Vec<spinon_layout::LayoutCssMathValue>,
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
    compute_runtime_block_flex_grid_template_style_layout_with_options(
        snapshot,
        view,
        root,
        author_stylesheets,
        viewport,
        style_revision,
        false,
    )
}

/// C11.3.3에서만 trailing implicit named line 생성을 opt-in 합니다.
pub fn compute_runtime_block_flex_grid_template_implicit_named_lines_style_layout(
    snapshot: &HostDocumentSnapshot,
    view: &StyloDocumentView,
    root: HostNodeHandle,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    compute_runtime_block_flex_grid_template_style_layout_with_options(
        snapshot,
        view,
        root,
        author_stylesheets,
        viewport,
        style_revision,
        true,
    )
}

fn compute_runtime_block_flex_grid_template_style_layout_with_options(
    snapshot: &HostDocumentSnapshot,
    view: &StyloDocumentView,
    root: HostNodeHandle,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
    allow_implicit_named_lines: bool,
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
    compute_runtime_style_layout_with_implicit_named_lines(
        snapshot,
        root,
        computed_styles,
        viewport,
        allow_implicit_named_lines,
    )
}

/// C11.4 intrinsic Grid track sizing profile을 cascade부터 Taffy까지 연결합니다.
pub fn compute_runtime_block_flex_grid_track_sizing_style_layout(
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
    let computed_styles =
        match compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            view,
            author_stylesheets,
            viewport,
            style_revision,
        ) {
            Ok(snapshot) => snapshot,
            Err(spinon_style::CssCascadeError::UnsupportedInlineAuthorProperty {
                node,
                property,
            }) => return Err(StyleLayoutError::UnsupportedInlineProperty { node, property }),
            Err(error) => return Err(StyleLayoutError::Cascade(error)),
        };
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
    compute_runtime_style_layout_with_implicit_named_lines(
        snapshot,
        root,
        computed_styles,
        viewport,
        false,
    )
}

/// 동일한 computed-style profile에서 C11.3.3 trailing implicit named line subset을 opt-in 합니다.
pub fn compute_runtime_style_layout_with_implicit_named_lines(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    allow_implicit_named_lines: bool,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    if allow_implicit_named_lines
        && computed_styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
    {
        return Err(StyleLayoutError::UnsupportedProfile {
            profile: format!("{:?}", computed_styles.profile),
        });
    }
    let reject_cascade_diagnostics = matches!(
        computed_styles.profile,
        ComputedStyleProfile::RuntimeBlockPositioningV1
            | ComputedStyleProfile::RuntimeBlockFlexStackingV1
            | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
            | ComputedStyleProfile::C19BoxPaintOpacityV1
            | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
            | ComputedStyleProfile::RuntimeBlockFlexGridV1
            | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
    );
    compute_layout_from_styles_with_grid_options(
        snapshot,
        root,
        computed_styles,
        viewport,
        reject_cascade_diagnostics,
        None,
        allow_implicit_named_lines,
    )
}

/// C04.12 고정 Ahem profile의 paragraph를 같은 HostDocument·computed-style snapshot에서 계산합니다.
#[cfg(feature = "c04-12-root-html-text-paint-fixture-test")]
pub fn compute_runtime_style_layout_with_text_paragraphs(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    paragraph_styles: &BTreeMap<NodeId, ParagraphStyle>,
    text_engine: &TextLayoutEngine,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    let inline_run_styles = BTreeMap::new();
    compute_runtime_style_layout_with_text_profiles(
        snapshot,
        root,
        computed_styles,
        viewport,
        paragraph_styles,
        &inline_run_styles,
        text_engine,
    )
}

/// Root HTML computed style의 block·inline font을 같은 HostDocument의 glyph layout에 연결합니다.
pub fn compute_runtime_style_layout_with_text_profiles(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    paragraph_styles: &BTreeMap<NodeId, ParagraphStyle>,
    inline_run_styles: &BTreeMap<NodeId, spinon_layout::InlineRunStyle>,
    text_engine: &TextLayoutEngine,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    if computed_styles.profile != ComputedStyleProfile::RuntimeRootHtmlTextPaintV1 {
        return Err(StyleLayoutError::UnsupportedProfile {
            profile: format!("{:?}", computed_styles.profile),
        });
    }
    if computed_styles.viewport != viewport {
        return Err(StyleLayoutError::SnapshotMismatch { field: "viewport" });
    }
    compute_layout_from_styles_with_text(
        snapshot,
        root,
        computed_styles,
        viewport,
        true,
        None,
        Some((paragraph_styles, inline_run_styles, text_engine)),
    )
}

/// C11.4 fixture에서 direct-leaf text를 Ahem 기반 Grid intrinsic sizing에 연결합니다.
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub fn compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    paragraph_styles: &BTreeMap<NodeId, ParagraphStyle>,
    text_engine: &TextLayoutEngine,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs_and_options(
        snapshot,
        root,
        computed_styles,
        viewport,
        paragraph_styles,
        text_engine,
        RuntimeGridProjectionOptions::default(),
    )
}

/// `auto <ratio>`의 content-box 효과를 하나의 C11.4 direct-text owner에만 허용합니다.
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub fn compute_runtime_block_flex_grid_track_sizing_style_layout_with_content_box_ratio_text(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    paragraph_styles: &BTreeMap<NodeId, ParagraphStyle>,
    text_engine: &TextLayoutEngine,
    content_box_ratio_node: NodeId,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs_and_options(
        snapshot,
        root,
        computed_styles,
        viewport,
        paragraph_styles,
        text_engine,
        RuntimeGridProjectionOptions {
            content_box_ratio_node: Some(content_box_ratio_node),
            ..RuntimeGridProjectionOptions::default()
        },
    )
}

/// C11.4 direct-text min/max constraints are admitted for one exact ratio item owner.
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub fn compute_runtime_block_flex_grid_track_sizing_style_layout_with_aspect_ratio_minmax_text(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    paragraph_styles: &BTreeMap<NodeId, ParagraphStyle>,
    text_engine: &TextLayoutEngine,
    aspect_ratio_minmax_node: NodeId,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs_and_options(
        snapshot,
        root,
        computed_styles,
        viewport,
        paragraph_styles,
        text_engine,
        RuntimeGridProjectionOptions {
            content_box_ratio_node: None,
            aspect_ratio_minmax_node: Some(aspect_ratio_minmax_node),
            ..RuntimeGridProjectionOptions::default()
        },
    )
}

/// C11.4 fixed direct-text baseline fixture에만 Grid first/last-baseline 정렬을 허용합니다.
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub fn compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_baseline_fixture(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    paragraph_styles: &BTreeMap<NodeId, ParagraphStyle>,
    text_engine: &TextLayoutEngine,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs_and_options(
        snapshot,
        root,
        computed_styles,
        viewport,
        paragraph_styles,
        text_engine,
        RuntimeGridProjectionOptions {
            allow_c11_4_grid_text_baseline_fixture: true,
            ..RuntimeGridProjectionOptions::default()
        },
    )
}

/// C11.4 고정 nested Grid fixture에서 양 축 intrinsic sizing을 opt-in 합니다.
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub fn compute_runtime_block_flex_grid_track_sizing_style_layout_with_two_axis_nested_grid_text_paragraphs(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    paragraph_styles: &BTreeMap<NodeId, ParagraphStyle>,
    text_engine: &TextLayoutEngine,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs_and_options(
        snapshot,
        root,
        computed_styles,
        viewport,
        paragraph_styles,
        text_engine,
        RuntimeGridProjectionOptions {
            allow_c11_4_nested_grid_two_axis_intrinsics: true,
            ..RuntimeGridProjectionOptions::default()
        },
    )
}

/// C11.4 fixed fixture에서만 item overflow를 intrinsic automatic minimum으로 전달합니다.
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub fn compute_runtime_block_flex_grid_track_sizing_style_layout_with_overflow_automatic_minimum_fixture(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    paragraph_styles: &BTreeMap<NodeId, ParagraphStyle>,
    text_engine: &TextLayoutEngine,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs_and_options(
        snapshot,
        root,
        computed_styles,
        viewport,
        paragraph_styles,
        text_engine,
        RuntimeGridProjectionOptions {
            allow_c11_4_overflow_auto_minimum_fixture: true,
            ..RuntimeGridProjectionOptions::default()
        },
    )
}

/// C11.4 overflow fixture에서 text 입력과 replaced intrinsic snapshot을 함께 계산합니다.
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub fn compute_runtime_block_flex_grid_track_sizing_style_layout_with_overflow_automatic_minimum_fixture_and_intrinsic_sizes(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    paragraph_styles: &BTreeMap<NodeId, ParagraphStyle>,
    text_engine: &TextLayoutEngine,
    intrinsic_sizes: LayoutIntrinsicSizeSnapshot,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    if computed_styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 {
        return Err(StyleLayoutError::UnsupportedProfile {
            profile: format!("{:?}", computed_styles.profile),
        });
    }
    if computed_styles.viewport != viewport {
        return Err(StyleLayoutError::SnapshotMismatch { field: "viewport" });
    }
    let inline_run_styles = BTreeMap::new();
    compute_layout_from_styles_with_text_and_grid_options(
        snapshot,
        root,
        computed_styles,
        viewport,
        Some(intrinsic_sizes),
        Some((paragraph_styles, &inline_run_styles, text_engine)),
        StyleLayoutProjectionOptions {
            reject_cascade_diagnostics: true,
            allow_implicit_named_lines: false,
            allow_c11_4_anonymous_grid_text_items: true,
            blockify_c11_4_anonymous_grid_images: false,
            allow_c11_4_nested_grid_two_axis_intrinsics: true,
            allow_c11_4_replaced_grid_item_align_self_start: true,
            allow_c11_4_overflow_auto_minimum_fixture: true,
            allow_c11_4_grid_text_baseline_fixture: false,
            content_box_ratio_node: None,
            aspect_ratio_minmax_node: None,
        },
    )
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[derive(Clone, Copy, Debug, Default)]
struct RuntimeGridProjectionOptions {
    allow_c11_4_nested_grid_two_axis_intrinsics: bool,
    allow_c11_4_overflow_auto_minimum_fixture: bool,
    allow_c11_4_grid_text_baseline_fixture: bool,
    content_box_ratio_node: Option<NodeId>,
    aspect_ratio_minmax_node: Option<NodeId>,
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs_and_options(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    paragraph_styles: &BTreeMap<NodeId, ParagraphStyle>,
    text_engine: &TextLayoutEngine,
    options: RuntimeGridProjectionOptions,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    if computed_styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 {
        return Err(StyleLayoutError::UnsupportedProfile {
            profile: format!("{:?}", computed_styles.profile),
        });
    }
    if computed_styles.viewport != viewport {
        return Err(StyleLayoutError::SnapshotMismatch { field: "viewport" });
    }
    let inline_run_styles = BTreeMap::new();
    compute_layout_from_styles_with_text_and_grid_options(
        snapshot,
        root,
        computed_styles,
        viewport,
        None,
        Some((paragraph_styles, &inline_run_styles, text_engine)),
        StyleLayoutProjectionOptions {
            reject_cascade_diagnostics: true,
            allow_implicit_named_lines: false,
            allow_c11_4_anonymous_grid_text_items: true,
            blockify_c11_4_anonymous_grid_images: false,
            allow_c11_4_nested_grid_two_axis_intrinsics: options
                .allow_c11_4_nested_grid_two_axis_intrinsics,
            allow_c11_4_replaced_grid_item_align_self_start: false,
            allow_c11_4_overflow_auto_minimum_fixture: options
                .allow_c11_4_overflow_auto_minimum_fixture,
            allow_c11_4_grid_text_baseline_fixture: options.allow_c11_4_grid_text_baseline_fixture,
            content_box_ratio_node: options.content_box_ratio_node,
            aspect_ratio_minmax_node: options.aspect_ratio_minmax_node,
        },
    )
}

fn validate_c11_4_writing_styles_without_text_profile(
    computed_styles: &ComputedStyleSnapshot,
) -> Result<(), StyleLayoutError> {
    for (node, writing_mode, _) in computed_styles.writing_styles.iter() {
        if *writing_mode == WritingMode::HorizontalTb {
            continue;
        }
        let property = "writing-mode";
        let value = computed_styles
            .elements
            .iter()
            .find(|element| element.node_id == *node)
            .and_then(|element| element.properties.get(property))
            .cloned()
            .unwrap_or_else(|| format!("{writing_mode:?}"));
        return Err(StyleLayoutError::UnsupportedComputedValue {
            node: *node,
            property,
            value,
        });
    }
    Ok(())
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn validate_c11_4_orthogonal_text_styles(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: &ComputedStyleSnapshot,
    paragraph_styles: &BTreeMap<NodeId, ParagraphStyle>,
) -> Result<(), StyleLayoutError> {
    let mut hidden_nodes = BTreeSet::new();
    let mut pending = vec![(root, false)];
    while let Some((handle, ancestor_hidden)) = pending.pop() {
        let node = snapshot
            .node(handle)
            .ok_or(StyleLayoutError::SnapshotMismatch { field: "HostNode" })?;
        let display_none = matches!(node.kind(), HostNodeKind::Element(_))
            && computed_styles
                .elements
                .iter()
                .find(|element| element.node_id == node.id())
                .and_then(|element| element.properties.get("display"))
                .is_some_and(|display| display == "none");
        let hidden = ancestor_hidden || display_none;
        if hidden {
            hidden_nodes.insert(node.id());
        }
        if let Some(children) = snapshot.children(handle) {
            pending.extend(children.map(|child| (child, hidden)));
        }
    }
    let vertical_paragraphs = paragraph_styles
        .iter()
        .filter(|(_, style)| style.writing_mode == WritingMode::VerticalRl)
        .collect::<Vec<_>>();
    let expected_vertical_nodes = vertical_paragraphs
        .iter()
        .map(|(node, _)| **node)
        .collect::<BTreeSet<_>>();
    let computed_vertical_nodes = computed_styles
        .writing_styles
        .iter()
        .filter_map(|(node, mode, _)| {
            (*mode == WritingMode::VerticalRl && !hidden_nodes.contains(node)).then_some(*node)
        })
        .collect::<BTreeSet<_>>();
    if expected_vertical_nodes.is_empty() && computed_vertical_nodes.is_empty() {
        return Ok(());
    }
    if expected_vertical_nodes != computed_vertical_nodes {
        let node = expected_vertical_nodes
            .symmetric_difference(&computed_vertical_nodes)
            .next()
            .copied()
            .unwrap_or(root.id());
        return Err(StyleLayoutError::UnsupportedOrthogonalTextProfile {
            node,
            reason: "computed vertical text nodes must match the paragraph inputs",
        });
    }
    if computed_styles.writing_style_for(root.id())
        != Some((WritingMode::HorizontalTb, TextOrientation::Mixed))
    {
        return Err(StyleLayoutError::UnsupportedOrthogonalTextProfile {
            node: root.id(),
            reason: "root computed writing-mode/text-orientation must be horizontal-tb/mixed",
        });
    }
    for (node, style) in vertical_paragraphs {
        if style.text_orientation != TextOrientation::Mixed
            || computed_styles.writing_style_for(*node)
                != Some((WritingMode::VerticalRl, TextOrientation::Mixed))
        {
            return Err(StyleLayoutError::UnsupportedOrthogonalTextProfile {
                node: *node,
                reason: "paragraph and computed writing-mode/text-orientation must both be vertical-rl/mixed",
            });
        }
        let computed_line_break = computed_styles
            .elements
            .iter()
            .find(|element| element.node_id == *node)
            .and_then(|element| element.properties.get("line-break"))
            .map(String::as_str);
        if style.line_break != LineBreakMode::Normal
            || !matches!(computed_line_break, Some("normal" | "auto"))
        {
            return Err(StyleLayoutError::UnsupportedOrthogonalTextProfile {
                node: *node,
                reason: "paragraph line-break must be normal and computed line-break must be the supported normal or legacy auto value",
            });
        }
    }
    Ok(())
}

/// C11.4 mixed fixture에서 Ahem paragraph와 replaced-resource snapshot을 함께 연결합니다.
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub fn compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_and_intrinsic_sizes(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    paragraph_styles: &BTreeMap<NodeId, ParagraphStyle>,
    text_engine: &TextLayoutEngine,
    intrinsic_sizes: LayoutIntrinsicSizeSnapshot,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    if computed_styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 {
        return Err(StyleLayoutError::UnsupportedProfile {
            profile: format!("{:?}", computed_styles.profile),
        });
    }
    if computed_styles.viewport != viewport {
        return Err(StyleLayoutError::SnapshotMismatch { field: "viewport" });
    }
    let inline_run_styles = BTreeMap::new();
    compute_layout_from_styles_with_text(
        snapshot,
        root,
        computed_styles,
        viewport,
        true,
        Some(intrinsic_sizes),
        Some((paragraph_styles, &inline_run_styles, text_engine)),
    )
}

/// C11.4 고정 direct-child Grid fixture에서 text/image와 양 축 intrinsic sizing을 함께 계산합니다.
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub fn compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_and_intrinsic_sizes_and_two_axis_grid_fixture(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    paragraph_styles: &BTreeMap<NodeId, ParagraphStyle>,
    text_engine: &TextLayoutEngine,
    intrinsic_sizes: LayoutIntrinsicSizeSnapshot,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    if computed_styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 {
        return Err(StyleLayoutError::UnsupportedProfile {
            profile: format!("{:?}", computed_styles.profile),
        });
    }
    if computed_styles.viewport != viewport {
        return Err(StyleLayoutError::SnapshotMismatch { field: "viewport" });
    }
    let inline_run_styles = BTreeMap::new();
    compute_layout_from_styles_with_text_and_grid_options(
        snapshot,
        root,
        computed_styles,
        viewport,
        Some(intrinsic_sizes),
        Some((paragraph_styles, &inline_run_styles, text_engine)),
        StyleLayoutProjectionOptions {
            reject_cascade_diagnostics: true,
            allow_implicit_named_lines: false,
            allow_c11_4_anonymous_grid_text_items: true,
            blockify_c11_4_anonymous_grid_images: false,
            allow_c11_4_nested_grid_two_axis_intrinsics: true,
            allow_c11_4_replaced_grid_item_align_self_start: true,
            allow_c11_4_overflow_auto_minimum_fixture: false,
            allow_c11_4_grid_text_baseline_fixture: false,
            content_box_ratio_node: None,
            aspect_ratio_minmax_node: None,
        },
    )
}

/// C11.4 opt-in fixture에서 anonymous Grid text와 replaced-resource snapshot을 함께 연결합니다.
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub fn compute_runtime_block_flex_grid_track_sizing_style_layout_with_anonymous_text_and_intrinsic_sizes(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    paragraph_styles: &BTreeMap<NodeId, ParagraphStyle>,
    text_engine: &TextLayoutEngine,
    intrinsic_sizes: LayoutIntrinsicSizeSnapshot,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    compute_runtime_block_flex_grid_track_sizing_style_layout_with_anonymous_text_and_intrinsic_sizes_and_options(
        snapshot,
        root,
        computed_styles,
        viewport,
        C11_4AnonymousGridTextProjectionInput {
            paragraph_styles,
            text_engine,
            intrinsic_sizes,
            allow_two_axis_nested_grid: false,
        },
    )
}

/// C11.4 depth-three 고정 fixture에서 anonymous text/image와 양 축 intrinsic sizing을 함께 연결합니다.
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub fn compute_runtime_block_flex_grid_track_sizing_style_layout_with_anonymous_text_and_intrinsic_sizes_and_two_axis_nested_grid(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    paragraph_styles: &BTreeMap<NodeId, ParagraphStyle>,
    text_engine: &TextLayoutEngine,
    intrinsic_sizes: LayoutIntrinsicSizeSnapshot,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    compute_runtime_block_flex_grid_track_sizing_style_layout_with_anonymous_text_and_intrinsic_sizes_and_options(
        snapshot,
        root,
        computed_styles,
        viewport,
        C11_4AnonymousGridTextProjectionInput {
            paragraph_styles,
            text_engine,
            intrinsic_sizes,
            allow_two_axis_nested_grid: true,
        },
    )
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
struct C11_4AnonymousGridTextProjectionInput<'a> {
    paragraph_styles: &'a BTreeMap<NodeId, ParagraphStyle>,
    text_engine: &'a TextLayoutEngine,
    intrinsic_sizes: LayoutIntrinsicSizeSnapshot,
    allow_two_axis_nested_grid: bool,
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn compute_runtime_block_flex_grid_track_sizing_style_layout_with_anonymous_text_and_intrinsic_sizes_and_options(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    projection_input: C11_4AnonymousGridTextProjectionInput<'_>,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    if computed_styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 {
        return Err(StyleLayoutError::UnsupportedProfile {
            profile: format!("{:?}", computed_styles.profile),
        });
    }
    if computed_styles.viewport != viewport {
        return Err(StyleLayoutError::SnapshotMismatch { field: "viewport" });
    }
    let inline_run_styles = BTreeMap::new();
    compute_layout_from_styles_with_text_and_grid_options(
        snapshot,
        root,
        computed_styles,
        viewport,
        Some(projection_input.intrinsic_sizes),
        Some((
            projection_input.paragraph_styles,
            &inline_run_styles,
            projection_input.text_engine,
        )),
        StyleLayoutProjectionOptions {
            reject_cascade_diagnostics: true,
            allow_implicit_named_lines: false,
            allow_c11_4_anonymous_grid_text_items: true,
            blockify_c11_4_anonymous_grid_images: true,
            allow_c11_4_nested_grid_two_axis_intrinsics: projection_input
                .allow_two_axis_nested_grid,
            allow_c11_4_replaced_grid_item_align_self_start: false,
            allow_c11_4_overflow_auto_minimum_fixture: false,
            allow_c11_4_grid_text_baseline_fixture: false,
            content_box_ratio_node: None,
            aspect_ratio_minmax_node: None,
        },
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
            | ComputedStyleProfile::C19BoxPaintOpacityV1
            | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
            | ComputedStyleProfile::RuntimeBlockFlexGridV1
            | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
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

/// C11.4 fixture-test에서 고정 two-axis Grid intrinsic image scene을 계산합니다.
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub fn compute_runtime_style_layout_with_intrinsic_size_snapshot_for_c11_4_two_axis_grid_fixture(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    intrinsic_sizes: LayoutIntrinsicSizeSnapshot,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    if computed_styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 {
        return Err(StyleLayoutError::UnsupportedProfile {
            profile: format!("{:?}", computed_styles.profile),
        });
    }
    if computed_styles.viewport != viewport {
        return Err(StyleLayoutError::SnapshotMismatch { field: "viewport" });
    }
    compute_layout_from_styles_with_text_and_grid_options(
        snapshot,
        root,
        computed_styles,
        viewport,
        Some(intrinsic_sizes),
        None,
        StyleLayoutProjectionOptions {
            reject_cascade_diagnostics: true,
            allow_implicit_named_lines: false,
            allow_c11_4_anonymous_grid_text_items: false,
            blockify_c11_4_anonymous_grid_images: false,
            allow_c11_4_nested_grid_two_axis_intrinsics: true,
            allow_c11_4_replaced_grid_item_align_self_start: false,
            allow_c11_4_overflow_auto_minimum_fixture: false,
            allow_c11_4_grid_text_baseline_fixture: false,
            content_box_ratio_node: None,
            aspect_ratio_minmax_node: None,
        },
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
        | ComputedStyleProfile::RuntimeFlexCustomPropertiesMediaEnvironmentV1
        | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
        | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
        | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
        | ComputedStyleProfile::RuntimeBlockPaintV1
        | ComputedStyleProfile::RuntimeRootHtmlBlockPaintV1
        | ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
        | ComputedStyleProfile::C19ComputedColorV1
        | ComputedStyleProfile::RuntimeBlockImageV1
        | ComputedStyleProfile::RuntimeBlockFormattingV1
        | ComputedStyleProfile::RuntimeBlockPositioningV1
        | ComputedStyleProfile::RuntimeBlockFlexStackingV1
        | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
        | ComputedStyleProfile::C19BoxPaintOpacityV1
        | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
        | ComputedStyleProfile::RuntimeBlockFlexGridV1
        | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
        | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
        | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
        | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
        | ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 => {
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
    compute_layout_from_styles_with_grid_options(
        snapshot,
        root,
        computed_styles,
        viewport,
        reject_cascade_diagnostics,
        intrinsic_sizes,
        false,
    )
}

fn compute_layout_from_styles_with_grid_options(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    reject_cascade_diagnostics: bool,
    intrinsic_sizes: Option<LayoutIntrinsicSizeSnapshot>,
    allow_implicit_named_lines: bool,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    compute_layout_from_styles_with_text_and_grid_options(
        snapshot,
        root,
        computed_styles,
        viewport,
        intrinsic_sizes,
        None,
        StyleLayoutProjectionOptions {
            reject_cascade_diagnostics,
            allow_implicit_named_lines,
            allow_c11_4_anonymous_grid_text_items: false,
            blockify_c11_4_anonymous_grid_images: false,
            allow_c11_4_nested_grid_two_axis_intrinsics: false,
            allow_c11_4_replaced_grid_item_align_self_start: false,
            allow_c11_4_overflow_auto_minimum_fixture: false,
            allow_c11_4_grid_text_baseline_fixture: false,
            content_box_ratio_node: None,
            aspect_ratio_minmax_node: None,
        },
    )
}

fn compute_layout_from_styles_with_text(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    reject_cascade_diagnostics: bool,
    intrinsic_sizes: Option<LayoutIntrinsicSizeSnapshot>,
    text_profile: Option<TextProjectionProfile<'_>>,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    compute_layout_from_styles_with_text_and_grid_options(
        snapshot,
        root,
        computed_styles,
        viewport,
        intrinsic_sizes,
        text_profile,
        StyleLayoutProjectionOptions {
            reject_cascade_diagnostics,
            allow_implicit_named_lines: false,
            allow_c11_4_anonymous_grid_text_items: false,
            blockify_c11_4_anonymous_grid_images: false,
            allow_c11_4_nested_grid_two_axis_intrinsics: false,
            allow_c11_4_replaced_grid_item_align_self_start: false,
            allow_c11_4_overflow_auto_minimum_fixture: false,
            allow_c11_4_grid_text_baseline_fixture: false,
            content_box_ratio_node: None,
            aspect_ratio_minmax_node: None,
        },
    )
}

#[derive(Clone, Copy)]
struct StyleLayoutProjectionOptions {
    reject_cascade_diagnostics: bool,
    allow_implicit_named_lines: bool,
    allow_c11_4_anonymous_grid_text_items: bool,
    blockify_c11_4_anonymous_grid_images: bool,
    allow_c11_4_nested_grid_two_axis_intrinsics: bool,
    allow_c11_4_replaced_grid_item_align_self_start: bool,
    allow_c11_4_overflow_auto_minimum_fixture: bool,
    allow_c11_4_grid_text_baseline_fixture: bool,
    content_box_ratio_node: Option<NodeId>,
    aspect_ratio_minmax_node: Option<NodeId>,
}

fn compute_layout_from_styles_with_text_and_grid_options(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    computed_styles: ComputedStyleSnapshot,
    viewport: CssViewport,
    intrinsic_sizes: Option<LayoutIntrinsicSizeSnapshot>,
    text_profile: Option<TextProjectionProfile<'_>>,
    options: StyleLayoutProjectionOptions,
) -> Result<StyleLayoutOutput, StyleLayoutError> {
    assert_matching_revision(snapshot, &computed_styles, computed_styles.profile)?;
    if computed_styles.profile == ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        && text_profile.is_none()
    {
        validate_c11_4_writing_styles_without_text_profile(&computed_styles)?;
    }
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    if computed_styles.profile == ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        && let Some((paragraph_styles, _, _)) = text_profile
    {
        validate_c11_4_orthogonal_text_styles(snapshot, root, &computed_styles, paragraph_styles)?;
    }
    if options.reject_cascade_diagnostics
        && let Some(diagnostic) = computed_styles.diagnostics.first().cloned()
    {
        return Err(StyleLayoutError::CascadeDiagnostic(diagnostic));
    }
    if matches!(
        computed_styles.profile,
        ComputedStyleProfile::RuntimeBlockPaintV1
            | ComputedStyleProfile::RuntimeRootHtmlBlockPaintV1
            | ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
            | ComputedStyleProfile::RuntimeBlockImageV1
            | ComputedStyleProfile::RuntimeBlockFormattingV1
            | ComputedStyleProfile::RuntimeBlockPositioningV1
            | ComputedStyleProfile::RuntimeBlockFlexStackingV1
            | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
            | ComputedStyleProfile::C19BoxPaintOpacityV1
            | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
            | ComputedStyleProfile::RuntimeBlockFlexGridV1
            | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
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
                ComputedStyleProfile::RuntimeRootHtmlTextPaintV1 => {
                    matches!(display.as_str(), "block" | "inline" | "none")
                }
                ComputedStyleProfile::RuntimeBlockFlexStackingV1 => {
                    matches!(display.as_str(), "block" | "flow-root" | "flex" | "none")
                }
                ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
                | ComputedStyleProfile::C19BoxPaintOpacityV1
                | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1 => {
                    matches!(display.as_str(), "block" | "flow-root" | "flex" | "none")
                }
                ComputedStyleProfile::RuntimeBlockFlexGridV1
                | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
                | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
                | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
                | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
                | ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 => {
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
                            | ComputedStyleProfile::C19BoxPaintOpacityV1
                            | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
                            | ComputedStyleProfile::RuntimeBlockFlexGridV1
                            | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
                            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
                            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
                            | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
                            | ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
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
    let mut projected = project_styles(snapshot, root, &computed_styles, options)?;
    validate_intrinsic_image_inputs(
        snapshot,
        root,
        &mut projected.styles,
        intrinsic_sizes.as_ref(),
        computed_styles.profile == ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
        options.allow_c11_4_nested_grid_two_axis_intrinsics,
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
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesMediaEnvironmentV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
            | ComputedStyleProfile::RuntimeBlockPaintV1
            | ComputedStyleProfile::RuntimeRootHtmlBlockPaintV1
            | ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
            | ComputedStyleProfile::RuntimeBlockImageV1
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
    let mut input = if let Some((paragraph_styles, inline_run_styles, _)) = text_profile {
        let text_styles = spinon_layout::HostTextStyleProfile {
            paragraphs: paragraph_styles,
            inline_runs: inline_run_styles,
        };
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        let input = if options.allow_c11_4_anonymous_grid_text_items {
            LayoutInput::from_host_document_with_viewport_containing_block_and_c11_4_anonymous_grid_text_items(
                snapshot,
                root,
                layout_viewport,
                &styles,
                text_styles,
                computed_styles.style_revision,
                viewport.environment_revision,
            )?
        } else {
            LayoutInput::from_host_document_with_viewport_containing_block_and_text_paragraphs_and_run_styles_ignoring_collapsible_whitespace(
                snapshot,
                root,
                layout_viewport,
                &styles,
                text_styles,
                computed_styles.style_revision,
                viewport.environment_revision,
            )?
        };
        #[cfg(not(feature = "c11-4-grid-track-sizing-fixture-test"))]
        let input = {
            debug_assert!(!options.allow_c11_4_anonymous_grid_text_items);
            LayoutInput::from_host_document_with_viewport_containing_block_and_text_paragraphs_and_run_styles_ignoring_collapsible_whitespace(
                snapshot,
                root,
                layout_viewport,
                &styles,
                text_styles,
                computed_styles.style_revision,
                viewport.environment_revision,
            )?
        };
        input
    } else if matches!(
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
    } else if computed_styles.profile == ComputedStyleProfile::RuntimeRootHtmlBlockPaintV1 {
        LayoutInput::from_host_document_with_viewport_containing_block_ignoring_collapsible_whitespace(
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
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesMediaEnvironmentV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
            | ComputedStyleProfile::RuntimeBlockPaintV1
            | ComputedStyleProfile::RuntimeRootHtmlBlockPaintV1
            | ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
            | ComputedStyleProfile::RuntimeBlockImageV1
            | ComputedStyleProfile::RuntimeBlockFormattingV1
            | ComputedStyleProfile::RuntimeBlockPositioningV1
            | ComputedStyleProfile::RuntimeBlockFlexStackingV1
            | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
            | ComputedStyleProfile::C19BoxPaintOpacityV1
            | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
            | ComputedStyleProfile::RuntimeBlockFlexGridV1
            | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
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
    };
    let layout_node_ids = input
        .nodes()
        .iter()
        .map(|node| node.id)
        .collect::<BTreeSet<_>>();
    let positioning = positioning
        .into_iter()
        .filter(|(node, _)| layout_node_ids.contains(node))
        .collect();
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    let css_math = projected.css_math.clone();
    input = input
        .with_css_math(projected.css_math)
        .with_positioning(positioning)
        .with_grid_layout(grid_containers, grid_items);
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    let grid_container_inputs = input.grid_containers().clone();
    let mut layout = match (text_profile, intrinsic_sizes) {
        (Some((_, _, text_engine)), Some(intrinsic_sizes)) => {
            #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
            {
                TaffyLayoutEngine.compute_with_intrinsic_size_snapshot_and_text_layout_engine(
                    &input,
                    &intrinsic_sizes,
                    text_engine,
                )?
            }
            #[cfg(not(feature = "c11-4-grid-track-sizing-fixture-test"))]
            {
                let _ = (intrinsic_sizes, text_engine);
                return Err(StyleLayoutError::UnsupportedProfile {
                    profile:
                        "mixed text and replaced intrinsic inputs require the C11.4 fixture feature"
                            .to_owned(),
                });
            }
        }
        (Some((_, _, text_engine)), None) => {
            TaffyLayoutEngine.compute_with_text_layout_engine(&input, text_engine)?
        }
        (None, Some(intrinsic_sizes)) => {
            TaffyLayoutEngine.compute_with_intrinsic_size_snapshot(&input, &intrinsic_sizes)?
        }
        (None, None) => TaffyLayoutEngine.compute(&input)?,
    };
    if matches!(
        computed_styles.profile,
        ComputedStyleProfile::RuntimeFlexLayoutV1
            | ComputedStyleProfile::RuntimeFlexPaintV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesMediaEnvironmentV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
            | ComputedStyleProfile::RuntimeBlockPaintV1
            | ComputedStyleProfile::RuntimeRootHtmlBlockPaintV1
            | ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
            | ComputedStyleProfile::RuntimeBlockImageV1
            | ComputedStyleProfile::RuntimeBlockFormattingV1
            | ComputedStyleProfile::RuntimeBlockPositioningV1
            | ComputedStyleProfile::RuntimeBlockFlexStackingV1
            | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
            | ComputedStyleProfile::C19BoxPaintOpacityV1
            | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
            | ComputedStyleProfile::RuntimeBlockFlexGridV1
            | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
    ) {
        zero_display_none_frames(snapshot, root, &styles, &mut layout);
    }
    Ok(StyleLayoutOutput {
        computed_styles,
        layout,
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        grid_container_inputs,
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        css_math,
    })
}

fn validate_intrinsic_image_inputs(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &mut BTreeMap<NodeId, LayoutStyle>,
    intrinsic_sizes: Option<&LayoutIntrinsicSizeSnapshot>,
    allow_root_grid_intrinsic_content: bool,
    allow_c11_4_nested_grid_two_axis_intrinsics: bool,
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
        // C11.4 track-sizing profile은 Grid의 inline min/max-content width를 허용한다.
        // intrinsic block size, non-Grid, 다른 profile은 기존 fail-closed 경계를 유지한다.
        let c11_4_grid_intrinsic_width = allow_root_grid_intrinsic_content
            && style.display == spinon_layout::LayoutDisplay::Grid
            && matches!(
                style.width,
                spinon_layout::LayoutDimension::MinContent
                    | spinon_layout::LayoutDimension::MaxContent
            )
            && !matches!(
                style.height,
                spinon_layout::LayoutDimension::MinContent
                    | spinon_layout::LayoutDimension::MaxContent
                    | spinon_layout::LayoutDimension::FitContent
            );
        let root_grid_max_content = allow_root_grid_intrinsic_content
            && handle == root
            && style.display == spinon_layout::LayoutDisplay::Grid
            && (style.width == spinon_layout::LayoutDimension::MaxContent
                || style.height == spinon_layout::LayoutDimension::MaxContent)
            && [style.width, style.height].into_iter().all(|dimension| {
                !matches!(
                    dimension,
                    spinon_layout::LayoutDimension::MinContent
                        | spinon_layout::LayoutDimension::FitContent
                )
            });
        let root_grid_min_content_width_max_content_height = allow_root_grid_intrinsic_content
            && handle == root
            && style.display == spinon_layout::LayoutDisplay::Grid
            && style.width == spinon_layout::LayoutDimension::MinContent
            && style.height == spinon_layout::LayoutDimension::MaxContent;
        let c11_4_nested_grid_two_axis_intrinsics = allow_c11_4_nested_grid_two_axis_intrinsics
            && style.display == spinon_layout::LayoutDisplay::Grid
            && matches!(
                (style.width, style.height),
                (
                    spinon_layout::LayoutDimension::MinContent,
                    spinon_layout::LayoutDimension::MinContent
                ) | (
                    spinon_layout::LayoutDimension::MaxContent,
                    spinon_layout::LayoutDimension::MaxContent
                )
            );
        let root_grid_intrinsic_content = c11_4_grid_intrinsic_width
            || root_grid_max_content
            || root_grid_min_content_width_max_content_height
            || c11_4_nested_grid_two_axis_intrinsics;
        if has_intrinsic_keyword && !is_html_image && !root_grid_intrinsic_content {
            let supported_image = if style.width == spinon_layout::LayoutDimension::MinContent {
                is_supported_cyclic_min_content_container(snapshot, handle, style, styles)
            } else {
                None
            };
            let Some((image, fraction)) = supported_image else {
                return Err(LayoutError::UnsupportedIntrinsicSizing {
                    node: node.id(),
                    reason: "intrinsic sizing은 C14.3 이미지·제한 cyclic parent 또는 C11.4 track-sizing profile의 Grid min/max-content width와 root Grid max-content만 지원합니다",
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
    fn non_layout_snapshot_profiles_are_rejected_by_the_flex_layout_adapter() {
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
        for profile in [
            ComputedStyleProfile::SupportedElementsUaV1,
            ComputedStyleProfile::C19ComputedColorV1,
        ] {
            let error = compute_profile_layout(
                &snapshot,
                &view,
                root,
                &[],
                CssViewport::C04_FIXTURE,
                StyleRevision::default(),
                profile,
            )
            .unwrap_err();

            assert!(matches!(
                error,
                StyleLayoutError::UnsupportedProfile { profile: error_profile }
                    if error_profile == format!("{profile:?}")
            ));
        }
    }
}
