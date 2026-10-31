use std::collections::BTreeMap;

use spinon_core::NodeId;

use super::{RuntimeLayoutFailure, layout_failure};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct TextIndentScene {
    pub(super) id: &'static str,
    pub(super) text: &'static str,
    pub(super) computed_value: &'static str,
    pub(super) text_indent: f32,
    pub(super) is_min_content: bool,
}

pub(super) const TEXT_INDENT_SCENES: [TextIndentScene; 30] = [
    scene(
        "text-indent-zero-two-words-min-content",
        "XX XX",
        "0px",
        0.0,
        true,
    ),
    scene(
        "text-indent-zero-two-words-max-content",
        "XX XX",
        "0px",
        0.0,
        false,
    ),
    scene(
        "text-indent-zero-collapsed-three-words-min-content",
        "XX   XX XX  ",
        "0px",
        0.0,
        true,
    ),
    scene(
        "text-indent-zero-collapsed-three-words-max-content",
        "XX   XX XX  ",
        "0px",
        0.0,
        false,
    ),
    scene(
        "text-indent-zero-single-word-min-content",
        "XX",
        "0px",
        0.0,
        true,
    ),
    scene(
        "text-indent-zero-single-word-max-content",
        "XX",
        "0px",
        0.0,
        false,
    ),
    scene(
        "text-indent-five-two-words-min-content",
        "XX XX",
        "5px",
        5.0,
        true,
    ),
    scene(
        "text-indent-five-two-words-max-content",
        "XX XX",
        "5px",
        5.0,
        false,
    ),
    scene(
        "text-indent-five-collapsed-three-words-min-content",
        "XX   XX XX  ",
        "5px",
        5.0,
        true,
    ),
    scene(
        "text-indent-five-collapsed-three-words-max-content",
        "XX   XX XX  ",
        "5px",
        5.0,
        false,
    ),
    scene(
        "text-indent-five-single-word-min-content",
        "XX",
        "5px",
        5.0,
        true,
    ),
    scene(
        "text-indent-five-single-word-max-content",
        "XX",
        "5px",
        5.0,
        false,
    ),
    scene(
        "text-indent-ten-two-words-min-content",
        "XX XX",
        "10px",
        10.0,
        true,
    ),
    scene(
        "text-indent-ten-two-words-max-content",
        "XX XX",
        "10px",
        10.0,
        false,
    ),
    scene(
        "text-indent-ten-collapsed-three-words-min-content",
        "XX   XX XX  ",
        "10px",
        10.0,
        true,
    ),
    scene(
        "text-indent-ten-collapsed-three-words-max-content",
        "XX   XX XX  ",
        "10px",
        10.0,
        false,
    ),
    scene(
        "text-indent-ten-single-word-min-content",
        "XX",
        "10px",
        10.0,
        true,
    ),
    scene(
        "text-indent-ten-single-word-max-content",
        "XX",
        "10px",
        10.0,
        false,
    ),
    scene(
        "text-indent-negative-five-first-word-long-min-content",
        "XXX X",
        "-5px",
        -5.0,
        true,
    ),
    scene(
        "text-indent-negative-five-first-word-long-max-content",
        "XXX X",
        "-5px",
        -5.0,
        false,
    ),
    scene(
        "text-indent-negative-ten-first-word-long-min-content",
        "XXX X",
        "-10px",
        -10.0,
        true,
    ),
    scene(
        "text-indent-negative-ten-first-word-long-max-content",
        "XXX X",
        "-10px",
        -10.0,
        false,
    ),
    scene(
        "text-indent-negative-five-first-word-short-min-content",
        "X XXX",
        "-5px",
        -5.0,
        true,
    ),
    scene(
        "text-indent-negative-five-first-word-short-max-content",
        "X XXX",
        "-5px",
        -5.0,
        false,
    ),
    scene(
        "text-indent-negative-ten-first-word-short-min-content",
        "X XXX",
        "-10px",
        -10.0,
        true,
    ),
    scene(
        "text-indent-negative-ten-first-word-short-max-content",
        "X XXX",
        "-10px",
        -10.0,
        false,
    ),
    scene(
        "text-indent-negative-five-single-word-min-content",
        "XX",
        "-5px",
        -5.0,
        true,
    ),
    scene(
        "text-indent-negative-five-single-word-max-content",
        "XX",
        "-5px",
        -5.0,
        false,
    ),
    scene(
        "text-indent-negative-ten-single-word-min-content",
        "XX",
        "-10px",
        -10.0,
        true,
    ),
    scene(
        "text-indent-negative-ten-single-word-max-content",
        "XX",
        "-10px",
        -10.0,
        false,
    ),
];

const fn scene(
    id: &'static str,
    text: &'static str,
    computed_value: &'static str,
    text_indent: f32,
    is_min_content: bool,
) -> TextIndentScene {
    TextIndentScene {
        id,
        text,
        computed_value,
        text_indent,
        is_min_content,
    }
}

