use serde_json::Value;

const CASES_JSON: &str = include_str!("grid_auto_fit_columns_wpt_cases.json");

fn cases() -> Option<Value> {
    serde_json::from_str(CASES_JSON).ok()
}

fn js_string(value: &str) -> Option<String> {
    serde_json::to_string(value).ok()
}

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    let cases = cases()?;
    let scenes = cases.get("scenes")?.as_array()?;
    let scene = scenes
        .iter()
        .find(|scene| scene.get("sceneId").and_then(Value::as_str) == Some(scene_id))?;
    let root_id = scene.get("rootId")?.as_str()?;
    let root_style = scene.get("rootStyle")?.as_str()?;
    let wpt_index = scene.get("wptIndex")?.as_u64()?;
    let root_id_js = js_string(root_id)?;
    let root_style_js = js_string(root_style)?;
    let scene_id_js = js_string(scene_id)?;
    let items = scene.get("items")?.as_array()?;
    if items.is_empty() {
        return None;
    }

    let mut node_ids = vec![root_id_js.clone()];
    let mut item_ids = Vec::with_capacity(items.len());
    let mut source = format!(
        "(() => {{\nconst root = document.createElement('div');\nroot.setAttribute('id', {root_id_js});\nroot.setAttribute('data-node', '');\nroot.setAttribute('style', {root_style_js});\ndocument.appendChild(root);\nconst nodeIds = [{root_id_js}];\nconst itemIds = [];\n"
    );

    for item in items {
        let item_id = item.get("id")?.as_str()?;
        let item_style = item.get("style")?.as_str()?;
        let item_id_js = js_string(item_id)?;
        let item_style_js = js_string(item_style)?;
        let ordinal = item_ids.len() + 1;
        source.push_str(&format!(
            "const item{ordinal} = document.createElement('div');\nitem{ordinal}.setAttribute('id', {item_id_js});\nitem{ordinal}.setAttribute('data-node', '');\nitem{ordinal}.setAttribute('style', {item_style_js});\nroot.appendChild(item{ordinal});\nnodeIds.push({item_id_js});\nitemIds.push({item_id_js});\n"
        ));
        node_ids.push(item_id_js.clone());
        item_ids.push(item_id_js);
    }

    source.push_str(&format!(
        "return {{ sceneId:{scene_id_js}, wptIndex:{wpt_index}, rootId:{root_id_js}, nodeIds:[{}], itemIds:[{}] }};\n}})()",
        node_ids.join(","),
        item_ids.join(","),
    ));
    Some(source)
}

pub(crate) fn expected_report(scene_id: &str) -> Option<String> {
    let cases = cases()?;
    let scene = cases
        .get("scenes")?
        .as_array()?
        .iter()
        .find(|scene| scene.get("sceneId").and_then(Value::as_str) == Some(scene_id))?;
    let repeat = scene.get("repeatExpectation")?;
    let used_tracks = scene.get("usedTrackObservation")?;
    let items = scene.get("items")?.as_array()?;
    let report = serde_json::json!({
        "sceneId": scene_id,
        "wptIndex": scene.get("wptIndex")?,
        "expectedRootFrame": scene.get("expectedRootFrame")?,
        "rootStyle": scene.get("rootStyle")?,
        "repeat": {
            "authoredTrackList": repeat.get("authoredGridTemplateColumns")?,
            "mode": repeat.get("repeatMode")?,
            "expectedCount": repeat.get("expectedRepeatCount")?,
            "prefixTrackCount": repeat.get("prefixTrackCount")?,
            "patternTrackCount": repeat.get("repeatedPatternCountSizesPx")?.as_array()?.len(),
            "suffixTrackCount": repeat.get("suffixTrackCount")?,
            "definiteContentBasisPx": repeat.get("definiteContentBasisPx")?,
            "columnGapPx": repeat.get("columnGapPx")?,
            "selectedCountRequiredSizePx": repeat.get("requiredSizeAtSelectedCountPx")?,
            "nextCountRequiredSizePx": repeat.get("requiredSizeAtNextCountPx")?,
        },
        "usedTrackObservation": {
            "source": used_tracks.get("source")?,
            "gridTemplateColumns": used_tracks.get("gridTemplateColumns")?,
            "gridTemplateRows": used_tracks.get("gridTemplateRows")?,
            "columnGap": used_tracks.get("columnGap")?,
            "rowGap": used_tracks.get("rowGap")?,
            "explicitColumnTrackCount": used_tracks.get("explicitColumnTrackCount")?,
            "negativeImplicitColumnTrackCount": used_tracks.get("negativeImplicitColumnTrackCount")?,
            "positiveImplicitColumnTrackCount": used_tracks.get("positiveImplicitColumnTrackCount")?,
            "explicitRowTrackCount": used_tracks.get("explicitRowTrackCount")?,
            "negativeImplicitRowTrackCount": used_tracks.get("negativeImplicitRowTrackCount")?,
            "positiveImplicitRowTrackCount": used_tracks.get("positiveImplicitRowTrackCount")?,
        },
        "items": items.iter().map(|item| {
            serde_json::json!({
                "id": item.get("id"),
                "authoredStyle": item.get("authoredStyle"),
                "expected": item.get("expected"),
            })
        }).collect::<Vec<_>>(),
    });
    serde_json::to_string(&report).ok()
}

