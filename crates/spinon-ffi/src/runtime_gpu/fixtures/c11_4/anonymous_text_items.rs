const SCENE_IDS: [&str; 7] = [
    "anonymous-min-content-wrap",
    "anonymous-normal-min-content",
    "anonymous-nowrap-min-content",
    "anonymous-comment-continuation",
    "anonymous-runs-around-block",
    "anonymous-hidden-child-continuation",
    "whitespace-does-not-create-item",
];

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    if !SCENE_IDS.contains(&scene_id) {
        return None;
    }
    let root_id = format!("c114-{scene_id}-grid");
    let (width, height, columns, rows, wrap_mode) = match scene_id {
        "anonymous-min-content-wrap" => (120, "20px", "min-content 1fr", "auto", "wrap"),
        "anonymous-normal-min-content" => (20, "auto", "min-content", "auto", "wrap"),
        "anonymous-nowrap-min-content" => (20, "auto", "min-content", "auto", "nowrap"),
        "anonymous-comment-continuation" => (120, "10px", "max-content 1fr", "10px", "wrap"),
        "anonymous-runs-around-block" => (
            120,
            "10px",
            "min-content min-content min-content",
            "10px",
            "wrap",
        ),
        "anonymous-hidden-child-continuation" => (120, "10px", "max-content 1fr", "10px", "wrap"),
        "whitespace-does-not-create-item" => (120, "10px", "20px 30px 40px", "10px", "wrap"),
        _ => return None,
    };
    let mut source = format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:{width}px;height:{height};margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{columns};grid-template-rows:{rows};font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:{wrap_mode};word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#101820');
document.appendChild(root);
const nodeIds = [{root_id:?}];
"#
    );
    match scene_id {
        "anonymous-min-content-wrap"
        | "anonymous-normal-min-content"
        | "anonymous-nowrap-min-content" => source.push_str(
            "root.appendChild(document.createTextNode('XX XX'));\n",
        ),
        "anonymous-comment-continuation" => source.push_str(
            "root.appendChild(document.createTextNode('XX'));\nroot.appendChild(document.createTextNode('XX'));\n",
        ),
        "anonymous-hidden-child-continuation" => source.push_str(
            r#"root.appendChild(document.createTextNode('XX'));
const hidden = document.createElement('span');
hidden.setAttribute('id', 'c114-anonymous-hidden-child-continuation-hidden');
hidden.setAttribute('style', 'display:none');
hidden.appendChild(document.createTextNode('ignored'));
root.appendChild(hidden);
root.appendChild(document.createTextNode('XX'));
"#,
        ),
        "anonymous-runs-around-block" => source.push_str(
            r#"root.appendChild(document.createTextNode('XX'));
const block = document.createElement('div');
block.setAttribute('id', 'c114-anonymous-runs-around-block-item');
block.setAttribute('data-node', '');
block.setAttribute('style', 'box-sizing:border-box;display:block;width:30px;height:10px;min-width:0;min-height:0;margin:0;padding:0;border:0;background-color:#2866f6');
root.appendChild(block);
nodeIds.push('c114-anonymous-runs-around-block-item');
root.appendChild(document.createTextNode('XXXX'));
"#,
        ),
        "whitespace-does-not-create-item" => source.push_str(
            r#"const first = document.createElement('div');
first.setAttribute('id', 'c114-whitespace-does-not-create-item-first');
first.setAttribute('data-node', '');
first.setAttribute('style', 'box-sizing:border-box;display:block;width:10px;height:10px;min-width:0;min-height:0;margin:0;padding:0;border:0;background-color:#e5484d');
root.appendChild(first);
nodeIds.push('c114-whitespace-does-not-create-item-first');
root.appendChild(document.createTextNode(' \t\n'));
const second = document.createElement('div');
second.setAttribute('id', 'c114-whitespace-does-not-create-item-second');
second.setAttribute('data-node', '');
second.setAttribute('style', 'box-sizing:border-box;display:block;width:10px;height:10px;min-width:0;min-height:0;margin:0;padding:0;border:0;background-color:#2a9d8f');
root.appendChild(second);
nodeIds.push('c114-whitespace-does-not-create-item-second');
"#,
        ),
        _ => return None,
    }
    source.push_str(&format!(
        "return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds }};\n}})()"
    ));
    Some(source)
}

#[cfg(test)]
mod tests {
    use super::{SCENE_IDS, runtime_source};
    use std::collections::BTreeSet;

    #[test]
    fn anonymous_grid_text_sources_are_a_fixed_seven_scene_matrix() {
        let unique_ids = SCENE_IDS.iter().copied().collect::<BTreeSet<_>>();
        assert_eq!(SCENE_IDS.len(), 7);
        assert_eq!(unique_ids.len(), SCENE_IDS.len());
        for (scene_id, text_nodes, elements, width, height, wrap_mode) in [
            ("anonymous-min-content-wrap", 1, 0, 120, "20px", "wrap"),
            ("anonymous-normal-min-content", 1, 0, 20, "auto", "wrap"),
            ("anonymous-nowrap-min-content", 1, 0, 20, "auto", "nowrap"),
            ("anonymous-comment-continuation", 2, 0, 120, "10px", "wrap"),
            ("anonymous-runs-around-block", 2, 1, 120, "10px", "wrap"),
            (
                "anonymous-hidden-child-continuation",
                3,
                0,
                120,
                "10px",
                "wrap",
            ),
            ("whitespace-does-not-create-item", 1, 2, 120, "10px", "wrap"),
        ] {
            let source = runtime_source(scene_id).expect("known C11.4 scene");
            assert!(source.len() < 4 * 1024);
            assert!(source.contains(&format!("sceneId:{scene_id:?}")));
            assert!(source.contains(&format!("display:grid;width:{width}px;height:{height};")));
            assert!(source.contains(&format!("text-wrap-mode:{wrap_mode}")));
            assert_eq!(
                source.matches("document.createTextNode(").count(),
                text_nodes
            );
            assert_eq!(
                source.matches("document.createElement('div')").count(),
                elements + 1
            );
            if scene_id == "anonymous-hidden-child-continuation" {
                assert!(source.contains("document.createElement('span')"));
                assert!(source.contains("setAttribute('style', 'display:none')"));
            }
        }
        assert!(runtime_source("unknown-anonymous-scene").is_none());
    }
}
