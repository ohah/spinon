#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    root_id: &'static str,
    align_items: &'static str,
    small_align_self: &'static str,
    large_align_self: &'static str,
    reversed: bool,
}

const SCENES: [Scene; 4] = [
    Scene {
        id: "grid-text-first-baseline-align-items",
        root_id: "c114-grid-text-first-baseline-align-items",
        align_items: "first baseline",
        small_align_self: "auto",
        large_align_self: "auto",
        reversed: false,
    },
    Scene {
        id: "grid-text-first-baseline-start-control",
        root_id: "c114-grid-text-first-baseline-start-control",
        align_items: "start",
        small_align_self: "auto",
        large_align_self: "auto",
        reversed: false,
    },
    Scene {
        id: "grid-text-first-baseline-align-self",
        root_id: "c114-grid-text-first-baseline-align-self",
        align_items: "start",
        small_align_self: "first baseline",
        large_align_self: "first baseline",
        reversed: false,
    },
    Scene {
        id: "grid-text-first-baseline-reversed",
        root_id: "c114-grid-text-first-baseline-reversed",
        align_items: "first baseline",
        small_align_self: "auto",
        large_align_self: "auto",
        reversed: true,
    },
];

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    let scene = SCENES.iter().find(|scene| scene.id == scene_id)?;
    let first = if scene.reversed { "large" } else { "small" };
    let second = if scene.reversed { "small" } else { "large" };
    Some(format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;position:static;width:max-content;height:auto;margin:0;padding:0;border:0;grid-template-columns:auto auto;grid-template-rows:auto;grid-auto-flow:row;column-gap:8px;row-gap:0;align-items:{align_items};align-content:normal;justify-content:normal;justify-items:normal;direction:ltr;writing-mode:horizontal-tb;background-color:#eef3f8');
document.appendChild(root);
const small = document.createElement('div');
small.setAttribute('id', {small_id:?});
small.setAttribute('data-node', '');
small.setAttribute('style', 'box-sizing:border-box;display:block;position:static;width:auto;height:auto;min-width:auto;max-width:none;min-height:auto;max-height:none;margin:0;padding:0;border:0;align-self:{small_align_self};justify-self:auto;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:nowrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;writing-mode:horizontal-tb;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d;color:#e5484d');
small.appendChild(document.createTextNode('XX'));
const large = document.createElement('div');
large.setAttribute('id', {large_id:?});
large.setAttribute('data-node', '');
large.setAttribute('style', 'box-sizing:border-box;display:block;position:static;width:auto;height:auto;min-width:auto;max-width:none;min-height:auto;max-height:none;margin:0;padding:0;border:0;align-self:{large_align_self};justify-self:auto;font-family:Ahem;font-size:20px;font-weight:400;font-style:normal;line-height:20px;white-space-collapse:collapse;text-wrap-mode:nowrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;writing-mode:horizontal-tb;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#3a86ff;color:#3a86ff');
large.appendChild(document.createTextNode('XX'));
root.appendChild({first});
root.appendChild({second});
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds:[{root_id:?}, {small_id:?}, {large_id:?}] }};
}})()"#,
        root_id = scene.root_id,
        align_items = scene.align_items,
        small_id = format!("{}-small", scene.root_id),
        large_id = format!("{}-large", scene.root_id),
        small_align_self = scene.small_align_self,
        large_align_self = scene.large_align_self,
        first = first,
        second = second,
        scene_id = scene.id,
    ))
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};
    use crate::runtime_gpu::fixtures::c11_4::c11_4_grid_track_sizing_runtime_source;

    #[test]
    fn exposes_the_four_pinned_first_baseline_scenes_only() {
        assert_eq!(SCENES.len(), 4);
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("registered baseline scene");
            assert!(source.len() < 5 * 1024, "{}", scene.id);
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains(&format!("setAttribute('id', {:?})", scene.root_id)));
            assert!(source.contains("grid-template-columns:auto auto;grid-template-rows:auto"));
            assert!(source.contains("column-gap:8px;row-gap:0"));
            assert_eq!(source.matches("document.createElement('div')").count(), 3);
            assert_eq!(source.matches("document.createTextNode('XX')").count(), 2);
            assert!(c11_4_grid_track_sizing_runtime_source(scene.id).is_ok());
        }
        assert!(runtime_source("grid-text-first-baseline-unlisted").is_none());
        assert!(
            c11_4_grid_track_sizing_runtime_source("grid-text-first-baseline-unlisted").is_err()
        );
    }
}
