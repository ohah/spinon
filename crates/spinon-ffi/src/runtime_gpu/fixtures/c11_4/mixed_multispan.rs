#[derive(Clone, Copy)]
enum Kind {
    Text(&'static str),
    Image { height: u8 },
}

#[derive(Clone, Copy)]
struct Item {
    id: &'static str,
    kind: Kind,
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
    id: "mixed-text-a-forward",
    kind: Kind::Text("XX XXXXX"),
    column_start: 1,
    column_span: 2,
    row: 1,
};
const B_FORWARD: Item = Item {
    id: "mixed-image-b-forward",
    kind: Kind::Image { height: 48 },
    column_start: 2,
    column_span: 2,
    row: 2,
};
const B_REVERSED: Item = Item {
    id: "mixed-image-b-reversed",
    ..B_FORWARD
};
const A_REVERSED: Item = Item {
    id: "mixed-text-a-reversed",
    ..A_FORWARD
};
const A_THREE: Item = Item {
    id: "mixed-text-c-a",
    ..A_FORWARD
};
const B_THREE: Item = Item {
    id: "mixed-image-c-b",
    ..B_FORWARD
};
const C_THREE: Item = Item {
    id: "mixed-text-c-c",
    kind: Kind::Text("XXXXXXXXXXXXXXXXXX"),
    column_start: 1,
    column_span: 3,
    row: 3,
};
const C_THREE_REVERSED: Item = Item {
    id: "mixed-text-r-c",
    ..C_THREE
};
const B_THREE_REVERSED: Item = Item {
    id: "mixed-image-r-b",
    ..B_THREE
};
const A_THREE_REVERSED: Item = Item {
    id: "mixed-text-r-a",
    ..A_THREE
};

const SPAN2_FORWARD: [Item; 2] = [A_FORWARD, B_FORWARD];
const SPAN2_REVERSED: [Item; 2] = [B_REVERSED, A_REVERSED];
const SPAN2_THEN_SPAN3: [Item; 3] = [A_THREE, B_THREE, C_THREE];
const SPAN3_THEN_SPAN2: [Item; 3] = [C_THREE_REVERSED, B_THREE_REVERSED, A_THREE_REVERSED];

const SCENES: [Scene; 4] = [
    Scene {
        id: "mixed-text-image-span2-forward",
        items: &SPAN2_FORWARD,
    },
    Scene {
        id: "mixed-text-image-span2-reversed",
        items: &SPAN2_REVERSED,
    },
    Scene {
        id: "mixed-text-image-span2-before-span3",
        items: &SPAN2_THEN_SPAN3,
    },
    Scene {
        id: "mixed-text-image-span3-before-span2",
        items: &SPAN3_THEN_SPAN2,
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
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:220px;height:180px;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:repeat(4,min-content);grid-template-rows:repeat(3,60px);background-color:#101820');
document.appendChild(root);
const nodeIds = [{root_id:?}];
"#
    );
    for (index, item) in scene.items.iter().enumerate() {
        let declaration = match item.kind {
            Kind::Text(text) => format!(
                r#"const item{index} = document.createElement('div');
item{index}.setAttribute('id', {item_id:?});
item{index}.setAttribute('data-node', '');
item{index}.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;grid-column:{column_start} / span {column_span};grid-row:{row};font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d');
item{index}.appendChild(document.createTextNode({text:?}));
"#,
                item_id = item.id,
                column_start = item.column_start,
                column_span = item.column_span,
                row = item.row,
            ),
            Kind::Image { height } => format!(
                r#"const item{index} = document.createElement('img');
item{index}.setAttribute('id', {item_id:?});
item{index}.setAttribute('data-node', '');
item{index}.setAttribute('src', 'https://spinon.invalid/c14-1-png-basic.png');
item{index}.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:{height}px;margin:0;padding:0;border:0;grid-column:{column_start} / span {column_span};grid-row:{row};justify-self:auto;align-self:auto;background-color:#e5484d');
"#,
                item_id = item.id,
                column_start = item.column_start,
                column_span = item.column_span,
                row = item.row,
            ),
        };
        source.push_str(&declaration);
        source.push_str(&format!(
            "root.appendChild(item{index});\nnodeIds.push({:?});\n",
            item.id
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
    use super::{Kind, SCENES, runtime_source};

    #[test]
    fn exposes_only_four_mixed_text_image_multispan_scenes() {
        assert_eq!(SCENES.len(), 4);
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("known mixed multi-span scene");
            assert!(source.len() < 6 * 1024);
            assert!(source.contains("width:220px;height:180px"));
            assert!(source.contains("grid-template-columns:repeat(4,min-content)"));
            assert!(source.contains("grid-template-rows:repeat(3,60px)"));
            assert!(source.contains("https://spinon.invalid/c14-1-png-basic.png"));
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert_eq!(source.matches("document.createElement('img')").count(), 1);
            assert_eq!(
                source.matches("document.createElement('div')").count(),
                scene
                    .items
                    .iter()
                    .filter(|item| matches!(item.kind, Kind::Text(_)))
                    .count()
                    + 1
            );
            assert_eq!(
                source.matches("document.createTextNode(").count(),
                scene
                    .items
                    .iter()
                    .filter(|item| matches!(item.kind, Kind::Text(_)))
                    .count()
            );
            assert_eq!(source.matches("nodeIds.push(").count(), scene.items.len());
        }
        assert!(runtime_source("unknown-mixed-multispan").is_none());
    }
}
