use spinon_core::{NodeId, StyleRevision, TextOrientation, WritingMode};
use style::{
    properties::{LonghandId, PropertyDeclarationId},
    shared_lock::StylesheetGuards,
    stylist::Stylist,
};
use url::Url;

use super::super::{
    CascadeDiagnostic, ComputedStyleProfile, ComputedStyleSnapshot, CssCascadeError, CssViewport,
    FLEX_ALIGNMENT_AUTHOR_PROPERTIES, FLEX_LAYOUT_AUTHOR_PROPERTIES, UA_STYLESHEET_ID,
    UA_STYLESHEET_URL,
    device::{
        LayoutThreadState, acquire_stylo_preferences, initialize_stylo_preferences, make_device,
    },
    margin::FLEX_MARGIN_AUTHOR_PROPERTIES,
    runtime_grid::{
        runtime_block_flex_grid_author_properties,
        runtime_block_flex_grid_auto_placement_author_properties,
        runtime_block_flex_grid_dense_auto_placement_author_properties,
        runtime_block_flex_grid_placement_author_properties,
        runtime_block_flex_grid_template_author_properties,
        runtime_block_flex_grid_track_sizing_author_properties,
    },
    runtime_layout::RUNTIME_FLEX_LAYOUT_AUTHOR_PROPERTIES,
    runtime_overflow::runtime_block_flex_overflow_clip_author_properties,
    runtime_paint::RUNTIME_BLOCK_IMAGE_AUTHOR_PROPERTIES,
    runtime_paint::RUNTIME_BLOCK_PAINT_AUTHOR_PROPERTIES,
    runtime_paint::RUNTIME_ROOT_HTML_BLOCK_PAINT_AUTHOR_PROPERTIES,
    runtime_paint::RUNTIME_ROOT_HTML_TEXT_PAINT_AUTHOR_PROPERTIES,
    runtime_scroll_gesture::runtime_block_flex_scroll_gesture_author_properties,
    runtime_stacking::runtime_block_flex_stacking_author_properties,
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

pub(in crate::stylo_dom::cascade) struct CascadeComputeRequest<'a> {
    pub view: &'a StyloDocumentView,
    pub additional_user_agent_stylesheets: &'a [StylesheetSource],
    pub author_stylesheets: &'a [StylesheetSource],
    pub viewport: CssViewport,
    pub style_revision: StyleRevision,
    pub properties: &'a [(&'a str, LonghandId)],
    pub profile: ComputedStyleProfile,
    pub author_properties_override: Option<&'a [&'a str]>,
    pub reuse: Option<(&'a ComputedStyleSnapshot, &'a [NodeId])>,
}

