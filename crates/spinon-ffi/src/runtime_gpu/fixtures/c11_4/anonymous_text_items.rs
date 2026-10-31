#[path = "anonymous_text_items/computed_font_size.rs"]
mod computed_font_size;
#[path = "anonymous_text_items/computed_letter_spacing.rs"]
mod computed_letter_spacing;
#[path = "anonymous_text_items/computed_line_break.rs"]
mod computed_line_break;
#[path = "anonymous_text_items/computed_line_height.rs"]
mod computed_line_height;
#[path = "anonymous_text_items/computed_overflow_wrap.rs"]
mod computed_overflow_wrap;
#[path = "anonymous_text_items/computed_text_indent.rs"]
mod computed_text_indent;
#[path = "anonymous_text_items/computed_text_transform.rs"]
mod computed_text_transform;
#[path = "anonymous_text_items/computed_word_break.rs"]
mod computed_word_break;
#[path = "anonymous_text_items/computed_word_spacing.rs"]
mod computed_word_spacing;

const SCENE_IDS: [&str; 55] = [
    "anonymous-min-content-wrap",
    "anonymous-normal-min-content",
    "anonymous-nowrap-min-content",
    "anonymous-normal-newline-max-content",
    "anonymous-pre-line-max-content",
    "anonymous-comment-continuation",
    "anonymous-runs-around-block",
    "anonymous-hidden-child-continuation",
    "whitespace-does-not-create-item",
    "anonymous-normal-max-content-spaces",
    "anonymous-pre-wrap-max-content-spaces",
    "anonymous-pre-wrap-comment-split-max-content",
    "anonymous-pre-wrap-min-content-spaces",
    "anonymous-pre-wrap-soft-wrap",
    "anonymous-pre-wrap-forced-break",
    "anonymous-pre-wrap-whitespace-before-block",
    "anonymous-break-spaces-max-content",
    "anonymous-break-spaces-comment-split-max-content",
    "anonymous-break-spaces-min-content",
    "anonymous-break-spaces-soft-wrap-25",
    "anonymous-break-spaces-forced-break-max-content",
    "anonymous-break-spaces-forced-break-min-content",
    "anonymous-break-spaces-trailing-soft-wrap-25",
    "anonymous-break-spaces-trailing-max-content",
    "anonymous-break-spaces-whitespace-only",
    "anonymous-break-spaces-whitespace-before-block",
    "anonymous-pre-max-content-spaces",
    "anonymous-pre-min-content-spaces",
    "anonymous-pre-forced-break",
    "anonymous-normal-fixed-width-control",
    "anonymous-pre-fixed-width-nowrap",
    "anonymous-pre-tab-max-content",
    "anonymous-pre-tab-leading",
    "anonymous-pre-tab-exact-stop",
    "anonymous-pre-tab-consecutive",
    "anonymous-pre-tab-forced-break",
    "anonymous-pre-tab-trailing",
    "anonymous-pre-tab-min-content",
    "anonymous-pre-tab-fixed-width-nowrap",
    "anonymous-pre-tab-size-2-two-columns",
    "anonymous-pre-tab-size-2-leading",
    "anonymous-pre-tab-size-2-exact-stop",
    "anonymous-pre-tab-size-2-consecutive",
    "anonymous-pre-tab-size-2-reset",
    "anonymous-pre-tab-size-2-crlf-reset",
    "anonymous-pre-tab-size-2-trailing",
    "anonymous-pre-tab-size-2-min-content",
    "anonymous-pre-tab-size-2-fixed-overflow",
    "anonymous-pre-tab-size-1-leading",
    "anonymous-pre-tab-size-1-exact-stop",
    "anonymous-pre-tab-size-4-leading",
    "anonymous-pre-tab-size-4-exact-stop",
    "anonymous-pre-tab-size-0-negative",
    "anonymous-pre-tab-size-fractional-negative",
    "anonymous-pre-tab-size-length-negative",
];

const FULL_TAB_VALUE_IDS: [&str; 14] = [
    "number-zero",
    "number-quarter",
    "number-three-tenths",
    "number-half",
    "number-one-and-half",
    "number-two-and-half",
    "number-three-and-quarter",
    "length-zero",
    "length-three-tenths",
    "length-two",
    "length-five",
    "length-twelve",
    "length-em",
    "length-calc",
];

const FULL_TAB_CASE_IDS: [&str; 10] = [
    "leading",
    "after-one",
    "after-two",
    "exact-stop",
    "consecutive",
    "trailing",
    "lf-reset",
    "crlf-reset",
    "min-content",
    "fixed-overflow",
];

