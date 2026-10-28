const SCENES: [(&str, &str, &str); 5] = [
    (
        "columns-span2-visible-content-minimum",
        "c114-auto-minmax-span-overflow-visible",
        "visible",
    ),
    (
        "columns-span2-clip-content-minimum",
        "c114-auto-minmax-span-overflow-clip",
        "clip",
    ),
    (
        "columns-span2-hidden-zero-minimum",
        "c114-auto-minmax-span-overflow-hidden",
        "hidden",
    ),
    (
        "columns-span2-auto-zero-minimum",
        "c114-auto-minmax-span-overflow-auto",
        "auto",
    ),
    (
        "columns-span2-scroll-zero-minimum",
        "c114-auto-minmax-span-overflow-scroll",
        "scroll",
    ),
];

const SINGLE_TRACK_SCENES: [&str; 14] = [
    "column-visible-content-minimum",
    "column-clip-content-minimum",
    "column-hidden-zero-minimum",
    "column-auto-zero-minimum",
    "column-scroll-zero-minimum",
    "column-specified-minimum-control",
    "column-zero-track-control",
    "row-visible-content-minimum",
    "row-clip-content-minimum",
    "row-hidden-zero-minimum",
    "row-auto-zero-minimum",
    "row-scroll-zero-minimum",
    "row-specified-minimum-control",
    "row-zero-track-control",
];

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    if let Some((scene_id, root_id, overflow_x)) = SCENES
        .iter()
        .copied()
        .find(|(known_scene_id, _, _)| *known_scene_id == scene_id)
    {
        return Some(format!(
            r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', '{root_id}');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:80px;height:20px;margin:0;padding:0;border:0;column-gap:0;row-gap:0;grid-template-columns:minmax(auto,max-content) minmax(auto,max-content);grid-template-rows:20px');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-auto-minmax-span-overflow-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;margin:0;padding:0;border:0;background-color:#e5484d;grid-column:1 / span 2;grid-row:1;font-family:Ahem;font-size:10px;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;overflow-x:{overflow_x}');
item.appendChild(document.createTextNode('XXXXXXXXXX'));
root.appendChild(item);
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds:[{root_id:?}, 'c114-auto-minmax-span-overflow-item'] }};
}})()"#
        ));
    }
    single_track_runtime_source(scene_id)
}

pub(super) fn single_track_runtime_source(scene_id: &str) -> Option<String> {
    if !SINGLE_TRACK_SCENES.contains(&scene_id) {
        return None;
    }
    let is_columns = scene_id.starts_with("column-");
    let zero_item_minimum = scene_id.contains("specified-minimum-control");
    let zero_track_minimum = scene_id.contains("zero-track-control");
    let target_overflow = if scene_id.contains("-clip-") {
        "clip"
    } else if scene_id.contains("-hidden-") {
        "hidden"
    } else if scene_id.contains("-auto-") {
        "auto"
    } else if scene_id.contains("-scroll-") {
        "scroll"
    } else {
        "visible"
    };
    let (overflow_x, overflow_y) = if is_columns {
        (target_overflow, "visible")
    } else {
        ("visible", target_overflow)
    };
    let root_id = format!("c114-auto-min-scrollable-overflow-{scene_id}");
    let (width, height, columns, rows) = if is_columns {
        (
            "60px",
            "20px",
            if zero_track_minimum {
                "minmax(0,1fr)"
            } else {
                "minmax(auto,1fr)"
            },
            "20px",
        )
    } else {
        (
            "30px",
            "5px",
            "10px",
            if zero_track_minimum {
                "minmax(0,1fr)"
            } else {
                "minmax(auto,1fr)"
            },
        )
    };
    let item_dimensions = if is_columns {
        format!(
            "width:auto;height:auto;min-width:{};min-height:auto;",
            if zero_item_minimum { "0" } else { "auto" }
        )
    } else {
        format!(
            "width:10px;height:auto;min-width:auto;min-height:{};",
            if zero_item_minimum { "0" } else { "auto" }
        )
    };
    let item_style = format!(
        "box-sizing:border-box;display:block;{item_dimensions}margin:0;padding:0;border:0;grid-column:1 / 2;grid-row:1 / 2;background-color:#e5484d;{text_style}overflow-x:{overflow_x};overflow-y:{overflow_y}",
        text_style = if is_columns {
            "font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;"
        } else {
            ""
        },
    );
    let root_style = format!(
        "box-sizing:border-box;display:grid;width:{width};height:{height};margin:0;padding:0;border:0;column-gap:0;row-gap:0;align-items:stretch;align-content:start;justify-content:start;grid-template-columns:{columns};grid-template-rows:{rows}"
    );
    let item_source = if is_columns {
        format!(
            r#"const item = document.createElement('div');
item.setAttribute('id', 'c114-auto-min-scrollable-overflow-item');
item.setAttribute('data-node', '');
item.setAttribute('style', {item_style:?});
item.appendChild(document.createTextNode('XXXXXXXXXX'));
root.appendChild(item);
"#
        )
    } else {
        format!(
            r#"const item = document.createElement('img');
item.setAttribute('id', 'c114-auto-min-scrollable-overflow-item');
item.setAttribute('data-node', '');
item.setAttribute('src', 'https://spinon.invalid/c11-4-overflow-auto-minimum-10x10.png');
item.setAttribute('style', {item_style:?});
root.appendChild(item);
"#
        )
    };
    Some(format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', {root_style:?});
document.appendChild(root);
{item_source}
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds:[{root_id:?}, 'c114-auto-min-scrollable-overflow-item'] }};
}})()"#,
        scene_id = scene_id,
    ))
}

