const SCENE_ID: &str = "generic-root-normal";

const SOURCE: &str = r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'generic-root-normal');
root.setAttribute('data-node', '');
root.setAttribute('class', 'scene');
root.setAttribute('style', 'box-sizing:border-box;display:grid;position:static;width:100px;height:auto;margin:0;padding:0;border:0;grid-template-columns:min-content max-content;grid-template-rows:auto;grid-auto-flow:row;column-gap:10px;row-gap:0;align-items:start;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;background-color:#101820');
document.appendChild(root);
const first = document.createElement('div');
first.setAttribute('id', 'generic-normal-a');
first.setAttribute('data-node', '');
first.setAttribute('class', 'item');
first.setAttribute('style', 'box-sizing:border-box;display:block;position:static;width:auto;height:auto;min-width:auto;max-width:none;min-height:auto;max-height:none;margin:0;padding:0;border:0;font-family:inherit;font-size:inherit;font-weight:inherit;font-style:normal;line-height:inherit;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;overflow-wrap:normal;background-color:#e5484d;color:#e5484d');
first.appendChild(document.createTextNode('A BC'));
root.appendChild(first);
const second = document.createElement('div');
second.setAttribute('id', 'generic-normal-b');
second.setAttribute('data-node', '');
second.setAttribute('class', 'item');
second.setAttribute('style', 'box-sizing:border-box;display:block;position:static;width:auto;height:auto;min-width:auto;max-width:none;min-height:auto;max-height:none;margin:0;padding:0;border:0;font-family:inherit;font-size:inherit;font-weight:inherit;font-style:normal;line-height:inherit;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;overflow-wrap:normal;background-color:#e5484d;color:#e5484d');
second.appendChild(document.createTextNode('A BC'));
root.appendChild(second);
return { sceneId:'generic-root-normal', rootId:'generic-root-normal', nodeIds:['generic-root-normal', 'generic-normal-a', 'generic-normal-b'] };
})()"#;

pub(crate) fn runtime_source(scene_id: &str) -> Option<&'static str> {
    (scene_id == SCENE_ID).then_some(SOURCE)
}

#[cfg(test)]
mod tests {
    use super::runtime_source;

    #[test]
    fn exposes_only_the_canonical_generic_direct_text_scene() {
        let source = runtime_source("generic-root-normal").expect("canonical text scene");
        assert!(source.len() < 4 * 1024);
        assert!(source.contains("sceneId:'generic-root-normal'"));
        assert!(source.contains("rootId:'generic-root-normal'"));
        assert!(source.contains("generic-normal-a"));
        assert!(source.contains("generic-normal-b"));
        assert!(source.contains("grid-template-columns:min-content max-content"));
        assert!(source.contains("column-gap:10px"));
        assert_eq!(
            source
                .matches("max-width:none;min-height:auto;max-height:none")
                .count(),
            2
        );
        assert!(!source.contains("align-content:start"));
        assert!(!source.contains("justify-content:start"));
        assert_eq!(source.matches("document.createElement('div')").count(), 3);
        assert_eq!(source.matches("document.createTextNode('A BC')").count(), 2);
        assert!(runtime_source("generic-root-nowrap").is_none());
        assert!(runtime_source("generic-root-break-all").is_none());
    }
}
