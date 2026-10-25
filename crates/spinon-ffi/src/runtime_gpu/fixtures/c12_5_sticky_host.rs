use std::collections::BTreeMap;

use super::super::{ERR_LAYOUT, RuntimeGpuHost};
use spinon_core::{AttributeName, HostDocumentSnapshot, HostNodeKind, NodeId};
use spinon_render::{RuntimeRenderBox, RuntimeRenderSnapshot};

pub(crate) const RUNTIME_CSS_C12_5_STICKY_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c12/runtime-sticky-positioning-c12-5.js");

impl RuntimeGpuHost {
    pub(in crate::runtime_gpu) fn eval_c12_5_sticky_fixture(
        &self,
        layout_timeout_millis: u64,
    ) -> Result<String, (i32, String)> {
        let eval_report = self.eval(
            RUNTIME_CSS_C12_5_STICKY_FIXTURE_SOURCE,
            layout_timeout_millis,
        )?;
        if !eval_report.contains("layout=ready") {
            return Err((
                ERR_LAYOUT,
                format!("C12.5 initial layout이 준비되지 않았습니다: {eval_report}"),
            ));
        }
        let document = self
            .session
            .snapshot_document_for_c14_1_fixture()
            .map_err(|error| {
                (
                    ERR_LAYOUT,
                    format!("C12.5 V8 document snapshot 실패: {error}"),
                )
            })?;
        let ids = sticky_fixture_node_ids(&document)?;
        let initial = self.c13_2_current_snapshot()?;
        let outer = ids["outer"];
        let inner = ids["inner"];
        let section = ids["section"];
        let sticky = ids["sticky"];
        for owner_id in [outer, inner] {
            let owner = scroll_owner(&initial, owner_id).ok_or((
                ERR_LAYOUT,
                format!("C12.5 scroll owner가 없습니다: {owner_id}"),
            ))?;
            if owner.max_offset_y_css_px() < 212.0 {
                return Err((
                    ERR_LAYOUT,
                    format!(
                        "C12.5 scroll owner range가 oracle sample보다 작습니다: owner={owner_id} range={} ",
                        owner.max_offset_y_css_px()
                    ),
                ));
            }
        }
        if initial
            .scroll_ancestors_for_node(sticky)
            .and_then(|ancestors| ancestors.first().copied())
            != Some(inner)
        {
            return Err((
                ERR_LAYOUT,
                "C12.5 sticky nearest scroll owner가 inner가 아닙니다".to_owned(),
            ));
        }
        let inner_owner = scroll_owner(&initial, inner)
            .ok_or((ERR_LAYOUT, "C12.5 inner scroll owner가 없습니다".to_owned()))?;
        if (inner_owner.scrollport_css_px().y() - 76.0).abs() > 0.5 {
            return Err((
                ERR_LAYOUT,
                format!(
                    "C12.5 inner scrollport y가 다릅니다: {} != 76",
                    inner_owner.scrollport_css_px().y()
                ),
            ));
        }
        check_top(&initial, sticky, 118.0, "initial")?;
        check_top_for(&initial, section, 118.0, "initial section")?;

        let outer_moved = transition(&initial, &[(outer, 0.0, 24.0)])?;
        check_top(&outer_moved, sticky, 94.0, "outer scroll")?;
        check_top_for(&outer_moved, section, 94.0, "outer scroll section")?;

        let inner_before_threshold =
            transition(&outer_moved, &[(outer, 0.0, 0.0), (inner, 0.0, 20.0)])?;
        check_top(
            &inner_before_threshold,
            sticky,
            98.0,
            "inner before threshold",
        )?;

        let inner_pinned = transition(&inner_before_threshold, &[(inner, 0.0, 36.0)])?;
        check_top(&inner_pinned, sticky, 90.0, "inner pinned")?;

        let inner_pinned_later = transition(&inner_pinned, &[(inner, 0.0, 160.0)])?;
        check_top(&inner_pinned_later, sticky, 90.0, "inner pinned later")?;

        let section_end = transition(&inner_pinned_later, &[(inner, 0.0, 212.0)])?;
        check_top(&section_end, sticky, 54.0, "containing-block end")?;
        let section_frame = frame(&section_end, section)?;
        if ((section_frame.y() + section_frame.height()) - 82.0).abs() > 0.5 {
            return Err((
                ERR_LAYOUT,
                format!(
                    "C12.5 containing-block bottom이 다릅니다: {} != 82",
                    section_frame.y() + section_frame.height()
                ),
            ));
        }
        let sticky_box = render_box(&section_end, sticky)?;
        if !sticky_box.clip_css_px().contains(154.0, 78.0)
            || section_end
                .hit_test_css_point(154.0, 78.0)
                .is_none_or(|hit| hit.node_id() != sticky)
            || section_end
                .hit_test_css_point(154.0, 123.0)
                .is_some_and(|hit| hit.node_id() == sticky)
        {
            return Err((
                ERR_LAYOUT,
                "C12.5 clipped paint/hit-test geometry가 sticky frame과 일치하지 않습니다"
                    .to_owned(),
            ));
        }

        let reverse = transition(&section_end, &[(outer, 0.0, 0.0), (inner, 0.0, 0.0)])?;
        check_top(&reverse, sticky, 118.0, "reverse to zero")?;

        let content_edge_style_report = self.eval(
            "globalThis.spinonC125SetParentContentEdge();",
            layout_timeout_millis,
        )?;
        if !content_edge_style_report.contains("layout=ready") {
            return Err((
                ERR_LAYOUT,
                format!(
                    "C12.5 parent content-edge 변형 layout이 준비되지 않았습니다: {content_edge_style_report}"
                ),
            ));
        }
        let content_edge_initial = self.c13_2_current_snapshot()?;
        check_top(&content_edge_initial, sticky, 118.0, "content-edge initial")?;
        check_top_for(
            &content_edge_initial,
            section,
            118.0,
            "content-edge initial section",
        )?;

        let content_edge_outer_moved = transition(&content_edge_initial, &[(outer, 0.0, 24.0)])?;
        check_top(
            &content_edge_outer_moved,
            sticky,
            94.0,
            "content-edge outer scroll",
        )?;
        let content_edge_inner_before_threshold = transition(
            &content_edge_outer_moved,
            &[(outer, 0.0, 0.0), (inner, 0.0, 20.0)],
        )?;
        check_top(
            &content_edge_inner_before_threshold,
            sticky,
            98.0,
            "content-edge inner before threshold",
        )?;
        let content_edge_inner_pinned =
            transition(&content_edge_inner_before_threshold, &[(inner, 0.0, 36.0)])?;
        check_top(
            &content_edge_inner_pinned,
            sticky,
            90.0,
            "content-edge inner pinned",
        )?;
        let content_edge_inner_pinned_later =
            transition(&content_edge_inner_pinned, &[(inner, 0.0, 160.0)])?;
        check_top(
            &content_edge_inner_pinned_later,
            sticky,
            90.0,
            "content-edge inner pinned later",
        )?;
        let content_edge_end =
            transition(&content_edge_inner_pinned_later, &[(inner, 0.0, 212.0)])?;
        check_top(&content_edge_end, sticky, 40.0, "parent content-edge end")?;
        let content_edge_section_frame = frame(&content_edge_end, section)?;
        let content_edge_border_bottom =
            content_edge_section_frame.y() + content_edge_section_frame.height();
        if (content_edge_border_bottom - 82.0).abs() > 0.5
            || (content_edge_border_bottom - 14.0 - 68.0).abs() > 0.5
        {
            return Err((
                ERR_LAYOUT,
                format!(
                    "C12.5 section border/content edge가 다릅니다: border_bottom={content_edge_border_bottom} content_bottom={} expected=82/68",
                    content_edge_border_bottom - 14.0
                ),
            ));
        }
        let section_pixel_hit = content_edge_end
            .hit_test_css_point(200.0, 78.0)
            .is_some_and(|hit| hit.node_id() == section);
        let section_after_pixel_hit = content_edge_end
            .hit_test_css_point(240.0, 100.0)
            .is_some_and(|hit| hit.node_id() == ids["section-after"]);
        if !section_pixel_hit || !section_after_pixel_hit {
            return Err((
                ERR_LAYOUT,
                format!(
                    "C12.5 content-edge pixel/hit-test 불일치: section_at_200_78={section_pixel_hit} section_after_at_240_100={section_after_pixel_hit}"
                ),
            ));
        }
        let content_edge_sticky_box = render_box(&content_edge_end, sticky)?;
        if !content_edge_sticky_box.clip_css_px().contains(200.0, 78.0)
            || content_edge_sticky_box.clip_css_px().contains(200.0, 42.0)
        {
            return Err((
                ERR_LAYOUT,
                "C12.5 content-edge sticky scrollport clip이 pixel visibility와 맞지 않습니다"
                    .to_owned(),
            ));
        }

        self.publish_c13_2_scroll_snapshot(content_edge_end.clone())?;
        let report = serde_json::json!({
            "status": "passed",
            "case": "c12-5-top-only-nested-scrollport-content-edge",
            "scroll_owners": 2,
            "samples": {
                "initial": frame_top(&initial, sticky)?,
                "outer_24": frame_top(&outer_moved, sticky)?,
                "inner_20": frame_top(&inner_before_threshold, sticky)?,
                "inner_36": frame_top(&inner_pinned, sticky)?,
                "inner_160": frame_top(&inner_pinned_later, sticky)?,
                "inner_212": frame_top(&section_end, sticky)?,
                "reverse_zero": frame_top(&reverse, sticky)?,
                "parent_content_edge_inner_212": frame_top(&content_edge_end, sticky)?,
                "parent_content_edge_section_pixel_y78": "#f5f5f5",
                "parent_content_edge_section_after_pixel_y100": "#cde"
            },
            "section_bottom_at_inner_212": section_frame.y() + section_frame.height(),
            "parent_content_edge": {
                "padding_bottom_css_px": 12,
                "border_bottom_css_px": 2,
                "section_border_bottom": content_edge_border_bottom,
                "section_content_bottom": content_edge_border_bottom - 14.0,
                "sticky_top": frame_top(&content_edge_end, sticky)?,
                "pixel_200_78_hit": "section",
                "pixel_200_78_expected_rgba": [245, 245, 245, 255],
                "pixel_240_100_hit": "section-after",
                "pixel_240_100_expected_rgba": [204, 221, 238, 255]
            },
            "nearest_scroll_owner": inner.to_string(),
            "scroll_state_revision": content_edge_end.key().scroll_state_revision().get(),
            "clip_and_hit_test": "passed",
            "browser_reverse_observation": false,
            "content_edge_style_eval": content_edge_style_report
        });
        serde_json::to_string(&report)
            .map(|json| {
                format!(
                    "c12_5_sticky_runtime={json} eval=[{eval_report}] content_edge_eval=[{content_edge_style_report}]"
                )
            })
            .map_err(|error| {
                (
                    ERR_LAYOUT,
                    format!("C12.5 runtime report JSON 변환 실패: {error}"),
                )
            })
    }
}

