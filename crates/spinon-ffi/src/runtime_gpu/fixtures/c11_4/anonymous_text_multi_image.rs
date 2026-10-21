const SCENE_IDS: [&str; 4] = [
    "anonymous-text-two-images-forward",
    "two-images-anonymous-text-reversed",
    "anonymous-text-three-images-forward",
    "three-images-anonymous-text-reversed",
];

#[derive(Clone, Copy)]
struct Image {
    id: &'static str,
    height: u8,
    column: u8,
    span: u8,
    row: u8,
}

const FORWARD_TWO: [Image; 2] = [
    Image {
        id: "c114-image-a-forward",
        height: 16,
        column: 1,
        span: 2,
        row: 1,
    },
    Image {
        id: "c114-image-b-forward",
        height: 32,
        column: 2,
        span: 2,
        row: 2,
    },
];
const REVERSED_TWO: [Image; 2] = [
    Image {
        id: "c114-image-b-reversed",
        height: 32,
        column: 2,
        span: 2,
        row: 2,
    },
    Image {
        id: "c114-image-a-reversed",
        height: 16,
        column: 1,
        span: 2,
        row: 1,
    },
];
const FORWARD_THREE: [Image; 3] = [
    Image {
        id: "c114-image-a-three-forward",
        height: 16,
        column: 1,
        span: 2,
        row: 1,
    },
    Image {
        id: "c114-image-b-three-forward",
        height: 32,
        column: 2,
        span: 2,
        row: 2,
    },
    Image {
        id: "c114-image-c-three-forward",
        height: 48,
        column: 1,
        span: 3,
        row: 3,
    },
];
const REVERSED_THREE: [Image; 3] = [
    Image {
        id: "c114-image-c-three-reversed",
        height: 48,
        column: 1,
        span: 3,
        row: 3,
    },
    Image {
        id: "c114-image-b-three-reversed",
        height: 32,
        column: 2,
        span: 2,
        row: 2,
    },
    Image {
        id: "c114-image-a-three-reversed",
        height: 16,
        column: 1,
        span: 2,
        row: 1,
    },
];

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    if !SCENE_IDS.contains(&scene_id) {
        return None;
    }
    let root_id = format!("c114-{scene_id}-grid");
    let (images, reversed) = match scene_id {
        "anonymous-text-two-images-forward" => (&FORWARD_TWO[..], false),
        "two-images-anonymous-text-reversed" => (&REVERSED_TWO[..], true),
        "anonymous-text-three-images-forward" => (&FORWARD_THREE[..], false),
        "three-images-anonymous-text-reversed" => (&REVERSED_THREE[..], true),
        _ => return None,
    };
    let mut source = format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:220px;height:180px;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:repeat(4,min-content);grid-template-rows:repeat(3,60px);font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#101820');
document.appendChild(root);
const nodeIds = [{root_id:?}];
"#
    );
    if !reversed {
        source.push_str("root.appendChild(document.createTextNode('XX'));\n");
    }
    for image in images {
        source.push_str(&format!(
            r#"const image{index} = document.createElement('img');
image{index}.setAttribute('id', {id:?});
image{index}.setAttribute('data-node', '');
image{index}.setAttribute('src', 'https://spinon.invalid/c14-1-png-basic.png');
image{index}.setAttribute('style', 'box-sizing:border-box;width:auto;height:{height}px;margin:0;padding:0;border:0;grid-column:{column} / span {span};grid-row:{row}');
root.appendChild(image{index});
nodeIds.push({id:?});
"#,
            index = node_ids_len(&source),
            id = image.id,
            height = image.height,
            column = image.column,
            span = image.span,
            row = image.row,
        ));
        if reversed && image.height == 32 {
            source.push_str("root.appendChild(document.createTextNode('XX'));\n");
        }
    }
    source.push_str(&format!(
        "return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds }};\n}})()"
    ));
    Some(source)
}

fn node_ids_len(source: &str) -> usize {
    source.matches("nodeIds.push(").count() + 1
}

#[cfg(test)]
mod tests {
    use super::{SCENE_IDS, runtime_source};
    use std::collections::BTreeSet;

    #[test]
    fn anonymous_text_and_multiple_spanning_image_sources_form_a_fixed_four_scene_matrix() {
        let unique_ids = SCENE_IDS.iter().copied().collect::<BTreeSet<_>>();
        assert_eq!(SCENE_IDS.len(), 4);
        assert_eq!(unique_ids.len(), SCENE_IDS.len());
        for (scene_id, image_count) in [
            ("anonymous-text-two-images-forward", 2),
            ("two-images-anonymous-text-reversed", 2),
            ("anonymous-text-three-images-forward", 3),
            ("three-images-anonymous-text-reversed", 3),
        ] {
            let source = runtime_source(scene_id).expect("known scene");
            assert!(source.len() < 8 * 1024);
            assert!(source.contains(&format!("sceneId:{scene_id:?}")));
            assert!(source.contains("width:220px;height:180px"));
            assert!(source.contains("grid-template-columns:repeat(4,min-content)"));
            assert!(source.contains("grid-template-rows:repeat(3,60px)"));
            assert_eq!(
                source.matches("document.createElement('img')").count(),
                image_count
            );
            assert_eq!(source.matches("document.createTextNode('XX')").count(), 1);
            assert_eq!(source.matches("nodeIds.push(").count(), image_count);
            assert!(source.contains("https://spinon.invalid/c14-1-png-basic.png"));
            for height in [16, 32, 48] {
                if source.contains(&format!("height:{height}px")) {
                    assert!(source.contains("grid-column:"));
                    assert!(source.contains("grid-row:"));
                }
            }
        }
        assert!(runtime_source("anonymous-text-two-images-unknown").is_none());
    }
}
