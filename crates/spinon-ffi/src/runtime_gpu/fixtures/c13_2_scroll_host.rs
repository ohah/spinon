use super::super::{ERR_LAYOUT, ERR_STALE, RuntimeGpuHost, lock, scene_matches_layout_key};
use spinon_core::NodeId;
use spinon_render::{RuntimeRenderBox, RuntimeRenderSnapshot, RuntimeScrollContainer};
use std::sync::Arc;
use std::sync::atomic::Ordering;

mod root_body;

impl RuntimeGpuHost {
    pub(in crate::runtime_gpu) fn reapply_c13_2_scroll_state(
        &self,
        scene: RuntimeRenderSnapshot,
    ) -> Result<RuntimeRenderSnapshot, (i32, String)> {
        let Some(previous) = lock(&self.c13_2_scroll_state).clone() else {
            return Ok(scene);
        };
        if previous.key().generation() != scene.key().generation() {
            return Ok(scene);
        }

        let mut geometry_changed =
            previous.scroll_containers().len() != scene.scroll_containers().len();
        let mut requests = Vec::new();
        if self
            .c13_2_fail_next_scroll_reserve
            .swap(false, Ordering::AcqRel)
        {
            self.c13_2_reserve_failure_observed
                .store(true, Ordering::Release);
            return Err((
                ERR_LAYOUT,
                "c13_2_injected_reserve_failure: C13.2 offset 복구 예약을 주입 실패시켰습니다"
                    .to_owned(),
            ));
        }
        requests
            .try_reserve_exact(previous.scroll_containers().len())
            .map_err(|_| {
                (
                    ERR_LAYOUT,
                    "C13.2 offset 복구 공간을 확보하지 못했습니다".to_owned(),
                )
            })?;
        for old in previous.scroll_containers() {
            let Some(current) = scene
                .scroll_containers()
                .iter()
                .find(|current| current.owner() == old.owner())
            else {
                geometry_changed = true;
                continue;
            };
            geometry_changed |= !same_scroll_geometry(old, current);
            if old.parent() == current.parent() {
                requests.push((old.owner(), old.offset_x_css_px(), old.offset_y_css_px()));
            }
        }
        geometry_changed |= scene.scroll_containers().iter().any(|current| {
            !previous
                .scroll_containers()
                .iter()
                .any(|old| old.owner() == current.owner())
        });

        let restored = scene
            .with_scroll_offsets(scene.key().scroll_state_revision(), &requests)
            .map_err(|error| (ERR_LAYOUT, format!("C13.2 offset 복구 실패: {error}")))?;
        let offsets_changed = previous.scroll_containers().iter().any(|old| {
            restored
                .scroll_containers()
                .iter()
                .find(|current| current.owner() == old.owner())
                .is_none_or(|current| {
                    current.offset_x_css_px() != old.offset_x_css_px()
                        || current.offset_y_css_px() != old.offset_y_css_px()
                })
        });
        let base_revision = previous
            .key()
            .scroll_state_revision()
            .max(restored.key().scroll_state_revision());
        let revision = if geometry_changed || offsets_changed {
            base_revision.checked_next().ok_or((
                ERR_LAYOUT,
                "C13.2 scroll revision이 소진됐습니다".to_owned(),
            ))?
        } else {
            base_revision
        };
        Ok(restored.with_scroll_state_revision(revision))
    }

