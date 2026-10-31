const SCENES: [(&str, &str, &str, &str); 16] = [
    (
        "anonymous-letter-spacing-normal-min-content",
        "normal",
        "XX XX",
        "min-content",
    ),
    (
        "anonymous-letter-spacing-normal-max-content",
        "normal",
        "XX XX",
        "max-content",
    ),
    (
        "anonymous-letter-spacing-minus-one-min-content",
        "-1px",
        "XX XX",
        "min-content",
    ),
    (
        "anonymous-letter-spacing-minus-one-max-content",
        "-1px",
        "XX XX",
        "max-content",
    ),
    (
        "anonymous-letter-spacing-zero-min-content",
        "0px",
        "XX XX",
        "min-content",
    ),
    (
        "anonymous-letter-spacing-zero-max-content",
        "0px",
        "XX XX",
        "max-content",
    ),
    (
        "anonymous-letter-spacing-half-min-content",
        "0.5px",
        "XX XX",
        "min-content",
    ),
    (
        "anonymous-letter-spacing-half-max-content",
        "0.5px",
        "XX XX",
        "max-content",
    ),
    (
        "anonymous-letter-spacing-one-min-content",
        "1px",
        "XX XX",
        "min-content",
    ),
    (
        "anonymous-letter-spacing-one-max-content",
        "1px",
        "XX XX",
        "max-content",
    ),
    (
        "anonymous-letter-spacing-two-min-content",
        "2px",
        "XX XX",
        "min-content",
    ),
    (
        "anonymous-letter-spacing-two-max-content",
        "2px",
        "XX XX",
        "max-content",
    ),
    (
        "anonymous-letter-spacing-minus-one-single-min-content",
        "-1px",
        "XX",
        "min-content",
    ),
    (
        "anonymous-letter-spacing-minus-one-single-max-content",
        "-1px",
        "XX",
        "max-content",
    ),
    (
        "anonymous-letter-spacing-one-single-min-content",
        "1px",
        "XX",
        "min-content",
    ),
    (
        "anonymous-letter-spacing-one-single-max-content",
        "1px",
        "XX",
        "max-content",
    ),
];

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    let (_, letter_spacing, text, column) = SCENES
        .iter()
        .find(|(known_id, _, _, _)| *known_id == scene_id)?;
    let root_id = format!("c114-{scene_id}-grid");
    Some(format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:120px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{column};grid-template-rows:auto;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;tab-size:8;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:{letter_spacing};word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#101820');
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
    fn computed_letter_spacing_matrix_generates_sixteen_exact_direct_text_scenes() {
        let mut unique = BTreeSet::new();
        for (scene_id, value, text, column) in SCENES {
            assert!(unique.insert(scene_id), "duplicate {scene_id}");
            let source = runtime_source(scene_id).expect("listed letter-spacing scene");
            assert!(source.contains(&format!("sceneId:{scene_id:?}")));
            assert!(source.contains(&format!("letter-spacing:{value};")));
            assert!(source.contains(&format!("grid-template-columns:{column};")));
            assert!(source.contains("display:grid;width:120px;height:auto"));
            assert!(source.contains("grid-template-rows:auto"));
            assert!(source.contains(&format!("document.createTextNode({text:?})")));
        }
        assert_eq!(unique.len(), 16);
        assert!(runtime_source("anonymous-letter-spacing-unlisted").is_none());
    }
}
