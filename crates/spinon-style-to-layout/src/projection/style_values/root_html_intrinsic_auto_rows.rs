pub(super) fn supports_text_source(text: &str) -> bool {
    let mut has_token = false;
    for token in text.split([' ', '\t', '\n', '\r']) {
        if token.is_empty() {
            continue;
        }
        if !token.split('-').all(|segment| {
            !segment.is_empty() && segment.bytes().all(|byte| byte.is_ascii_alphanumeric())
        }) {
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
    fn direct_text_grammar_accepts_ascii_alphanumeric_hyphen_words_and_collapsible_whitespace() {
        for text in [
            "A",
            "1",
            "12",
            "A1",
            "BB2",
            "C33",
            "1A",
            "22B",
            "12345678",
            "A1B2C3D4",
            "A-A",
            "12-3",
            "A1-BB2",
            "AA-BB CC",
            "A-B C-DE",
            "A1-B2-C3",
            "12345678-12",
            "AA",
            "AB BBB A",
            "aa bb ccc",
            "AAAAAAAA",
            "A A",
            "A  AA",
            "A A A A A A",
            "  A   A A  A A A  ",
            "A\tA\tA",
            "A\nA\nA",
            "A\rA\rA",
            " \tA\t A\nA \r\n A \tA\nA  ",
            "  A-A   B-2 ",
            "\tA-A\r12-3\n",
        ] {
            assert!(supports_text_source(text), "{text:?}");
        }
        for text in [
            "",
            " ",
            "\t\n\r",
            "-A",
            "A-",
            "A--B",
            "A1--B",
            "A\u{2010}B",
            "A\u{2212}B",
            "A_B",
            "A/B",
            "1.2",
            "A!",
            "A_",
            "Ａ",
            "１２",
            "١٢",
            "é1",
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