fn transition(
    current: &RuntimeRenderSnapshot,
    updates: &[(NodeId, f32, f32)],
) -> Result<RuntimeRenderSnapshot, (i32, String)> {
    current
        .with_scroll_offsets(current.key().scroll_state_revision(), updates)
        .map_err(|error| {
            (
                ERR_LAYOUT,
                format!("C12.5 scroll snapshot 생성 실패: {error}"),
            )
        })
}

fn check_top(
    snapshot: &RuntimeRenderSnapshot,
    node: NodeId,
    expected: f32,
    label: &str,
) -> Result<(), (i32, String)> {
    let actual = frame_top(snapshot, node)?;
    if (actual - expected).abs() <= 0.5 {
        Ok(())
    } else {
        Err((
            ERR_LAYOUT,
            format!("C12.5 {label} sticky top 불일치: {actual} != {expected}"),
        ))
    }
}

fn check_top_for(
    snapshot: &RuntimeRenderSnapshot,
    node: NodeId,
    expected: f32,
    label: &str,
) -> Result<(), (i32, String)> {
    let actual = frame_top(snapshot, node)?;
    if (actual - expected).abs() <= 0.5 {
        Ok(())
    } else {
        Err((
            ERR_LAYOUT,
            format!("C12.5 {label} top 불일치: {actual} != {expected}"),
        ))
    }
}

