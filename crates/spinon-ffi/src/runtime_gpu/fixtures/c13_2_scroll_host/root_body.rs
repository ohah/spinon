use super::super::super::{ERR_LAYOUT, RuntimeGpuHost, scene_matches_layout_key};
use super::{frame_by_id, node_id_at, scroll_owner_by_node};
use spinon_render::RuntimeRenderSnapshot;
use std::sync::Arc;

impl RuntimeGpuHost {
    pub(in crate::runtime_gpu) fn apply_c13_2_root_body_propagation_offset(
        &self,
    ) -> Result<String, (i32, String)> {
        let snapshot = self.c13_2_current_snapshot()?;
        let frames = self.c13_2_root_body_frames(&snapshot)?;
        let html = node_id_at(&frames, 0)?;
        let body = node_id_at(&frames, 1)?;
        let content = node_id_at(&frames, 2)?;
        let marker = node_id_at(&frames, 3)?;
        let fixed = node_id_at(&frames, 4)?;
        if snapshot.scroll_containers().len() != 1 {
            return Err((
                ERR_LAYOUT,
                format!(
                    "C13.2 visible html/body owner 수가 다릅니다: {} != 1",
                    snapshot.scroll_containers().len()
                ),
            ));
        }
        let viewport_owner = scroll_owner_by_node(&snapshot, html).ok_or((
            ERR_LAYOUT,
            "C13.2 visible html root에 viewport scroll owner가 없습니다".to_owned(),
        ))?;
        if viewport_owner.parent().is_some()
            || viewport_owner.max_offset_x_css_px() != 64.0
            || viewport_owner.max_offset_y_css_px() != 84.0
            || scroll_owner_by_node(&snapshot, body).is_some()
        {
            return Err((
                ERR_LAYOUT,
                "C13.2 body auto가 visible html에서 viewport owner로 전파되지 않았습니다"
                    .to_owned(),
            ));
        }
        let before_content = frame_by_id(&snapshot, content)
            .ok_or((
                ERR_LAYOUT,
                "C13.2 root/body content frame이 없습니다".to_owned(),
            ))?
            .frame_css_px();
        let before_marker = frame_by_id(&snapshot, marker)
            .ok_or((
                ERR_LAYOUT,
                "C13.2 root/body marker frame이 없습니다".to_owned(),
            ))?
            .frame_css_px();
        let before_fixed = frame_by_id(&snapshot, fixed)
            .ok_or((
                ERR_LAYOUT,
                "C13.2 root/body fixed frame이 없습니다".to_owned(),
            ))?
            .frame_css_px();
        if before_fixed.x() != 340.0 || before_fixed.y() != 204.0 {
            return Err((
                ERR_LAYOUT,
                format!(
                    "C13.2 visible root/body fixed frame이 다릅니다: ({}, {}) != (340, 204)",
                    before_fixed.x(),
                    before_fixed.y()
                ),
            ));
        }
        let moved = snapshot
            .with_scroll_offsets(
                snapshot.key().scroll_state_revision(),
                &[(html, 64.0, 84.0)],
            )
            .map_err(|error| {
                (
                    ERR_LAYOUT,
                    format!("C13.2 propagated viewport scroll snapshot 실패: {error}"),
                )
            })?;
        let moved_content = frame_by_id(&moved, content)
            .ok_or((
                ERR_LAYOUT,
                "C13.2 moved root/body content frame이 없습니다".to_owned(),
            ))?
            .frame_css_px();
        let moved_marker = frame_by_id(&moved, marker)
            .ok_or((
                ERR_LAYOUT,
                "C13.2 moved root/body marker frame이 없습니다".to_owned(),
            ))?
            .frame_css_px();
        let moved_fixed = frame_by_id(&moved, fixed)
            .ok_or((
                ERR_LAYOUT,
                "C13.2 moved root/body fixed frame이 없습니다".to_owned(),
            ))?
            .frame_css_px();
        let moved_viewport_owner = scroll_owner_by_node(&moved, html).ok_or((
            ERR_LAYOUT,
            "C13.2 moved viewport scroll owner가 없습니다".to_owned(),
        ))?;
        if moved_viewport_owner.offset_x_css_px() != 64.0
            || moved_viewport_owner.offset_y_css_px() != 84.0
            || moved_content.x() != before_content.x() - 64.0
            || moved_content.y() != before_content.y() - 84.0
            || moved_marker.x() != before_marker.x() - 64.0
            || moved_marker.y() != before_marker.y() - 84.0
            || moved_fixed != before_fixed
            || moved
                .hit_test_css_point(moved_marker.x() + 10.0, moved_marker.y() + 10.0)
                .is_none_or(|hit| hit.node_id() != marker)
            || moved
                .hit_test_css_point(before_marker.x() + 10.0, before_marker.y() + 10.0)
                .is_some_and(|hit| hit.node_id() == marker)
        {
            return Err((
                ERR_LAYOUT,
                "C13.2 propagated viewport paint/hit offset or fixed control is inconsistent"
                    .to_owned(),
            ));
        }
        self.publish_c13_2_scroll_snapshot(moved.clone())?;
        Ok(format!(
            "c13_2_root_body_propagation=passed viewport_owner=html body_owner=none range=64,84 offset=64,84 marker_hit=matched old_marker_no_hit=true fixed=340,204 revision={}",
            moved.key().scroll_state_revision().get()
        ))
    }

