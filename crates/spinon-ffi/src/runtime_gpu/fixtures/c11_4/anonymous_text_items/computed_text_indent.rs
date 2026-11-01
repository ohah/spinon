const SCENES: [(&str, &str, &str, &str); 30] = [
    (
        "text-indent-zero-two-words-min-content",
        "0px",
        "XX XX",
        "min-content",
    ),
    (
        "text-indent-zero-two-words-max-content",
        "0px",
        "XX XX",
        "max-content",
    ),
    (
        "text-indent-zero-collapsed-three-words-min-content",
        "0px",
        "XX   XX XX  ",
        "min-content",
    ),
    (
        "text-indent-zero-collapsed-three-words-max-content",
        "0px",
        "XX   XX XX  ",
        "max-content",
    ),
    (
        "text-indent-zero-single-word-min-content",
        "0px",
        "XX",
        "min-content",
    ),
    (
        "text-indent-zero-single-word-max-content",
        "0px",
        "XX",
        "max-content",
    ),
    (
        "text-indent-five-two-words-min-content",
        "5px",
        "XX XX",
        "min-content",
    ),
    (
        "text-indent-five-two-words-max-content",
        "5px",
        "XX XX",
        "max-content",
    ),
    (
        "text-indent-five-collapsed-three-words-min-content",
        "5px",
        "XX   XX XX  ",
        "min-content",
    ),
    (
        "text-indent-five-collapsed-three-words-max-content",
        "5px",
        "XX   XX XX  ",
        "max-content",
    ),
    (
        "text-indent-five-single-word-min-content",
        "5px",
        "XX",
        "min-content",
    ),
    (
        "text-indent-five-single-word-max-content",
        "5px",
        "XX",
        "max-content",
    ),
    (
        "text-indent-ten-two-words-min-content",
        "10px",
        "XX XX",
        "min-content",
    ),
    (
        "text-indent-ten-two-words-max-content",
        "10px",
        "XX XX",
        "max-content",
    ),
    (
        "text-indent-ten-collapsed-three-words-min-content",
        "10px",
        "XX   XX XX  ",
        "min-content",
    ),
    (
        "text-indent-ten-collapsed-three-words-max-content",
        "10px",
        "XX   XX XX  ",
        "max-content",
    ),
    (
        "text-indent-ten-single-word-min-content",
        "10px",
        "XX",
        "min-content",
    ),
    (
        "text-indent-ten-single-word-max-content",
        "10px",
        "XX",
        "max-content",
    ),
    (
        "text-indent-negative-five-first-word-long-min-content",
        "-5px",
        "XXX X",
        "min-content",
    ),
    (
        "text-indent-negative-five-first-word-long-max-content",
        "-5px",
        "XXX X",
        "max-content",
    ),
    (
        "text-indent-negative-ten-first-word-long-min-content",
        "-10px",
        "XXX X",
        "min-content",
    ),
    (
        "text-indent-negative-ten-first-word-long-max-content",
        "-10px",
        "XXX X",
        "max-content",
    ),
    (
        "text-indent-negative-five-first-word-short-min-content",
        "-5px",
        "X XXX",
        "min-content",
    ),
    (
        "text-indent-negative-five-first-word-short-max-content",
        "-5px",
        "X XXX",
        "max-content",
    ),
    (
        "text-indent-negative-ten-first-word-short-min-content",
        "-10px",
        "X XXX",
        "min-content",
    ),
    (
        "text-indent-negative-ten-first-word-short-max-content",
        "-10px",
        "X XXX",
        "max-content",
    ),
    (
        "text-indent-negative-five-single-word-min-content",
        "-5px",
        "XX",
        "min-content",
    ),
    (
        "text-indent-negative-five-single-word-max-content",
        "-5px",
        "XX",
        "max-content",
    ),
    (
        "text-indent-negative-ten-single-word-min-content",
        "-10px",
        "XX",
        "min-content",
    ),
    (
        "text-indent-negative-ten-single-word-max-content",
        "-10px",
        "XX",
        "max-content",
    ),
];

