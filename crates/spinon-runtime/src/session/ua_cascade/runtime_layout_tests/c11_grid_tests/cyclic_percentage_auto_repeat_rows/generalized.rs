use super::*;

#[derive(Clone, Copy)]
struct SceneOptions {
    root_width: &'static str,
    root_height: &'static str,
    columns: &'static str,
    rows: &'static str,
    row_gap: &'static str,
    align_content: &'static str,
    child_count: usize,
}

fn options_for_scene(root_height: &'static str, mode: &'static str) -> SceneOptions {
    SceneOptions {
        root_width: "24px",
        root_height,
        columns: "24px",
        rows: match mode {
            "auto-fill" => "8px repeat(auto-fill,minmax(0px,25%)) 8px",
            "auto-fit" => "8px repeat(auto-fit,minmax(0px,25%)) 8px",
            _ => unreachable!("known generalized row mode"),
        },
        row_gap: "2px",
        align_content: "start",
        child_count: 3,
    }
}

struct Probe {
    request: WorkRequest,
    root_id: u64,
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    child_ids: [u64; 3],
}

fn request_for_scene(scene_id: &str, options: SceneOptions) -> Probe {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let children = [
        document.reserve_node_handle().unwrap(),
        document.reserve_node_handle().unwrap(),
        document.reserve_node_handle().unwrap(),
    ];
    let owner = OwnerId::new(11_429).unwrap();
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    let root_style = format!(
        "box-sizing:content-box;display:grid;width:{};height:{};grid-template-columns:{};grid-template-rows:{};column-gap:0;row-gap:{};margin:0;padding:0;border:0;align-content:{};justify-content:start;grid-auto-flow:row",
        options.root_width,
        options.root_height,
        options.columns,
        options.rows,
        options.row_gap,
        options.align_content,
    );
    let root_id = format!("c114-{scene_id}-grid");
    push_element(&mut batch, root, &root_id, HostParent::Root, &root_style);

    let child_styles = [
        ("prefix", "24px", "8px", "1"),
        ("primary", "24px", "24px", "2"),
        ("secondary", "16px", "16px", "2"),
    ];
    for (index, (role, width, height, row)) in child_styles.iter().enumerate() {
        if index >= options.child_count {
            break;
        }
        let child_id = format!("c114-{scene_id}-{role}");
        let style = format!(
            "display:block;box-sizing:border-box;width:{width};height:{height};min-width:0;min-height:0;grid-row:{row};grid-column:1;margin:0;padding:0;border:0"
        );
        push_element(
            &mut batch,
            children[index],
            &child_id,
            HostParent::Node(root),
            &style,
        );
    }

    document.commit(batch).unwrap();
    let snapshot = Arc::new(document.snapshot());
    Probe {
        request: WorkRequest {
            key: key_for(&snapshot, EnvironmentRevision::INITIAL),
            snapshot,
            viewport: CssViewport {
                width_css_px: 120.0,
                height_css_px: 80.0,
                device_scale_factor: 1.0,
                environment_revision: EnvironmentRevision::INITIAL,
                media_environment: CssMediaEnvironment::MOBILE,
            },
            previous_snapshot: None,
            force_full: true,
            invalidation: None,
            previous_styles: None,
        },
        root_id: root.id().get(),
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        child_ids: children.map(|child| child.id().get()),
    }
}

