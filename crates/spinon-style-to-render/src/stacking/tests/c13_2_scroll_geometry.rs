use super::{
    FaultInjectingReserve, SystemReserve, build_runtime_render_snapshot_with_reserve,
    overflow_runtime_inputs_with_tags, overflow_runtime_inputs_with_viewport,
};
use crate::{
    RuntimeScrollOffsetRequest, StyleRenderError,
    build_runtime_render_snapshot_with_scroll_requests,
};
use spinon_core::NodeId;
use spinon_render::{RuntimeRenderSnapshot, RuntimeScrollContainer};

fn scene_for(styles: &[&str], parents: &[Option<usize>]) -> (RuntimeRenderSnapshot, Vec<NodeId>) {
    let (input, nodes) = overflow_runtime_inputs_with_viewport(styles, parents, 400.0, 240.0);
    let scene = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut SystemReserve,
    )
    .unwrap();
    (scene, nodes)
}

#[test]
fn visible_border_escape_paints_outside_without_creating_scroll_state() {
    let styles = [
        "position:relative;display:flow-root;box-sizing:border-box;width:120px;height:82px;overflow:visible;background-color:#101827",
        "position:absolute;left:130px;top:8px;width:30px;height:24px;background-color:#e3544e",
    ];
    let (scene, nodes) = scene_for(&styles, &[None, Some(0)]);
    let parent = scene
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == nodes[0])
        .unwrap();
    let child = scene
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == nodes[1])
        .unwrap();

    assert!(child.frame_css_px().x() > parent.frame_css_px().x() + parent.frame_css_px().width());
    assert!(child.clip_css_px().x().is_none());
    assert!(scene.scroll_containers().is_empty());
    assert_eq!(
        scene
            .hit_test_css_point(
                child.frame_css_px().x() + 2.0,
                child.frame_css_px().y() + 2.0
            )
            .map(|hit| hit.node_id()),
        Some(nodes[1]),
        "visible overflow outside the parent border remains a hit target"
    );
}

fn html_root_scroll_inputs(
    root_overflow: &str,
    body_overflow: &str,
) -> (super::RuntimeInputs, Vec<NodeId>) {
    let root_style = format!(
        "display:block;box-sizing:border-box;width:640px;height:480px;margin:0;overflow:{root_overflow};background-color:#ffffff"
    );
    let body_style = format!(
        "display:block;box-sizing:border-box;width:640px;height:480px;margin:0;overflow:{body_overflow};background-color:#f4f7fa"
    );
    let styles = [
        root_style.as_str(),
        body_style.as_str(),
        "display:block;box-sizing:border-box;width:704px;height:964px;background-color:#57a6d9",
        "position:fixed;right:8px;bottom:8px;width:52px;height:28px;background-color:#ed8b36",
    ];
    overflow_runtime_inputs_with_tags(
        &styles,
        &["html", "body", "main", "div"],
        &[None, Some(0), Some(1), Some(1)],
        640.0,
        480.0,
    )
}

#[test]
fn visible_html_propagates_body_auto_to_the_viewport_owner() {
    let (input, nodes) = html_root_scroll_inputs("visible", "auto");
    let initial = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut SystemReserve,
    )
    .unwrap();
    assert_eq!(initial.scroll_containers().len(), 1);
    let viewport_owner = initial.scroll_containers()[0];
    assert_eq!(viewport_owner.owner(), nodes[0]);
    assert_eq!(viewport_owner.parent(), None);
    assert_eq!(
        (
            viewport_owner.scrollport_css_px().x(),
            viewport_owner.scrollport_css_px().y(),
            viewport_owner.scrollport_css_px().width(),
            viewport_owner.scrollport_css_px().height(),
        ),
        (0.0, 0.0, 640.0, 480.0)
    );
    assert_eq!(viewport_owner.max_offset_x_css_px(), 64.0);
    assert_eq!(viewport_owner.max_offset_y_css_px(), 484.0);

    let content_before = initial
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == nodes[2])
        .unwrap()
        .frame_css_px();
    let fixed_before = initial
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == nodes[3])
        .unwrap()
        .frame_css_px();
    let moved = build_runtime_render_snapshot_with_scroll_requests(
        &input.document,
        input.root,
        &input.output,
        input.current,
        Some(&initial),
        &[RuntimeScrollOffsetRequest::new(
            nodes[0],
            initial.key().scroll_state_revision(),
            64.0,
            83.0,
        )],
    )
    .unwrap();
    let content_after = moved
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == nodes[2])
        .unwrap()
        .frame_css_px();
    let fixed_after = moved
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == nodes[3])
        .unwrap()
        .frame_css_px();
    assert_eq!(content_after.x(), content_before.x() - 64.0);
    assert_eq!(content_after.y(), content_before.y() - 83.0);
    assert_eq!(fixed_after, fixed_before);
}

