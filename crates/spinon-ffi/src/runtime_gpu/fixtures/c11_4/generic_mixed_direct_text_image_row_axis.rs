#[derive(Clone, Copy)]
enum Track {
    MinContent,
    MaxContent,
}

#[derive(Clone, Copy)]
enum Kind {
    Text,
    Image,
}

#[derive(Clone, Copy)]
struct Item {
    id: &'static str,
    kind: Kind,
    column: u8,
}

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    root_id: &'static str,
    track: Track,
    items: &'static [Item; 2],
}

const MIN_FORWARD_ITEMS: [Item; 2] = [
    Item {
        id: "novel-generic-row-min-forward-text",
        kind: Kind::Text,
        column: 1,
    },
    Item {
        id: "novel-generic-row-min-forward-image",
        kind: Kind::Image,
        column: 2,
    },
];
const MIN_REVERSED_ITEMS: [Item; 2] = [
    Item {
        id: "novel-generic-row-min-reversed-image",
        kind: Kind::Image,
        column: 2,
    },
    Item {
        id: "novel-generic-row-min-reversed-text",
        kind: Kind::Text,
        column: 1,
    },
];
const MAX_FORWARD_ITEMS: [Item; 2] = [
    Item {
        id: "novel-generic-row-max-forward-text",
        kind: Kind::Text,
        column: 1,
    },
    Item {
        id: "novel-generic-row-max-forward-image",
        kind: Kind::Image,
        column: 2,
    },
];
const MAX_REVERSED_ITEMS: [Item; 2] = [
    Item {
        id: "novel-generic-row-max-reversed-image",
        kind: Kind::Image,
        column: 2,
    },
    Item {
        id: "novel-generic-row-max-reversed-text",
        kind: Kind::Text,
        column: 1,
    },
];

const SCENES: [Scene; 4] = [
    Scene {
        id: "direct-row-min-content-generic-forward",
        root_id: "novel-generic-row-min-forward-root",
        track: Track::MinContent,
        items: &MIN_FORWARD_ITEMS,
    },
    Scene {
        id: "direct-row-min-content-generic-reversed",
        root_id: "novel-generic-row-min-reversed-root",
        track: Track::MinContent,
        items: &MIN_REVERSED_ITEMS,
    },
    Scene {
        id: "direct-row-max-content-generic-forward",
        root_id: "novel-generic-row-max-forward-root",
        track: Track::MaxContent,
        items: &MAX_FORWARD_ITEMS,
    },
    Scene {
        id: "direct-row-max-content-generic-reversed",
        root_id: "novel-generic-row-max-reversed-root",
        track: Track::MaxContent,
        items: &MAX_REVERSED_ITEMS,
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
    let row = match scene.track {
        Track::MinContent => "min-content",
        Track::MaxContent => "max-content",
    };
    let mut source = format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'display:grid;width:74px;height:auto;margin:0;padding:0;column-gap:0;row-gap:0;writing-mode:horizontal-tb;direction:ltr;align-items:start;grid-template-columns:50px 24px;grid-template-rows:{row};background-color:#101820');
document.appendChild(root);
const nodeIds = [{root_id:?}];
"#,
        root_id = scene.root_id,
    );
    for (index, item) in scene.items.iter().enumerate() {
        let declaration = match item.kind {
            Kind::Text => format!(
                r#"const item{index} = document.createElement('div');
item{index}.setAttribute('id', {item_id:?});
item{index}.setAttribute('data-node', '');
item{index}.setAttribute('style', 'display:block;position:static;width:auto;height:auto;margin:0;padding:0;grid-column:{column} / span 1;grid-row:1;justify-self:auto;align-self:auto;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d;color:#e5484d');
item{index}.appendChild(document.createTextNode('XX XXXXX'));
"#,
                item_id = item.id,
                column = item.column,
            ),
            Kind::Image => format!(
                r#"const item{index} = document.createElement('img');
item{index}.setAttribute('id', {item_id:?});
item{index}.setAttribute('data-node', '');
item{index}.setAttribute('src', 'https://spinon.invalid/c14-1-png-basic.png');
item{index}.setAttribute('alt', '');
item{index}.setAttribute('style', 'display:block;position:static;width:auto;height:auto;min-width:auto;min-height:auto;margin:0;padding:0;grid-column:{column} / span 1;grid-row:1;justify-self:auto;align-self:auto;background-color:#e5484d');
"#,
                item_id = item.id,
                column = item.column,
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
        scene.id,
        root_id = scene.root_id,
    ));
    source
}

#[cfg(test)]
mod tests {
    use super::{Kind, SCENES, Track, runtime_source};

    #[test]
    fn routes_four_novel_id_mixed_text_image_row_scenes() {
        assert_eq!(SCENES.len(), 4);
        for (index, scene) in SCENES.iter().enumerate() {
            assert!(SCENES[..index].iter().all(|prior| prior.id != scene.id));
            assert!(scene.root_id.starts_with("novel-generic-row-"));
            assert_eq!(scene.items.len(), 2);
            assert_eq!(
                scene
                    .items
                    .iter()
                    .filter(|item| matches!(item.kind, Kind::Text))
                    .count(),
                1
            );
            assert_eq!(
                scene
                    .items
                    .iter()
                    .filter(|item| matches!(item.kind, Kind::Image))
                    .count(),
                1
            );
            let source = runtime_source(scene.id).expect("listed generic row scene");
            assert!(
                super::super::c11_4_grid_track_sizing_runtime_source(scene.id).is_ok(),
                "{} reaches the C11.4 fixture router",
                scene.id
            );
            assert!(source.len() < 4 * 1024);
            assert!(source.contains("grid-template-columns:50px 24px"));
            assert!(source.contains("grid-row:1"));
            assert!(source.contains("align-items:start"));
            assert!(!source.contains("align-content:"));
            assert!(!source.contains("justify-items:"));
            assert!(!source.contains("box-sizing:border-box"));
            assert!(source.contains("XX XXXXX"));
            assert!(source.contains("https://spinon.invalid/c14-1-png-basic.png"));
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains(&format!("rootId:{:?}", scene.root_id)));
            assert_eq!(source.matches("nodeIds.push(").count(), 2);
        }
        assert!(matches!(SCENES[0].track, Track::MinContent));
        assert!(matches!(SCENES[1].track, Track::MinContent));
        assert!(matches!(SCENES[2].track, Track::MaxContent));
        assert!(matches!(SCENES[3].track, Track::MaxContent));
        assert!(runtime_source("direct-row-min-content-forward").is_none());
        assert!(runtime_source("unknown-generic-row-scene").is_none());
    }
}
