use super::*;
#[path = "cache_key_tests.rs"]
mod cache_key_tests;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::thread;

const HTML: &str = "http://www.w3.org/1999/xhtml";

struct Fixture {
    document: HostDocument,
    owner: OwnerId,
    root: HostNodeHandle,
    detached: HostNodeHandle,
}

impl Fixture {
    fn new(root_style: &str) -> Self {
        let mut document = HostDocument::new().unwrap();
        let owner = OwnerId::new(73).unwrap();
        let root = document.reserve_node_handle().unwrap();
        let detached = document.reserve_node_handle().unwrap();
        let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
        batch.push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        set_style(&mut batch, root, root_style);
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        });
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
        }
    }

    fn snapshot(&self) -> Arc<HostDocumentSnapshot> {
        Arc::new(self.document.snapshot())
    }

    fn set_detached_style(&mut self, style: &str) {
        let mut batch = DocumentChangeBatch::new(self.owner, self.document.document_revision());
        set_style(&mut batch, self.detached, style);
        self.document.commit(batch).unwrap();
    }

    fn connect_detached(&mut self) {
        let mut batch = DocumentChangeBatch::new(self.owner, self.document.document_revision());
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(self.root),
            node: self.detached,
            before: None,
        });
        self.document.commit(batch).unwrap();
    }

    fn disconnect_detached(&mut self) {
        let mut batch = DocumentChangeBatch::new(self.owner, self.document.document_revision());
        batch.push(DocumentOperation::RemoveChild {
            parent: HostParent::Node(self.root),
            node: self.detached,
        });
        self.document.commit(batch).unwrap();
    }
}

fn set_style(batch: &mut DocumentChangeBatch, node: HostNodeHandle, style: &str) {
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, "style").unwrap(),
        value: style.into(),
    });
}

fn wait_for_key(
    handle: &RuntimeUaCascadeHandle,
    expected: RuntimeUaCascadeKey,
) -> (RuntimeUaCascadeSnapshot, RuntimeLayoutSnapshot) {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let cascade = handle.snapshot();
        let layout = handle.layout_snapshot();
        if cascade.requested == Some(expected)
            && cascade.state != RuntimeUaCascadeState::Pending
            && layout.requested == Some(expected)
            && layout.state != RuntimeLayoutState::Pending
        {
            return (cascade, layout);
        }
        assert!(
            Instant::now() < deadline,
            "요청 결과 대기 시간 초과: {cascade:?} {layout:?}"
        );
        thread::sleep(Duration::from_millis(1));
    }
}

fn request_key(
    snapshot: &HostDocumentSnapshot,
    environment_revision: EnvironmentRevision,
) -> RuntimeUaCascadeKey {
    key_for(snapshot, environment_revision)
}

fn runtime_gpu_computer(count: Arc<AtomicUsize>) -> Arc<CascadeComputer> {
    Arc::new(move |request| {
        count.fetch_add(1, Ordering::SeqCst);
        compute_request_for_runtime_gpu(request)
    })
}

fn set_mobile_environment(handle: &RuntimeUaCascadeHandle) -> EnvironmentRevision {
    handle
        .set_environment(390.0, 844.0, 3.0, CssMediaEnvironment::MOBILE)
        .unwrap()
}

