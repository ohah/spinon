use super::super::super::{ERR_ARGUMENT, ERR_LAYOUT, RuntimeGpuHost, lock};
use spinon_render::{RuntimeOverscrollBehavior, RuntimeRenderSnapshot};

impl RuntimeGpuHost {
    pub(in crate::runtime_gpu) fn apply_c13_3_style_invalidation(
        &self,
        layout_timeout_millis: u64,
    ) -> Result<String, (i32, String)> {
        let state = self.c13_3_gesture_state.as_ref().ok_or((
            ERR_ARGUMENT,
            "C13.3 gesture fixture host가 아닙니다".to_owned(),
        ))?;
        let mode = state.scenario.style_invalidation_mode().ok_or((
            ERR_ARGUMENT,
            "C13.3 style invalidation scenario가 아닙니다".to_owned(),
        ))?;
        let source = state.scenario.style_invalidation_source().ok_or((
            ERR_ARGUMENT,
            "C13.3 style invalidation source가 없습니다".to_owned(),
        ))?;
        if !matches!(
            lock(&state.session).state(),
            spinon_render::RuntimeScrollGestureState::Panning(_)
        ) {
            return Err((
                ERR_LAYOUT,
                "C13.3 style invalidation은 active pan 뒤에만 허용됩니다".to_owned(),
            ));
        }
        let before = state
            .current_snapshot()
            .map_err(|error| (ERR_LAYOUT, error))?;
        let eval_report = self.eval(source, layout_timeout_millis)?;
        let after = state
            .current_snapshot()
            .map_err(|error| (ERR_LAYOUT, error))?;
        let document_changed = before.key().document_revision() != after.key().document_revision();
        let tree_changed =
            before.key().render_tree_revision() != after.key().render_tree_revision();
        let style_changed = before.key().style_revision() != after.key().style_revision();
        if !(document_changed && tree_changed && style_changed) {
            return Err((
                ERR_LAYOUT,
                format!(
                    "C13.3 style mutation revision tuple이 갱신되지 않았습니다 mode={mode} before={:?} after={:?}",
                    before.key(),
                    after.key()
                ),
            ));
        }
        let geometry_unchanged = same_gesture_geometry(&before, &after);
        let owners = state.owners().map_err(|error| (ERR_LAYOUT, error))?;
        let before_inner = scroll_owner(&before, owners.inner).ok_or((
            ERR_LAYOUT,
            "C13.3 style mutation 이전 inner owner가 없습니다".to_owned(),
        ))?;
        let after_inner = scroll_owner(&after, owners.inner).ok_or((
            ERR_LAYOUT,
            "C13.3 style mutation 뒤 inner owner가 없습니다".to_owned(),
        ))?;
        match mode {
            8 => {
                if !geometry_unchanged
                    || after_inner.input_policy().overscroll_x()
                        != RuntimeOverscrollBehavior::Contain
                    || after.touch_action_at_css_point(100.0, 100.0)
                        != Some(spinon_render::RuntimeTouchAction::None)
                    || before.key().scroll_state_revision() != after.key().scroll_state_revision()
                    || before_inner.offset_x_css_px() != after_inner.offset_x_css_px()
                    || before_inner.offset_y_css_px() != after_inner.offset_y_css_px()
                {
                    return Err((
                        ERR_LAYOUT,
                        format!(
                            "C13.3 policy update가 geometry·offset revision을 바꿨습니다 geometry_unchanged={geometry_unchanged} policy={:?} touch_action={:?} scroll_revision={}→{} offsets_before=[{}] offsets_after=[{}]",
                            after_inner.input_policy(),
                            after.touch_action_at_css_point(100.0, 100.0),
                            before.key().scroll_state_revision().get(),
                            after.key().scroll_state_revision().get(),
                            super::c13_3_offsets(&before, owners),
                            super::c13_3_offsets(&after, owners)
                        ),
                    ));
                }
            }
            9 => {
                if geometry_unchanged
                    || after_inner.max_offset_x_css_px() != 66.0
                    || after_inner.offset_x_css_px() != 66.0
                    || after.key().scroll_state_revision() <= before.key().scroll_state_revision()
                {
                    return Err((
                        ERR_LAYOUT,
                        format!(
                            "C13.3 geometry update 결과가 예상과 다릅니다 geometry_unchanged={geometry_unchanged} range={} offset={} scroll_revision={}→{}",
                            after_inner.max_offset_x_css_px(),
                            after_inner.offset_x_css_px(),
                            before.key().scroll_state_revision().get(),
                            after.key().scroll_state_revision().get()
                        ),
                    ));
                }
            }
            10 => {
                let before_paint =
                    render_box(&before, owners.inner_content).map(|value| value.paint());
                let after_paint =
                    render_box(&after, owners.inner_content).map(|value| value.paint());
                if !geometry_unchanged
                    || before_paint == after_paint
                    || before.key().scroll_state_revision() != after.key().scroll_state_revision()
                {
                    return Err((
                        ERR_LAYOUT,
                        format!(
                            "C13.3 paint-only update 결과가 예상과 다릅니다 geometry_unchanged={geometry_unchanged} paint={before_paint:?}→{after_paint:?} scroll_revision={}→{} inner_offset={},{}→{},{}",
                            before.key().scroll_state_revision().get(),
                            after.key().scroll_state_revision().get(),
                            before_inner.offset_x_css_px(),
                            before_inner.offset_y_css_px(),
                            after_inner.offset_x_css_px(),
                            after_inner.offset_y_css_px()
                        ),
                    ));
                }
            }
            _ => {
                return Err((
                    ERR_ARGUMENT,
                    format!("C13.3 style invalidation mode가 올바르지 않습니다: {mode}"),
                ));
            }
        }
        Ok(format!(
            "c13_3_style_invalidation=applied mode={mode} document_revision={}→{} render_tree_revision={}→{} style_revision={}→{} scroll_state_revision={}→{} geometry_unchanged={geometry_unchanged} inner_offset={},{} inner_range={},{} eval=[{eval_report}]",
            before.key().document_revision().get(),
            after.key().document_revision().get(),
            before.key().render_tree_revision().get(),
            after.key().render_tree_revision().get(),
            before.key().style_revision().get(),
            after.key().style_revision().get(),
            before.key().scroll_state_revision().get(),
            after.key().scroll_state_revision().get(),
            after_inner.offset_x_css_px(),
            after_inner.offset_y_css_px(),
            after_inner.max_offset_x_css_px(),
            after_inner.max_offset_y_css_px()
        ))
    }

