use spinon_core::{NodeId, StyleRevision};
use style::{properties::LonghandId, shared_lock::StylesheetGuards, stylist::Stylist};
use url::Url;

use super::super::{
    CascadeDiagnostic, ComputedStyleProfile, ComputedStyleSnapshot, CssCascadeError, CssViewport,
    FLEX_ALIGNMENT_AUTHOR_PROPERTIES, FLEX_LAYOUT_AUTHOR_PROPERTIES, UA_STYLESHEET_ID,
    UA_STYLESHEET_URL,
    device::{LayoutThreadState, make_device},
    margin::FLEX_MARGIN_AUTHOR_PROPERTIES,
    runtime_layout::RUNTIME_FLEX_LAYOUT_AUTHOR_PROPERTIES,
    s04,
};
use super::{
    RuntimeCascadeReuseStats, StyloDocumentView, computed_font_size_css_px,
    element::{compute_element_style, compute_element_style_with_math, computed_element_output},
    is_runtime_layout_profile,
    reuse::make_reuse_plan,
    validate_synthetic_document_box,
};
use crate::{
    CssOrigin, StylesheetRegistry, StylesheetSource, UA_STYLESHEET,
    container_relative_units::first_container_relative_unit,
    font_relative_units::first_font_metric_unit, s04_color_syntax::first_invalid_background_color,
};

pub(in crate::stylo_dom::cascade) fn compute_cascade_with_reuse(
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
    if is_runtime_layout_profile(profile) {
        for source in author_stylesheets {
            match first_container_relative_unit(&source.css) {
                Ok(Some(unit)) => {
                    return Err(CssCascadeError::UnsupportedContainerRelativeUnit {
                        source_id: source.id.clone(),
                        node: None,
                        unit,
                    });
                }
                Ok(None) => {}
                Err(()) => {
                    return Err(CssCascadeError::InvalidContainerRelativeUnitInput {
                        source_id: source.id.clone(),
                        node: None,
                    });
                }
            }
            match first_font_metric_unit(&source.css) {
                Ok(Some(unit)) => {
                    return Err(CssCascadeError::UnsupportedFontMetricUnit {
                        source_id: source.id.clone(),
                        node: None,
                        unit,
                    });
                }
                Ok(None) => {}
                Err(()) => {
                    return Err(CssCascadeError::InvalidFontMetricInput {
                        source_id: source.id.clone(),
                        node: None,
                    });
                }
            }
        }
        for (node, css) in view.inline_style_sources() {
            let source_id = format!("inline:{node}");
            match first_container_relative_unit(css) {
                Ok(Some(unit)) => {
                    return Err(CssCascadeError::UnsupportedContainerRelativeUnit {
                        source_id,
                        node: Some(node),
                        unit,
                    });
                }
                Ok(None) => {}
                Err(()) => {
                    return Err(CssCascadeError::InvalidContainerRelativeUnitInput {
                        source_id,
                        node: Some(node),
                    });
                }
            }
            let source_id = format!("inline:{node}");
            match first_font_metric_unit(css) {
                Ok(Some(unit)) => {
                    return Err(CssCascadeError::UnsupportedFontMetricUnit {
                        source_id,
                        node: Some(node),
                        unit,
                    });
                }
                Ok(None) => {}
                Err(()) => {
                    return Err(CssCascadeError::InvalidFontMetricInput {
                        source_id,
                        node: Some(node),
                    });
                }
            }
        }
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
    let mut document_root_font_size_css_px = stylist.device().root_font_size().px();
    let mut mount_parent_style = None;
    if is_runtime_layout_profile(profile) && view.has_synthetic_html_document() {
        let html = view.root_element();
        let html_style = compute_element_style(&stylist, html, &guards, None);
        validate_synthetic_document_box(&html_style, "html")?;
        document_root_font_size_css_px = computed_font_size_css_px(&html_style);
        if !document_root_font_size_css_px.is_finite() || document_root_font_size_css_px < 0.0 {
            return Err(CssCascadeError::InvalidComputedFontSize { node: None });
        }
        stylist.device().set_root_style(&html_style);
        stylist
            .device()
            .set_root_font_size(document_root_font_size_css_px);
        let body = view
            .synthetic_body_element()
            .expect("합성 HTML 문서에는 body wrapper가 있어야 합니다");
        let body_style = compute_element_style(&stylist, body, &guards, Some(&html_style));
        validate_synthetic_document_box(&body_style, "body")?;
        mount_parent_style = Some(body_style);
    }
    let mut pending = vec![(view.root_handle(), mount_parent_style)];
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
                let (computed, source_math) = compute_element_style_with_math(
                    &stylist,
                    element,
                    &guards,
                    parent_style.as_deref(),
                );
                if handle == view.root_handle() && view.root_matches_root_pseudo() {
                    document_root_font_size_css_px = computed_font_size_css_px(&computed);
                    if !document_root_font_size_css_px.is_finite()
                        || document_root_font_size_css_px < 0.0
                    {
                        return Err(CssCascadeError::InvalidComputedFontSize {
                            node: Some(node_id),
                        });
                    }
                    stylist.device().set_root_style(&computed);
                    stylist
                        .device()
                        .set_root_font_size(document_root_font_size_css_px);
                }
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
                        properties,
                        profile,
                        &computed,
                        node_id,
                        source_math,
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
            document_root_font_size_css_px,
            elements: elements.into(),
            diagnostics,
        },
        reuse_stats,
    )))
}
