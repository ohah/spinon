use super::*;
use spinon_core::HostNodeHandle;

pub(super) fn payload_is_supported(
    tree: &HostDocumentSnapshot,
    nodes: &RuntimeGridNodes,
    snapshot: &ComputedStyleSnapshot,
    styles: &std::collections::BTreeMap<spinon_core::NodeId, &ComputedElementStyle>,
    payload: HostNodeHandle,
) -> bool {
    let node = payload.id();
    let Some(style) = styles.get(&node).copied() else {
        return false;
    };
    let unsupported = nodes.grid_containers.contains(&node)
        || style.properties.get("display").map(String::as_str) != Some("block")
        || style.properties.get("box-sizing").map(String::as_str) != Some("border-box")
        || style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || !automatic_limits(style)
        || style.layout_position != ComputedCssPosition::Static
        || style.layout_aspect_ratio.is_some()
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || !zero_border(style)
        || style.properties.get("font-family").map(String::as_str) != Some("monospace")
        || style.properties.get("font-size").map(String::as_str) != Some("10px")
        || style.properties.get("font-weight").map(String::as_str) != Some("400")
        || style.properties.get("font-style").map(String::as_str) != Some("normal")
        || style.properties.get("line-height").map(String::as_str) != Some("10px")
        || style
            .properties
            .get("white-space-collapse")
            .map(String::as_str)
            != Some("collapse")
        || style.properties.get("text-wrap-mode").map(String::as_str) != Some("wrap")
        || style.properties.get("word-break").map(String::as_str) != Some("normal")
        || style.properties.get("direction").map(String::as_str) != Some("ltr")
        || style.properties.get("text-align").map(String::as_str) != Some("start")
        || style.properties.get("overflow-wrap").map(String::as_str) != Some("normal")
        || style.properties.get("text-transform").map(String::as_str) != Some("none")
        || style.properties.get("writing-mode").map(String::as_str) != Some("horizontal-tb")
        || snapshot.writing_style_for(node).is_some_and(|writing| {
            writing
                != (
                    spinon_core::WritingMode::HorizontalTb,
                    spinon_core::TextOrientation::Mixed,
                )
        })
        || style
            .properties
            .get("text-orientation")
            .is_some_and(|value| value != "mixed")
        || !is_zero_length_property(style, "letter-spacing")
        || !is_zero_length_property(style, "word-spacing")
        || !is_zero_length_property(style, "text-indent")
        || style.font_size_css_px != 10.0;
    !unsupported && payload_has_supported_text(tree, payload)
}

pub(super) fn payload_has_supported_text(
    tree: &HostDocumentSnapshot,
    payload: HostNodeHandle,
) -> bool {
    let Some(children) = tree.children(payload) else {
        return false;
    };
    let children = children.collect::<Vec<_>>();
    let [text] = children.as_slice() else {
        return false;
    };
    matches!(tree.node(*text).map(|node| node.kind()), Some(HostNodeKind::Text(text))
        if supports_content_grammar(&text.to_string_lossy()))
}

fn supports_content_grammar(text: &str) -> bool {
    let bytes = text.as_bytes();
    if bytes.is_empty() || bytes.len() > 32 {
        return false;
    }

    let mut word_length = 0;
    for byte in bytes {
        match byte {
            b'A'..=b'Z' => {
                word_length += 1;
                if word_length > 6 {
                    return false;
                }
            }
            b' ' => {
                if word_length == 0 {
                    return false;
                }
                word_length = 0;
            }
            _ => return false,
        }
    }

    word_length > 0
}

fn is_zero_length_property(style: &ComputedElementStyle, property: &str) -> bool {
    style.properties.get(property).is_some_and(|value| {
        value == "normal"
            || value == "0"
            || value
                .strip_suffix("px")
                .and_then(|value| value.parse::<f32>().ok())
                .is_some_and(|value| value == 0.0)
    })
}

#[cfg(test)]
mod tests {
    use super::supports_content_grammar;

    #[test]
    fn content_grammar_accepts_ascii_word_and_byte_boundaries() {
        for text in ["A", "ABCDEF", "A B", "ABCDEF ABCDEF ABCDEF ABCDEF ABCD"] {
            assert!(
                supports_content_grammar(text),
                "expected acceptance: {text:?}"
            );
        }
    }

    #[test]
    fn content_grammar_rejects_non_ascii_and_malformed_spacing() {
        for text in [
            "",
            " ",
            " A",
            "A ",
            "A  B",
            "a",
            "1",
            "A?",
            "ABCDEFG",
            "ABCDEF ABCDEF ABCDEF ABCDEF ABCDE",
            "A\tB",
            "A\nB",
            "가",
        ] {
            assert!(
                !supports_content_grammar(text),
                "expected rejection: {text:?}"
            );
        }
    }
}