const FULL_TAB_SPECIFIED_VALUES: [&str; 14] = [
    "0",
    "0.25",
    "0.3",
    "0.5",
    "1.5",
    "2.5",
    "3.25",
    "0px",
    "0.3px",
    "2px",
    "5px",
    "12px",
    "1.5em",
    "calc(2px + 3px)",
];

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    if let Some(source) = computed_overflow_wrap::runtime_source(scene_id) {
        return Some(source);
    }
    if let Some(source) = computed_line_break::runtime_source(scene_id) {
        return Some(source);
    }
    if let Some(source) = computed_word_break::runtime_source(scene_id) {
        return Some(source);
    }
    if let Some(source) = computed_font_size::runtime_source(scene_id) {
        return Some(source);
    }
    if let Some(source) = computed_line_height::runtime_source(scene_id) {
        return Some(source);
    }
    if let Some(source) = computed_letter_spacing::runtime_source(scene_id) {
        return Some(source);
    }
    if let Some(source) = computed_word_spacing::runtime_source(scene_id) {
        return Some(source);
    }
    if let Some(source) = computed_text_indent::runtime_source(scene_id) {
        return Some(source);
    }
    if let Some(source) = computed_text_transform::runtime_source(scene_id) {
        return Some(source);
    }
    if let Some((value_index, case_index)) = full_tab_scene(scene_id) {
        return Some(full_tab_runtime_source(scene_id, value_index, case_index));
    }
    if !SCENE_IDS.contains(&scene_id) {
        return None;
    }
    let root_id = format!("c114-{scene_id}-grid");
    let (width, height, columns, rows, wrap_mode) = match scene_id {
        "anonymous-min-content-wrap" => (120, "20px", "min-content 1fr", "auto", "wrap"),
        "anonymous-normal-min-content" => (20, "auto", "min-content", "auto", "wrap"),
        "anonymous-nowrap-min-content" => (20, "auto", "min-content", "auto", "nowrap"),
        "anonymous-normal-newline-max-content" | "anonymous-pre-line-max-content" => {
            (20, "auto", "max-content", "auto", "wrap")
        }
        "anonymous-normal-max-content-spaces"
        | "anonymous-pre-wrap-max-content-spaces"
        | "anonymous-pre-wrap-comment-split-max-content"
        | "anonymous-pre-wrap-forced-break" => (20, "auto", "max-content", "auto", "wrap"),
        "anonymous-pre-wrap-min-content-spaces" => (20, "auto", "min-content", "auto", "wrap"),
        "anonymous-pre-max-content-spaces" | "anonymous-pre-forced-break" => {
            (20, "auto", "max-content", "auto", "nowrap")
        }
        "anonymous-pre-min-content-spaces" => (20, "auto", "min-content", "auto", "nowrap"),
        "anonymous-normal-fixed-width-control" => (25, "auto", "25px", "auto", "wrap"),
        "anonymous-pre-fixed-width-nowrap" | "anonymous-pre-tab-fixed-width-nowrap" => {
            (25, "auto", "25px", "auto", "nowrap")
        }
        "anonymous-pre-tab-max-content" | "anonymous-pre-tab-forced-break" => {
            (100, "auto", "max-content", "auto", "nowrap")
        }
        "anonymous-pre-tab-leading" => (90, "auto", "max-content", "auto", "nowrap"),
        "anonymous-pre-tab-exact-stop" | "anonymous-pre-tab-consecutive" => {
            (170, "auto", "max-content", "auto", "nowrap")
        }
        "anonymous-pre-tab-trailing" => (80, "auto", "max-content", "auto", "nowrap"),
        "anonymous-pre-tab-min-content" => (100, "auto", "min-content", "auto", "nowrap"),
        "anonymous-pre-tab-size-2-two-columns" | "anonymous-pre-tab-size-2-reset" => {
            (60, "auto", "max-content", "auto", "nowrap")
        }
        "anonymous-pre-tab-size-2-leading" => (30, "auto", "max-content", "auto", "nowrap"),
        "anonymous-pre-tab-size-2-exact-stop" | "anonymous-pre-tab-size-2-consecutive" => {
            (70, "auto", "max-content", "auto", "nowrap")
        }
        "anonymous-pre-tab-size-2-crlf-reset" => (30, "auto", "max-content", "auto", "nowrap"),
        "anonymous-pre-tab-size-2-trailing" => (40, "auto", "max-content", "auto", "nowrap"),
        "anonymous-pre-tab-size-2-min-content" => (60, "auto", "min-content", "auto", "nowrap"),
        "anonymous-pre-tab-size-2-fixed-overflow" => (25, "auto", "25px", "auto", "nowrap"),
        "anonymous-pre-tab-size-1-leading" => (20, "auto", "max-content", "auto", "nowrap"),
        "anonymous-pre-tab-size-1-exact-stop" => (30, "auto", "max-content", "auto", "nowrap"),
        "anonymous-pre-tab-size-4-leading" => (50, "auto", "max-content", "auto", "nowrap"),
        "anonymous-pre-tab-size-4-exact-stop" => (90, "auto", "max-content", "auto", "nowrap"),
        "anonymous-pre-tab-size-0-negative"
        | "anonymous-pre-tab-size-fractional-negative"
        | "anonymous-pre-tab-size-length-negative" => (60, "auto", "max-content", "auto", "nowrap"),
        "anonymous-pre-wrap-soft-wrap" => (25, "auto", "25px", "auto", "wrap"),
        "anonymous-pre-wrap-whitespace-before-block" => (40, "10px", "20px 20px", "10px", "wrap"),
        "anonymous-break-spaces-max-content"
        | "anonymous-break-spaces-comment-split-max-content" => {
            (60, "auto", "max-content", "auto", "wrap")
        }
        "anonymous-break-spaces-min-content" => (30, "auto", "min-content", "auto", "wrap"),
        "anonymous-break-spaces-soft-wrap-25" | "anonymous-break-spaces-trailing-soft-wrap-25" => {
            (25, "auto", "25px", "auto", "wrap")
        }
        "anonymous-break-spaces-forced-break-max-content" => {
            (40, "auto", "max-content", "auto", "wrap")
        }
        "anonymous-break-spaces-forced-break-min-content" => {
            (30, "auto", "min-content", "auto", "wrap")
        }
        "anonymous-break-spaces-trailing-max-content" => {
            (40, "auto", "max-content", "auto", "wrap")
        }
        "anonymous-break-spaces-whitespace-only" => (0, "auto", "auto", "auto", "wrap"),
        "anonymous-break-spaces-whitespace-before-block" => {
            (40, "10px", "20px 20px", "10px", "wrap")
        }
        "anonymous-comment-continuation" => (120, "10px", "max-content 1fr", "10px", "wrap"),
        "anonymous-runs-around-block" => (
            120,
            "10px",
            "min-content min-content min-content",
            "10px",
            "wrap",
        ),
        "anonymous-hidden-child-continuation" => (120, "10px", "max-content 1fr", "10px", "wrap"),
        "whitespace-does-not-create-item" => (120, "10px", "20px 30px 40px", "10px", "wrap"),
        _ => return None,
    };
    let white_space_collapse = match scene_id {
        "anonymous-pre-line-max-content" => "preserve-breaks",
        "anonymous-pre-wrap-max-content-spaces"
        | "anonymous-pre-wrap-comment-split-max-content"
        | "anonymous-pre-wrap-min-content-spaces"
        | "anonymous-pre-wrap-soft-wrap"
        | "anonymous-pre-wrap-forced-break"
        | "anonymous-pre-wrap-whitespace-before-block" => "preserve",
        "anonymous-pre-max-content-spaces"
        | "anonymous-pre-min-content-spaces"
        | "anonymous-pre-forced-break"
        | "anonymous-pre-fixed-width-nowrap"
        | "anonymous-pre-tab-max-content"
        | "anonymous-pre-tab-leading"
        | "anonymous-pre-tab-exact-stop"
        | "anonymous-pre-tab-consecutive"
        | "anonymous-pre-tab-forced-break"
        | "anonymous-pre-tab-trailing"
        | "anonymous-pre-tab-min-content"
        | "anonymous-pre-tab-fixed-width-nowrap"
        | "anonymous-pre-tab-size-2-two-columns"
        | "anonymous-pre-tab-size-2-leading"
        | "anonymous-pre-tab-size-2-exact-stop"
        | "anonymous-pre-tab-size-2-consecutive"
        | "anonymous-pre-tab-size-2-reset"
        | "anonymous-pre-tab-size-2-crlf-reset"
        | "anonymous-pre-tab-size-2-trailing"
        | "anonymous-pre-tab-size-2-min-content"
        | "anonymous-pre-tab-size-2-fixed-overflow"
        | "anonymous-pre-tab-size-1-leading"
        | "anonymous-pre-tab-size-1-exact-stop"
        | "anonymous-pre-tab-size-4-leading"
        | "anonymous-pre-tab-size-4-exact-stop"
        | "anonymous-pre-tab-size-0-negative"
        | "anonymous-pre-tab-size-fractional-negative"
        | "anonymous-pre-tab-size-length-negative" => "preserve",
        "anonymous-break-spaces-max-content"
        | "anonymous-break-spaces-comment-split-max-content"
        | "anonymous-break-spaces-min-content"
        | "anonymous-break-spaces-soft-wrap-25"
        | "anonymous-break-spaces-forced-break-max-content"
        | "anonymous-break-spaces-forced-break-min-content"
        | "anonymous-break-spaces-trailing-soft-wrap-25"
        | "anonymous-break-spaces-trailing-max-content"
        | "anonymous-break-spaces-whitespace-only"
        | "anonymous-break-spaces-whitespace-before-block" => "break-spaces",
        _ => "collapse",
    };
    let tab_size = match scene_id {
        "anonymous-pre-tab-size-1-leading" | "anonymous-pre-tab-size-1-exact-stop" => "1",
        "anonymous-pre-tab-size-2-two-columns"
        | "anonymous-pre-tab-size-2-leading"
        | "anonymous-pre-tab-size-2-exact-stop"
        | "anonymous-pre-tab-size-2-consecutive"
        | "anonymous-pre-tab-size-2-reset"
        | "anonymous-pre-tab-size-2-crlf-reset"
        | "anonymous-pre-tab-size-2-trailing"
        | "anonymous-pre-tab-size-2-min-content"
        | "anonymous-pre-tab-size-2-fixed-overflow" => "2",
        "anonymous-pre-tab-size-4-leading" | "anonymous-pre-tab-size-4-exact-stop" => "4",
        "anonymous-pre-tab-size-0-negative" => "0",
        "anonymous-pre-tab-size-fractional-negative" => "1.5",
        "anonymous-pre-tab-size-length-negative" => "20px",
        _ => "8",
    };
    let mut source = format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:{width}px;height:{height};margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{columns};grid-template-rows:{rows};font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;tab-size:{tab_size};white-space-collapse:{white_space_collapse};text-wrap-mode:{wrap_mode};word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#101820');
