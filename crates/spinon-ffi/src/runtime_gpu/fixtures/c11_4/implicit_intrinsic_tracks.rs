#[derive(Clone, Copy)]
struct TextItem {
    content: &'static str,
    placement: &'static str,
}

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    width: u32,
    height: u32,
    border: u32,
    border_color: &'static str,
    columns: &'static str,
    rows: &'static str,
    auto_columns: &'static str,
    auto_rows: &'static str,
    auto_flow: &'static str,
    items: &'static [TextItem],
}

const TEXT_STYLE: &str = "box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;color:#e5484d;background-color:#e5484d;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;";

const SCENES: [Scene; 20] = [
    Scene {
        id: "row-min-content",
        width: 40,
        height: 80,
        border: 0,
        border_color: "#101820",
        columns: "40px",
        rows: "12px",
        auto_columns: "auto",
        auto_rows: "min-content",
        auto_flow: "row",
        items: &[TextItem {
            content: "XX XXXXXXX",
            placement: "grid-row:2 / 3;grid-column:1 / 2;",
        }],
    },
    Scene {
        id: "row-max-content",
        width: 40,
        height: 80,
        border: 0,
        border_color: "#101820",
        columns: "40px",
        rows: "12px",
        auto_columns: "auto",
        auto_rows: "max-content",
        auto_flow: "row",
        items: &[TextItem {
            content: "XX XXXXXXX",
            placement: "grid-row:2 / 3;grid-column:1 / 2;",
        }],
    },
    Scene {
        id: "column-min-content",
        width: 160,
        height: 40,
        border: 0,
        border_color: "#101820",
        columns: "24px",
        rows: "40px",
        auto_columns: "min-content",
        auto_rows: "auto",
        auto_flow: "column",
        items: &[TextItem {
            content: "XX XXXXXXX",
            placement: "grid-row:1 / 2;grid-column:2 / 3;",
        }],
    },
    Scene {
        id: "column-max-content",
        width: 160,
        height: 40,
        border: 0,
        border_color: "#101820",
        columns: "24px",
        rows: "40px",
        auto_columns: "max-content",
        auto_rows: "auto",
        auto_flow: "column",
        items: &[TextItem {
            content: "XX XXXXXXX",
            placement: "grid-row:1 / 2;grid-column:2 / 3;",
        }],
    },
    Scene {
        id: "row-auto-placement-min-content",
        width: 40,
        height: 80,
        border: 0,
        border_color: "#101820",
        columns: "40px",
        rows: "12px",
        auto_columns: "auto",
        auto_rows: "min-content",
        auto_flow: "row",
        items: &[
            TextItem {
                content: "XX",
                placement: "grid-row:1 / 2;grid-column:1 / 2;",
            },
            TextItem {
                content: "XX XXXXXXX",
                placement: "",
            },
        ],
    },
    Scene {
        id: "column-auto-placement-max-content",
        width: 160,
        height: 40,
        border: 0,
        border_color: "#101820",
        columns: "24px",
        rows: "40px",
        auto_columns: "max-content",
        auto_rows: "auto",
        auto_flow: "column",
        items: &[
            TextItem {
                content: "XX",
                placement: "grid-row:1 / 2;grid-column:1 / 2;",
            },
            TextItem {
                content: "XX XXXXXXX",
                placement: "",
            },
        ],
    },
    Scene {
        id: "row-auto-placement-auto",
        width: 40,
        height: 80,
        border: 0,
        border_color: "#101820",
        columns: "40px",
        rows: "12px",
        auto_columns: "auto",
        auto_rows: "auto",
        auto_flow: "row",
        items: &[
            TextItem {
                content: "XX",
                placement: "grid-row:1 / 2;grid-column:1 / 2;",
            },
            TextItem {
                content: "XX XXXXXXX",
                placement: "",
            },
        ],
    },
    Scene {
        id: "column-auto-placement-auto",
        width: 160,
        height: 40,
        border: 0,
        border_color: "#101820",
        columns: "24px",
        rows: "40px",
        auto_columns: "auto",
        auto_rows: "auto",
        auto_flow: "column",
        items: &[
            TextItem {
                content: "XX",
                placement: "grid-row:1 / 2;grid-column:1 / 2;",
            },
            TextItem {
                content: "XX XXXXXXX",
                placement: "",
            },
        ],
    },
    Scene {
        id: "two-implicit-columns-min-content",
        width: 240,
        height: 40,
        border: 0,
        border_color: "#101820",
        columns: "24px",
        rows: "40px",
        auto_columns: "min-content",
        auto_rows: "auto",
        auto_flow: "column",
        items: &[
            TextItem {
                content: "XX",
                placement: "grid-row:1 / 2;grid-column:2 / 3;",
            },
            TextItem {
                content: "XXXX",
                placement: "grid-row:1 / 2;grid-column:3 / 4;",
            },
        ],
    },
    Scene {
        id: "cycled-intrinsic-columns",
        width: 240,
        height: 40,
        border: 0,
        border_color: "#101820",
        columns: "24px",
        rows: "40px",
        auto_columns: "min-content max-content",
        auto_rows: "auto",
        auto_flow: "column",
        items: &[
            TextItem {
                content: "XX",
                placement: "",
            },
            TextItem {
                content: "XX",
                placement: "",
            },
            TextItem {
                content: "XX XXXXXXX",
                placement: "",
            },
            TextItem {
                content: "XXXX",
                placement: "",
            },
        ],
    },
    Scene {
        id: "cycled-fixed-max-rows",
        width: 40,
        height: 100,
        border: 0,
        border_color: "#101820",
        columns: "40px",
        rows: "12px",
        auto_columns: "auto",
        auto_rows: "10px max-content",
        auto_flow: "row",
        items: &[
            TextItem {
                content: "XX",
                placement: "",
            },
            TextItem {
                content: "XX",
                placement: "",
            },
            TextItem {
                content: "XX XXXXXXX",
                placement: "",
            },
            TextItem {
                content: "XXXX",
                placement: "",
            },
        ],
    },
    Scene {
        id: "implicit-percentage-column",
        width: 160,
        height: 40,
        border: 0,
        border_color: "#101820",
        columns: "24px",
        rows: "40px",
        auto_columns: "25%",
        auto_rows: "auto",
        auto_flow: "column",
        items: &[
            TextItem {
                content: "XX",
                placement: "",
            },
            TextItem {
                content: "XX",
                placement: "",
            },
        ],
    },
    Scene {
        id: "implicit-fraction-column",
        width: 160,
        height: 40,
        border: 0,
        border_color: "#101820",
        columns: "24px",
        rows: "40px",
        auto_columns: "1fr",
        auto_rows: "auto",
        auto_flow: "column",
        items: &[
            TextItem {
                content: "XX",
                placement: "",
            },
            TextItem {
                content: "XX",
                placement: "",
            },
        ],
    },
    Scene {
        id: "implicit-minmax-column",
        width: 160,
        height: 40,
        border: 0,
        border_color: "#101820",
        columns: "24px",
        rows: "40px",
        auto_columns: "minmax(20px, 40px)",
        auto_rows: "auto",
        auto_flow: "column",
        items: &[
            TextItem {
                content: "XX",
                placement: "",
            },
            TextItem {
                content: "XX",
                placement: "",
            },
        ],
    },
    Scene {
        id: "implicit-fit-content-column",
        width: 160,
        height: 40,
        border: 0,
        border_color: "#101820",
        columns: "24px",
        rows: "40px",
        auto_columns: "fit-content(30px)",
        auto_rows: "auto",
        auto_flow: "column",
        items: &[
            TextItem {
                content: "XX",
                placement: "",
            },
            TextItem {
                content: "XX XXXXXXX",
                placement: "",
            },
        ],
    },
    Scene {
        id: "implicit-min-content-overflow-border",
        width: 44,
        height: 44,
        border: 2,
        border_color: "#11cc88",
        columns: "20px",
        rows: "40px",
        auto_columns: "min-content",
        auto_rows: "auto",
        auto_flow: "column",
        items: &[
            TextItem {
                content: "XX",
                placement: "",
            },
            TextItem {
                content: "XX XXXXXXX",
                placement: "",
            },
        ],
    },
    Scene {
        id: "implicit-calc-column",
        width: 160,
        height: 40,
        border: 0,
        border_color: "#101820",
        columns: "24px",
        rows: "40px",
        auto_columns: "calc(10% + 5px)",
        auto_rows: "auto",
        auto_flow: "column",
        items: &[
            TextItem {
                content: "XX",
                placement: "",
            },
            TextItem {
                content: "XX",
                placement: "",
            },
        ],
    },
    Scene {
        id: "explicit-track-control",
        width: 40,
        height: 24,
        border: 0,
        border_color: "#101820",
        columns: "40px",
        rows: "24px",
        auto_columns: "min-content",
        auto_rows: "max-content",
        auto_flow: "row",
        items: &[TextItem {
            content: "XX",
            placement: "grid-row:1 / 2;grid-column:1 / 2;",
        }],
    },
    Scene {
        id: "row-positive-implicit-locked-item",
        width: 40,
        height: 80,
        border: 0,
        border_color: "#101820",
        columns: "40px",
        rows: "12px",
        auto_columns: "auto",
        auto_rows: "min-content",
        auto_flow: "row",
        items: &[TextItem {
            content: "XX XXXXXXX",
            placement: "grid-row:2 / 3;",
        }],
    },
    Scene {
        id: "column-positive-implicit-locked-item",
        width: 160,
        height: 40,
        border: 0,
        border_color: "#101820",
        columns: "24px",
        rows: "40px",
        auto_columns: "min-content",
        auto_rows: "auto",
        auto_flow: "column",
        items: &[TextItem {
            content: "XX XXXXXXX",
            placement: "grid-column:2 / 3;",
        }],
    },
];

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    let scene = SCENES.iter().find(|scene| scene.id == scene_id)?;
    let root_id = format!("c114-{}-grid", scene.id);
    let root_style = format!(
        "box-sizing:border-box;display:grid;width:{}px;height:{}px;margin:0;padding:0;border:{}px solid {};gap:0;align-content:start;justify-content:start;grid-template-columns:{};grid-template-rows:{};grid-auto-columns:{};grid-auto-rows:{};grid-auto-flow:{};background-color:#101820",
        scene.width,
        scene.height,
        scene.border,
        scene.border_color,
        scene.columns,
        scene.rows,
        scene.auto_columns,
        scene.auto_rows,
        scene.auto_flow,
    );
    let mut source = format!(
        "(() => {{\nconst root = document.createElement('div');\nroot.setAttribute('id', {root_id:?});\nroot.setAttribute('data-node', '');\nroot.setAttribute('style', {root_style:?});\ndocument.appendChild(root);\n"
    );
    let mut node_ids = vec![root_id.clone()];
    for (index, item) in scene.items.iter().enumerate() {
        let item_id = format!("c114-{}-item-{}", scene.id, index + 1);
        let item_style = format!("{TEXT_STYLE}{}", item.placement);
        source.push_str(&format!(
            "const item{index} = document.createElement('div');\nitem{index}.setAttribute('id', {item_id:?});\nitem{index}.setAttribute('data-node', '');\nitem{index}.setAttribute('style', {item_style:?});\nitem{index}.appendChild(document.createTextNode({content:?}));\nroot.appendChild(item{index});\n",
            content = item.content,
        ));
        node_ids.push(item_id);
    }
    source.push_str(&format!(
        "return {{ sceneId:{:?}, rootId:{:?}, nodeIds:{node_ids:?} }};\n}})()",
        scene.id, root_id,
    ));
    Some(source)
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};
    use std::collections::BTreeSet;

    #[test]
    fn exposes_the_20_allowlisted_implicit_intrinsic_track_scenes() {
        let ids: BTreeSet<_> = SCENES.iter().map(|scene| scene.id).collect();
        assert_eq!(SCENES.len(), 20);
        assert_eq!(ids.len(), SCENES.len());
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("known scene must have a source");
            assert!(
                source.len() < 4 * 1024,
                "{} source grew too large",
                scene.id
            );
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains(&format!("grid-template-columns:{}", scene.columns)));
            assert!(source.contains(&format!("grid-template-rows:{}", scene.rows)));
            assert!(source.contains("font-family:Ahem"));
            assert_eq!(
                source.matches("document.createTextNode").count(),
                scene.items.len(),
                "{} source item count drifted",
                scene.id
            );
        }
        assert!(runtime_source("unknown;throw 1").is_none());
    }
}
