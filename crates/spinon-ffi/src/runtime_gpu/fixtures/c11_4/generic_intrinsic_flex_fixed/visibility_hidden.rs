pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    let (rows, visibility) = match scene_id {
        "column-visibility-hidden" => (false, "hidden"),
        "column-visibility-visible-control" => (false, "visible"),
        "row-visibility-hidden" => (true, "hidden"),
        "row-visibility-visible-control" => (true, "visible"),
        _ => return None,
    };
    let root_id = format!("novel-mixed-intrinsic-flex-fixed-{scene_id}-root");
    let a_id = format!("novel-mixed-intrinsic-flex-fixed-{scene_id}-a");
    let span_id = format!("novel-mixed-intrinsic-flex-fixed-{scene_id}-visibility");
    let image_id = format!("novel-mixed-intrinsic-flex-fixed-{scene_id}-b");
    let c_id = format!("novel-mixed-intrinsic-flex-fixed-{scene_id}-c");
    let (width, height, column_gap, row_gap, columns, rows_style) = if rows {
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
    let (span_column, span_row, image_column, image_row, c_column, c_row) = if rows {
        ("1 / 2", "1 / 2", "1 / 2", "2 / 3", "1 / 2", "3 / 4")
    } else {
        ("1 / 2", "1 / 2", "2 / 3", "1 / 2", "3 / 4", "1 / 2")
    };

    Some(format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:{width}px;height:{height}px;margin:0;padding:0;column-gap:{column_gap}px;row-gap:{row_gap}px;align-content:start;justify-content:start;align-items:normal;justify-items:normal;direction:ltr;writing-mode:horizontal-tb;grid-template-columns:{columns};grid-template-rows:{rows_style};background-color:#e8edf5');
document.appendChild(root);
const nodeIds = [{root_id:?}];
const itema = document.createElement('div');
itema.setAttribute('id', {a_id:?});
itema.setAttribute('data-node', '');
itema.setAttribute('style', 'box-sizing:border-box;display:block;position:static;width:auto;height:auto;min-width:auto;min-height:auto;margin:0;padding:0;grid-column:1 / 2;grid-row:1 / 2;{writing_mode}font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:nowrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#2864dc;color:#2864dc');
itema.appendChild(document.createTextNode('XXXXXXXXXX'));
root.appendChild(itema);
nodeIds.push({a_id:?});
const visibility = document.createElement('span');
visibility.setAttribute('id', {span_id:?});
visibility.setAttribute('data-node', '');
visibility.setAttribute('style', 'box-sizing:border-box;display:block;position:static;width:auto;height:auto;min-width:auto;min-height:auto;margin:0;padding:0;grid-column:{span_column};grid-row:{span_row};{writing_mode}font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:nowrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;visibility:{visibility};background-color:#8e44ad;color:#8e44ad');
visibility.appendChild(document.createTextNode('XXXXXXXXXXXXXXX'));
root.appendChild(visibility);
nodeIds.push({span_id:?});
const image = document.createElement('img');
image.setAttribute('id', {image_id:?});
image.setAttribute('data-node', '');
image.setAttribute('src', 'https://spinon.invalid/c14-1-png-basic.png');
image.setAttribute('alt', '');
image.setAttribute('style', 'box-sizing:border-box;display:block;position:static;width:auto;height:auto;min-width:auto;min-height:auto;margin:0;padding:0;grid-column:{image_column};grid-row:{image_row};background-color:#d73b51');
root.appendChild(image);
nodeIds.push({image_id:?});
const itemc = document.createElement('div');
itemc.setAttribute('id', {c_id:?});
itemc.setAttribute('data-node', '');
itemc.setAttribute('style', 'box-sizing:border-box;display:block;position:static;width:auto;height:auto;min-width:auto;min-height:auto;margin:0;padding:0;grid-column:{c_column};grid-row:{c_row};{writing_mode}font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:nowrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#2864dc;color:#2864dc');
itemc.appendChild(document.createTextNode('XXXX'));
root.appendChild(itemc);
nodeIds.push({c_id:?});
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds }};
}})()"#
    ))
}

#[cfg(test)]
mod tests {
    use super::runtime_source;
    use crate::runtime_gpu::fixtures::c11_4::c11_4_grid_track_sizing_runtime_source;

    #[test]
    fn routes_only_the_four_exact_visibility_mixed_track_scenes() {
        for (scene_id, rows, visibility) in [
            ("column-visibility-hidden", false, "hidden"),
            ("column-visibility-visible-control", false, "visible"),
            ("row-visibility-hidden", true, "hidden"),
            ("row-visibility-visible-control", true, "visible"),
        ] {
            let source = runtime_source(scene_id).expect("fixed visibility scene");
            assert!(source.contains(&format!("sceneId:{scene_id:?}")));
            assert!(source.contains(&format!("novel-mixed-intrinsic-flex-fixed-{scene_id}-root")));
            assert!(source.contains("minmax(min-content,1fr) 40px minmax(0px,2fr)"));
            assert_eq!(source.matches("document.createElement('div')").count(), 3);
            assert_eq!(source.matches("document.createElement('span')").count(), 1);
            assert_eq!(source.matches("document.createElement('img')").count(), 1);
            assert_eq!(source.matches("document.createTextNode(").count(), 3);
            assert!(source.contains(&format!("visibility:{visibility};")));
            assert!(source.contains("XXXXXXXXXXXXXXX"));
            assert!(source.contains("https://spinon.invalid/c14-1-png-basic.png"));
            assert!(c11_4_grid_track_sizing_runtime_source(scene_id).is_ok());
            assert_eq!(
                source.matches("writing-mode:vertical-rl;").count(),
                if rows { 3 } else { 0 }
            );

            let a_id = format!("novel-mixed-intrinsic-flex-fixed-{scene_id}-a");
            let span_id = format!("novel-mixed-intrinsic-flex-fixed-{scene_id}-visibility");
            let image_id = format!("novel-mixed-intrinsic-flex-fixed-{scene_id}-b");
            let c_id = format!("novel-mixed-intrinsic-flex-fixed-{scene_id}-c");
            let a = source
                .find(&format!("setAttribute('id', {a_id:?})"))
                .unwrap();
            let span = source
                .find(&format!("setAttribute('id', {span_id:?})"))
                .unwrap();
            let image = source
                .find(&format!("setAttribute('id', {image_id:?})"))
                .unwrap();
            let c = source
                .find(&format!("setAttribute('id', {c_id:?})"))
                .unwrap();
            assert!(a < span && span < image && image < c);
        }
        for unknown in [
            "column-visibility-hidden-extra",
            "column-visibility-collapse",
            "",
        ] {
            assert!(runtime_source(unknown).is_none());
            assert!(c11_4_grid_track_sizing_runtime_source(unknown).is_err());
        }
    }
}
