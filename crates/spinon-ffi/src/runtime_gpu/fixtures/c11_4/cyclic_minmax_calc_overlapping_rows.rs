#[derive(Clone, Copy)]
struct Item {
    kind: &'static str,
    text: &'static str,
    placement: &'static str,
}

const LEFT: Item = Item {
    kind: "span2-left",
    text: "aaaaaaa",
    placement: "grid-row:1 / 3",
};
const RIGHT: Item = Item {
    kind: "span2-right",
    text: "aaaaaaaa",
    placement: "grid-row:2 / 4",
};
const SPAN3: Item = Item {
    kind: "span3",
    text: "aaaaaaaaaaaaaa",
    placement: "grid-row:1 / 4",
};

struct Scene {
    id: &'static str,
    height: &'static str,
    items: [Item; 3],
}

const SCENES: [Scene; 6] = [
    Scene {
        id: "cyclic-minmax-calc-overlap-rows-forward",
        height: "max-content",
        items: [LEFT, RIGHT, SPAN3],
    },
    Scene {
        id: "cyclic-minmax-calc-overlap-rows-reverse",
        height: "max-content",
        items: [SPAN3, RIGHT, LEFT],
    },
    Scene {
        id: "definite-minmax-calc-overlap-rows-forward",
        height: "140px",
        items: [LEFT, RIGHT, SPAN3],
    },
    Scene {
        id: "definite-minmax-calc-overlap-rows-reverse",
        height: "140px",
        items: [SPAN3, RIGHT, LEFT],
    },
    Scene {
        id: "definite-minmax-calc-overlap-rows-small-forward",
        height: "100px",
        items: [LEFT, RIGHT, SPAN3],
    },
    Scene {
        id: "definite-minmax-calc-overlap-rows-small-reverse",
        height: "100px",
        items: [SPAN3, RIGHT, LEFT],
    },
];

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    SCENES
        .iter()
        .find(|scene| scene.id == scene_id)
        .map(scene_source)
}

fn scene_source(scene: &Scene) -> String {
    let root_id = format!("c114-{}-grid", scene.id);
    let rows = "minmax(calc(10% + 2px),max-content) ".repeat(3);
    let rows = rows.trim_end();
    let mut elements = Vec::with_capacity(scene.items.len());
    let mut node_ids = Vec::with_capacity(scene.items.len() + 1);
    node_ids.push(root_id.clone());
    for item in scene.items {
        let item_id = format!("c114-{}-item-{}", scene.id, item.kind);
        elements.push(format!(
            r#"const item_{kind} = document.createElement('div');
item_{kind}.setAttribute('id', {item_id:?});
item_{kind}.setAttribute('data-node', '');
item_{kind}.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;min-width:0;min-height:0;margin:0;padding:0;border:0;grid-column:1 / 2;{placement};font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:break-all;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#e5484d');
item_{kind}.appendChild(document.createTextNode({text:?}));
root.appendChild(item_{kind});"#,
            kind = item.kind.replace('-', "_"),
            item_id = item_id,
            placement = item.placement,
            text = item.text,
        ));
        node_ids.push(item_id);
    }
    format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:content-box;display:grid;width:10px;height:{height};margin:0;padding:0;border:2px solid #f0c674;column-gap:0;row-gap:2px;align-content:start;justify-content:start;grid-template-columns:10px;grid-template-rows:{rows};background-color:#101820;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;color:#e5484d');
document.appendChild(root);
{items}
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds:{node_ids:?} }};
}})()"#,
        height = scene.height,
        rows = rows,
        items = elements.join("\n"),
        scene_id = scene.id,
        node_ids = node_ids,
    )
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};
    use std::collections::BTreeSet;

    #[test]
    fn exposes_only_fixed_overlapping_cyclic_calc_row_scenes() {
        let unique = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 6);
        assert_eq!(unique.len(), SCENES.len());
        for scene in &SCENES {
            let source = runtime_source(scene.id).expect("allowlisted row-span scene");
            assert!(source.len() < 8 * 1024);
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains(&format!("width:10px;height:{}", scene.height)));
            assert!(source.contains("grid-template-columns:10px"));
            assert_eq!(source.matches("grid-template-rows:").count(), 1);
            assert_eq!(source.matches("document.createElement('div')").count(), 4);
            assert_eq!(source.matches("document.createTextNode(").count(), 3);
            assert_eq!(source.matches("data-node").count(), 4);
            assert!(source.contains("column-gap:0;row-gap:2px"));
            assert!(source.contains("word-break:break-all"));
            for item in scene.items {
                assert!(source.contains(item.placement));
                assert!(source.contains(&format!("createTextNode({:?})", item.text)));
            }
        }
        assert!(runtime_source("cyclic-minmax-calc-overlap-rows-unlisted").is_none());
        assert!(runtime_source("definite-minmax-calc-overlap-rows-arbitrary-span").is_none());
    }
}
