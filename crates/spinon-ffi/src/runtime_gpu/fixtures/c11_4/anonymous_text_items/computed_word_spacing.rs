const SCENES: [(&str, &str, &str, &str); 24] = [
    (
        "word-spacing-normal-two-words-min-content",
        "normal",
        "XX XX",
        "min-content",
    ),
    (
        "word-spacing-normal-two-words-max-content",
        "normal",
        "XX XX",
        "max-content",
    ),
    (
        "word-spacing-normal-collapsed-three-words-min-content",
        "normal",
        "XX   XX XX  ",
        "min-content",
    ),
    (
        "word-spacing-normal-collapsed-three-words-max-content",
        "normal",
        "XX   XX XX  ",
        "max-content",
    ),
    (
        "word-spacing-normal-single-word-min-content",
        "normal",
        "XX",
        "min-content",
    ),
    (
        "word-spacing-normal-single-word-max-content",
        "normal",
        "XX",
        "max-content",
    ),
    (
        "word-spacing-zero-two-words-min-content",
        "0px",
        "XX XX",
        "min-content",
    ),
    (
        "word-spacing-zero-two-words-max-content",
        "0px",
        "XX XX",
        "max-content",
    ),
    (
        "word-spacing-zero-collapsed-three-words-min-content",
        "0px",
        "XX   XX XX  ",
        "min-content",
    ),
    (
        "word-spacing-zero-collapsed-three-words-max-content",
        "0px",
        "XX   XX XX  ",
        "max-content",
    ),
    (
        "word-spacing-zero-single-word-min-content",
        "0px",
        "XX",
        "min-content",
    ),
    (
        "word-spacing-zero-single-word-max-content",
        "0px",
        "XX",
        "max-content",
    ),
    (
        "word-spacing-minus-one-two-words-min-content",
        "-1px",
        "XX XX",
        "min-content",
    ),
    (
        "word-spacing-minus-one-two-words-max-content",
        "-1px",
        "XX XX",
        "max-content",
    ),
    (
        "word-spacing-minus-one-collapsed-three-words-min-content",
        "-1px",
        "XX   XX XX  ",
        "min-content",
    ),
    (
        "word-spacing-minus-one-collapsed-three-words-max-content",
        "-1px",
        "XX   XX XX  ",
        "max-content",
    ),
    (
        "word-spacing-minus-one-single-word-min-content",
        "-1px",
        "XX",
        "min-content",
    ),
    (
        "word-spacing-minus-one-single-word-max-content",
        "-1px",
        "XX",
        "max-content",
    ),
    (
        "word-spacing-plus-one-two-words-min-content",
        "1px",
        "XX XX",
        "min-content",
    ),
    (
        "word-spacing-plus-one-two-words-max-content",
        "1px",
        "XX XX",
        "max-content",
    ),
    (
        "word-spacing-plus-one-collapsed-three-words-min-content",
        "1px",
        "XX   XX XX  ",
        "min-content",
    ),
    (
        "word-spacing-plus-one-collapsed-three-words-max-content",
        "1px",
        "XX   XX XX  ",
        "max-content",
    ),
    (
        "word-spacing-plus-one-single-word-min-content",
        "1px",
        "XX",
        "min-content",
    ),
    (
        "word-spacing-plus-one-single-word-max-content",
        "1px",
        "XX",
        "max-content",
    ),
];

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    let (_, word_spacing, text, column) = SCENES
        .iter()
        .find(|(known_id, _, _, _)| *known_id == scene_id)?;
    let root_id = format!("c114-{scene_id}-grid");
    Some(format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:120px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{column};grid-template-rows:auto;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;tab-size:8;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:{word_spacing};text-indent:0px;color:#e5484d;background-color:#101820');
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
    fn computed_word_spacing_matrix_generates_twenty_four_exact_direct_text_scenes() {
        let mut unique = BTreeSet::new();
        for (scene_id, word_spacing, text, column) in SCENES {
            assert!(unique.insert(scene_id), "duplicate {scene_id}");
            let source = runtime_source(scene_id).expect("listed word-spacing scene");
            assert!(source.contains(&format!("sceneId:{scene_id:?}")));
            assert!(source.contains(&format!("word-spacing:{word_spacing};")));
            assert!(source.contains(&format!("grid-template-columns:{column};")));
            assert!(source.contains("display:grid;width:120px;height:auto"));
            assert!(source.contains("grid-template-rows:auto"));
            assert!(source.contains(&format!("document.createTextNode({text:?})")));
        }
        assert_eq!(unique.len(), 24);
        assert!(runtime_source("word-spacing-unlisted").is_none());
    }
}
