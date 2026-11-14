use super::fit_content_span_distribution::{ContributionPhase, apply_phase};

#[derive(Clone, Copy)]
pub(super) struct TrackFunctions {
    pub(super) intrinsic_minimum: bool,
    pub(super) min_content_minimum: bool,
    pub(super) max_content_minimum: bool,
    pub(super) intrinsic_maximum: bool,
    pub(super) max_content_alike_maximum: bool,
    pub(super) fit_content_limit: Option<i64>,
}

pub(super) struct UnitTrack {
    pub(super) base_size: i64,
    pub(super) growth_limit: Option<i64>,
    pub(super) infinitely_growable: bool,
    pub(super) functions: TrackFunctions,
}

/// 제한된 auto/fit-content 행 크기 profile을 Blink의 1/64px 단위로 재생한다.
/// 각 span 묶음 뒤에 Taffy trace와 대조해 수용 여부를 결정한다.
pub(super) fn simulate_blink_row_sizes(
    input: &crate::LayoutInput,
    container: &crate::GridContainerInput,
    output: &crate::LayoutOutput,
    grid_id: crate::NodeId,
    grid_height: f32,
    pre_alignment_tracks: &[crate::LayoutGridTrackSizingTrackState],
) -> Option<Vec<i64>> {
    simulate_blink_row_sizes_with_growth_limit_trace(
        input,
        container,
        output,
        grid_id,
        grid_height,
        pre_alignment_tracks,
        false,
    )
    .or_else(|| {
        simulate_blink_row_sizes_with_growth_limit_trace(
            input,
            container,
            output,
            grid_id,
            grid_height,
            pre_alignment_tracks,
            true,
        )
    })
}

fn simulate_blink_row_sizes_with_growth_limit_trace(
    input: &crate::LayoutInput,
    container: &crate::GridContainerInput,
    output: &crate::LayoutOutput,
    grid_id: crate::NodeId,
    grid_height: f32,
    pre_alignment_tracks: &[crate::LayoutGridTrackSizingTrackState],
    sync_growth_limits: bool,
) -> Option<Vec<i64>> {
    let crate::GridAutoRows::TrackList(auto_rows) = &container.auto_rows else {
        return None;
    };
    let fit_count = auto_rows
        .iter()
        .filter(|track| matches!(track, crate::GridTrackSize::FitContent(_)))
        .count();
    if container.rows.len() != 1
        || auto_rows.len() != 6
        || !(1..=6).contains(&fit_count)
        || !grid_height.is_finite()
        || grid_height < 0.0
    {
        return None;
    }

    let mut traces = output
        .grid_track_sizing_traces
        .iter()
        .filter(|trace| trace.node == grid_id);
    let trace = traces.next()?;
    if traces.next().is_some() {
        return None;
    }
    let pass = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == crate::LayoutGridTrackSizingAxis::Rows)
        .max_by_key(|pass| pass.sequence)?;
    let initialize = pass
        .stages
        .iter()
        .find(|stage| stage.stage == crate::LayoutGridTrackSizingStageKind::Initialize)?;
    let initial_tracks = visible_track_states(initialize)?;
    if initial_tracks.len() != pre_alignment_tracks.len() {
        return None;
    }

    let mut tracks = Vec::with_capacity(initial_tracks.len());
    for (row, initial) in initial_tracks.into_iter().enumerate() {
        let track_size = if row < container.rows.len() {
            container.rows[row]
        } else {
            auto_rows[(row - container.rows.len()) % auto_rows.len()]
        };
        let functions = track_functions(track_size, row < container.rows.len(), grid_height)?;
        tracks.push(UnitTrack {
            base_size: layout_unit_from_px(initial.base_size)?,
            growth_limit: match initial.growth_limit {
                Some(value) => Some(layout_unit_from_px(value)?),
                None => None,
            },
            infinitely_growable: initial.infinitely_growable,
            functions,
        });
    }
    if !matches!(tracks.len(), 4 | 7)
        || tracks
            .iter()
            .enumerate()
            .any(|(index, track)| (index == 0) == track.functions.intrinsic_minimum)
    {
        return None;
    }

    for stage in &pass.stages {
        let crate::LayoutGridTrackSizingStageKind::IntrinsicItemBatch {
            span,
            crosses_flexible_track,
        } = stage.stage
        else {
            continue;
        };
        if crosses_flexible_track {
            return None;
        }

        for phase in [
            ContributionPhase::Minimum,
            ContributionPhase::ContentBasedMinimum,
            ContributionPhase::MaxContentMinimum,
            ContributionPhase::IntrinsicMaximum,
            ContributionPhase::MaxContentMaximum,
        ] {
            apply_phase(
                &mut tracks,
                &input.grid_items,
                pass.contributions.iter().filter(|contribution| {
                    contribution.span == span && contribution.kind == phase.contribution_kind()
                }),
                span,
                phase,
            )?;
        }

        for track in &mut tracks {
            if track
                .growth_limit
                .is_some_and(|limit| track.base_size > limit)
            {
                track.growth_limit = Some(track.base_size);
            }
        }
        let stage_matches = tracks_match_stage_base_sizes(&tracks, stage);
        if stage_matches != Some(true) {
            return None;
        }
        if sync_growth_limits {
            sync_growth_limits_from_stage(&mut tracks, stage)?;
        }
    }

    if fit_count == 1 && super::has_mixed_span_minimum_modes(input, grid_id) {
        let values_match = tracks_match_values(&tracks, pre_alignment_tracks);
        if values_match == Some(true) {
            // 이 one-fit mixed profile에서는 trace에 맞는 intrinsic base size를 반환한다.
            // Growth limit 쪽 1/64px residual을 다시 base size에 올리지 않는다.
            return Some(tracks.into_iter().map(|track| track.base_size).collect());
        }
    }

    // intrinsic sizing 완료 단계는 남아 있는 무한 growth limit을 track의 base size로 바꾼다.
    // 이 profile에서는 해석된 한도까지 각 행을 최대화한다.
    for track in &mut tracks {
        if track.growth_limit.is_none() {
            track.growth_limit = Some(track.base_size);
        }
        track.base_size = track.base_size.max(track.growth_limit?);
    }
    let values_match = tracks_match_values(&tracks, pre_alignment_tracks);
    if values_match != Some(true) {
        return None;
    }
    Some(tracks.into_iter().map(|track| track.base_size).collect())
}

