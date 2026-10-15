use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{NodeId, ScrollStateRevision};

use super::{RuntimeOverscrollBehavior, RuntimeRenderError, RuntimeRenderSnapshot};

/// 한 owner가 한 sample에서 소비한 축 delta와 최종 offset입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RuntimeScrollDeltaChange {
    owner: NodeId,
    consumed_x_css_px: f32,
    consumed_y_css_px: f32,
    offset_x_css_px: f32,
    offset_y_css_px: f32,
}

impl RuntimeScrollDeltaChange {
    pub const fn owner(self) -> NodeId {
        self.owner
    }

    pub const fn consumed_x_css_px(self) -> f32 {
        self.consumed_x_css_px
    }

    pub const fn consumed_y_css_px(self) -> f32 {
        self.consumed_y_css_px
    }

    pub const fn offset_x_css_px(self) -> f32 {
        self.offset_x_css_px
    }

    pub const fn offset_y_css_px(self) -> f32 {
        self.offset_y_css_px
    }
}

/// 한 hit target에 적용한 scroll sample 결과입니다. Offset 변경이 없어도 target과 remainder를 돌려줍니다.
#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeScrollDeltaOutcome {
    target: NodeId,
    snapshot: RuntimeRenderSnapshot,
    remaining_x_css_px: f32,
    remaining_y_css_px: f32,
    changes: Vec<RuntimeScrollDeltaChange>,
}

impl RuntimeScrollDeltaOutcome {
    pub const fn target(&self) -> NodeId {
        self.target
    }

    pub const fn snapshot(&self) -> &RuntimeRenderSnapshot {
        &self.snapshot
    }

    pub const fn remaining_x_css_px(&self) -> f32 {
        self.remaining_x_css_px
    }

    pub const fn remaining_y_css_px(&self) -> f32 {
        self.remaining_y_css_px
    }

    pub fn changes(&self) -> &[RuntimeScrollDeltaChange] {
        &self.changes
    }

    pub fn into_snapshot(self) -> RuntimeRenderSnapshot {
        self.snapshot
    }
}

impl RuntimeRenderSnapshot {
    /// 현재 clip-aware hit target에서 시작해 inner-to-outer로 scroll delta를 한 번에 계산합니다.
    ///
    /// Delta는 finger movement가 아니라 scroll 방향의 CSS px입니다. `None`은 hit miss이며,
    /// 성공한 결과는 변경된 모든 owner와 새 immutable snapshot을 함께 보유합니다.
    pub fn route_scroll_delta_at(
        &self,
        expected_revision: ScrollStateRevision,
        x_css_px: f32,
        y_css_px: f32,
        delta_x_css_px: f32,
        delta_y_css_px: f32,
    ) -> Result<Option<RuntimeScrollDeltaOutcome>, RuntimeRenderError> {
        self.validate_scroll_delta(expected_revision, delta_x_css_px, delta_y_css_px)?;
        if !x_css_px.is_finite() || !y_css_px.is_finite() {
            return Err(RuntimeRenderError::InvalidScrollPoint);
        }
        let Some(target) = self.hit_test_css_point(x_css_px, y_css_px) else {
            return Ok(None);
        };
        self.route_scroll_delta_for_target(
            expected_revision,
            target.node_id(),
            delta_x_css_px,
            delta_y_css_px,
        )
        .map(Some)
    }