pub(in crate::stylo_dom::cascade) fn compute_cascade_with_reuse(
    request: CascadeComputeRequest<'_>,
) -> Result<Option<(ComputedStyleSnapshot, RuntimeCascadeReuseStats)>, CssCascadeError> {
    let CascadeComputeRequest {
        view,
        additional_user_agent_stylesheets,
        author_stylesheets,
        viewport,
        style_revision,
        properties,
        profile,
        author_properties_override,
        reuse,
    } = request;
    if profile == ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 && reuse.is_some() {
        return Ok(None);
    }
    let _preferences = acquire_stylo_preferences(profile);
    initialize_stylo_preferences();
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
    registry.append_with_stylo_preferences_held(StylesheetSource {
        id: UA_STYLESHEET_ID.to_owned(),
        base_url: Url::parse(UA_STYLESHEET_URL)
            .expect("내장 UA stylesheet URL은 절대 URL이어야 합니다")
            .to_string(),
        origin: CssOrigin::UserAgent,
        css: UA_STYLESHEET.to_owned(),
    })?;
    for source in additional_user_agent_stylesheets {
        if source.origin != CssOrigin::UserAgent {
            return Err(CssCascadeError::InvalidStylesheetOrigin {
                id: source.id.clone(),
            });
        }
        registry.append_with_stylo_preferences_held(source.clone())?;
    }
    for source in author_stylesheets {
        if source.origin != CssOrigin::Author {
            return Err(CssCascadeError::InvalidStylesheetOrigin {
                id: source.id.clone(),
            });
        }
        registry.append_with_stylo_preferences_held(source.clone())?;
    }
    let stylesheet_border_sources = registry.raw_style_declaration_sources();
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
    let runtime_stacking_author_properties = runtime_block_flex_stacking_author_properties();
    let runtime_overflow_clip_author_properties =
        runtime_block_flex_overflow_clip_author_properties();
    let runtime_scroll_gesture_author_properties =
        runtime_block_flex_scroll_gesture_author_properties();
    let runtime_grid_author_properties = runtime_block_flex_grid_author_properties();
    let runtime_grid_placement_author_properties =
        runtime_block_flex_grid_placement_author_properties();
    let runtime_grid_auto_placement_author_properties =
        runtime_block_flex_grid_auto_placement_author_properties();
    let runtime_grid_dense_auto_placement_author_properties =
        runtime_block_flex_grid_dense_auto_placement_author_properties();
    let runtime_grid_template_author_properties =
        runtime_block_flex_grid_template_author_properties();
    let runtime_grid_track_sizing_author_properties =
        runtime_block_flex_grid_track_sizing_author_properties();
    let mut runtime_root_html_author_properties = RUNTIME_FLEX_LAYOUT_AUTHOR_PROPERTIES.to_vec();
    runtime_root_html_author_properties.push("color-scheme");
    runtime_root_html_author_properties.push("background-color");
    let profile_author_properties: Option<&[&str]> = match profile {
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
        ComputedStyleProfile::RuntimeFlexCustomPropertiesMediaEnvironmentV1 => {
            Some(runtime_root_html_author_properties.as_slice())
        }
        ComputedStyleProfile::RuntimeFlexLayoutV1
        | ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
        | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
        | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
        | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1 => {
            Some(RUNTIME_FLEX_LAYOUT_AUTHOR_PROPERTIES)
        }
        ComputedStyleProfile::RuntimeBlockPaintV1 | ComputedStyleProfile::C19ComputedColorV1 => {
            Some(RUNTIME_BLOCK_PAINT_AUTHOR_PROPERTIES)
        }
        ComputedStyleProfile::RuntimeRootHtmlBlockPaintV1 => {
            Some(RUNTIME_ROOT_HTML_BLOCK_PAINT_AUTHOR_PROPERTIES)
        }
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1 => {
            Some(RUNTIME_ROOT_HTML_TEXT_PAINT_AUTHOR_PROPERTIES)
        }
        ComputedStyleProfile::RuntimeBlockImageV1 => Some(RUNTIME_BLOCK_IMAGE_AUTHOR_PROPERTIES),
        ComputedStyleProfile::RuntimeBlockFormattingV1 => {
            Some(super::runtime_paint::RUNTIME_BLOCK_FORMATTING_AUTHOR_PROPERTIES)
        }
        ComputedStyleProfile::RuntimeBlockPositioningV1 => {
            Some(super::runtime_paint::RUNTIME_BLOCK_POSITIONING_AUTHOR_PROPERTIES)
        }
        ComputedStyleProfile::RuntimeBlockFlexStackingV1 => {
            Some(runtime_stacking_author_properties.as_slice())
        }
        ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
        | ComputedStyleProfile::C19BoxPaintOpacityV1 => {
            Some(runtime_overflow_clip_author_properties.as_slice())
        }
        ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1 => {
            Some(runtime_scroll_gesture_author_properties.as_slice())
        }
        ComputedStyleProfile::RuntimeBlockFlexGridV1 => {
            Some(runtime_grid_author_properties.as_slice())
        }
        ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1 => {
            Some(runtime_grid_placement_author_properties.as_slice())
        }
        ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1 => {
            Some(runtime_grid_auto_placement_author_properties.as_slice())
        }
        ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1 => {
            Some(runtime_grid_dense_auto_placement_author_properties.as_slice())
        }
        ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1 => {
            Some(runtime_grid_template_author_properties.as_slice())
        }
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 => {
            Some(runtime_grid_track_sizing_author_properties.as_slice())
        }
        ComputedStyleProfile::RuntimeFlexPaintV1 => None,
    };
    let allowed_author_properties = author_properties_override.or(profile_author_properties);
    let unsupported_author_feature = match profile {
        ComputedStyleProfile::FlexAlignmentCascadeLayersV1 => allowed_author_properties
            .and_then(|allowed| registry.first_unsupported_author_feature_with_layers(allowed)),
        ComputedStyleProfile::FlexMediaEnvironmentV1 => allowed_author_properties
            .and_then(|allowed| registry.first_unsupported_author_feature_with_media(allowed)),
        ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
        | ComputedStyleProfile::RuntimeFlexCustomPropertiesMediaEnvironmentV1
        | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1 => allowed_author_properties
            .and_then(|allowed| {
                if profile == ComputedStyleProfile::RuntimeFlexCustomPropertiesMediaEnvironmentV1 {
                    registry.first_unsupported_runtime_author_feature_with_media(allowed)
                } else {
                    registry.first_unsupported_runtime_author_feature(
                        allowed,
                        profile == ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1,
                        true,
                        profile == ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1,
                    )
                }
            }),
        ComputedStyleProfile::RuntimeRootHtmlBlockPaintV1 => {
            allowed_author_properties.and_then(|allowed| {
                registry.first_unsupported_runtime_author_feature_with_media(allowed)
            })
        }
        ComputedStyleProfile::RuntimeRootHtmlTextPaintV1 => {
            allowed_author_properties.and_then(|allowed| {
                registry.first_unsupported_runtime_author_feature_with_media(allowed)
            })
        }
        ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
        | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1 => allowed_author_properties
            .and_then(|allowed| {
                registry.first_unsupported_runtime_registered_properties_author_feature(
                    allowed,
                    profile == ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1,
                )
            }),
        ComputedStyleProfile::RuntimeBlockPositioningV1 => {
            allowed_author_properties.and_then(|allowed| {
                registry.first_unsupported_runtime_author_feature(allowed, true, true, true)
            })
        }
        ComputedStyleProfile::RuntimeBlockFlexStackingV1 => {
            allowed_author_properties.and_then(|allowed| {
                registry.first_unsupported_runtime_layered_registered_properties_author_feature(
                    allowed, true,
                )
            })
        }
        ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
        | ComputedStyleProfile::C19BoxPaintOpacityV1
        | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1 => allowed_author_properties
            .and_then(|allowed| {
                registry.first_unsupported_runtime_layered_registered_properties_author_feature(
                    allowed, true,
                )
            }),
        ComputedStyleProfile::RuntimeBlockImageV1 => allowed_author_properties
            .and_then(|allowed| registry.first_unsupported_author_feature(allowed))
            .or_else(|| registry.first_unsupported_runtime_block_image_value()),
        _ => allowed_author_properties
            .and_then(|allowed| registry.first_unsupported_author_feature(allowed)),
    };
    if let Some((stylesheet_id, feature)) = unsupported_author_feature {
        return Err(CssCascadeError::UnsupportedAuthorCss {
            stylesheet_id,
            feature,
        });
    }
    if profile == ComputedStyleProfile::RuntimeFlexCustomPropertiesMediaEnvironmentV1
        && let Some((stylesheet_id, feature)) =
            registry.first_unsupported_runtime_root_html_system_color()
    {
        return Err(CssCascadeError::UnsupportedAuthorCss {
            stylesheet_id,
            feature,
        });
    }
    if (profile == ComputedStyleProfile::RuntimeRootHtmlBlockPaintV1
        || profile == ComputedStyleProfile::RuntimeRootHtmlTextPaintV1)
        && let Some((stylesheet_id, diagnostic)) = registry.first_runtime_author_diagnostic()
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
        ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesMediaEnvironmentV1
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
            | ComputedStyleProfile::RuntimeBlockFlexStackingV1
            | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
            | ComputedStyleProfile::C19BoxPaintOpacityV1
            | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
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
    let mut writing_styles = Vec::new();
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
    let needs_synthetic_document_cascade =
        is_runtime_layout_profile(profile) || profile == ComputedStyleProfile::C19ComputedColorV1;
    if needs_synthetic_document_cascade && view.has_synthetic_html_document() {
        let html = view.root_element();
        let html_style = compute_element_style(
            &stylist,
            html,
            &guards,
            None,
            viewport,
            &stylesheet_border_sources,
        );
        if is_runtime_layout_profile(profile) {
            validate_synthetic_document_box(
                &html_style,
                "html",
                profile == ComputedStyleProfile::RuntimeFlexCustomPropertiesMediaEnvironmentV1,
            )?;
        }
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
        let body_style = compute_element_style(
            &stylist,
            body,
            &guards,
            Some(&html_style),
            viewport,
            &stylesheet_border_sources,
        );
        if is_runtime_layout_profile(profile) {
            validate_synthetic_document_box(
                &body_style,
                "body",
                profile == ComputedStyleProfile::RuntimeFlexCustomPropertiesMediaEnvironmentV1,
            )?;
        }
        mount_parent_style = Some(body_style);
    }
    let mut pending = vec![(view.root_handle(), mount_parent_style, false)];
    while let Some((handle, parent_style, ancestor_hidden)) = pending.pop() {
        let mut subtree_hidden = ancestor_hidden;
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
                let (computed, source_math, source_border_widths) = compute_element_style_with_math(
                    &stylist,
                    element,
                    &guards,
                    parent_style.as_deref(),
                    viewport,
                    &stylesheet_border_sources,
                );
                if profile == ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 {
                    let (writing_mode, text_orientation) =
                        computed_writing_style(&computed, node_id)?;
                    writing_styles.push((node_id, writing_mode, text_orientation));
                }
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
                    let output = reuse_plan
                        .as_ref()
                        .and_then(|plan| plan.old_styles.get(&node_id))
                        .expect("context ancestor style output must exist")
                        .clone();
                    subtree_hidden = ancestor_hidden
                        || output
                            .properties
                            .get("display")
                            .is_some_and(|value| value == "none");
                    elements.push(output);
                    reuse_stats.reused_style_elements += 1;
                    reuse_stats.context_style_elements += 1;
                } else {
                    let output = computed_element_output(
                        properties,
                        profile,
                        &computed,
                        node_id,
                        ancestor_hidden,
                        source_math,
                        source_border_widths,
                    )?;
                    subtree_hidden = ancestor_hidden
                        || output
                            .properties
                            .get("display")
                            .is_some_and(|value| value == "none");
                    elements.push(output);
                    reuse_stats.recomputed_style_elements += 1;
                }
                Some(computed)
            } else {
                let output = reuse_plan
                    .as_ref()
                    .and_then(|plan| plan.old_styles.get(&node_id))
                    .expect("재사용 대상 요소의 computed style output이 있어야 합니다")
                    .clone();
                subtree_hidden = ancestor_hidden
                    || output
                        .properties
                        .get("display")
                        .is_some_and(|value| value == "none");
                elements.push(output);
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
                    .map(|child| (child, computed.clone(), subtree_hidden)),
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
            writing_styles: writing_styles.into(),
            diagnostics,
        },
        reuse_stats,
    )))
}

fn computed_writing_style(
    computed: &style::properties::ComputedValues,
    node: NodeId,
) -> Result<(WritingMode, TextOrientation), CssCascadeError> {
    let writing_mode_css =
        computed.computed_value_to_string(PropertyDeclarationId::Longhand(LonghandId::WritingMode));
    let writing_mode = match computed.get_inherited_box().clone_writing_mode() {
        style::logical_geometry::WritingModeProperty::HorizontalTb => WritingMode::HorizontalTb,
        style::logical_geometry::WritingModeProperty::VerticalRl => WritingMode::VerticalRl,
        _ => {
            return Err(CssCascadeError::UnsupportedComputedWritingMode {
                node,
                value: writing_mode_css,
            });
        }
    };
    let text_orientation_css = computed
        .computed_value_to_string(PropertyDeclarationId::Longhand(LonghandId::TextOrientation));
    let text_orientation = match computed.get_inherited_box().clone_text_orientation() {
        style::properties::longhands::text_orientation::computed_value::T::Mixed => {
            TextOrientation::Mixed
        }
        _ => {
            return Err(CssCascadeError::UnsupportedComputedTextOrientation {
                node,
                value: text_orientation_css,
            });
        }
    };
    Ok((writing_mode, text_orientation))
}