#[test]
fn hidden_html_keeps_viewport_at_zero_and_gives_body_an_independent_scrollport() {
    let (input, nodes) = html_root_scroll_inputs("hidden", "auto");
    let initial = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut SystemReserve,
    )
    .unwrap();
    assert_eq!(initial.scroll_containers().len(), 2);
    let viewport_owner = initial
        .scroll_containers()
        .iter()
        .find(|container| container.owner() == nodes[0])
        .unwrap();
    let body_owner = initial
        .scroll_containers()
        .iter()
        .find(|container| container.owner() == nodes[1])
        .unwrap();
    assert_eq!(viewport_owner.max_offset_x_css_px(), 0.0);
    assert_eq!(viewport_owner.max_offset_y_css_px(), 0.0);
    assert_eq!(body_owner.parent(), Some(nodes[0]));
    assert_eq!(body_owner.max_offset_x_css_px(), 64.0);
    assert_eq!(body_owner.max_offset_y_css_px(), 484.0);

    let content_before = initial
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == nodes[2])
        .unwrap()
        .frame_css_px();
    let fixed_before = initial
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == nodes[3])
        .unwrap()
        .frame_css_px();
    let moved = build_runtime_render_snapshot_with_scroll_requests(
        &input.document,
        input.root,
        &input.output,
        input.current,
        Some(&initial),
        &[RuntimeScrollOffsetRequest::new(
            nodes[1],
            initial.key().scroll_state_revision(),
            64.0,
            83.0,
        )],
    )
    .unwrap();
    let content_after = moved
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == nodes[2])
        .unwrap()
        .frame_css_px();
    let fixed_after = moved
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == nodes[3])
        .unwrap()
        .frame_css_px();
    assert_eq!(moved.scroll_containers()[0].offset_x_css_px(), 0.0);
    assert_eq!(moved.scroll_containers()[0].offset_y_css_px(), 0.0);
    assert_eq!(content_after.x(), content_before.x() - 64.0);
    assert_eq!(content_after.y(), content_before.y() - 83.0);
    assert_eq!(fixed_after, fixed_before);
}

#[test]
fn scroll_owner_records_positive_absolute_extent_and_zero_initial_offset() {
    let styles = [
        "position:relative;display:flow-root;box-sizing:border-box;width:120px;height:82px;border:3px solid;padding:4px;overflow:auto;background-color:#101827",
        "position:absolute;left:130px;top:4px;width:30px;height:20px;background-color:#e3544e",
        "position:fixed;left:320px;top:190px;width:40px;height:20px;background-color:#57a6d9",
    ];
    let (scene, nodes) = scene_for(&styles, &[None, Some(0), Some(0)]);
    let input_owner = scene.scroll_containers();
    assert_eq!(input_owner.len(), 1);
    let container = input_owner[0];
    let scrollport = container.scrollport_css_px();
    let abs = scene
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == nodes[1])
        .unwrap()
        .frame_css_px();
    let fixed = scene
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == nodes[2])
        .unwrap()
        .frame_css_px();

    assert_eq!(container.owner(), nodes[0]);
    assert_eq!(scrollport.width(), 114.0);
    assert_eq!(scrollport.height(), 76.0);
    assert_eq!(
        container.scrollable_width_css_px(),
        (abs.x() + abs.width() - scrollport.x()).max(114.0)
    );
    assert!(container.scrollable_width_css_px() < fixed.x() + fixed.width() - scrollport.x());
    assert_eq!(
        container.max_offset_x_css_px(),
        container.scrollable_width_css_px() - 114.0
    );
    assert_eq!(container.offset_x_css_px(), 0.0);
    assert_eq!(container.offset_y_css_px(), 0.0);
}