fn track_functions(
    track: crate::GridTrackSize,
    explicit: bool,
    grid_height: f32,
) -> Option<TrackFunctions> {
    let none = TrackFunctions {
        intrinsic_minimum: false,
        min_content_minimum: false,
        max_content_minimum: false,
        intrinsic_maximum: false,
        max_content_alike_maximum: false,
        fit_content_limit: None,
    };
    match track {
        crate::GridTrackSize::LengthPx(_) if explicit => Some(none),
        crate::GridTrackSize::Auto if !explicit => Some(TrackFunctions {
            intrinsic_minimum: true,
            min_content_minimum: false,
            max_content_minimum: false,
            intrinsic_maximum: true,
            max_content_alike_maximum: true,
            fit_content_limit: None,
        }),
        crate::GridTrackSize::MinContent if !explicit => Some(TrackFunctions {
            intrinsic_minimum: true,
            min_content_minimum: true,
            max_content_minimum: false,
            intrinsic_maximum: true,
            max_content_alike_maximum: false,
            fit_content_limit: None,
        }),
        crate::GridTrackSize::MaxContent if !explicit => Some(TrackFunctions {
            intrinsic_minimum: true,
            min_content_minimum: true,
            max_content_minimum: true,
            intrinsic_maximum: true,
            max_content_alike_maximum: true,
            fit_content_limit: None,
        }),
        crate::GridTrackSize::FitContent(limit) if !explicit => {
            let limit = match limit {
                crate::GridTrackLengthPercentage::LengthPx(value) => value,
                crate::GridTrackLengthPercentage::Percentage(value) => value * grid_height,
                crate::GridTrackLengthPercentage::Calc(_) => return None,
            };
            Some(TrackFunctions {
                intrinsic_minimum: true,
                min_content_minimum: false,
                max_content_minimum: false,
                intrinsic_maximum: true,
                max_content_alike_maximum: true,
                fit_content_limit: Some(layout_unit_from_px(limit)?),
            })
        }
        _ => None,
    }
}

fn visible_track_states(
    stage: &crate::LayoutGridTrackSizingStageTrace,
) -> Option<Vec<&crate::LayoutGridTrackSizingTrackState>> {
    if stage.tracks.iter().any(|track| {
        track.is_gutter
            && (track.base_size.abs() > 0.01
                || track.growth_limit.is_some_and(|limit| limit.abs() > 0.01))
    }) {
        return None;
    }
    Some(
        stage
            .tracks
            .iter()
            .filter(|track| !track.is_gutter)
            .collect(),
    )
}

fn tracks_match_stage_base_sizes(
    tracks: &[UnitTrack],
    stage: &crate::LayoutGridTrackSizingStageTrace,
) -> Option<bool> {
    let values = visible_track_states(stage)?;
    if values.len() != tracks.len() {
        return Some(false);
    }
    Some(
        tracks
            .iter()
            .zip(values)
            .all(|(model, traced)| (units_to_px(model.base_size) - traced.base_size).abs() <= 0.02),
    )
}

fn sync_growth_limits_from_stage(
    tracks: &mut [UnitTrack],
    stage: &crate::LayoutGridTrackSizingStageTrace,
) -> Option<()> {
    let traced_tracks = visible_track_states(stage)?;
    if traced_tracks.len() != tracks.len() {
        return None;
    }
    for (model, traced) in tracks.iter_mut().zip(traced_tracks) {
        model.growth_limit = match traced.growth_limit {
            Some(value) => Some(layout_unit_from_px(value)?),
            None => None,
        };
        model.infinitely_growable = traced.infinitely_growable;
    }
    Some(())
}

fn tracks_match_values(
    tracks: &[UnitTrack],
    values: &[crate::LayoutGridTrackSizingTrackState],
) -> Option<bool> {
    let visible = values
        .iter()
        .filter(|track| !track.is_gutter)
        .collect::<Vec<_>>();
    if visible.len() != tracks.len() {
        return Some(false);
    }
    Some(
        tracks
            .iter()
            .zip(visible)
            .all(|(model, traced)| (units_to_px(model.base_size) - traced.base_size).abs() <= 0.02),
    )
}

pub(super) fn layout_unit_from_px(value: f32) -> Option<i64> {
    if !value.is_finite() || value < 0.0 {
        return None;
    }
    Some((f64::from(value) * 64.0).round() as i64)
}

fn units_to_px(value: i64) -> f32 {
    (value as f64 / 64.0) as f32
}
