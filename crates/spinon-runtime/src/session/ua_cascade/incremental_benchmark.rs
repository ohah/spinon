use super::*;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};
use std::sync::Arc;
use std::time::Duration;

const HTML: &str = "http://www.w3.org/1999/xhtml";
const SAMPLE_COUNT: usize = 30;

struct Fixture {
    document: HostDocument,
    owner: OwnerId,
    branch: HostNodeHandle,
    size: u32,
    connected_nodes: usize,
}

impl Fixture {
    fn new(connected_nodes: usize) -> Self {
        assert!(connected_nodes >= 3);
        let mut document = HostDocument::new().unwrap();
        let owner = OwnerId::new(754).unwrap();
        let root = document.reserve_node_handle().unwrap();
        let branch = document.reserve_node_handle().unwrap();
        let tile = document.reserve_node_handle().unwrap();
        let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
        create_element(
            &mut batch,
            root,
            "display:flex;width:390px;height:30000px;background-color:#123456",
        );
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        });
        create_element(
            &mut batch,
            branch,
            "display:block;flex-shrink:0;width:140px;height:80px;--tile-size:32px;background-color:#224466",
        );
        create_element(
            &mut batch,
            tile,
            "display:block;flex-shrink:0;width:var(--tile-size);height:24px;background-color:#3366ff",
        );
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(branch),
            node: tile,
            before: None,
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: branch,
            before: None,
        });
        for _ in 3..connected_nodes {
            let node = document.reserve_node_handle().unwrap();
            create_element(
                &mut batch,
                node,
                "display:block;flex-shrink:0;width:12px;height:8px;background-color:#22aa66",
            );
            batch.push(DocumentOperation::InsertBefore {
                parent: HostParent::Node(root),
                node,
                before: None,
            });
        }
        document.commit(batch).unwrap();
        Self {
            document,
            owner,
            branch,
            size: 32,
            connected_nodes,
        }
    }

    fn snapshot(&self) -> Arc<HostDocumentSnapshot> {
        Arc::new(self.document.snapshot())
    }

    fn toggle_branch_style(&mut self) {
        self.size = if self.size == 32 { 46 } else { 32 };
        let mut batch = DocumentChangeBatch::new(self.owner, self.document.document_revision());
        set_style(
            &mut batch,
            self.branch,
            &format!(
                "display:block;flex-shrink:0;width:140px;height:80px;--tile-size:{}px;background-color:#224466",
                self.size
            ),
        );
        self.document.commit(batch).unwrap();
    }
}

struct Observation {
    wall_ns: u128,
    cascade_us: u128,
    layout_scene_us: u128,
    recomputed: u64,
    reused: u64,
    context: u64,
    cascade: Arc<RuntimeUaCascadeCompleted>,
    layout: Arc<RuntimeLayoutCompleted>,
}

fn full_cascade_coordinator() -> RuntimeUaCascadeCoordinator {
    RuntimeUaCascadeCoordinator::new_with_computer(Arc::new(|request| {
        let request = WorkRequest {
            key: request.key,
            snapshot: Arc::clone(&request.snapshot),
            viewport: request.viewport,
            previous_snapshot: None,
            force_full: true,
            invalidation: Some(invalidation::RuntimeStyleInvalidation::Full),
            previous_styles: None,
        };
        compute_request_for_runtime_gpu(&request)
    }))
    .unwrap()
}

fn wait_for_result(
    handle: &RuntimeUaCascadeHandle,
    expected: RuntimeUaCascadeKey,
) -> (Arc<RuntimeUaCascadeCompleted>, Arc<RuntimeLayoutCompleted>) {
    let deadline = Instant::now() + Duration::from_secs(30);
    let mut state = lock(&handle.shared.state);
    loop {
        let terminal = state.requested == Some(expected)
            && state.status != RuntimeUaCascadeState::Pending
            && state.layout_status != RuntimeLayoutState::Pending;
        if terminal {
            assert_eq!(state.status, RuntimeUaCascadeState::Ready);
            assert_eq!(state.layout_status, RuntimeLayoutState::Ready);
            return (
                Arc::clone(state.completed.as_ref().unwrap()),
                Arc::clone(state.layout_completed.as_ref().unwrap()),
            );
        }
        let remaining = deadline.saturating_duration_since(Instant::now());
        assert!(!remaining.is_zero(), "증분 스타일 벤치마크 시간 초과");
        let (next, timeout) = handle
            .shared
            .wake
            .wait_timeout(state, remaining)
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        state = next;
        assert!(!timeout.timed_out(), "증분 스타일 벤치마크 시간 초과");
    }
}

