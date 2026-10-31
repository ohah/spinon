use super::{close_to, engine, paragraph};
use crate::{TabSize, TextLayoutError, TextSource, WhiteSpaceMode, WordBreakMode};

fn pre(text: &str) -> crate::ParagraphInput {
    let mut input = paragraph(
        text,
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Pre,
        WordBreakMode::Normal,
    );
    input.style.direction = crate::TextDirection::Ltr;
    input
}

fn pre_with_tab_size(text: &str, tab_size: TabSize) -> crate::ParagraphInput {
    let mut input = pre(text);
    input.style.white_space = WhiteSpaceMode::PreWithTabSize(tab_size);
    input
}

#[test]
fn pre_preserves_spaces_and_disables_soft_wrapping() {
    let mut engine = engine();
    let input = pre("XX  XX");
    let sizes = engine.intrinsic_inline_sizes(&input).unwrap();
    close_to(sizes.min_content(), 60.0);
    close_to(sizes.max_content(), 60.0);
    assert_eq!(sizes.min_content_height(), 10.0);
    assert_eq!(sizes.max_content_height(), 10.0);

    let layout = engine.layout_paragraph(&input, 25.0).unwrap();
    assert_eq!(layout.text, "XX  XX");
    assert_eq!(layout.lines.len(), 1);
    close_to(layout.lines[0].advance, 60.0);
    close_to(layout.width, 60.0);
    assert_eq!(layout.height, 10.0);
}

#[test]
fn pre_keeps_space_boundaries_unbreakable_for_min_content() {
    let mut engine = engine();
    let input = pre("XX XX");
    let sizes = engine.intrinsic_inline_sizes(&input).unwrap();
    close_to(sizes.min_content(), 50.0);
    close_to(sizes.max_content(), 50.0);
    assert_eq!(sizes.min_content_height(), 10.0);
    assert_eq!(sizes.max_content_height(), 10.0);
}

#[test]
fn pre_preserves_forced_segment_breaks_and_source_provenance() {
    let mut engine = engine();
    let crlf = pre("XX\r\nXX");
    let layout = engine.layout_paragraph(&crlf, 25.0).unwrap();

    assert_eq!(layout.text, "XX\nXX");
    assert_eq!(layout.lines.len(), 2);
    close_to(layout.lines[0].advance, 20.0);
    close_to(layout.lines[1].advance, 20.0);
    close_to(layout.width, 20.0);
    assert_eq!(layout.height, 20.0);
    assert_eq!(layout.sources[2].source_utf16, 2..3);
    assert_eq!(layout.sources[3].source_utf16, 3..4);
    assert_eq!(layout.sources[2].paragraph_utf8, 2..3);
    assert_eq!(layout.sources[3].paragraph_utf8, 2..3);

    let form_feed = engine.layout_paragraph(&pre("X\u{000c}X"), 25.0).unwrap();
    assert_eq!(form_feed.text, "X\nX");
    assert_eq!(form_feed.lines.len(), 2);
}

#[test]
fn pre_tabs_advance_to_the_next_default_eight_space_stop() {
    let mut engine = engine();
    for (text, normalized, advance) in [
        ("XX\tXX", "XX      XX", 100.0),
        ("\tX", "        X", 90.0),
        ("XXXXXXXX\tX", "XXXXXXXX        X", 170.0),
        ("XX\t\tX", "XX              X", 170.0),
        ("XX\t", "XX      ", 80.0),
    ] {
        let input = pre(text);
        let layout = engine.layout_paragraph(&input, 250.0).unwrap();

        assert_eq!(layout.text, normalized, "input {text:?}");
        assert_eq!(layout.lines.len(), 1, "input {text:?}");
        close_to(layout.lines[0].advance, advance);
        close_to(layout.width, advance);
    }
}

