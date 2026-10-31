const SCENES: [(&str, &str, &str); 10] = [
    ("font-size-5-min-content", "5px", "min-content"),
    ("font-size-5-max-content", "5px", "max-content"),
    ("font-size-7-5-min-content", "7.5px", "min-content"),
    ("font-size-7-5-max-content", "7.5px", "max-content"),
    ("font-size-10-min-content", "10px", "min-content"),
    ("font-size-10-max-content", "10px", "max-content"),
    ("font-size-12-5-min-content", "12.5px", "min-content"),
    ("font-size-12-5-max-content", "12.5px", "max-content"),
    ("font-size-20-min-content", "20px", "min-content"),
    ("font-size-20-max-content", "20px", "max-content"),
];

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    let (_, font_size, column) = SCENES
        .iter()
        .find(|(known_id, _, _)| *known_id == scene_id)?;
    let root_id = format!("c114-{scene_id}-grid");
    Some(format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:20px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{column};grid-template-rows:auto;font-family:Ahem;font-size:{font_size};font-weight:400;font-style:normal;line-height:10px;tab-size:8;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#101820');
document.appendChild(root);
const nodeIds = [{root_id:?}];
root.appendChild(document.createTextNode('XX XX'));
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds }};
}})()"#
    ))
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};
    use std::collections::BTreeSet;

    #[test]
    fn computed_font_size_matrix_generates_ten_exact_direct_text_scenes() {
        let mut unique = BTreeSet::new();
        for (scene_id, font_size, column) in SCENES {
            assert!(unique.insert(scene_id), "duplicate {scene_id}");
            let source = runtime_source(scene_id).expect("listed computed font-size scene");
            assert!(source.contains(&format!("sceneId:{scene_id:?}")));
            assert!(source.contains(&format!("font-size:{font_size};")));
            assert!(source.contains(&format!("grid-template-columns:{column};")));
            assert!(source.contains("grid-template-rows:auto"));
            assert!(source.contains("display:grid;width:20px;height:auto"));
            assert!(source.contains("line-height:10px"));
            assert!(source.contains("white-space-collapse:collapse;text-wrap-mode:wrap"));
            assert!(source.contains("document.createTextNode('XX XX')"));
        }
        assert_eq!(unique.len(), 10);
        assert!(runtime_source("font-size-6-min-content").is_none());
    }
}
