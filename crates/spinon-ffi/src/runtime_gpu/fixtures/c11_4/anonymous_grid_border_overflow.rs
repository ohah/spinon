#[derive(Clone, Copy)]
struct Image {
    id: &'static str,
    height: u8,
    column: u8,
    row: u8,
}

#[derive(Clone, Copy)]
enum Child {
    Text,
    Image(Image),
}

const FORWARD: [Child; 4] = [
    Child::Text,
    Child::Image(Image {
        id: "c114-overflow-image-a-forward",
        height: 16,
        column: 1,
        row: 1,
    }),
    Child::Text,
    Child::Image(Image {
        id: "c114-overflow-image-b-forward",
        height: 32,
        column: 2,
        row: 2,
    }),
];

const REVERSED: [Child; 4] = [
    Child::Image(Image {
        id: "c114-overflow-image-b-reversed",
        height: 32,
        column: 2,
        row: 2,
    }),
    Child::Text,
    Child::Image(Image {
        id: "c114-overflow-image-a-reversed",
        height: 16,
        column: 1,
        row: 1,
    }),
    Child::Text,
];

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    let children = match scene_id {
        "anonymous-runs-two-images-forward" => &FORWARD[..],
        "two-images-anonymous-runs-reversed" => &REVERSED[..],
        _ => return None,
    };
    let root_id = format!("c114-{scene_id}-grid");
    let mut source = format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:40px;height:184px;margin:0;padding:0;border:2px solid #f0c674;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:repeat(4,min-content);grid-template-rows:repeat(3,60px);font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#101820');
document.appendChild(root);
const nodeIds = [{root_id:?}];
"#
    );
    let mut image_index = 0;
    for child in children {
        match child {
            Child::Text => source.push_str("root.appendChild(document.createTextNode('XX'));\n"),
            Child::Image(image) => {
                source.push_str(&format!(
                    r#"const image{image_index} = document.createElement('img');
image{image_index}.setAttribute('id', {id:?});
image{image_index}.setAttribute('data-node', '');
image{image_index}.setAttribute('src', 'https://spinon.invalid/c14-1-png-basic.png');
image{image_index}.setAttribute('style', 'box-sizing:border-box;width:auto;height:{height}px;margin:0;padding:0;border:0;grid-column:{column} / span 2;grid-row:{row}');
root.appendChild(image{image_index});
nodeIds.push({id:?});
"#,
                    id = image.id,
                    height = image.height,
                    column = image.column,
                    row = image.row,
                ));
                image_index += 1;
            }
        }
    }
    source.push_str(&format!(
        "return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds }};\n}})()"
    ));
    Some(source)
}

#[cfg(test)]
mod tests {
    use super::runtime_source;
    use std::collections::BTreeSet;

    #[test]
    fn two_text_runs_and_images_have_two_bordered_overflow_scenes() {
        let scene_ids = [
            "anonymous-runs-two-images-forward",
            "two-images-anonymous-runs-reversed",
        ];
        let unique = scene_ids.into_iter().collect::<BTreeSet<_>>();
        assert_eq!(unique.len(), 2);
        for scene_id in scene_ids {
            let source = runtime_source(scene_id).expect("known bordered Grid scene");
            assert!(source.len() < 8 * 1024);
            assert!(source.contains(&format!("sceneId:{scene_id:?}")));
            assert!(source.contains("width:40px;height:184px"));
            assert!(source.contains("border:2px solid #f0c674"));
            assert!(!source.contains("overflow:visible"));
            assert!(source.contains("grid-template-columns:repeat(4,min-content)"));
            assert_eq!(source.matches("document.createTextNode('XX')").count(), 2);
            assert_eq!(source.matches("document.createElement('img')").count(), 2);
            assert_eq!(source.matches("nodeIds.push(").count(), 2);
            assert!(source.contains("https://spinon.invalid/c14-1-png-basic.png"));
        }
        assert!(runtime_source("anonymous-runs-two-images-unknown").is_none());
    }
}