    /// 이미 gesture에서 고정한 target에 한 scroll sample을 계산합니다.
    pub fn route_scroll_delta_for_target(
        &self,
        expected_revision: ScrollStateRevision,
        target: NodeId,
        delta_x_css_px: f32,
        delta_y_css_px: f32,
    ) -> Result<RuntimeScrollDeltaOutcome, RuntimeRenderError> {
        self.validate_scroll_delta(expected_revision, delta_x_css_px, delta_y_css_px)?;
        if !self
            .boxes()
            .iter()
            .any(|render_box| render_box.node_id() == target)
        {
            return Err(RuntimeRenderError::InvalidScrollTarget(target));
        }

        let mut owners = Vec::new();
        let mut seen = BTreeSet::new();
        if self
            .scroll_containers()
            .iter()
            .any(|container| container.owner() == target)
        {
            seen.insert(target);
            owners.push(target);
        }
        let ancestors = self
            .scroll_ancestors_for_node(target)
            .ok_or(RuntimeRenderError::InvalidScrollTarget(target))?;
        for owner in ancestors {
            if !seen.insert(*owner) {
                return Err(RuntimeRenderError::InvalidScrollState);
            }
            owners.push(*owner);
        }

        let mut offsets = BTreeMap::new();
        let mut consumed = BTreeMap::<NodeId, (f32, f32)>::new();
        let mut remaining_x = delta_x_css_px;
        let mut remaining_y = delta_y_css_px;
        let mut x_chain_stopped = false;
        let mut y_chain_stopped = false;

        for owner in &owners {
            let container = self
                .scroll_containers()
                .iter()
                .find(|container| container.owner() == *owner)
                .ok_or(RuntimeRenderError::InvalidScrollState)?;
            let policy = container.input_policy();
            let mut next_x = container.offset_x_css_px();
            let mut next_y = container.offset_y_css_px();
            let mut used_x = 0.0;
            let mut used_y = 0.0;

            if policy.user_scrollable_x() && !x_chain_stopped {
                (next_x, used_x, remaining_x) =
                    consume_axis(next_x, container.max_offset_x_css_px(), remaining_x);
            }
            if policy.user_scrollable_y() && !y_chain_stopped {
                (next_y, used_y, remaining_y) =
                    consume_axis(next_y, container.max_offset_y_css_px(), remaining_y);
            }

            if used_x != 0.0 || used_y != 0.0 {
                offsets.insert(*owner, (next_x, next_y));
                consumed.insert(*owner, (used_x, used_y));
            }
            if remaining_x != 0.0 && policy.overscroll_x() != RuntimeOverscrollBehavior::Auto {
                // `contain`과 `none`은 현재 runtime의 non-local chain을 모두 막습니다.
                x_chain_stopped = true;
            }
            if remaining_y != 0.0 && policy.overscroll_y() != RuntimeOverscrollBehavior::Auto {
                // 각 축은 서로 독립적으로 chain을 끊습니다.
                y_chain_stopped = true;
            }
            if (remaining_x == 0.0 || x_chain_stopped) && (remaining_y == 0.0 || y_chain_stopped) {
                break;
            }
        }

        let updates = owners
            .iter()
            .filter_map(|owner| offsets.get(owner).copied().map(|(x, y)| (*owner, x, y)))
            .collect::<Vec<_>>();
        let changes = owners
            .iter()
            .filter_map(|owner| {
                let (consumed_x_css_px, consumed_y_css_px) = consumed.get(owner).copied()?;
                let container = self
                    .scroll_containers()
                    .iter()
                    .find(|container| container.owner() == *owner)?;
                let (offset_x_css_px, offset_y_css_px) = offsets.get(owner).copied()?;
                Some(RuntimeScrollDeltaChange {
                    owner: *owner,
                    consumed_x_css_px,
                    consumed_y_css_px,
                    offset_x_css_px: if consumed_x_css_px == 0.0 {
                        container.offset_x_css_px()
                    } else {
                        offset_x_css_px
                    },
                    offset_y_css_px: if consumed_y_css_px == 0.0 {
                        container.offset_y_css_px()
                    } else {
                        offset_y_css_px
                    },
                })
            })
            .collect::<Vec<_>>();
        let snapshot = if updates.is_empty() {
            self.clone()
        } else {
            self.with_scroll_offsets(expected_revision, &updates)?
        };

        Ok(RuntimeScrollDeltaOutcome {
            target,
            snapshot,
            remaining_x_css_px: remaining_x,
            remaining_y_css_px: remaining_y,
            changes,
        })
    }

    fn validate_scroll_delta(
        &self,
        expected_revision: ScrollStateRevision,
        delta_x_css_px: f32,
        delta_y_css_px: f32,
    ) -> Result<(), RuntimeRenderError> {
        if self.key().scroll_state_revision() != expected_revision {
            return Err(RuntimeRenderError::StaleScrollRevision);
        }
        if !delta_x_css_px.is_finite() || !delta_y_css_px.is_finite() {
            return Err(RuntimeRenderError::InvalidScrollDelta);
        }
        Ok(())
    }
}

fn consume_axis(offset: f32, max_offset: f32, requested: f32) -> (f32, f32, f32) {
    if requested == 0.0 {
        return (offset, 0.0, 0.0);
    }
    let next = (f64::from(offset) + f64::from(requested)).clamp(0.0, f64::from(max_offset)) as f32;
    let consumed = next - offset;
    (next, consumed, requested - consumed)
}