document.appendChild(root);
const nodeIds = [{root_id:?}];
"#
    );
    match scene_id {
        "anonymous-min-content-wrap"
        | "anonymous-normal-min-content"
        | "anonymous-nowrap-min-content" => source.push_str(
            "root.appendChild(document.createTextNode('XX XX'));\n",
        ),
        "anonymous-normal-newline-max-content" | "anonymous-pre-line-max-content" => {
            source.push_str("root.appendChild(document.createTextNode('XX\\nXX'));\n")
        }
        "anonymous-normal-max-content-spaces"
        | "anonymous-pre-wrap-max-content-spaces"
        | "anonymous-pre-wrap-min-content-spaces" => {
            source.push_str("root.appendChild(document.createTextNode('XX  XX'));\n")
        }
        "anonymous-pre-wrap-comment-split-max-content" => source.push_str(
            "root.appendChild(document.createTextNode('XX '));\nroot.appendChild(document.createTextNode(' XX'));\n",
        ),
        "anonymous-pre-wrap-soft-wrap" => {
            source.push_str("root.appendChild(document.createTextNode('XX XX'));\n")
        }
        "anonymous-pre-wrap-forced-break" => {
            source.push_str("root.appendChild(document.createTextNode('XX\\nXX'));\n")
        }
        "anonymous-comment-continuation" => source.push_str(
            "root.appendChild(document.createTextNode('XX'));\nroot.appendChild(document.createTextNode('XX'));\n",
        ),
        "anonymous-hidden-child-continuation" => source.push_str(
            r#"root.appendChild(document.createTextNode('XX'));
const hidden = document.createElement('span');
hidden.setAttribute('id', 'c114-anonymous-hidden-child-continuation-hidden');
hidden.setAttribute('style', 'display:none');
hidden.appendChild(document.createTextNode('ignored'));
root.appendChild(hidden);
root.appendChild(document.createTextNode('XX'));
"#,
        ),
        "anonymous-runs-around-block" => source.push_str(
            r#"root.appendChild(document.createTextNode('XX'));
const block = document.createElement('div');
block.setAttribute('id', 'c114-anonymous-runs-around-block-item');
block.setAttribute('data-node', '');
block.setAttribute('style', 'box-sizing:border-box;display:block;width:30px;height:10px;min-width:0;min-height:0;margin:0;padding:0;border:0;background-color:#2866f6');
root.appendChild(block);
nodeIds.push('c114-anonymous-runs-around-block-item');
root.appendChild(document.createTextNode('XXXX'));
"#,
        ),
        "whitespace-does-not-create-item" => source.push_str(
            r#"const first = document.createElement('div');
first.setAttribute('id', 'c114-whitespace-does-not-create-item-first');
first.setAttribute('data-node', '');
first.setAttribute('style', 'box-sizing:border-box;display:block;width:10px;height:10px;min-width:0;min-height:0;margin:0;padding:0;border:0;background-color:#e5484d');
root.appendChild(first);
nodeIds.push('c114-whitespace-does-not-create-item-first');
root.appendChild(document.createTextNode(' \t\n'));
const second = document.createElement('div');
second.setAttribute('id', 'c114-whitespace-does-not-create-item-second');
second.setAttribute('data-node', '');
second.setAttribute('style', 'box-sizing:border-box;display:block;width:10px;height:10px;min-width:0;min-height:0;margin:0;padding:0;border:0;background-color:#2a9d8f');
root.appendChild(second);
nodeIds.push('c114-whitespace-does-not-create-item-second');
"#,
        ),
        "anonymous-pre-wrap-whitespace-before-block" => source.push_str(
            r#"root.appendChild(document.createTextNode('  '));
const block = document.createElement('div');
block.setAttribute('id', 'c114-anonymous-pre-wrap-whitespace-before-block-item');
block.setAttribute('data-node', '');
block.setAttribute('style', 'box-sizing:border-box;display:block;width:10px;height:10px;min-width:0;min-height:0;margin:0;padding:0;border:0;background-color:#2866f6');
root.appendChild(block);
nodeIds.push('c114-anonymous-pre-wrap-whitespace-before-block-item');
"#,
        ),
        "anonymous-pre-max-content-spaces" => {
            source.push_str("root.appendChild(document.createTextNode('XX  XX'));\n")
        }
        "anonymous-pre-min-content-spaces"
        | "anonymous-normal-fixed-width-control"
        | "anonymous-pre-fixed-width-nowrap" => {
            source.push_str("root.appendChild(document.createTextNode('XX XX'));\n")
        }
        "anonymous-pre-forced-break" => {
            source.push_str("root.appendChild(document.createTextNode('XX\\nXX'));\n")
        }
        "anonymous-pre-tab-max-content"
        | "anonymous-pre-tab-min-content"
        | "anonymous-pre-tab-fixed-width-nowrap"
        | "anonymous-pre-tab-size-2-two-columns"
        | "anonymous-pre-tab-size-2-min-content"
        | "anonymous-pre-tab-size-2-fixed-overflow"
        | "anonymous-pre-tab-size-0-negative"
        | "anonymous-pre-tab-size-fractional-negative"
        | "anonymous-pre-tab-size-length-negative" => {
            source.push_str("root.appendChild(document.createTextNode('XX\\tXX'));\n")
        }
        "anonymous-pre-tab-leading"
        | "anonymous-pre-tab-size-2-leading"
        | "anonymous-pre-tab-size-1-leading"
        | "anonymous-pre-tab-size-4-leading" => {
            source.push_str("root.appendChild(document.createTextNode('\\tX'));\n")
        }
        "anonymous-pre-tab-exact-stop" => {
            source.push_str("root.appendChild(document.createTextNode('XXXXXXXX\\tX'));\n")
        }
        "anonymous-pre-tab-size-4-exact-stop" => {
            source.push_str("root.appendChild(document.createTextNode('XXXX\\tX'));\n")
        }
        "anonymous-pre-tab-size-2-exact-stop" => {
            source.push_str("root.appendChild(document.createTextNode('XXXX\\tX'));\n")
        }
        "anonymous-pre-tab-size-1-exact-stop" => {
            source.push_str("root.appendChild(document.createTextNode('X\\tX'));\n")
        }
        "anonymous-pre-tab-consecutive" | "anonymous-pre-tab-size-2-consecutive" => {
            source.push_str("root.appendChild(document.createTextNode('XX\\t\\tX'));\n")
        }
        "anonymous-pre-tab-forced-break" => {
            source.push_str("root.appendChild(document.createTextNode('XX\\tXX\\nX\\tX'));\n")
        }
        "anonymous-pre-tab-trailing" | "anonymous-pre-tab-size-2-trailing" => {
            source.push_str("root.appendChild(document.createTextNode('XX\\t'));\n")
        }
        "anonymous-pre-tab-size-2-reset" => {
            source.push_str("root.appendChild(document.createTextNode('XX\\tXX\\nX\\tX'));\n")
        }
        "anonymous-pre-tab-size-2-crlf-reset" => {
            source.push_str("root.appendChild(document.createTextNode('X\\t\\r\\nX\\tX'));\n")
        }
        "anonymous-break-spaces-max-content"
        | "anonymous-break-spaces-min-content"
        | "anonymous-break-spaces-soft-wrap-25" => {
            source.push_str("root.appendChild(document.createTextNode('XX  XX'));\n")
        }
        "anonymous-break-spaces-comment-split-max-content" => source.push_str(
            "root.appendChild(document.createTextNode('XX '));\nroot.appendChild(document.createTextNode(' XX'));\n",
        ),
        "anonymous-break-spaces-forced-break-max-content"
        | "anonymous-break-spaces-forced-break-min-content" => {
            source.push_str("root.appendChild(document.createTextNode('XX  \\nXX'));\n")
        }
        "anonymous-break-spaces-trailing-soft-wrap-25"
        | "anonymous-break-spaces-trailing-max-content" => {
            source.push_str("root.appendChild(document.createTextNode('XX  '));\n")
        }
        "anonymous-break-spaces-whitespace-only" => {
            source.push_str("root.appendChild(document.createTextNode('  '));\n")
        }
        "anonymous-break-spaces-whitespace-before-block" => source.push_str(
            r#"root.appendChild(document.createTextNode('  '));
const block = document.createElement('div');
block.setAttribute('id', 'c114-anonymous-break-spaces-whitespace-before-block-item');
block.setAttribute('data-node', '');
block.setAttribute('style', 'box-sizing:border-box;display:block;width:10px;height:10px;min-width:0;min-height:0;margin:0;padding:0;border:0;background-color:#2866f6');
root.appendChild(block);
nodeIds.push('c114-anonymous-break-spaces-whitespace-before-block-item');
"#,
        ),
        _ => return None,
    }
    source.push_str(&format!(
        "return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds }};\n}})()"
    ));
    Some(source)
}

