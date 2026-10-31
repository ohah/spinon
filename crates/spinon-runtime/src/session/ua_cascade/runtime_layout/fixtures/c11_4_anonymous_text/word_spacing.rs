use std::collections::BTreeMap;

use spinon_core::NodeId;

use super::{RuntimeLayoutFailure, layout_failure};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct WordSpacingScene {
    pub(super) id: &'static str,
    pub(super) text: &'static str,
    pub(super) computed_value: &'static str,
    pub(super) word_spacing: f32,
    pub(super) is_min_content: bool,
}

pub(super) const WORD_SPACING_SCENES: [WordSpacingScene; 24] = [
    scene(
        "word-spacing-normal-two-words-min-content",
        "XX XX",
        "0px",
        0.0,
        true,
    ),
    scene(
        "word-spacing-normal-two-words-max-content",
        "XX XX",
        "0px",
        0.0,
        false,
    ),
    scene(
        "word-spacing-normal-collapsed-three-words-min-content",
        "XX   XX XX  ",
        "0px",
        0.0,
        true,
    ),
    scene(
        "word-spacing-normal-collapsed-three-words-max-content",
        "XX   XX XX  ",
        "0px",
        0.0,
        false,
    ),
    scene(
        "word-spacing-normal-single-word-min-content",
        "XX",
        "0px",
        0.0,
        true,
    ),
    scene(
        "word-spacing-normal-single-word-max-content",
        "XX",
        "0px",
        0.0,
        false,
    ),
    scene(
        "word-spacing-zero-two-words-min-content",
        "XX XX",
        "0px",
        0.0,
        true,
    ),
    scene(
        "word-spacing-zero-two-words-max-content",
        "XX XX",
        "0px",
        0.0,
        false,
    ),
    scene(
        "word-spacing-zero-collapsed-three-words-min-content",
        "XX   XX XX  ",
        "0px",
        0.0,
        true,
    ),
    scene(
        "word-spacing-zero-collapsed-three-words-max-content",
        "XX   XX XX  ",
        "0px",
        0.0,
        false,
    ),
    scene(
        "word-spacing-zero-single-word-min-content",
        "XX",
        "0px",
        0.0,
        true,
    ),
    scene(
        "word-spacing-zero-single-word-max-content",
        "XX",
        "0px",
        0.0,
        false,
    ),
    scene(
        "word-spacing-minus-one-two-words-min-content",
        "XX XX",
        "-1px",
        -1.0,
        true,
    ),
    scene(
        "word-spacing-minus-one-two-words-max-content",
        "XX XX",
        "-1px",
        -1.0,
        false,
    ),
    scene(
        "word-spacing-minus-one-collapsed-three-words-min-content",
        "XX   XX XX  ",
        "-1px",
        -1.0,
        true,
    ),
    scene(
        "word-spacing-minus-one-collapsed-three-words-max-content",
        "XX   XX XX  ",
        "-1px",
        -1.0,
        false,
    ),
    scene(
        "word-spacing-minus-one-single-word-min-content",
        "XX",
        "-1px",
        -1.0,
        true,
    ),
    scene(
        "word-spacing-minus-one-single-word-max-content",
        "XX",
        "-1px",
        -1.0,
        false,
    ),
    scene(
        "word-spacing-plus-one-two-words-min-content",
        "XX XX",
        "1px",
        1.0,
        true,
    ),
    scene(
        "word-spacing-plus-one-two-words-max-content",
        "XX XX",
        "1px",
        1.0,
        false,
    ),
    scene(
        "word-spacing-plus-one-collapsed-three-words-min-content",
        "XX   XX XX  ",
        "1px",
        1.0,
        true,
    ),
    scene(
        "word-spacing-plus-one-collapsed-three-words-max-content",
        "XX   XX XX  ",
        "1px",
        1.0,
        false,
    ),
    scene(
        "word-spacing-plus-one-single-word-min-content",
        "XX",
        "1px",
        1.0,
        true,
    ),
    scene(
        "word-spacing-plus-one-single-word-max-content",
        "XX",
        "1px",
        1.0,
        false,
    ),
];

const fn scene(
    id: &'static str,
    text: &'static str,
    computed_value: &'static str,
    word_spacing: f32,
    is_min_content: bool,
) -> WordSpacingScene {
    WordSpacingScene {
        id,
        text,
        computed_value,
        word_spacing,
        is_min_content,
    }
}

