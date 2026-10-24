const SCENES_JSON: &str = include_str!("auto_repeat_fixed_repeat_scenes.json");

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    let scenes: serde_json::Value = serde_json::from_str(SCENES_JSON).ok()?;
    let scene = scenes.as_array()?.iter().find(|scene| {
        scene
            .get("id")
            .and_then(serde_json::Value::as_str)
            .is_some_and(|id| id == scene_id)
    })?;
    let scene_json = serde_json::to_string(scene).ok()?;
    Some(format!(
        r#"(() => {{
const scene = JSON.parse({scene_json:?});
const rootId = `c114-${{scene.id}}-grid`;
const root = document.createElement('div');
root.setAttribute('id', rootId);
root.setAttribute('data-node', '');
root.setAttribute('style', [
  'box-sizing:border-box', 'display:grid', `width:${{scene.width}}`,
  `height:${{scene.height}}`, 'min-width:0', 'min-height:0',
  'margin:0', 'padding:0', 'border:0', 'align-content:start', 'justify-content:start',
  `column-gap:${{scene.columnGap}}`, `row-gap:${{scene.rowGap}}`,
  `grid-template-columns:${{scene.columns}}`, `grid-template-rows:${{scene.rows}}`,
  ...(scene.autoColumns ? [`grid-auto-columns:${{scene.autoColumns}}`] : []),
  ...(scene.autoRows ? [`grid-auto-rows:${{scene.autoRows}}`] : []),
  ...(scene.areas ? [`grid-template-areas:${{scene.areas}}`] : []),
  ...(scene.autoFlow ? [`grid-auto-flow:${{scene.autoFlow}}`] : []),
  `background-color:${{scene.rootColor}}`,
].join(';'));
document.appendChild(root);
const itemIds = scene.items.map((itemSpec) => {{
  const item = document.createElement('div');
  const itemId = `c114-${{scene.id}}-${{itemSpec.name}}`;
  item.setAttribute('id', itemId);
  item.setAttribute('data-node', '');
  const placement = itemSpec.placement ?? {{}};
  const placementStyles = Object.entries(placement).map(([property, value]) => {{
    const kebabProperty = property.replace(/[A-Z]/g, (letter) => `-${{letter.toLowerCase()}}`);
    const cssProperty = property === 'gridArea'
      ? 'grid-area'
      : property.startsWith('grid') ? kebabProperty : `grid-${{kebabProperty}}`;
    return `${{cssProperty}}:${{value}}`;
  }});
  item.setAttribute('style', [
    'box-sizing:border-box', 'display:block',
    `width:${{itemSpec.width ?? 'auto'}}`, `height:${{itemSpec.height ?? 'auto'}}`,
    'min-width:0', 'min-height:0', 'margin:0', 'padding:0', 'border:0',
    ...placementStyles,
    `background-color:${{itemSpec.color}}`,
  ].join(';'));
  root.appendChild(item);
  return itemId;
}});
return {{ sceneId:scene.id, rootId, nodeIds:[rootId, ...itemIds] }};
}})()"#
    ))
}

#[cfg(test)]
mod tests {
    use super::{SCENES_JSON, runtime_source};
    use serde_json::Value;
    use std::collections::BTreeSet;

    fn css_rule_has_declaration(html: &str, selector: &str, declaration: &str) -> bool {
        let Some(style) = html
            .split_once("<style>")
            .and_then(|(_, remainder)| remainder.split_once("</style>"))
            .map(|(style, _)| style)
        else {
            return false;
        };

        style.split('}').any(|rule| {
            let Some((selectors, declarations)) = rule.rsplit_once('{') else {
                return false;
            };
            selectors
                .split(',')
                .any(|candidate| candidate.trim() == selector)
                && declarations
                    .split(';')
                    .any(|candidate| candidate.trim() == declaration)
        })
    }

