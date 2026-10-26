use spinon_core::{TextOrientation, WritingMode};
use spinon_layout::LayoutSourceRevision;
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingTrace, LayoutIntrinsicMeasurementSource,
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
    {
        return Err("C11.4 intrinsic input revision이 trace revision과 다릅니다".to_owned());
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
            })
        })
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
    let report = serde_json::json!({
        "layoutInputRevision": {
            "source": format!("{:?}", diagnostics.revision.source()),
            "style": diagnostics.revision.style().get(),
            "environment": diagnostics.revision.environment().get(),
            "intrinsicSize": diagnostics.revision.intrinsic_size().get(),
        },
        "intrinsicMeasurements": measurements,
        "gridItemIntrinsicInputs": item_inputs,
        "gridItemFrames": item_frames,
        "trackSizingTraces": traces,
        "fitContentAutoNeighbor": fit_content_auto_neighbor,
    });
    serde_json::to_string(&report)
        .map(|json| format!(" grid_track_sizing_diagnostics={json}"))
        .map_err(|error| format!("C11.4 trace JSON 변환 실패: {error}"))
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
                "autoRepeatCount": pass.fixture_auto_repeat_count,
                "percentageBasis": pass.fixture_percentage_basis,
                "stages": stages,
                "contributions": contributions,
            })
        })
        .collect::<Vec<_>>();
    serde_json::json!({
        "grid": trace.node.get(),
        "passes": passes,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        OrthogonalTextSourceFields, grid_track_sizing_trace_report, orthogonal_text_source_json,
        track_trace_json,
    };
    use spinon_core::{
        EnvironmentRevision, NodeId, Revision, StyleRevision, TextOrientation, WritingMode,
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
                item_frames: Arc::from([]),
                traces: Arc::from([]),
                fit_content_auto_neighbor: None,
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
                fixture_percentage_basis: Some(24.0),
            }],
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
            item_frames: Arc::from([]),
            traces: vec![LayoutGridTrackSizingTrace {
                node: NodeId::new(7).unwrap(),
                passes: Vec::new(),
            }]
            .into(),
            fit_content_auto_neighbor: None,
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