#[test]
fn detached_only_changes_reuse_and_rekey_the_immutable_runtime_result() {
    let count = Arc::new(AtomicUsize::new(0));
    let coordinator =
        RuntimeUaCascadeCoordinator::new_with_computer(runtime_gpu_computer(Arc::clone(&count)))
            .unwrap();
    let handle = coordinator.handle();
    let mut fixture = Fixture::new("display:flex;width:120px;height:80px;flex-direction:row");
    let first_snapshot = fixture.snapshot();
    handle
        .register_document_snapshot(Arc::clone(&first_snapshot))
        .unwrap();
    let environment = set_mobile_environment(&handle);
    let first_key = request_key(&first_snapshot, environment);
    let (first, first_layout) = wait_for_key(&handle, first_key);
    let first_completed = first.completed.unwrap();
    let first_layout_completed = first_layout.completed.unwrap();
    assert!(!first_completed.cache_hit);
    assert!(!first_layout_completed.cache_hit);
    assert_eq!(count.load(Ordering::SeqCst), 1);

    handle
        .register_document_snapshot(Arc::clone(&first_snapshot))
        .unwrap();
    let repeated = wait_for_key(&handle, first_key);
    assert!(repeated.0.completed.unwrap().cache_hit);
    assert!(repeated.1.completed.unwrap().cache_hit);
    assert_eq!(count.load(Ordering::SeqCst), 1);

    let before_detached_change = fixture.snapshot();
    fixture.set_detached_style("display:block;width:33px;height:17px;background-color:#ff0000");
    let detached_snapshot = fixture.snapshot();
    assert_ne!(
        before_detached_change.document_revision(),
        detached_snapshot.document_revision()
    );
    assert_eq!(
        before_detached_change.render_tree_revision(),
        detached_snapshot.render_tree_revision()
    );
    handle
        .register_document_snapshot(Arc::clone(&detached_snapshot))
        .unwrap();
    let detached_key = request_key(&detached_snapshot, environment);
    let (detached, detached_layout) = wait_for_key(&handle, detached_key);
    let detached_completed = detached.completed.unwrap();
    let detached_layout_completed = detached_layout.completed.unwrap();
    assert!(detached_completed.cache_hit);
    assert!(detached_layout_completed.cache_hit);
    assert_eq!(count.load(Ordering::SeqCst), 1);
    assert_eq!(
        detached_completed.key.document_revision,
        detached_snapshot.document_revision().get()
    );
    assert_eq!(
        detached_completed.roots[0].styles.document_revision,
        detached_snapshot.document_revision()
    );
    assert_eq!(detached_layout_completed.key, detached_key);
    let render_key = detached_layout_completed
        .render_snapshot
        .as_ref()
        .unwrap()
        .key();
    assert_eq!(
        render_key.document_revision(),
        detached_snapshot.document_revision()
    );
    assert!(Arc::ptr_eq(
        &first_completed.roots[0].styles.elements,
        &detached_completed.roots[0].styles.elements
    ));
    assert!(Arc::ptr_eq(
        &first_layout_completed.frames,
        &detached_layout_completed.frames
    ));
    assert_eq!(
        first_layout_completed.frames.as_ref(),
        detached_layout_completed.frames.as_ref()
    );

    fixture.connect_detached();
    let connected_snapshot = fixture.snapshot();
    assert_ne!(
        connected_snapshot.render_tree_revision(),
        detached_snapshot.render_tree_revision()
    );
    handle
        .register_document_snapshot(Arc::clone(&connected_snapshot))
        .unwrap();
    let connected_key = request_key(&connected_snapshot, environment);
    let (connected, connected_layout) = wait_for_key(&handle, connected_key);
    assert!(!connected.completed.unwrap().cache_hit);
    let connected_layout = connected_layout.completed.unwrap();
    assert!(!connected_layout.cache_hit);
    assert!(
        connected_layout
            .frames
            .iter()
            .any(|frame| { frame.node_id == fixture.detached.id().get() })
    );
    assert_eq!(count.load(Ordering::SeqCst), 2);

    fixture.disconnect_detached();
    let disconnected_snapshot = fixture.snapshot();
    handle
        .register_document_snapshot(Arc::clone(&disconnected_snapshot))
        .unwrap();
    let disconnected_key = request_key(&disconnected_snapshot, environment);
    let (disconnected, disconnected_layout) = wait_for_key(&handle, disconnected_key);
    assert!(!disconnected.completed.unwrap().cache_hit);
    let disconnected_layout = disconnected_layout.completed.unwrap();
    assert!(!disconnected_layout.cache_hit);
    assert!(
        disconnected_layout
            .frames
            .iter()
            .all(|frame| frame.node_id != fixture.detached.id().get())
    );
    assert_eq!(count.load(Ordering::SeqCst), 3);

    fixture.set_detached_style("display:block;width:47px;height:19px");
    let changed_while_detached = fixture.snapshot();
    handle
        .register_document_snapshot(Arc::clone(&changed_while_detached))
        .unwrap();
    let detached_again_key = request_key(&changed_while_detached, environment);
    let (detached_again, detached_again_layout) = wait_for_key(&handle, detached_again_key);
    assert!(detached_again.completed.unwrap().cache_hit);
    assert!(detached_again_layout.completed.unwrap().cache_hit);
    assert_eq!(count.load(Ordering::SeqCst), 3);
    coordinator.shutdown().unwrap();
}