    pub(in crate::runtime_gpu) fn reapply_c13_3_scroll_state(
        &self,
        scene: RuntimeRenderSnapshot,
    ) -> Result<RuntimeRenderSnapshot, (i32, String)> {
        let Some(state) = self.c13_3_gesture_state.as_ref() else {
            return Ok(scene);
        };
        let Some(previous) = state.ready_snapshot() else {
            return Ok(scene);
        };
        if previous.key().generation() != scene.key().generation() {
            return Ok(scene);
        }
        let mut requests = Vec::new();
        requests
            .try_reserve_exact(previous.scroll_containers().len())
            .map_err(|_| {
                (
                    ERR_LAYOUT,
                    "C13.3 offset 복구 공간을 확보하지 못했습니다".to_owned(),
                )
            })?;
        for old in previous.scroll_containers() {
            if let Some(current) = scroll_owner(&scene, old.owner())
                && old.parent() == current.parent()
            {
                requests.push((old.owner(), old.offset_x_css_px(), old.offset_y_css_px()));
            }
        }
        let restored = scene
            .with_scroll_offsets(scene.key().scroll_state_revision(), &requests)
            .map_err(|error| (ERR_LAYOUT, format!("C13.3 offset 복구 실패: {error}")))?;
        // Hit-test clips move with scroll offsets, so compare geometry after restoring them.
        let geometry_changed = !same_gesture_geometry(&previous, &restored);
        let offsets_changed = previous.scroll_containers().iter().any(|old| {
            scroll_owner(&restored, old.owner()).is_none_or(|current| {
                old.offset_x_css_px() != current.offset_x_css_px()
                    || old.offset_y_css_px() != current.offset_y_css_px()
            })
        });
        let base_revision = previous
            .key()
            .scroll_state_revision()
            .max(scene.key().scroll_state_revision());
        let revision = if geometry_changed || offsets_changed {
            base_revision.checked_next().ok_or((
                ERR_LAYOUT,
                "C13.3 scroll state revision이 소진됐습니다".to_owned(),
            ))?
        } else {
            base_revision
        };
        Ok(restored.with_scroll_state_revision(revision))
    }
}

fn same_gesture_geometry(left: &RuntimeRenderSnapshot, right: &RuntimeRenderSnapshot) -> bool {
    left.viewport_css_px() == right.viewport_css_px()
        && left.boxes().len() == right.boxes().len()
        && left.scroll_containers().len() == right.scroll_containers().len()
        && left.boxes().iter().all(|old| {
            let Some(current) = render_box(right, old.node_id()) else {
                return false;
            };
            old.frame_css_px() == current.frame_css_px()
                && old.clip_css_px() == current.clip_css_px()
                && old.hit_test_clip_css_px() == current.hit_test_clip_css_px()
                && old.hit_test_order() == current.hit_test_order()
                && left.scroll_ancestors_for_node(old.node_id())
                    == right.scroll_ancestors_for_node(old.node_id())
        })
        && left.scroll_containers().iter().all(|old| {
            scroll_owner(right, old.owner()).is_some_and(|current| {
                old.parent() == current.parent()
                    && old.scrollport_css_px() == current.scrollport_css_px()
                    && old.max_offset_x_css_px() == current.max_offset_x_css_px()
                    && old.max_offset_y_css_px() == current.max_offset_y_css_px()
            })
        })
}

fn render_box(
    scene: &RuntimeRenderSnapshot,
    node: spinon_core::NodeId,
) -> Option<&spinon_render::RuntimeRenderBox> {
    scene
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == node)
}

fn scroll_owner(
    scene: &RuntimeRenderSnapshot,
    owner: spinon_core::NodeId,
) -> Option<&spinon_render::RuntimeScrollContainer> {
    scene
        .scroll_containers()
        .iter()
        .find(|container| container.owner() == owner)
}
