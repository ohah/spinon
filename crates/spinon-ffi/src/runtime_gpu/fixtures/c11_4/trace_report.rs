use spinon_core::{TextOrientation, WritingMode};
use spinon_layout::LayoutSourceRevision;
use spinon_layout::{
    LayoutGridItemIntrinsicAxisContributionInput, LayoutGridItemIntrinsicSource,
    LayoutGridTrackSizingTrace, LayoutIntrinsicMeasurementSource, LayoutTextSnapshot,
};
use spinon_runtime::{RuntimeGridTrackSizingDiagnostics, RuntimeLayoutCompleted};

pub(crate) fn grid_track_sizing_trace_report(
    completed: &RuntimeLayoutCompleted,
) -> Result<String, String> {
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .ok_or_else(|| "C11.4 layout diagnostic snapshot이 없습니다".to_owned())?;
    if diagnostics.traces.is_empty() {
        return Err("C11.4 Grid track-sizing trace가 비어 있습니다".to_owned());
    }

    let source_revision_matches = match diagnostics.revision.source() {
        LayoutSourceRevision::HostDocument {
            generation,
            document,
            render_tree,
        } => {
            generation.get() == completed.key.generation
                && document.get() == completed.key.document_revision
                && render_tree.get() == completed.key.render_tree_revision
        }
        LayoutSourceRevision::Tree(_) => false,
    };
    if !source_revision_matches
        || diagnostics.revision.style() != completed.key.style_revision
        || diagnostics.revision.environment().get() != completed.key.environment_revision
    {
        return Err("C11.4 trace revision이 완료된 runtime key와 다릅니다".to_owned());
    }
    if diagnostics
        .intrinsic_measurements
        .iter()
        .any(|measurement| measurement.revision != diagnostics.revision)
        || diagnostics
            .item_inputs
            .iter()
            .any(|input| input.revision != diagnostics.revision)
        || diagnostics.item_text_layouts.iter().any(|text_layout| {
            !text_layout_revision_matches(text_layout.revision, diagnostics.revision)
        })
    {
        let measurement_revisions = diagnostics
            .intrinsic_measurements
            .iter()
            .map(|measurement| measurement.revision)
            .collect::<Vec<_>>();
        let item_revisions = diagnostics
            .item_inputs
            .iter()
            .map(|input| input.revision)
            .collect::<Vec<_>>();
        let text_layout_revisions = diagnostics
            .item_text_layouts
            .iter()
            .map(|text_layout| text_layout.revision)
            .collect::<Vec<_>>();
        return Err(format!(
            "C11.4 intrinsic input revision이 trace revision과 다릅니다: trace={:?}, measurements={measurement_revisions:?}, items={item_revisions:?}, text_layouts={text_layout_revisions:?}",
            diagnostics.revision
        ));
    }
    validate_orthogonal_registry_identity(diagnostics)?;
    let item_frame_by_node = diagnostics
        .item_frames
        .iter()
        .map(|(node, frame)| (*node, *frame))
        .collect::<std::collections::BTreeMap<_, _>>();
    if item_frame_by_node.len() != diagnostics.item_frames.len()
        || diagnostics.item_inputs.len() != diagnostics.item_frames.len()
        || diagnostics
            .item_inputs
            .iter()
            .any(|input| !item_frame_by_node.contains_key(&input.node))
    {
        return Err("C11.4 Grid item frame 집합이 intrinsic input과 다릅니다".to_owned());
    }
    let item_text_layout_by_node = diagnostics
        .item_text_layouts
        .iter()
        .map(|text_layout| (text_layout.input.node, text_layout))
        .collect::<std::collections::BTreeMap<_, _>>();
    if item_text_layout_by_node.len() != diagnostics.item_text_layouts.len()
        || diagnostics.item_text_layouts.iter().any(|text_layout| {
            text_layout.input.node != text_layout.layout.paragraph
                || !item_frame_by_node.contains_key(&text_layout.input.node)
        })
    {
        return Err("C11.4 Grid item text layout 소유자 집합이 올바르지 않습니다".to_owned());
    }

    let measurements = diagnostics
        .intrinsic_measurements
        .iter()
        .map(|measurement| {
            let source = match &measurement.source {
                LayoutIntrinsicMeasurementSource::Text {
                    font_registry_revision,
                } => serde_json::json!({
                    "kind": "text",
                    "fontRegistryRevision": font_registry_revision.0,
                }),
                LayoutIntrinsicMeasurementSource::OrthogonalText {
                    min_content,
                    max_content,
                    child_writing_mode,
                    parent_writing_mode,
                    text_orientation,
                    font_registry_revision,
                    font_registry_identity: _,
                } => orthogonal_text_source_json(OrthogonalTextSourceFields {
                    min_content_inline: min_content.inline_px,
                    min_content_block: min_content.block_px,
                    max_content_inline: max_content.inline_px,
                    max_content_block: max_content.block_px,
                    child_writing_mode: *child_writing_mode,
                    parent_writing_mode: *parent_writing_mode,
                    text_orientation: *text_orientation,
                    font_registry_revision: font_registry_revision.0,
                }),
                LayoutIntrinsicMeasurementSource::Replaced { natural } => {
                    serde_json::json!({
                        "kind": "replaced",
                        "natural": {
                            "width": natural.width,
                            "height": natural.height,
                            "aspectRatio": natural.aspect_ratio,
                        },
                    })
                }
            };
            serde_json::json!({
                "node": measurement.node.get(),
                "source": source,
                "availableSpace": format!("{:?}", measurement.available_space),
                "knownDimensions": {
                    "width": measurement.known_dimensions.width,
                    "height": measurement.known_dimensions.height,
                },
                "parentSize": {
                    "width": measurement.parent_size.width,
                    "height": measurement.parent_size.height,
                },
                "contentSize": {
                    "width": measurement.content_size.width,
                    "height": measurement.content_size.height,
                },
            })
        })
        .collect::<Vec<_>>();
    let item_inputs = diagnostics
        .item_inputs
        .iter()
        .map(|input| {
            let contribution_input = input
                .contribution_input()
                .ok()
                .map(|contribution| {
                    serde_json::json!({
                        "width": axis_contribution_json(contribution.width),
                        "height": axis_contribution_json(contribution.height),
                    })
                })
                .unwrap_or(serde_json::Value::Null);
            let source = match &input.source {
                LayoutGridItemIntrinsicSource::Text { sizes } => serde_json::json!({
                    "kind": "text",
                    "minContent": sizes.min_content(),
                    "maxContent": sizes.max_content(),
                    "minContentHeight": sizes.min_content_height(),
                    "maxContentHeight": sizes.max_content_height(),
                    "fontRegistryRevision": sizes.font_registry_revision().0,
                }),
                LayoutGridItemIntrinsicSource::OrthogonalText {
                    min_content,
                    max_content,
                    child_writing_mode,
                    parent_writing_mode,
                    text_orientation,
                    font_registry_revision,
                    font_registry_identity: _,
                } => orthogonal_text_source_json(OrthogonalTextSourceFields {
                    min_content_inline: min_content.inline_px,
                    min_content_block: min_content.block_px,
                    max_content_inline: max_content.inline_px,
                    max_content_block: max_content.block_px,
                    child_writing_mode: *child_writing_mode,
                    parent_writing_mode: *parent_writing_mode,
                    text_orientation: *text_orientation,
                    font_registry_revision: font_registry_revision.0,
                }),
                LayoutGridItemIntrinsicSource::Replaced { natural } => {
                    serde_json::json!({
                        "kind": "replaced",
                        "natural": {
                            "width": natural.width,
                            "height": natural.height,
                            "aspectRatio": natural.aspect_ratio,
                        },
                    })
                }
            };
            serde_json::json!({
                "grid": input.grid_container.get(),
                "node": input.node.get(),
                "sourceOrder": input.source_order,
                "source": source,
                "textSources": input.text_sources.iter().map(|source| {
                    serde_json::json!({
                        "node": source.node.get(),
                        "text": source.text,
                    })
                }).collect::<Vec<_>>(),
                "contributionInput": contribution_input,
            })
        })
        .collect::<Vec<_>>();
    let item_text_layouts = diagnostics
        .item_text_layouts
        .iter()
        .map(item_text_layout_json)
        .collect::<Vec<_>>();
    let item_frames = diagnostics
        .item_frames
        .iter()
        .map(|(node, frame)| {
            serde_json::json!({
                "node": node.get(),
                "x": frame.x,
                "y": frame.y,
                "width": frame.width,
                "height": frame.height,
            })
        })
        .collect::<Vec<_>>();
    let traces = diagnostics
        .traces
        .iter()
        .map(track_trace_json)
        .collect::<Vec<_>>();
    let fit_content_auto_neighbor =
        super::fit_content_auto_neighbor_report::report_json(completed)?;
    let fit_content_min_height = super::fit_content_min_height_report::report_json(completed)?;
    let fit_content_percentage_argument =
        super::fit_content_percentage_argument_report::report_json(completed)?;
    let fit_content_implicit_column =
        super::fit_content_implicit_column_report::report_json(completed)?;
    let fit_content_zoomed_grid = super::fit_content_zoomed_grid_report::report_json(completed)?;
    let fit_content_auto_min_floor =
        super::fit_content_auto_min_floor_report::report_json(completed)?;
    let fit_content_spanning = super::fit_content_spanning_report::report_json(completed)?;
    let fit_content_limit_boundaries =
        super::fit_content_limit_boundaries_report::report_json(completed)?;
    let fit_content_limit_columns =
        super::fit_content_limit_columns_report::report_json(completed)?;
    let grid_layout_auto_tracks = super::grid_layout_auto_tracks_report::report_json(completed)?;
    let overflow_auto_minimum =
        if let Some(observation) = diagnostics.c11_4_overflow_auto_minimum_fixture.as_ref() {
            let item_style = observation
                .style_snapshot
                .elements
                .iter()
                .find(|style| style.node_id == observation.item)
                .ok_or_else(|| "C11.4 overflow item computed style이 없습니다".to_owned())?;
            let columns = diagnostics
                .traces
                .iter()
                .find(|trace| trace.node == observation.grid_root)
                .and_then(|trace| {
                    trace
                        .passes
                        .iter()
                        .find(|pass| pass.axis == spinon_layout::LayoutGridTrackSizingAxis::Columns)
                })
                .and_then(|pass| pass.stages.last())
                .ok_or_else(|| "C11.4 overflow Columns 최종 track stage가 없습니다".to_owned())?;
            let actual_track_base_sizes = columns
                .tracks
                .iter()
                .filter(|track| !track.is_gutter && !track.is_collapsed)
                .map(|track| track.base_size)
                .collect::<Vec<_>>();
            let root_frame = completed
                .frames
                .iter()
                .find(|frame| frame.node_id == observation.grid_root.get())
                .ok_or_else(|| "C11.4 overflow root frame이 없습니다".to_owned())?;
            let item_frame = completed
                .frames
                .iter()
                .find(|frame| frame.node_id == observation.item.get())
                .ok_or_else(|| "C11.4 overflow item frame이 없습니다".to_owned())?;
            Some(serde_json::json!({
                "sceneId": observation.scene_id.as_str(),
                "authoredOverflowX": observation.authored_overflow_x.as_str(),
                "computedOverflowX": computed_overflow_name(observation.computed_overflow_x),
                "computedOverflowY": computed_overflow_name(observation.computed_overflow_y),
                "actualComputedOverflowX": item_style.layout_overflow_x.map(computed_overflow_name),
                "actualComputedOverflowY": item_style.layout_overflow_y.map(computed_overflow_name),
                "gridRoot": observation.grid_root.get(),
                "item": observation.item.get(),
                "layoutInputColumns": format!("{:?}", observation.grid_container_input.columns),
                "layoutInputRows": format!("{:?}", observation.grid_container_input.rows),
                "placementMode": format!("{:?}", observation.grid_container_input.placement_mode),
                "expectedTrackBaseSizes": observation.expected_track_base_sizes,
                "actualTrackBaseSizes": actual_track_base_sizes,
                "expectedItemWidth": observation.expected_item_width,
                "actualItemWidth": item_frame.width,
                "expectedMinimumValue": observation.expected_minimum_value,
                "expectsContentBasedMinimum": observation.expects_content_based_minimum,
                "rootFrame": {
                    "x": root_frame.x,
                    "y": root_frame.y,
                    "width": root_frame.width,
                    "height": root_frame.height,
                },
                "itemFrame": {
                    "x": item_frame.x,
                    "y": item_frame.y,
                    "width": item_frame.width,
                    "height": item_frame.height,
                },
            }))
        } else {
            None
        };
    let single_track_overflow_auto_minimum = if let Some(observation) = diagnostics
        .c11_4_single_track_overflow_auto_minimum_fixture
        .as_ref()
    {
        let item_style = observation
            .style_snapshot
            .elements
            .iter()
            .find(|style| style.node_id == observation.item)
            .ok_or_else(|| {
                "C11.4 single-track overflow item computed style이 없습니다".to_owned()
            })?;
        let axis_pass = diagnostics
            .traces
            .iter()
            .find(|trace| trace.node == observation.grid_root)
            .and_then(|trace| {
                trace
                    .passes
                    .iter()
                    .find(|pass| pass.axis == observation.axis)
            })
            .ok_or_else(|| "C11.4 single-track overflow target pass가 없습니다".to_owned())?;
        let actual_track_base_size = axis_pass
            .stages
            .last()
            .ok_or_else(|| "C11.4 single-track overflow final stage가 없습니다".to_owned())?
            .tracks
            .iter()
            .find(|track| !track.is_gutter && !track.is_collapsed)
            .map(|track| track.base_size)
            .ok_or_else(|| "C11.4 single-track overflow used track가 없습니다".to_owned())?;
        let root_frame = completed
            .frames
            .iter()
            .find(|frame| frame.node_id == observation.grid_root.get())
            .ok_or_else(|| "C11.4 single-track overflow root frame이 없습니다".to_owned())?;
        let item_frame = completed
            .frames
            .iter()
            .find(|frame| frame.node_id == observation.item.get())
            .ok_or_else(|| "C11.4 single-track overflow item frame이 없습니다".to_owned())?;
        Some(serde_json::json!({
            "sceneId": observation.scene_id,
            "axis": format!("{:?}", observation.axis),
            "authoredOverflowX": observation.authored_overflow_x,
            "authoredOverflowY": observation.authored_overflow_y,
            "computedOverflowX": computed_overflow_name(observation.computed_overflow_x),
            "computedOverflowY": computed_overflow_name(observation.computed_overflow_y),
            "actualComputedOverflowX": item_style.layout_overflow_x.map(computed_overflow_name),
            "actualComputedOverflowY": item_style.layout_overflow_y.map(computed_overflow_name),
            "gridRoot": observation.grid_root.get(),
            "item": observation.item.get(),
            "placementMode": format!("{:?}", observation.grid_container_input.placement_mode),
            "layoutInputColumns": format!("{:?}", observation.grid_container_input.columns),
            "layoutInputRows": format!("{:?}", observation.grid_container_input.rows),
            "expectedTrackBaseSize": observation.expected_track_base_size,
            "actualTrackBaseSize": actual_track_base_size,
            "expectedItemWidth": observation.expected_item_width,
            "actualItemWidth": item_frame.width,
            "expectedItemHeight": observation.expected_item_height,
            "actualItemHeight": item_frame.height,
            "expectedMinimumValue": observation.expected_minimum_value,
            "expectedMinimumSource": format!("{:?}", observation.expected_minimum_source),
            "generation": observation.style_snapshot.generation.get(),
            "documentRevision": observation.style_snapshot.document_revision.get(),
            "renderTreeRevision": observation.style_snapshot.render_tree_revision.get(),
            "styleRevision": observation.style_snapshot.style_revision.get(),
            "rootFrame": {
                "x": root_frame.x,
                "y": root_frame.y,
                "width": root_frame.width,
                "height": root_frame.height,
            },
            "itemFrame": {
                "x": item_frame.x,
                "y": item_frame.y,
                "width": item_frame.width,
                "height": item_frame.height,
            },
        }))
    } else {
        None
    };
    let report = serde_json::json!({
        "layoutInputRevision": {
            "source": format!("{:?}", diagnostics.revision.source()),
            "style": diagnostics.revision.style().get(),
            "environment": diagnostics.revision.environment().get(),
            "intrinsicSize": diagnostics.revision.intrinsic_size().get(),
        },
        "intrinsicMeasurements": measurements,
        "gridItemIntrinsicInputs": item_inputs,
        "gridItemTextLayouts": item_text_layouts,
        "gridItemFrames": item_frames,
        "trackSizingTraces": traces,
        "fitContentAutoNeighbor": fit_content_auto_neighbor,
        "fitContentMinHeight": fit_content_min_height,
        "fitContentPercentageArgument": fit_content_percentage_argument,
        "fitContentImplicitColumn": fit_content_implicit_column,
        "fitContentZoomedGrid": fit_content_zoomed_grid,
        "fitContentAutoMinFloor": fit_content_auto_min_floor,
        "fitContentSpanning": fit_content_spanning,
        "fitContentLimitBoundaries": fit_content_limit_boundaries,
        "fitContentLimitColumns": fit_content_limit_columns,
        "gridLayoutAutoTracks": grid_layout_auto_tracks,
        "overflowAutomaticMinimum": overflow_auto_minimum,
        "singleTrackOverflowAutoMinimum": single_track_overflow_auto_minimum,
    });
    serde_json::to_string(&report)
        .map(|json| format!(" grid_track_sizing_diagnostics={json}"))
        .map_err(|error| format!("C11.4 trace JSON 변환 실패: {error}"))
}

