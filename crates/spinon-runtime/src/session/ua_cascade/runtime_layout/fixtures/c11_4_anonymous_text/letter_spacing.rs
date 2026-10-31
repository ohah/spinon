use std::collections::BTreeMap;

use spinon_core::NodeId;

use super::{RuntimeLayoutFailure, layout_failure};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct LetterSpacingScene {
    pub(super) id: &'static str,
    pub(super) text: &'static str,
    pub(super) computed_value: &'static str,
    pub(super) letter_spacing: f32,
    pub(super) is_min_content: bool,
}

pub(super) const LETTER_SPACING_SCENES: [LetterSpacingScene; 16] = [
    scene(
        "anonymous-letter-spacing-normal-min-content",
        "XX XX",
        "normal",
        0.0,
        true,
    ),
    scene(
        "anonymous-letter-spacing-normal-max-content",
        "XX XX",
        "normal",
        0.0,
        false,
    ),
    scene(
        "anonymous-letter-spacing-minus-one-min-content",
        "XX XX",
        "-1px",
        -1.0,
        true,
    ),
    scene(
        "anonymous-letter-spacing-minus-one-max-content",
        "XX XX",
        "-1px",
        -1.0,
        false,
    ),
    scene(
        "anonymous-letter-spacing-zero-min-content",
        "XX XX",
        "normal",
        0.0,
        true,
    ),
    scene(
        "anonymous-letter-spacing-zero-max-content",
        "XX XX",
        "normal",
        0.0,
        false,
    ),
    scene(
        "anonymous-letter-spacing-half-min-content",
        "XX XX",
        "0.5px",
        0.5,
        true,
    ),
    scene(
        "anonymous-letter-spacing-half-max-content",
        "XX XX",
        "0.5px",
        0.5,
        false,
    ),
    scene(
        "anonymous-letter-spacing-one-min-content",
        "XX XX",
        "1px",
        1.0,
        true,
    ),
    scene(
        "anonymous-letter-spacing-one-max-content",
        "XX XX",
        "1px",
        1.0,
        false,
    ),
    scene(
        "anonymous-letter-spacing-two-min-content",
        "XX XX",
        "2px",
        2.0,
        true,
    ),
    scene(
        "anonymous-letter-spacing-two-max-content",
        "XX XX",
        "2px",
        2.0,
        false,
    ),
    scene(
        "anonymous-letter-spacing-minus-one-single-min-content",
        "XX",
        "-1px",
        -1.0,
        true,
    ),
    scene(
        "anonymous-letter-spacing-minus-one-single-max-content",
        "XX",
        "-1px",
        -1.0,
        false,
    ),
    scene(
        "anonymous-letter-spacing-one-single-min-content",
        "XX",
        "1px",
        1.0,
        true,
    ),
    scene(
        "anonymous-letter-spacing-one-single-max-content",
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
    letter_spacing: f32,
    is_min_content: bool,
) -> LetterSpacingScene {
    LetterSpacingScene {
        id,
        text,
        computed_value,
        letter_spacing,
        is_min_content,
    }
}

pub(super) const LETTER_SPACING_SCENE_IDS: [&str; LETTER_SPACING_SCENES.len()] =
    scene_ids(LETTER_SPACING_SCENES);

const fn scene_ids(
    scenes: [LetterSpacingScene; LETTER_SPACING_SCENES.len()],
) -> [&'static str; LETTER_SPACING_SCENES.len()] {
    let mut ids = [""; LETTER_SPACING_SCENES.len()];
    let mut index = 0;
    while index < scenes.len() {
        ids[index] = scenes[index].id;
        index += 1;
    }
    ids
}

pub(super) fn find_scene(scene_id: &str) -> Option<LetterSpacingScene> {
    LETTER_SPACING_SCENES
        .iter()
        .copied()
        .find(|scene| scene.id == scene_id)
}

pub(super) fn validate_computed_letter_spacing(
    node_id: NodeId,
    properties: &BTreeMap<String, String>,
    scene_id: &str,
) -> Result<f32, RuntimeLayoutFailure> {
    let expected = find_scene(scene_id);
    let expected_serialization = expected.map_or("normal", |scene| scene.computed_value);
    let invalid = || {
        layout_failure(
            "unsupported_c114_anonymous_text_style",
            Some(node_id.get()),
            Some(format!(
                "letter-spacing: expected {expected_serialization}, got {:?}",
                properties.get("letter-spacing")
            )),
        )
    };
    let value = properties
        .get("letter-spacing")
        .filter(|value| value.as_str() == expected_serialization)
        .ok_or_else(invalid)?;
    let parsed = parse_computed_letter_spacing(value).ok_or_else(invalid)?;
    let expected_px = expected.map_or(0.0, |scene| scene.letter_spacing);
    if parsed.to_bits() != expected_px.to_bits() {
        return Err(invalid());
    }
    Ok(parsed)
}

fn parse_computed_letter_spacing(value: &str) -> Option<f32> {
    if value == "normal" {
        return Some(0.0);
    }
    let number = value.strip_suffix("px")?;
    if number.is_empty() || number.trim() != number {
        return None;
    }
    let number = number.parse::<f64>().ok()?;
    if !number.is_finite() || !(-2.0..=2.0).contains(&number) || number == 0.0 && value != "0px" {
        return None;
    }
    let value = number as f32;
    value.is_finite().then_some(value)
}

#[cfg(test)]
mod tests {
    use super::{
        LETTER_SPACING_SCENE_IDS, LETTER_SPACING_SCENES, find_scene, parse_computed_letter_spacing,
        validate_computed_letter_spacing,
    };
    use spinon_core::NodeId;
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn computed_letter_spacing_matrix_has_sixteen_unique_scenes() {
        let unique = LETTER_SPACING_SCENE_IDS
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        assert_eq!(LETTER_SPACING_SCENES.len(), 16);
        assert_eq!(unique.len(), LETTER_SPACING_SCENES.len());
        assert!(find_scene("anonymous-letter-spacing-minus-one-single-min-content").is_some());
        assert!(find_scene("anonymous-letter-spacing-unlisted-min-content").is_none());
    }

    #[test]
    fn computed_letter_spacing_requires_the_exact_serialization_and_finite_px_value() {
        let node_id = NodeId::new(1).unwrap();
        for item in LETTER_SPACING_SCENES {
            let properties =
                BTreeMap::from([("letter-spacing".to_owned(), item.computed_value.to_owned())]);
            assert_eq!(
                validate_computed_letter_spacing(node_id, &properties, item.id),
                Ok(item.letter_spacing),
                "{}",
                item.id
            );
        }
        let properties = BTreeMap::from([("letter-spacing".to_owned(), "normal".to_owned())]);
        assert_eq!(
            validate_computed_letter_spacing(node_id, &properties, "anonymous-normal-min-content"),
            Ok(0.0)
        );

        for (scene_id, value) in [
            ("anonymous-letter-spacing-minus-one-min-content", "-1.00px"),
            ("anonymous-letter-spacing-minus-one-min-content", "0px"),
            ("anonymous-letter-spacing-two-max-content", "2.01px"),
            ("anonymous-letter-spacing-two-max-content", "3px"),
            ("anonymous-letter-spacing-two-max-content", "infpx"),
            ("anonymous-letter-spacing-two-max-content", "NaNpx"),
            ("anonymous-letter-spacing-two-max-content", "2em"),
            ("anonymous-letter-spacing-minus-two-min-content", "-2px"),
            (
                "anonymous-letter-spacing-two-max-content",
                "calc(1px + 1px)",
            ),
            ("anonymous-letter-spacing-zero-min-content", "-0px"),
            ("anonymous-normal-min-content", "1px"),
            ("unknown-letter-spacing-scene", "1px"),
        ] {
            let properties = BTreeMap::from([("letter-spacing".to_owned(), value.to_owned())]);
            assert!(
                validate_computed_letter_spacing(node_id, &properties, scene_id).is_err(),
                "{scene_id}: {value}"
            );
        }
        assert_eq!(parse_computed_letter_spacing("normal"), Some(0.0));
        assert_eq!(parse_computed_letter_spacing("-2px"), Some(-2.0));
        assert_eq!(parse_computed_letter_spacing("2px"), Some(2.0));
        assert_eq!(parse_computed_letter_spacing("-0px"), None);
    }
}