fn frame_top(snapshot: &RuntimeRenderSnapshot, node: NodeId) -> Result<f32, (i32, String)> {
    Ok(frame(snapshot, node)?.y())
}

fn frame(
    snapshot: &RuntimeRenderSnapshot,
    node: NodeId,
) -> Result<spinon_render::CssRect, (i32, String)> {
    Ok(render_box(snapshot, node)?.frame_css_px())
}

fn render_box(
    snapshot: &RuntimeRenderSnapshot,
    node: NodeId,
) -> Result<&RuntimeRenderBox, (i32, String)> {
    snapshot
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == node)
        .ok_or((ERR_LAYOUT, format!("C12.5 render box가 없습니다: {node}")))
}

fn scroll_owner(
    snapshot: &RuntimeRenderSnapshot,
    node: NodeId,
) -> Option<&spinon_render::RuntimeScrollContainer> {
    snapshot
        .scroll_containers()
        .iter()
        .find(|container| container.owner() == node)
}

fn sticky_fixture_node_ids(
    document: &HostDocumentSnapshot,
) -> Result<BTreeMap<String, NodeId>, (i32, String)> {
    const EXPECTED: [&str; 11] = [
        "outer",
        "outer-content",
        "before-inner",
        "inner",
        "inner-content",
        "before-section",
        "section",
        "sticky",
        "section-after",
        "inner-tail",
        "outer-tail",
    ];
    let id_attribute = AttributeName::new(None, "id").ok_or((
        ERR_LAYOUT,
        "C12.5 id attribute 이름을 만들 수 없습니다".to_owned(),
    ))?;
    let mut found = BTreeMap::new();
    let mut pending = document.root_children().collect::<Vec<_>>();
    while let Some(handle) = pending.pop() {
        let node = document.node(handle).ok_or((
            ERR_LAYOUT,
            "C12.5 V8 document traversal node가 없습니다".to_owned(),
        ))?;
        if let HostNodeKind::Element(element) = node.kind()
            && let Some(id) = element
                .attribute(&id_attribute)
                .and_then(|value| String::from_utf16(value.code_units()).ok())
        {
            if !EXPECTED.contains(&id.as_str()) && id.starts_with("c125-") {
                return Err((
                    ERR_LAYOUT,
                    format!("C12.5 예기치 않은 fixture ID가 있습니다: {id}"),
                ));
            }
            if EXPECTED.contains(&id.as_str()) && found.insert(id.clone(), node.id()).is_some() {
                return Err((ERR_LAYOUT, format!("C12.5 중복 fixture ID: {id}")));
            }
        }
        if let Some(children) = document.children(handle) {
            pending.extend(children);
        }
    }
    let expected_count = EXPECTED.len();
    if found.len() != expected_count {
        return Err((
            ERR_LAYOUT,
            format!(
                "C12.5 fixture ID inventory가 다릅니다: {} != {expected_count}",
                found.len()
            ),
        ));
    }
    Ok(found)
}