fn computed_overflow_name(value: spinon_style::ComputedCssOverflow) -> &'static str {
    match value {
        spinon_style::ComputedCssOverflow::Visible => "visible",
        spinon_style::ComputedCssOverflow::Clip => "clip",
        spinon_style::ComputedCssOverflow::Hidden => "hidden",
        spinon_style::ComputedCssOverflow::Scroll => "scroll",
        spinon_style::ComputedCssOverflow::Auto => "auto",
    }
}

fn text_layout_revision_matches(
    text_layout: spinon_layout::LayoutInputRevision,
    diagnostics: spinon_layout::LayoutInputRevision,
) -> bool {
    text_layout.source() == diagnostics.source()
        && text_layout.style() == diagnostics.style()
        && text_layout.environment() == diagnostics.environment()
}

fn axis_contribution_json(
    input: LayoutGridItemIntrinsicAxisContributionInput,
) -> serde_json::Value {
    serde_json::json!({
        "minContent": input.min_content,
        "maxContent": input.max_content,
        "preferredSize": layout_dimension_json(input.preferred_size),
        "minSize": layout_dimension_json(input.min_size),
        "maxSize": layout_dimension_json(input.max_size),
        "marginStart": layout_length_percentage_json(input.margin_start),
        "marginEnd": layout_length_percentage_json(input.margin_end),
        "paddingStart": layout_length_percentage_json(input.padding_start),
        "paddingEnd": layout_length_percentage_json(input.padding_end),
        "borderStart": input.border_start,
        "borderEnd": input.border_end,
    })
}