fn push_element(
    batch: &mut DocumentChangeBatch,
    node: HostNodeHandle,
    dom_id: &str,
    parent: HostParent,
    style: &str,
) {
    batch.push(DocumentOperation::CreateElement {
        node,
        namespace: "http://www.w3.org/1999/xhtml".to_owned(),
        local_name: "div".to_owned(),
    });
    for (name, value) in [("id", dom_id.to_owned()), ("style", style.to_owned())] {
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, name).unwrap(),
            value: value.into(),
        });
    }
    batch.push(DocumentOperation::InsertBefore {
        parent,
        node,
        before: None,
    });
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn assert_frame(
    completed: &RuntimeLayoutCompleted,
    node_id: u64,
    expected: (f32, f32, f32, f32),
    scene: &str,
) {
    let frame = completed
        .frames
        .iter()
        .find(|frame| frame.node_id == node_id)
        .unwrap_or_else(|| panic!("{scene} missing frame for node {node_id}"));
    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        expected,
        "{scene} node {node_id} frame"
    );
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn assert_tracks(
    pass: &spinon_layout::LayoutGridTrackSizingPassTrace,
    expected: &[(bool, bool, f32)],
    scene: &str,
) {
    let tracks = &pass.stages.last().expect("final row trace stage").tracks;
    assert_eq!(tracks.len(), expected.len(), "{scene} track-state count");
    for (index, (actual, (is_gutter, is_collapsed, base_size))) in
        tracks.iter().zip(expected).enumerate()
    {
        assert_eq!(actual.internal_index, index as u16, "{scene} index {index}");
        assert_eq!(actual.is_gutter, *is_gutter, "{scene} index {index} gutter");
        assert_eq!(
            actual.is_collapsed, *is_collapsed,
            "{scene} index {index} collapsed"
        );
        assert!(
            (actual.base_size - base_size).abs() <= 1.0 / 64.0,
            "{scene} index {index} size: {} != {base_size}",
            actual.base_size
        );
    }
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn final_used_tracks(pass: &spinon_layout::LayoutGridTrackSizingPassTrace) -> Vec<f32> {
    pass.stages
        .last()
        .expect("final track trace stage")
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect()
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn assert_rejected(probe: &Probe, expected_property: &str, scene: &str) {
    let calculation = compute_request_for_block_flex_grid_track_sizing(&probe.request).unwrap();
    let failure = calculation.layout.unwrap_err();
    assert_eq!(failure.code, "invalid_grid_track_list", "{scene} code");
    assert_eq!(failure.node_id, Some(probe.root_id), "{scene} owner");
    assert_eq!(
        failure.property.as_deref(),
        Some(expected_property),
        "{scene} property"
    );
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn four_generalized_row_scenes_preserve_frames_tracks_and_trace_phases() {
    use spinon_layout::{
        LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
        LayoutGridTrackSizingFixturePhase,
    };

    let scenes = [
        (
            "cyclic-max-content-auto-fill-prefix-suffix",
            "max-content",
            "auto-fill",
            1,
            2,
            Some(44.0),
            &[8.0, 11.0, 8.0][..],
            13.0,
            &[
                (true, true, 0.0),
                (false, false, 8.0),
                (true, false, 2.0),
                (false, false, 11.0),
                (true, false, 2.0),
                (false, false, 8.0),
                (true, true, 0.0),
            ][..],
            23.0,
        ),
        (
            "cyclic-max-content-auto-fit-prefix-suffix",
            "max-content",
            "auto-fit",
            1,
            2,
            Some(44.0),
            &[8.0, 11.0, 8.0][..],
            13.0,
            &[
                (true, true, 0.0),
                (false, false, 8.0),
                (true, false, 2.0),
                (false, false, 11.0),
                (true, false, 2.0),
                (false, false, 8.0),
                (true, true, 0.0),
            ][..],
            23.0,
        ),
        (
            "definite-44px-auto-fill-prefix-suffix-control",
            "44px",
            "auto-fill",
            2,
            1,
            Some(44.0),
            &[8.0, 11.0, 11.0, 8.0][..],
            0.0,
            &[
                (true, true, 0.0),
                (false, false, 8.0),
                (true, false, 2.0),
                (false, false, 11.0),
                (true, false, 2.0),
                (false, false, 11.0),
                (true, false, 2.0),
                (false, false, 8.0),
                (true, true, 0.0),
            ][..],
            36.0,
        ),
        (
            "definite-44px-auto-fit-prefix-suffix-control",
            "44px",
            "auto-fit",
            2,
            1,
            Some(44.0),
            &[8.0, 11.0, 8.0][..],
            13.0,
            &[
                (true, true, 0.0),
                (false, false, 8.0),
                (true, false, 2.0),
                (false, false, 11.0),
                (true, false, 2.0),
                (false, true, 0.0),
                (true, true, 0.0),
                (false, false, 8.0),
                (true, true, 0.0),
            ][..],
            23.0,
        ),
    ];

    for (
        scene,
        height,
        mode,
        repeat_count,
        row_pass_count,
        basis,
        expected_rows,
        trailing_space,
        expected_tracks,
        suffix_y,
    ) in scenes
    {
        let probe = request_for_scene(scene, options_for_scene(height, mode));
        let calculation = compute_request_for_block_flex_grid_track_sizing(&probe.request).unwrap();
        let completed = calculation
            .layout
            .unwrap_or_else(|failure| panic!("{scene} layout failed: {failure:?}"));
        assert_eq!(completed.key, probe.request.key, "{scene} runtime key");
        assert_eq!(
            completed.frames.len(),
            4,
            "{scene} root and three children only"
        );
        assert_frame(&completed, probe.root_id, (0.0, 0.0, 24.0, 44.0), scene);
        assert_frame(&completed, probe.child_ids[0], (0.0, 0.0, 24.0, 8.0), scene);
        assert_frame(
            &completed,
            probe.child_ids[1],
            (0.0, 10.0, 24.0, 24.0),
            scene,
        );
        assert_frame(
            &completed,
            probe.child_ids[2],
            (0.0, 10.0, 16.0, 16.0),
            scene,
        );

        let diagnostics = completed
            .grid_track_sizing_diagnostics
            .as_ref()
            .expect("fixture diagnostics");
        let trace = diagnostics
            .traces
            .iter()
            .find(|trace| trace.node.get() == probe.root_id)
            .unwrap_or_else(|| panic!("{scene} root trace"));
        let row_passes = trace
            .passes
            .iter()
            .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
            .collect::<Vec<_>>();
        assert_eq!(row_passes.len(), row_pass_count, "{scene} row pass count");

        if height == "max-content" {
            assert_eq!(
                row_passes[0].fixture_phase,
                Some(LayoutGridTrackSizingFixturePhase::IntrinsicMaxContent),
                "{scene} intrinsic phase"
            );
            assert_eq!(row_passes[0].fixture_auto_repeat_count, Some(1));
            assert_eq!(row_passes[0].fixture_percentage_basis, None);
            let contributions = &row_passes[0].contributions;
            assert_eq!(contributions.len(), 2, "{scene} contribution count");
            for (node_id, expected) in [(probe.child_ids[1], 24.0), (probe.child_ids[2], 16.0)] {
                let contribution = contributions
                    .iter()
                    .find(|contribution| contribution.item.get() == node_id)
                    .unwrap_or_else(|| panic!("{scene} missing item {node_id} contribution"));
                assert_eq!(
                    contribution.kind,
                    LayoutGridTrackSizingContributionKind::MaxContent,
                    "{scene} contribution kind"
                );
                assert_eq!(contribution.span, 1, "{scene} contribution span");
                assert_eq!(contribution.value, expected, "{scene} contribution size");
            }
            assert_tracks(
                row_passes[0],
                &[
                    (true, true, 0.0),
                    (false, false, 8.0),
                    (true, false, 2.0),
                    (false, false, 24.0),
                    (true, false, 2.0),
                    (false, false, 8.0),
                    (true, true, 0.0),
                ],
                scene,
            );
            assert_eq!(
                row_passes[1].fixture_phase,
                Some(LayoutGridTrackSizingFixturePhase::FinalDefinite),
                "{scene} final phase"
            );
            assert_eq!(row_passes[1].fixture_auto_repeat_count, Some(1));
            assert_eq!(row_passes[1].fixture_percentage_basis, Some(44.0));
            assert!(
                row_passes[0].sequence < row_passes[1].sequence,
                "{scene} intrinsic pass precedes final pass"
            );
        } else {
            assert_eq!(
                row_passes[0].fixture_phase,
                Some(LayoutGridTrackSizingFixturePhase::FinalDefinite),
                "{scene} definite phase"
            );
        }

        let final_pass = row_passes.last().unwrap();
        assert_eq!(final_pass.fixture_auto_repeat_count, Some(repeat_count));
        assert_eq!(final_pass.fixture_percentage_basis, basis);
        assert_tracks(final_pass, expected_tracks, scene);
        let final_tracks = &final_pass.stages.last().unwrap().tracks;
        let suffix_index = final_tracks.len() - 2;
        let suffix_start = final_tracks[..suffix_index]
            .iter()
            .map(|track| track.base_size)
            .sum::<f32>();
        assert_eq!(suffix_start, suffix_y, "{scene} suffix row start");
        let track_and_gutter_sum = final_tracks
            .iter()
            .map(|track| track.base_size)
            .sum::<f32>();
        assert_eq!(
            44.0 - track_and_gutter_sum,
            trailing_space,
            "{scene} trailing space"
        );
        let used_rows = final_used_tracks(final_pass);
        assert_eq!(used_rows, expected_rows, "{scene} used rows");
        assert_eq!(24.0 - used_rows[1], 13.0, "{scene} primary overflow");
        assert_eq!(16.0 - used_rows[1], 5.0, "{scene} secondary overflow");

        let column_pass = trace
            .passes
            .iter()
            .rev()
            .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
            .unwrap_or_else(|| panic!("{scene} column trace"));
        assert_eq!(
            final_used_tracks(column_pass),
            vec![24.0],
            "{scene} columns"
        );
    }
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn generalized_row_mutations_fail_at_the_expected_property_owner() {
    let mut options = options_for_scene("max-content", "auto-fill");
    options.rows = "8px repeat(auto-fill,minmax(0px,25%)) 9px";
    assert_rejected(
        &request_for_scene("generalized-suffix-track-mutation", options),
        "grid-template-rows",
        "suffix track mutation",
    );

    let mut options = options_for_scene("max-content", "auto-fill");
    options.child_count = 2;
    assert_rejected(
        &request_for_scene("generalized-missing-child-mutation", options),
        "grid-template-rows",
        "missing direct child mutation",
    );

    let mut options = options_for_scene("max-content", "auto-fill");
    options.root_width = "max-content";
    options.columns = "repeat(auto-fill,minmax(0px,25%))";
    assert_rejected(
        &request_for_scene("generalized-two-cyclic-axes-mutation", options),
        "grid-template-columns",
        "two cyclic axes mutation",
    );
}

#[cfg(not(feature = "c11-4-grid-track-sizing-fixture-test"))]
#[test]
fn generalized_cyclic_rows_remain_rejected_without_fixture_feature() {
    let probe = request_for_scene(
        "cyclic-max-content-auto-fill-prefix-suffix",
        options_for_scene("max-content", "auto-fill"),
    );
    let calculation = compute_request_for_block_flex_grid_track_sizing(&probe.request).unwrap();
    let failure = calculation.layout.unwrap_err();
    assert_eq!(failure.code, "invalid_grid_track_list");
    assert_eq!(failure.node_id, Some(probe.root_id));
    assert_eq!(failure.property.as_deref(), Some("grid-template-rows"));
}
