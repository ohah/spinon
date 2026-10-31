use std::collections::BTreeMap;

use spinon_core::NodeId;

use super::{RuntimeLayoutFailure, layout_failure};

const COMPUTED_FONT_SIZE_PX: f64 = 10.0;

pub(super) const LINE_HEIGHT_SCENE_IDS: [&str; 16] = [
    "line-height-normal-two-lines",
    "line-height-zero-number",
    "line-height-half-number",
    "line-height-one-number",
    "line-height-one-point-five-number",
    "line-height-two-number",
    "line-height-zero-length",
    "line-height-five-length",
    "line-height-twelve-point-five-length",
    "line-height-one-hundred-twenty-five-percent",
    "line-height-one-point-two-five-em",
    "line-height-calc-length",
    "line-height-normal-single-line",
    "line-height-half-number-single-line",
    "line-height-one-point-five-soft-wrap",
    "line-height-zero-number-soft-wrap",
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum ComputedLineHeight {
    Normal,
    AbsolutePx(f32),
}

pub(super) fn line_height_text(scene_id: &str) -> Option<&'static str> {
    match scene_id {
        "line-height-normal-two-lines"
        | "line-height-zero-number"
        | "line-height-half-number"
        | "line-height-one-number"
        | "line-height-one-point-five-number"
        | "line-height-two-number"
        | "line-height-zero-length"
        | "line-height-five-length"
        | "line-height-twelve-point-five-length"
        | "line-height-one-hundred-twenty-five-percent"
        | "line-height-one-point-two-five-em"
        | "line-height-calc-length" => Some("XX\nXX"),
        "line-height-normal-single-line" | "line-height-half-number-single-line" => Some("XX"),
        "line-height-one-point-five-soft-wrap" | "line-height-zero-number-soft-wrap" => {
            Some("XX XX")
        }
        _ => None,
    }
}

pub(super) fn validate_computed_line_height(
    node_id: NodeId,
    properties: &BTreeMap<String, String>,
    scene_id: &str,
) -> Result<ComputedLineHeight, RuntimeLayoutFailure> {
    let invalid = || {
        layout_failure(
            "unsupported_c114_anonymous_text_style",
            Some(node_id.get()),
            Some("line-height".to_owned()),
        )
    };
    let value = properties.get("line-height").ok_or_else(invalid)?;
    if Some(value.as_str()) != expected_line_height_serialization(scene_id) {
        return Err(invalid());
    }
    parse_computed_line_height(value).ok_or_else(invalid)
}

fn expected_line_height_serialization(scene_id: &str) -> Option<&'static str> {
    match scene_id {
        "line-height-normal-two-lines" | "line-height-normal-single-line" => Some("normal"),
        "line-height-zero-number" | "line-height-zero-number-soft-wrap" => Some("0px"),
        "line-height-half-number" | "line-height-half-number-single-line" => Some("5px"),
        "line-height-one-number" => Some("10px"),
        "line-height-one-point-five-number" | "line-height-one-point-five-soft-wrap" => {
            Some("15px")
        }
        "line-height-two-number" => Some("20px"),
        "line-height-zero-length" => Some("0px"),
        "line-height-five-length" => Some("5px"),
        "line-height-twelve-point-five-length"
        | "line-height-one-hundred-twenty-five-percent"
        | "line-height-one-point-two-five-em"
        | "line-height-calc-length" => Some("12.5px"),
        scene_id if !LINE_HEIGHT_SCENE_IDS.contains(&scene_id) => Some("10px"),
        _ => None,
    }
}

fn parse_computed_line_height(value: &str) -> Option<ComputedLineHeight> {
    if value == "normal" {
        return Some(ComputedLineHeight::Normal);
    }

    let (number, is_length) = if let Some(number) = value.strip_suffix("px") {
        (number, true)
    } else {
        (value, false)
    };
    if number.is_empty() || number.trim() != number {
        return None;
    }
    let number = number.parse::<f64>().ok()?;
    if !number.is_finite() || number < 0.0 {
        return None;
    }
    let resolved_px = if is_length {
        number
    } else {
        number * COMPUTED_FONT_SIZE_PX
    };
    if !resolved_px.is_finite() || resolved_px > f64::from(f32::MAX) {
        return None;
    }
    let resolved_px = resolved_px as f32;
    resolved_px
        .is_finite()
        .then_some(ComputedLineHeight::AbsolutePx(resolved_px))
}

