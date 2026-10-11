use std::collections::BTreeMap;

use spinon_core::{
    DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle, HostParent, OwnerId,
};

use super::{
    LayoutDimension, LayoutEngine, LayoutError, LayoutInput, LayoutStyle, MAX_LAYOUT_TREE_DEPTH,
    TaffyLayoutEngine, Viewport,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";

#[test]
fn direct_layout_rejects_the_first_node_past_the_depth_limit_before_taffy() {
    let (document, handles, styles) = layout_graph(MAX_LAYOUT_TREE_DEPTH + 1, false);
    let snapshot = document.snapshot();
    let input = LayoutInput::from_host_document_with_viewport_containing_block(
        &snapshot,
        handles[0],
        Viewport {
            width: 100.0,
            height: 100.0,
        },
        &styles,
        spinon_core::StyleRevision::INITIAL,
        spinon_core::EnvironmentRevision::INITIAL,
    )
    .unwrap();

    assert_eq!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::TreeDepthExceeded {
            node: handles[MAX_LAYOUT_TREE_DEPTH].id(),
            depth: MAX_LAYOUT_TREE_DEPTH + 1,
            maximum: MAX_LAYOUT_TREE_DEPTH,
        })
    );
}

#[test]
fn depth_limit_does_not_reject_a_wide_tree_with_the_same_node_count() {
    let (document, handles, styles) = layout_graph(MAX_LAYOUT_TREE_DEPTH + 1, true);
    let snapshot = document.snapshot();
    let input = LayoutInput::from_host_document_with_viewport_containing_block(
        &snapshot,
        handles[0],
        Viewport {
            width: 100.0,
            height: 100.0,
        },
        &styles,
        spinon_core::StyleRevision::INITIAL,
        spinon_core::EnvironmentRevision::INITIAL,
    )
    .unwrap();

    let output = TaffyLayoutEngine.compute(&input).unwrap();

    assert_eq!(output.frames.len(), handles.len());
}

fn layout_graph(
    count: usize,
    wide: bool,
) -> (
    HostDocument,
    Vec<HostNodeHandle>,
    BTreeMap<spinon_core::NodeId, LayoutStyle>,
) {
    let mut document = HostDocument::new().unwrap();
    let owner = OwnerId::new(91).unwrap();
    let handles = (0..count)
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    let mut styles = BTreeMap::new();

    for (index, handle) in handles.iter().copied().enumerate() {
        batch.push(DocumentOperation::CreateElement {
            node: handle,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        let parent = if index == 0 {
            HostParent::Root
        } else if wide {
            HostParent::Node(handles[0])
        } else {
            HostParent::Node(handles[index - 1])
        };
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node: handle,
            before: None,
        });
        styles.insert(
            handle.id(),
            LayoutStyle {
                width: LayoutDimension::Fixed(if index == 0 { 100.0 } else { 1.0 }),
                height: LayoutDimension::Fixed(if index == 0 { 100.0 } else { 1.0 }),
                ..LayoutStyle::default()
            },
        );
    }

    document.commit(batch).unwrap();
    (document, handles, styles)
}