fn layout_dimension_json(value: spinon_layout::LayoutDimension) -> serde_json::Value {
    match value {
        spinon_layout::LayoutDimension::Auto => serde_json::json!({ "kind": "auto" }),
        spinon_layout::LayoutDimension::Fixed(css_px) => {
            serde_json::json!({ "kind": "fixed", "cssPx": css_px })
        }
        spinon_layout::LayoutDimension::Percent(fraction) => {
            serde_json::json!({ "kind": "percent", "fraction": fraction })
        }
        spinon_layout::LayoutDimension::CyclicMinContentPercent(fraction) => {
            serde_json::json!({ "kind": "cyclicMinContentPercent", "fraction": fraction })
        }
        spinon_layout::LayoutDimension::MinContent => {
            serde_json::json!({ "kind": "minContent" })
        }
        spinon_layout::LayoutDimension::MaxContent => {
            serde_json::json!({ "kind": "maxContent" })
        }
        spinon_layout::LayoutDimension::FitContent => {
            serde_json::json!({ "kind": "fitContent" })
        }
        spinon_layout::LayoutDimension::Calc(id) => {
            serde_json::json!({ "kind": "calc", "id": format!("{id:?}") })
        }
    }
}

fn layout_length_percentage_json(
    value: spinon_layout::LayoutLengthPercentage,
) -> serde_json::Value {
    match value {
        spinon_layout::LayoutLengthPercentage::Auto => serde_json::json!({ "kind": "auto" }),
        spinon_layout::LayoutLengthPercentage::LengthPx(css_px) => {
            serde_json::json!({ "kind": "length", "cssPx": css_px })
        }
        spinon_layout::LayoutLengthPercentage::Percentage(fraction) => {
            serde_json::json!({ "kind": "percent", "fraction": fraction })
        }
        spinon_layout::LayoutLengthPercentage::Calc(id) => {
            serde_json::json!({ "kind": "calc", "id": format!("{id:?}") })
        }
    }
}

