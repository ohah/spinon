#[derive(Clone, Copy)]
struct Item {
    id: &'static str,
    text: &'static str,
    placement: &'static str,
    width: &'static str,
    height: &'static str,
}

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    width: &'static str,
    order: [Item; 2],
}

const TRACK: &str = "minmax(calc(10% + 2px),max-content)";
const ITEM_A: Item = Item {
    id: "a",
    text: "AA A A",
    placement: "grid-column:1 / 3;grid-row:1 / 3",
    width: "auto",
    height: "auto",
};
const ITEM_B: Item = Item {
    id: "b",
    text: "XXXX",
    placement: "grid-column:2 / 4;grid-row:1 / 3",
    width: "40px",
    height: "10px",
};

const SCENES: [Scene; 4] = [
    Scene {
        id: "cyclic-minmax-calc-cross-axis-span-groups-intrinsic-forward",
        width: "min-content",
        order: [ITEM_A, ITEM_B],
    },
    Scene {
        id: "cyclic-minmax-calc-cross-axis-span-groups-intrinsic-reverse",
        width: "min-content",
        order: [ITEM_B, ITEM_A],
    },
    Scene {
        id: "definite-minmax-calc-cross-axis-span-groups-forward",
        width: "40px",
        order: [ITEM_A, ITEM_B],
    },
    Scene {
        id: "definite-minmax-calc-cross-axis-span-groups-reverse",
        width: "40px",
        order: [ITEM_B, ITEM_A],
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
    let items = scene
        .order
        .iter()
        .map(|item| {
            let item_id = format!("c114-{}-item-{}", scene.id, item.id);
            format!(
                r#"const item{key} = document.createElement('div');
item{key}.setAttribute('id', {item_id:?});
item{key}.setAttribute('data-node', '');
item{key}.setAttribute('style', 'box-sizing:border-box;display:block;width:{width};height:{height};min-width:auto;min-height:auto;margin:0;padding:0;border:0;{placement};font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#e5484d');
item{key}.appendChild(document.createTextNode({text:?}));
root.appendChild(item{key});"#,
                key = item.id,
                item_id = item_id,
                width = item.width,
                height = item.height,
                placement = item.placement,
                text = item.text,
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let item_a_id = format!("c114-{}-item-a", scene.id);
    let item_b_id = format!("c114-{}-item-b", scene.id);
    let node_ids = scene
        .order
        .iter()
        .map(|item| format!("'c114-{}-item-{}'", scene.id, item.id))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:content-box;display:grid;width:{width};height:max-content;margin:0;padding:0;border:2px solid #f0c674;column-gap:2px;row-gap:2px;align-content:start;justify-content:start;grid-template-columns:{track} {track} {track};grid-template-rows:{track} {track};background-color:#101820;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d');
document.appendChild(root);
{items}
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds:[{root_id:?}, {node_ids}], itemIds:[{item_a_id:?}, {item_b_id:?}] }};
}})()"#,
        root_id = root_id,
        width = scene.width,
        track = TRACK,
        items = items,
        scene_id = scene.id,
        node_ids = node_ids,
        item_a_id = item_a_id,
        item_b_id = item_b_id,
    )
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};
    use std::collections::BTreeSet;

    #[test]
    fn exposes_only_four_fixed_cross_axis_span_group_scenes() {
        let ids = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 4);
        assert_eq!(ids.len(), SCENES.len());
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("allowlisted cross-axis span scene");
            assert!(source.len() < 8 * 1024);
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains(&format!("width:{};height:max-content", scene.width)));
            assert_eq!(
                source
                    .matches("grid-template-columns:minmax(calc(10% + 2px),max-content)")
                    .count(),
                1
            );
            assert_eq!(
                source
                    .matches("grid-template-rows:minmax(calc(10% + 2px),max-content)")
                    .count(),
                1
            );
            assert_eq!(source.matches("document.createElement('div')").count(), 3);
            assert_eq!(source.matches("document.createTextNode(").count(), 2);
            assert!(source.contains("grid-column:1 / 3;grid-row:1 / 3"));
            assert!(source.contains("grid-column:2 / 4;grid-row:1 / 3"));
            assert!(source.contains("document.createTextNode(\"AA A A\")"));
            assert!(source.contains("document.createTextNode(\"XXXX\")"));
            assert!(source.contains("word-break:normal"));
            assert!(source.contains("background-color:#e5484d"));
            assert!(source.contains("width:40px;height:10px"));
            assert!(source.contains("min-width:auto;min-height:auto"));
        }
        for unlisted in [
            "cyclic-minmax-calc-cross-axis-span-groups-unlisted",
            "cyclic-minmax-calc-cross-axis-span-groups-three-items",
            "definite-minmax-calc-cross-axis-span-groups-item-a-only",
        ] {
            assert!(runtime_source(unlisted).is_none(), "{unlisted}");
        }
    }

    #[test]
    fn reverses_only_dom_order_for_reverse_scenes() {
        for (forward, reverse) in [
            (
                "cyclic-minmax-calc-cross-axis-span-groups-intrinsic-forward",
                "cyclic-minmax-calc-cross-axis-span-groups-intrinsic-reverse",
            ),
            (
                "definite-minmax-calc-cross-axis-span-groups-forward",
                "definite-minmax-calc-cross-axis-span-groups-reverse",
            ),
        ] {
            let forward = runtime_source(forward).unwrap();
            let reverse = runtime_source(reverse).unwrap();
            for item in ["itema", "itemb"] {
                let marker = format!("const {item} = document.createElement('div')");
                assert_eq!(forward.matches(&marker).count(), 1);
                assert_eq!(reverse.matches(&marker).count(), 1);
            }
            assert!(forward.find("const itema").unwrap() < forward.find("const itemb").unwrap());
            assert!(reverse.find("const itemb").unwrap() < reverse.find("const itema").unwrap());
            for placement in [
                "grid-column:1 / 3;grid-row:1 / 3",
                "grid-column:2 / 4;grid-row:1 / 3",
            ] {
                assert!(forward.contains(placement));
                assert!(reverse.contains(placement));
            }
        }
    }
}