#[test]
fn pre_tabs_reset_after_crlf_and_preserve_source_ranges() {
    let mut engine = engine();
    let layout = engine.layout_paragraph(&pre("X\t\r\nX\tX"), 200.0).unwrap();

    assert_eq!(layout.text, "X       \nX       X");
    assert_eq!(layout.lines.len(), 2);
    close_to(layout.lines[0].advance, 80.0);
    close_to(layout.lines[1].advance, 90.0);
    close_to(layout.width, 90.0);
    assert_eq!(layout.sources[1].source_utf16, 1..2);
    assert_eq!(layout.sources[1].paragraph_utf8, 1..8);
    assert_eq!(layout.sources[2].source_utf16, 2..3);
    assert_eq!(layout.sources[3].source_utf16, 3..4);
    assert_eq!(layout.sources[2].paragraph_utf8, 8..9);
    assert_eq!(layout.sources[3].paragraph_utf8, 8..9);
    assert_eq!(layout.sources[5].source_utf16, 5..6);
    assert_eq!(layout.sources[5].paragraph_utf8, 10..17);
}

#[test]
fn pre_tabs_keep_intrinsic_width_unwrapped_at_a_narrow_available_width() {
    let mut engine = engine();
    let input = pre("XX\tXX");
    let sizes = engine.intrinsic_inline_sizes(&input).unwrap();

    close_to(sizes.min_content(), 100.0);
    close_to(sizes.max_content(), 100.0);
    assert_eq!(sizes.min_content_height(), 10.0);
    assert_eq!(sizes.max_content_height(), 10.0);

    let layout = engine.layout_paragraph(&input, 25.0).unwrap();
    assert_eq!(layout.text, "XX      XX");
    assert_eq!(layout.lines.len(), 1);
    close_to(layout.lines[0].advance, 100.0);
    close_to(layout.width, 100.0);
    assert_eq!(layout.height, 10.0);
    assert_eq!(layout.sources[2].source_utf16, 2..3);
    assert_eq!(layout.sources[2].paragraph_utf8, 2..8);
}

#[test]
fn pre_tabs_keep_source_nodes_when_text_sources_are_joined() {
    let mut engine = engine();
    let mut input = pre("XX\tXX");
    let first = input.sources[0].clone();
    input.sources = vec![
        TextSource {
            node: first.node,
            inline_ancestors: Vec::new(),
            text: "XX".to_owned(),
        },
        TextSource {
            node: spinon_core::NodeId::new(3).unwrap(),
            inline_ancestors: Vec::new(),
            text: "\tXX".to_owned(),
        },
    ];
    let layout = engine.layout_paragraph(&input, 200.0).unwrap();

    assert_eq!(layout.text, "XX      XX");
    assert_eq!(layout.sources[1].node, first.node);
    assert_eq!(layout.sources[2].node, spinon_core::NodeId::new(3).unwrap());
    assert_eq!(layout.sources[2].source_utf16, 0..1);
    assert_eq!(layout.sources[2].paragraph_utf8, 2..8);
}

#[test]
fn pre_tabs_fail_closed_outside_the_fixed_profile() {
    let mut engine = engine();
    let mut non_ahem = pre("XX\tXX");
    non_ahem.style.font_family = "Roboto".to_owned();
    let unsupported_character = pre("YX\tX");
    let mut inline_ancestor = pre("XX\tXX");
    inline_ancestor.sources[0]
        .inline_ancestors
        .push(spinon_core::NodeId::new(9).unwrap());

    for input in [non_ahem, unsupported_character, inline_ancestor] {
        assert!(matches!(
            engine.intrinsic_inline_sizes(&input),
            Err(TextLayoutError::UnsupportedPreTabProfile(node)) if node == input.node
        ));
        assert!(matches!(
            engine.layout_paragraph(&input, 100.0),
            Err(TextLayoutError::UnsupportedPreTabProfile(node)) if node == input.node
        ));
    }
}

#[test]
fn pre_numeric_tab_sizes_advance_to_the_next_stop() {
    let mut engine = engine();
    for (tab_size, cases) in [
        (
            TabSize::One,
            vec![("\tX", " X", 20.0), ("X\tX", "X X", 30.0)],
        ),
        (
            TabSize::Two,
            vec![
                ("XX\tXX", "XX  XX", 60.0),
                ("\tX", "  X", 30.0),
                ("XXXX\tX", "XXXX  X", 70.0),
                ("XX\t\tX", "XX    X", 70.0),
                ("XX\t", "XX  ", 40.0),
            ],
        ),
        (
            TabSize::Four,
            vec![("\tX", "    X", 50.0), ("XXXX\tX", "XXXX    X", 90.0)],
        ),
    ] {
        for (text, normalized, advance) in cases {
            let input = pre_with_tab_size(text, tab_size);
            let layout = engine.layout_paragraph(&input, 25.0).unwrap();

            assert_eq!(
                layout.text, normalized,
                "tab size {tab_size:?}, input {text:?}"
            );
            assert_eq!(
                layout.lines.len(),
                1,
                "tab size {tab_size:?}, input {text:?}"
            );
            close_to(layout.lines[0].advance, advance);
            close_to(layout.width, advance);
        }
    }
}

