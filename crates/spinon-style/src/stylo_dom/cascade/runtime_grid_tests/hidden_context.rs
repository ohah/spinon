use super::*;

#[test]
fn incremental_restyle_preserves_hidden_context_for_unsupported_nested_grid() {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let hidden = document.reserve_node_handle().unwrap();
    let grid = document.reserve_node_handle().unwrap();
    let item = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(11_223).unwrap(), document.document_revision());
    for (node, parent, name, node_style) in [
        (
            root,
            HostParent::Root,
            "root",
            "display:block;box-sizing:border-box;width:360px;height:800px;margin:0;padding:0",
        ),
        (
            hidden,
            HostParent::Node(root),
            "hidden",
            "display:none;box-sizing:border-box;width:200px;height:100px;margin:0;padding:0",
        ),
        (
            grid,
            HostParent::Node(hidden),
            "grid",
            "display:grid;box-sizing:border-box;width:200px;height:100px;grid-template-columns:repeat(auto-fit,80px);grid-template-rows:40px;margin:0;padding:0",
        ),
        (
            item,
            HostParent::Node(grid),
            "item",
            "display:block;box-sizing:border-box;width:20px;height:10px;margin:0;padding:0;grid-column:1 / 2;grid-row:1 / 2",
        ),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "id").unwrap(),
            value: name.to_owned().into(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "style").unwrap(),
            value: node_style.to_owned().into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();

    let initial_snapshot = document.snapshot();
    let initial_view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::new(initial_snapshot), root)
            .unwrap();
    let previous = compute_runtime_block_flex_grid_placement_cascade_with_stylesheets(
        &initial_view,
        &[],
        viewport(),
        StyleRevision::INITIAL,
    )
    .unwrap();

    let mut update =
        DocumentChangeBatch::new(OwnerId::new(11_223).unwrap(), document.document_revision());
    update.push(DocumentOperation::SetAttribute {
        node: item,
        name: AttributeName::new(None, "style").unwrap(),
        value: "display:block;box-sizing:border-box;width:20px;height:10px;margin:0;padding:0;grid-column:2 / 3;grid-row:1 / 2"
            .to_owned()
            .into(),
    });
    document.commit(update).unwrap();

    let updated_snapshot = document.snapshot();
    let updated_view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::new(updated_snapshot), root)
            .unwrap();
    let (updated, stats) = compute_runtime_incremental_cascade_with_stylesheets(
        &updated_view,
        &[],
        viewport(),
        StyleRevision::INITIAL,
        ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1,
        &previous,
        &[item.id()],
    )
    .unwrap()
    .expect("숨겨진 조상 아래의 안전한 item 변경은 incremental cascade 대상이어야 합니다");

    assert_eq!(stats.context_style_elements, 3);
    assert!(
        updated
            .elements
            .iter()
            .find(|style| style.node_id == grid.id())
            .unwrap()
            .grid_layout
            .is_none()
    );
    assert!(
        updated
            .elements
            .iter()
            .find(|style| style.node_id == item.id())
            .unwrap()
            .grid_item_placement
            .is_none()
    );
}