#[cfg(test)]
mod tests {
    use super::{
        ComputedLineHeight, LINE_HEIGHT_SCENE_IDS, line_height_text, parse_computed_line_height,
        validate_computed_line_height,
    };
    use spinon_core::NodeId;
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn computed_line_height_scene_matrix_is_unique_and_has_expected_text_shapes() {
        let unique = LINE_HEIGHT_SCENE_IDS
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        assert_eq!(LINE_HEIGHT_SCENE_IDS.len(), 16);
        assert_eq!(unique.len(), LINE_HEIGHT_SCENE_IDS.len());
        for scene_id in LINE_HEIGHT_SCENE_IDS {
            assert!(line_height_text(scene_id).is_some(), "{scene_id}");
        }
        assert!(line_height_text("line-height-unlisted").is_none());
    }

    #[test]
    fn computed_line_height_accepts_only_finite_nonnegative_stylo_values() {
        for (serialized, expected) in [
            ("normal", ComputedLineHeight::Normal),
            ("0", ComputedLineHeight::AbsolutePx(0.0)),
            ("0px", ComputedLineHeight::AbsolutePx(0.0)),
            ("5", ComputedLineHeight::AbsolutePx(50.0)),
            ("0.5", ComputedLineHeight::AbsolutePx(5.0)),
            ("1", ComputedLineHeight::AbsolutePx(10.0)),
            ("1.5", ComputedLineHeight::AbsolutePx(15.0)),
            ("2", ComputedLineHeight::AbsolutePx(20.0)),
            ("5px", ComputedLineHeight::AbsolutePx(5.0)),
            ("12.5px", ComputedLineHeight::AbsolutePx(12.5)),
        ] {
            assert_eq!(
                parse_computed_line_height(serialized),
                Some(expected),
                "{serialized}"
            );
        }

        for unsupported in [
            "",
            "normal ",
            "125%",
            "1.25em",
            "calc(12.5px)",
            "-1",
            "-1px",
            "NaN",
            "inf",
            "1e100px",
            "1e100",
            "5PX",
            "5 px",
            " 5px",
            "5px ",
        ] {
            assert_eq!(
                parse_computed_line_height(unsupported),
                None,
                "{unsupported}"
            );
        }

        let node_id = NodeId::new(1).unwrap();
        for (scene_id, value) in [
            ("line-height-normal-two-lines", "normal"),
            ("line-height-zero-number", "0px"),
            ("line-height-half-number", "5px"),
            ("line-height-calc-length", "12.5px"),
        ] {
            let properties = BTreeMap::from([("line-height".to_owned(), value.to_owned())]);
            assert!(
                validate_computed_line_height(node_id, &properties, scene_id).is_ok(),
                "computed value {value} for {scene_id}"
            );
        }
        for value in ["125%", "-1px", "NaN", "12.5px garbage"] {
            let properties = BTreeMap::from([("line-height".to_owned(), value.to_owned())]);
            assert!(
                validate_computed_line_height(node_id, &properties, "line-height-zero-number",)
                    .is_err(),
                "unsupported computed value {value}"
            );
        }
    }

    #[test]
    fn computed_line_height_rejects_values_that_do_not_match_the_scene_fixture() {
        let node = NodeId::new(7).unwrap();
        for (scene_id, expected, mismatched) in [
            ("line-height-normal-two-lines", "normal", "10px"),
            ("line-height-zero-number", "0px", "0"),
            ("line-height-half-number", "5px", "10px"),
            ("line-height-calc-length", "12.5px", "calc(10px + 2.5px)"),
            ("anonymous-normal-min-content", "10px", "normal"),
        ] {
            let properties = BTreeMap::from([("line-height".to_owned(), expected.to_owned())]);
            assert!(validate_computed_line_height(node, &properties, scene_id).is_ok());
            let properties = BTreeMap::from([("line-height".to_owned(), mismatched.to_owned())]);
            assert_eq!(
                validate_computed_line_height(node, &properties, scene_id),
                Err(super::layout_failure(
                    "unsupported_c114_anonymous_text_style",
                    Some(node.get()),
                    Some("line-height".to_owned()),
                )),
                "{scene_id} accepted a mismatched computed value"
            );
        }
    }
}