fn full_tab_scene(scene_id: &str) -> Option<(usize, usize)> {
    let scene_id = scene_id.strip_prefix("anonymous-pre-tab-value-")?;
    for (value_index, value_id) in FULL_TAB_VALUE_IDS.iter().enumerate() {
        let Some(case_id) = scene_id
            .strip_prefix(value_id)
            .and_then(|value| value.strip_prefix('-'))
        else {
            continue;
        };
        if let Some(case_index) = FULL_TAB_CASE_IDS.iter().position(|case| *case == case_id) {
            return Some((value_index, case_index));
        }
    }
    None
}

#[cfg(test)]
fn full_tab_scene_id(value_id: &str, case_id: &str) -> String {
    format!("anonymous-pre-tab-value-{value_id}-{case_id}")
}

fn full_tab_runtime_source(scene_id: &str, value_index: usize, case_index: usize) -> String {
    let root_id = format!("c114-{scene_id}-grid");
    let width = if case_index == 9 { 25 } else { 120 };
    let columns = match case_index {
        8 => "min-content",
        9 => "25px",
        _ => "max-content",
    };
    let text = match case_index {
        0 => "\tX",
        1 => "X\tX",
        2 => "XX\tX",
        3 => "XXXX\tX",
        4 => "X\t\tX",
        5 => "XX\t",
        6 => "XX\tXX\nX\tX",
        7 => "X\t\r\nX\tX",
        8 | 9 => "XX\tXX",
        _ => "XX\tXX",
    };
    format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:{width}px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{columns};grid-template-rows:auto;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;tab-size:{tab_size};white-space-collapse:preserve;text-wrap-mode:nowrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#101820');
document.appendChild(root);
const nodeIds = [{root_id:?}];
root.appendChild(document.createTextNode({text:?}));
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds }};
}})()"#,
        tab_size = FULL_TAB_SPECIFIED_VALUES[value_index],
    )
}