#[test]
fn hidden_scroll_and_auto_own_zero_range_while_clip_and_visible_do_not() {
    for (value, expected_owner_count) in [
        ("visible", 0),
        ("clip", 0),
        ("hidden", 1),
        ("scroll", 1),
        ("auto", 1),
    ] {
        let style = format!(
            "position:relative;display:flow-root;box-sizing:border-box;width:120px;height:82px;overflow:{value};background-color:#101827"
        );
        let styles = [
            style.as_str(),
            "width:40px;height:24px;background-color:#57a6d9",
        ];
        let (scene, _) = scene_for(&styles, &[None, Some(0)]);
        assert_eq!(
            scene.scroll_containers().len(),
            expected_owner_count,
            "overflow:{value} owner count"
        );
        if let Some(container) = scene.scroll_containers().first() {
            assert_eq!(container.max_offset_x_css_px(), 0.0);
            assert_eq!(container.max_offset_y_css_px(), 0.0);
        }
    }
}

#[test]
fn scroll_request_moves_paint_and_hit_geometry_but_keeps_the_scrollport_fixed() {
    let styles = [
        "position:relative;display:flow-root;box-sizing:border-box;width:120px;height:82px;border:3px solid;padding:4px;overflow:auto;background-color:#101827",
        "position:absolute;left:130px;top:10px;width:30px;height:20px;background-color:#e3544e",
    ];
    let (input, nodes) =
        overflow_runtime_inputs_with_viewport(&styles, &[None, Some(0)], 400.0, 240.0);
    let initial = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut SystemReserve,
    )
    .unwrap();
    let initial_child = initial
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == nodes[1])
        .unwrap();
    let initial_frame = initial_child.frame_css_px();
    let initial_clip = initial_child.clip_css_px();
    let owner = initial.scroll_containers()[0];
    assert!(owner.max_offset_x_css_px() > 0.0);

    let request = RuntimeScrollOffsetRequest::new(
        nodes[0],
        initial.key().scroll_state_revision(),
        owner.max_offset_x_css_px(),
        0.0,
    );
    let moved = build_runtime_render_snapshot_with_scroll_requests(
        &input.document,
        input.root,
        &input.output,
        input.current,
        Some(&initial),
        &[request],
    )
    .unwrap();
    let moved_child = moved
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == nodes[1])
        .unwrap();
    let moved_frame = moved_child.frame_css_px();

    assert_eq!(moved.key().scroll_state_revision().get(), 1);
    assert_eq!(
        moved.scroll_containers()[0].offset_x_css_px(),
        owner.max_offset_x_css_px()
    );
    assert_eq!(
        moved_frame.x(),
        initial_frame.x() - owner.max_offset_x_css_px()
    );
    assert_eq!(moved_child.clip_css_px(), initial_clip);
    assert_eq!(
        moved
            .hit_test_css_point(moved_frame.x() + 2.0, moved_frame.y() + 2.0)
            .map(|render_box| render_box.node_id()),
        Some(nodes[1])
    );
    assert_ne!(
        moved
            .hit_test_css_point(initial_frame.x() + 2.0, initial_frame.y() + 2.0)
            .map(|render_box| render_box.node_id()),
        Some(nodes[1])
    );

    let no_op = build_runtime_render_snapshot_with_scroll_requests(
        &input.document,
        input.root,
        &input.output,
        input.current,
        Some(&moved),
        &[],
    )
    .unwrap();
    assert_eq!(
        no_op.key().scroll_state_revision(),
        moved.key().scroll_state_revision()
    );

    let stale_request =
        RuntimeScrollOffsetRequest::new(nodes[0], initial.key().scroll_state_revision(), 0.0, 0.0);
    assert!(matches!(
        build_runtime_render_snapshot_with_scroll_requests(
            &input.document,
            input.root,
            &input.output,
            input.current,
            Some(&moved),
            &[stale_request],
        ),
        Err(StyleRenderError::StaleScrollState)
    ));
}