fn initialize(coordinator: &RuntimeUaCascadeCoordinator, fixture: &Fixture) {
    let handle = coordinator.handle();
    let environment = handle
        .set_environment(390.0, 30_000.0, 1.0, CssMediaEnvironment::MOBILE)
        .unwrap();
    let snapshot = fixture.snapshot();
    handle
        .register_document_snapshot(Arc::clone(&snapshot))
        .unwrap();
    let (cascade, layout) = wait_for_result(&handle, key_for(&snapshot, environment));
    assert!(!cascade.cache_hit);
    assert_eq!(
        cascade.cascade_recomputed_style_elements,
        fixture.connected_nodes as u64
    );
    assert!(!layout.cache_hit);
}

fn measure(
    coordinator: &RuntimeUaCascadeCoordinator,
    fixture: &mut Fixture,
    expect_incremental: bool,
) -> Observation {
    fixture.toggle_branch_style();
    let snapshot = fixture.snapshot();
    let handle = coordinator.handle();
    let started = Instant::now();
    handle
        .register_document_snapshot(Arc::clone(&snapshot))
        .unwrap();
    let key = RuntimeUaCascadeKey {
        generation: snapshot.generation().get(),
        document_revision: snapshot.document_revision().get(),
        render_tree_revision: snapshot.render_tree_revision().get(),
        style_revision: StyleRevision::INITIAL.get(),
        environment_revision: EnvironmentRevision::INITIAL.get(),
    };
    let (cascade, layout) = wait_for_result(&handle, key);
    let wall_ns = started.elapsed().as_nanos();
    assert!(!cascade.cache_hit);
    assert!(!layout.cache_hit);
    if expect_incremental {
        assert_eq!(cascade.cascade_recomputed_style_elements, 2);
        assert_eq!(
            cascade.cascade_reused_style_elements,
            fixture.connected_nodes as u64 - 2
        );
        assert_eq!(cascade.cascade_context_style_elements, 1);
    } else {
        assert_eq!(
            cascade.cascade_recomputed_style_elements,
            fixture.connected_nodes as u64
        );
        assert_eq!(cascade.cascade_reused_style_elements, 0);
        assert_eq!(cascade.cascade_context_style_elements, 0);
    }
    Observation {
        wall_ns,
        cascade_us: cascade.computation_duration_us,
        layout_scene_us: layout.projection_duration_us,
        recomputed: cascade.cascade_recomputed_style_elements,
        reused: cascade.cascade_reused_style_elements,
        context: cascade.cascade_context_style_elements,
        cascade,
        layout,
    }
}

fn percentile(samples: &[u128], percentile: f64, scale: f64) -> f64 {
    let mut sorted = samples.to_vec();
    sorted.sort_unstable();
    let index = ((sorted.len() as f64 * percentile).ceil() as usize)
        .saturating_sub(1)
        .min(sorted.len() - 1);
    sorted[index] as f64 / scale
}

fn summarize_nanoseconds(samples: &[u128]) -> (f64, f64) {
    (
        percentile(samples, 0.50, 1_000.0),
        percentile(samples, 0.95, 1_000.0),
    )
}

fn summarize_microseconds(samples: &[u128]) -> (f64, f64) {
    (
        percentile(samples, 0.50, 1.0),
        percentile(samples, 0.95, 1.0),
    )
}

fn create_element(batch: &mut DocumentChangeBatch, node: HostNodeHandle, style: &str) {
    batch.push(DocumentOperation::CreateElement {
        node,
        namespace: HTML.to_owned(),
        local_name: "div".to_owned(),
    });
    set_style(batch, node, style);
}

fn set_style(batch: &mut DocumentChangeBatch, node: HostNodeHandle, style: &str) {
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, "style").unwrap(),
        value: style.into(),
    });
}

