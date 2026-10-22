#[derive(Clone, Copy)]
struct Item {
    key: &'static str,
    height: u8,
    column: &'static str,
}

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    root_size: &'static str,
    items: &'static [Item; 3],
}

const A: Item = Item {
    key: "a",
    height: 8,
    column: "1 / 3",
};
const B: Item = Item {
    key: "b",
    height: 12,
    column: "2 / 4",
};
const C: Item = Item {
    key: "c",
    height: 16,
    column: "1 / 4",
};
const FORWARD: [Item; 3] = [A, B, C];
const REVERSE: [Item; 3] = [C, B, A];
const TRACK: &str = "minmax(calc(10% + 2px),max-content)";

const SCENES: [Scene; 4] = [
    Scene {
        id: "cyclic-minmax-replaced-image-span-intrinsic-forward",
        root_size: "min-content",
        items: &FORWARD,
    },
    Scene {
        id: "cyclic-minmax-replaced-image-span-intrinsic-reverse",
        root_size: "min-content",
        items: &REVERSE,
    },
    Scene {
        id: "cyclic-minmax-replaced-image-span-definite-forward",
        root_size: "80px",
        items: &FORWARD,
    },
    Scene {
        id: "cyclic-minmax-replaced-image-span-definite-reverse",
        root_size: "80px",
        items: &REVERSE,
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
root.setAttribute('style', 'box-sizing:content-box;display:grid;width:{size};height:{size};margin:0;padding:0;border:2px solid #f0c674;column-gap:2px;row-gap:2px;align-items:start;align-content:start;justify-content:start;grid-template-columns:repeat(3,{track});grid-template-rows:repeat(2,{track});background-color:#101820');
document.appendChild(root);
const nodeIds = [{root_id:?}];
"#,
        size = scene.root_size,
        track = TRACK,
    );
    for (index, item) in scene.items.iter().enumerate() {
        let item_id = format!("c114-{}-image-{}", scene.id, item.key);
        source.push_str(&format!(
            r#"const item{index} = document.createElement('img');
item{index}.setAttribute('id', {item_id:?});
item{index}.setAttribute('data-node', '');
item{index}.setAttribute('src', 'https://spinon.invalid/c14-1-png-basic.png');
item{index}.setAttribute('style', 'box-sizing:content-box;display:block;width:auto;height:{height}px;margin:0;padding:0;border:0;grid-column:{column};grid-row:1 / 3;justify-self:auto;align-self:auto;background-color:#e5484d');
root.appendChild(item{index});
nodeIds.push({item_id:?});
"#,
            index = index + 1,
            item_id = item_id,
            height = item.height,
            column = item.column,
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
    fn exposes_only_four_fixed_replaced_image_cyclic_minmax_scenes() {
        assert_eq!(SCENES.len(), 4);
        for (index, scene) in SCENES.iter().enumerate() {
            assert!(SCENES[..index].iter().all(|prior| prior.id != scene.id));
            let source = runtime_source(scene.id).expect("fixed replaced image scene");
            assert!(source.len() < 4 * 1024);
            assert!(
                source.contains("width:min-content;height:min-content")
                    || source.contains("width:80px;height:80px")
            );
            assert!(source.contains("repeat(3,minmax(calc(10% + 2px),max-content))"));
            assert!(source.contains("repeat(2,minmax(calc(10% + 2px),max-content))"));
            assert!(source.contains("https://spinon.invalid/c14-1-png-basic.png"));
            assert_eq!(source.matches("document.createElement('img')").count(), 3);
            assert_eq!(source.matches("nodeIds.push(").count(), 3);
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
        }
        assert!(runtime_source("cyclic-minmax-replaced-image-span-unlisted").is_none());
    }
}
