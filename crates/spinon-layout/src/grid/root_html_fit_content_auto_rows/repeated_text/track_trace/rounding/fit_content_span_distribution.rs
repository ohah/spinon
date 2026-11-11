use super::fit_content_span_sizing::{TrackFunctions, UnitTrack, layout_unit_from_px};

#[derive(Clone, Copy, Debug)]
pub(super) enum ContributionPhase {
    Minimum,
    ContentBasedMinimum,
    MaxContentMinimum,
    IntrinsicMaximum,
    MaxContentMaximum,
}

impl ContributionPhase {
    pub(super) const fn contribution_kind(self) -> crate::LayoutGridTrackSizingContributionKind {
        match self {
            Self::Minimum => crate::LayoutGridTrackSizingContributionKind::Minimum,
            Self::ContentBasedMinimum | Self::IntrinsicMaximum => {
                crate::LayoutGridTrackSizingContributionKind::MinContent
            }
            Self::MaxContentMinimum | Self::MaxContentMaximum => {
                crate::LayoutGridTrackSizingContributionKind::MaxContent
            }
        }
    }

    pub(super) const fn grows_growth_limit(self) -> bool {
        matches!(self, Self::IntrinsicMaximum | Self::MaxContentMaximum)
    }

    pub(super) const fn grows_limit_in_beyond_step(self) -> bool {
        matches!(
            self,
            Self::Minimum | Self::ContentBasedMinimum | Self::MaxContentMinimum
        )
    }
}

fn item_row_range(
    items: &std::collections::BTreeMap<crate::NodeId, crate::GridItemInput>,
    item: crate::NodeId,
    expected_span: u16,
    track_count: usize,
) -> Option<std::ops::Range<usize>> {
    let placement = items.get(&item)?;
    let crate::GridPlacementInput::Line(start_line) = &placement.row_start else {
        return None;
    };
    let start = usize::try_from(start_line.checked_sub(1)?).ok()?;
    let end = match &placement.row_end {
        crate::GridPlacementInput::Span(span) => start.checked_add(usize::try_from(*span).ok()?)?,
        crate::GridPlacementInput::Line(end_line) => {
            usize::try_from(end_line.checked_sub(1)?).ok()?
        }
        _ => return None,
    };
    if start >= end || end > track_count || end - start != usize::from(expected_span) {
        return None;
    }
    Some(start..end)
}

pub(super) fn apply_phase<'a>(
    tracks: &mut [UnitTrack],
    items: &std::collections::BTreeMap<crate::NodeId, crate::GridItemInput>,
    contributions: impl Iterator<Item = &'a crate::LayoutGridTrackSizingContributionTrace>,
    span: u16,
    phase: ContributionPhase,
) -> Option<()> {
    let grows_growth_limit = phase.grows_growth_limit();
    let mut planned_increases = vec![None::<i64>; tracks.len()];
    for contribution in contributions {
        let range = item_row_range(items, contribution.item, span, tracks.len())?;
        let affected = range
            .clone()
            .filter(|index| is_affected(tracks[*index].functions, phase))
            .collect::<Vec<_>>();
        if affected.is_empty() {
            continue;
        }
        let contribution_size = layout_unit_from_px(contribution.value)?;
        let spanned_size = range
            .clone()
            .map(|index| {
                if grows_growth_limit {
                    tracks[index]
                        .growth_limit
                        .unwrap_or(tracks[index].base_size)
                } else {
                    tracks[index].base_size
                }
            })
            .sum::<i64>();
        let mut extra_space = (contribution_size - spanned_size).max(0);

        let mut item_increases = vec![0_i64; tracks.len()];
        let mut distribution_order = affected.clone();
        distribution_order.sort_by(|left, right| {
            compare_potential(
                growth_potential(&tracks[*left], grows_growth_limit, false, 0),
                growth_potential(&tracks[*right], grows_growth_limit, false, 0),
            )
        });
        distribute_shares(
            tracks,
            &distribution_order,
            &mut item_increases,
            &mut extra_space,
            grows_growth_limit,
            true,
        );

        if extra_space > 0 {
            let mut beyond_order = if phase.grows_limit_in_beyond_step() {
                affected
                    .iter()
                    .copied()
                    .filter(|index| should_grow_beyond(tracks[*index].functions, phase))
                    .collect::<Vec<_>>()
            } else {
                affected.clone()
            };
            if beyond_order.is_empty() {
                beyond_order = affected.clone();
            }
            beyond_order.sort_by(|left, right| {
                compare_potential(
                    growth_potential(&tracks[*left], grows_growth_limit, false, 0),
                    growth_potential(&tracks[*right], grows_growth_limit, false, 0),
                )
            });
            distribute_shares(
                tracks,
                &beyond_order,
                &mut item_increases,
                &mut extra_space,
                grows_growth_limit,
                false,
            );
        }

        for index in affected {
            // Taffy는 영향받은 각 track에 growth limit 증가량 0도 기록한다.
            // 한도가 무한대여도 이 기록은 현재 base size로 한도를 확정한다.
            let planned = &mut planned_increases[index];
            *planned = Some(planned.unwrap_or_default().max(item_increases[index]));
        }
    }
    for (index, track) in tracks.iter_mut().enumerate() {
        let Some(increase) = planned_increases[index] else {
            track.infinitely_growable = false;
            continue;
        };
        if grows_growth_limit {
            let was_infinite = track.growth_limit.is_none();
            let previous = track.growth_limit.unwrap_or(track.base_size);
            track.growth_limit = Some(previous + increase);
            track.infinitely_growable =
                matches!(phase, ContributionPhase::IntrinsicMaximum) && was_infinite;
        } else {
            track.base_size += increase;
            track.infinitely_growable = false;
        }
    }
    Some(())
}