#[test]
fn x_only_y_only_and_hidden_offsets_match_the_requested_axis() {
    for (overflow_x, overflow_y, width, height, expected_x, expected_y) in [
        ("auto", "hidden", 200.0, 36.0, 86.0, 0.0),
        ("hidden", "auto", 36.0, 150.0, 0.0, 74.0),
        ("hidden", "hidden", 200.0, 150.0, 86.0, 74.0),
    ] {
        let owner_style = format!(
            "position:relative;display:flow-root;box-sizing:border-box;width:120px;height:82px;border:3px solid;overflow-x:{overflow_x};overflow-y:{overflow_y};background-color:#101827"
        );
        let child_style =
            format!("position:relative;width:{width}px;height:{height}px;background-color:#57a6d9");
        let styles = [owner_style.as_str(), child_style.as_str()];
        let (scene, nodes) = scene_for(&styles, &[None, Some(0)]);
        let [owner] = scene.scroll_containers() else {
            panic!("overflow-{overflow_x}/{overflow_y} must create one owner")
        };
        assert_eq!(owner.max_offset_x_css_px(), expected_x);
        assert_eq!(owner.max_offset_y_css_px(), expected_y);

        let moved = scene
            .with_scroll_offsets(
                scene.key().scroll_state_revision(),
                &[(nodes[0], 55.0, 44.0)],
            )
            .unwrap();
        assert_eq!(
            moved.scroll_containers()[0].offset_x_css_px(),
            55.0_f32.min(expected_x)
        );
        assert_eq!(
            moved.scroll_containers()[0].offset_y_css_px(),
            44.0_f32.min(expected_y)
        );
    }
}

#[test]
fn display_none_subtree_does_not_create_scroll_owners_or_extent() {
    let styles = [
        "display:flow-root;box-sizing:border-box;width:80px;height:60px;overflow:auto;background-color:#101827",
        "display:none;position:relative;box-sizing:border-box;width:40px;height:30px;overflow:auto;background-color:#334155",
        "position:absolute;left:500px;top:400px;width:300px;height:300px;background-color:#e3544e",
    ];
    let (scene, nodes) = scene_for(&styles, &[None, Some(0), Some(1)]);

    assert_eq!(scene.scroll_containers().len(), 1);
    assert_eq!(scene.scroll_containers()[0].owner(), nodes[0]);
    assert_eq!(scene.scroll_containers()[0].max_offset_x_css_px(), 0.0);
    assert_eq!(scene.scroll_containers()[0].max_offset_y_css_px(), 0.0);
    assert!(
        scene
            .boxes()
            .iter()
            .all(|render_box| render_box.node_id() != nodes[1])
    );
    assert!(
        scene
            .boxes()
            .iter()
            .all(|render_box| render_box.node_id() != nodes[2])
    );
}

#[test]
fn clip_has_no_programmatic_offset_owner() {
    let styles = [
        "position:relative;display:flow-root;box-sizing:border-box;width:120px;height:82px;border:3px solid;overflow:clip;background-color:#101827",
        "position:absolute;left:90px;top:70px;width:80px;height:50px;background-color:#57a6d9",
    ];
    let (input, nodes) =
        overflow_runtime_inputs_with_viewport(&styles, &[None, Some(0)], 400.0, 240.0);
    let scene = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut SystemReserve,
    )
    .unwrap();
    assert!(scene.scroll_containers().is_empty());
    assert!(matches!(
        build_runtime_render_snapshot_with_scroll_requests(
            &input.document,
            input.root,
            &input.output,
            input.current,
            Some(&scene),
            &[RuntimeScrollOffsetRequest::new(
                nodes[0],
                scene.key().scroll_state_revision(),
                55.0,
                44.0,
            )],
        ),
        Err(StyleRenderError::InvalidScrollRequest(owner)) if owner == nodes[0]
    ));
}

