#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Scene {
    id: &'static str,
    axis: Axis,
    tracks: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Axis {
    Columns,
    Rows,
}

const SCENES: [Scene; 4] = [
    Scene {
        id: "weighted-columns",
        axis: Axis::Columns,
        tracks: "minmax(0px, 1fr) minmax(0px, 2fr)",
    },
    Scene {
        id: "frozen-row",
        axis: Axis::Rows,
        tracks: "minmax(80px, 1fr) minmax(0px, 2fr)",
    },
    Scene {
        id: "subunit-columns",
        axis: Axis::Columns,
        tracks: "minmax(0px, 0.5fr) 20px",
    },
    Scene {
        id: "zero-fr-row",
        axis: Axis::Rows,
        tracks: "minmax(24px, 0fr) minmax(0px, 1fr)",
    },
];

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    let scene = SCENES.iter().find(|scene| scene.id == scene_id)?;
    Some(scene_source(*scene))
}

fn scene_source(scene: Scene) -> String {
    let root_id = format!("c114-{}-grid", scene.id);
    let (width, height, columns, rows) = match scene.axis {
        Axis::Columns => (200, 120, scene.tracks, "120px"),
        Axis::Rows => (120, 200, "120px", scene.tracks),
    };
    let source = format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:{width}px;height:{height}px;margin:0;padding:0;border:0;column-gap:{column_gap}px;row-gap:{row_gap}px;align-content:start;justify-content:start;grid-template-columns:{columns};grid-template-rows:{rows};background-color:#101820');
document.appendChild(root);
const nodeIds = [{root_id:?}];
const itemA = document.createElement('div');
itemA.setAttribute('id', {item_a_id:?});
itemA.setAttribute('data-node', '');
itemA.setAttribute('style', 'box-sizing:border-box;min-width:0;min-height:0;padding:0;border:0;grid-column:{column_a};grid-row:{row_a};background-color:#e5484d');
root.appendChild(itemA);
nodeIds.push({item_a_id:?});
const itemB = document.createElement('div');
itemB.setAttribute('id', {item_b_id:?});
itemB.setAttribute('data-node', '');
itemB.setAttribute('style', 'box-sizing:border-box;min-width:0;min-height:0;padding:0;border:0;grid-column:{column_b};grid-row:{row_b};background-color:#30a46c');
root.appendChild(itemB);
nodeIds.push({item_b_id:?});
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds }};
}})()"#,
        column_gap = if scene.axis == Axis::Columns { 10 } else { 0 },
        row_gap = if scene.axis == Axis::Rows { 10 } else { 0 },
        item_a_id = format!("c114-{}-item-a", scene.id),
        item_b_id = format!("c114-{}-item-b", scene.id),
        scene_id = scene.id,
        column_a = "1 / 2",
        row_a = "1 / 2",
        column_b = if scene.axis == Axis::Columns {
            "2 / 3"
        } else {
            "1 / 2"
        },
        row_b = if scene.axis == Axis::Rows {
            "2 / 3"
        } else {
            "1 / 2"
        },
    );
    source
}

#[cfg(test)]
mod tests {
    use super::{Axis, SCENES, runtime_source};

    #[test]
    fn exposes_only_the_four_fixed_flexible_track_scenes() {
        let expected = [
            (
                "weighted-columns",
                Axis::Columns,
                "minmax(0px, 1fr) minmax(0px, 2fr)",
            ),
            (
                "frozen-row",
                Axis::Rows,
                "minmax(80px, 1fr) minmax(0px, 2fr)",
            ),
            ("subunit-columns", Axis::Columns, "minmax(0px, 0.5fr) 20px"),
            (
                "zero-fr-row",
                Axis::Rows,
                "minmax(24px, 0fr) minmax(0px, 1fr)",
            ),
        ];
        assert_eq!(SCENES.len(), expected.len());

        for (scene, (id, axis, tracks)) in SCENES.iter().zip(expected) {
            assert_eq!(scene.id, id);
            assert_eq!(scene.axis, axis);
            assert_eq!(scene.tracks, tracks);
            let source = runtime_source(id).expect("listed scene has a source");
            let root_id = format!("c114-{id}-grid");
            assert!(source.len() < 2 * 1024);
            assert!(source.contains(&format!("sceneId:{id:?}")));
            assert!(source.contains(&format!("rootId:{root_id:?}")));
            assert!(
                source.contains("width:200px;height:120px")
                    || source.contains("width:120px;height:200px")
            );
            assert!(source.contains(&format!(
                "grid-template-{}:{tracks}",
                match axis {
                    Axis::Columns => "columns",
                    Axis::Rows => "rows",
                }
            )));
            assert!(
                source.contains("column-gap:10px;row-gap:0px")
                    || source.contains("column-gap:0px;row-gap:10px")
            );
            assert_eq!(source.matches("document.createElement('div')").count(), 3);
            assert_eq!(source.matches("nodeIds.push(").count(), 2);
            assert_eq!(source.matches("min-width:0;min-height:0").count(), 2);
            assert!(
                source.contains("grid-column:1 / 2;grid-row:1 / 2")
                    || source.contains("grid-column:2 / 3;grid-row:1 / 2")
            );
            assert!(!source.contains("createTextNode"));
        }

        for unknown in ["weighted-columns-extra", "unknown-flex-scene", ""] {
            assert!(runtime_source(unknown).is_none());
        }
    }
}
