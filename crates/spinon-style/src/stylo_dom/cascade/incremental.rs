use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{NodeId, StyleRevision};
use style::{properties::LonghandId, shared_lock::StylesheetGuards, stylist::Stylist};
use url::Url;

use super::{
    CascadeDiagnostic, ComputedElementStyle, ComputedStyleProfile, ComputedStyleSnapshot,
    CssCascadeError, CssViewport, FLEX_ALIGNMENT_AUTHOR_PROPERTIES, FLEX_LAYOUT_AUTHOR_PROPERTIES,
    StyloDocumentView, UA_STYLESHEET_ID, UA_STYLESHEET_URL,
    device::{LayoutThreadState, make_device},
    margin::FLEX_MARGIN_AUTHOR_PROPERTIES,
    runtime_layout::{self, RUNTIME_FLEX_LAYOUT_AUTHOR_PROPERTIES},
    s04,
};
use crate::{
    CssOrigin, StylesheetRegistry, StylesheetSource, UA_STYLESHEET,
    s04_color_syntax::first_invalid_background_color,
};

#[path = "incremental/element.rs"]
mod element;
use element::{compute_element_style, computed_element_output};

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
    let has_paint = matches!(
        profile,
        ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
    );
    match profile {
        ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
        | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
        | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
        | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1 => {
            let mut properties = runtime_layout::RUNTIME_FLEX_LAYOUT_PROPERTIES.to_vec();
            if has_paint {
                properties.push(("background-color", LonghandId::BackgroundColor));
            }
            Some(properties)
        }
        _ => None,
    }
}

fn same_viewport(left: CssViewport, right: CssViewport) -> bool {
    left.width_css_px.to_bits() == right.width_css_px.to_bits()
        && left.height_css_px.to_bits() == right.height_css_px.to_bits()
        && left.device_scale_factor.to_bits() == right.device_scale_factor.to_bits()
        && left.environment_revision == right.environment_revision
        && left.media_environment == right.media_environment
}

struct CascadeReusePlan {
    old_styles: BTreeMap<NodeId, ComputedElementStyle>,
    dirty_nodes: BTreeSet<NodeId>,
    context_nodes: BTreeSet<NodeId>,
}

fn make_reuse_plan(
    view: &StyloDocumentView,
    previous: &ComputedStyleSnapshot,
    dirty_root_ids: &[NodeId],
) -> Option<CascadeReusePlan> {
    let mut current_handles = BTreeMap::new();
    let mut pending = vec![view.root_handle()];
    while let Some(handle) = pending.pop() {
        current_handles.insert(handle.id(), handle);
        if let Some(children) = view.snapshot().children(handle) {
            pending.extend(children);
        }
    }

    let mut old_styles = BTreeMap::new();
    for old in previous.elements.iter() {
        if old_styles.insert(old.node_id, old.clone()).is_some() {
            return None;
        }
    }
    let current_element_ids = current_handles
        .iter()
        .filter_map(|(id, handle)| view.element(*handle).map(|_| *id))
        .collect::<BTreeSet<_>>();
    if current_element_ids != old_styles.keys().copied().collect::<BTreeSet<_>>()
        || previous.elements.first().map(|style| style.node_id) != Some(view.root_handle().id())
    {
        return None;
    }

    let mut dirty_nodes = BTreeSet::new();
    for dirty_root_id in dirty_root_ids {
        let root = *current_handles.get(dirty_root_id)?;
        view.element(root)?;
        let mut subtree = vec![root];
        while let Some(handle) = subtree.pop() {
            if view.element(handle).is_some() {
                dirty_nodes.insert(handle.id());
            }
            if let Some(children) = view.snapshot().children(handle) {
                subtree.extend(children);
            }
        }
    }

    let mut context_nodes = BTreeSet::new();
    for dirty_root_id in dirty_root_ids {
        let mut handle = *current_handles.get(dirty_root_id)?;
        while handle != view.root_handle() {
            let parent = match view.snapshot().parent(handle)? {
                spinon_core::HostParent::Node(parent) => parent,
                spinon_core::HostParent::Root => break,
            };
            if view.element(parent).is_some() && !dirty_nodes.contains(&parent.id()) {
                context_nodes.insert(parent.id());
            }
            handle = parent;
        }
    }

    Some(CascadeReusePlan {
        old_styles,
        dirty_nodes,
        context_nodes,
    })
}