fn item_text_layout_json(snapshot: &LayoutTextSnapshot) -> serde_json::Value {
    let style = &snapshot.input.style;
    serde_json::json!({
        "node": snapshot.input.node.get(),
        "layoutParagraph": snapshot.layout.paragraph.get(),
        "revision": {
            "source": format!("{:?}", snapshot.revision.source()),
            "style": snapshot.revision.style().get(),
            "environment": snapshot.revision.environment().get(),
            "intrinsicSize": snapshot.revision.intrinsic_size().get(),
        },
        "sources": snapshot.input.sources.iter().map(|source| {
            serde_json::json!({
                "node": source.node.get(),
                "text": source.text,
                "inlineAncestors": source.inline_ancestors.iter()
                    .map(|ancestor| ancestor.get()).collect::<Vec<_>>(),
            })
        }).collect::<Vec<_>>(),
        "text": snapshot.layout.text,
        "paragraphStyle": {
            "fontFamily": style.font_family,
            "fontSize": style.font_size,
            "lineHeight": style.line_height,
            "lineHeightIsNormal": style.line_height_is_normal,
            "fontWeight": style.font_weight,
            "fontStyle": format!("{:?}", style.font_style),
            "whiteSpace": format!("{:?}", style.white_space),
            "wordBreak": format!("{:?}", style.word_break),
            "direction": format!("{:?}", style.direction),
            "writingMode": format!("{:?}", style.writing_mode),
            "textOrientation": format!("{:?}", style.text_orientation),
            "lineBreak": format!("{:?}", style.line_break),
            "ligatures": format!("{:?}", style.ligatures),
        },
        "fontRegistryRevision": snapshot.layout.font_registry_revision.0,
        "availableWidth": snapshot.layout.available_width,
        "width": snapshot.layout.width,
        "height": snapshot.layout.height,
        "lines": snapshot.layout.lines.iter().map(|line| {
            serde_json::json!({
                "advance": line.advance,
                "lineTop": line.line_top,
                "lineHeight": line.line_height,
                "baseline": line.baseline,
            })
        }).collect::<Vec<_>>(),
    })
}