    #[test]
    fn fixed_repeat_auto_repeat_matrix_has_eighteen_bounded_unique_scenes() {
        let scenes: Value = serde_json::from_str(SCENES_JSON).unwrap();
        let scenes = scenes.as_array().unwrap();
        assert_eq!(scenes.len(), 18);
        let ids = scenes
            .iter()
            .map(|scene| scene["id"].as_str().unwrap())
            .collect::<BTreeSet<_>>();
        assert_eq!(ids.len(), scenes.len());
        for expected_id in [
            "fixed-repeat-fill-prefix-empty",
            "fixed-repeat-fill-suffix-empty",
            "fixed-repeat-fill-prefix-calc-constant",
            "fixed-repeat-fit-prefix-empty",
            "fixed-repeat-fit-prefix-occupied",
            "fixed-repeat-row-fill-prefix-empty",
            "fixed-repeat-row-fill-suffix-empty",
            "fixed-repeat-row-fit-prefix-empty",
            "fixed-repeat-row-fit-prefix-occupied",
        ] {
            assert!(ids.contains(expected_id), "missing scene {expected_id}");
        }

        for scene in scenes {
            let scene_id = scene["id"].as_str().unwrap();
            let expected = &scene["expect"];
            let items = scene["items"].as_array().unwrap();
            let expected_items = expected["items"].as_array().unwrap();
            assert_eq!(expected["root"].as_array().unwrap().len(), 2, "{scene_id}");
            assert_eq!(items.len(), expected_items.len(), "{scene_id}");
            for (item, expected_item) in items.iter().zip(expected_items) {
                assert_eq!(item["name"], expected_item["name"], "{scene_id}");
                assert_eq!(
                    expected_item["frame"].as_array().unwrap().len(),
                    4,
                    "{scene_id}"
                );
            }
            for axis in ["columns", "rows"] {
                let tracks = expected["tracks"][axis].as_array().unwrap();
                let collapsed = expected["collapsed"][axis].as_array().unwrap();
                assert_eq!(tracks.len(), collapsed.len(), "{scene_id}/{axis}");
                assert!(
                    tracks.iter().all(|track| track.as_f64().is_some()),
                    "{scene_id}/{axis}"
                );
            }
            let source = runtime_source(scene_id).unwrap();
            assert!(source.len() < 8 * 1024, "{scene_id}");
            assert!(source.contains(scene_id));
            assert!(source.contains("grid-template-columns:"), "{scene_id}");
            assert!(source.contains("grid-template-rows:"), "{scene_id}");
            assert!(source.contains("background-color:"), "{scene_id}");
            assert!(source.contains("kebabProperty"), "{scene_id}");
            if !items.is_empty() {
                assert!(source.contains("placementStyles"), "{scene_id}");
            }
        }
        assert!(runtime_source("fixed-repeat-unlisted").is_none());
    }

