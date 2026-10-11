use std::{sync::Arc, time::Instant};

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_style::{
    CssMediaEnvironment, CssViewport, StyloDocumentView,
    compute_runtime_block_flex_stacking_cascade_with_stylesheets,
};
use spinon_style_to_layout::compute_runtime_style_layout;

use crate::{CurrentLayoutInputs, build_runtime_render_snapshot};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const DEFAULT_DEPTH: usize = 128;

#[test]
fn host_document_to_paint_snapshot_handles_a_deep_stacking_graph() {
    run_deep_graph(DEFAULT_DEPTH);
}

#[test]
#[ignore = "대형 스택 스트레스는 RUST_MIN_STACK=33554432로 별도 실행"]
fn host_document_to_paint_snapshot_handles_a_1024_node_stacking_graph() {
    run_deep_graph(1_024);
}

fn run_deep_graph(depth: usize) {
    let total_started = Instant::now();
    let document_started = Instant::now();
    let (document, handles) = deep_document(depth);
    let document_elapsed = document_started.elapsed();
    eprintln!("C12.4 deep graph phase=host-document nodes={depth} elapsed={document_elapsed:?}");

    let snapshot_started = Instant::now();
    let snapshot = document.snapshot();
    let snapshot_elapsed = snapshot_started.elapsed();
    eprintln!("C12.4 deep graph phase=snapshot elapsed={snapshot_elapsed:?}");
    let root = handles[0];
    let view = StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), root)
        .expect("deep HostDocument forms a Stylo view");
    eprintln!("C12.4 deep graph phase=stylo-view");

    let cascade_started = Instant::now();
    let viewport = CssViewport {
        width_css_px: 120.0,
        height_css_px: 100.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    };
    let styles = compute_runtime_block_flex_stacking_cascade_with_stylesheets(
        &view,
        &[],
        viewport,
        StyleRevision::default(),
    )
    .expect("deep HostDocument cascades");
    let cascade_elapsed = cascade_started.elapsed();
    eprintln!("C12.4 deep graph phase=cascade elapsed={cascade_elapsed:?}");

    let layout_started = Instant::now();
    let output = compute_runtime_style_layout(&snapshot, root, styles, viewport)
        .expect("deep HostDocument lays out");
    let layout_elapsed = layout_started.elapsed();
    eprintln!("C12.4 deep graph phase=taffy-layout elapsed={layout_elapsed:?}");

    let paint_started = Instant::now();
    let current =
        CurrentLayoutInputs::for_host_document(&snapshot, StyleRevision::default(), viewport);
    let render = build_runtime_render_snapshot(&snapshot, root, &output, current)
        .expect("deep HostDocument builds a render snapshot");
    let paint_elapsed = paint_started.elapsed();
    eprintln!("C12.4 deep graph phase=paint-snapshot elapsed={paint_elapsed:?}");

    assert_eq!(snapshot.node_count(), depth);
    assert_eq!(output.computed_styles.elements.len(), depth);
    assert_eq!(output.layout.frames.len(), depth);
    assert_eq!(render.boxes().len(), depth);
    let deepest_frame = output.layout.frames[&handles[depth - 1].id()];
    assert_eq!(deepest_frame.x, 0.0);
    assert_eq!(deepest_frame.y, 0.0);
    assert_eq!(deepest_frame.width, 1.0);
    assert_eq!(deepest_frame.height, 1.0);
    assert!(render.boxes().iter().enumerate().all(|(index, item)| {
        item.node_id() == handles[index].id() && item.paint_order() == index as u32
    }));

    eprintln!(
        "C12.4 deep graph nodes={depth} document={document_elapsed:?} snapshot={snapshot_elapsed:?} cascade={cascade_elapsed:?} layout={layout_elapsed:?} paint={paint_elapsed:?} total={:?}",
        total_started.elapsed()
    );
}

fn deep_document(depth: usize) -> (HostDocument, Vec<HostNodeHandle>) {
    let mut document = HostDocument::new().expect("HostDocument");
    let handles = (0..depth)
        .map(|_| document.reserve_node_handle().expect("node handle"))
        .collect::<Vec<_>>();
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(8125).expect("owner id"),
        document.document_revision(),
    );

    for (index, handle) in handles.iter().copied().enumerate() {
        batch.push(DocumentOperation::CreateElement {
            node: handle,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: handle,
            name: AttributeName::new(None, "style").expect("style attribute"),
            value: "position:relative;z-index:0;width:1px;height:1px;background-color:#336699"
                .to_owned()
                .into(),
        });
        let parent = index
            .checked_sub(1)
            .map_or(HostParent::Root, |parent| HostParent::Node(handles[parent]));
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node: handle,
            before: None,
        });
    }

    document.commit(batch).expect("deep tree commit");
    (document, handles)
}