fn validate_orthogonal_registry_identity(
    diagnostics: &RuntimeGridTrackSizingDiagnostics,
) -> Result<(), String> {
    let first_registry_identity =
        diagnostics
            .intrinsic_measurements
            .iter()
            .find_map(|measurement| match &measurement.source {
                LayoutIntrinsicMeasurementSource::OrthogonalText {
                    font_registry_identity,
                    ..
                } => Some(font_registry_identity),
                _ => None,
            });
    for measurement in diagnostics.intrinsic_measurements.iter() {
        let LayoutIntrinsicMeasurementSource::OrthogonalText {
            min_content,
            max_content,
            child_writing_mode,
            parent_writing_mode,
            text_orientation,
            font_registry_revision,
            font_registry_identity,
        } = &measurement.source
        else {
            continue;
        };
        if font_registry_identity.paragraph() != measurement.node
            || font_registry_identity.font_registry_revision() != *font_registry_revision
            || font_registry_identity.writing_mode() != *child_writing_mode
            || font_registry_identity.text_orientation() != *text_orientation
            || font_registry_identity.min_content() != *min_content
            || font_registry_identity.max_content() != *max_content
            || *parent_writing_mode != WritingMode::HorizontalTb
        {
            return Err(
                "C11.4 orthogonal measure의 논리 입력과 font registry identity가 다릅니다"
                    .to_owned(),
            );
        }
        if first_registry_identity.is_some_and(|identity| {
            !identity.uses_same_font_registry_revision(font_registry_identity)
        }) {
            return Err(
                "C11.4 orthogonal measure가 서로 다른 font registry token을 사용합니다".to_owned(),
            );
        }
        let item = diagnostics
            .item_inputs
            .iter()
            .find(|item| item.node == measurement.node)
            .ok_or_else(|| {
                "C11.4 orthogonal measure에 대응하는 item input이 없습니다".to_owned()
            })?;
        let LayoutGridItemIntrinsicSource::OrthogonalText {
            font_registry_identity: item_identity,
            ..
        } = &item.source
        else {
            return Err("C11.4 orthogonal measure의 item source가 text가 아닙니다".to_owned());
        };
        if !font_registry_identity.uses_same_font_registry_revision(item_identity) {
            return Err(
                "C11.4 orthogonal measure와 item input의 font registry token이 다릅니다".to_owned(),
            );
        }
        let contribution = item
            .contribution_input()
            .map_err(|error| format!("C11.4 orthogonal contribution input 오류: {error}"))?;
        let LayoutGridItemIntrinsicSource::OrthogonalText {
            font_registry_identity: contribution_identity,
            ..
        } = &contribution.source
        else {
            return Err("C11.4 orthogonal contribution source가 text가 아닙니다".to_owned());
        };
        if !font_registry_identity.uses_same_font_registry_revision(contribution_identity) {
            return Err(
                "C11.4 orthogonal contribution에서 font registry token이 바뀌었습니다".to_owned(),
            );
        }
    }
    Ok(())
}

struct OrthogonalTextSourceFields {
    min_content_inline: f32,
    min_content_block: f32,
    max_content_inline: f32,
    max_content_block: f32,
    child_writing_mode: WritingMode,
    parent_writing_mode: WritingMode,
    text_orientation: TextOrientation,
    font_registry_revision: u64,
}

fn orthogonal_text_source_json(fields: OrthogonalTextSourceFields) -> serde_json::Value {
    serde_json::json!({
        "kind": "orthogonalText",
        "minContentInline": fields.min_content_inline,
        "maxContentInline": fields.max_content_inline,
        "minContentBlock": fields.min_content_block,
        "maxContentBlock": fields.max_content_block,
        "childWritingMode": format!("{:?}", fields.child_writing_mode),
        "parentWritingMode": format!("{:?}", fields.parent_writing_mode),
        "textOrientation": format!("{:?}", fields.text_orientation),
        "fontRegistryRevision": fields.font_registry_revision,
    })
}

