use super::*;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};
use std::sync::Arc;
use std::time::Duration;

const HTML: &str = "http://www.w3.org/1999/xhtml";
const SAMPLE_COUNT: usize = 30;

struct BenchmarkFixture {
    document: HostDocument,
    owner: OwnerId,
    root: HostNodeHandle,
    detached: HostNodeHandle,
    connected_nodes: usize,
}

impl BenchmarkFixture {
    fn new(connected_nodes: usize) -> Self {
        assert!(connected_nodes > 0);
        let mut document = HostDocument::new().unwrap();
        let owner = OwnerId::new(74).unwrap();
        let root = document.reserve_node_handle().unwrap();
        let detached = document.reserve_node_handle().unwrap();
        let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
        batch.push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        set_style(&mut batch, root, &root_style(connected_nodes as u32 * 12));
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        });
        for _ in 1..connected_nodes {
            let child = document.reserve_node_handle().unwrap();
            batch.push(DocumentOperation::CreateElement {
                node: child,
                namespace: HTML.to_owned(),
                local_name: "div".to_owned(),
            });
            set_style(&mut batch, child, "display:block;width:12px;height:8px");
            batch.push(DocumentOperation::InsertBefore {
                parent: HostParent::Node(root),
                node: child,
                before: None,
            });
        }
        batch.push(DocumentOperation::CreateElement {
            node: detached,
            namespace: HTML.to_owned(),
            local_name: "button".to_owned(),
        });
        set_style(&mut batch, detached, "display:block;width:20px;height:10px");
        document.commit(batch).unwrap();
        Self {
            document,
            owner,
            root,
            detached,
            connected_nodes,
        }
    }

    fn snapshot(&self) -> Arc<HostDocumentSnapshot> {
        Arc::new(self.document.snapshot())
    }

    fn change_root_height(&mut self, height: u32) {
        let mut batch = DocumentChangeBatch::new(self.owner, self.document.document_revision());
        set_style(&mut batch, self.root, &root_style(height));
        self.document.commit(batch).unwrap();
    }

    fn change_detached_style(&mut self, width: u32) {
        let mut batch = DocumentChangeBatch::new(self.owner, self.document.document_revision());
        set_style(
            &mut batch,
            self.detached,
            &format!("display:block;width:{width}px;height:10px"),
        );
        self.document.commit(batch).unwrap();
    }
}

fn root_style(height: u32) -> String {
    format!("display:flex;flex-direction:column;width:390px;height:{height}px")
}

fn set_style(batch: &mut DocumentChangeBatch, node: HostNodeHandle, style: &str) {
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, "style").unwrap(),
        value: style.into(),
    });
}

fn wait_for_key(handle: &RuntimeUaCascadeHandle, expected: RuntimeUaCascadeKey) -> (bool, bool) {
    let mut state = lock(&handle.shared.state);
    loop {
        let terminal = state.requested == Some(expected)
            && state.status != RuntimeUaCascadeState::Pending
            && state.layout_status != RuntimeLayoutState::Pending;
        if terminal {
            let cascade_hit = state
                .completed
                .as_ref()
                .expect("성공한 benchmark fixture는 cascade 완료 결과가 있어야 합니다")
                .cache_hit;
            let layout_hit = state
                .layout_completed
                .as_ref()
                .expect("성공한 benchmark fixture는 layout 완료 결과가 있어야 합니다")
                .cache_hit;
            return (cascade_hit, layout_hit);
        }
        let (next, timeout) = handle
            .shared
            .wake
            .wait_timeout(state, Duration::from_secs(30))
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        state = next;
        assert!(
            !timeout.timed_out(),
            "runtime cache benchmark 요청 시간 초과"
        );
    }
}

fn percentile_micros(samples: &[u128], percentile: f64) -> f64 {
    let mut sorted = samples.to_vec();
    sorted.sort_unstable();
    let index = ((sorted.len() as f64 * percentile).ceil() as usize)
        .saturating_sub(1)
        .min(sorted.len() - 1);
    sorted[index] as f64 / 1_000.0
}

#[test]
#[ignore = "release-only C05.3 worker latency benchmark"]
fn c05_runtime_result_cache_release_benchmark() {
    for connected_nodes in [16, 256, 2048] {
        let coordinator = RuntimeUaCascadeCoordinator::new_with_computer(Arc::new(|request| {
            compute_request_for_runtime_gpu(request)
        }))
        .unwrap();
        let handle = coordinator.handle();
        let mut fixture = BenchmarkFixture::new(connected_nodes);
        let initial = fixture.snapshot();
        let environment = handle
            .set_environment(390.0, 30_000.0, 1.0, CssMediaEnvironment::MOBILE)
            .unwrap();
        handle
            .register_document_snapshot(Arc::clone(&initial))
            .unwrap();
        assert_eq!(
            wait_for_key(&handle, key_for(&initial, environment)),
            (false, false)
        );

        let mut miss_samples = Vec::with_capacity(SAMPLE_COUNT);
        let mut hit_samples = Vec::with_capacity(SAMPLE_COUNT);
        for sample in 0..SAMPLE_COUNT {
            fixture.change_root_height(connected_nodes as u32 * 12 + sample as u32 + 1);
            let miss_snapshot = fixture.snapshot();
            let miss_started = Instant::now();
            handle
                .register_document_snapshot(Arc::clone(&miss_snapshot))
                .unwrap();
            assert_eq!(
                wait_for_key(&handle, key_for(&miss_snapshot, environment)),
                (false, false),
                "connected mutation must miss for {connected_nodes} nodes"
            );
            miss_samples.push(miss_started.elapsed().as_nanos());
            let miss_completed = handle.snapshot().completed.unwrap();
            let miss_layout = handle.layout_snapshot().completed.unwrap();

            fixture.change_detached_style(21 + sample as u32);
            let hit_snapshot = fixture.snapshot();
            let hit_started = Instant::now();
            handle
                .register_document_snapshot(Arc::clone(&hit_snapshot))
                .unwrap();
            assert_eq!(
                wait_for_key(&handle, key_for(&hit_snapshot, environment)),
                (true, true),
                "detached mutation must hit for {connected_nodes} nodes"
            );
            hit_samples.push(hit_started.elapsed().as_nanos());
            let hit_completed = handle.snapshot().completed.unwrap();
            let hit_layout = handle.layout_snapshot().completed.unwrap();
            assert!(Arc::ptr_eq(
                &miss_completed.roots[0].styles.elements,
                &hit_completed.roots[0].styles.elements
            ));
            assert!(Arc::ptr_eq(&miss_layout.frames, &hit_layout.frames));
            assert_eq!(fixture.connected_nodes, connected_nodes);
        }

        println!(
            "C05.3 release worker latency: connected_nodes={connected_nodes} samples={SAMPLE_COUNT} miss_p50_us={:.1} miss_p95_us={:.1} hit_p50_us={:.1} hit_p95_us={:.1} style_arc_shared=true frame_arc_shared=true",
            percentile_micros(&miss_samples, 0.50),
            percentile_micros(&miss_samples, 0.95),
            percentile_micros(&hit_samples, 0.50),
            percentile_micros(&hit_samples, 0.95),
        );
        coordinator.shutdown().unwrap();
    }
}
