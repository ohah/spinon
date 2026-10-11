use super::*;

#[test]
fn runtime_scene_paints_absolute_flex_children_after_in_flow_order_without_using_authored_order() {
    let root_style = "display:flex;position:relative;left:40px;top:20px;box-sizing:border-box;width:120px;height:40px;flex-direction:row;align-items:flex-start;justify-content:flex-start;background-color:#101827";
    let (document, root, children) = fixture_with_root_style(
        root_style,
        &[
            "display:block;box-sizing:border-box;width:40px;height:30px;order:0;margin-left:-40px;background-color:#e5484d",
            "display:block;position:absolute;left:-40px;top:0;width:40px;height:30px;order:50;background-color:#2866f6",
            "display:block;box-sizing:border-box;width:40px;height:30px;order:-1;margin-left:-40px;background-color:#28a745",
            "display:block;box-sizing:border-box;width:40px;height:30px;order:1;margin-left:-40px;background-color:#a855f7",
            "display:block;position:absolute;left:-40px;top:0;width:40px;height:30px;order:-25;background-color:#f59e0b",
        ],
        false,
    );
    let snapshot = document.snapshot();
    let (output, current) = build_fixture(&document, root);
    let render = build_runtime_render_snapshot(&snapshot, root, &output, current).unwrap();

    assert_eq!(
        snapshot.children(root).unwrap().collect::<Vec<_>>(),
        children,
        "paint 순서 계산은 HostDocument source order를 바꾸지 않습니다"
    );
    assert_eq!(
        render
            .boxes()
            .iter()
            .map(|box_| box_.node_id())
            .collect::<Vec<_>>(),
        [
            root.id(),
            children[2].id(),
            children[0].id(),
            children[3].id(),
            children[1].id(),
            children[4].id(),
        ],
        "in-flow item은 order로, absolute item은 source order로 in-flow 뒤에 그립니다"
    );
    assert_eq!(
        render
            .boxes()
            .iter()
            .map(|box_| box_.paint_order())
            .collect::<Vec<_>>(),
        [0, 1, 2, 3, 4, 5]
    );

    let reference: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../tests/fixtures/css/references/c10-3-5-positioned-flex-v1.json"
    ))
    .unwrap();
    let expected_names = reference["observations"][0]["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|fixture_case| fixture_case["id"] == "paint-order")
        .unwrap()["paintOrder"]
        .as_array()
        .unwrap();
    let expected_top_to_bottom = expected_names
        .iter()
        .map(|name| match name.as_str().unwrap() {
            "c1035-paint-abs-later" => children[4].id(),
            "c1035-paint-abs" => children[1].id(),
            "c1035-paint-positive" => children[3].id(),
            "c1035-paint-zero" => children[0].id(),
            "c1035-paint-negative" => children[2].id(),
            name => panic!("Chromium fixture contains unknown paint node: {name}"),
        })
        .collect::<Vec<_>>();
    let point = (5.0, 25.0);
    let actual_top_to_bottom = render
        .boxes()
        .iter()
        .rev()
        .filter(|box_| {
            let frame = box_.frame_css_px();
            point.0 >= frame.x()
                && point.0 < frame.x() + frame.width()
                && point.1 >= frame.y()
                && point.1 < frame.y() + frame.height()
                && box_.paint() != spinon_render::RuntimePaint::None
        })
        .map(|box_| box_.node_id())
        .collect::<Vec<_>>();
    assert_eq!(
        actual_top_to_bottom, expected_top_to_bottom,
        "겹치는 WGPU 장면 순서를 pinned Chrome elementsFromPoint와 대조합니다"
    );
}

#[test]
fn runtime_scene_emits_nested_positioned_nodes_once_in_source_preorder() {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let absolute_parent = document.reserve_node_handle().unwrap();
    let nested_absolute = document.reserve_node_handle().unwrap();
    let nested_flow = document.reserve_node_handle().unwrap();
    let root_flow = document.reserve_node_handle().unwrap();
    let root_absolute = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(412).unwrap(), document.document_revision());
    for (node, parent, style) in [
        (
            root,
            HostParent::Root,
            "display:flex;position:relative;box-sizing:border-box;width:160px;height:80px;flex-direction:row;align-items:flex-start;justify-content:flex-start;background-color:#101827",
        ),
        (
            absolute_parent,
            HostParent::Node(root),
            "display:flex;position:absolute;left:0;top:0;box-sizing:border-box;width:80px;height:60px;flex-direction:row;align-items:flex-start;justify-content:flex-start;order:8;background-color:#243047",
        ),
        (
            nested_absolute,
            HostParent::Node(absolute_parent),
            "display:block;position:absolute;box-sizing:border-box;width:12px;height:12px;order:-99;background-color:#2866f6",
        ),
        (
            nested_flow,
            HostParent::Node(absolute_parent),
            "display:block;box-sizing:border-box;width:20px;height:20px;order:-1;background-color:#28a745",
        ),
        (
            root_flow,
            HostParent::Node(root),
            "display:block;box-sizing:border-box;width:30px;height:30px;order:0;background-color:#e5484d",
        ),
        (
            root_absolute,
            HostParent::Node(root),
            "display:block;position:absolute;left:0;top:0;box-sizing:border-box;width:16px;height:16px;order:-50;background-color:#f59e0b",
        ),
    ] {
        create_element(&mut batch, node, style);
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();

    let snapshot = document.snapshot();
    let (output, current) = build_fixture(&document, root);
    let render = build_runtime_render_snapshot(&snapshot, root, &output, current).unwrap();
    let painted = render
        .boxes()
        .iter()
        .map(|box_| box_.node_id())
        .collect::<Vec<_>>();

    assert_eq!(
        snapshot.children(root).unwrap().collect::<Vec<_>>(),
        [absolute_parent, root_flow, root_absolute],
        "paint 수집은 source child order를 바꾸지 않습니다"
    );
    assert_eq!(
        snapshot
            .children(absolute_parent)
            .unwrap()
            .collect::<Vec<_>>(),
        [nested_absolute, nested_flow],
        "중첩 positioned 자식도 원본 순서로 남깁니다"
    );
    assert_eq!(
        painted,
        [
            root.id(),
            root_flow.id(),
            absolute_parent.id(),
            nested_flow.id(),
            nested_absolute.id(),
            root_absolute.id(),
        ],
        "in-flow phase 다음 nested/outer absolute node를 source preorder로 한 번씩 그립니다"
    );
    assert_eq!(
        painted
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        painted.len(),
        "한 element를 두 번 제출하지 않습니다"
    );
}