#[cfg(test)]
mod tests {
    use super::{
        FULL_TAB_CASE_IDS, FULL_TAB_SPECIFIED_VALUES, FULL_TAB_VALUE_IDS, SCENE_IDS,
        full_tab_scene_id, runtime_source,
    };
    use std::collections::BTreeSet;

    #[test]
    fn anonymous_grid_text_sources_are_a_fixed_fifty_five_scene_matrix() {
        let unique_ids = SCENE_IDS.iter().copied().collect::<BTreeSet<_>>();
        assert_eq!(SCENE_IDS.len(), 55);
        assert_eq!(unique_ids.len(), SCENE_IDS.len());
        for (scene_id, text_nodes, elements, width, height, wrap_mode, white_space_collapse) in [
            (
                "anonymous-min-content-wrap",
                1,
                0,
                120,
                "20px",
                "wrap",
                "collapse",
            ),
            (
                "anonymous-normal-min-content",
                1,
                0,
                20,
                "auto",
                "wrap",
                "collapse",
            ),
            (
                "anonymous-nowrap-min-content",
                1,
                0,
                20,
                "auto",
                "nowrap",
                "collapse",
            ),
            (
                "anonymous-normal-newline-max-content",
                1,
                0,
                20,
                "auto",
                "wrap",
                "collapse",
            ),
            (
                "anonymous-pre-line-max-content",
                1,
                0,
                20,
                "auto",
                "wrap",
                "preserve-breaks",
            ),
            (
                "anonymous-normal-max-content-spaces",
                1,
                0,
                20,
                "auto",
                "wrap",
                "collapse",
            ),
            (
                "anonymous-pre-wrap-max-content-spaces",
                1,
                0,
                20,
                "auto",
                "wrap",
                "preserve",
            ),
            (
                "anonymous-pre-wrap-comment-split-max-content",
                2,
                0,
                20,
                "auto",
                "wrap",
                "preserve",
            ),
            (
                "anonymous-pre-wrap-min-content-spaces",
                1,
                0,
                20,
                "auto",
                "wrap",
                "preserve",
            ),
            (
                "anonymous-pre-wrap-soft-wrap",
                1,
                0,
                25,
                "auto",
                "wrap",
                "preserve",
            ),
            (
                "anonymous-pre-wrap-forced-break",
                1,
                0,
                20,
                "auto",
                "wrap",
                "preserve",
            ),
            (
                "anonymous-comment-continuation",
                2,
                0,
                120,
                "10px",
                "wrap",
                "collapse",
            ),
            (
                "anonymous-runs-around-block",
                2,
                1,
                120,
                "10px",
                "wrap",
                "collapse",
            ),
            (
                "anonymous-hidden-child-continuation",
                3,
                0,
                120,
                "10px",
                "wrap",
                "collapse",
            ),
            (
                "whitespace-does-not-create-item",
                1,
                2,
                120,
                "10px",
                "wrap",
                "collapse",
            ),
            (
                "anonymous-pre-wrap-whitespace-before-block",
                1,
                1,
                40,
                "10px",
                "wrap",
                "preserve",
            ),
            (
                "anonymous-break-spaces-max-content",
                1,
                0,
                60,
                "auto",
                "wrap",
                "break-spaces",
            ),
            (
                "anonymous-break-spaces-comment-split-max-content",
                2,
                0,
                60,
                "auto",
                "wrap",
                "break-spaces",
            ),
            (
                "anonymous-break-spaces-min-content",
                1,
                0,
                30,
                "auto",
                "wrap",
                "break-spaces",
            ),
            (
                "anonymous-break-spaces-soft-wrap-25",
                1,
                0,
                25,
                "auto",
                "wrap",
                "break-spaces",
            ),
            (
                "anonymous-break-spaces-forced-break-max-content",
                1,
                0,
                40,
                "auto",
                "wrap",
                "break-spaces",
            ),
            (
                "anonymous-break-spaces-forced-break-min-content",
                1,
                0,
                30,
                "auto",
                "wrap",
                "break-spaces",
            ),
            (
                "anonymous-break-spaces-trailing-soft-wrap-25",
                1,
                0,
                25,
                "auto",
                "wrap",
                "break-spaces",
            ),
            (
                "anonymous-break-spaces-trailing-max-content",
                1,
                0,
                40,
                "auto",
                "wrap",
                "break-spaces",
            ),
            (
                "anonymous-break-spaces-whitespace-only",
                1,
                0,
                0,
                "auto",
                "wrap",
                "break-spaces",
            ),
            (
                "anonymous-break-spaces-whitespace-before-block",
                1,
                1,
                40,
                "10px",
                "wrap",
                "break-spaces",
            ),
            (
                "anonymous-pre-max-content-spaces",
                1,
                0,
                20,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-min-content-spaces",
                1,
                0,
                20,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-forced-break",
                1,
                0,
                20,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-normal-fixed-width-control",
                1,
                0,
                25,
                "auto",
                "wrap",
                "collapse",
            ),
            (
                "anonymous-pre-fixed-width-nowrap",
                1,
                0,
                25,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-max-content",
                1,
                0,
                100,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-leading",
                1,
                0,
                90,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-exact-stop",
                1,
                0,
                170,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-consecutive",
                1,
                0,
                170,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-forced-break",
                1,
                0,
                100,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-trailing",
                1,
                0,
                80,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-min-content",
                1,
                0,
                100,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-fixed-width-nowrap",
                1,
                0,
                25,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-size-2-two-columns",
                1,
                0,
                60,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-size-2-leading",
                1,
                0,
                30,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-size-2-exact-stop",
                1,
                0,
                70,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-size-2-consecutive",
                1,
                0,
                70,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-size-2-reset",
                1,
                0,
                60,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-size-2-crlf-reset",
                1,
                0,
                30,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-size-2-trailing",
                1,
                0,
                40,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-size-2-min-content",
                1,
                0,
                60,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-size-2-fixed-overflow",
                1,
                0,
                25,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-size-1-leading",
                1,
                0,
                20,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-size-1-exact-stop",
                1,
                0,
                30,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-size-4-leading",
                1,
                0,
                50,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-size-4-exact-stop",
                1,
                0,
                90,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-size-0-negative",
                1,
                0,
                60,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-size-fractional-negative",
                1,
                0,
                60,
                "auto",
                "nowrap",
                "preserve",
            ),
            (
                "anonymous-pre-tab-size-length-negative",
                1,
                0,
                60,
                "auto",
                "nowrap",
                "preserve",
            ),
        ] {
            let source = runtime_source(scene_id).expect("known C11.4 scene");
            assert!(source.len() < 4 * 1024);
            assert!(source.contains(&format!("sceneId:{scene_id:?}")));
            assert!(source.contains(&format!("display:grid;width:{width}px;height:{height};")));
            assert!(source.contains(&format!("text-wrap-mode:{wrap_mode}")));
            assert!(source.contains(&format!("white-space-collapse:{white_space_collapse}")));
            let expected_tab_size = match scene_id {
                "anonymous-pre-tab-size-1-leading" | "anonymous-pre-tab-size-1-exact-stop" => "1",
                "anonymous-pre-tab-size-2-two-columns"
                | "anonymous-pre-tab-size-2-leading"
                | "anonymous-pre-tab-size-2-exact-stop"
                | "anonymous-pre-tab-size-2-consecutive"
                | "anonymous-pre-tab-size-2-reset"
                | "anonymous-pre-tab-size-2-crlf-reset"
                | "anonymous-pre-tab-size-2-trailing"
                | "anonymous-pre-tab-size-2-min-content"
                | "anonymous-pre-tab-size-2-fixed-overflow" => "2",
                "anonymous-pre-tab-size-4-leading" | "anonymous-pre-tab-size-4-exact-stop" => "4",
                "anonymous-pre-tab-size-0-negative" => "0",
                "anonymous-pre-tab-size-fractional-negative" => "1.5",
                "anonymous-pre-tab-size-length-negative" => "20px",
                _ => "8",
            };
            assert!(source.contains(&format!("tab-size:{expected_tab_size};")));
            assert_eq!(
                source.matches("document.createTextNode(").count(),
                text_nodes
            );
            assert_eq!(
                source.matches("document.createElement('div')").count(),
                elements + 1
            );
            if scene_id == "anonymous-hidden-child-continuation" {
                assert!(source.contains("document.createElement('span')"));
                assert!(source.contains("setAttribute('style', 'display:none')"));
            }
        }
        assert!(runtime_source("unknown-anonymous-scene").is_none());
    }

