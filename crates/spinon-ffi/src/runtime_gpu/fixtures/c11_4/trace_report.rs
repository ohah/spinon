use spinon_layout::LayoutSourceRevision;
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingTrace, LayoutIntrinsicMeasurementSource,
};
use spinon_runtime::RuntimeLayoutCompleted;

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

    let measurements = diagnostics
        .intrinsic_measurements
        .iter()
        .map(|measurement| {
            let source = match measurement.source {
                LayoutIntrinsicMeasurementSource::Text {
                    font_registry_revision,
                } => serde_json::json!({
                    "kind": "text",
                    "fontRegistryRevision": font_registry_revision.0,
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
            })
        })
        .collect::<Vec<_>>();
    let traces = diagnostics
        .traces
        .iter()
        .map(track_trace_json)
        .collect::<Vec<_>>();
    let report = serde_json::json!({
        "layoutInputRevision": {
            "source": format!("{:?}", diagnostics.revision.source()),
            "style": diagnostics.revision.style().get(),
            "environment": diagnostics.revision.environment().get(),
            "intrinsicSize": diagnostics.revision.intrinsic_size().get(),
        },
        "intrinsicMeasurements": measurements,
        "gridItemIntrinsicInputs": item_inputs,
        "trackSizingTraces": traces,
    });
    serde_json::to_string(&report)
        .map(|json| format!(" grid_track_sizing_diagnostics={json}"))
        .map_err(|error| format!("C11.4 trace JSON 변환 실패: {error}"))
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
    use super::{grid_track_sizing_trace_report, track_trace_json};
    use spinon_core::{NodeId, StyleRevision};
    use spinon_layout::{
        LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
        LayoutGridTrackSizingContributionTrace, LayoutGridTrackSizingPassTrace,
        LayoutGridTrackSizingStageKind, LayoutGridTrackSizingStageTrace,
        LayoutGridTrackSizingTrace, LayoutGridTrackSizingTrackState,
    };
    use spinon_runtime::{RuntimeLayoutCompleted, RuntimeUaCascadeKey};

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
            }],
        };

        let json = track_trace_json(&trace);
        assert_eq!(json["grid"], 7);
        assert_eq!(json["passes"][0]["axis"], "Columns");
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

        assert!(
            grid_track_sizing_trace_report(&completed)
                .unwrap_err()
                .contains("diagnostic snapshot")
        );
    }
}
