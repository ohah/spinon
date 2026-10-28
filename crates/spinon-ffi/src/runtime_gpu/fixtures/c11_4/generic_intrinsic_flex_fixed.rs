#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Axis {
    Columns,
    Rows,
}

#[path = "generic_intrinsic_flex_fixed/hidden.rs"]
mod hidden;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Contribution {
    Freeze,
    NoFreeze,
    Equality,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Scene {
    id: &'static str,
    axis: Axis,
    contribution: Contribution,
    reverse_source_order: bool,
}

const SCENES: [Scene; 8] = [
    Scene {
        id: "column-freeze",
        axis: Axis::Columns,
        contribution: Contribution::Freeze,
        reverse_source_order: false,
    },
    Scene {
        id: "column-no-freeze",
        axis: Axis::Columns,
        contribution: Contribution::NoFreeze,
        reverse_source_order: false,
    },
    Scene {
        id: "column-equal-boundary",
        axis: Axis::Columns,
        contribution: Contribution::Equality,
        reverse_source_order: false,
    },
    Scene {
        id: "column-freeze-reversed",
        axis: Axis::Columns,
        contribution: Contribution::Freeze,
        reverse_source_order: true,
    },
    Scene {
        id: "row-freeze",
        axis: Axis::Rows,
        contribution: Contribution::Freeze,
        reverse_source_order: false,
    },
    Scene {
        id: "row-no-freeze",
        axis: Axis::Rows,
        contribution: Contribution::NoFreeze,
        reverse_source_order: false,
    },
    Scene {
        id: "row-equal-boundary",
        axis: Axis::Rows,
        contribution: Contribution::Equality,
        reverse_source_order: false,
    },
    Scene {
        id: "row-freeze-reversed",
        axis: Axis::Rows,
        contribution: Contribution::Freeze,
        reverse_source_order: true,
    },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ItemKind {
    A,
    B,
    C,
}

impl ItemKind {
    fn id(self, scene_id: &str) -> String {
        format!(
            "novel-mixed-intrinsic-flex-fixed-{scene_id}-{}",
            self.suffix()
        )
    }

    fn suffix(self) -> &'static str {
        match self {
            Self::A => "a",
            Self::B => "b",
            Self::C => "c",
        }
    }

    fn source_order(scene: Scene) -> [Self; 3] {
        if scene.reverse_source_order {
            [Self::C, Self::B, Self::A]
        } else {
            [Self::A, Self::B, Self::C]
        }
    }
}

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    if let Some(scene) = SCENES.iter().find(|scene| scene.id == scene_id).copied() {
        Some(scene_source(scene))
    } else {
        hidden::runtime_source(scene_id)
    }
}

fn scene_source(scene: Scene) -> String {
    let root_id = format!("novel-mixed-intrinsic-flex-fixed-{}-root", scene.id);
    let (width, height, column_gap, row_gap, columns, rows) = match scene.axis {
        Axis::Columns => (
            300,
            120,
            10,
            0,
            "minmax(min-content,1fr) 40px minmax(0px,2fr)",
            "120px",
        ),
        Axis::Rows => (
            120,
            300,
            0,
            10,
            "120px",
            "minmax(min-content,1fr) 40px minmax(0px,2fr)",
        ),
    };
    let row_text = scene.axis == Axis::Rows;
    let a_text = "X".repeat(scene.contribution.character_count());
    let mut source = format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:{width}px;height:{height}px;margin:0;padding:0;column-gap:{column_gap}px;row-gap:{row_gap}px;align-content:start;justify-content:start;align-items:normal;justify-items:normal;direction:ltr;writing-mode:horizontal-tb;grid-template-columns:{columns};grid-template-rows:{rows};background-color:#e8edf5');
document.appendChild(root);
const nodeIds = [{root_id:?}];
"#
    );

    for item in ItemKind::source_order(scene) {
        let item_id = item.id(scene.id);
        match item {
            ItemKind::A | ItemKind::C => {
                let is_a = item == ItemKind::A;
                let text = if is_a { a_text.as_str() } else { "XXXX" };
                let (column, row) = match (scene.axis, item) {
                    (Axis::Columns, ItemKind::A) => ("1 / 2", "1 / 2"),
                    (Axis::Columns, ItemKind::C) => ("3 / 4", "1 / 2"),
                    (Axis::Rows, ItemKind::A) => ("1 / 2", "1 / 2"),
                    (Axis::Rows, ItemKind::C) => ("1 / 2", "3 / 4"),
                    (_, ItemKind::B) => unreachable!("image has its own source builder"),
                };
                let writing_mode = if row_text {
                    "writing-mode:vertical-rl;"
                } else {
                    ""
                };
                let color = if is_a
                    && matches!(
                        scene.contribution,
                        Contribution::Freeze | Contribution::NoFreeze
                    ) {
                    "#f5c542"
                } else {
                    "#2864dc"
                };
                source.push_str(&format!(
                    r#"const item{suffix} = document.createElement('div');
item{suffix}.setAttribute('id', {item_id:?});
item{suffix}.setAttribute('data-node', '');
item{suffix}.setAttribute('style', 'box-sizing:border-box;display:block;position:static;width:auto;height:auto;min-width:auto;min-height:auto;margin:0;padding:0;grid-column:{column};grid-row:{row};{writing_mode}font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:nowrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:{color};color:{color}');
item{suffix}.appendChild(document.createTextNode({text:?}));
root.appendChild(item{suffix});
nodeIds.push({item_id:?});
"#,
                    suffix = item.suffix(),
                    text = text,
                ));
            }
            ItemKind::B => {
                let (column, row) = match scene.axis {
                    Axis::Columns => ("2 / 3", "1 / 2"),
                    Axis::Rows => ("1 / 2", "2 / 3"),
                };
                source.push_str(&format!(
                    r#"const itemb = document.createElement('img');
itemb.setAttribute('id', {item_id:?});
itemb.setAttribute('data-node', '');
itemb.setAttribute('src', 'https://spinon.invalid/c14-1-png-basic.png');
itemb.setAttribute('alt', '');
itemb.setAttribute('style', 'box-sizing:border-box;display:block;position:static;width:auto;height:auto;min-width:auto;min-height:auto;margin:0;padding:0;grid-column:{column};grid-row:{row};background-color:#d73b51');
root.appendChild(itemb);
nodeIds.push({item_id:?});
"#
                ));
            }
        }
    }

    source.push_str(&format!(
        "return {{ sceneId:{:?}, rootId:{root_id:?}, nodeIds }};\n}})()",
        scene.id,
    ));
    source
}