fn distribute_shares(
    tracks: &[UnitTrack],
    order: &[usize],
    item_increases: &mut [i64],
    extra_space: &mut i64,
    grows_growth_limit: bool,
    enforce_infinite_growth: bool,
) {
    let mut growable_count = order
        .iter()
        .filter(|index| {
            is_positive_potential(growth_potential(
                &tracks[**index],
                grows_growth_limit,
                enforce_infinite_growth,
                item_increases[**index],
            ))
        })
        .count();
    let mut share_ratio_sum = growable_count;
    for &index in order {
        if growable_count == 0 || *extra_space == 0 {
            break;
        }
        let potential = growth_potential(
            &tracks[index],
            grows_growth_limit,
            enforce_infinite_growth,
            item_increases[index],
        );
        if !is_positive_potential(potential) {
            continue;
        }
        let share = if share_ratio_sum == 1 {
            *extra_space
        } else {
            *extra_space / i64::try_from(share_ratio_sum).expect("small track count")
        };
        let share = potential.map_or(share, |limit| share.min(limit));
        item_increases[index] += share;
        *extra_space -= share;
        growable_count -= 1;
        share_ratio_sum -= 1;
    }
}

fn is_affected(functions: TrackFunctions, phase: ContributionPhase) -> bool {
    match phase {
        ContributionPhase::Minimum => functions.intrinsic_minimum,
        ContributionPhase::ContentBasedMinimum => functions.min_content_minimum,
        ContributionPhase::MaxContentMinimum => functions.max_content_minimum,
        ContributionPhase::IntrinsicMaximum => functions.intrinsic_maximum,
        ContributionPhase::MaxContentMaximum => functions.max_content_alike_maximum,
    }
}

fn should_grow_beyond(functions: TrackFunctions, phase: ContributionPhase) -> bool {
    match phase {
        ContributionPhase::Minimum | ContributionPhase::ContentBasedMinimum => {
            functions.intrinsic_maximum
        }
        ContributionPhase::MaxContentMinimum => functions.max_content_alike_maximum,
        ContributionPhase::IntrinsicMaximum | ContributionPhase::MaxContentMaximum => false,
    }
}

/// `None`은 무한 growth potential을 나타내며 유한 potential은 0 아래로 내려가지 않는다.
fn growth_potential(
    track: &UnitTrack,
    grows_growth_limit: bool,
    enforce_infinite_growth: bool,
    item_increase: i64,
) -> Option<i64> {
    if grows_growth_limit {
        if enforce_infinite_growth && track.growth_limit.is_some() && !track.infinitely_growable {
            return Some(0);
        }
        track.functions.fit_content_limit.map(|limit| {
            (limit - track.growth_limit.unwrap_or(track.base_size) - item_increase).max(0)
        })
    } else {
        track
            .growth_limit
            .map(|limit| (limit - track.base_size - item_increase).max(0))
    }
}

fn is_positive_potential(potential: Option<i64>) -> bool {
    potential.is_none_or(|value| value > 0)
}

fn compare_potential(left: Option<i64>, right: Option<i64>) -> std::cmp::Ordering {
    match (left, right) {
        (Some(left), Some(right)) => left.cmp(&right),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    }
}