    pub(in crate::runtime_gpu) fn apply_c13_2_scroll_fixture_offset(
        &self,
    ) -> Result<String, (i32, String)> {
        let _operation = lock(&self.operation);
        let (sequence_before, current) = self.presentation.lock_current();
        let initial = current
            .as_ref()
            .map(|published| Arc::clone(&published.snapshot))
            .ok_or((
                ERR_LAYOUT,
                "C13.2 initial render snapshot이 없습니다".to_owned(),
            ))?;
        drop(current);

        if self.presentation_sequence() != sequence_before {
            return Err((
                ERR_STALE,
                "C13.2 scroll offset을 적용하기 전에 presentation이 바뀌었습니다".to_owned(),
            ));
        }
        if initial.scroll_containers().len() != 9 {
            return Err((
                ERR_LAYOUT,
                format!(
                    "C13.2 고정 matrix owner 수가 다릅니다: {} != 9",
                    initial.scroll_containers().len()
                ),
            ));
        }

        let layout_snapshot = self.session.layout_snapshot();
        let completed_key = layout_snapshot
            .completed
            .as_ref()
            .map(|completed| completed.key);
        let completed = layout_snapshot
            .completed
            .filter(|completed| {
                completed.render_snapshot.is_some()
                    && scene_matches_layout_key(initial.as_ref(), completed.key)
            })
            .ok_or_else(|| {
                (
                    ERR_STALE,
                    format!(
                        "C13.2 layout frame과 initial render snapshot의 layout revision이 다릅니다: scene={:?} completed={completed_key:?} requested={:?}",
                        initial.key(),
                        layout_snapshot.requested,
                    ),
                )
            })?;
        let frames = &completed.frames;
        if frames.len() != 26 {
            return Err((
                ERR_LAYOUT,
                format!("C13.2 fixture frame 수가 다릅니다: {} != 26", frames.len()),
            ));
        }
        let visible_parent_id = node_id_at(frames, 1)?;
        let border_escape_id = node_id_at(frames, 2)?;
        let scroll_content_id = node_id_at(frames, 4)?;
        let scroll_marker_id = node_id_at(frames, 5)?;
        let fixed_control_id = node_id_at(frames, 6)?;
        let expected_owners = [
            (3, 106.0, 84.0),
            (7, 0.0, 0.0),
            (9, 86.0, 0.0),
            (11, 0.0, 74.0),
            (13, 86.0, 74.0),
            (17, 64.0, 0.0),
            (20, 88.0, 78.0),
            (22, 66.0, 54.0),
            (24, 0.0, 0.0),
        ];
        for (frame_index, expected_x, expected_y) in expected_owners {
            let node = node_id_at(frames, frame_index)?;
            let owner = scroll_owner_by_node(initial.as_ref(), node).ok_or((
                ERR_LAYOUT,
                format!("C13.2 owner for fixture frame {frame_index}가 없습니다"),
            ))?;
            if owner.max_offset_x_css_px() != expected_x
                || owner.max_offset_y_css_px() != expected_y
            {
                return Err((
                    ERR_LAYOUT,
                    format!(
                        "C13.2 owner frame {frame_index} range가 다릅니다: ({}, {}) != ({expected_x}, {expected_y})",
                        owner.max_offset_x_css_px(),
                        owner.max_offset_y_css_px()
                    ),
                ));
            }
        }
        let visible_owner_id = node_id_at(frames, 1)?;
        let clip_owner_id = node_id_at(frames, 15)?;
        if scroll_owner_by_node(initial.as_ref(), visible_owner_id).is_some()
            || scroll_owner_by_node(initial.as_ref(), clip_owner_id).is_some()
        {
            return Err((
                ERR_LAYOUT,
                "C13.2 visible/clip 경계가 scroll owner를 만들었습니다".to_owned(),
            ));
        }
        let visible_parent = frame_by_id(initial.as_ref(), visible_parent_id).ok_or((
            ERR_LAYOUT,
            "C13.2 visible parent render box가 없습니다".to_owned(),
        ))?;
        let border_escape = frame_by_id(initial.as_ref(), border_escape_id).ok_or((
            ERR_LAYOUT,
            "C13.2 border escape render box가 없습니다".to_owned(),
        ))?;
        if border_escape.frame_css_px().x()
            <= visible_parent.frame_css_px().x() + visible_parent.frame_css_px().width()
        {
            return Err((
                ERR_LAYOUT,
                "C13.2 visible child가 parent border 바깥에 있지 않습니다".to_owned(),
            ));
        }
        let escape_x =
            border_escape.frame_css_px().x() + border_escape.frame_css_px().width() / 2.0;
        let escape_y =
            border_escape.frame_css_px().y() + border_escape.frame_css_px().height() / 2.0;
        if initial
            .hit_test_css_point(escape_x, escape_y)
            .is_none_or(|hit| hit.node_id() != border_escape_id)
        {
            return Err((
                ERR_LAYOUT,
                "C13.2 visible border escape paint/hit target가 initial snapshot에서 일치하지 않습니다"
                    .to_owned(),
            ));
        }

        let initial_content = frame_by_id(initial.as_ref(), scroll_content_id).ok_or((
            ERR_LAYOUT,
            "C13.2 scroll content render box가 없습니다".to_owned(),
        ))?;
        let initial_marker = frame_by_id(initial.as_ref(), scroll_marker_id).ok_or((
            ERR_LAYOUT,
            "C13.2 scroll marker render box가 없습니다".to_owned(),
        ))?;
        let initial_fixed = frame_by_id(initial.as_ref(), fixed_control_id)
            .ok_or((
                ERR_LAYOUT,
                "C13.2 fixed control render box가 없습니다".to_owned(),
            ))?
            .frame_css_px();
        let old_marker_x = initial_marker.frame_css_px().x() + 12.0;
        let old_marker_y = initial_marker.frame_css_px().y() + 10.0;

        let updates = [
            (node_id_at(frames, 3)?, 40.0, 20.0),
            (node_id_at(frames, 9)?, 40.0, 0.0),
            (node_id_at(frames, 11)?, 0.0, 30.0),
            (node_id_at(frames, 13)?, 55.0, 44.0),
            (node_id_at(frames, 17)?, 30.0, 0.0),
            (node_id_at(frames, 20)?, 22.0, 17.0),
            (node_id_at(frames, 22)?, 42.0, 29.0),
        ];
        let moved = initial
            .with_scroll_offsets(initial.key().scroll_state_revision(), &updates)
            .map_err(|error| {
                (
                    ERR_LAYOUT,
                    format!("C13.2 scroll snapshot 생성 실패: {error}"),
                )
            })?;
        for (owner_node, expected_x, expected_y) in [
            (3, 40.0, 20.0),
            (7, 0.0, 0.0),
            (9, 40.0, 0.0),
            (11, 0.0, 30.0),
            (13, 55.0, 44.0),
            (17, 30.0, 0.0),
            (20, 22.0, 17.0),
            (22, 42.0, 29.0),
            (24, 0.0, 0.0),
        ] {
            let owner_id = node_id_at(frames, owner_node)?;
            let moved_owner = scroll_owner_by_node(&moved, owner_id).ok_or((
                ERR_LAYOUT,
                format!("C13.2 moved owner frame {owner_node}가 없습니다"),
            ))?;
            if moved_owner.offset_x_css_px() != expected_x
                || moved_owner.offset_y_css_px() != expected_y
            {
                return Err((
                    ERR_LAYOUT,
                    format!(
                        "C13.2 owner frame {owner_node} offset가 다릅니다: ({}, {}) != ({expected_x}, {expected_y})",
                        moved_owner.offset_x_css_px(),
                        moved_owner.offset_y_css_px()
                    ),
                ));
            }
        }
        let moved_content = frame_by_id(&moved, scroll_content_id)
            .ok_or((ERR_LAYOUT, "C13.2 moved content box가 없습니다".to_owned()))?;
        let moved_marker = frame_by_id(&moved, scroll_marker_id)
            .ok_or((ERR_LAYOUT, "C13.2 moved marker box가 없습니다".to_owned()))?;
        let moved_fixed = frame_by_id(&moved, fixed_control_id)
            .ok_or((
                ERR_LAYOUT,
                "C13.2 moved fixed control이 없습니다".to_owned(),
            ))?
            .frame_css_px();
        if moved_content.frame_css_px().x() != initial_content.frame_css_px().x() - 40.0
            || moved_content.frame_css_px().y() != initial_content.frame_css_px().y() - 20.0
            || moved_fixed != initial_fixed
            || moved_marker.clip_css_px() != initial_marker.clip_css_px()
        {
            return Err((
                ERR_LAYOUT,
                "C13.2 paint offset, stationary scrollport clip 또는 fixed control이 일치하지 않습니다"
                    .to_owned(),
            ));
        }
        let moved_marker_x = moved_marker.frame_css_px().x() + 12.0;
        let moved_marker_y = moved_marker.frame_css_px().y() + 10.0;
        if moved
            .hit_test_css_point(moved_marker_x, moved_marker_y)
            .is_none_or(|hit| hit.node_id() != scroll_marker_id)
            || moved
                .hit_test_css_point(old_marker_x, old_marker_y)
                .is_some_and(|hit| hit.node_id() == scroll_marker_id)
            || moved
                .hit_test_css_point(escape_x, escape_y)
                .is_none_or(|hit| hit.node_id() != border_escape_id)
        {
            return Err((
                ERR_LAYOUT,
                "C13.2 moved marker 또는 visible border escape hit identity가 일치하지 않습니다"
                    .to_owned(),
            ));
        }

        self.publish_c13_2_scroll_snapshot(moved.clone())?;
        Ok(format!(
            "c13_2_scroll_fixture=passed owners=9 matrix=no-overflow:0,0 x-only:86,0 y-only:0,74 hidden:86,74 clip:none absolute:64,0 nested:88,78+66,54 forced-scroll:0,0 offset=40,20 revision={} border_escape_outside_parent=true border_escape_hit=matched marker_hit_after_scroll=matched old_marker_no_hit=true scrollport_clip_stationary=true fixed_control_stationary=true",
            moved.key().scroll_state_revision().get(),
        ))
    }

