#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Item {
    id: &'static str,
    height: u16,
    row_start: u8,
    row_end: u8,
}

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    items: &'static [Item],
}

const A: Item = Item {
    id: "a",
    height: 80,
    row_start: 1,
    row_end: 3,
};
const B: Item = Item {
    id: "b",
    height: 120,
    row_start: 2,
    row_end: 4,
};
const C: Item = Item {
    id: "c",
    height: 200,
    row_start: 1,
    row_end: 4,
};

const SCENES: [Scene; 4] = [
    Scene {
        id: "row-axis-span2-forward",
        items: &[A, B],
    },
    Scene {
        id: "row-axis-span2-reversed",
        items: &[B, A],
    },
    Scene {
        id: "row-axis-span2-before-span3",
        items: &[A, B, C],
    },
    Scene {
        id: "row-axis-span3-before-span2",
        items: &[C, B, A],
    },
];

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    SCENES
        .iter()
        .find(|scene| scene.id == scene_id)
        .copied()
        .map(scene_source)
}

fn scene_source(scene: Scene) -> String {
    let root_id = format!("c114-{}-grid", scene.id);
    let mut source = format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:220px;height:220px;margin:0;padding:0;border:0;column-gap:0px;row-gap:0px;align-content:start;grid-template-columns:220px;grid-template-rows:repeat(4,minmax(0,max-content));background-color:#101820');
document.appendChild(root);
const nodeIds = [{root_id:?}];
"#
    );

    for (index, item) in scene.items.iter().enumerate() {
        let item_id = format!("c114-{}-item-{}", scene.id, item.id);
        source.push_str(&format!(
            r#"const item{index} = document.createElement('div');
item{index}.setAttribute('id', {item_id:?});
item{index}.setAttribute('data-node', '');
item{index}.setAttribute('style', 'box-sizing:border-box;display:block;width:220px;height:{height}px;margin:0;padding:0;border:0;grid-row:{row_start} / {row_end};grid-column:1 / 2;background-color:#e5484d');
root.appendChild(item{index});
nodeIds.push({item_id:?});
"#,
            item_id = item_id,
            height = item.height,
            row_start = item.row_start,
            row_end = item.row_end,
        ));
    }

    source.push_str(&format!(
        "return {{ sceneId:{:?}, rootId:{root_id:?}, nodeIds }};\n}})()",
        scene.id
    ));
    source
}

#[cfg(test)]
mod tests {
    use super::{A, B, C, SCENES, runtime_source};

    #[test]
    fn exposes_only_fixed_overlapping_row_span_source_order_scenes() {
        let expected = [
            ("row-axis-span2-forward", &[A, B][..]),
            ("row-axis-span2-reversed", &[B, A][..]),
            ("row-axis-span2-before-span3", &[A, B, C][..]),
            ("row-axis-span3-before-span2", &[C, B, A][..]),
        ];
        assert_eq!(SCENES.len(), expected.len());

        for (scene, (expected_id, expected_items)) in SCENES.iter().zip(expected) {
            assert_eq!(scene.id, expected_id);
            assert_eq!(scene.items, expected_items);

            let source = runtime_source(scene.id).expect("known row span scene");
            let root_id = format!("c114-{}-grid", scene.id);
            assert!(source.len() < 4 * 1024);
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains(&format!("rootId:{root_id:?}")));
            assert!(source.contains("width:220px;height:220px"));
            assert!(source.contains("column-gap:0px;row-gap:0px;align-content:start"));
            assert!(source.contains("grid-template-columns:220px"));
            assert!(source.contains("grid-template-rows:repeat(4,minmax(0,max-content))"));
            assert_eq!(
                source.matches("document.createElement('div')").count(),
                scene.items.len() + 1
            );
            assert!(!source.contains("createTextNode"));
            assert_eq!(source.matches("nodeIds.push(").count(), scene.items.len());

            let mut previous_item_start = 0;
            for (index, item) in scene.items.iter().enumerate() {
                let item_id = format!("c114-{}-item-{}", scene.id, item.id);
                let item_declaration =
                    format!("const item{index} = document.createElement('div');");
                let item_start = source
                    .find(&item_declaration)
                    .expect("item is emitted in DOM order");
                assert!(item_start > previous_item_start);
                previous_item_start = item_start;
                assert!(source.contains(&format!("item{index}.setAttribute('id', {item_id:?})")));
                assert!(source.contains(&format!(
                    "width:220px;height:{}px;margin:0;padding:0;border:0;grid-row:{} / {};grid-column:1 / 2",
                    item.height, item.row_start, item.row_end
                )));
                assert!(source.contains(&format!("root.appendChild(item{index})")));
                assert!(source.contains(&format!("nodeIds.push({item_id:?})")));
            }
        }

        assert!(runtime_source("row-axis-span2-forward-unknown").is_none());
        assert!(runtime_source("unknown-row-axis-span-scene").is_none());
    }
}