impl Contribution {
    fn character_count(self) -> usize {
        match self {
            Self::Freeze => 10,
            Self::NoFreeze => 4,
            Self::Equality => 8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Axis, Contribution, ItemKind, SCENES, runtime_source};
    use crate::runtime_gpu::fixtures::c11_4::c11_4_grid_track_sizing_runtime_source;

    #[test]
    fn routes_eight_intrinsic_flex_fixed_scenes_through_generic_layout() {
        assert_eq!(SCENES.len(), 8);
        for (index, scene) in SCENES.iter().enumerate() {
            assert!(
                SCENES[..index]
                    .iter()
                    .all(|previous| previous.id != scene.id)
            );
            let source = runtime_source(scene.id).expect("listed mixed intrinsic scene");
            assert!(source.len() < 8 * 1024);
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains(&format!(
                "novel-mixed-intrinsic-flex-fixed-{}-root",
                scene.id
            )));
            assert!(source.contains("minmax(min-content,1fr) 40px minmax(0px,2fr)"));
            assert_eq!(source.matches("document.createElement('div')").count(), 3);
            assert_eq!(source.matches("document.createElement('img')").count(), 1);
            assert_eq!(source.matches("document.createTextNode(").count(), 2);
            assert!(source.contains("https://spinon.invalid/c14-1-png-basic.png"));
            assert!(source.contains("width:auto;height:auto;min-width:auto;min-height:auto"));
            assert!(!source.contains("justify-self:start;align-self:start"));
            assert!(c11_4_grid_track_sizing_runtime_source(scene.id).is_ok());
        }

        for (scene, axis, characters, order) in [
            ("column-freeze", Axis::Columns, 10, ["a", "b", "c"]),
            ("column-no-freeze", Axis::Columns, 4, ["a", "b", "c"]),
            ("column-equal-boundary", Axis::Columns, 8, ["a", "b", "c"]),
            ("column-freeze-reversed", Axis::Columns, 10, ["c", "b", "a"]),
            ("row-freeze", Axis::Rows, 10, ["a", "b", "c"]),
            ("row-no-freeze", Axis::Rows, 4, ["a", "b", "c"]),
            ("row-equal-boundary", Axis::Rows, 8, ["a", "b", "c"]),
            ("row-freeze-reversed", Axis::Rows, 10, ["c", "b", "a"]),
        ] {
            let source = runtime_source(scene).expect("listed mixed intrinsic scene");
            let ids = order.map(|item| format!("novel-mixed-intrinsic-flex-fixed-{scene}-{item}"));
            let positions =
                ids.map(|id| source.find(&format!("setAttribute('id', {id:?})")).unwrap());
            assert!(positions[0] < positions[1] && positions[1] < positions[2]);
            let text_a = "X".repeat(characters);
            assert!(source.contains(&format!("createTextNode({text_a:?})")));
            assert!(source.contains("createTextNode(\"XXXX\")"));
            match axis {
                Axis::Columns => {
                    assert!(source.contains("width:300px;height:120px"));
                    assert!(source.contains("column-gap:10px;row-gap:0px"));
                    assert_eq!(source.matches("writing-mode:vertical-rl;").count(), 0);
                }
                Axis::Rows => {
                    assert!(source.contains("width:120px;height:300px"));
                    assert!(source.contains("column-gap:0px;row-gap:10px"));
                    assert_eq!(source.matches("writing-mode:vertical-rl;").count(), 2);
                }
            }
        }

        assert_eq!(SCENES[0].contribution, Contribution::Freeze);
        assert_eq!(
            ItemKind::source_order(SCENES[3]),
            [ItemKind::C, ItemKind::B, ItemKind::A]
        );
        for unknown in ["column-freeze-extra", "unknown;throw 1", ""] {
            assert!(runtime_source(unknown).is_none());
            assert!(c11_4_grid_track_sizing_runtime_source(unknown).is_err());
        }
    }
}