    pub(in crate::runtime_gpu) fn verify_c13_2_scroll_lifecycle(
        &self,
        layout_timeout_millis: u64,
    ) -> Result<String, (i32, String)> {
        let initial = self.c13_2_current_snapshot()?;
        if initial.scroll_containers().len() != 9 {
            return Err((
                ERR_LAYOUT,
                "C13.2 lifecycle 시작 전 owner 수가 9가 아닙니다".to_owned(),
            ));
        }
        let x_owner_id = initial.scroll_containers()[2].owner();
        let hidden_owner_id = initial.scroll_containers()[4].owner();
        let start_revision = initial.key().scroll_state_revision().get();
        let mut reports = Vec::new();

        for phase in 0_u64..4 {
            let report = self.eval(
                super::c13_2_scroll::RUNTIME_CSS_C13_2_SCROLL_LIFECYCLE_SOURCE,
                layout_timeout_millis,
            )?;
            if !report.contains("layout=ready") {
                return Err((
                    ERR_LAYOUT,
                    format!(
                        "C13.2 lifecycle phase {} layout가 준비되지 않았습니다: {report}",
                        phase + 1
                    ),
                ));
            }
            reports.push(report);
            let scene = self.c13_2_current_snapshot()?;
            let expected_revision = start_revision + phase + 1;
            if scene.key().scroll_state_revision().get() != expected_revision {
                return Err((
                    ERR_LAYOUT,
                    format!(
                        "C13.2 lifecycle phase {} scroll revision이 다릅니다: {} != {expected_revision}",
                        phase + 1,
                        scene.key().scroll_state_revision().get()
                    ),
                ));
            }
            let x_owner = scroll_owner_by_node(&scene, x_owner_id);
            let hidden_owner = scroll_owner_by_node(&scene, hidden_owner_id);
            match phase {
                0 if x_owner.is_some_and(|owner| {
                    owner.max_offset_x_css_px() == 146.0
                        && owner.offset_x_css_px() == 40.0
                        && owner.offset_y_css_px() == 0.0
                }) => {}
                1 if x_owner.is_some_and(|owner| {
                    owner.max_offset_x_css_px() == 0.0
                        && owner.offset_x_css_px() == 0.0
                        && owner.offset_y_css_px() == 0.0
                }) => {}
                2 if x_owner.is_some_and(|owner| owner.offset_x_css_px() == 0.0)
                    && hidden_owner.is_none() => {}
                3 if x_owner.is_some_and(|owner| owner.offset_x_css_px() == 0.0)
                    && hidden_owner.is_some_and(|owner| {
                        owner.max_offset_x_css_px() == 86.0
                            && owner.max_offset_y_css_px() == 74.0
                            && owner.offset_x_css_px() == 0.0
                            && owner.offset_y_css_px() == 0.0
                    }) => {}
                _ => {
                    return Err((
                        ERR_LAYOUT,
                        format!(
                            "C13.2 lifecycle phase {} range/offset 결과가 다릅니다",
                            phase + 1
                        ),
                    ));
                }
            }
            if scene.scroll_containers().len() != if phase == 2 { 8 } else { 9 } {
                return Err((
                    ERR_LAYOUT,
                    format!("C13.2 lifecycle phase {} owner 집합이 다릅니다", phase + 1),
                ));
            }
        }

        let final_scene = self.c13_2_current_snapshot()?;
        Ok(format!(
            "c13_2_lifecycle=passed growth_offset_retained=true shrink_clamped_to_zero=true detach_removed_owner=true reparent_reset_offset=true revisions={}-{} phases={} final_owners={}",
            start_revision,
            final_scene.key().scroll_state_revision().get(),
            reports.len(),
            final_scene.scroll_containers().len(),
        ))
    }