    #[test]
    fn fixed_auto_repeat_template_area_scenes_match_precomparison_profiles() {
        let scenes: Value = serde_json::from_str(SCENES_JSON).unwrap();
        let scenes = scenes.as_array().unwrap();
        let precomparison: Value = serde_json::from_str(include_str!(
            "../../../../../../spec/internal/evidence/c11-4-fixed-auto-repeat-areas-precomparison-2026-10-24/expected-values.json"
        ))
        .unwrap();
        let precomparison_html = include_str!(
            "../../../../../../tests/fixtures/css/c11/grid-fixed-auto-repeat-template-areas-c114.html"
        );
        let expected_ids = [
            "fixed-auto-area-columns-fill-collision",
            "fixed-auto-area-columns-fit-empty",
            "fixed-auto-area-columns-fit-occupied",
            "fixed-auto-area-columns-area-larger",
            "fixed-auto-area-columns-area-shorter",
            "fixed-auto-area-rows-fill-collision",
            "fixed-auto-area-rows-fit-empty",
            "fixed-auto-area-rows-fit-occupied",
            "fixed-auto-area-rows-area-larger",
        ];

        for expected_id in expected_ids {
            let scene = scenes
                .iter()
                .find(|scene| scene["id"] == expected_id)
                .unwrap_or_else(|| panic!("missing scene {expected_id}"));
            let oracle_id = expected_id.strip_prefix("fixed-auto-area-").unwrap();
            let oracle = &precomparison["scenes"][oracle_id];
            assert_eq!(scene["oracleId"], oracle_id, "{expected_id}");
            assert!(scene["areas"].as_str().is_some(), "{expected_id}");
            assert_eq!(scene["rootColor"], "#22c55e", "{expected_id}/root color");
            assert!(
                css_rule_has_declaration(
                    precomparison_html,
                    ".grid",
                    &format!("background-color: {}", scene["rootColor"].as_str().unwrap()),
                ),
                "{expected_id}/root color differs from canonical CSS"
            );
            assert_eq!(scene["repeatAxis"], oracle["axis"], "{expected_id}");

            let selector = format!(".{oracle_id} {{");
            let css_start = precomparison_html
                .find(&selector)
                .unwrap_or_else(|| panic!("missing canonical CSS block {selector}"));
            let css_body_start = css_start + selector.len();
            let css_body_end = css_body_start
                + precomparison_html[css_body_start..]
                    .find('}')
                    .unwrap_or_else(|| panic!("unclosed canonical CSS block {selector}"));
            let css_body = &precomparison_html[css_body_start..css_body_end];
            for (property, value) in [
                ("grid-template-columns", scene["columns"].as_str().unwrap()),
                ("grid-template-rows", scene["rows"].as_str().unwrap()),
                ("grid-template-areas", scene["areas"].as_str().unwrap()),
            ] {
                assert!(
                    css_body.contains(&format!("{property}: {value};")),
                    "{expected_id}/{property}"
                );
            }
            if let Some(value) = scene.get("autoColumns").and_then(Value::as_str) {
                assert!(
                    css_body.contains(&format!("grid-auto-columns: {value};")),
                    "{expected_id}/grid-auto-columns"
                );
            }
            if let Some(value) = scene.get("autoRows").and_then(Value::as_str) {
                assert!(
                    css_body.contains(&format!("grid-auto-rows: {value};")),
                    "{expected_id}/grid-auto-rows"
                );
            }

            let root = scene["expect"]["root"].as_array().unwrap();
            assert_eq!(root.len(), 2, "{expected_id}");
            assert_eq!(
                root[0].as_u64().unwrap(),
                scene["width"]
                    .as_str()
                    .unwrap()
                    .trim_end_matches("px")
                    .parse::<u64>()
                    .unwrap(),
                "{expected_id}"
            );
            assert_eq!(
                root[1].as_u64().unwrap(),
                scene["height"]
                    .as_str()
                    .unwrap()
                    .trim_end_matches("px")
                    .parse::<u64>()
                    .unwrap(),
                "{expected_id}"
            );
            let available_dimension = if oracle["axis"] == "columns" {
                root[0].as_u64().unwrap()
            } else {
                root[1].as_u64().unwrap()
            };
            assert_eq!(
                available_dimension, oracle["availableSizePx"],
                "{expected_id}"
            );
            assert_eq!(
                scene["expect"]["tracks"][oracle["axis"].as_str().unwrap()],
                oracle["usedTrackSizesPx"],
                "{expected_id}"
            );

            let items = scene["items"].as_array().unwrap();
            let expected_items = scene["expect"]["items"].as_array().unwrap();
            assert_eq!(items.len(), expected_items.len(), "{expected_id}");
            for (item, expected_item) in items.iter().zip(expected_items) {
                assert_eq!(item["name"], expected_item["name"], "{expected_id}");
                let item_name = item["name"].as_str().unwrap();
                assert!(
                    css_rule_has_declaration(
                        precomparison_html,
                        &format!(".{item_name}"),
                        &format!("background-color: {}", item["color"].as_str().unwrap()),
                    ),
                    "{expected_id}/{item_name} color differs from canonical CSS"
                );
                assert_eq!(
                    &expected_item["frame"],
                    oracle["children"]
                        .get(format!("child {}", item["name"].as_str().unwrap()).as_str())
                        .unwrap(),
                    "{expected_id}"
                );
                for (property, value) in item["placement"].as_object().unwrap() {
                    let css_property = match property.as_str() {
                        "gridArea" => "grid-area",
                        "gridColumn" => "grid-column",
                        "gridRow" => "grid-row",
                        _ => panic!("{expected_id}: unhandled CSS placement {property}"),
                    };
                    let value = value.as_str().unwrap();
                    assert!(
                        css_rule_has_declaration(
                            precomparison_html,
                            &format!(".{oracle_id} .{item_name}"),
                            &format!("{css_property}: {value}"),
                        ),
                        "{expected_id}/{css_property}"
                    );
                }
            }
            if let Some(expected_overflow) = oracle.get("overflowBeyondRootPx") {
                let overflow_axis = if oracle["axis"] == "columns" { 0 } else { 1 };
                let right_frame = expected_items
                    .iter()
                    .find(|item| item["name"] == "right")
                    .unwrap()["frame"]
                    .as_array()
                    .unwrap();
                let overflow = right_frame[overflow_axis].as_u64().unwrap()
                    + right_frame[overflow_axis + 2].as_u64().unwrap()
                    - root[overflow_axis].as_u64().unwrap();
                assert_eq!(
                    overflow,
                    expected_overflow.as_u64().unwrap(),
                    "{expected_id} overflow"
                );
            }

            for axis in ["columns", "rows"] {
                let tracks = scene["expect"]["tracks"][axis].as_array().unwrap();
                let collapsed = scene["expect"]["collapsed"][axis].as_array().unwrap();
                assert_eq!(tracks.len(), collapsed.len(), "{expected_id}/{axis}");
                let should_collapse = axis == oracle["axis"] && oracle["mode"] == "auto-fit-empty";
                for (index, collapsed_track) in collapsed.iter().enumerate() {
                    assert_eq!(
                        collapsed_track.as_bool().unwrap(),
                        should_collapse && (4..8).contains(&index),
                        "{expected_id}/{axis}/{index}"
                    );
                }
            }

            let source = runtime_source(expected_id).unwrap();
            assert!(source.len() < 8 * 1024, "{expected_id}");
            assert!(source.contains("grid-template-areas:"), "{expected_id}");
            assert!(source.contains("grid-area"), "{expected_id}");
            assert!(source.contains("placementStyles"), "{expected_id}");
        }

        for forbidden_prefix in [
            "fixed-repeat-fixed-auto-area-columns-fill-collision",
            "fixed-repeat-fixed-auto-area-rows-area-larger",
        ] {
            assert!(runtime_source(forbidden_prefix).is_none());
        }

        let collision_source = runtime_source("fixed-auto-area-columns-fill-collision").unwrap();
        assert!(collision_source.contains("hero-start 3 / hero-end 3"));
        assert!(collision_source.contains("hero-start -1 / hero-end -1"));
        let implicit_track_source = runtime_source("fixed-auto-area-columns-area-larger").unwrap();
        assert!(implicit_track_source.contains("scene.autoColumns"));
        let row_implicit_track_source = runtime_source("fixed-auto-area-rows-area-larger").unwrap();
        assert!(row_implicit_track_source.contains("scene.autoRows"));
    }
}
