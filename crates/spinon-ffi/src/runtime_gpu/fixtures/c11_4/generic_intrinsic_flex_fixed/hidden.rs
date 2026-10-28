pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    let rows = match scene_id {
        "column-hidden" => false,
        "row-hidden" => true,
        _ => return None,
    };
    let root_id = format!("novel-mixed-intrinsic-flex-fixed-{scene_id}-root");
    let hidden_id = format!("novel-mixed-intrinsic-flex-fixed-{scene_id}-hidden");
    let a_id = format!("novel-mixed-intrinsic-flex-fixed-{scene_id}-a");
    let b_id = format!("novel-mixed-intrinsic-flex-fixed-{scene_id}-b");
    let c_id = format!("novel-mixed-intrinsic-flex-fixed-{scene_id}-c");
    let (width, height, column_gap, row_gap, columns, grid_rows) = if rows {
        (
            120,
            300,
            0,
            10,
            "120px",
            "minmax(min-content,1fr) 40px minmax(0px,2fr)",
        )
    } else {
        (
            300,
            120,
            10,
            0,
            "minmax(min-content,1fr) 40px minmax(0px,2fr)",
            "120px",
        )
    };
    let writing_mode = if rows {
        "writing-mode:vertical-rl;"
    } else {
        ""
    };

    Some(format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:{width}px;height:{height}px;margin:0;padding:0;column-gap:{column_gap}px;row-gap:{row_gap}px;align-content:start;justify-content:start;align-items:normal;justify-items:normal;direction:ltr;writing-mode:horizontal-tb;grid-template-columns:{columns};grid-template-rows:{grid_rows};background-color:#e8edf5');
document.appendChild(root);
const nodeIds = [{root_id:?}];
const hidden = document.createElement('span');
hidden.setAttribute('id', {hidden_id:?});
hidden.setAttribute('data-node', '');
hidden.setAttribute('style', 'box-sizing:border-box;display:none;position:static;width:auto;height:auto;min-width:auto;min-height:auto;margin:0;padding:0;grid-column:1 / 2;grid-row:1 / 2;{writing_mode}font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:nowrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e500a4;color:#e500a4');
hidden.appendChild(document.createTextNode('XXXXXXXXXXXXXXX'));
root.appendChild(hidden);
nodeIds.push({hidden_id:?});
const itema = document.createElement('div');
itema.setAttribute('id', {a_id:?});
itema.setAttribute('data-node', '');
itema.setAttribute('style', 'box-sizing:border-box;display:block;position:static;width:auto;height:auto;min-width:auto;min-height:auto;margin:0;padding:0;grid-column:1 / 2;grid-row:1 / 2;{writing_mode}font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:nowrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#f5c542;color:#f5c542');
itema.appendChild(document.createTextNode('XXXXXXXXXX'));
root.appendChild(itema);
nodeIds.push({a_id:?});
const itemb = document.createElement('img');
itemb.setAttribute('id', {b_id:?});
itemb.setAttribute('data-node', '');
itemb.setAttribute('src', 'https://spinon.invalid/c14-1-png-basic.png');
itemb.setAttribute('alt', '');
itemb.setAttribute('style', 'box-sizing:border-box;display:block;position:static;width:auto;height:auto;min-width:auto;min-height:auto;margin:0;padding:0;grid-column:{image_column};grid-row:{image_row};background-color:#d73b51');
root.appendChild(itemb);
nodeIds.push({b_id:?});
const itemc = document.createElement('div');
itemc.setAttribute('id', {c_id:?});
itemc.setAttribute('data-node', '');
itemc.setAttribute('style', 'box-sizing:border-box;display:block;position:static;width:auto;height:auto;min-width:auto;min-height:auto;margin:0;padding:0;grid-column:{text_column};grid-row:{text_row};{writing_mode}font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:nowrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#2864dc;color:#2864dc');
itemc.appendChild(document.createTextNode('XXXX'));
root.appendChild(itemc);
nodeIds.push({c_id:?});
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds }};
}})()"#,
        image_column = if rows { "1 / 2" } else { "2 / 3" },
        image_row = if rows { "2 / 3" } else { "1 / 2" },
        text_column = if rows { "1 / 2" } else { "3 / 4" },
        text_row = if rows { "3 / 4" } else { "1 / 2" },
    ))
}

#[cfg(test)]
mod tests {
    use super::runtime_source;

    #[test]
    fn routes_only_the_two_exact_hidden_mixed_track_scenes() {
        for (scene_id, dimensions, tracks, gaps, item_placements, writing_mode) in [
            (
                "column-hidden",
                "width:300px;height:120px",
                "grid-template-columns:minmax(min-content,1fr) 40px minmax(0px,2fr)",
                "column-gap:10px;row-gap:0px",
                [
                    "grid-column:1 / 2;grid-row:1 / 2",
                    "grid-column:2 / 3;grid-row:1 / 2",
                    "grid-column:3 / 4;grid-row:1 / 2",
                ],
                0,
            ),
            (
                "row-hidden",
                "width:120px;height:300px",
                "grid-template-rows:minmax(min-content,1fr) 40px minmax(0px,2fr)",
                "column-gap:0px;row-gap:10px",
                [
                    "grid-column:1 / 2;grid-row:1 / 2",
                    "grid-column:1 / 2;grid-row:2 / 3",
                    "grid-column:1 / 2;grid-row:3 / 4",
                ],
                3,
            ),
        ] {
            let source = runtime_source(scene_id).expect("known hidden scene");
            assert!(source.contains(&format!("sceneId:{scene_id:?}")));
            assert!(source.contains(dimensions));
            assert!(source.contains(tracks));
            assert!(source.contains(gaps));
            assert_eq!(source.matches("document.createElement('span')").count(), 1);
            assert_eq!(source.matches("document.createElement('div')").count(), 3);
            assert_eq!(source.matches("document.createElement('img')").count(), 1);
            assert_eq!(source.matches("document.createTextNode(").count(), 3);
            assert!(source.contains("display:none"));
            assert!(source.contains("XXXXXXXXXXXXXXX"));
            assert_eq!(
                source.matches("writing-mode:vertical-rl;").count(),
                writing_mode
            );
            for placement in item_placements {
                assert!(source.contains(placement));
            }
            assert!(source.contains("https://spinon.invalid/c14-1-png-basic.png"));
        }
        for unknown in ["column-visible-control", "row-hidden-extra", ""] {
            assert!(runtime_source(unknown).is_none());
        }
    }
}
