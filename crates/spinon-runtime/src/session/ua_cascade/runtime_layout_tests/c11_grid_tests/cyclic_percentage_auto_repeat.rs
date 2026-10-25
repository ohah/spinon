use super::*;

struct Probe {
    request: WorkRequest,
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    root_id: u64,
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    item_id: u64,
}

fn request_for_grid(width: &str, columns: &str, extra_child: bool) -> Probe {
    const HTML: &str = "http://www.w3.org/1999/xhtml";
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let item = document.reserve_node_handle().unwrap();
    let extra = extra_child.then(|| document.reserve_node_handle().unwrap());
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(11_427).unwrap(), document.document_revision());
    for (node, id, parent, style) in [
        (
            root,
            "c114-cyclic-max-content-auto-fill-grid",
            HostParent::Root,
            format!(
                "box-sizing:border-box;display:grid;width:{width};grid-template-columns:{columns};grid-template-rows:auto;column-gap:0;row-gap:0;margin:0;padding:0;border:0;align-content:start;justify-content:start"
            ),
        ),
        (
            item,
            "c114-cyclic-max-content-auto-fill-item",
            HostParent::Node(root),
            "box-sizing:border-box;width:24px;height:16px;margin:0;padding:0;border:0".to_owned(),
        ),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        for (name, value) in [("id", id.to_owned()), ("style", style)] {
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
        batch.push(DocumentOperation::CreateElement {
            node: extra,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        for (name, value) in [
            ("id", "c114-cyclic-max-content-unexpected-item".to_owned()),
            (
                "style",
                "box-sizing:border-box;width:1px;height:1px;margin:0;padding:0;border:0".to_owned(),
            ),
        ] {
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
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        root_id: root.id().get(),
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        item_id: item.id().get(),
    }
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn final_used_columns(pass: &spinon_layout::LayoutGridTrackSizingPassTrace) -> Vec<f32> {
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CyclicAutoRepeatTraceGateError {
    MissingSemanticPhase { node_id: u64 },
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn require_cyclic_auto_repeat_phase_labels(
    trace: &spinon_layout::LayoutGridTrackSizingTrace,
) -> Result<(), CyclicAutoRepeatTraceGateError> {
    if trace.passes.iter().any(|pass| {
        pass.axis == spinon_layout::LayoutGridTrackSizingAxis::Columns
            && pass.fixture_phase.is_none()
    }) {
        return Err(CyclicAutoRepeatTraceGateError::MissingSemanticPhase {
            node_id: trace.node.get(),
        });
    }
    Ok(())
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn c114_cyclic_percentage_auto_repeat_repeats_after_intrinsic_sizing() {
    let probe = request_for_grid("max-content", "repeat(auto-fill,minmax(0px,50%))", false);
    let calculation = compute_request_for_block_flex_grid_track_sizing(&probe.request).unwrap();
    let completed = calculation.layout.unwrap();
    let diagnostics = completed.grid_track_sizing_diagnostics.unwrap();
    let trace = diagnostics
        .traces
        .iter()
        .find(|trace| trace.node.get() == probe.root_id)
        .unwrap();
    require_cyclic_auto_repeat_phase_labels(trace).unwrap();
    let intrinsic = trace
        .passes
        .iter()
        .find(|pass| {
            pass.fixture_phase
                == Some(spinon_layout::LayoutGridTrackSizingFixturePhase::IntrinsicMaxContent)
        })
        .unwrap();
    let final_definite = trace
        .passes
        .iter()
        .find(|pass| {
            pass.fixture_phase
                == Some(spinon_layout::LayoutGridTrackSizingFixturePhase::FinalDefinite)
        })
        .unwrap();

    assert_eq!(intrinsic.fixture_auto_repeat_count, Some(1));
    assert_eq!(intrinsic.fixture_percentage_basis, None);
    assert_eq!(final_used_columns(intrinsic), vec![24.0]);
    assert_eq!(final_definite.fixture_auto_repeat_count, Some(2));
    assert_eq!(final_definite.fixture_percentage_basis, Some(24.0));
    assert_eq!(final_used_columns(final_definite), vec![12.0, 12.0]);
    for (node_id, width, height) in [(probe.root_id, 24.0, 16.0), (probe.item_id, 24.0, 16.0)] {
        let frame = completed
            .frames
            .iter()
            .find(|frame| frame.node_id == node_id)
            .unwrap();
        assert_eq!(
            (frame.x, frame.y, frame.width, frame.height),
            (0.0, 0.0, width, height)
        );
    }
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn c114_cyclic_percentage_auto_repeat_trace_rejects_ambiguous_phase() {
    let probe = request_for_grid("max-content", "repeat(auto-fill,minmax(0px,50%))", false);
    let calculation = compute_request_for_block_flex_grid_track_sizing(&probe.request).unwrap();
    let completed = calculation.layout.unwrap();
    let diagnostics = completed.grid_track_sizing_diagnostics.unwrap();
    let trace = diagnostics
        .traces
        .iter()
        .find(|trace| trace.node.get() == probe.root_id)
        .unwrap();
    let mut ambiguous = trace.clone();
    for pass in ambiguous
        .passes
        .iter_mut()
        .filter(|pass| pass.axis == spinon_layout::LayoutGridTrackSizingAxis::Columns)
    {
        pass.fixture_phase = None;
    }

    assert_eq!(
        require_cyclic_auto_repeat_phase_labels(&ambiguous),
        Err(CyclicAutoRepeatTraceGateError::MissingSemanticPhase {
            node_id: probe.root_id,
        })
    );
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn c114_cyclic_percentage_auto_repeat_definite_control_uses_two_tracks() {
    let probe = request_for_grid("40px", "repeat(auto-fill,minmax(0px,50%))", false);
    let calculation = compute_request_for_block_flex_grid_track_sizing(&probe.request).unwrap();
    let completed = calculation.layout.unwrap();
    let diagnostics = completed.grid_track_sizing_diagnostics.unwrap();
    let trace = diagnostics
        .traces
        .iter()
        .find(|trace| trace.node.get() == probe.root_id)
        .unwrap();
    require_cyclic_auto_repeat_phase_labels(trace).unwrap();
    let columns = trace
        .passes
        .iter()
        .find(|pass| {
            pass.axis == spinon_layout::LayoutGridTrackSizingAxis::Columns
                && pass.fixture_phase
                    == Some(spinon_layout::LayoutGridTrackSizingFixturePhase::FinalDefinite)
        })
        .unwrap();
    assert_eq!(columns.fixture_auto_repeat_count, Some(2));
    assert_eq!(columns.fixture_percentage_basis, Some(40.0));
    assert_eq!(final_used_columns(columns), vec![20.0, 20.0]);
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[test]
fn c114_cyclic_percentage_auto_repeat_fixture_gate_rejects_neighbor_inputs() {
    for (columns, extra_child) in [
        ("repeat(auto-fill,minmax(0px,40%))", false),
        ("repeat(auto-fill,minmax(10%,50%))", false),
        ("repeat(auto-fit,minmax(0px,50%))", false),
        ("repeat(auto-fill,minmax(0px,50%))", true),
    ] {
        let probe = request_for_grid("max-content", columns, extra_child);
        let calculation = compute_request_for_block_flex_grid_track_sizing(&probe.request).unwrap();
        let error = calculation.layout.unwrap_err();
        assert_eq!(
            error.code, "invalid_grid_track_list",
            "{columns}; extra={extra_child}"
        );
    }
}

#[cfg(not(feature = "c11-4-grid-track-sizing-fixture-test"))]
#[test]
fn c114_cyclic_percentage_auto_repeat_remains_rejected_without_fixture_feature() {
    let probe = request_for_grid("max-content", "repeat(auto-fill,minmax(0px,50%))", false);
    let calculation = compute_request_for_block_flex_grid_track_sizing(&probe.request).unwrap();
    let error = calculation.layout.unwrap_err();
    assert_eq!(error.code, "invalid_grid_track_list");
}