#[test]
fn pre_numeric_tab_sizes_reset_after_crlf_and_keep_tab_provenance() {
    let mut engine = engine();
    let lf = engine
        .layout_paragraph(&pre_with_tab_size("XX\tXX\nX\tX", TabSize::Two), 25.0)
        .unwrap();
    assert_eq!(lf.text, "XX  XX\nX X");
    assert_eq!(lf.lines.len(), 2);
    close_to(lf.lines[0].advance, 60.0);
    close_to(lf.lines[1].advance, 30.0);

    let input = pre_with_tab_size("X\t\r\nX\tX", TabSize::Two);
    let layout = engine.layout_paragraph(&input, 25.0).unwrap();

    assert_eq!(layout.text, "X \nX X");
    assert_eq!(layout.lines.len(), 2);
    close_to(layout.lines[0].advance, 20.0);
    close_to(layout.lines[1].advance, 30.0);
    close_to(layout.width, 30.0);
    assert_eq!(layout.sources[1].source_utf16, 1..2);
    assert_eq!(layout.sources[1].paragraph_utf8, 1..2);
    assert_eq!(layout.sources[2].source_utf16, 2..3);
    assert_eq!(layout.sources[3].source_utf16, 3..4);
    assert_eq!(layout.sources[2].paragraph_utf8, 2..3);
    assert_eq!(layout.sources[3].paragraph_utf8, 2..3);
    assert_eq!(layout.sources[5].source_utf16, 5..6);
    assert_eq!(layout.sources[5].paragraph_utf8, 4..5);
}

#[test]
fn pre_numeric_tabs_keep_min_content_unwrapped_and_joined_source_ownership() {
    let mut engine = engine();
    let input = pre_with_tab_size("XX\tXX", TabSize::Two);
    let sizes = engine.intrinsic_inline_sizes(&input).unwrap();
    close_to(sizes.min_content(), 60.0);
    close_to(sizes.max_content(), 60.0);
    let layout = engine.layout_paragraph(&input, 25.0).unwrap();
    assert_eq!(layout.text, "XX  XX");
    assert_eq!(layout.lines.len(), 1);
    close_to(layout.lines[0].advance, 60.0);
    close_to(layout.width, 60.0);

    let first = input.sources[0].clone();
    let mut joined = pre_with_tab_size("XX\tXX", TabSize::Two);
    joined.sources = vec![
        TextSource {
            node: first.node,
            inline_ancestors: Vec::new(),
            text: "XX".to_owned(),
        },
        TextSource {
            node: spinon_core::NodeId::new(3).unwrap(),
            inline_ancestors: Vec::new(),
            text: "\tXX".to_owned(),
        },
    ];
    let joined_layout = engine.layout_paragraph(&joined, 100.0).unwrap();
    assert_eq!(joined_layout.text, "XX  XX");
    assert_eq!(
        joined_layout.sources[2].node,
        spinon_core::NodeId::new(3).unwrap()
    );
    assert_eq!(joined_layout.sources[2].source_utf16, 0..1);
    assert_eq!(joined_layout.sources[2].paragraph_utf8, 2..4);
}

#[test]
fn pre_numeric_tab_sizes_reject_text_outside_the_fixed_profile() {
    let mut engine = engine();
    let mut non_ahem = pre_with_tab_size("X\tX", TabSize::Two);
    non_ahem.style.font_family = "Roboto".to_owned();
    let unsupported_character = pre_with_tab_size("Y\tX", TabSize::Four);

    for input in [non_ahem, unsupported_character] {
        assert!(matches!(
            engine.intrinsic_inline_sizes(&input),
            Err(TextLayoutError::UnsupportedPreTabProfile(node)) if node == input.node
        ));
        assert!(matches!(
            engine.layout_paragraph(&input, 100.0),
            Err(TextLayoutError::UnsupportedPreTabProfile(node)) if node == input.node
        ));
    }
}