fn track_trace_json(trace: &LayoutGridTrackSizingTrace) -> serde_json::Value {
    let passes = trace
        .passes
        .iter()
        .map(|pass| {
            let stages = pass
                .stages
                .iter()
                .map(|stage| {
                    let tracks = stage
                        .tracks
                        .iter()
                        .map(|track| {
                            serde_json::json!({
                                "internalIndex": track.internal_index,
                                "gutter": track.is_gutter,
                                "collapsed": track.is_collapsed,
                                "baseSize": track.base_size,
                                "growthLimit": track.growth_limit,
                                "infinitelyGrowable": track.infinitely_growable,
                            })
                        })
                        .collect::<Vec<_>>();
                    serde_json::json!({
                        "stage": format!("{:?}", stage.stage),
                        "tracks": tracks,
                    })
                })
                .collect::<Vec<_>>();
            let contributions = pass
                .contributions
                .iter()
                .map(|contribution| {
                    serde_json::json!({
                        "item": contribution.item.get(),
                        "kind": format!("{:?}", contribution.kind),
                        "span": contribution.span,
                        "crossesFlexibleTrack": contribution.crosses_flexible_track,
                        "value": contribution.value,
                        "minimumSource": contribution
                            .minimum_source
                            .map(|source| format!("{:?}", source)),
                    })
                })
                .collect::<Vec<_>>();
            serde_json::json!({
                "sequence": pass.sequence,
                "axis": format!("{:?}", pass.axis),
                "phase": pass.fixture_phase.map(|phase| format!("{:?}", phase)),
                "autoRepeatCount": pass.auto_repeat_count.or(pass.fixture_auto_repeat_count),
                "percentageBasis": pass.fixture_percentage_basis,
                "stages": stages,
                "contributions": contributions,
            })
        })
        .collect::<Vec<_>>();
    serde_json::json!({
        "grid": trace.node.get(),
        "usedColumns": resolved_axis_json(&trace.used_columns),
        "usedRows": resolved_axis_json(&trace.used_rows),
        "passes": passes,
    })
}

