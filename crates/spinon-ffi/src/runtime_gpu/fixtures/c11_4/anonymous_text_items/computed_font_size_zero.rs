const SCENES: [&str; 4] = [
    "font-size-0-min-content",
    "font-size-0-max-content",
    "font-size-0-text-before-box",
    "font-size-0-whitespace-only",
];

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    if !SCENES.contains(&scene_id) {
        return None;
    }
    let root_id = format!("c114-{scene_id}-grid");
    let (columns, text) = match scene_id {
        "font-size-0-min-content" => ("min-content", "XX XX"),
        "font-size-0-max-content" => ("max-content", "XX XX"),
        "font-size-0-text-before-box" => ("min-content", "XX"),
        "font-size-0-whitespace-only" => ("none", "   "),
        _ => return None,
    };
    let mut source = format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:100px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{columns};grid-template-rows:auto;font-family:Ahem;font-size:0px;font-weight:400;font-style:normal;line-height:10px;tab-size:8;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#101820');
document.appendChild(root);
const nodeIds = [{root_id:?}];
root.appendChild(document.createTextNode({text:?}));
"#
    );
    if scene_id == "font-size-0-text-before-box" {
        let box_id = "c114-font-size-0-following-box";
        source.push_str(&format!(
            r#"const box = document.createElement('div');
box.setAttribute('id', {box_id:?});
box.setAttribute('data-node', '');
box.setAttribute('style', 'display:block;box-sizing:border-box;position:static;width:5px;height:3px;min-width:0px;min-height:0px;margin:0;padding:0;border:0;order:0;background-color:#e5484d');
root.appendChild(box);
nodeIds.push({box_id:?});
"#
        ));
    }
    source.push_str(&format!(
        "return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds }};\n}})()"
    ));
    Some(source)
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};
    use std::collections::BTreeSet;

    #[test]
    fn zero_font_size_matrix_generates_four_exact_text_and_empty_controls() {
        let mut unique = BTreeSet::new();
        for scene_id in SCENES {
            assert!(unique.insert(scene_id), "duplicate {scene_id}");
            let source = runtime_source(scene_id).expect("listed zero font-size scene");
            assert!(source.contains(&format!("sceneId:{scene_id:?}")));
            assert!(source.contains("font-size:0px"));
            assert!(source.contains("width:100px;height:auto"));
            assert!(source.contains("grid-template-rows:auto"));
            assert!(source.contains("line-height:10px"));
            assert!(source.contains("white-space-collapse:collapse;text-wrap-mode:wrap"));
        }
        assert_eq!(unique.len(), 4);
        for scene_id in ["font-size-0-min-content", "font-size-0-max-content"] {
            let source = runtime_source(scene_id).unwrap();
            assert!(source.contains("document.createTextNode(\"XX XX\")"));
        }
        let before_box = runtime_source("font-size-0-text-before-box").unwrap();
        assert!(before_box.contains("grid-template-columns:min-content"));
        assert!(before_box.contains("document.createTextNode(\"XX\")"));
        assert!(before_box.contains("width:5px;height:3px"));
        assert!(before_box.contains("c114-font-size-0-following-box"));
        let whitespace = runtime_source("font-size-0-whitespace-only").unwrap();
        assert!(whitespace.contains("grid-template-columns:none"));
        assert!(whitespace.contains("document.createTextNode(\"   \")"));
        assert!(runtime_source("font-size-0-unknown").is_none());
    }
}
