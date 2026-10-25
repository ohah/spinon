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
    track: Track,
    items: &'static [Item; 2],
}

const fn text(id: &'static str) -> Item {
    Item {
        id,
        kind: Kind::Text,
        column: 1,
    }
}

const fn image(id: &'static str) -> Item {
    Item {
        id,
        kind: Kind::Image,
        column: 2,
    }
}

const MIN_FORWARD: [Item; 2] = [
    text("c114-direct-row-min-content-forward-text"),
    image("c114-direct-row-min-content-forward-image"),
];
const MIN_REVERSED: [Item; 2] = [
    image("c114-direct-row-min-content-reversed-image"),
    text("c114-direct-row-min-content-reversed-text"),
];
const MAX_FORWARD: [Item; 2] = [
    text("c114-direct-row-max-content-forward-text"),
    image("c114-direct-row-max-content-forward-image"),
];
const MAX_REVERSED: [Item; 2] = [
    image("c114-direct-row-max-content-reversed-image"),
    text("c114-direct-row-max-content-reversed-text"),
];

const SCENES: [Scene; 4] = [
    Scene {
        id: "direct-row-min-content-forward",
        track: Track::MinContent,
        items: &MIN_FORWARD,
    },
    Scene {
        id: "direct-row-min-content-reversed",
        track: Track::MinContent,
        items: &MIN_REVERSED,
    },
    Scene {
        id: "direct-row-max-content-forward",
        track: Track::MaxContent,
        items: &MAX_FORWARD,
    },
    Scene {
        id: "direct-row-max-content-reversed",
        track: Track::MaxContent,
        items: &MAX_REVERSED,
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
    let row = match scene.track {
        Track::MinContent => "min-content",
        Track::MaxContent => "max-content",
    };
    let mut source = format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:74px;height:auto;margin:0;padding:0;border:0;column-gap:0;row-gap:0;direction:ltr;align-items:start;align-content:start;justify-content:start;grid-template-columns:50px 24px;grid-template-rows:{row};background-color:#101820');
document.appendChild(root);
const nodeIds = [{root_id:?}];
"#
    );

    for (index, item) in scene.items.iter().enumerate() {
        let declaration = match item.kind {
            Kind::Text => format!(
                r#"const item{index} = document.createElement('div');
item{index}.setAttribute('id', {item_id:?});
item{index}.setAttribute('data-node', '');
item{index}.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;grid-column:{column} / span 1;grid-row:1;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d;color:#e5484d');
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
item{index}.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;min-width:auto;min-height:auto;margin:0;padding:0;border:0;grid-column:{column} / span 1;grid-row:1;background-color:#e5484d');
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
        scene.id
    ));
    source
}

#[cfg(test)]
mod tests {
    use super::{Kind, SCENES, Track, runtime_source};

    #[test]
    fn exposes_only_four_direct_mixed_intrinsic_row_scenes() {
        assert_eq!(SCENES.len(), 4);
        for (index, scene) in SCENES.iter().enumerate() {
            assert!(SCENES[..index].iter().all(|prior| prior.id != scene.id));
            let source = runtime_source(scene.id).expect("known direct mixed row scene");
            assert!(source.len() < 4 * 1024);
            assert!(source.contains("width:74px;height:auto"));
            assert!(source.contains("grid-template-columns:50px 24px"));
            assert!(source.contains("row-gap:0;direction:ltr"));
            assert!(!source.contains("writing-mode:"));
            let expected_row = match scene.track {
                Track::MinContent => "grid-template-rows:min-content",
                Track::MaxContent => "grid-template-rows:max-content",
            };
            assert!(source.contains(expected_row));
            assert_eq!(source.matches("grid-row:1").count(), 2);
            assert!(!source.contains("justify-self:"));
            assert!(!source.contains("align-self:"));
            assert!(source.contains("XX XXXXX"));
            assert!(source.contains("https://spinon.invalid/c14-1-png-basic.png"));
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains(&format!("c114-{}-text", scene.id)));
            assert!(source.contains(&format!("c114-{}-image", scene.id)));
            assert_eq!(source.matches("document.createElement('img')").count(), 1);
            assert_eq!(source.matches("document.createElement('div')").count(), 2);
            assert_eq!(source.matches("document.createTextNode(").count(), 1);
            assert_eq!(source.matches("nodeIds.push(").count(), 2);
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
        }
        assert!(runtime_source("not-a-row-scene").is_none());
    }
}