const FLAG_SCENES: [(&str, &str, &str, &str, &str); 20] = [
    (
        "text-indent-flags-soft-min-content-indent",
        "10px",
        "XXX X",
        "min-content",
        "collapse",
    ),
    (
        "text-indent-flags-soft-max-content-indent",
        "10px",
        "XXX X",
        "max-content",
        "collapse",
    ),
    (
        "text-indent-flags-soft-min-content-each-line",
        "10px each-line",
        "XXX X",
        "min-content",
        "collapse",
    ),
    (
        "text-indent-flags-soft-max-content-each-line",
        "10px each-line",
        "XXX X",
        "max-content",
        "collapse",
    ),
    (
        "text-indent-flags-soft-min-content-hanging",
        "10px hanging",
        "XXX X",
        "min-content",
        "collapse",
    ),
    (
        "text-indent-flags-soft-max-content-hanging",
        "10px hanging",
        "XXX X",
        "max-content",
        "collapse",
    ),
    (
        "text-indent-flags-soft-min-content-hanging-each-line",
        "10px hanging each-line",
        "XXX X",
        "min-content",
        "collapse",
    ),
    (
        "text-indent-flags-soft-max-content-hanging-each-line",
        "10px hanging each-line",
        "XXX X",
        "max-content",
        "collapse",
    ),
    (
        "text-indent-flags-soft-min-content-none",
        "0px",
        "XXX X",
        "min-content",
        "collapse",
    ),
    (
        "text-indent-flags-soft-max-content-none",
        "0px",
        "XXX X",
        "max-content",
        "collapse",
    ),
    (
        "text-indent-flags-forced-min-content-indent",
        "10px",
        "XXX\nX",
        "min-content",
        "preserve-breaks",
    ),
    (
        "text-indent-flags-forced-max-content-indent",
        "10px",
        "XXX\nX",
        "max-content",
        "preserve-breaks",
    ),
    (
        "text-indent-flags-forced-min-content-each-line",
        "10px each-line",
        "XXX\nX",
        "min-content",
        "preserve-breaks",
    ),
    (
        "text-indent-flags-forced-max-content-each-line",
        "10px each-line",
        "XXX\nX",
        "max-content",
        "preserve-breaks",
    ),
    (
        "text-indent-flags-forced-min-content-hanging",
        "10px hanging",
        "XXX\nX",
        "min-content",
        "preserve-breaks",
    ),
    (
        "text-indent-flags-forced-max-content-hanging",
        "10px hanging",
        "XXX\nX",
        "max-content",
        "preserve-breaks",
    ),
    (
        "text-indent-flags-forced-min-content-hanging-each-line",
        "10px hanging each-line",
        "XXX\nX",
        "min-content",
        "preserve-breaks",
    ),
    (
        "text-indent-flags-forced-max-content-hanging-each-line",
        "10px hanging each-line",
        "XXX\nX",
        "max-content",
        "preserve-breaks",
    ),
    (
        "text-indent-flags-forced-min-content-none",
        "0px",
        "XXX\nX",
        "min-content",
        "preserve-breaks",
    ),
    (
        "text-indent-flags-forced-max-content-none",
        "0px",
        "XXX\nX",
        "max-content",
        "preserve-breaks",
    ),
];

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    if let Some((_, text_indent, text, column, white_space_collapse)) = FLAG_SCENES
        .iter()
        .find(|(known_id, ..)| *known_id == scene_id)
    {
        let root_id = format!("c114-{scene_id}-grid");
        return Some(format!(
            r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:120px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{column};grid-template-rows:auto;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;tab-size:8;white-space-collapse:{white_space_collapse};text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:{text_indent};color:#e5484d;background-color:#101820');
document.appendChild(root);
const nodeIds = [{root_id:?}];
root.appendChild(document.createTextNode({text:?}));
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds }};
}})()"#
        ));
    }
    let (_, text_indent, text, column) = SCENES
        .iter()
        .find(|(known_id, _, _, _)| *known_id == scene_id)?;
    let root_id = format!("c114-{scene_id}-grid");
    Some(format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:120px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{column};grid-template-rows:auto;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;tab-size:8;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:{text_indent};color:#e5484d;background-color:#101820');
document.appendChild(root);
const nodeIds = [{root_id:?}];
root.appendChild(document.createTextNode({text:?}));
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds }};
}})()"#
    ))
}

#[cfg(test)]
mod tests {
    use super::{FLAG_SCENES, SCENES, runtime_source};
    use std::collections::BTreeSet;

    #[test]
    fn computed_text_indent_matrix_generates_thirty_exact_direct_text_scenes() {
        let mut unique = BTreeSet::new();
        for (scene_id, text_indent, text, column) in SCENES {
            assert!(unique.insert(scene_id), "duplicate {scene_id}");
            let source = runtime_source(scene_id).expect("listed text-indent scene");
            assert!(source.contains(&format!("sceneId:{scene_id:?}")));
            assert!(source.contains(&format!("text-indent:{text_indent};")));
            assert!(source.contains(&format!("grid-template-columns:{column};")));
            assert!(source.contains("display:grid;width:120px;height:auto"));
            assert!(source.contains("grid-template-rows:auto"));
            assert!(source.contains(&format!("document.createTextNode({text:?})")));
        }
        assert_eq!(unique.len(), 30);
        assert!(runtime_source("text-indent-negative-ten-first-word-short-min-content").is_some());
        assert!(runtime_source("text-indent-unlisted").is_none());
    }

    #[test]
    fn computed_text_indent_flags_matrix_generates_twenty_direct_text_scenes() {
        let mut unique = BTreeSet::new();
        for (scene_id, text_indent, text, column, white_space_collapse) in FLAG_SCENES {
            assert!(unique.insert(scene_id), "duplicate {scene_id}");
            let source = runtime_source(scene_id).expect("listed text-indent flags scene");
            assert!(source.contains(&format!("sceneId:{scene_id:?}")));
            assert!(source.contains(&format!("text-indent:{text_indent};")));
            assert!(source.contains(&format!("grid-template-columns:{column};")));
            assert!(source.contains(&format!("white-space-collapse:{white_space_collapse};")));
            assert!(source.contains(&format!("document.createTextNode({text:?})")));
        }
        assert_eq!(unique.len(), 20);
        assert!(runtime_source("text-indent-flags-forced-max-content-hanging-each-line").is_some());
    }
}