#[cfg(test)]
mod tests {
    use super::{CASES_JSON, runtime_source};
    use crate::runtime_gpu::fixtures::c11_4::c11_4_grid_track_sizing_runtime_source;
    use serde_json::Value;
    use std::collections::BTreeSet;

    #[test]
    fn registers_all_and_only_the_pinned_wpt_roots_and_direct_items() {
        let cases: Value = serde_json::from_str(CASES_JSON).expect("fixture JSON is valid");
        assert_eq!(
            cases["source"]["wptRevision"],
            "c999c58338ee1d223df5ad62e48be1202ced0d37"
        );
        assert_eq!(cases["caseCount"], 35);
        assert_eq!(cases["itemCount"], 37);

        let scenes = cases["scenes"].as_array().expect("scene list");
        assert_eq!(scenes.len(), 35);
        let mut ids = BTreeSet::new();
        let mut item_count = 0;
        for (index, scene) in scenes.iter().enumerate() {
            let expected_id = format!("grid-auto-fit-columns-{:02}", index + 1);
            let scene_id = scene["sceneId"].as_str().expect("scene ID");
            assert_eq!(scene_id, expected_id);
            assert!(ids.insert(scene_id));
            assert_eq!(scene["wptIndex"], index + 1);

            let items = scene["items"].as_array().expect("direct items");
            item_count += items.len();
            let source = runtime_source(scene_id).expect("pinned scene is allowlisted");
            assert_eq!(
                source.matches("document.createElement('div')").count(),
                items.len() + 1
            );
            assert_eq!(source.matches("root.appendChild(item").count(), items.len());
            assert!(source.contains("wptIndex:"));
            assert!(c11_4_grid_track_sizing_runtime_source(scene_id).is_ok());

            for item in items {
                let expected = &item["expected"];
                for key in ["x", "y", "width", "height"] {
                    assert!(expected[key].is_number(), "{scene_id} expected {key}");
                }
                assert!(item["style"].as_str().is_some());
            }
        }
        assert_eq!(item_count, 37);
        assert!(runtime_source("grid-auto-fit-columns-00").is_none());
        assert!(runtime_source("grid-auto-fit-columns-36").is_none());
        assert!(runtime_source("grid-auto-fit-columns-01;throw 1").is_none());
        assert!(c11_4_grid_track_sizing_runtime_source("grid-auto-fit-columns-36").is_err());
    }

    #[test]
    fn preserves_authored_track_lists_placement_and_wpt_geometry() {
        let cases: Value = serde_json::from_str(CASES_JSON).expect("fixture JSON is valid");
        let scenes = cases["scenes"].as_array().expect("scene list");
        for scene in scenes {
            let scene_id = scene["sceneId"].as_str().expect("scene ID");
            let source = runtime_source(scene_id).expect("pinned scene");
            let root_style = scene["rootStyle"].as_str().expect("root CSS");
            assert!(source.contains(root_style));
            assert!(source.contains("grid-template-columns:"), "{scene_id}");
            assert!(source.contains("grid-auto-columns: 157px"), "{scene_id}");
            assert!(source.contains("grid-auto-rows: 25px"), "{scene_id}");
            for item in scene["items"].as_array().expect("items") {
                let item_style = item["style"].as_str().expect("item CSS");
                assert!(source.contains(item_style), "{scene_id}: {item_style}");
            }
        }

        let first = runtime_source("grid-auto-fit-columns-01").expect("first case");
        assert!(first.contains("repeat(auto-fit, 30px [autobar])"));
        assert!(first.contains("grid-column: 1 / -1"));
        let last = runtime_source("grid-auto-fit-columns-35").expect("last case");
        assert!(last.contains("grid-column-start: 1"));
        assert!(last.contains("grid-column-start: 2"));
        assert!(last.contains("background-color: green"));
    }

    #[test]
    fn emits_independent_geometry_and_repeat_count_expectations_for_each_scene() {
        for index in 1..=35 {
            let scene_id = format!("grid-auto-fit-columns-{index:02}");
            let expected: Value = serde_json::from_str(
                &super::expected_report(&scene_id).expect("allowlisted scene expectation"),
            )
            .expect("serialized expectation");
            assert_eq!(expected["sceneId"], scene_id);
            assert_eq!(expected["wptIndex"], index);
            assert!(expected["expectedRootFrame"]["width"].is_number());
            assert!(expected["expectedRootFrame"]["height"].is_number());
            assert!(expected["repeat"]["expectedCount"].is_number());
            assert_eq!(
                expected["items"].as_array().expect("items").len(),
                if index == 32 || index == 35 { 2 } else { 1 }
            );
        }
        assert!(super::expected_report("grid-auto-fit-columns-36").is_none());
    }
}
