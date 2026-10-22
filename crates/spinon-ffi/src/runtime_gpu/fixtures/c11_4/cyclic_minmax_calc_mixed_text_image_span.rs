#[derive(Clone, Copy)]
struct Item {
    key: &'static str,
    tag: &'static str,
    placement: &'static str,
    text: Option<&'static str>,
}

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    width: &'static str,
    order: [Item; 3],
}

const A: Item = Item {
    key: "a",
    tag: "div",
    placement: "grid-column:1 / 3;grid-row:1 / 3",
    text: Some("AA A A"),
};
const B: Item = Item {
    key: "b",
    tag: "img",
    placement: "grid-column:2 / 4;grid-row:1 / 3",
    text: None,
};
const C: Item = Item {
    key: "c",
    tag: "div",
    placement: "grid-column:1 / 4;grid-row:1 / 3",
    text: Some("AA AA AA AA"),
};
const FORWARD: [Item; 3] = [A, B, C];
const REVERSE: [Item; 3] = [C, B, A];
const TRACK: &str = "minmax(calc(10% + 2px),max-content)";
const IMAGE_URL: &str = "https://spinon.invalid/c14-1-png-basic.png";

const SCENES: [Scene; 8] = [
    Scene {
        id: "cyclic-minmax-calc-mixed-text-image-span-intrinsic-forward",
        width: "min-content",
        order: FORWARD,
    },
    Scene {
        id: "cyclic-minmax-calc-mixed-text-image-span-intrinsic-reverse",
        width: "min-content",
        order: REVERSE,
    },
    Scene {
        id: "definite-minmax-calc-mixed-text-image-span-forward",
        width: "40px",
        order: FORWARD,
    },
    Scene {
        id: "definite-minmax-calc-mixed-text-image-span-reverse",
        width: "40px",
        order: REVERSE,
    },
    Scene {
        id: "max-content-forward",
        width: "max-content",
        order: FORWARD,
    },
    Scene {
        id: "max-content-reverse",
        width: "max-content",
        order: REVERSE,
    },
    Scene {
        id: "definite-forward",
        width: "40px",
        order: FORWARD,
    },
    Scene {
        id: "definite-reverse",
        width: "40px",
        order: REVERSE,
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
    let item_sources = scene
        .order
        .iter()
        .map(|item| {
            let item_id = format!("c114-{}-item-{}", scene.id, item.key);
            if item.tag == "img" {
                format!(
                    r#"const item{key} = document.createElement('img');
item{key}.setAttribute('id', {item_id:?});
item{key}.setAttribute('data-node', '');
item{key}.setAttribute('src', {IMAGE_URL:?});
item{key}.setAttribute('style', 'box-sizing:border-box;display:block;width:40px;height:auto;min-width:40px;min-height:auto;align-self:start;margin:0;padding:0;border:0;{placement};background-color:#e5484d');
root.appendChild(item{key});"#,
                    key = item.key,
                    item_id = item_id,
                    placement = item.placement,
                )
            } else {
                format!(
                    r#"const item{key} = document.createElement('div');
item{key}.setAttribute('id', {item_id:?});
item{key}.setAttribute('data-node', '');
item{key}.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;min-width:auto;min-height:auto;align-self:auto;margin:0;padding:0;border:0;{placement};font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#e5484d');
item{key}.appendChild(document.createTextNode({text:?}));
root.appendChild(item{key});"#,
                    key = item.key,
                    item_id = item_id,
                    placement = item.placement,
                    text = item.text.expect("text item has fixed text"),
                )
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let item_ids = ["a", "b", "c"]
        .into_iter()
        .map(|key| format!("'c114-{}-item-{key}'", scene.id))
        .collect::<Vec<_>>()
        .join(", ");
    let node_ids = scene
        .order
        .iter()
        .map(|item| format!("'c114-{}-item-{}'", scene.id, item.key))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:content-box;display:grid;width:{width};height:max-content;margin:0;padding:0;border:2px solid #f0c674;column-gap:2px;row-gap:2px;align-content:start;justify-content:start;grid-template-columns:repeat(3,{track});grid-template-rows:repeat(2,{track});background-color:#101820;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d');
document.appendChild(root);
{item_sources}
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds:[{root_id:?}, {node_ids}], itemIds:[{item_ids}] }};
}})()"#,
        root_id = root_id,
        width = scene.width,
        track = TRACK,
        item_sources = item_sources,
        scene_id = scene.id,
        node_ids = node_ids,
        item_ids = item_ids,
    )
}

#[cfg(test)]
mod tests {
    use super::runtime_source;

    #[test]
    fn exposes_only_eight_fixed_mixed_text_image_scenes() {
        for (scene_id, root_width, reverse) in [
            (
                "cyclic-minmax-calc-mixed-text-image-span-intrinsic-forward",
                "min-content",
                false,
            ),
            (
                "cyclic-minmax-calc-mixed-text-image-span-intrinsic-reverse",
                "min-content",
                true,
            ),
            (
                "definite-minmax-calc-mixed-text-image-span-forward",
                "40px",
                false,
            ),
            (
                "definite-minmax-calc-mixed-text-image-span-reverse",
                "40px",
                true,
            ),
            ("max-content-forward", "max-content", false),
            ("max-content-reverse", "max-content", true),
            ("definite-forward", "40px", false),
            ("definite-reverse", "40px", true),
        ] {
            let source = runtime_source(scene_id).expect("registered scene");
            assert!(source.len() < 7 * 1024);
            assert!(source.contains(&format!("sceneId:{scene_id:?}")));
            assert!(source.contains(&format!("width:{root_width};height:max-content")));
            assert!(source.contains("repeat(3,minmax(calc(10% + 2px),max-content))"));
            assert!(source.contains("repeat(2,minmax(calc(10% + 2px),max-content))"));
            assert!(source.contains("grid-column:1 / 3;grid-row:1 / 3"));
            assert!(source.contains("grid-column:2 / 4;grid-row:1 / 3"));
            assert!(source.contains("grid-column:1 / 4;grid-row:1 / 3"));
            assert!(source.contains("align-self:start"));
            assert!(source.contains("https://spinon.invalid/c14-1-png-basic.png"));
            assert_eq!(source.matches("document.createElement('img')").count(), 1);
            assert_eq!(source.matches("document.createTextNode(").count(), 2);
            assert_eq!(source.matches("document.createElement('div')").count(), 3);
            assert!(source.contains("document.createTextNode(\"AA A A\")"));
            assert!(source.contains("document.createTextNode(\"AA AA AA AA\")"));
            if reverse {
                assert!(source.find("const itemc").unwrap() < source.find("const itemb").unwrap());
                assert!(source.find("const itemb").unwrap() < source.find("const itema").unwrap());
            } else {
                assert!(source.find("const itema").unwrap() < source.find("const itemb").unwrap());
                assert!(source.find("const itemb").unwrap() < source.find("const itemc").unwrap());
            }
        }
        for unlisted in [
            "cyclic-minmax-calc-mixed-text-image-span-unlisted",
            "cyclic-minmax-calc-mixed-text-image-span-intrinsic-four-items",
            "definite-minmax-calc-mixed-text-image-span-item-a-only",
        ] {
            assert!(runtime_source(unlisted).is_none(), "{unlisted}");
        }
    }
}
