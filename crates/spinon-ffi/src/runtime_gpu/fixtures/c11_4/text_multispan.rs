#[derive(Clone, Copy)]
struct Item {
    text: &'static str,
    column_start: u8,
    column_span: u8,
    row: u8,
}

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    items: &'static [Item],
}

const A: Item = Item {
    text: "XX XXXXX",
    column_start: 1,
    column_span: 2,
    row: 1,
};
const B: Item = Item {
    text: "XXXX XXXXXXX",
    column_start: 2,
    column_span: 2,
    row: 2,
};
const C: Item = Item {
    text: "XXXXXXXXXXXXXXXXXX",
    column_start: 1,
    column_span: 3,
    row: 3,
};

const SCENES: [Scene; 4] = [
    Scene {
        id: "text-span2-forward",
        items: &[A, B],
    },
    Scene {
        id: "text-span2-reversed",
        items: &[B, A],
    },
    Scene {
        id: "text-span2-before-span3",
        items: &[A, B, C],
    },
    Scene {
        id: "text-span3-before-span2",
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
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:220px;height:60px;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:repeat(4,min-content);grid-template-rows:repeat(3,20px);background-color:#101820');
document.appendChild(root);
const nodeIds = [{root_id:?}];
"#
    );
    for (index, item) in scene.items.iter().enumerate() {
        let item_id = format!("c114-{}-item-{}", scene.id, index + 1);
        source.push_str(&format!(
        r#"const item{index} = document.createElement('div');
item{index}.setAttribute('id', {item_id:?});
item{index}.setAttribute('data-node', '');
item{index}.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;grid-column:{column_start} / span {column_span};grid-row:{row};font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d');
item{index}.appendChild(document.createTextNode({text:?}));
root.appendChild(item{index});
nodeIds.push({item_id:?});
"#,
            index = index + 1,
            item_id = item_id,
            column_start = item.column_start,
            column_span = item.column_span,
            row = item.row,
            text = item.text,
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
    fn exposes_only_the_four_fixed_direct_leaf_text_multispan_scenes() {
        assert_eq!(SCENES.len(), 4);
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("known text multi-span scene");
            assert!(source.len() < 4 * 1024);
            assert!(source.contains("grid-template-columns:repeat(4,min-content)"));
            assert!(source.contains("grid-template-rows:repeat(3,20px)"));
            assert!(source.contains("document.createTextNode"));
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert_eq!(
                source.matches("document.createElement('div')").count(),
                scene.items.len() + 1
            );
            assert_eq!(
                source.matches("document.createTextNode(").count(),
                scene.items.len()
            );
            assert_eq!(source.matches("nodeIds.push(").count(), scene.items.len());
        }
        assert!(runtime_source("unknown-text-multispan").is_none());
    }
}