#[test]
fn viewport_media_generation_and_float_bits_are_part_of_the_cache_key() {
    let count = Arc::new(AtomicUsize::new(0));
    let coordinator =
        RuntimeUaCascadeCoordinator::new_with_computer(runtime_gpu_computer(Arc::clone(&count)))
            .unwrap();
    let handle = coordinator.handle();
    let fixture = Fixture::new("display:flex;width:120px;height:80px");
    let first_snapshot = fixture.snapshot();
    handle
        .register_document_snapshot(Arc::clone(&first_snapshot))
        .unwrap();
    let initial_environment = set_mobile_environment(&handle);
    wait_for_key(&handle, request_key(&first_snapshot, initial_environment));

    let next_environment = handle
        .set_environment(390.00003, 844.0, 3.0, CssMediaEnvironment::MOBILE)
        .unwrap();
    let after_viewport = wait_for_key(&handle, request_key(&first_snapshot, next_environment));
    assert!(!after_viewport.0.completed.unwrap().cache_hit);
    assert_eq!(count.load(Ordering::SeqCst), 2);

    let media = CssMediaEnvironment {
        color_scheme: spinon_style::CssColorScheme::Dark,
        ..CssMediaEnvironment::MOBILE
    };
    let media_environment = handle
        .set_environment(390.00003, 844.0, 3.0, media)
        .unwrap();
    let after_media = wait_for_key(&handle, request_key(&first_snapshot, media_environment));
    assert!(!after_media.0.completed.unwrap().cache_hit);
    assert_eq!(count.load(Ordering::SeqCst), 3);

    let replacement = Fixture::new("display:flex;width:120px;height:80px");
    let replacement_snapshot = replacement.snapshot();
    handle
        .register_document_snapshot(Arc::clone(&replacement_snapshot))
        .unwrap();
    let replacement_key = request_key(&replacement_snapshot, media_environment);
    let after_generation = wait_for_key(&handle, replacement_key);
    assert!(!after_generation.0.completed.unwrap().cache_hit);
    assert_eq!(count.load(Ordering::SeqCst), 4);
    coordinator.shutdown().unwrap();
}

#[test]
fn failed_layout_is_never_cached() {
    let count = Arc::new(AtomicUsize::new(0));
    let coordinator =
        RuntimeUaCascadeCoordinator::new_with_computer(runtime_gpu_computer(Arc::clone(&count)))
            .unwrap();
    let handle = coordinator.handle();
    let mut fixture = Fixture::new("display:block;width:120px;height:80px;color:red");
    let first_snapshot = fixture.snapshot();
    handle
        .register_document_snapshot(Arc::clone(&first_snapshot))
        .unwrap();
    let environment = set_mobile_environment(&handle);
    let first = wait_for_key(&handle, request_key(&first_snapshot, environment));
    assert_eq!(first.1.state, RuntimeLayoutState::Failed);
    assert_eq!(count.load(Ordering::SeqCst), 1);

    fixture.set_detached_style("display:block;width:31px;height:13px");
    let changed = fixture.snapshot();
    handle
        .register_document_snapshot(Arc::clone(&changed))
        .unwrap();
    let second = wait_for_key(&handle, request_key(&changed, environment));
    assert_eq!(second.1.state, RuntimeLayoutState::Failed);
    assert!(!second.0.completed.unwrap().cache_hit);
    assert_eq!(count.load(Ordering::SeqCst), 2);
    coordinator.shutdown().unwrap();
}

#[test]
fn a_stale_in_flight_result_does_not_seed_the_cache() {
    let count = Arc::new(AtomicUsize::new(0));
    let worker_count = Arc::clone(&count);
    let (started_sender, started_receiver) = mpsc::channel();
    let (release_sender, release_receiver) = mpsc::channel();
    let release_receiver = Arc::new(Mutex::new(release_receiver));
    let coordinator = RuntimeUaCascadeCoordinator::new_with_computer(Arc::new(move |request| {
        let invocation = worker_count.fetch_add(1, Ordering::SeqCst) + 1;
        if invocation == 2 {
            started_sender.send(()).unwrap();
            release_receiver.lock().unwrap().recv().unwrap();
        }
        compute_request_for_runtime_gpu(request)
    }))
    .unwrap();
    let handle = coordinator.handle();
    let mut fixture = Fixture::new("display:flex;width:120px;height:80px");
    let initial = fixture.snapshot();
    handle
        .register_document_snapshot(Arc::clone(&initial))
        .unwrap();
    let initial_environment = set_mobile_environment(&handle);
    wait_for_key(&handle, request_key(&initial, initial_environment));
    assert_eq!(count.load(Ordering::SeqCst), 1);

    let changed_environment = handle
        .set_environment(401.0, 844.0, 3.0, CssMediaEnvironment::MOBILE)
        .unwrap();
    started_receiver
        .recv_timeout(Duration::from_secs(5))
        .unwrap();
    fixture.set_detached_style("display:block;width:35px;height:15px");
    let current = fixture.snapshot();
    handle
        .register_document_snapshot(Arc::clone(&current))
        .unwrap();
    release_sender.send(()).unwrap();

    let current_key = request_key(&current, changed_environment);
    let result = wait_for_key(&handle, current_key);
    assert!(!result.0.completed.unwrap().cache_hit);
    assert_eq!(count.load(Ordering::SeqCst), 3);

    fixture.set_detached_style("display:block;width:36px;height:16px");
    let detached_again = fixture.snapshot();
    handle
        .register_document_snapshot(Arc::clone(&detached_again))
        .unwrap();
    let result = wait_for_key(&handle, request_key(&detached_again, changed_environment));
    assert!(result.0.completed.unwrap().cache_hit);
    assert_eq!(count.load(Ordering::SeqCst), 3);
    coordinator.shutdown().unwrap();
}

