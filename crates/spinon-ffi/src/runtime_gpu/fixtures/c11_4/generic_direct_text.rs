#[derive(Clone, Copy)]
struct Item {
    id: &'static str,
    class: &'static str,
    text: &'static str,
    text_wrap_mode: &'static str,
    word_break: &'static str,
}

#[derive(Clone, Copy)]
struct Scene {
    route_id: &'static str,
    root_id: &'static str,
    root_class: &'static str,
    items: [Item; 2],
}

const SCENES: [Scene; 6] = [
    Scene {
        route_id: "generic-root-normal",
        root_id: "generic-root-normal",
        root_class: "scene",
        items: [
            Item {
                id: "generic-normal-a",
                class: "item",
                text: "A BC",
                text_wrap_mode: "wrap",
                word_break: "normal",
            },
            Item {
                id: "generic-normal-b",
                class: "item",
                text: "A BC",
                text_wrap_mode: "wrap",
                word_break: "normal",
            },
        ],
    },
    Scene {
        route_id: "generic-root-nowrap",
        root_id: "generic-root-nowrap",
        root_class: "scene nowrap",
        items: [
            Item {
                id: "generic-nowrap-a",
                class: "item",
                text: "A BC",
                text_wrap_mode: "nowrap",
                word_break: "normal",
            },
            Item {
                id: "generic-nowrap-b",
                class: "item",
                text: "A BC",
                text_wrap_mode: "nowrap",
                word_break: "normal",
            },
        ],
    },
    Scene {
        route_id: "generic-root-break-all",
        root_id: "generic-root-break-all",
        root_class: "scene break-all",
        items: [
            Item {
                id: "generic-break-a",
                class: "item",
                text: "ABCD",
                text_wrap_mode: "wrap",
                word_break: "break-all",
            },
            Item {
                id: "generic-break-b",
                class: "item",
                text: "ABCD",
                text_wrap_mode: "wrap",
                word_break: "break-all",
            },
        ],
    },
    Scene {
        route_id: "generic-root-content-variation",
        root_id: "grid-9x",
        root_class: "scene",
        items: [
            Item {
                id: "item-zeta",
                class: "item",
                text: "A BCD",
                text_wrap_mode: "wrap",
                word_break: "normal",
            },
            Item {
                id: "item-3",
                class: "item",
                text: "A BCD",
                text_wrap_mode: "wrap",
                word_break: "normal",
            },
        ],
    },
    Scene {
        route_id: "generic-root-whitespace",
        root_id: "generic-root-whitespace",
        root_class: "scene",
        items: [
            Item {
                id: "generic-ws-a",
                class: "item",
                text: " A  BC ",
                text_wrap_mode: "wrap",
                word_break: "normal",
            },
            Item {
                id: "generic-ws-b",
                class: "item",
                text: " A  BC ",
                text_wrap_mode: "wrap",
                word_break: "normal",
            },
        ],
    },
    Scene {
        route_id: "generic-root-reorder",
        root_id: "grid-reorder",
        root_class: "scene",
        items: [
            Item {
                id: "item-a",
                class: "item break-all",
                text: "ABCD",
                text_wrap_mode: "wrap",
                word_break: "break-all",
            },
            Item {
                id: "item-z",
                class: "item",
                text: "A BC",
                text_wrap_mode: "wrap",
                word_break: "normal",
            },
        ],
    },
];

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    let scene = SCENES.iter().find(|scene| scene.route_id == scene_id)?;
    let [first, second] = scene.items;
    Some(format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', '{}');
root.setAttribute('data-node', '');
root.setAttribute('class', '{}');
root.setAttribute('style', 'box-sizing:border-box;display:grid;position:static;width:100px;height:auto;margin:0;padding:0;border:0;grid-template-columns:min-content max-content;grid-template-rows:auto;grid-auto-flow:row;column-gap:10px;row-gap:0;align-items:start;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;background-color:#101820');
document.appendChild(root);
const first = document.createElement('div');
first.setAttribute('id', '{}');
first.setAttribute('data-node', '');
first.setAttribute('class', '{}');
first.setAttribute('style', 'box-sizing:border-box;display:block;position:static;width:auto;height:auto;min-width:auto;max-width:none;min-height:auto;max-height:none;margin:0;padding:0;border:0;font-family:inherit;font-size:inherit;font-weight:inherit;font-style:normal;line-height:inherit;white-space-collapse:collapse;text-wrap-mode:{};word-break:{};overflow-wrap:normal;background-color:#e5484d;color:#e5484d');
first.appendChild(document.createTextNode('{}'));
root.appendChild(first);
const second = document.createElement('div');
second.setAttribute('id', '{}');
second.setAttribute('data-node', '');
second.setAttribute('class', '{}');
second.setAttribute('style', 'box-sizing:border-box;display:block;position:static;width:auto;height:auto;min-width:auto;max-width:none;min-height:auto;max-height:none;margin:0;padding:0;border:0;font-family:inherit;font-size:inherit;font-weight:inherit;font-style:normal;line-height:inherit;white-space-collapse:collapse;text-wrap-mode:{};word-break:{};overflow-wrap:normal;background-color:#e5484d;color:#e5484d');
second.appendChild(document.createTextNode('{}'));
root.appendChild(second);
return {{ sceneId:'{}', rootId:'{}', nodeIds:['{}', '{}', '{}'] }};
}})()"#,
        scene.root_id,
        scene.root_class,
        first.id,
        first.class,
        first.text_wrap_mode,
        first.word_break,
        first.text,
        second.id,
        second.class,
        second.text_wrap_mode,
        second.word_break,
        second.text,
        scene.route_id,
        scene.root_id,
        scene.root_id,
        first.id,
        second.id,
    ))
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};

    #[test]
    fn exposes_the_six_canonical_generic_direct_text_scenes_only() {
        assert_eq!(SCENES.len(), 6);
        for scene in SCENES {
            let source = runtime_source(scene.route_id).expect("registered text scene");
            assert!(source.len() < 4 * 1024, "{}", scene.route_id);
            assert!(source.contains(&format!("sceneId:'{}'", scene.route_id)));
            assert!(source.contains(&format!("rootId:'{}'", scene.root_id)));
            assert!(source.contains(&format!("setAttribute('id', '{}')", scene.root_id)));
            assert!(source.contains("grid-template-columns:min-content max-content"));
            assert!(source.contains("column-gap:10px"));
            assert_eq!(source.matches("document.createElement('div')").count(), 3);
            assert_eq!(source.matches("document.createTextNode('").count(), 2);
            assert_eq!(
                source
                    .matches("max-width:none;min-height:auto;max-height:none")
                    .count(),
                2
            );
        }

        let whitespace = runtime_source("generic-root-whitespace").unwrap();
        assert_eq!(
            whitespace
                .matches("document.createTextNode(' A  BC ')")
                .count(),
            2
        );
        let reorder = runtime_source("generic-root-reorder").unwrap();
        assert!(reorder.contains("setAttribute('class', 'item break-all')"));
        assert!(reorder.contains("text-wrap-mode:wrap;word-break:break-all"));
        assert!(reorder.contains("document.createTextNode('A BC')"));
        let nowrap = runtime_source("generic-root-nowrap").unwrap();
        assert_eq!(nowrap.matches("text-wrap-mode:nowrap").count(), 2);
        assert!(runtime_source("unknown-generic-text-scene").is_none());
    }
}