#[test]
fn nested_owners_keep_independent_ranges_offsets_and_clip_ancestry() {
    let styles = [
        "position:relative;display:flow-root;width:400px;height:240px;background-color:#101827",
        "position:absolute;left:8px;top:8px;box-sizing:border-box;width:150px;height:104px;border:3px solid;overflow:auto;background-color:#ffffff",
        "width:232px;height:176px;padding:6px;background-color:#344256",
        "width:126px;height:88px;box-sizing:border-box;border:2px solid;overflow:auto;background-color:#ffffff",
        "width:188px;height:138px;background-color:#57a6d9",
    ];
    let parents = [None, Some(0), Some(1), Some(2), Some(3)];
    let (input, nodes) = overflow_runtime_inputs_with_viewport(&styles, &parents, 400.0, 240.0);
    let initial = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut SystemReserve,
    )
    .unwrap();
    assert_eq!(initial.scroll_containers().len(), 2);
    let outer = initial
        .scroll_containers()
        .iter()
        .find(|owner| owner.owner() == nodes[1])
        .unwrap();
    let inner = initial
        .scroll_containers()
        .iter()
        .find(|owner| owner.owner() == nodes[3])
        .unwrap();
    assert_eq!(
        (outer.max_offset_x_css_px(), outer.max_offset_y_css_px()),
        (100.0, 90.0)
    );
    assert_eq!(
        (inner.max_offset_x_css_px(), inner.max_offset_y_css_px()),
        (66.0, 54.0)
    );
    let inner_box = initial
        .boxes()
        .iter()
        .find(|item| item.node_id() == nodes[3])
        .unwrap();
    let fill_box = initial
        .boxes()
        .iter()
        .find(|item| item.node_id() == nodes[4])
        .unwrap();
    let inner_before = inner_box.frame_css_px();
    let fill_before = fill_box.frame_css_px();
    let inner_clip_before = fill_box.clip_css_px();

    let moved = initial
        .with_scroll_offsets(
            initial.key().scroll_state_revision(),
            &[(nodes[1], 22.0, 17.0), (nodes[3], 42.0, 29.0)],
        )
        .unwrap();
    let inner_after = moved
        .boxes()
        .iter()
        .find(|item| item.node_id() == nodes[3])
        .unwrap();
    let fill_after = moved
        .boxes()
        .iter()
        .find(|item| item.node_id() == nodes[4])
        .unwrap();
    assert_eq!(inner_after.frame_css_px().x(), inner_before.x() - 22.0);
    assert_eq!(inner_after.frame_css_px().y(), inner_before.y() - 17.0);
    assert_eq!(fill_after.frame_css_px().x(), fill_before.x() - 64.0);
    assert_eq!(fill_after.frame_css_px().y(), fill_before.y() - 46.0);
    assert_ne!(fill_after.clip_css_px(), inner_clip_before);
    assert_eq!(moved.scroll_containers()[0].offset_x_css_px(), 22.0);
    assert_eq!(moved.scroll_containers()[1].offset_x_css_px(), 42.0);
}

