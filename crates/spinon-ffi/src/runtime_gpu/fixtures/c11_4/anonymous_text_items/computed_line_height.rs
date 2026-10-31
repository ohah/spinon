const SCENES: [(&str, &str, &str, &str); 16] = [
    (
        "line-height-normal-two-lines",
        "normal",
        "XX\nXX",
        "preserve-breaks",
    ),
    ("line-height-zero-number", "0", "XX\nXX", "preserve-breaks"),
    (
        "line-height-half-number",
        "0.5",
        "XX\nXX",
        "preserve-breaks",
    ),
    ("line-height-one-number", "1", "XX\nXX", "preserve-breaks"),
    (
        "line-height-one-point-five-number",
        "1.5",
        "XX\nXX",
        "preserve-breaks",
    ),
    ("line-height-two-number", "2", "XX\nXX", "preserve-breaks"),
    (
        "line-height-zero-length",
        "0px",
        "XX\nXX",
        "preserve-breaks",
    ),
    (
        "line-height-five-length",
        "5px",
        "XX\nXX",
        "preserve-breaks",
    ),
    (
        "line-height-twelve-point-five-length",
        "12.5px",
        "XX\nXX",
        "preserve-breaks",
    ),
    (
        "line-height-one-hundred-twenty-five-percent",
        "125%",
        "XX\nXX",
        "preserve-breaks",
    ),
    (
        "line-height-one-point-two-five-em",
        "1.25em",
        "XX\nXX",
        "preserve-breaks",
    ),
    (
        "line-height-calc-length",
        "calc(10px + 2.5px)",
        "XX\nXX",
        "preserve-breaks",
    ),
    (
        "line-height-normal-single-line",
        "normal",
        "XX",
        "preserve-breaks",
    ),
    (
        "line-height-half-number-single-line",
        "0.5",
        "XX",
        "preserve-breaks",
    ),
    (
        "line-height-one-point-five-soft-wrap",
        "1.5",
        "XX XX",
        "collapse",
    ),
    (
        "line-height-zero-number-soft-wrap",
        "0",
        "XX XX",
        "collapse",
    ),
];

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    let (_, source_value, text, white_space_collapse) = SCENES
        .iter()
        .find(|(known_id, _, _, _)| *known_id == scene_id)?;
    let root_id = format!("c114-{scene_id}-grid");
    Some(format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:20px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:20px;grid-template-rows:auto;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:{source_value};tab-size:8;white-space-collapse:{white_space_collapse};text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#101820');
document.appendChild(root);
const nodeIds = [{root_id:?}];
root.appendChild(document.createTextNode({text:?}));
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds }};
}})()"#
    ))
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};
    use std::collections::BTreeSet;

    #[test]
    fn computed_line_height_matrix_generates_sixteen_direct_text_scenes() {
        let mut unique = BTreeSet::new();
        for (scene_id, source_value, text, white_space_collapse) in SCENES {
            assert!(unique.insert(scene_id), "duplicate {scene_id}");
            let source = runtime_source(scene_id).expect("listed line-height scene");
            assert!(source.contains(&format!("sceneId:{scene_id:?}")));
            assert!(source.contains(&format!("line-height:{source_value};")));
            assert!(source.contains(&format!("white-space-collapse:{white_space_collapse};")));
            assert!(source.contains(&format!("document.createTextNode({text:?})")));
            assert!(source.contains("grid-template-rows:auto"));
            assert!(source.contains("display:grid;width:20px;height:auto"));
        }
        assert_eq!(unique.len(), 16);
        assert!(runtime_source("line-height-unlisted").is_none());
    }
}