    #[test]
    fn anonymous_grid_full_tab_size_values_generate_all_140_runtime_scenes() {
        let mut unique = BTreeSet::new();
        for (value_index, value_id) in FULL_TAB_VALUE_IDS.iter().enumerate() {
            for (case_index, case_id) in FULL_TAB_CASE_IDS.iter().enumerate() {
                let scene_id = full_tab_scene_id(value_id, case_id);
                assert!(unique.insert(scene_id.clone()));
                let source = runtime_source(&scene_id).expect("full tab-size scene");
                assert!(source.len() < 4 * 1024, "{scene_id}");
                assert!(source.contains(&format!("sceneId:{scene_id:?}")));
                assert!(source.contains(&format!(
                    "tab-size:{};",
                    FULL_TAB_SPECIFIED_VALUES[value_index]
                )));
                assert!(source.contains("white-space-collapse:preserve;text-wrap-mode:nowrap"));
                assert_eq!(source.matches("document.createTextNode(").count(), 1);
                assert_eq!(source.matches("document.createElement('div')").count(), 1);
                if case_index == 8 {
                    assert!(source.contains("grid-template-columns:min-content"));
                } else if case_index == 9 {
                    assert!(source.contains("grid-template-columns:25px"));
                    assert!(source.contains("width:25px;height:auto"));
                } else {
                    assert!(source.contains("grid-template-columns:max-content"));
                }
            }
        }
        assert_eq!(unique.len(), 140);
        assert!(runtime_source("anonymous-pre-tab-value-number-zero-unknown").is_none());
    }
}
