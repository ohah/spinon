#[derive(Clone, Copy)]
struct Item {
    suffix: &'static str,
    text: &'static str,
    column: u8,
    span: u8,
    row: u8,
}

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    items: &'static [Item],
}

const A: Item = Item {
    suffix: "a",
    text: "XX XXXXX",
    column: 1,
    span: 2,
    row: 1,
};
const B: Item = Item {
    suffix: "b",
    text: "XXXX XXXXXXX",
    column: 2,
    span: 2,
    row: 2,
};
const C: Item = Item {
    suffix: "c",
    text: "XXXXXXXXXXXXXXXXXX",
    column: 1,
    span: 3,
    row: 3,
};

const SCENES: [Scene; 4] = [
    Scene {
        id: "nested-text-span2-forward",
        items: &[A, B],
    },
    Scene {
        id: "nested-text-span2-reversed",
        items: &[B, A],
    },
    Scene {
        id: "nested-text-span2-before-span3",
        items: &[A, B, C],
    },
    Scene {
        id: "nested-text-span3-before-span2",
        items: &[C, B, A],
    },
];

const GRID_ROLES: [&str; 4] = ["root", "middle", "branch", "inner"];

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    SCENES
        .iter()
        .find(|scene| scene.id == scene_id)
        .copied()
        .map(scene_source)
}

fn scene_source(scene: Scene) -> String {
    let grid_ids = GRID_ROLES
        .iter()
        .map(|role| format!("c114-{}-{role}", scene.id))
        .collect::<Vec<_>>();
    let item_ids = scene
        .items
        .iter()
        .map(|item| format!("c114-{}-item-{}", scene.id, item.suffix))
        .collect::<Vec<_>>();
    let node_ids = grid_ids
        .iter()
        .chain(&item_ids)
        .cloned()
        .collect::<Vec<_>>();
    let mut source = format!(
        r#"(() => {{
const gridIds = {grid_ids:?};
const grids = gridIds.map((id, index) => {{
  const grid = document.createElement('div');
  const inner = index === 3;
  const backgrounds = ['#e9f1ff', '#e7f6ed', '#f1e9ff', '#fff0e1'];
  const columns = inner ? 'repeat(4,min-content)' : 'min-content';
  const rows = inner ? 'repeat(3,20px)' : '60px';
  grid.setAttribute('id', id);
  grid.setAttribute('data-node', '');
  grid.setAttribute('style', `box-sizing:border-box;display:grid;width:min-content;height:60px;margin:0;padding:0;border:0;column-gap:0;row-gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:${{columns}};grid-template-rows:${{rows}};background-color:${{backgrounds[index]}}`);
  return grid;
}});
document.appendChild(grids[0]);
for (let index = 1; index < grids.length; index += 1) grids[index - 1].appendChild(grids[index]);
const items = [];
"#
    );
    for (item, item_id) in scene.items.iter().zip(&item_ids) {
        source.push_str(&format!(
            r#"{{
  const item = document.createElement('div');
  item.setAttribute('id', {item_id:?});
  item.setAttribute('data-node', '');
  item.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;grid-column:{column} / span {span};grid-row:{row};background-color:#ffffff');
  item.appendChild(document.createTextNode({text:?}));
  grids[3].appendChild(item);
  items.push(item);
}}
"#,
            item_id = item_id,
            column = item.column,
            span = item.span,
            row = item.row,
            text = item.text,
        ));
    }
    source.push_str(&format!(
        r#"return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds:{node_ids:?}, gridIds }};
}})()"#,
        scene_id = scene.id,
        root_id = grid_ids[0],
        node_ids = node_ids,
    ));
    source
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};

    #[test]
    fn emits_only_four_fixed_nested_text_multispan_scenes() {
        assert_eq!(SCENES.len(), 4);
        for scene in SCENES {
            let source =
                runtime_source(scene.id).expect("allowlisted nested text multi-span scene");
            assert!(source.len() < 10 * 1024);
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains("grid-template-columns:${columns}"));
            assert!(source.contains("grid-template-rows:${rows}"));
            assert!(source.contains("width:min-content;height:60px"));
            assert!(source.contains("font-family:Ahem;font-size:10px"));
            assert!(source.contains("column-gap:0;row-gap:0"));
            assert_eq!(
                source.matches("document.createTextNode(").count(),
                scene.items.len()
            );
            assert!(!source.contains("writing-mode:"));
            let grid_ids = ["root", "middle", "branch", "inner"]
                .map(|role| format!("c114-{}-{role}", scene.id));
            let mut node_ids = grid_ids.to_vec();
            node_ids.extend(
                scene
                    .items
                    .iter()
                    .map(|item| format!("c114-{}-item-{}", scene.id, item.suffix)),
            );
            assert!(source.contains(&format!("const gridIds = {grid_ids:?};")));
            assert!(source.contains(&format!("nodeIds:{node_ids:?}")));
        }
        assert!(runtime_source("nested-text-span2-unlisted").is_none());
        assert!(runtime_source("nested-text-span4-forward").is_none());
    }
}
