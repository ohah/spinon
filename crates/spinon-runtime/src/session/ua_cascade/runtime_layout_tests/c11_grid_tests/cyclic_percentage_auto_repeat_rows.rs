use super::*;

#[path = "cyclic_percentage_auto_repeat_rows/generalized.rs"]
mod generalized;

#[derive(Clone, Copy)]
struct SceneOptions {
    root_width: &'static str,
    root_height: &'static str,
    columns: &'static str,
    rows: &'static str,
    row_gap: &'static str,
    align_content: &'static str,
    grid_auto_flow: &'static str,
    item_row: &'static str,
    item_min_height: &'static str,
    root_padding_top: &'static str,
    root_min_height: Option<&'static str>,
    extra_in_flow_child: bool,
    absolute_marker_child: bool,
}

fn target_options() -> SceneOptions {
    SceneOptions {
        root_width: "24px",
        root_height: "max-content",
        columns: "24px",
        rows: "repeat(auto-fill,minmax(0px,50%))",
        row_gap: "0",
        align_content: "start",
        grid_auto_flow: "row",
        item_row: "1",
        item_min_height: "0",
        root_padding_top: "0",
        root_min_height: None,
        extra_in_flow_child: false,
        absolute_marker_child: false,
    }
}

struct Probe {
    request: WorkRequest,
    root_id: u64,
    item_id: u64,
}

fn request_for_target(options: SceneOptions) -> Probe {
    request_for_scene(
        "c114-cyclic-max-content-auto-fill-rows-grid",
        "c114-cyclic-max-content-auto-fill-rows-item",
        "c114-cyclic-max-content-auto-fill-rows-extra",
        options,
    )
}

fn request_for_control(options: SceneOptions) -> Probe {
    request_for_scene(
        "c114-definite-40px-auto-fill-control-rows-grid",
        "c114-definite-40px-auto-fill-control-rows-item",
        "c114-definite-40px-auto-fill-control-rows-extra",
        options,
    )
}

