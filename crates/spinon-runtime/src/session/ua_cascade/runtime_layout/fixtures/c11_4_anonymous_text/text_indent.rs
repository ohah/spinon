use std::collections::BTreeMap;

use spinon_core::NodeId;
use spinon_layout::TextIndentFlags;
use spinon_style::CascadeDiagnostic;

use super::{RuntimeLayoutFailure, layout_failure};

#[path = "text_indent/scenes.rs"]
mod scenes;
#[cfg(test)]
use scenes::TEXT_INDENT_SCENES;

pub(super) const TEXT_INDENT_SCENE_IDS: [&str; 50] = scenes::TEXT_INDENT_SCENE_IDS;

pub(super) fn find_scene(scene_id: &str) -> Option<TextIndentScene> {
    scenes::find_scene(scene_id)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct TextIndentScene {
    pub(super) id: &'static str,
    pub(super) text: &'static str,
    pub(super) computed_value: &'static str,
    pub(super) text_indent: f32,
    pub(super) text_indent_flags: TextIndentFlags,
    pub(super) is_min_content: bool,
    pub(super) is_pre_line: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct ComputedTextIndent {
    pub(super) value: f32,
    pub(super) flags: TextIndentFlags,
}

pub(super) fn is_single_expected_stylo_flags_diagnostic(
    diagnostics: &[CascadeDiagnostic],
    node_id: NodeId,
    computed_value: &str,
    line: u32,
    column: u32,
) -> bool {
    if !matches!(
        computed_value,
        "10px each-line" | "10px hanging" | "10px hanging each-line"
    ) {
        return false;
    }
    let [diagnostic] = diagnostics else {
        return false;
    };
    diagnostic.source_id == format!("inline:{}", node_id.get())
        && diagnostic.node_id == Some(node_id)
        && diagnostic.diagnostic.line == line
        && diagnostic.diagnostic.column == column
        && [
            format!(
                "Unsupported property declaration: 'text-indent:{computed_value}', OtherInvalidValue"
            ),
            format!(
                "Unsupported property declaration: 'text-indent:{computed_value};', OtherInvalidValue"
            ),
        ]
        .contains(&diagnostic.diagnostic.message)
}

pub(super) fn validate_computed_text_indent(
    node_id: NodeId,
    properties: &BTreeMap<String, String>,
    scene_id: &str,
    authored_inline_value: Option<&str>,
    stylo_flags_fallback_allowed: bool,
) -> Result<ComputedTextIndent, RuntimeLayoutFailure> {
    let expected = find_scene(scene_id);
    let expected_serialization = expected.map_or("0px", |scene| scene.computed_value);
    let expected_flags =
        expected.map_or_else(TextIndentFlags::default, |scene| scene.text_indent_flags);
    let expected_px = expected.map_or(0.0, |scene| scene.text_indent);
    let invalid = || {
        layout_failure(
            "unsupported_c114_anonymous_text_style",
            Some(node_id.get()),
            Some(format!(
                "text-indent: expected {expected_serialization} in the style snapshot or the fixed inline compatibility source, got computed={:?}, authored={authored_inline_value:?}",
                properties.get("text-indent"),
            )),
        )
    };
    let computed_value = properties.get("text-indent").map(String::as_str);
    let authored_matches = authored_inline_value == Some(expected_serialization);
    let flags_are_present = expected_flags != TextIndentFlags::default();
    let stylo_keyword_fallback = flags_are_present
        && stylo_flags_fallback_allowed
        && computed_value == Some("0px")
        && authored_matches;
    let computed_matches = computed_value == Some(expected_serialization);
    if !computed_matches && !stylo_keyword_fallback || flags_are_present && !authored_matches {
        return Err(invalid());
    }
    let value = if computed_matches {
        expected_serialization
    } else {
        authored_inline_value.ok_or_else(invalid)?
    };
    let parsed = parse_computed_text_indent(value).ok_or_else(invalid)?;
    if parsed.value.to_bits() != expected_px.to_bits() || parsed.flags != expected_flags {
        return Err(invalid());
    }
    Ok(parsed)
}

fn parse_computed_text_indent(value: &str) -> Option<ComputedTextIndent> {
    let mut tokens = value.split_ascii_whitespace();
    let number = tokens.next()?.strip_suffix("px")?;
    if number.is_empty() || number.trim() != number {
        return None;
    }
    let number = number.parse::<f64>().ok()?;
    if !number.is_finite() || !(-10.0..=10.0).contains(&number) || number == 0.0 && value != "0px" {
        return None;
    }
    let value = number as f32;
    if !value.is_finite() {
        return None;
    }
    let mut flags = TextIndentFlags::default();
    let mut previous_flag_order = 0;
    for token in tokens {
        match token {
            "hanging" if !flags.hanging && previous_flag_order < 1 => {
                flags.hanging = true;
                previous_flag_order = 1;
            }
            "each-line" if !flags.each_line && previous_flag_order < 2 => {
                flags.each_line = true;
                previous_flag_order = 2;
            }
            _ => return None,
        }
    }
    Some(ComputedTextIndent { value, flags })
}

#[cfg(test)]
mod tests {
    use super::{
        ComputedTextIndent, TEXT_INDENT_SCENE_IDS, TEXT_INDENT_SCENES, TextIndentFlags, find_scene,
        is_single_expected_stylo_flags_diagnostic, parse_computed_text_indent,
        validate_computed_text_indent,
    };
    use spinon_core::NodeId;
    use spinon_style::{CascadeDiagnostic, CssParseDiagnostic};
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn computed_text_indent_matrix_has_fifty_unique_scenes() {
        let unique = TEXT_INDENT_SCENE_IDS
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        assert_eq!(TEXT_INDENT_SCENES.len(), 50);
        assert_eq!(unique.len(), TEXT_INDENT_SCENES.len());
        assert!(find_scene("text-indent-ten-collapsed-three-words-max-content").is_some());
        assert!(find_scene("text-indent-negative-ten-first-word-short-min-content").is_some());
        assert!(find_scene("text-indent-flags-forced-min-content-hanging-each-line").is_some());
        assert!(find_scene("text-indent-unlisted").is_none());
    }

    #[test]
    fn computed_text_indent_accepts_only_the_fixed_scene_values() {
        let node_id = NodeId::new(1).unwrap();
        for scene in TEXT_INDENT_SCENES {
            let properties =
                BTreeMap::from([("text-indent".to_owned(), scene.computed_value.to_owned())]);
            assert_eq!(
                validate_computed_text_indent(
                    node_id,
                    &properties,
                    scene.id,
                    Some(scene.computed_value),
                    false,
                ),
                Ok(ComputedTextIndent {
                    value: scene.text_indent,
                    flags: scene.text_indent_flags
                }),
                "{}",
                scene.id
            );
        }
        for value in [
            "-11px",
            "11px",
            "-0px",
            "5%",
            "10px each-line hanging",
            "10px hanging each-line hanging",
            "10px hanging unknown",
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
                validate_computed_text_indent(node_id, &properties, five.id, Some(value), false)
                    .is_err(),
                "accepted {value} for {}",
                five.id
            );
        }
        let default = BTreeMap::from([("text-indent".to_owned(), "0px".to_owned())]);
        assert_eq!(
            validate_computed_text_indent(
                node_id,
                &default,
                "anonymous-normal-min-content",
                None,
                false,
            ),
            Ok(ComputedTextIndent {
                value: 0.0,
                flags: TextIndentFlags::default()
            })
        );
        let changed = BTreeMap::from([("text-indent".to_owned(), "5px".to_owned())]);
        assert!(
            validate_computed_text_indent(
                node_id,
                &changed,
                "anonymous-normal-min-content",
                None,
                false,
            )
            .is_err()
        );

        let flags = find_scene("text-indent-flags-soft-min-content-each-line").unwrap();
        let stylo_servo_fallback = BTreeMap::from([("text-indent".to_owned(), "0px".to_owned())]);
        assert!(
            validate_computed_text_indent(
                node_id,
                &stylo_servo_fallback,
                flags.id,
                Some(flags.computed_value),
                false,
            )
            .is_err(),
            "fallback must reject a missing Stylo diagnostic"
        );
        assert_eq!(
            validate_computed_text_indent(
                node_id,
                &stylo_servo_fallback,
                flags.id,
                Some(flags.computed_value),
                true,
            ),
            Ok(ComputedTextIndent {
                value: 10.0,
                flags: TextIndentFlags {
                    each_line: true,
                    hanging: false,
                },
            })
        );
        assert!(
            validate_computed_text_indent(node_id, &stylo_servo_fallback, flags.id, None, true,)
                .is_err()
        );
        assert!(
            validate_computed_text_indent(
                node_id,
                &stylo_servo_fallback,
                flags.id,
                Some("10px hanging each-line"),
                true,
            )
            .is_err()
        );
    }

    #[test]
    fn stylo_flags_diagnostic_exception_is_single_exact_and_source_bound() {
        let node_id = NodeId::new(501).unwrap();
        let expected = CascadeDiagnostic {
            source_id: "inline:501".to_owned(),
            node_id: Some(node_id),
            diagnostic: CssParseDiagnostic {
                line: 0,
                column: 501,
                message: "Unsupported property declaration: 'text-indent:10px each-line', OtherInvalidValue".to_owned(),
            },
        };
        assert!(is_single_expected_stylo_flags_diagnostic(
            std::slice::from_ref(&expected),
            node_id,
            "10px each-line",
            0,
            501,
        ));

        let mut with_declaration_terminator = expected.clone();
        with_declaration_terminator.diagnostic.message =
            "Unsupported property declaration: 'text-indent:10px each-line;', OtherInvalidValue"
                .to_owned();
        assert!(is_single_expected_stylo_flags_diagnostic(
            &[with_declaration_terminator],
            node_id,
            "10px each-line",
            0,
            501,
        ));

        let mut wrong_source = expected.clone();
        wrong_source.source_id = "author:sheet-1".to_owned();
        assert!(!is_single_expected_stylo_flags_diagnostic(
            &[wrong_source],
            node_id,
            "10px each-line",
            0,
            501,
        ));

        let mut wrong_message = expected.clone();
        wrong_message.diagnostic.message.push_str("; extra");
        assert!(!is_single_expected_stylo_flags_diagnostic(
            &[wrong_message],
            node_id,
            "10px each-line",
            0,
            501,
        ));

        let mut extra_diagnostic = expected.clone();
        extra_diagnostic.diagnostic.message =
            "Unsupported property declaration: 'color:bad', OtherInvalidValue".to_owned();
        assert!(!is_single_expected_stylo_flags_diagnostic(
            &[expected.clone(), extra_diagnostic],
            node_id,
            "10px each-line",
            0,
            501,
        ));
        assert!(!is_single_expected_stylo_flags_diagnostic(
            &[expected],
            node_id,
            "10px each-line",
            0,
            502,
        ));
    }
}
