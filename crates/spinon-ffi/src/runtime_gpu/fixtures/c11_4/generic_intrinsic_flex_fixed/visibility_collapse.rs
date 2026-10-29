#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Axis {
    Columns,
    Rows,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ItemVisibility {
    Collapse,
    Hidden,
    DisplayNone,
    CollapseVisibleDescendant,
    Visible,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Scene {
    id: &'static str,
    axis: Axis,
    visibility: ItemVisibility,
}

const SCENES: [Scene; 10] = [
    Scene {
        id: "collapse-column",
        axis: Axis::Columns,
        visibility: ItemVisibility::Collapse,
    },
    Scene {
        id: "hidden-column",
        axis: Axis::Columns,
        visibility: ItemVisibility::Hidden,
    },
    Scene {
        id: "display-none-column",
        axis: Axis::Columns,
        visibility: ItemVisibility::DisplayNone,
    },
    Scene {
        id: "collapse-visible-descendant-column",
        axis: Axis::Columns,
        visibility: ItemVisibility::CollapseVisibleDescendant,
    },
    Scene {
        id: "collapse-row",
        axis: Axis::Rows,
        visibility: ItemVisibility::Collapse,
    },
    Scene {
        id: "hidden-row",
        axis: Axis::Rows,
        visibility: ItemVisibility::Hidden,
    },
    Scene {
        id: "display-none-row",
        axis: Axis::Rows,
        visibility: ItemVisibility::DisplayNone,
    },
    Scene {
        id: "collapse-visible-descendant-row",
        axis: Axis::Rows,
        visibility: ItemVisibility::CollapseVisibleDescendant,
    },
    Scene {
        id: "visible-column",
        axis: Axis::Columns,
        visibility: ItemVisibility::Visible,
    },
    Scene {
        id: "visible-row",
        axis: Axis::Rows,
        visibility: ItemVisibility::Visible,
    },
];

pub(super) fn is_scene(scene_id: &str) -> bool {
    SCENES.iter().any(|scene| scene.id == scene_id)
}

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    let scene = SCENES.iter().find(|scene| scene.id == scene_id).copied()?;
    let root_id = format!("c114-grid-visibility-collapse-{}", scene.id);
    let item_id = format!("{root_id}-item");
    let child_id = format!("{root_id}-visible-child");
    let content_id = format!("{root_id}-content");
    let (width, height, columns, rows, grid_column, grid_row, content_width, content_height) =
        match scene.axis {
            Axis::Columns => (
                80,
                20,
                "minmax(auto,max-content) minmax(auto,max-content)",
                "20px",
                "1 / span 2",
                "1 / 2",
                100,
                20,
            ),
            Axis::Rows => (
                20,
                80,
                "20px",
                "minmax(auto,max-content) minmax(auto,max-content)",
                "1 / 2",
                "1 / span 2",
                20,
                if scene.visibility == ItemVisibility::CollapseVisibleDescendant {
                    92
                } else {
                    100
                },
            ),
        };
    let (display, visibility, visible_descendant) = match scene.visibility {
        ItemVisibility::Collapse => ("block", "collapse", false),
        ItemVisibility::Hidden => ("block", "hidden", false),
        ItemVisibility::DisplayNone => ("none", "visible", false),
        ItemVisibility::CollapseVisibleDescendant => ("block", "collapse", true),
        ItemVisibility::Visible => ("block", "visible", false),
    };
    let mut source = format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:{width}px;height:{height}px;margin:0;padding:0;column-gap:0;row-gap:0;align-content:start;justify-content:start;align-items:normal;justify-items:normal;direction:ltr;writing-mode:horizontal-tb;grid-template-columns:{columns};grid-template-rows:{rows};background-color:#f0f0f0');
document.appendChild(root);
const nodeIds = [{root_id:?}];
const item = document.createElement('div');
item.setAttribute('id', {item_id:?});
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;display:{display};position:static;width:auto;height:auto;min-width:auto;min-height:auto;margin:0;padding:0;grid-column:{grid_column};grid-row:{grid_row};visibility:{visibility}');
root.appendChild(item);
nodeIds.push({item_id:?});
"#
    );
    if visible_descendant {
        source.push_str(&format!(
            r#"const visibleChild = document.createElement('div');
visibleChild.setAttribute('id', {child_id:?});
visibleChild.setAttribute('data-node', '');
visibleChild.setAttribute('style', 'box-sizing:border-box;display:block;position:static;width:8px;height:8px;min-width:0;min-height:0;margin:0;padding:0;visibility:visible;background-color:#0000ff');
item.appendChild(visibleChild);
nodeIds.push({child_id:?});
"#
        ));
    }
    source.push_str(&format!(
        r#"const content = document.createElement('div');
content.setAttribute('id', {content_id:?});
content.setAttribute('data-node', '');
content.setAttribute('style', 'box-sizing:border-box;display:block;position:static;width:{content_width}px;height:{content_height}px;min-width:0;min-height:0;margin:0;padding:0;background-color:#ff0000');
item.appendChild(content);
nodeIds.push({content_id:?});
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds }};
}})()"#,
    ));
    Some(source)
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};
    use crate::runtime_gpu::fixtures::c11_4::c11_4_grid_track_sizing_runtime_source;

    #[test]
    fn routes_only_ten_fixed_grid_visibility_collapse_scenes() {
        assert_eq!(SCENES.len(), 10);
        for (index, scene) in SCENES.iter().enumerate() {
            assert!(
                SCENES[..index]
                    .iter()
                    .all(|previous| previous.id != scene.id)
            );
            let source = runtime_source(scene.id).expect("fixed visibility scene");
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains("minmax(auto,max-content) minmax(auto,max-content)"));
            assert!(source.contains("background-color:#f0f0f0"));
            assert_eq!(
                source.matches("document.createElement('div')").count(),
                if scene.id.contains("visible-descendant") {
                    4
                } else {
                    3
                }
            );
            assert!(c11_4_grid_track_sizing_runtime_source(scene.id).is_ok());
        }
        for unknown in ["collapse-column-extra", "row-visibility-collapse", ""] {
            assert!(runtime_source(unknown).is_none());
            assert!(c11_4_grid_track_sizing_runtime_source(unknown).is_err());
        }
    }
}
