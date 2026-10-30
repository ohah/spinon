const SCENE_IDS: [&str; 31] = [
    "anonymous-min-content-wrap",
    "anonymous-normal-min-content",
    "anonymous-nowrap-min-content",
    "anonymous-normal-newline-max-content",
    "anonymous-pre-line-max-content",
    "anonymous-comment-continuation",
    "anonymous-runs-around-block",
    "anonymous-hidden-child-continuation",
    "whitespace-does-not-create-item",
    "anonymous-normal-max-content-spaces",
    "anonymous-pre-wrap-max-content-spaces",
    "anonymous-pre-wrap-comment-split-max-content",
    "anonymous-pre-wrap-min-content-spaces",
    "anonymous-pre-wrap-soft-wrap",
    "anonymous-pre-wrap-forced-break",
    "anonymous-pre-wrap-whitespace-before-block",
    "anonymous-break-spaces-max-content",
    "anonymous-break-spaces-comment-split-max-content",
    "anonymous-break-spaces-min-content",
    "anonymous-break-spaces-soft-wrap-25",
    "anonymous-break-spaces-forced-break-max-content",
    "anonymous-break-spaces-forced-break-min-content",
    "anonymous-break-spaces-trailing-soft-wrap-25",
    "anonymous-break-spaces-trailing-max-content",
    "anonymous-break-spaces-whitespace-only",
    "anonymous-break-spaces-whitespace-before-block",
    "anonymous-pre-max-content-spaces",
    "anonymous-pre-min-content-spaces",
    "anonymous-pre-forced-break",
    "anonymous-normal-fixed-width-control",
    "anonymous-pre-fixed-width-nowrap",
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
        "anonymous-normal-newline-max-content" | "anonymous-pre-line-max-content" => {
            (20, "auto", "max-content", "auto", "wrap")
        }
        "anonymous-normal-max-content-spaces"
        | "anonymous-pre-wrap-max-content-spaces"
        | "anonymous-pre-wrap-comment-split-max-content"
        | "anonymous-pre-wrap-forced-break" => (20, "auto", "max-content", "auto", "wrap"),
        "anonymous-pre-wrap-min-content-spaces" => (20, "auto", "min-content", "auto", "wrap"),
        "anonymous-pre-max-content-spaces" | "anonymous-pre-forced-break" => {
            (20, "auto", "max-content", "auto", "nowrap")
        }
        "anonymous-pre-min-content-spaces" => (20, "auto", "min-content", "auto", "nowrap"),
        "anonymous-normal-fixed-width-control" => (25, "auto", "25px", "auto", "wrap"),
        "anonymous-pre-fixed-width-nowrap" => (25, "auto", "25px", "auto", "nowrap"),
        "anonymous-pre-wrap-soft-wrap" => (25, "auto", "25px", "auto", "wrap"),
        "anonymous-pre-wrap-whitespace-before-block" => (40, "10px", "20px 20px", "10px", "wrap"),
        "anonymous-break-spaces-max-content"
        | "anonymous-break-spaces-comment-split-max-content" => {
            (60, "auto", "max-content", "auto", "wrap")
        }
        "anonymous-break-spaces-min-content" => (30, "auto", "min-content", "auto", "wrap"),
        "anonymous-break-spaces-soft-wrap-25" | "anonymous-break-spaces-trailing-soft-wrap-25" => {
            (25, "auto", "25px", "auto", "wrap")
        }
        "anonymous-break-spaces-forced-break-max-content" => {
            (40, "auto", "max-content", "auto", "wrap")
        }
        "anonymous-break-spaces-forced-break-min-content" => {
            (30, "auto", "min-content", "auto", "wrap")
        }
        "anonymous-break-spaces-trailing-max-content" => {
            (40, "auto", "max-content", "auto", "wrap")
        }
        "anonymous-break-spaces-whitespace-only" => (0, "auto", "auto", "auto", "wrap"),
        "anonymous-break-spaces-whitespace-before-block" => {
            (40, "10px", "20px 20px", "10px", "wrap")
        }
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
    let white_space_collapse = match scene_id {
        "anonymous-pre-line-max-content" => "preserve-breaks",
        "anonymous-pre-wrap-max-content-spaces"
        | "anonymous-pre-wrap-comment-split-max-content"
        | "anonymous-pre-wrap-min-content-spaces"
        | "anonymous-pre-wrap-soft-wrap"
        | "anonymous-pre-wrap-forced-break"
        | "anonymous-pre-wrap-whitespace-before-block" => "preserve",
        "anonymous-pre-max-content-spaces"
        | "anonymous-pre-min-content-spaces"
        | "anonymous-pre-forced-break"
        | "anonymous-pre-fixed-width-nowrap" => "preserve",
        "anonymous-break-spaces-max-content"
        | "anonymous-break-spaces-comment-split-max-content"
        | "anonymous-break-spaces-min-content"
        | "anonymous-break-spaces-soft-wrap-25"
        | "anonymous-break-spaces-forced-break-max-content"
        | "anonymous-break-spaces-forced-break-min-content"
        | "anonymous-break-spaces-trailing-soft-wrap-25"
        | "anonymous-break-spaces-trailing-max-content"
        | "anonymous-break-spaces-whitespace-only"
        | "anonymous-break-spaces-whitespace-before-block" => "break-spaces",
        _ => "collapse",
    };
    let mut source = format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:{width}px;height:{height};margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{columns};grid-template-rows:{rows};font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:{white_space_collapse};text-wrap-mode:{wrap_mode};word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#101820');
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
        "anonymous-normal-newline-max-content" | "anonymous-pre-line-max-content" => {
            source.push_str("root.appendChild(document.createTextNode('XX\\nXX'));\n")
        }
        "anonymous-normal-max-content-spaces"
        | "anonymous-pre-wrap-max-content-spaces"
        | "anonymous-pre-wrap-min-content-spaces" => {
            source.push_str("root.appendChild(document.createTextNode('XX  XX'));\n")
        }
        "anonymous-pre-wrap-comment-split-max-content" => source.push_str(
            "root.appendChild(document.createTextNode('XX '));\nroot.appendChild(document.createTextNode(' XX'));\n",
        ),
        "anonymous-pre-wrap-soft-wrap" => {
            source.push_str("root.appendChild(document.createTextNode('XX XX'));\n")
        }
        "anonymous-pre-wrap-forced-break" => {
            source.push_str("root.appendChild(document.createTextNode('XX\\nXX'));\n")
        }
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
        "anonymous-pre-wrap-whitespace-before-block" => source.push_str(
            r#"root.appendChild(document.createTextNode('  '));
const block = document.createElement('div');
block.setAttribute('id', 'c114-anonymous-pre-wrap-whitespace-before-block-item');
block.setAttribute('data-node', '');
block.setAttribute('style', 'box-sizing:border-box;display:block;width:10px;height:10px;min-width:0;min-height:0;margin:0;padding:0;border:0;background-color:#2866f6');
root.appendChild(block);
nodeIds.push('c114-anonymous-pre-wrap-whitespace-before-block-item');
"#,
        ),
        "anonymous-pre-max-content-spaces" => {
            source.push_str("root.appendChild(document.createTextNode('XX  XX'));\n")
        }
        "anonymous-pre-min-content-spaces"
        | "anonymous-normal-fixed-width-control"
        | "anonymous-pre-fixed-width-nowrap" => {
            source.push_str("root.appendChild(document.createTextNode('XX XX'));\n")
        }
        "anonymous-pre-forced-break" => {
            source.push_str("root.appendChild(document.createTextNode('XX\\nXX'));\n")
        }
        "anonymous-break-spaces-max-content"
        | "anonymous-break-spaces-min-content"
        | "anonymous-break-spaces-soft-wrap-25" => {
            source.push_str("root.appendChild(document.createTextNode('XX  XX'));\n")
        }
        "anonymous-break-spaces-comment-split-max-content" => source.push_str(
            "root.appendChild(document.createTextNode('XX '));\nroot.appendChild(document.createTextNode(' XX'));\n",
        ),
        "anonymous-break-spaces-forced-break-max-content"
        | "anonymous-break-spaces-forced-break-min-content" => {
            source.push_str("root.appendChild(document.createTextNode('XX  \\nXX'));\n")
        }
        "anonymous-break-spaces-trailing-soft-wrap-25"
        | "anonymous-break-spaces-trailing-max-content" => {
            source.push_str("root.appendChild(document.createTextNode('XX  '));\n")
        }
        "anonymous-break-spaces-whitespace-only" => {
            source.push_str("root.appendChild(document.createTextNode('  '));\n")
        }
        "anonymous-break-spaces-whitespace-before-block" => source.push_str(
            r#"root.appendChild(document.createTextNode('  '));
const block = document.createElement('div');
block.setAttribute('id', 'c114-anonymous-break-spaces-whitespace-before-block-item');
block.setAttribute('data-node', '');
block.setAttribute('style', 'box-sizing:border-box;display:block;width:10px;height:10px;min-width:0;min-height:0;margin:0;padding:0;border:0;background-color:#2866f6');
root.appendChild(block);
nodeIds.push('c114-anonymous-break-spaces-whitespace-before-block-item');
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
    fn anonymous_grid_text_sources_are_a_fixed_thirty_one_scene_matrix() {
        let unique_ids = SCENE_IDS.iter().copied().collect::<BTreeSet<_>>();
        assert_eq!(SCENE_IDS.len(), 31);
        assert_eq!(unique_ids.len(), SCENE_IDS.len());
        for (scene_id, text_nodes, elements, width, height, wrap_mode, white_space_collapse) in [
            (
                "anonymous-min-content-wrap",
                1,
                0,
                120,
                "20px",
                "wrap",
                "collapse",
            ),
            (
                "anonymous-normal-min-content",
                1,
                0,
                20,
                "auto",
                "wrap",
                "collapse",
            ),
            (
                "anonymous-nowrap-min-content",
                1,
                0,
                20,
                "auto",
                "nowrap",
                "collapse",
            ),
            (
                "anonymous-normal-newline-max-content",
                1,
                0,
                20,
                "auto",
                "wrap",
                "collapse",
            ),
            (
                "anonymous-pre-line-max-content",
                1,
                0,
                20,
                "auto",
                "wrap",
                "preserve-breaks",
            ),
            (
                "anonymous-normal-max-content-spaces",
                1,
                0,
                20,
                "auto",
                "wrap",
                "collapse",
            ),
            (
                "anonymous-pre-wrap-max-content-spaces",
                1,
                0,
                20,
                "auto",
                "wrap",
                "preserve",
            ),
            (
                "anonymous-pre-wrap-comment-split-max-content",
                2,
                0,
                20,
                "auto",
                "wrap",
                "preserve",
            ),
            (
                "anonymous-pre-wrap-min-content-spaces",
                1,
                0,
                20,
                "auto",
                "wrap",
                "preserve",
            ),
            (
                "anonymous-pre-wrap-soft-wrap",
                1,
                0,
                25,
                "auto",
                "wrap",
                "preserve",
            ),
            (
                "anonymous-pre-wrap-forced-break",
                1,
                0,
                20,
                "auto",
                "wrap",
                "preserve",
            ),
            (
                "anonymous-comment-continuation",
                2,
                0,
                120,
                "10px",
                "wrap",
                "collapse",
            ),
            (
                "anonymous-runs-around-block",
                2,
                1,
                120,
                "10px",
                "wrap",
                "collapse",
            ),
            (
                "anonymous-hidden-child-continuation",
                3,
                0,
                120,
                "10px",
                "wrap",
                "collapse",
            ),
            (
                "whitespace-does-not-create-item",
                1,
                2,
                120,
                "10px",
                "wrap",
                "collapse",
            ),
            (
                "anonymous-pre-wrap-whitespace-before-block",
                1,
                1,
                40,
                "10px",
                "wrap",
                "preserve",
            ),
            (
                "anonymous-break-spaces-max-content",
                1,
                0,
                60,
                "auto",
                "wrap",
                "break-spaces",
            ),
            (
                "anonymous-break-spaces-comment-split-max-content",
                2,
                0,
                60,
                "auto",
                "wrap",
                "break-spaces",
            ),
            (
                "anonymous-break-spaces-min-content",
                1,
                0,
                30,
                "auto",
                "wrap",
                "break-spaces",
            ),
            (
                "anonymous-break-spaces-soft-wrap-25",
                1,
                0,
                25,
                "auto",
                "wrap",
                "break-spaces",
            ),
            (
                "anonymous-break-spaces-forced-break-max-content",
                1,
                0,
                40,
                "auto",
                "wrap",
                "break-spaces",
            ),
            (
                "anonymous-break-spaces-forced-break-min-content",
                1,
                0,
                30,
                "auto",
                "wrap",
                "break-spaces",
            ),
            (
                "anonymous-break-spaces-trailing-soft-wrap-25",
                1,
                0,
                25,
                "auto",
                "wrap",
                "break-spaces",
            ),
            (
                "anonymous-break-spaces-trailing-max-content",
                1,
                0,
                40,
                "auto",
                "wrap",
                "break-spaces",
            ),
            (
                "anonymous-break-spaces-whitespace-only",
                1,
                0,
                0,
                "auto",
                "wrap",
                "break-spaces",
            ),
            (
                "anonymous-break-spaces-whitespace-before-block",
                1,
                1,
                40,
                "10px",
                "wrap",
                "break-spaces",
            ),
            (
                "anonymous-pre-max-content-spaces",
                1,
                0,
                20,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-min-content-spaces",
                1,
                0,
                20,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-forced-break",
                1,
                0,
                20,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-normal-fixed-width-control",
                1,
                0,
                25,
                "auto",
                "wrap",
                "collapse",
            ),
            (
                "anonymous-pre-fixed-width-nowrap",
                1,
                0,
                25,
                "auto",
                "nowrap",
                "preserve",
            ),
        ] {
            let source = runtime_source(scene_id).expect("known C11.4 scene");
            assert!(source.len() < 4 * 1024);
            assert!(source.contains(&format!("sceneId:{scene_id:?}")));
            assert!(source.contains(&format!("display:grid;width:{width}px;height:{height};")));
            assert!(source.contains(&format!("text-wrap-mode:{wrap_mode}")));
            assert!(source.contains(&format!("white-space-collapse:{white_space_collapse}")));
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
