use super::*;

#[derive(Clone, Copy, Debug)]
pub(super) enum Mutation {
    Valid,
    UnlistedScene,
    WrongItemPlacement,
    WrongText,
    WrongGridTracks,
    ExtraInnerItem,
}

pub(super) struct Nodes {
    pub(super) grids: Vec<HostNodeHandle>,
    pub(super) items: Vec<HostNodeHandle>,
}

pub(super) fn fixture(
    scene: &Scene,
    owner: u64,
    mutation: Mutation,
) -> (Arc<HostDocumentSnapshot>, Nodes) {
    const HTML: &str = "http://www.w3.org/1999/xhtml";
    let mut document = HostDocument::new().unwrap();
    let grids = (0..4)
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let items = scene
        .items
        .iter()
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let item_text_nodes = scene
        .items
        .iter()
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let scene_id = if matches!(mutation, Mutation::UnlistedScene) {
        "nested-text-span9-unlisted"
    } else {
        scene.id
    };
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(owner).unwrap(), document.document_revision());
    for (index, grid) in grids.iter().enumerate() {
        batch.push(DocumentOperation::CreateElement {
            node: *grid,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        let inner = index == 3;
        let columns = if inner {
            "repeat(4,min-content)"
        } else {
            "min-content"
        };
        let columns = if inner && matches!(mutation, Mutation::WrongGridTracks) {
            "repeat(3,min-content)"
        } else {
            columns
        };
        let rows = if inner { "repeat(3,20px)" } else { "60px" };
        let style = format!(
            "box-sizing:border-box;display:grid;width:min-content;height:60px;margin:0;padding:0;border:0;column-gap:0;row-gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{columns};grid-template-rows:{rows}"
        );
        for (name, value) in [
            (
                "id",
                format!(
                    "c114-{scene_id}-{}",
                    ["root", "middle", "branch", "inner"][index]
                ),
            ),
            ("data-node", String::new()),
            ("style", style),
        ] {
            batch.push(DocumentOperation::SetAttribute {
                node: *grid,
                name: AttributeName::new(None, name).unwrap(),
                value: value.into(),
            });
        }
        batch.push(DocumentOperation::InsertBefore {
            parent: if index == 0 {
                HostParent::Root
            } else {
                HostParent::Node(grids[index - 1])
            },
            node: *grid,
            before: None,
        });
    }
    for (index, ((item, text_node), expected)) in items
        .iter()
        .zip(&item_text_nodes)
        .zip(scene.items)
        .enumerate()
    {
        batch.push(DocumentOperation::CreateElement {
            node: *item,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        let text = if matches!(mutation, Mutation::WrongText) && index == 0 {
            "wrong"
        } else {
            expected.text
        };
        let placement = if matches!(mutation, Mutation::WrongItemPlacement) && index == 0 {
            "grid-column:1 / span 3;grid-row:1"
        } else {
            // Keep the exact fixed line placement from the independent fixture.
            // Forward scenes use the item's authored placement regardless of DOM order.
            ""
        };
        let expected_placement = format!(
            "grid-column:{} / span {};grid-row:{}",
            expected.column, expected.span, expected.row
        );
        let style = format!(
            "box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;{};background-color:#ffffff",
            if placement.is_empty() {
                &expected_placement
            } else {
                placement
            },
        );
        for (name, value) in [
            ("id", format!("c114-{scene_id}-item-{}", expected.suffix)),
            ("data-node", String::new()),
            ("style", style),
        ] {
            batch.push(DocumentOperation::SetAttribute {
                node: *item,
                name: AttributeName::new(None, name).unwrap(),
                value: value.into(),
            });
        }
        batch.push(DocumentOperation::CreateText {
            node: *text_node,
            data: text.into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(grids[3]),
            node: *item,
            before: None,
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(*item),
            node: *text_node,
            before: None,
        });
    }
    if matches!(mutation, Mutation::ExtraInnerItem) {
        let extra = document.reserve_node_handle().unwrap();
        batch.push(DocumentOperation::CreateElement {
            node: extra,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: extra,
            name: AttributeName::new(None, "id").unwrap(),
            value: format!("c114-{scene_id}-unexpected").into(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: extra,
            name: AttributeName::new(None, "data-node").unwrap(),
            value: String::new().into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(grids[3]),
            node: extra,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    (Arc::new(document.snapshot()), Nodes { grids, items })
}