#[test]
fn an_error_result_does_not_become_a_reusable_cache_entry() {
    let count = Arc::new(AtomicUsize::new(0));
    let computer = runtime_gpu_computer(Arc::clone(&count));
    let worker_count = Arc::clone(&count);
    let coordinator = RuntimeUaCascadeCoordinator::new_with_computer(Arc::new(move |request| {
        if worker_count.load(Ordering::SeqCst) == 0 {
            worker_count.fetch_add(1, Ordering::SeqCst);
            Err("의도한 cascade 오류".to_owned())
        } else {
            computer(request)
        }
    }))
    .unwrap();
    let handle = coordinator.handle();
    let mut fixture = Fixture::new("display:flex;width:120px;height:80px");
    let initial = fixture.snapshot();
    handle
        .register_document_snapshot(Arc::clone(&initial))
        .unwrap();
    let environment = set_mobile_environment(&handle);
    let failed = wait_for_key(&handle, request_key(&initial, environment));
    assert_eq!(failed.0.state, RuntimeUaCascadeState::Failed);
    assert_eq!(count.load(Ordering::SeqCst), 1);

    fixture.set_detached_style("display:block;width:38px;height:18px");
    let changed = fixture.snapshot();
    handle
        .register_document_snapshot(Arc::clone(&changed))
        .unwrap();
    let recovered = wait_for_key(&handle, request_key(&changed, environment));
    assert!(!recovered.0.completed.unwrap().cache_hit);
    assert_eq!(count.load(Ordering::SeqCst), 2);
    coordinator.shutdown().unwrap();
}

#[test]
fn an_empty_connected_scene_can_reuse_after_detached_document_mutation() {
    let count = Arc::new(AtomicUsize::new(0));
    let coordinator =
        RuntimeUaCascadeCoordinator::new_with_computer(runtime_gpu_computer(Arc::clone(&count)))
            .unwrap();
    let handle = coordinator.handle();
    let owner = OwnerId::new(75).unwrap();
    let mut document = HostDocument::new().unwrap();
    let initial = Arc::new(document.snapshot());
    handle
        .register_document_snapshot(Arc::clone(&initial))
        .unwrap();
    let environment = set_mobile_environment(&handle);
    let first = wait_for_key(&handle, request_key(&initial, environment));
    assert_eq!(first.0.state, RuntimeUaCascadeState::Empty);
    assert_eq!(first.1.state, RuntimeLayoutState::Empty);
    assert!(!first.0.completed.unwrap().cache_hit);

    let detached = document.reserve_node_handle().unwrap();
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    batch.push(DocumentOperation::CreateElement {
        node: detached,
        namespace: HTML.to_owned(),
        local_name: "button".to_owned(),
    });
    set_style(&mut batch, detached, "display:block;width:22px;height:14px");
    document.commit(batch).unwrap();
    let changed = Arc::new(document.snapshot());
    assert_ne!(initial.document_revision(), changed.document_revision());
    assert_eq!(
        initial.render_tree_revision(),
        changed.render_tree_revision()
    );

    handle
        .register_document_snapshot(Arc::clone(&changed))
        .unwrap();
    let reused = wait_for_key(&handle, request_key(&changed, environment));
    assert_eq!(reused.0.state, RuntimeUaCascadeState::Empty);
    assert_eq!(reused.1.state, RuntimeLayoutState::Empty);
    assert!(reused.0.completed.unwrap().cache_hit);
    assert!(reused.1.completed.unwrap().cache_hit);
    assert_eq!(count.load(Ordering::SeqCst), 1);
    coordinator.shutdown().unwrap();
}