pub(super) fn compute_cascade_with_reuse(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: StyleRevision,
    properties: &[(&str, LonghandId)],
    profile: ComputedStyleProfile,
    reuse: Option<(&ComputedStyleSnapshot, &[NodeId])>,
) -> Result<Option<(ComputedStyleSnapshot, RuntimeCascadeReuseStats)>, CssCascadeError> {
    let reuse_plan = match reuse {
        Some((previous, dirty_roots)) => {
            let Some(plan) = make_reuse_plan(view, previous, dirty_roots) else {
                return Ok(None);
            };
            Some(plan)
        }
        None => None,
    };
    if !viewport.is_valid() {
        return Err(CssCascadeError::InvalidViewport);
    }
    if !viewport.media_environment.is_valid() {
        return Err(CssCascadeError::InvalidMediaEnvironment);
    }

    let mut registry = StylesheetRegistry::with_shared_lock(view.shared_lock().clone());
    registry.append(StylesheetSource {
        id: UA_STYLESHEET_ID.to_owned(),
        base_url: Url::parse(UA_STYLESHEET_URL)
            .expect("내장 UA stylesheet URL은 절대 URL이어야 합니다")
            .to_string(),
        origin: CssOrigin::UserAgent,
        css: UA_STYLESHEET.to_owned(),
    })?;
    for source in author_stylesheets {
        if source.origin != CssOrigin::Author {
            return Err(CssCascadeError::InvalidStylesheetOrigin {
                id: source.id.clone(),
            });
        }
        registry.append(source.clone())?;
    }
    let allowed_author_properties = match profile {
        ComputedStyleProfile::BasicCascadeV1 => None,
        ComputedStyleProfile::SupportedElementsUaV1 => None,
        ComputedStyleProfile::FlexLayoutV1 => Some(FLEX_LAYOUT_AUTHOR_PROPERTIES),
        ComputedStyleProfile::FlexMarginV1 | ComputedStyleProfile::FlexMediaEnvironmentV1 => {
            Some(FLEX_MARGIN_AUTHOR_PROPERTIES)
        }
        ComputedStyleProfile::FlexAlignmentV1
        | ComputedStyleProfile::FlexAlignmentCascadeLayersV1 => {
            Some(FLEX_ALIGNMENT_AUTHOR_PROPERTIES)
        }
        ComputedStyleProfile::S04FlexPaintV1 => Some(s04::S04_FLEX_PAINT_AUTHOR_PROPERTIES),
        ComputedStyleProfile::RuntimeFlexLayoutV1
        | ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
        | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
        | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
        | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1 => {
            Some(RUNTIME_FLEX_LAYOUT_AUTHOR_PROPERTIES)
        }
        ComputedStyleProfile::RuntimeFlexPaintV1 => None,
    };
    let unsupported_author_feature = match profile {
        ComputedStyleProfile::FlexAlignmentCascadeLayersV1 => allowed_author_properties
            .and_then(|allowed| registry.first_unsupported_author_feature_with_layers(allowed)),
        ComputedStyleProfile::FlexMediaEnvironmentV1 => allowed_author_properties
            .and_then(|allowed| registry.first_unsupported_author_feature_with_media(allowed)),
        ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
        | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1 => allowed_author_properties
            .and_then(|allowed| {
                registry.first_unsupported_runtime_author_feature(
                    allowed,
                    true,
                    profile == ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1,
                )
            }),
        ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
        | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1 => allowed_author_properties
            .and_then(|allowed| {
                registry.first_unsupported_runtime_registered_properties_author_feature(
                    allowed,
                    profile == ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1,
                )
            }),
        _ => allowed_author_properties
            .and_then(|allowed| registry.first_unsupported_author_feature(allowed)),
    };
    if let Some((stylesheet_id, feature)) = unsupported_author_feature {
        return Err(CssCascadeError::UnsupportedAuthorCss {
            stylesheet_id,
            feature,
        });
    }
    if matches!(
        profile,
        ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
    ) && let Some((stylesheet_id, diagnostic)) = registry.first_runtime_author_diagnostic()
    {
        return Err(CssCascadeError::AuthorStylesheetDiagnostic {
            stylesheet_id,
            line: diagnostic.line,
            column: diagnostic.column,
            message: diagnostic.message,
        });
    }
    if matches!(
        profile,
        ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
    ) && let Some((stylesheet_id, diagnostic)) =
        registry.first_runtime_registered_property_author_diagnostic()
    {
        return Err(CssCascadeError::AuthorStylesheetDiagnostic {
            stylesheet_id,
            line: diagnostic.line,
            column: diagnostic.column,
            message: diagnostic.message,
        });
    }
    if profile == ComputedStyleProfile::S04FlexPaintV1 {
        for source in author_stylesheets {
            if let Some(feature) = first_invalid_background_color(&source.css) {
                return Err(CssCascadeError::UnsupportedAuthorCss {
                    stylesheet_id: source.id.clone(),
                    feature,
                });
            }
        }
    }

    let device = make_device(view.quirks_mode(), viewport);
    let mut stylist = Stylist::new(device, view.quirks_mode());
    let mut elements = Vec::new();
    let mut diagnostics = Vec::new();
    for stylesheet in registry.iter() {
        diagnostics.extend(
            stylesheet
                .diagnostics()
                .iter()
                .filter(|diagnostic| {
                    !matches!(
                        profile,
                        ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
                            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
                    ) || !diagnostic
                        .message
                        .starts_with("Unsupported @property descriptor declaration:")
                })
                .cloned()
                .map(|diagnostic| CascadeDiagnostic {
                    source_id: stylesheet.id().to_owned(),
                    node_id: None,
                    diagnostic,
                }),
        );
    }

    let guard = view.shared_lock().read();
    for (_, stylesheet) in registry.iter_stylo_sheets() {
        stylist.append_stylesheet(stylesheet.clone(), &guard);
    }
    let guards = StylesheetGuards::same(&guard);
    stylist.flush(&guards);

    let _layout_state = LayoutThreadState::enter();
    let mut reuse_stats = RuntimeCascadeReuseStats::default();
    let mut pending = vec![(view.root_handle(), None)];
    while let Some((handle, parent_style)) = pending.pop() {
        let computed = if let Some(element) = view.element(handle) {
            for diagnostic in &element.data().inline_style_diagnostics {
                diagnostics.push(CascadeDiagnostic {
                    source_id: format!("inline:{}", handle.id()),
                    node_id: Some(handle.id()),
                    diagnostic: diagnostic.clone(),
                });
            }
            let node_id = handle.id();
            let should_compute = reuse_plan.as_ref().is_none_or(|plan| {
                plan.dirty_nodes.contains(&node_id) || plan.context_nodes.contains(&node_id)
            });
            if should_compute {
                let computed =
                    compute_element_style(&stylist, element, &guards, parent_style.as_deref());
                if reuse_plan
                    .as_ref()
                    .is_some_and(|plan| plan.context_nodes.contains(&node_id))
                {
                    elements.push(
                        reuse_plan
                            .as_ref()
                            .and_then(|plan| plan.old_styles.get(&node_id))
                            .expect("context ancestor style output must exist")
                            .clone(),
                    );
                    reuse_stats.reused_style_elements += 1;
                    reuse_stats.context_style_elements += 1;
                } else {
                    elements.push(computed_element_output(
                        properties, profile, &computed, node_id,
                    )?);
                    reuse_stats.recomputed_style_elements += 1;
                }
                Some(computed)
            } else {
                elements.push(
                    reuse_plan
                        .as_ref()
                        .and_then(|plan| plan.old_styles.get(&node_id))
                        .expect("reused style output must exist")
                        .clone(),
                );
                reuse_stats.reused_style_elements += 1;
                None
            }
        } else {
            parent_style
        };

        if let Some(children) = view.snapshot().children(handle) {
            let children = children.collect::<Vec<_>>();
            pending.extend(
                children
                    .into_iter()
                    .rev()
                    .map(|child| (child, computed.clone())),
            );
        }
    }

    Ok(Some((
        ComputedStyleSnapshot {
            profile,
            viewport,
            style_revision,
            generation: view.snapshot().generation(),
            document_revision: view.document_revision(),
            render_tree_revision: view.render_tree_revision(),
            elements: elements.into(),
            diagnostics,
        },
        reuse_stats,
    )))
}
