use std::collections::BTreeMap;

use spinon_core::NodeId;
use spinon_style::ComputedElementStyle;

use super::{RuntimeLayoutFailure, layout_failure};

pub(super) const FONT_SIZE_SCENE_IDS: [&str; 10] = [
    "font-size-5-min-content",
    "font-size-5-max-content",
    "font-size-7-5-min-content",
    "font-size-7-5-max-content",
    "font-size-10-min-content",
    "font-size-10-max-content",
    "font-size-12-5-min-content",
    "font-size-12-5-max-content",
    "font-size-20-min-content",
    "font-size-20-max-content",
];

pub(super) fn validate_computed_font_size(
    style: &ComputedElementStyle,
    scene_id: &str,
) -> Result<f32, RuntimeLayoutFailure> {
    validate_font_size(
        style.node_id,
        &style.properties,
        style.font_size_css_px,
        scene_id,
    )
}

fn validate_font_size(
    node_id: NodeId,
    properties: &BTreeMap<String, String>,
    computed_font_size: f32,
    scene_id: &str,
) -> Result<f32, RuntimeLayoutFailure> {
    let Some(expected) = expected_font_size(scene_id) else {
        return Err(layout_failure(
            "unsupported_c114_anonymous_text_style",
            Some(node_id.get()),
            Some(format!("font-size scene id is not allowlisted: {scene_id}")),
        ));
    };
    let expected_css = font_size_css_value_for_px(expected);
    let invalid = || {
        layout_failure(
            "unsupported_c114_anonymous_text_style",
            Some(node_id.get()),
            Some(format!(
                "font-size: expected {expected_css} / {expected}px, got {:?} / {computed_font_size}px",
                properties.get("font-size")
            )),
        )
    };
    if properties.get("font-size").map(String::as_str) != Some(expected_css)
        || !computed_font_size.is_finite()
        || computed_font_size <= 0.0
        || computed_font_size != expected
    {
        return Err(invalid());
    }
    Ok(expected)
}

fn expected_font_size(scene_id: &str) -> Option<f32> {
    match scene_id {
        "font-size-5-min-content" | "font-size-5-max-content" => Some(5.0),
        "font-size-7-5-min-content" | "font-size-7-5-max-content" => Some(7.5),
        "font-size-10-min-content" | "font-size-10-max-content" => Some(10.0),
        "font-size-12-5-min-content" | "font-size-12-5-max-content" => Some(12.5),
        "font-size-20-min-content" | "font-size-20-max-content" => Some(20.0),
        _ if super::SCENE_IDS.contains(&scene_id)
            || super::LINE_HEIGHT_SCENE_IDS.contains(&scene_id)
            || super::full_tab_scene(scene_id).is_some() =>
        {
            Some(10.0)
        }
        _ => None,
    }
}

pub(super) fn font_size_css_value(scene_id: &str) -> &'static str {
    expected_font_size(scene_id)
        .map(font_size_css_value_for_px)
        .unwrap_or("unsupported-font-size")
}

pub(super) fn font_size_css_value_for_px(font_size: f32) -> &'static str {
    match font_size {
        5.0 => "5px",
        7.5 => "7.5px",
        10.0 => "10px",
        12.5 => "12.5px",
        20.0 => "20px",
        _ => "unsupported-font-size",
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FONT_SIZE_SCENE_IDS, font_size_css_value, font_size_css_value_for_px, validate_font_size,
    };
    use spinon_core::NodeId;
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn computed_font_size_scene_matrix_has_ten_unique_allowlisted_scenes() {
        let unique = FONT_SIZE_SCENE_IDS.iter().copied().collect::<BTreeSet<_>>();
        assert_eq!(FONT_SIZE_SCENE_IDS.len(), 10);
        assert_eq!(unique.len(), FONT_SIZE_SCENE_IDS.len());
        assert!(font_size_css_value("font-size-7-5-min-content") == "7.5px");
        assert!(font_size_css_value("font-size-12-5-max-content") == "12.5px");
        assert!(font_size_css_value("anonymous-normal-min-content") == "10px");
        assert!(font_size_css_value("unknown-font-size-scene") == "unsupported-font-size");
    }

    #[test]
    fn computed_font_size_requires_matching_property_and_typed_value() {
        let node_id = NodeId::new(1).unwrap();
        for (scene_id, css, computed) in [
            ("font-size-5-min-content", "5px", 5.0),
            ("font-size-7-5-max-content", "7.5px", 7.5),
            ("font-size-10-min-content", "10px", 10.0),
            ("font-size-12-5-max-content", "12.5px", 12.5),
            ("font-size-20-max-content", "20px", 20.0),
            ("anonymous-normal-min-content", "10px", 10.0),
        ] {
            let properties = BTreeMap::from([("font-size".to_owned(), css.to_owned())]);
            assert_eq!(
                validate_font_size(node_id, &properties, computed, scene_id),
                Ok(computed)
            );
        }
        let properties = BTreeMap::from([("font-size".to_owned(), "10px".to_owned())]);
        assert!(validate_font_size(node_id, &properties, 10.0, "unknown-font-size-scene").is_err());

        for (scene_id, css, computed) in [
            ("font-size-7-5-min-content", "7.50px", 7.5),
            ("font-size-7-5-min-content", "7.5px", 7.5001),
            ("font-size-7-5-min-content", "7.5px", f32::NAN),
            ("font-size-7-5-min-content", "7.5px", f32::INFINITY),
            ("font-size-7-5-min-content", "7.5px", 0.0),
            ("font-size-7-5-min-content", "7.5px", -1.0),
            ("font-size-6-min-content", "6px", 6.0),
            ("anonymous-normal-min-content", "20px", 20.0),
        ] {
            let properties = BTreeMap::from([("font-size".to_owned(), css.to_owned())]);
            assert!(
                validate_font_size(node_id, &properties, computed, scene_id).is_err(),
                "{scene_id}: {css}, {computed}"
            );
        }
    }

    #[test]
    fn computed_font_size_css_serialization_stays_on_allowlist() {
        assert_eq!(font_size_css_value_for_px(5.0), "5px");
        assert_eq!(font_size_css_value_for_px(7.5), "7.5px");
        assert_eq!(font_size_css_value_for_px(10.0), "10px");
        assert_eq!(font_size_css_value_for_px(12.5), "12.5px");
        assert_eq!(font_size_css_value_for_px(20.0), "20px");
        assert_eq!(font_size_css_value_for_px(7.5001), "unsupported-font-size");
    }
}
