#[derive(Clone, Copy)]
struct Item {
    id: &'static str,
    width: u16,
    column_start: u8,
    column_end: u8,
}

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    items: &'static [Item],
}

const A: Item = Item {
    id: "span2-a",
    width: 80,
    column_start: 1,
    column_end: 3,
};
const B: Item = Item {
    id: "span2-b",
    width: 120,
    column_start: 2,
    column_end: 4,
};
const C: Item = Item {
    id: "span3-c",
    width: 200,
    column_start: 1,
    column_end: 4,
};

const SCENES: [Scene; 4] = [
    Scene {
        id: "competing-span2-forward",
        items: &[A, B],
    },
    Scene {
        id: "competing-span2-reversed",
        items: &[B, A],
    },
    Scene {
        id: "span2-before-span3",
        items: &[A, B, C],
    },
    Scene {
        id: "span3-before-span2",
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
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:220px;height:40px;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:repeat(4,minmax(0,max-content));grid-template-rows:10px;grid-auto-rows:10px;background-color:#22c55e');
document.appendChild(root);
const nodeIds = [{root_id:?}];
"#
    );
    for (index, item) in scene.items.iter().enumerate() {
        let item_id = format!("c114-{}-{}", scene.id, item.id);
        source.push_str(&format!(
            r#"const item{index} = document.createElement('div');
item{index}.setAttribute('id', {item_id:?});
item{index}.setAttribute('data-node', '');
item{index}.setAttribute('style', 'box-sizing:border-box;display:block;width:{width}px;height:10px;margin:0;padding:0;border:0;grid-column:{column_start} / {column_end};grid-row:{} / {};background-color:#e5484d');
root.appendChild(item{index});
nodeIds.push({item_id:?});
"#,
            index + 1,
            index + 2,
            item_id = item_id,
            width = item.width,
            column_start = item.column_start,
            column_end = item.column_end,
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
    use super::{SCENES, runtime_source};

    #[test]
    fn exposes_fixed_multispan_source_order_cases() {
        assert_eq!(SCENES.len(), 4);
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("known multi-span scene");
            assert!(source.len() < 4 * 1024);
            assert!(source.contains("grid-template-columns:repeat(4,minmax(0,max-content))"));
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert_eq!(
                source.matches("document.createElement('div')").count(),
                scene.items.len() + 1
            );
            assert_eq!(source.matches("nodeIds.push(").count(), scene.items.len());
        }
        assert!(runtime_source("unknown-multispan-scene").is_none());
    }
}
