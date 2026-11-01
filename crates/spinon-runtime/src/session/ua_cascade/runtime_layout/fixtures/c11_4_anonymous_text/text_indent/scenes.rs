use spinon_layout::TextIndentFlags;

use super::TextIndentScene;

pub(super) const TEXT_INDENT_SCENES: [TextIndentScene; 50] = [
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
    flag_scene(
        "text-indent-flags-soft-min-content-indent",
        "XXX X",
        "10px",
        10.0,
        indent_flags(false, false),
        true,
        false,
    ),
    flag_scene(
        "text-indent-flags-soft-max-content-indent",
        "XXX X",
        "10px",
        10.0,
        indent_flags(false, false),
        false,
        false,
    ),
    flag_scene(
        "text-indent-flags-soft-min-content-each-line",
        "XXX X",
        "10px each-line",
        10.0,
        indent_flags(true, false),
        true,
        false,
    ),
    flag_scene(
        "text-indent-flags-soft-max-content-each-line",
        "XXX X",
        "10px each-line",
        10.0,
        indent_flags(true, false),
        false,
        false,
    ),
    flag_scene(
        "text-indent-flags-soft-min-content-hanging",
        "XXX X",
        "10px hanging",
        10.0,
        indent_flags(false, true),
        true,
        false,
    ),
    flag_scene(
        "text-indent-flags-soft-max-content-hanging",
        "XXX X",
        "10px hanging",
        10.0,
        indent_flags(false, true),
        false,
        false,
    ),
    flag_scene(
        "text-indent-flags-soft-min-content-hanging-each-line",
        "XXX X",
        "10px hanging each-line",
        10.0,
        indent_flags(true, true),
        true,
        false,
    ),
    flag_scene(
        "text-indent-flags-soft-max-content-hanging-each-line",
        "XXX X",
        "10px hanging each-line",
        10.0,
        indent_flags(true, true),
        false,
        false,
    ),
    flag_scene(
        "text-indent-flags-soft-min-content-none",
        "XXX X",
        "0px",
        0.0,
        indent_flags(false, false),
        true,
        false,
    ),
    flag_scene(
        "text-indent-flags-soft-max-content-none",
        "XXX X",
        "0px",
        0.0,
        indent_flags(false, false),
        false,
        false,
    ),
    flag_scene(
        "text-indent-flags-forced-min-content-indent",
        "XXX\nX",
        "10px",
        10.0,
        indent_flags(false, false),
        true,
        true,
    ),
    flag_scene(
        "text-indent-flags-forced-max-content-indent",
        "XXX\nX",
        "10px",
        10.0,
        indent_flags(false, false),
        false,
        true,
    ),
    flag_scene(
        "text-indent-flags-forced-min-content-each-line",
        "XXX\nX",
        "10px each-line",
        10.0,
        indent_flags(true, false),
        true,
        true,
    ),
    flag_scene(
        "text-indent-flags-forced-max-content-each-line",
        "XXX\nX",
        "10px each-line",
        10.0,
        indent_flags(true, false),
        false,
        true,
    ),
    flag_scene(
        "text-indent-flags-forced-min-content-hanging",
        "XXX\nX",
        "10px hanging",
        10.0,
        indent_flags(false, true),
        true,
        true,
    ),
    flag_scene(
        "text-indent-flags-forced-max-content-hanging",
        "XXX\nX",
        "10px hanging",
        10.0,
        indent_flags(false, true),
        false,
        true,
    ),
    flag_scene(
        "text-indent-flags-forced-min-content-hanging-each-line",
        "XXX\nX",
        "10px hanging each-line",
        10.0,
        indent_flags(true, true),
        true,
        true,
    ),
    flag_scene(
        "text-indent-flags-forced-max-content-hanging-each-line",
        "XXX\nX",
        "10px hanging each-line",
        10.0,
        indent_flags(true, true),
        false,
        true,
    ),
    flag_scene(
        "text-indent-flags-forced-min-content-none",
        "XXX\nX",
        "0px",
        0.0,
        indent_flags(false, false),
        true,
        true,
    ),
    flag_scene(
        "text-indent-flags-forced-max-content-none",
        "XXX\nX",
        "0px",
        0.0,
        indent_flags(false, false),
        false,
        true,
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
        text_indent_flags: TextIndentFlags {
            each_line: false,
            hanging: false,
        },
        is_min_content,
        is_pre_line: false,
    }
}

const fn flag_scene(
    id: &'static str,
    text: &'static str,
    computed_value: &'static str,
    text_indent: f32,
    text_indent_flags: TextIndentFlags,
    is_min_content: bool,
    is_pre_line: bool,
) -> TextIndentScene {
    TextIndentScene {
        id,
        text,
        computed_value,
        text_indent,
        text_indent_flags,
        is_min_content,
        is_pre_line,
    }
}

const fn indent_flags(each_line: bool, hanging: bool) -> TextIndentFlags {
    TextIndentFlags { each_line, hanging }
}

pub(super) const TEXT_INDENT_SCENE_IDS: [&str; TEXT_INDENT_SCENES.len()] =
    scene_ids(TEXT_INDENT_SCENES);

const fn scene_ids(
    scenes: [TextIndentScene; TEXT_INDENT_SCENES.len()],
) -> [&'static str; TEXT_INDENT_SCENES.len()] {
    let mut ids = [""; TEXT_INDENT_SCENES.len()];
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
