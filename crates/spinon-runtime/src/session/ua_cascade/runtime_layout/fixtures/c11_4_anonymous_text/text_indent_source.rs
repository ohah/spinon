use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind};

use super::{RuntimeLayoutFailure, layout_failure};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct AuthoredInlineTextIndent {
    pub(super) value: String,
    pub(super) line: u32,
    pub(super) column: u32,
}

/// HostDocument의 고정 inline style declaration에서 `text-indent` 원문을 얻습니다.
/// Stylo 0.22의 `servo` feature가 이 두 keyword 선언을 computed style에서 버려서,
/// opt-in fixture가 같은 document revision의 유일한 inline declaration을 보존합니다.
pub(super) fn authored_inline_text_indent(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
) -> Result<Option<AuthoredInlineTextIndent>, RuntimeLayoutFailure> {
    let node = snapshot
        .node(root)
        .ok_or_else(|| layout_failure("missing_node", Some(root.id().get()), None))?;
    let HostNodeKind::Element(element) = node.kind() else {
        return Err(layout_failure(
            "unsupported_c114_anonymous_text_style",
            Some(root.id().get()),
            Some("text-indent source is not an element".to_owned()),
        ));
    };
    let Some(style) = element.attributes().iter().find_map(|(name, value)| {
        (name.namespace().is_none() && name.local_name() == "style")
            .then(|| value.to_string_lossy())
    }) else {
        return Ok(None);
    };

    let mut text_indent = None;
    let mut byte_offset = 0;
    for declaration in style.split_inclusive(';') {
        let Some((name, value)) = declaration.split_once(':') else {
            byte_offset += declaration.len();
            continue;
        };
        if name.trim().eq_ignore_ascii_case("text-indent") {
            if text_indent.is_some() {
                return Err(layout_failure(
                    "unsupported_c114_anonymous_text_style",
                    Some(root.id().get()),
                    Some("duplicate inline text-indent declaration".to_owned()),
                ));
            }
            let leading_whitespace = name.len() - name.trim_start().len();
            let property_offset = byte_offset + leading_whitespace;
            let (line, column) = declaration_location(&style, property_offset);
            text_indent = Some(AuthoredInlineTextIndent {
                value: declaration_value(value),
                line,
                column,
            });
        }
        byte_offset += declaration.len();
    }
    Ok(text_indent)
}

fn declaration_value(value: &str) -> String {
    let value = value.trim();
    value.strip_suffix(';').unwrap_or(value).trim().to_owned()
}

fn declaration_location(style: &str, property_offset: usize) -> (u32, u32) {
    let prefix = &style[..property_offset];
    let line = prefix.bytes().filter(|byte| *byte == b'\n').count() as u32;
    let column = prefix
        .rsplit('\n')
        .next()
        .unwrap_or_default()
        .encode_utf16()
        .count() as u32
        + 1;
    (line, column)
}

#[cfg(test)]
mod tests {
    use super::{declaration_location, declaration_value};

    #[test]
    fn declaration_value_removes_only_the_declaration_terminator() {
        assert_eq!(declaration_value(" 10px each-line; "), "10px each-line");
        assert_eq!(
            declaration_value("10px hanging each-line"),
            "10px hanging each-line"
        );
        assert_eq!(declaration_value("10px each-line;;"), "10px each-line;");
    }

    #[test]
    fn declaration_location_uses_one_based_utf16_columns() {
        assert_eq!(declaration_location("a\nA🟦", 7), (1, 4));
    }
}