    pub(in crate::runtime_gpu) fn c13_2_current_snapshot(
        &self,
    ) -> Result<RuntimeRenderSnapshot, (i32, String)> {
        let _operation = lock(&self.operation);
        let (_, current) = self.presentation.lock_current();
        current
            .as_ref()
            .map(|published| (*published.snapshot).clone())
            .ok_or((
                ERR_LAYOUT,
                "C13.2 current render snapshot이 없습니다".to_owned(),
            ))
    }

    pub(in crate::runtime_gpu) fn publish_c13_2_scroll_snapshot(
        &self,
        snapshot: RuntimeRenderSnapshot,
    ) -> Result<(), (i32, String)> {
        let sequence = self
            .invalidate_scene()
            .map_err(|error| (ERR_STALE, error))?;
        if !self.presentation.publish(sequence, snapshot.clone()) {
            return Err((
                ERR_STALE,
                "C13.2 scroll snapshot publish 중 새 scene 요청이 도착했습니다".to_owned(),
            ));
        }
        *lock(&self.c13_2_scroll_state) = Some(snapshot);
        Ok(())
    }

    pub(in crate::runtime_gpu) fn reset_c13_2_scroll_state_for_fixture_transition(&self) {
        lock(&self.c13_2_scroll_state).take();
    }