pub(super) const TEXT_INDENT_SCENE_IDS: [&str; TEXT_INDENT_SCENES.len()] =
    scene_ids(TEXT_INDENT_SCENES);

const fn scene_ids(scenes: [TextIndentScene; TEXT_INDENT_SCENES.len()]) -> [&'static str; 30] {
    let mut ids = [""; 30];
    let mut index = 0;
    while index < scenes.len() {
        ids[index] = scenes[index].id;
        index += 1;
    }
    ids
}

pub(super) fn find_scene(scene_id: &str) -> Option<TextIndentScene> {
    TEXT_INDENT_SCENES
        .iter()
        .copied()
        .find(|scene| scene.id == scene_id)
}

pub(super) fn validate_computed_text_indent(
    node_id: NodeId,
    properties: &BTreeMap<String, String>,
    scene_id: &str,
) -> Result<f32, RuntimeLayoutFailure> {
    let expected = find_scene(scene_id);
    let expected_serialization = expected.map_or("0px", |scene| scene.computed_value);
    let invalid = || {
        layout_failure(
            "unsupported_c114_anonymous_text_style",
            Some(node_id.get()),
            Some(format!(
                "text-indent: expected {expected_serialization}, got {:?}",
                properties.get("text-indent")
            )),
        )
    };
    let value = properties
        .get("text-indent")
        .filter(|value| value.as_str() == expected_serialization)
        .ok_or_else(invalid)?;
    let parsed = parse_computed_text_indent(value).ok_or_else(invalid)?;
    let expected_px = expected.map_or(0.0, |scene| scene.text_indent);
    if parsed.to_bits() != expected_px.to_bits() {
        return Err(invalid());
    }
    Ok(parsed)
}

fn parse_computed_text_indent(value: &str) -> Option<f32> {
    let number = value.strip_suffix("px")?;
    if number.is_empty() || number.trim() != number {
        return None;
    }
    let number = number.parse::<f64>().ok()?;
    if !number.is_finite() || !(-10.0..=10.0).contains(&number) || number == 0.0 && value != "0px" {
        return None;
    }
    let value = number as f32;
    value.is_finite().then_some(value)
}

#[cfg(test)]
mod tests {
    use super::{
        TEXT_INDENT_SCENE_IDS, TEXT_INDENT_SCENES, find_scene, parse_computed_text_indent,
        validate_computed_text_indent,
    };
    use spinon_core::NodeId;
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn computed_text_indent_matrix_has_thirty_unique_scenes() {
        let unique = TEXT_INDENT_SCENE_IDS
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        assert_eq!(TEXT_INDENT_SCENES.len(), 30);
        assert_eq!(unique.len(), TEXT_INDENT_SCENES.len());
        assert!(find_scene("text-indent-ten-collapsed-three-words-max-content").is_some());
        assert!(find_scene("text-indent-negative-ten-first-word-short-min-content").is_some());
        assert!(find_scene("text-indent-unlisted").is_none());
    }

    #[test]
    fn computed_text_indent_accepts_only_the_fixed_scene_values() {
        let node_id = NodeId::new(1).unwrap();
        for scene in TEXT_INDENT_SCENES {
            let properties =
                BTreeMap::from([("text-indent".to_owned(), scene.computed_value.to_owned())]);
            assert_eq!(
                validate_computed_text_indent(node_id, &properties, scene.id),
                Ok(scene.text_indent),
                "{}",
                scene.id
            );
        }
        for value in [
            "-11px",
            "11px",
            "-0px",
            "5%",
            "5px hanging",
            "5px each-line",
            "NaNpx",
            "infpx",
        ] {
            assert!(
                parse_computed_text_indent(value).is_none(),
                "accepted {value} in the fixed px profile"
            );
        }
        let five = find_scene("text-indent-five-two-words-min-content").unwrap();
        for value in [
            "0px", "10px", "5.0px", "+5px", " 5px", "-5px", "-5.0px", "-7px", "-30px",
        ] {
            let properties = BTreeMap::from([("text-indent".to_owned(), value.to_owned())]);
            assert!(
                validate_computed_text_indent(node_id, &properties, five.id).is_err(),
                "accepted {value} for {}",
                five.id
            );
        }
        let default = BTreeMap::from([("text-indent".to_owned(), "0px".to_owned())]);
        assert_eq!(
            validate_computed_text_indent(node_id, &default, "anonymous-normal-min-content"),
            Ok(0.0)
        );
        let changed = BTreeMap::from([("text-indent".to_owned(), "5px".to_owned())]);
        assert!(
            validate_computed_text_indent(node_id, &changed, "anonymous-normal-min-content")
                .is_err()
        );
    }
}
