pub(super) fn supports_text_source(text: &str) -> bool {
    let mut has_token = false;
    for token in text.split([' ', '\t', '\n', '\r']) {
        if token.is_empty() {
            continue;
        }
        if token != "A" {
            return false;
        }
        has_token = true;
    }
    has_token
}

pub(super) fn is_whitespace_text(text: &str) -> bool {
    text.chars()
        .all(|character| matches!(character, '\t' | '\n' | '\u{000c}' | '\r' | ' '))
}

pub(super) fn required_rows(
    visible_items: usize,
    explicit_columns: usize,
    explicit_rows: usize,
) -> Option<usize> {
    if explicit_columns == 0 {
        return None;
    }
    Some(visible_items.div_ceil(explicit_columns).max(explicit_rows))
}

pub(super) fn exceeds_track_limit(
    visible_items: usize,
    explicit_columns: usize,
    explicit_rows: usize,
    maximum_tracks: usize,
) -> Option<bool> {
    required_rows(visible_items, explicit_columns, explicit_rows)
        .map(|required| required > maximum_tracks)
}

#[cfg(test)]
mod tests {
    use super::{exceeds_track_limit, is_whitespace_text, required_rows, supports_text_source};

    #[test]
    fn direct_text_grammar_accepts_a_tokens_separated_by_collapsible_ascii_whitespace() {
        for text in [
            "A",
            "A A",
            "A A A A A A",
            "  A   A A  A A A  ",
            "A\tA\tA",
            "A\nA\nA",
            "A\rA\rA",
            " \tA\t A\nA \r\n A \tA\nA  ",
        ] {
            assert!(supports_text_source(text), "{text:?}");
        }
        for text in [
            "",
            " ",
            "\t\n\r",
            "AA",
            "A  AA",
            "B",
            "Ａ",
            "A\u{000c}A",
            "A\u{000b}A",
            "A\u{00a0}A",
            "A\u{0085}A",
            "A\u{2003}A",
        ] {
            assert!(!supports_text_source(text), "{text:?}");
        }
    }

    #[test]
    fn direct_grid_whitespace_is_classified_without_accepting_other_text() {
        assert!(is_whitespace_text(" \t\n\u{000c}\r"));
        assert!(!is_whitespace_text("A"));
        assert!(!is_whitespace_text("\u{00a0}"));
    }

    #[test]
    fn item_count_maps_to_bounded_implicit_rows_without_rounding_down() {
        const MAX_TRACKS: usize = 10_000;
        assert_eq!(required_rows(MAX_TRACKS, 1, 1), Some(MAX_TRACKS));
        assert_eq!(required_rows(MAX_TRACKS + 1, 1, 1), Some(MAX_TRACKS + 1));
        assert_eq!(required_rows(10_001, 2, 1), Some(5_001));
        assert_eq!(required_rows(1, 0, 1), None);
        assert_eq!(
            exceeds_track_limit(MAX_TRACKS, 1, 1, MAX_TRACKS),
            Some(false)
        );
        assert_eq!(
            exceeds_track_limit(MAX_TRACKS + 1, 1, 1, MAX_TRACKS),
            Some(true)
        );
        assert_eq!(exceeds_track_limit(10_001, 2, 1, MAX_TRACKS), Some(false));
        assert_eq!(exceeds_track_limit(1, 0, 1, MAX_TRACKS), None);
    }
}