    pub(in crate::runtime_gpu) fn verify_c13_2_failure_controls(
        &self,
        layout_timeout_millis: u64,
    ) -> Result<String, (i32, String)> {
        let initial = self.c13_2_scroll_state_snapshot()?;
        if initial.scroll_containers().len() != 9 {
            return Err((
                ERR_LAYOUT,
                format!(
                    "C13.2 negative control 시작 owner 수가 다릅니다: {} != 9",
                    initial.scroll_containers().len()
                ),
            ));
        }

        self.c13_2_reserve_failure_observed
            .store(false, Ordering::Release);
        self.c13_2_fail_next_scroll_reserve
            .store(true, Ordering::Release);
        let reserve_failure = self.eval(
            super::c13_2_scroll::RUNTIME_CSS_C13_2_SCROLL_FIXTURE_SOURCE,
            layout_timeout_millis,
        );
        self.c13_2_fail_next_scroll_reserve
            .store(false, Ordering::Release);
        let reserve_failure_observed = self
            .c13_2_reserve_failure_observed
            .swap(false, Ordering::AcqRel);
        match reserve_failure {
            Err((ERR_LAYOUT, report))
                if reserve_failure_observed
                    && report.contains("c13_2_injected_reserve_failure") => {}
            Err((status, report)) => {
                return Err((
                    status,
                    format!(
                        "C13.2 reserve failpoint 결과가 기대와 다릅니다: status={status} observed={reserve_failure_observed} injected_marker={} tail={}",
                        report.contains("c13_2_injected_reserve_failure"),
                        c13_2_report_tail(&report)
                    ),
                ));
            }
            Ok(report) => {
                return Err((
                    ERR_LAYOUT,
                    format!(
                        "C13.2 reserve failpoint이 실패를 만들지 않았습니다: observed={reserve_failure_observed} tail={}",
                        c13_2_report_tail(&report)
                    ),
                ));
            }
        }
        if self.c13_2_scroll_state_snapshot()? != initial {
            return Err((
                ERR_LAYOUT,
                "C13.2 reserve failpoint 뒤 이전 scroll-state snapshot이 바뀌었습니다".to_owned(),
            ));
        }

        let recovered = self
            .eval(
                super::c13_2_scroll::RUNTIME_CSS_C13_2_SCROLL_FIXTURE_SOURCE,
                layout_timeout_millis,
            )
            .map_err(|(status, report)| {
                (
                    status,
                    format!("C13.2 reserve failure 뒤 같은 host recovery 실패: {report}"),
                )
            })?;
        if !recovered.contains("layout=ready") {
            return Err((
                ERR_LAYOUT,
                format!("C13.2 reserve failure 뒤 layout이 준비되지 않았습니다: {recovered}"),
            ));
        }
        let before_stale = self.c13_2_scroll_state_snapshot()?;
        let stale_sequence = self.presentation_sequence();
        self.invalidate_scene()
            .map_err(|error| (ERR_STALE, error))?;
        if self
            .presentation
            .publish(stale_sequence, before_stale.clone())
        {
            return Err((
                ERR_STALE,
                "C13.2 이전 presentation sequence의 publish가 거부되지 않았습니다".to_owned(),
            ));
        }
        if self.c13_2_scroll_state_snapshot()? != before_stale {
            return Err((
                ERR_STALE,
                "C13.2 stale publish 거부 뒤 scroll-state snapshot이 바뀌었습니다".to_owned(),
            ));
        }

        let recovery = self
            .eval(
                super::c13_2_scroll::RUNTIME_CSS_C13_2_SCROLL_FIXTURE_SOURCE,
                layout_timeout_millis,
            )
            .map_err(|(status, report)| {
                (
                    status,
                    format!("C13.2 stale publish 거부 뒤 같은 host recovery 실패: {report}"),
                )
            })?;
        if !recovery.contains("layout=ready") {
            return Err((
                ERR_LAYOUT,
                format!("C13.2 stale recovery layout가 준비되지 않았습니다: {recovery}"),
            ));
        }
        let offset_report = self.apply_c13_2_scroll_fixture_offset()?;
        let final_state = self.c13_2_scroll_state_snapshot()?;
        let (sequence, published) = self.presentation.lock_current();
        let published = published.as_ref().ok_or((
            ERR_STALE,
            "C13.2 negative control recovery 뒤 presentation이 없습니다".to_owned(),
        ))?;
        if published.sequence != sequence || *published.snapshot != final_state {
            return Err((
                ERR_STALE,
                "C13.2 negative control recovery snapshot이 presentation과 다릅니다".to_owned(),
            ));
        }
        Ok(format!(
            "c13_2_failure_controls=passed reserve_failure=atomic reserve_failure_marker=matched stale_publish=rejected recovery=same_host final_presentation={sequence} {offset_report}"
        ))
    }

