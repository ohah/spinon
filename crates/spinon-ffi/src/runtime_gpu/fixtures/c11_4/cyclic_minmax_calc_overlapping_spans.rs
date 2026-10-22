#[derive(Clone, Copy)]
struct Item {
    kind: &'static str,
    text: &'static str,
    placement: &'static str,
}

const LEFT: Item = Item {
    kind: "span2-left",
    text: "aaaaaaa",
    placement: "grid-column:1 / 3",
};
const RIGHT: Item = Item {
    kind: "span2-right",
    text: "aaaaaaaa",
    placement: "grid-column:2 / 4",
};
const SPAN3: Item = Item {
    kind: "span3",
    text: "aaaaaaaaaaaaaa",
    placement: "grid-column:1 / 4",
};

struct Scene {
    id: &'static str,
    width: &'static str,
    items: [Item; 3],
}

const SCENES: [Scene; 6] = [
    Scene {
        id: "cyclic-minmax-calc-overlap-forward",
        width: "max-content",
        items: [LEFT, RIGHT, SPAN3],
    },
    Scene {
        id: "cyclic-minmax-calc-overlap-reverse",
        width: "max-content",
        items: [SPAN3, RIGHT, LEFT],
    },
    Scene {
        id: "definite-minmax-calc-overlap-forward",
        width: "140px",
        items: [LEFT, RIGHT, SPAN3],
    },
    Scene {
        id: "definite-minmax-calc-overlap-reverse",
        width: "140px",
        items: [SPAN3, RIGHT, LEFT],
    },
    Scene {
        id: "definite-minmax-calc-overlap-small-forward",
        width: "100px",
        items: [LEFT, RIGHT, SPAN3],
    },
    Scene {
        id: "definite-minmax-calc-overlap-small-reverse",
        width: "100px",
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
    let columns = "minmax(calc(10% + 2px),max-content) ".repeat(3);
    let columns = columns.trim_end();
    let mut elements = Vec::with_capacity(scene.items.len());
    let mut node_ids = Vec::with_capacity(scene.items.len() + 1);
    node_ids.push(root_id.clone());
    for item in scene.items {
        let item_id = format!("c114-{}-item-{}", scene.id, item.kind);
        elements.push(format!(
            r#"const item_{kind} = document.createElement('div');
item_{kind}.setAttribute('id', {item_id:?});
item_{kind}.setAttribute('data-node', '');
item_{kind}.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;min-width:0;min-height:0;margin:0;padding:0;border:0;{placement};grid-row:1 / 2;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:nowrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#e5484d');
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
root.setAttribute('style', 'box-sizing:content-box;display:grid;width:{width};height:10px;margin:0;padding:0;border:2px solid #f0c674;column-gap:2px;row-gap:0;align-content:start;justify-content:start;grid-template-columns:{columns};grid-template-rows:10px;background-color:#101820;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;color:#e5484d');
document.appendChild(root);
{items}
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds:{node_ids:?} }};
}})()"#,
        width = scene.width,
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
    fn exposes_only_fixed_overlapping_cyclic_calc_scenes() {
        let unique = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 6);
        assert_eq!(unique.len(), SCENES.len());
        for scene in &SCENES {
            let source = runtime_source(scene.id).expect("allowlisted overlapping span scene");
            assert!(source.len() < 8 * 1024);
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains(&format!("width:{};height:10px", scene.width)));
            assert_eq!(source.matches("document.createElement('div')").count(), 4);
            assert_eq!(source.matches("document.createTextNode(").count(), 3);
            assert_eq!(source.matches("data-node").count(), 4);
            assert_eq!(source.matches("grid-template-columns:").count(), 1);
            assert!(source.contains("column-gap:2px;row-gap:0"));
            for item in scene.items {
                assert!(source.contains(item.placement));
                assert!(source.contains(&format!("createTextNode({:?})", item.text)));
            }
        }
        assert!(runtime_source("cyclic-minmax-calc-overlap-unlisted").is_none());
        assert!(runtime_source("definite-minmax-calc-overlap-arbitrary-span").is_none());
    }
}