#[cfg(test)]
mod tests {
    use super::{SCENES, SINGLE_TRACK_SCENES, runtime_source, single_track_runtime_source};

    #[test]
    fn exposes_only_the_five_fixed_overflow_automatic_minimum_scenes() {
        for (scene_id, root_id, overflow_x) in SCENES {
            let source = runtime_source(scene_id).expect("known fixed scene");
            assert!(source.contains(&format!("sceneId:{scene_id:?}")));
            assert!(source.contains(&format!("id', '{root_id}'")));
            assert!(source.contains(&format!("overflow-x:{overflow_x}")));
            assert!(source.contains("background-color:#e5484d"));
            assert!(source.contains(
                "grid-template-columns:minmax(auto,max-content) minmax(auto,max-content)"
            ));
            assert!(source.contains("grid-column:1 / span 2"));
            assert!(source.contains("XXXXXXXXXX"));
        }
        assert!(runtime_source("columns-span2-hidden-zero-minimun").is_none());
        assert!(runtime_source("unlisted-overflow-auto-minimum").is_none());
    }

    #[test]
    fn exposes_the_fourteen_single_track_axis_scenes() {
        assert_eq!(SINGLE_TRACK_SCENES.len(), 14);
        for scene_id in SINGLE_TRACK_SCENES {
            let source = single_track_runtime_source(scene_id).expect("fixed single-track scene");
            assert!(source.contains(&format!("sceneId:{scene_id:?}")));
            assert!(source.contains("grid-column:1 / 2;grid-row:1 / 2"));
            assert!(source.contains("overflow-x:"));
            assert!(source.contains("overflow-y:"));
            assert!(source.contains("align-items:stretch"));
            assert!(source.contains("grid-template-columns:"));
            assert!(source.contains("grid-template-rows:"));
            if scene_id.starts_with("column-") {
                assert!(source.contains("XXXXXXXXXX"));
                if scene_id.contains("zero-track-control") {
                    assert!(source.contains("grid-template-columns:minmax(0,1fr)"));
                } else {
                    assert!(source.contains("grid-template-columns:minmax(auto,1fr)"));
                }
            } else {
                assert!(source.contains("c11-4-overflow-auto-minimum-10x10.png"));
            }
        }
        assert!(single_track_runtime_source("column-unlisted").is_none());
    }
}