    fn c13_2_scroll_state_snapshot(&self) -> Result<RuntimeRenderSnapshot, (i32, String)> {
        lock(&self.c13_2_scroll_state).clone().ok_or((
            ERR_LAYOUT,
            "C13.2 scroll-state snapshot이 없습니다".to_owned(),
        ))
    }
}

fn c13_2_report_tail(report: &str) -> String {
    let tail = report.chars().rev().take(240).collect::<Vec<_>>();
    tail.into_iter().rev().collect()
}

fn same_scroll_geometry(
    left: &spinon_render::RuntimeScrollContainer,
    right: &spinon_render::RuntimeScrollContainer,
) -> bool {
    left.owner() == right.owner()
        && left.parent() == right.parent()
        && left.scrollport_css_px() == right.scrollport_css_px()
        && left.scrollable_width_css_px() == right.scrollable_width_css_px()
        && left.scrollable_height_css_px() == right.scrollable_height_css_px()
}

fn node_id_at(
    frames: &[spinon_runtime::RuntimeLayoutFrame],
    index: usize,
) -> Result<NodeId, (i32, String)> {
    frames
        .get(index)
        .and_then(|frame| NodeId::new(frame.node_id))
        .ok_or((
            ERR_LAYOUT,
            format!("C13.2 fixture node frame {index}가 없습니다"),
        ))
}

fn frame_by_id(scene: &RuntimeRenderSnapshot, node_id: NodeId) -> Option<&RuntimeRenderBox> {
    scene
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == node_id)
}

fn scroll_owner_by_node(
    scene: &RuntimeRenderSnapshot,
    owner: NodeId,
) -> Option<RuntimeScrollContainer> {
    scene
        .scroll_containers()
        .iter()
        .find(|container| container.owner() == owner)
        .copied()
}
