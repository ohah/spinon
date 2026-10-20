#[derive(Clone, Copy)]
struct Item {
    id: &'static str,
    height: u8,
    column_start: u8,
    column_span: u8,
    row: u8,
}

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    items: &'static [Item],
}

const A_FORWARD: Item = Item {
    id: "image-a-forward",
    height: 16,
    column_start: 1,
    column_span: 2,
    row: 1,
};
const B_FORWARD: Item = Item {
    id: "image-b-forward",
    height: 32,
    column_start: 2,
    column_span: 2,
    row: 2,
};
const B_REVERSED: Item = Item {
    id: "image-b-reversed",
    ..B_FORWARD
};
const A_REVERSED: Item = Item {
    id: "image-a-reversed",
    ..A_FORWARD
};
const A_THREE: Item = Item {
    id: "image-c-a",
    ..A_FORWARD
};
const B_THREE: Item = Item {
    id: "image-c-b",
    ..B_FORWARD
};
const C_THREE: Item = Item {
    id: "image-c-c",
    height: 48,
    column_start: 1,
    column_span: 3,
    row: 3,
};
const A_THREE_REVERSED: Item = Item {
    id: "image-r-a",
    ..A_FORWARD
};
const B_THREE_REVERSED: Item = Item {
    id: "image-r-b",
    ..B_FORWARD
};
const C_THREE_REVERSED: Item = Item {
    id: "image-r-c",
    ..C_THREE
};

const SCENES: [Scene; 4] = [
    Scene {
        id: "image-span2-forward",
        items: &[A_FORWARD, B_FORWARD],
    },
    Scene {
        id: "image-span2-reversed",
        items: &[B_REVERSED, A_REVERSED],
    },
    Scene {
        id: "image-span2-before-span3",
        items: &[A_THREE, B_THREE, C_THREE],
    },
    Scene {
        id: "image-span3-before-span2",
        items: &[C_THREE_REVERSED, B_THREE_REVERSED, A_THREE_REVERSED],
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
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:40px;height:60px;margin:0;padding:1px;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:repeat(4,min-content);grid-template-rows:repeat(3,20px);background-color:#101820');
document.appendChild(root);
const nodeIds = [{root_id:?}];
"#
    );
    for (index, item) in scene.items.iter().enumerate() {
        source.push_str(&format!(
            r#"const item{index} = document.createElement('img');
item{index}.setAttribute('id', {item_id:?});
item{index}.setAttribute('data-node', '');
item{index}.setAttribute('src', 'https://spinon.invalid/c14-1-png-basic.png');
item{index}.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:{height}px;margin:0;padding:0;border:0;grid-column:{column_start} / span {column_span};grid-row:{row};justify-self:auto;align-self:auto;background-color:#e5484d');
root.appendChild(item{index});
nodeIds.push({item_id:?});
"#,
            index = index + 1,
            item_id = item.id,
            height = item.height,
            column_start = item.column_start,
            column_span = item.column_span,
            row = item.row,
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
    fn exposes_only_the_four_fixed_direct_image_multispan_scenes() {
        assert_eq!(SCENES.len(), 4);
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("known image multi-span scene");
            assert!(source.len() < 4 * 1024);
            assert!(source.contains("grid-template-columns:repeat(4,min-content)"));
            assert!(source.contains("grid-template-rows:repeat(3,20px)"));
            assert!(source.contains("https://spinon.invalid/c14-1-png-basic.png"));
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert_eq!(
                source.matches("document.createElement('img')").count(),
                scene.items.len()
            );
            assert_eq!(source.matches("nodeIds.push(").count(), scene.items.len());
        }
        assert!(runtime_source("unknown-image-multispan").is_none());
    }
}
