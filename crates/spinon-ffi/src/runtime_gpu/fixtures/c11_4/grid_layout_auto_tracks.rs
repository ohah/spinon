const SCENE_ID: &str = "grid-layout-auto-tracks-c114-wpt";
const ROOT_ID: &str = "c114-grid-layout-auto-tracks-grid";

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    (scene_id == SCENE_ID).then(|| {
        format!(
            r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {ROOT_ID:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:100px;height:auto;margin:0;padding:0;border:0;column-gap:0;row-gap:0;align-items:start;grid-template-columns:100px;grid-template-rows:auto;grid-auto-columns:auto;grid-auto-rows:auto;grid-auto-flow:row;direction:ltr;writing-mode:horizontal-tb;background-color:#17212b');
document.appendChild(root);
const a = document.createElement('div');
a.setAttribute('id', 'c114-grid-layout-auto-tracks-a');
a.setAttribute('data-node', '');
a.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;grid-row:1 / 2;grid-column:1 / 2;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;writing-mode:horizontal-tb;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d');
a.appendChild(document.createTextNode('\u00a0'));
root.appendChild(a);
const b = document.createElement('div');
b.setAttribute('id', 'c114-grid-layout-auto-tracks-b');
b.setAttribute('data-node', '');
b.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;grid-row:1 / 2;grid-column:2 / 3;background-color:#3a86ff');
const bContent = document.createElement('div');
bContent.setAttribute('id', 'c114-grid-layout-auto-tracks-b-content');
bContent.setAttribute('data-node', '');
bContent.setAttribute('style', 'box-sizing:border-box;display:block;width:50px;height:auto;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;writing-mode:horizontal-tb;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#06d6a0');
bContent.appendChild(document.createTextNode('\u00a0'));
b.appendChild(bContent);
root.appendChild(b);
const c = document.createElement('div');
c.setAttribute('id', 'c114-grid-layout-auto-tracks-c');
c.setAttribute('data-node', '');
c.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;grid-row:2 / 3;grid-column:1 / 2;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;writing-mode:horizontal-tb;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#ffd166');
c.appendChild(document.createTextNode('\u00a0'));
root.appendChild(c);
return {{ sceneId:{SCENE_ID:?}, rootId:{ROOT_ID:?}, nodeIds:[{ROOT_ID:?}, 'c114-grid-layout-auto-tracks-a', 'c114-grid-layout-auto-tracks-b', 'c114-grid-layout-auto-tracks-b-content', 'c114-grid-layout-auto-tracks-c'] }};
}})()"#,
        )
    })
}

#[cfg(test)]
mod tests {
    use super::{ROOT_ID, SCENE_ID, runtime_source};
    use crate::runtime_gpu::fixtures::c11_4::c11_4_grid_track_sizing_runtime_source;

    #[test]
    fn exposes_only_the_pinned_auto_tracks_scene() {
        let source = runtime_source(SCENE_ID).expect("allowlisted WPT scene");
        assert!(source.len() < 8 * 1024);
        assert!(source.contains("grid-template-columns:100px;grid-template-rows:auto"));
        assert!(source.contains("grid-auto-columns:auto;grid-auto-rows:auto;grid-auto-flow:row"));
        assert!(source.contains("grid-row:1 / 2;grid-column:2 / 3"));
        assert!(source.contains("grid-row:2 / 3;grid-column:1 / 2"));
        assert_eq!(source.matches("document.createElement('div')").count(), 5);
        assert_eq!(source.matches("document.createTextNode(").count(), 3);
        assert!(source.contains("document.createTextNode('\\u00a0')"));
        assert!(source.contains(ROOT_ID));
        assert!(runtime_source("grid-layout-auto-tracks-c114-wpt;throw 1").is_none());
        assert!(c11_4_grid_track_sizing_runtime_source(SCENE_ID).is_ok());
        assert!(
            c11_4_grid_track_sizing_runtime_source("grid-layout-auto-tracks-unlisted").is_err()
        );
    }
}