#[test]
fn scroll_range_growth_shrink_detach_and_reparent_reconcile_atomically() {
    let styles = [
        "position:relative;display:flow-root;box-sizing:border-box;width:120px;height:82px;border:3px solid;overflow:auto;background-color:#101827",
        "width:220px;height:160px;background-color:#57a6d9",
    ];
    let (input, nodes) =
        overflow_runtime_inputs_with_viewport(&styles, &[None, Some(0)], 400.0, 240.0);
    let baseline = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut SystemReserve,
    )
    .unwrap();
    let owner = nodes[0];
    let scrolled = baseline
        .with_scroll_offsets(
            baseline.key().scroll_state_revision(),
            &[(owner, 70.0, 60.0)],
        )
        .unwrap();
    let generation = scrolled.key().generation();
    let port = scrolled.scroll_containers()[0].scrollport_css_px();
    let make_owner = |width, height, parent| {
        RuntimeScrollContainer::new(owner, port, width, height, 0.0, 0.0)
            .unwrap()
            .with_parent(parent)
    };

    let (growth_state, growth_revision) = super::super::scroll::reconcile_scroll_containers(
        vec![make_owner(264.0, 204.0, None)],
        Some(&scrolled),
        &[],
        generation,
    )
    .unwrap();
    assert_eq!(
        (
            growth_state[0].offset_x_css_px(),
            growth_state[0].offset_y_css_px()
        ),
        (70.0, 60.0)
    );
    assert_eq!(
        growth_revision.get(),
        scrolled.key().scroll_state_revision().get() + 1
    );

    let after_growth = scrolled.with_scroll_state_revision(growth_revision);
    let (shrink_state, shrink_revision) = super::super::scroll::reconcile_scroll_containers(
        vec![make_owner(148.0, 96.0, None)],
        Some(&after_growth),
        &[],
        generation,
    )
    .unwrap();
    assert_eq!(
        (
            shrink_state[0].offset_x_css_px(),
            shrink_state[0].offset_y_css_px()
        ),
        (34.0, 20.0)
    );
    assert_eq!(shrink_revision.get(), growth_revision.get() + 1);

    let shrink_snapshot = RuntimeRenderSnapshot::new_with_scroll_containers(
        scrolled.key().with_scroll_state_revision(shrink_revision),
        scrolled.viewport_css_px(),
        scrolled.boxes().to_vec(),
        shrink_state.clone(),
    )
    .unwrap();
    let (detached_state, detached_revision) = super::super::scroll::reconcile_scroll_containers(
        Vec::new(),
        Some(&shrink_snapshot),
        &[],
        generation,
    )
    .unwrap();
    assert!(detached_state.is_empty());
    assert_eq!(detached_revision.get(), shrink_revision.get() + 1);

    let detached_snapshot = RuntimeRenderSnapshot::new_with_scroll_containers(
        shrink_snapshot
            .key()
            .with_scroll_state_revision(detached_revision),
        shrink_snapshot.viewport_css_px(),
        shrink_snapshot.boxes().to_vec(),
        detached_state,
    )
    .unwrap();
    let after_detach = detached_snapshot.with_scroll_state_revision(detached_revision);
    let (reattached_state, reattached_revision) =
        super::super::scroll::reconcile_scroll_containers(
            vec![make_owner(264.0, 204.0, None)],
            Some(&after_detach),
            &[],
            generation,
        )
        .unwrap();
    assert_eq!(
        (
            reattached_state[0].offset_x_css_px(),
            reattached_state[0].offset_y_css_px()
        ),
        (0.0, 0.0)
    );
    assert_eq!(reattached_revision.get(), detached_revision.get() + 1);

    let moved_parent = NodeId::new(999_999).unwrap();
    let (reparented_state, reparented_revision) =
        super::super::scroll::reconcile_scroll_containers(
            vec![make_owner(264.0, 204.0, Some(moved_parent))],
            Some(&scrolled),
            &[],
            generation,
        )
        .unwrap();
    assert_eq!(
        (
            reparented_state[0].offset_x_css_px(),
            reparented_state[0].offset_y_css_px()
        ),
        (0.0, 0.0)
    );
    assert_eq!(
        reparented_revision.get(),
        scrolled.key().scroll_state_revision().get() + 1
    );
}

#[test]
fn scroll_reservation_failures_leave_no_partial_snapshot_and_recover() {
    let styles = [
        "position:relative;display:flow-root;box-sizing:border-box;width:120px;height:82px;border:3px solid;overflow:auto;background-color:#101827",
        "position:absolute;left:130px;top:10px;width:30px;height:20px;background-color:#57a6d9",
    ];
    let (input, _) = overflow_runtime_inputs_with_viewport(&styles, &[None, Some(0)], 400.0, 240.0);
    let baseline = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut SystemReserve,
    )
    .unwrap();
    let mut recording = FaultInjectingReserve::recording();
    let recorded = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut recording,
    )
    .unwrap();
    assert_eq!(recorded, baseline);

    for stage in [
        "scroll containers",
        "scroll descendant ancestry",
        "scroll render ancestry",
    ] {
        assert!(
            recording.stages().contains(&stage),
            "missing reservation stage {stage}"
        );
        let mut injected = FaultInjectingReserve::failing_at_stage_occurrence(stage, 1);
        let error = build_runtime_render_snapshot_with_reserve(
            &input.document,
            input.root,
            &input.output,
            input.current,
            &mut injected,
        )
        .expect_err("scroll reservation failure must discard the candidate snapshot");
        assert!(
            matches!(error, StyleRenderError::AllocationFailed { stage: actual } if actual == stage),
            "{stage} failure returned {error}"
        );

        let recovered = build_runtime_render_snapshot_with_reserve(
            &input.document,
            input.root,
            &input.output,
            input.current,
            &mut injected,
        )
        .expect("the same reserve must recover on the next full build");
        assert_eq!(recovered, baseline, "recovery after {stage}");
    }
}