    pub(in crate::runtime_gpu) fn apply_c13_2_root_body_hidden_body_offset(
        &self,
    ) -> Result<String, (i32, String)> {
        let snapshot = self.c13_2_current_snapshot()?;
        let frames = self.c13_2_root_body_frames(&snapshot)?;
        let html = node_id_at(&frames, 0)?;
        let body = node_id_at(&frames, 1)?;
        let content = node_id_at(&frames, 2)?;
        let marker = node_id_at(&frames, 3)?;
        let fixed = node_id_at(&frames, 4)?;
        if snapshot.scroll_containers().len() != 2 {
            return Err((
                ERR_LAYOUT,
                format!(
                    "C13.2 hidden html/body owner 수가 다릅니다: {} != 2",
                    snapshot.scroll_containers().len()
                ),
            ));
        }
        let viewport_owner = scroll_owner_by_node(&snapshot, html).ok_or((
            ERR_LAYOUT,
            "C13.2 hidden html root owner가 없습니다".to_owned(),
        ))?;
        let body_owner = scroll_owner_by_node(&snapshot, body).ok_or((
            ERR_LAYOUT,
            "C13.2 hidden html/body 독립 body owner가 없습니다".to_owned(),
        ))?;
        if viewport_owner.max_offset_x_css_px() != 0.0
            || viewport_owner.max_offset_y_css_px() != 0.0
            || viewport_owner.parent().is_some()
            || body_owner.parent() != Some(html)
            || body_owner.max_offset_x_css_px() != 64.0
            || body_owner.max_offset_y_css_px() != 84.0
            || body_owner.offset_x_css_px() != 0.0
            || body_owner.offset_y_css_px() != 0.0
        {
            return Err((
                ERR_LAYOUT,
                "C13.2 hidden html/body 독립 scrollport 범위가 다릅니다".to_owned(),
            ));
        }
        let before_content = frame_by_id(&snapshot, content)
            .ok_or((
                ERR_LAYOUT,
                "C13.2 hidden root/body content frame이 없습니다".to_owned(),
            ))?
            .frame_css_px();
        let before_marker = frame_by_id(&snapshot, marker)
            .ok_or((
                ERR_LAYOUT,
                "C13.2 hidden root/body marker frame이 없습니다".to_owned(),
            ))?
            .frame_css_px();
        let before_fixed = frame_by_id(&snapshot, fixed)
            .ok_or((
                ERR_LAYOUT,
                "C13.2 hidden root/body fixed frame이 없습니다".to_owned(),
            ))?
            .frame_css_px();
        if before_fixed.x() != 340.0 || before_fixed.y() != 204.0 {
            return Err((
                ERR_LAYOUT,
                format!(
                    "C13.2 hidden root/body fixed frame이 다릅니다: ({}, {}) != (340, 204)",
                    before_fixed.x(),
                    before_fixed.y()
                ),
            ));
        }
        let moved = snapshot
            .with_scroll_offsets(
                snapshot.key().scroll_state_revision(),
                &[(body, 64.0, 84.0)],
            )
            .map_err(|error| {
                (
                    ERR_LAYOUT,
                    format!("C13.2 independent body scroll snapshot 실패: {error}"),
                )
            })?;
        let moved_content = frame_by_id(&moved, content)
            .ok_or((
                ERR_LAYOUT,
                "C13.2 moved hidden content frame이 없습니다".to_owned(),
            ))?
            .frame_css_px();
        let moved_marker = frame_by_id(&moved, marker)
            .ok_or((
                ERR_LAYOUT,
                "C13.2 moved hidden marker frame이 없습니다".to_owned(),
            ))?
            .frame_css_px();
        let moved_fixed = frame_by_id(&moved, fixed)
            .ok_or((
                ERR_LAYOUT,
                "C13.2 moved hidden fixed frame이 없습니다".to_owned(),
            ))?
            .frame_css_px();
        let moved_viewport_owner = scroll_owner_by_node(&moved, html).ok_or((
            ERR_LAYOUT,
            "C13.2 moved hidden viewport owner가 없습니다".to_owned(),
        ))?;
        let moved_body_owner = scroll_owner_by_node(&moved, body).ok_or((
            ERR_LAYOUT,
            "C13.2 moved independent body owner가 없습니다".to_owned(),
        ))?;
        if moved_viewport_owner.offset_x_css_px() != 0.0
            || moved_viewport_owner.offset_y_css_px() != 0.0
            || moved_body_owner.offset_x_css_px() != 64.0
            || moved_body_owner.offset_y_css_px() != 84.0
            || moved_content.x() != before_content.x() - 64.0
            || moved_content.y() != before_content.y() - 84.0
            || moved_marker.x() != before_marker.x() - 64.0
            || moved_marker.y() != before_marker.y() - 84.0
            || moved_fixed != before_fixed
            || moved
                .hit_test_css_point(moved_marker.x() + 10.0, moved_marker.y() + 10.0)
                .is_none_or(|hit| hit.node_id() != marker)
            || moved
                .hit_test_css_point(before_marker.x() + 10.0, before_marker.y() + 10.0)
                .is_some_and(|hit| hit.node_id() == marker)
        {
            return Err((
                ERR_LAYOUT,
                "C13.2 independent body paint/hit offset or fixed control is inconsistent"
                    .to_owned(),
            ));
        }
        self.publish_c13_2_scroll_snapshot(moved.clone())?;
        Ok(format!(
            "c13_2_root_body_hidden=passed viewport_range=0,0 body_range=64,84 body_offset=64,84 marker_hit=matched fixed=340,204 revision={}",
            moved.key().scroll_state_revision().get()
        ))
    }

    fn c13_2_root_body_frames(
        &self,
        scene: &RuntimeRenderSnapshot,
    ) -> Result<Arc<[spinon_runtime::RuntimeLayoutFrame]>, (i32, String)> {
        let completed = self
            .session
            .layout_snapshot()
            .completed
            .filter(|completed| {
                completed
                    .render_snapshot
                    .as_ref()
                    .is_some_and(|layout_scene| {
                        scene_matches_layout_key(layout_scene, completed.key)
                    })
                    && scene_matches_layout_key(scene, completed.key)
            })
            .ok_or((
                ERR_LAYOUT,
                "C13.2 root/body layout와 render snapshot revision이 다릅니다".to_owned(),
            ))?;
        if completed.frames.len() != 5 {
            return Err((
                ERR_LAYOUT,
                format!(
                    "C13.2 root/body frame 수가 다릅니다: {} != 5",
                    completed.frames.len()
                ),
            ));
        }
        Ok(Arc::clone(&completed.frames))
    }
}