#[test]
#[ignore = "release-only C05.4 full-vs-incremental runtime benchmark"]
fn c05_runtime_incremental_restyle_release_benchmark() {
    for connected_nodes in [16, 256, 2048] {
        let mut full_fixture = Fixture::new(connected_nodes);
        let mut incremental_fixture = Fixture::new(connected_nodes);
        let full = full_cascade_coordinator();
        let incremental = RuntimeUaCascadeCoordinator::new_runtime_gpu().unwrap();
        initialize(&full, &full_fixture);
        initialize(&incremental, &incremental_fixture);

        let mut full_wall = Vec::with_capacity(SAMPLE_COUNT);
        let mut incremental_wall = Vec::with_capacity(SAMPLE_COUNT);
        let mut full_cascade = Vec::with_capacity(SAMPLE_COUNT);
        let mut incremental_cascade = Vec::with_capacity(SAMPLE_COUNT);
        let mut full_layout_scene = Vec::with_capacity(SAMPLE_COUNT);
        let mut incremental_layout_scene = Vec::with_capacity(SAMPLE_COUNT);
        let mut last_full = None;
        let mut last_incremental = None;
        for sample in 0..SAMPLE_COUNT {
            let (full_observation, incremental_observation) = if sample % 2 == 0 {
                (
                    measure(&full, &mut full_fixture, false),
                    measure(&incremental, &mut incremental_fixture, true),
                )
            } else {
                let incremental_observation = measure(&incremental, &mut incremental_fixture, true);
                let full_observation = measure(&full, &mut full_fixture, false);
                (full_observation, incremental_observation)
            };
            assert_eq!(
                full_observation.cascade.roots[0].styles.elements,
                incremental_observation.cascade.roots[0].styles.elements,
                "전체·증분 computed style이 {sample}회차에서 다릅니다"
            );
            assert_eq!(
                full_observation.cascade.roots[0].styles.diagnostics,
                incremental_observation.cascade.roots[0].styles.diagnostics,
                "전체·증분 진단이 {sample}회차에서 다릅니다"
            );
            assert_eq!(
                full_observation.layout.frames, incremental_observation.layout.frames,
                "전체·증분 frame이 {sample}회차에서 다릅니다"
            );
            full_wall.push(full_observation.wall_ns);
            incremental_wall.push(incremental_observation.wall_ns);
            full_cascade.push(full_observation.cascade_us);
            incremental_cascade.push(incremental_observation.cascade_us);
            full_layout_scene.push(full_observation.layout_scene_us);
            incremental_layout_scene.push(incremental_observation.layout_scene_us);
            last_full = Some(full_observation);
            last_incremental = Some(incremental_observation);
        }

        let full_wall_stats = summarize_nanoseconds(&full_wall);
        let incremental_wall_stats = summarize_nanoseconds(&incremental_wall);
        let full_cascade_stats = summarize_microseconds(&full_cascade);
        let incremental_cascade_stats = summarize_microseconds(&incremental_cascade);
        let full_layout_scene_stats = summarize_microseconds(&full_layout_scene);
        let incremental_layout_scene_stats = summarize_microseconds(&incremental_layout_scene);
        let full_last = last_full.unwrap();
        let incremental_last = last_incremental.unwrap();
        println!(
            "C05.4 release worker nodes={connected_nodes} samples={SAMPLE_COUNT} full_cascade_p50_p95_us={:.1}/{:.1} incremental_cascade_p50_p95_us={:.1}/{:.1} full_layout_scene_p50_p95_us={:.1}/{:.1} incremental_layout_scene_p50_p95_us={:.1}/{:.1} full_request_p50_p95_us={:.1}/{:.1} incremental_request_p50_p95_us={:.1}/{:.1} full_recomputed={} incremental_recomputed={} incremental_reused={} incremental_context={}",
            full_cascade_stats.0,
            full_cascade_stats.1,
            incremental_cascade_stats.0,
            incremental_cascade_stats.1,
            full_layout_scene_stats.0,
            full_layout_scene_stats.1,
            incremental_layout_scene_stats.0,
            incremental_layout_scene_stats.1,
            full_wall_stats.0,
            full_wall_stats.1,
            incremental_wall_stats.0,
            incremental_wall_stats.1,
            full_last.recomputed,
            incremental_last.recomputed,
            incremental_last.reused,
            incremental_last.context,
        );
        assert_eq!(full_fixture.connected_nodes, connected_nodes);
        assert_eq!(incremental_fixture.connected_nodes, connected_nodes);
        full.shutdown().unwrap();
        incremental.shutdown().unwrap();
    }
}
