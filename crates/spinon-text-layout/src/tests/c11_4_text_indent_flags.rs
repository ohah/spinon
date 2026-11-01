use super::{close_to, engine, paragraph};
use crate::{TextIndentFlags, WhiteSpaceMode, WordBreakMode};

#[test]
fn ahem_text_indent_flags_follow_soft_and_forced_break_line_sets() {
    let cases = [
        (
            "none",
            TextIndentFlags::default(),
            [10.0, 0.0],
            [30.0, 10.0],
            40.0,
            60.0,
        ),
        (
            "each-line",
            TextIndentFlags {
                each_line: true,
                hanging: false,
            },
            [10.0, 0.0],
            [30.0, 10.0],
            40.0,
            60.0,
        ),
        (
            "hanging",
            TextIndentFlags {
                each_line: false,
                hanging: true,
            },
            [0.0, 10.0],
            [30.0, 10.0],
            30.0,
            50.0,
        ),
        (
            "hanging each-line",
            TextIndentFlags {
                each_line: true,
                hanging: true,
            },
            [0.0, 10.0],
            [30.0, 10.0],
            30.0,
            50.0,
        ),
    ];

    for (name, flags, soft_line_indents, soft_line_advances, soft_min, soft_max) in cases {
        let mut soft = paragraph(
            "XXX X",
            "Ahem",
            10.0,
            10.0,
            WhiteSpaceMode::Normal,
            WordBreakMode::Normal,
        );
        soft.style.text_indent = 10.0;
        soft.style.text_indent_flags = flags;
        let mut text_engine = engine();
        let sizes = text_engine.intrinsic_inline_sizes(&soft).unwrap();
        close_to(sizes.min_content(), soft_min);
        close_to(sizes.max_content(), soft_max);
        close_to(sizes.min_content_height(), 20.0);
        close_to(sizes.max_content_height(), 10.0);

        let width = soft_min;
        let layout = text_engine.layout_paragraph(&soft, width).unwrap();
        assert_eq!(layout.lines.len(), 2, "soft wrap {name}");
        for (index, line) in layout.lines.iter().enumerate() {
            close_to(line.indent, soft_line_indents[index]);
            close_to(line.advance, soft_line_advances[index]);
        }
        for (line, expected_indent) in soft_line_indents.iter().copied().enumerate() {
            let run = layout
                .runs
                .iter()
                .find(|run| run.line_index == line)
                .unwrap();
            close_to(run.glyphs[0].x, expected_indent);
        }

        let mut forced = paragraph(
            "XXX\nX",
            "Ahem",
            10.0,
            10.0,
            WhiteSpaceMode::PreLine,
            WordBreakMode::Normal,
        );
        forced.style.text_indent = 10.0;
        forced.style.text_indent_flags = flags;
        let forced_sizes = text_engine.intrinsic_inline_sizes(&forced).unwrap();
        let forced_min = forced_line_indents(flags);
        let expected_min = forced_min[0] + 30.0;
        let expected_max = forced_min[0] + 30.0;
        close_to(forced_sizes.min_content(), expected_min);
        close_to(forced_sizes.max_content(), expected_max);
        close_to(forced_sizes.min_content_height(), 20.0);
        close_to(forced_sizes.max_content_height(), 20.0);

        let forced_layout = text_engine.layout_paragraph(&forced, expected_min).unwrap();
        assert_eq!(forced_layout.lines.len(), 2, "forced break {name}");
        for (index, line) in forced_layout.lines.iter().enumerate() {
            close_to(line.indent, forced_min[index]);
            close_to(line.advance, [30.0, 10.0][index]);
            let run = forced_layout
                .runs
                .iter()
                .find(|run| run.line_index == index)
                .unwrap();
            close_to(run.glyphs[0].x, forced_min[index]);
        }
    }
}

fn forced_line_indents(flags: TextIndentFlags) -> [f32; 2] {
    match (flags.each_line, flags.hanging) {
        (false, false) => [10.0, 0.0],
        (true, false) => [10.0, 10.0],
        (false, true) => [0.0, 10.0],
        (true, true) => [0.0, 0.0],
    }
}
