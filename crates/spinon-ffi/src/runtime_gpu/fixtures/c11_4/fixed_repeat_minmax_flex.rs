const SCENES_JSON: &str = include_str!("fixed_repeat_minmax_flex_scenes.json");

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
  `background-color:${{scene.rootColor}}`,
].join(';'));
document.appendChild(root);
const itemIds = scene.items.map((itemSpec) => {{
  const item = document.createElement('div');
  const itemId = `c114-${{scene.id}}-${{itemSpec.name}}`;
  item.setAttribute('id', itemId);
  item.setAttribute('data-node', '');
  const placementStyles = Object.entries(itemSpec.placement ?? {{}}).map(([property, value]) => {{
    const kebabProperty = property.replace(/[A-Z]/g, (letter) => `-${{letter.toLowerCase()}}`);
    return `${{kebabProperty}}:${{value}}`;
  }});
  item.setAttribute('style', [
    'box-sizing:border-box', 'display:block', `width:${{itemSpec.width ?? 'auto'}}`,
    `height:${{itemSpec.height ?? 'auto'}}`, 'min-width:0', 'min-height:0',
    'margin:0', 'padding:0', 'border:0', ...placementStyles,
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
    use crate::runtime_gpu::fixtures::c11_4::c11_4_grid_track_sizing_runtime_source;
    use serde_json::Value;
    use std::collections::BTreeSet;

    #[test]
    fn fixed_repeat_minmax_flex_matrix_matches_precomparison_and_routes() {
        let scenes: Value = serde_json::from_str(SCENES_JSON).unwrap();
        let scenes = scenes.as_array().unwrap();
        let expectations: Value = serde_json::from_str(include_str!(
            "../../../../../../spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-precomparison-2026-10-25/spec-derived-expectations.json"
        ))
        .unwrap();
        let chrome_observation: Value = serde_json::from_str(include_str!(
            "../../../../../../spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-precomparison-2026-10-25/chrome156-observation.json"
        ))
        .unwrap();
        let precomparison_html = include_str!(
            "../../../../../../tests/fixtures/css/c11/grid-fixed-repeat-minmax-flex-c114-precomparison.html"
        );
        let expectations = expectations["scenes"].as_array().unwrap();
        let chrome_scenes = chrome_observation["scenes"].as_array().unwrap();
        assert_eq!(scenes.len(), 6);
        assert_eq!(chrome_scenes.len(), scenes.len(), "Chrome scene count");
        let ids = scenes
            .iter()
            .map(|scene| scene["id"].as_str().unwrap())
            .collect::<BTreeSet<_>>();
        assert_eq!(ids.len(), scenes.len());
        let chrome_ids = chrome_scenes
            .iter()
            .map(|scene| scene["id"].as_str().unwrap())
            .collect::<BTreeSet<_>>();
        assert_eq!(chrome_ids.len(), chrome_scenes.len(), "Chrome scene IDs");

        for (scene_index, scene) in scenes.iter().enumerate() {
            let scene_id = scene["id"].as_str().unwrap();
            let oracle_id = scene["oracleId"].as_str().unwrap();
            let chrome_scene = &chrome_scenes[scene_index];
            assert_eq!(
                chrome_scene["id"].as_str().unwrap(),
                oracle_id,
                "{scene_id} Chrome observation order/ID"
            );
            let oracle = expectations
                .iter()
                .find(|expected| expected["id"] == oracle_id)
                .unwrap_or_else(|| panic!("missing independent expectation for {oracle_id}"));
            let axis = oracle["axis"].as_str().unwrap();
            assert_eq!(scene["repeatAxis"], axis, "{scene_id}");
            assert_eq!(
                scene["expect"]["tracks"][axis], oracle["expectedTracks"],
                "{scene_id} used tracks"
            );
            let track_indices = scene["expect"]["collapsed"][axis]
                .as_array()
                .unwrap()
                .iter()
                .enumerate()
                .filter_map(|(index, collapsed)| collapsed.as_bool().unwrap().then_some(index))
                .collect::<Vec<_>>();
            let expected_collapsed_indices = oracle["collapsedAutoRepeatTracks"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|index| index.as_u64().unwrap() as usize)
                .collect::<Vec<_>>();
            assert_eq!(
                track_indices, expected_collapsed_indices,
                "{scene_id} collapsed auto-repeat tracks"
            );
            let selector = format!("#{oracle_id} {{");
            let css_block = precomparison_html
                .split_once(&selector)
                .and_then(|(_, remainder)| remainder.split_once('}').map(|(block, _)| block))
                .unwrap_or_else(|| panic!("missing CSS input for {oracle_id}"));
            for (property, value) in [
                ("grid-template-columns", scene["columns"].as_str().unwrap()),
                ("grid-template-rows", scene["rows"].as_str().unwrap()),
            ] {
                assert!(
                    css_block.contains(&format!("{property}: {value};")),
                    "{scene_id} {property} differs from the precomparison input"
                );
            }
            let starts = oracle["expectedItemStarts"].as_array().unwrap();
            let items = scene["expect"]["items"].as_array().unwrap();
            let source_items = scene["items"].as_array().unwrap();
            let chrome_items = chrome_scene["items"].as_array().unwrap();
            assert_eq!(items.len(), starts.len(), "{scene_id} item count");
            assert_eq!(
                source_items.len(),
                items.len(),
                "{scene_id} source item count"
            );
            assert_eq!(
                chrome_items.len(),
                items.len(),
                "{scene_id} Chrome item count"
            );

            let oracle_tracks = oracle["expectedTracks"]
                .as_array()
                .unwrap()
                .iter()
                .map(|track| number(track, scene_id))
                .collect::<Vec<_>>();
            let collapsed_tracks = oracle["collapsedAutoRepeatTracks"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|index| index.as_u64().unwrap() as usize)
                .collect::<BTreeSet<_>>();
            assert!(
                collapsed_tracks
                    .iter()
                    .all(|index| *index < oracle_tracks.len()),
                "{scene_id} collapsed track index is outside the oracle track list"
            );

            let root_width = length_px(&scene["width"], scene_id);
            let root_height = length_px(&scene["height"], scene_id);
            assert_css_px_close(
                number(&chrome_scene["client"]["width"], scene_id),
                root_width,
                &format!("{scene_id} Chrome root width"),
            );
            assert_css_px_close(
                number(&chrome_scene["client"]["height"], scene_id),
                root_height,
                &format!("{scene_id} Chrome root height"),
            );
            let column_gap = length_px(&scene["columnGap"], scene_id);
            let row_gap = length_px(&scene["rowGap"], scene_id);
            let (column_tracks, collapsed_columns) = if axis == "columns" {
                (oracle_tracks.clone(), collapsed_tracks.clone())
            } else {
                let fixed_column_size = length_px(&scene["columns"], scene_id);
                assert_css_px_close(fixed_column_size, root_width, scene_id);
                (vec![root_width], BTreeSet::new())
            };
            let (row_tracks, collapsed_rows) = if axis == "rows" {
                (oracle_tracks, collapsed_tracks)
            } else {
                let fixed_row_size = length_px(&scene["rows"], scene_id);
                assert_css_px_close(fixed_row_size, root_height, scene_id);
                (vec![root_height], BTreeSet::new())
            };

            for (item_index, ((item, start), chrome_item)) in
                items.iter().zip(starts).zip(chrome_items).enumerate()
            {
                let source_item = &source_items[item_index];
                assert_eq!(item["name"], source_item["name"], "{scene_id} item order");
                let chrome_item_index = chrome_item["index"].as_u64().unwrap() as usize;
                assert_eq!(
                    chrome_item_index,
                    item_index + 1,
                    "{scene_id} Chrome item order"
                );
                assert_eq!(
                    source_item["name"].as_str().unwrap(),
                    format!("item-{chrome_item_index}"),
                    "{scene_id} Chrome item ID/order"
                );
                let column_range = grid_line_range(source_item, "gridColumn", scene_id);
                let row_range = grid_line_range(source_item, "gridRow", scene_id);
                let (x, width) = axis_rect(
                    &column_tracks,
                    &collapsed_columns,
                    column_range,
                    column_gap,
                    scene_id,
                );
                let (y, height) =
                    axis_rect(&row_tracks, &collapsed_rows, row_range, row_gap, scene_id);

                let oracle_start = number(start, scene_id);
                let computed_start = if axis == "columns" { x } else { y };
                assert_css_px_close(computed_start, oracle_start, scene_id);

                let expected_rect = [x, y, width, height];
                let fixture_rect = item["frame"].as_array().unwrap();
                assert_eq!(
                    fixture_rect.len(),
                    expected_rect.len(),
                    "{scene_id} item {item_index} rect length"
                );
                for (coordinate, (actual, expected)) in
                    fixture_rect.iter().zip(expected_rect).enumerate()
                {
                    assert_css_px_close(
                        number(actual, scene_id),
                        expected,
                        &format!("{scene_id} item {item_index} rect coordinate {coordinate}"),
                    );
                }
                for (coordinate, property) in ["x", "y", "width", "height"].iter().enumerate() {
                    assert_css_px_close(
                        number(&chrome_item["rect"][*property], scene_id),
                        number(&fixture_rect[coordinate], scene_id),
                        &format!("{scene_id} Chrome item {item_index} {property}"),
                    );
                }
            }

            let source = runtime_source(scene_id).unwrap();
            assert!(source.len() < 8 * 1024, "{scene_id} source is too large");
            assert!(source.contains(scene_id));
            assert!(source.contains("grid-template-columns:"));
            assert!(source.contains("grid-template-rows:"));
            assert!(source.contains("placementStyles"));
            assert!(
                c11_4_grid_track_sizing_runtime_source(scene_id).is_ok(),
                "{scene_id} is not reachable through the C11.4 runtime fixture router"
            );
        }
        assert!(runtime_source("fixed-repeat-minmax-flex-unlisted").is_none());
    }

    const CSS_PX_TOLERANCE: f64 = 1.0 / 64.0;

    fn number(value: &Value, context: &str) -> f64 {
        value
            .as_f64()
            .unwrap_or_else(|| panic!("{context}: expected numeric CSS-pixel value, got {value}"))
    }

    fn length_px(value: &Value, context: &str) -> f64 {
        let value = value
            .as_str()
            .unwrap_or_else(|| panic!("{context}: expected fixed px length"));
        let numeric = value
            .strip_suffix("px")
            .unwrap_or_else(|| panic!("{context}: expected px length, got {value}"));
        numeric
            .parse::<f64>()
            .unwrap_or_else(|error| panic!("{context}: invalid px length {value}: {error}"))
    }

    fn grid_line_range(item: &Value, property: &str, context: &str) -> std::ops::Range<usize> {
        let value = item["placement"][property]
            .as_str()
            .unwrap_or_else(|| panic!("{context}: missing {property} placement"));
        let lines = value.split('/').map(str::trim).collect::<Vec<_>>();
        assert!(
            lines.len() == 1 || lines.len() == 2,
            "{context}: invalid {property}: {value}"
        );
        let start_line = lines[0]
            .parse::<usize>()
            .unwrap_or_else(|error| panic!("{context}: invalid {property} start {value}: {error}"));
        let end_line = if lines.len() == 2 {
            lines[1].parse::<usize>().unwrap_or_else(|error| {
                panic!("{context}: invalid {property} end {value}: {error}")
            })
        } else {
            start_line
                .checked_add(1)
                .expect("single grid line must fit usize")
        };
        assert!(
            start_line > 0 && end_line > start_line,
            "{context}: {property} must be an increasing positive explicit line range: {value}"
        );
        (start_line - 1)..(end_line - 1)
    }

    fn axis_rect(
        tracks: &[f64],
        collapsed_tracks: &BTreeSet<usize>,
        range: std::ops::Range<usize>,
        gap: f64,
        context: &str,
    ) -> (f64, f64) {
        assert!(
            range.start < range.end && range.end <= tracks.len(),
            "{context}: item grid-line range {range:?} exceeds {} tracks",
            tracks.len()
        );

        let mut origin = 0.0;
        let mut preceding_live_tracks = 0_usize;
        for (index, track) in tracks[..range.start].iter().enumerate() {
            if !collapsed_tracks.contains(&index) {
                origin += track;
                preceding_live_tracks += 1;
            }
        }
        origin += gap * preceding_live_tracks as f64;

        let mut extent = 0.0;
        let mut live_tracks_in_range = 0_usize;
        for (index, track) in tracks[range.clone()].iter().enumerate() {
            let track_index = range.start + index;
            if !collapsed_tracks.contains(&track_index) {
                extent += track;
                live_tracks_in_range += 1;
            }
        }
        extent += gap * live_tracks_in_range.saturating_sub(1) as f64;
        (origin, extent)
    }

    fn assert_css_px_close(actual: f64, expected: f64, context: &str) {
        // Fractional geometry is compared within 1/64 CSS px (0.015625px).
        let difference = (actual - expected).abs();
        assert!(
            difference <= CSS_PX_TOLERANCE,
            "{context}: {actual} differs from {expected} by {difference}px; tolerance is 1/64 CSS px"
        );
    }
}
