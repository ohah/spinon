const SCENE_IDS: [&str; 5] = [
    "anonymous-text-image-text",
    "image-before-comment-group",
    "comment-group-before-image",
    "whitespace-around-image",
    "hidden-child-around-image",
];

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    if !SCENE_IDS.contains(&scene_id) {
        return None;
    }
    let root_id = format!("c114-{scene_id}-grid");
    let image_id = match scene_id {
        "anonymous-text-image-text" => "c114-anonymous-text-image-middle-image",
        "image-before-comment-group" => "c114-image-before-comment-group-image",
        "comment-group-before-image" => "c114-comment-group-before-image-image",
        "whitespace-around-image" => "c114-whitespace-around-image-image",
        "hidden-child-around-image" => "c114-hidden-child-around-image-image",
        _ => return None,
    };
    let mut source = format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:220px;height:60px;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:repeat(4,min-content);grid-template-rows:60px;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#101820');
document.appendChild(root);
const nodeIds = [{root_id:?}];
"#
    );
    let append_image = format!(
        r#"const image = document.createElement('img');
image.setAttribute('id', {image_id:?});
image.setAttribute('data-node', '');
image.setAttribute('src', 'https://spinon.invalid/c14-1-png-basic.png');
image.setAttribute('style', 'box-sizing:border-box;width:auto;height:48px;margin:0;padding:0;border:0;grid-column:span 2');
root.appendChild(image);
nodeIds.push({image_id:?});
"#
    );
    match scene_id {
        "anonymous-text-image-text" => {
            source.push_str("root.appendChild(document.createTextNode('XX'));\n");
            source.push_str(&append_image);
            source.push_str("root.appendChild(document.createTextNode('XX'));\n");
        }
        "image-before-comment-group" => {
            source.push_str(&append_image);
            source.push_str(
                "root.appendChild(document.createTextNode('XX'));\nroot.appendChild(document.createTextNode('XX'));\n",
            );
        }
        "comment-group-before-image" => {
            source.push_str(
                "root.appendChild(document.createTextNode('XX'));\nroot.appendChild(document.createTextNode('XX'));\n",
            );
            source.push_str(&append_image);
        }
        "whitespace-around-image" => {
            source.push_str("root.appendChild(document.createTextNode(' \\t\\n'));\n");
            source.push_str(&append_image);
            source.push_str("root.appendChild(document.createTextNode('\\n\\tXX'));\n");
        }
        "hidden-child-around-image" => {
            source.push_str("root.appendChild(document.createTextNode('XX'));\n");
            source.push_str(
                r#"const hidden = document.createElement('span');
hidden.setAttribute('id', 'c114-hidden-child-around-image-hidden');
hidden.setAttribute('style', 'display:none');
hidden.appendChild(document.createTextNode('ignored'));
root.appendChild(hidden);
"#,
            );
            source.push_str(&append_image);
            source.push_str("root.appendChild(document.createTextNode('XX'));\n");
        }
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
    fn anonymous_text_and_inline_image_sources_form_a_fixed_five_scene_matrix() {
        let unique_ids = SCENE_IDS.iter().copied().collect::<BTreeSet<_>>();
        assert_eq!(SCENE_IDS.len(), 5);
        assert_eq!(unique_ids.len(), SCENE_IDS.len());
        for (scene_id, text_nodes) in [
            ("anonymous-text-image-text", 2),
            ("image-before-comment-group", 2),
            ("comment-group-before-image", 2),
            ("whitespace-around-image", 2),
            ("hidden-child-around-image", 3),
        ] {
            let source = runtime_source(scene_id).expect("known scene");
            assert!(source.len() < 4 * 1024);
            assert!(source.contains(&format!("sceneId:{scene_id:?}")));
            assert!(source.contains("grid-template-columns:repeat(4,min-content)"));
            assert!(source.contains("grid-column:span 2"));
            assert!(source.contains("height:48px"));
            assert_eq!(source.matches("document.createElement('img')").count(), 1);
            assert_eq!(
                source.matches("document.createTextNode(").count(),
                text_nodes
            );
            assert!(!source.contains("display:block;width:auto;height:48px"));
            assert!(source.contains("https://spinon.invalid/c14-1-png-basic.png"));
            assert!(source.contains("nodeIds.push("));
            if scene_id == "hidden-child-around-image" {
                assert!(source.contains("display:none"));
                assert!(source.contains("createTextNode('ignored')"));
            }
        }
        assert!(runtime_source("anonymous-text-image-unknown").is_none());
    }
}