pub(super) const WORD_SPACING_SCENE_IDS: [&str; WORD_SPACING_SCENES.len()] =
    scene_ids(WORD_SPACING_SCENES);

const fn scene_ids(
    scenes: [WordSpacingScene; WORD_SPACING_SCENES.len()],
) -> [&'static str; WORD_SPACING_SCENES.len()] {
    let mut ids = [""; WORD_SPACING_SCENES.len()];
    let mut index = 0;
    while index < scenes.len() {
        ids[index] = scenes[index].id;
        index += 1;
    }
    ids
}

pub(super) fn find_scene(scene_id: &str) -> Option<WordSpacingScene> {
    WORD_SPACING_SCENES
        .iter()
        .copied()
        .find(|scene| scene.id == scene_id)
}

pub(super) fn validate_computed_word_spacing(
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
                "word-spacing: expected {expected_serialization}, got {:?}",
                properties.get("word-spacing")
            )),
        )
    };
    let value = properties
        .get("word-spacing")
        .filter(|value| value.as_str() == expected_serialization)
        .ok_or_else(invalid)?;
    let parsed = parse_computed_word_spacing(value).ok_or_else(invalid)?;
    let expected_px = expected.map_or(0.0, |scene| scene.word_spacing);
    if parsed.to_bits() != expected_px.to_bits() {
        return Err(invalid());
    }
    Ok(parsed)
}

fn parse_computed_word_spacing(value: &str) -> Option<f32> {
    let number = value.strip_suffix("px")?;
    if number.is_empty() || number.trim() != number {
        return None;
    }
    let number = number.parse::<f64>().ok()?;
    if !number.is_finite() || !(-1.0..=1.0).contains(&number) || number == 0.0 && value != "0px" {
        return None;
    }
    let value = number as f32;
    value.is_finite().then_some(value)
}

#[cfg(test)]
mod tests {
    use super::{
        WORD_SPACING_SCENE_IDS, WORD_SPACING_SCENES, find_scene, parse_computed_word_spacing,
        validate_computed_word_spacing,
    };
    use spinon_core::NodeId;
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn computed_word_spacing_matrix_has_twenty_four_unique_scenes() {
        let unique = WORD_SPACING_SCENE_IDS
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        assert_eq!(WORD_SPACING_SCENES.len(), 24);
        assert_eq!(unique.len(), WORD_SPACING_SCENES.len());
        assert!(find_scene("word-spacing-minus-one-collapsed-three-words-max-content").is_some());
        assert!(find_scene("word-spacing-unlisted").is_none());
    }

    #[test]
    fn computed_word_spacing_requires_the_exact_scene_value_and_finite_px() {
        let node_id = NodeId::new(1).unwrap();
        for scene in WORD_SPACING_SCENES {
            let properties =
                BTreeMap::from([("word-spacing".to_owned(), scene.computed_value.to_owned())]);
            assert_eq!(
                validate_computed_word_spacing(node_id, &properties, scene.id),
                Ok(scene.word_spacing),
                "{}",
                scene.id
            );
        }
        for value in ["normal", "2px", "NaNpx", "infpx", "1em"] {
            assert!(
                parse_computed_word_spacing(value).is_none(),
                "accepted {value}"
            );
        }
        let plus_one = find_scene("word-spacing-plus-one-two-words-min-content").unwrap();
        for value in ["-0px", "0.0px", "1.00px", "+1px", "1em"] {
            let properties = BTreeMap::from([("word-spacing".to_owned(), value.to_owned())]);
            assert!(
                validate_computed_word_spacing(node_id, &properties, plus_one.id).is_err(),
                "accepted {value} for {}",
                plus_one.id
            );
        }
        let default = BTreeMap::from([("word-spacing".to_owned(), "0px".to_owned())]);
        assert_eq!(
            validate_computed_word_spacing(node_id, &default, "anonymous-normal-min-content"),
            Ok(0.0)
        );
        let changed = BTreeMap::from([("word-spacing".to_owned(), "1px".to_owned())]);
        assert!(
            validate_computed_word_spacing(node_id, &changed, "anonymous-normal-min-content")
                .is_err()
        );
    }
}