fn resolved_axis_json(
    axis: &spinon_layout::LayoutGridTrackSizingResolvedAxis,
) -> serde_json::Value {
    serde_json::json!({
        "usedTrackList": axis.used_track_list,
        "explicitTrackCount": axis.explicit_track_count,
        "negativeImplicitTrackCount": axis.negative_implicit_track_count,
        "positiveImplicitTrackCount": axis.positive_implicit_track_count,
        "collapsedTrackIndices": axis.collapsed_track_indices,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        OrthogonalTextSourceFields, grid_track_sizing_trace_report, orthogonal_text_source_json,
        text_layout_revision_matches, track_trace_json,
    };
    use spinon_core::{
        EnvironmentRevision, IntrinsicSizeRevision, NodeId, Revision, StyleRevision,
        TextOrientation, WritingMode,
    };
    use spinon_layout::{
        FontStyleMode, LayoutGridItemIntrinsicInput, LayoutGridItemIntrinsicSource,
        LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
        LayoutGridTrackSizingContributionTrace, LayoutGridTrackSizingFixturePhase,
        LayoutGridTrackSizingPassTrace, LayoutGridTrackSizingStageKind,
        LayoutGridTrackSizingStageTrace, LayoutGridTrackSizingTrace,
        LayoutGridTrackSizingTrackState, LayoutInputRevision, LayoutIntrinsicMeasureAvailableSpace,
        LayoutIntrinsicMeasureKnownDimensions, LayoutIntrinsicMeasureParentSize,
        LayoutIntrinsicMeasureSize, LayoutIntrinsicMeasureSpace, LayoutIntrinsicMeasurement,
        LayoutIntrinsicMeasurementSource, LayoutSourceRevision, LayoutStyle, LigatureMode,
        LineBreakMode, ParagraphInput, ParagraphStyle, TextIntrinsicLogicalSizes, TextLayoutEngine,
        TextParagraphDirection, TextSource, WhiteSpaceMode, WordBreakMode,
    };
    use spinon_runtime::{
        RuntimeGridTrackSizingDiagnostics, RuntimeLayoutCompleted, RuntimeUaCascadeKey,
    };
    use std::sync::Arc;

    const AHEM: &[u8] = include_bytes!("../../../../../../tests/fixtures/resources/c15/Ahem.ttf");

    #[test]
    fn text_layout_revision_ignores_only_replaced_intrinsic_size_revision() {
        let source = LayoutSourceRevision::Tree(Revision::default());
        let text_revision =
            LayoutInputRevision::new(source, StyleRevision::INITIAL, EnvironmentRevision::INITIAL);
        let mixed_trace_revision = text_revision
            .with_intrinsic_size_revision(IntrinsicSizeRevision::INITIAL.checked_next().unwrap());
        assert!(text_layout_revision_matches(
            text_revision,
            mixed_trace_revision
        ));

        let changed_style_revision = LayoutInputRevision::new(
            source,
            StyleRevision::INITIAL.checked_next().unwrap(),
            EnvironmentRevision::INITIAL,
        )
        .with_intrinsic_size_revision(IntrinsicSizeRevision::INITIAL.checked_next().unwrap());
        assert!(!text_layout_revision_matches(
            text_revision,
            changed_style_revision
        ));

        let changed_environment_revision = LayoutInputRevision::new(
            source,
            StyleRevision::INITIAL,
            EnvironmentRevision::INITIAL.checked_next().unwrap(),
        )
        .with_intrinsic_size_revision(IntrinsicSizeRevision::INITIAL.checked_next().unwrap());
        assert!(!text_layout_revision_matches(
            text_revision,
            changed_environment_revision
        ));
    }

    #[test]
    fn reports_orthogonal_text_logical_intrinsics_and_writing_styles() {
        let source = orthogonal_text_source_json(OrthogonalTextSourceFields {
            min_content_inline: 50.0,
            min_content_block: 20.0,
            max_content_inline: 80.0,
            max_content_block: 10.0,
            child_writing_mode: WritingMode::VerticalRl,
            parent_writing_mode: WritingMode::HorizontalTb,
            text_orientation: TextOrientation::Mixed,
            font_registry_revision: 7,
        });

        assert_eq!(source["kind"], "orthogonalText");
        assert_eq!(source["minContentInline"], 50.0);
        assert_eq!(source["maxContentInline"], 80.0);
        assert_eq!(source["minContentBlock"], 20.0);
        assert_eq!(source["maxContentBlock"], 10.0);
        assert_eq!(source["childWritingMode"], "VerticalRl");
        assert_eq!(source["parentWritingMode"], "HorizontalTb");
        assert_eq!(source["textOrientation"], "Mixed");
        assert_eq!(source["fontRegistryRevision"], 7);
        assert!(source.get("fontRegistryIdentity").is_none());
    }

    #[test]
    fn rejects_equal_revision_but_different_orthogonal_registry_tokens() {
        let node = NodeId::new(2).unwrap();
        let primary = orthogonal_text_identity(node);
        let independent = orthogonal_text_identity(node);
        assert_eq!(
            primary.font_registry_revision(),
            independent.font_registry_revision()
        );
        assert!(!primary.uses_same_font_registry_revision(&independent));

        let diagnostics = |item_identity: &TextIntrinsicLogicalSizes| {
            let revision = LayoutInputRevision::default();
            let item_input = LayoutGridItemIntrinsicInput {
                grid_container: NodeId::new(1).unwrap(),
                node,
                source_order: 0,
                revision,
                source: orthogonal_item_source(item_identity),
                text_sources: Vec::new(),
                item_style: LayoutStyle::default(),
            };
            let measurement = LayoutIntrinsicMeasurement {
                node,
                revision,
                source: orthogonal_measurement_source(&primary),
                item_style: LayoutStyle::default(),
                available_space: LayoutIntrinsicMeasureAvailableSpace {
                    width: LayoutIntrinsicMeasureSpace::Definite(50.0),
                    height: LayoutIntrinsicMeasureSpace::MinContent,
                },
                known_dimensions: LayoutIntrinsicMeasureKnownDimensions::default(),
                parent_size: LayoutIntrinsicMeasureParentSize::default(),
                content_size: LayoutIntrinsicMeasureSize::default(),
            };
            RuntimeGridTrackSizingDiagnostics {
                revision,
                intrinsic_measurements: vec![measurement].into(),
                item_inputs: vec![item_input].into(),
                item_text_layouts: Arc::from([]),
                item_frames: Arc::from([]),
                traces: Arc::from([]),
                fit_content_auto_neighbor: None,
                fit_content_min_height: None,
                fit_content_percentage_argument: None,
                fit_content_implicit_column: None,
                fit_content_zoomed_grid: None,
                fit_content_auto_min_floor: None,
                fit_content_spanning: None,
                fit_content_limit_boundaries: None,
                fit_content_limit_columns: None,
                grid_layout_auto_tracks: None,
                c11_4_overflow_auto_minimum_fixture: None,
                c11_4_single_track_overflow_auto_minimum_fixture: None,
            }
        };

        assert!(super::validate_orthogonal_registry_identity(&diagnostics(&primary)).is_ok());
        assert!(super::validate_orthogonal_registry_identity(&diagnostics(&independent)).is_err());
    }

    fn orthogonal_text_identity(node: NodeId) -> TextIntrinsicLogicalSizes {
        let mut engine = TextLayoutEngine::new_without_system_fonts();
        engine.register_font("Ahem", AHEM).unwrap();
        engine
            .intrinsic_logical_sizes(&ParagraphInput {
                node,
                sources: vec![TextSource {
                    node: NodeId::new(3).unwrap(),
                    inline_ancestors: Vec::new(),
                    text: "AB CDE F".to_owned(),
                }],
                style: ParagraphStyle {
                    writing_mode: WritingMode::VerticalRl,
                    text_orientation: TextOrientation::Mixed,
                    font_family: "Ahem".to_owned(),
                    font_size: 10.0,
                    line_height: 10.0,
                    font_weight: 400.0,
                    font_style: FontStyleMode::Normal,
                    white_space: WhiteSpaceMode::Normal,
                    word_break: WordBreakMode::Normal,
                    direction: TextParagraphDirection::Ltr,
                    line_break: LineBreakMode::Normal,
                    ligatures: LigatureMode::Normal,
                    ..ParagraphStyle::default()
                },
                inline_styles: Default::default(),
                inline_run_styles: Default::default(),
            })
            .unwrap()
    }

    fn orthogonal_item_source(
        identity: &TextIntrinsicLogicalSizes,
    ) -> LayoutGridItemIntrinsicSource {
        LayoutGridItemIntrinsicSource::OrthogonalText {
            min_content: identity.min_content(),
            max_content: identity.max_content(),
            child_writing_mode: identity.writing_mode(),
            parent_writing_mode: WritingMode::HorizontalTb,
            text_orientation: identity.text_orientation(),
            font_registry_revision: identity.font_registry_revision(),
            font_registry_identity: identity.clone(),
        }
    }

    fn orthogonal_measurement_source(
        identity: &TextIntrinsicLogicalSizes,
    ) -> LayoutIntrinsicMeasurementSource {
        LayoutIntrinsicMeasurementSource::OrthogonalText {
            min_content: identity.min_content(),
            max_content: identity.max_content(),
            child_writing_mode: identity.writing_mode(),
            parent_writing_mode: WritingMode::HorizontalTb,
            text_orientation: identity.text_orientation(),
            font_registry_revision: identity.font_registry_revision(),
            font_registry_identity: identity.clone(),
        }
    }

    #[test]
    fn reports_stage_track_state_and_item_contribution_values() {
        let trace = LayoutGridTrackSizingTrace {
            node: NodeId::new(7).unwrap(),
            passes: vec![LayoutGridTrackSizingPassTrace {
                sequence: 1,
                axis: LayoutGridTrackSizingAxis::Columns,
                stages: vec![LayoutGridTrackSizingStageTrace {
                    stage: LayoutGridTrackSizingStageKind::Initialize,
                    tracks: vec![LayoutGridTrackSizingTrackState {
                        internal_index: 0,
                        is_gutter: false,
                        is_collapsed: false,
                        base_size: 24.0,
                        growth_limit: Some(100.0),
                        infinitely_growable: false,
                    }],
                }],
                contributions: vec![LayoutGridTrackSizingContributionTrace {
                    item: NodeId::new(8).unwrap(),
                    kind: LayoutGridTrackSizingContributionKind::Minimum,
                    span: 1,
                    crosses_flexible_track: false,
                    value: 16.0,
                    minimum_source: None,
                }],
                fixture_phase: Some(LayoutGridTrackSizingFixturePhase::FinalDefinite),
                fixture_auto_repeat_count: Some(2),
                auto_repeat_count: Some(2),
                fixture_percentage_basis: Some(24.0),
            }],
            used_columns: Default::default(),
            used_rows: Default::default(),
        };

        let json = track_trace_json(&trace);
        assert_eq!(json["grid"], 7);
        assert_eq!(json["passes"][0]["axis"], "Columns");
        assert_eq!(json["passes"][0]["phase"], "FinalDefinite");
        assert_eq!(json["passes"][0]["autoRepeatCount"], 2);
        assert_eq!(json["passes"][0]["percentageBasis"], 24.0);
        assert_eq!(json["passes"][0]["stages"][0]["stage"], "Initialize");
        assert_eq!(
            json["passes"][0]["stages"][0]["tracks"][0]["baseSize"],
            24.0
        );
        assert_eq!(
            json["passes"][0]["stages"][0]["tracks"][0]["growthLimit"],
            100.0
        );
        assert_eq!(json["passes"][0]["contributions"][0]["item"], 8);
        assert_eq!(json["passes"][0]["contributions"][0]["value"], 16.0);
        assert_eq!(json["usedColumns"]["usedTrackList"], "");
        assert_eq!(json["usedRows"]["explicitTrackCount"], 0);
    }

    #[test]
    fn rejects_missing_grid_track_sizing_diagnostics() {
        let completed = RuntimeLayoutCompleted {
            key: RuntimeUaCascadeKey {
                generation: 1,
                document_revision: 2,
                render_tree_revision: 2,
                style_revision: StyleRevision::INITIAL,
                environment_revision: 0,
            },
            frames: Vec::new().into(),
            render_snapshot: None,
            projection_duration_us: 0,
            cache_hit: false,
            grid_track_sizing_diagnostics: None,
        };

        assert_eq!(
            grid_track_sizing_trace_report(&completed).unwrap_err(),
            "C11.4 layout diagnostic snapshot이 없습니다"
        );
    }

    #[test]
    fn rejects_grid_track_sizing_diagnostics_from_a_different_runtime_revision() {
        let diagnostics = RuntimeGridTrackSizingDiagnostics {
            revision: LayoutInputRevision::new(
                LayoutSourceRevision::Tree(Revision::default()),
                StyleRevision::INITIAL,
                EnvironmentRevision::INITIAL,
            ),
            intrinsic_measurements: Arc::from([]),
            item_inputs: Arc::from([]),
            item_text_layouts: Arc::from([]),
            item_frames: Arc::from([]),
            traces: vec![LayoutGridTrackSizingTrace {
                node: NodeId::new(7).unwrap(),
                passes: Vec::new(),
                used_columns: Default::default(),
                used_rows: Default::default(),
            }]
            .into(),
            fit_content_auto_neighbor: None,
            fit_content_min_height: None,
            fit_content_percentage_argument: None,
            fit_content_implicit_column: None,
            fit_content_zoomed_grid: None,
            fit_content_auto_min_floor: None,
            fit_content_spanning: None,
            fit_content_limit_boundaries: None,
            fit_content_limit_columns: None,
            grid_layout_auto_tracks: None,
            c11_4_overflow_auto_minimum_fixture: None,
            c11_4_single_track_overflow_auto_minimum_fixture: None,
        };
        let completed = RuntimeLayoutCompleted {
            key: RuntimeUaCascadeKey {
                generation: 1,
                document_revision: 2,
                render_tree_revision: 2,
                style_revision: StyleRevision::INITIAL,
                environment_revision: 0,
            },
            frames: Vec::new().into(),
            render_snapshot: None,
            projection_duration_us: 0,
            cache_hit: false,
            grid_track_sizing_diagnostics: Some(Arc::new(diagnostics)),
        };

        assert_eq!(
            grid_track_sizing_trace_report(&completed).unwrap_err(),
            "C11.4 trace revision이 완료된 runtime key와 다릅니다"
        );
    }
}