fn request_for_scene(
    root_dom_id: &str,
    item_dom_id: &str,
    extra_dom_id: &str,
    options: SceneOptions,
) -> Probe {
    const HTML: &str = "http://www.w3.org/1999/xhtml";
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let item = document.reserve_node_handle().unwrap();
    let extra = (options.extra_in_flow_child || options.absolute_marker_child)
        .then(|| document.reserve_node_handle().unwrap());
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(11_428).unwrap(), document.document_revision());
    let root_min_height = options
        .root_min_height
        .map(|value| format!("min-height:{value};"))
        .unwrap_or_default();
    let root_style = format!(
        "box-sizing:content-box;display:grid;width:{};height:{};grid-template-columns:{};grid-template-rows:{};column-gap:0;row-gap:{};margin:0;padding:0;padding-top:{};border:0;align-content:{};justify-content:start;grid-auto-flow:{};{}",
        options.root_width,
        options.root_height,
        options.columns,
        options.rows,
        options.row_gap,
        options.root_padding_top,
        options.align_content,
        options.grid_auto_flow,
        root_min_height,
    );
    let item_style = format!(
        "display:block;box-sizing:border-box;width:24px;height:16px;min-width:0;min-height:{};grid-row:{};grid-column:1;margin:0;padding:0;border:0",
        options.item_min_height, options.item_row,
    );
    for (node, dom_id, parent, style) in [
        (root, root_dom_id, HostParent::Root, root_style),
        (item, item_dom_id, HostParent::Node(root), item_style),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        for (name, value) in [("id", dom_id.to_owned()), ("style", style)] {
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
    if let Some(extra) = extra {
        let (dom_id, style) = if options.absolute_marker_child {
            (
                "c114-cyclic-max-content-auto-fill-rows-browser-marker",
                "display:block;position:absolute;left:0;right:0;top:50%;width:24px;height:1px;margin:0;padding:0;border:0",
            )
        } else {
            (
                extra_dom_id,
                "display:block;box-sizing:border-box;width:1px;height:1px;margin:0;padding:0;border:0",
            )
        };
        batch.push(DocumentOperation::CreateElement {
            node: extra,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        for (name, value) in [("id", dom_id.to_owned()), ("style", style.to_owned())] {
            batch.push(DocumentOperation::SetAttribute {
                node: extra,
                name: AttributeName::new(None, name).unwrap(),
                value: value.into(),
            });
        }
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: extra,
            before: None,
        });
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
        item_id: item.id().get(),
    }
}

fn assert_frame(completed: &RuntimeLayoutCompleted, node_id: u64, expected: (f32, f32, f32, f32)) {
    let frame = completed
        .frames
        .iter()
        .find(|frame| frame.node_id == node_id)
        .unwrap();
    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        expected,
        "node {node_id} frame"
    );
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn assert_rejected_target(probe: &Probe, mutation: &str) {
    let calculation = compute_request_for_block_flex_grid_track_sizing(&probe.request).unwrap();
    let failure = calculation.layout.unwrap_err();
    if mutation == "target-alignment" {
        assert_eq!(failure.code, "unsupported_computed_value", "{mutation}");
        assert_eq!(failure.node_id, Some(probe.root_id), "{mutation} owner");
        assert_eq!(
            failure.property.as_deref(),
            Some("align-content"),
            "{mutation} property"
        );
        return;
    }
    assert_eq!(failure.code, "invalid_grid_track_list", "{mutation}");
    assert_eq!(failure.node_id, Some(probe.root_id), "{mutation} owner");
    assert_eq!(
        failure.property.as_deref(),
        Some("grid-template-rows"),
        "{mutation} property"
    );
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn final_used_rows(pass: &spinon_layout::LayoutGridTrackSizingPassTrace) -> Vec<f32> {
    pass.stages
        .last()
        .unwrap()
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect()
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn assert_rows(actual: &[f32], expected: &[f32], scene: &str) {
    assert_eq!(actual.len(), expected.len(), "{scene} track count");
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert!(
            (actual - expected).abs() <= 1.0 / 64.0,
            "{scene} row {index}: {actual} != {expected}"
        );
    }
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn cyclic_percentage_auto_repeat_rows_preserves_count_and_same_pass_basis() {
    let probe = request_for_target(target_options());
    let calculation = compute_request_for_block_flex_grid_track_sizing(&probe.request).unwrap();
    let completed = calculation.layout.unwrap();
    assert_frame(&completed, probe.root_id, (0.0, 0.0, 24.0, 16.0));
    assert_frame(&completed, probe.item_id, (0.0, 0.0, 24.0, 16.0));

    let diagnostics = completed.grid_track_sizing_diagnostics.unwrap();
    let trace = diagnostics
        .traces
        .iter()
        .find(|trace| trace.node.get() == probe.root_id)
        .unwrap();
    let item = completed
        .frames
        .iter()
        .find(|frame| frame.node_id == probe.item_id)
        .unwrap();
    let row_passes = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == spinon_layout::LayoutGridTrackSizingAxis::Rows)
        .collect::<Vec<_>>();
    assert_eq!(
        row_passes.len(),
        2,
        "target must expose both row sizing passes"
    );
    assert_eq!(
        row_passes[0].fixture_phase,
        Some(spinon_layout::LayoutGridTrackSizingFixturePhase::IntrinsicMaxContent),
        "intrinsic row sizing must precede final definite sizing"
    );
    assert_eq!(
        row_passes[1].fixture_phase,
        Some(spinon_layout::LayoutGridTrackSizingFixturePhase::FinalDefinite),
        "final definite row sizing must follow intrinsic sizing"
    );
    assert!(
        row_passes[0].sequence < row_passes[1].sequence,
        "row pass sequence must increase with intrinsic-to-final phase order"
    );
    let intrinsic = row_passes
        .iter()
        .copied()
        .find(|pass| {
            pass.fixture_phase
                == Some(spinon_layout::LayoutGridTrackSizingFixturePhase::IntrinsicMaxContent)
        })
        .unwrap();
    let final_definite = row_passes
        .iter()
        .copied()
        .find(|pass| {
            pass.fixture_phase
                == Some(spinon_layout::LayoutGridTrackSizingFixturePhase::FinalDefinite)
        })
        .unwrap();

    assert_eq!(intrinsic.fixture_auto_repeat_count, Some(1));
    assert_eq!(intrinsic.fixture_percentage_basis, None);
    assert_rows(&final_used_rows(intrinsic), &[16.0], "intrinsic target");
    assert_eq!(final_definite.fixture_auto_repeat_count, Some(1));
    assert_eq!(final_definite.fixture_percentage_basis, Some(16.0));
    let rows = final_used_rows(final_definite);
    assert_rows(&rows, &[8.0], "final target");
    assert_eq!(
        item.y + item.height - (item.y + rows[0]),
        8.0,
        "the item extends 8px past the only row track"
    );
}

#[test]
fn definite_40px_row_control_passes_general_admission() {
    let mut options = target_options();
    options.root_height = "40px";
    let probe = request_for_control(options);
    let calculation = compute_request_for_block_flex_grid_track_sizing(&probe.request).unwrap();
    let completed = calculation.layout.unwrap();
    assert_frame(&completed, probe.root_id, (0.0, 0.0, 24.0, 40.0));
    assert_frame(&completed, probe.item_id, (0.0, 0.0, 24.0, 16.0));
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn definite_40px_row_control_trace_uses_two_tracks_on_general_path() {
    let mut options = target_options();
    options.root_height = "40px";
    let probe = request_for_control(options);
    let calculation = compute_request_for_block_flex_grid_track_sizing(&probe.request).unwrap();
    let completed = calculation.layout.unwrap();
    let diagnostics = completed.grid_track_sizing_diagnostics.unwrap();
    let trace = diagnostics
        .traces
        .iter()
        .find(|trace| trace.node.get() == probe.root_id)
        .unwrap();
    let row_passes = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == spinon_layout::LayoutGridTrackSizingAxis::Rows)
        .collect::<Vec<_>>();
    assert_eq!(row_passes.len(), 1, "control must have one row pass");
    assert_eq!(
        row_passes[0].fixture_phase,
        Some(spinon_layout::LayoutGridTrackSizingFixturePhase::FinalDefinite)
    );
    assert!(
        row_passes[0].sequence > 0,
        "row control sequence is present"
    );
    let rows = row_passes[0];
    assert_eq!(rows.fixture_auto_repeat_count, Some(2));
    assert_eq!(rows.fixture_percentage_basis, Some(40.0));
    assert_rows(&final_used_rows(rows), &[20.0, 20.0], "definite control");
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn cyclic_percentage_row_scene_mutations_fail_closed() {
    let mut options = target_options();
    options.root_width = "23px";
    assert_rejected_target(&request_for_target(options), "target-width");

    let mut options = target_options();
    options.columns = "23px";
    assert_rejected_target(&request_for_target(options), "target-columns");

    let mut options = target_options();
    options.rows = "repeat(auto-fit,minmax(0px,50%))";
    assert_rejected_target(&request_for_target(options), "target-repeat-mode");

    let mut options = target_options();
    options.row_gap = "1px";
    assert_rejected_target(&request_for_target(options), "target-row-gap");

    let mut options = target_options();
    options.align_content = "center";
    assert_rejected_target(&request_for_target(options), "target-alignment");

    let mut options = target_options();
    options.grid_auto_flow = "column";
    assert_rejected_target(&request_for_target(options), "target-grid-flow");

    let mut options = target_options();
    options.item_row = "2";
    assert_rejected_target(&request_for_target(options), "target-placement");

    let mut options = target_options();
    options.item_min_height = "1px";
    assert_rejected_target(&request_for_target(options), "target-min-height");

    let mut options = target_options();
    options.root_padding_top = "1px";
    assert_rejected_target(&request_for_target(options), "target-inset");

    let mut options = target_options();
    options.extra_in_flow_child = true;
    assert_rejected_target(&request_for_target(options), "target-extra-inflow-child");

    let mut options = target_options();
    options.absolute_marker_child = true;
    assert_rejected_target(
        &request_for_target(options),
        "target-browser-marker-as-node",
    );

    let mut options = target_options();
    options.root_min_height = Some("0px");
    assert_rejected_target(
        &request_for_target(options),
        "target-root-min-height-fixed-zero",
    );
}

#[cfg(not(feature = "c11-4-grid-track-sizing-fixture-test"))]
#[test]
fn cyclic_percentage_rows_remain_rejected_without_fixture_feature() {
    let probe = request_for_target(target_options());
    let calculation = compute_request_for_block_flex_grid_track_sizing(&probe.request).unwrap();
    let failure = calculation.layout.unwrap_err();
    assert_eq!(failure.code, "invalid_grid_track_list");
    assert_eq!(failure.node_id, Some(probe.root_id));
    assert_eq!(failure.property.as_deref(), Some("grid-template-rows"));
}
