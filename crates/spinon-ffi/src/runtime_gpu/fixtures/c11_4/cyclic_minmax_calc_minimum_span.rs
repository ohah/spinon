#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    width: &'static str,
    height: &'static str,
    columns: &'static str,
    rows: &'static str,
    column_gap: u8,
    row_gap: u8,
    placement: &'static str,
    text: &'static str,
}

const TWO_COLUMN_TRACKS: &str =
    "minmax(calc(50% + 4px),max-content) minmax(calc(50% + 4px),max-content)";
const TWO_ROW_TRACKS: &str =
    "minmax(calc(50% + 2px),max-content) minmax(calc(50% + 2px),max-content)";

const SCENES: [Scene; 4] = [
    Scene {
        id: "cyclic-minmax-calc-min-span-columns",
        width: "max-content",
        height: "10px",
        columns: TWO_COLUMN_TRACKS,
        rows: "10px",
        column_gap: 4,
        row_gap: 0,
        placement: "grid-column:1 / span 2",
        text: "aa aa",
    },
    Scene {
        id: "definite-minmax-calc-min-span-columns",
        width: "50px",
        height: "10px",
        columns: TWO_COLUMN_TRACKS,
        rows: "10px",
        column_gap: 4,
        row_gap: 0,
        placement: "grid-column:1 / span 2",
        text: "aa aa",
    },
    Scene {
        id: "cyclic-minmax-calc-min-span-rows",
        width: "20px",
        height: "max-content",
        columns: "20px",
        rows: TWO_ROW_TRACKS,
        column_gap: 0,
        row_gap: 4,
        placement: "grid-row:1 / span 2",
        text: "a a",
    },
    Scene {
        id: "definite-minmax-calc-min-span-rows",
        width: "20px",
        height: "20px",
        columns: "20px",
        rows: TWO_ROW_TRACKS,
        column_gap: 0,
        row_gap: 4,
        placement: "grid-row:1 / span 2",
        text: "a a",
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
    let item_id = format!("c114-{}-item", scene.id);
    format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:content-box;display:grid;width:{width};height:{height};margin:0;padding:0;border:2px solid #f0c674;column-gap:{column_gap}px;row-gap:{row_gap}px;align-content:start;justify-content:start;grid-template-columns:{columns};grid-template-rows:{rows};background-color:#101820;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', {item_id:?});
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;min-width:0;min-height:0;margin:0;padding:0;border:0;{placement};font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#e5484d');
item.appendChild(document.createTextNode({text:?}));
root.appendChild(item);
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds:[{root_id:?}, {item_id:?}] }};
}})()"#,
        width = scene.width,
        height = scene.height,
        column_gap = scene.column_gap,
        row_gap = scene.row_gap,
        columns = scene.columns,
        rows = scene.rows,
        placement = scene.placement,
        text = scene.text,
        scene_id = scene.id,
    )
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};
    use std::collections::BTreeSet;

    #[test]
    fn exposes_only_four_fixed_two_track_span_scenes() {
        let unique_ids = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 4);
        assert_eq!(unique_ids.len(), SCENES.len());
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("allowlisted span scene");
            assert!(source.len() < 3 * 1024);
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains(&format!("width:{};height:{}", scene.width, scene.height)));
            assert!(source.contains(&format!("grid-template-columns:{}", scene.columns)));
            assert!(source.contains(&format!("grid-template-rows:{}", scene.rows)));
            assert!(source.contains(&format!(
                "column-gap:{}px;row-gap:{}px",
                scene.column_gap, scene.row_gap
            )));
            assert!(source.contains(&format!("{};font-family:Ahem", scene.placement)));
            assert!(source.contains("border:2px solid #f0c674"));
            assert!(source.contains(&format!("document.createTextNode({:?})", scene.text)));
            assert!(source.contains("background-color:#e5484d"));
            assert_eq!(source.matches("document.createElement('div')").count(), 2);
            assert_eq!(source.matches("data-node").count(), 2);
        }
        for unlisted in [
            "cyclic-minmax-calc-min-span-unlisted",
            "cyclic-minmax-calc-min-span-columns-3-tracks",
            "cyclic-minmax-calc-min-span-rows-3-tracks",
        ] {
            assert!(runtime_source(unlisted).is_none(), "{unlisted}");
        }
    }
}
