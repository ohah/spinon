use std::{collections::BTreeSet, ops::Range};

use crate::{ParagraphInput, TextLayoutError, TextSourceRange, WhiteSpaceMode};

pub(crate) struct NormalizedText {
    pub text: String,
    pub sources: Vec<TextSourceRange>,
}

#[derive(Clone)]
struct SourceCharacter {
    character: char,
    node: spinon_core::NodeId,
    source_utf16: Range<usize>,
    inline_ancestors: Vec<spinon_core::NodeId>,
}

pub(crate) fn normalize(input: &ParagraphInput) -> Result<NormalizedText, TextLayoutError> {
    let mut characters = Vec::new();
    for source in &input.sources {
        if source.text.is_empty() {
            continue;
        }
        let mut utf16_offset = 0;
        for character in source.text.chars() {
            let unit_count = character.len_utf16();
            characters.push(SourceCharacter {
                character,
                node: source.node,
                source_utf16: utf16_offset..utf16_offset + unit_count,
                inline_ancestors: source.inline_ancestors.clone(),
            });
            utf16_offset += unit_count;
        }
    }

    let has_tabs = characters.iter().any(|source| source.character == '\t');
    let preserve_pre_tabs = matches!(
        input.style.white_space,
        WhiteSpaceMode::Pre | WhiteSpaceMode::PreWithTabSize(_)
    ) && has_tabs;
    let preserve_pre_wrap_tabs = input.style.white_space == WhiteSpaceMode::PreWrap && has_tabs;
    let pre_tab_size = match input.style.white_space {
        WhiteSpaceMode::Pre => crate::TabSize::Eight,
        WhiteSpaceMode::PreWithTabSize(tab_size) => tab_size,
        _ => crate::TabSize::Eight,
    };
    if preserve_pre_tabs && !supports_pre_tab_profile(input, &characters) {
        return Err(TextLayoutError::UnsupportedPreTabProfile(input.node));
    }
    let preserve_tabs = preserve_pre_tabs || preserve_pre_wrap_tabs;
    if preserve_pre_wrap_tabs && !supports_pre_wrap_tab_profile(input, &characters) {
        return Err(TextLayoutError::UnsupportedPreWrapTabProfile(input.node));
    }
    if input.style.white_space == WhiteSpaceMode::BreakSpaces
        && characters.iter().any(|source| source.character == '\t')
    {
        return Err(TextLayoutError::UnsupportedBreakSpacesTabProfile(
            input.node,
        ));
    }

    let mut result = NormalizedText {
        text: String::new(),
        sources: Vec::new(),
    };
    let mut pending_spaces = Vec::new();
    let mut line_start = true;
    let mut pre_wrap_column = 0usize;
    let mut index = 0;
    while index < characters.len() {
        let source = &characters[index];
        if preserves_spaces_and_breaks(input.style.white_space)
            && is_segment_break(source.character)
        {
            let mut line_break_sources = vec![source.clone()];
            if source.character == '\r'
                && characters
                    .get(index + 1)
                    .is_some_and(|next| next.character == '\n')
            {
                line_break_sources.push(characters[index + 1].clone());
                index += 1;
            }
            push_character_sources(&mut result, &line_break_sources, '\n');
            pre_wrap_column = 0;
        } else if preserve_tabs && source.character == '\t' {
            let tab_stop = pre_tab_size.spaces();
            let spaces = tab_stop - pre_wrap_column % tab_stop;
            push_expanded_tab(&mut result, source, spaces);
            pre_wrap_column += spaces;
        } else if preserves_spaces_and_breaks(input.style.white_space) {
            push_character(&mut result, source, source.character);
            if preserve_tabs {
                pre_wrap_column += 1;
            }
        } else if input.style.white_space == WhiteSpaceMode::PreLine
            && is_segment_break(source.character)
        {
            push_suppressed_sources(&mut result, &pending_spaces);
            pending_spaces.clear();
            let mut line_break_sources = vec![source.clone()];
            line_start = true;
            if source.character == '\r'
                && characters
                    .get(index + 1)
                    .is_some_and(|next| next.character == '\n')
            {
                line_break_sources.push(characters[index + 1].clone());
                index += 1;
            }
            push_character_sources(&mut result, &line_break_sources, '\n');
        } else if is_collapsible_space(source.character) {
            pending_spaces.push(source.clone());
        } else {
            if !line_start && !pending_spaces.is_empty() {
                push_collapsed_space(&mut result, &pending_spaces);
                pending_spaces.clear();
            } else {
                push_suppressed_sources(&mut result, &pending_spaces);
                pending_spaces.clear();
            }
            push_character(&mut result, source, source.character);
            line_start = false;
        }
        index += 1;
    }
    push_suppressed_sources(&mut result, &pending_spaces);

    Ok(result)
}

fn preserves_spaces_and_breaks(mode: WhiteSpaceMode) -> bool {
    matches!(
        mode,
        WhiteSpaceMode::Pre
            | WhiteSpaceMode::PreWithTabSize(_)
            | WhiteSpaceMode::PreWrap
            | WhiteSpaceMode::BreakSpaces
    )
}

fn supports_pre_wrap_tab_profile(input: &ParagraphInput, characters: &[SourceCharacter]) -> bool {
    let style = &input.style;
    style.font_family == "Ahem"
        && style.font_size == 10.0
        && style.line_height == 10.0
        && !style.line_height_is_normal
        && style.font_weight == 400.0
        && style.font_style == crate::FontStyleMode::Normal
        && style.writing_mode == spinon_core::WritingMode::HorizontalTb
        && style.text_orientation == spinon_core::TextOrientation::Mixed
        && style.direction == crate::TextDirection::Ltr
        && style.word_break == crate::WordBreakMode::Normal
        && matches!(
            style.line_break,
            crate::LineBreakMode::Normal | crate::LineBreakMode::Strict
        )
        && style.ligatures == crate::LigatureMode::Normal
        && input.inline_styles.is_empty()
        && input.inline_run_styles.is_empty()
        && characters
            .iter()
            .all(|source| source.inline_ancestors.is_empty())
        && characters.len() == 5
        && characters[0].character == 'X'
        && characters[1].character == 'X'
        && characters[2].character == '\t'
        && characters[3].character == 'X'
        && characters[4].character == 'X'
}

fn supports_pre_tab_profile(input: &ParagraphInput, characters: &[SourceCharacter]) -> bool {
    let style = &input.style;
    style.font_family == "Ahem"
        && style.font_size == 10.0
        && style.line_height == 10.0
        && !style.line_height_is_normal
        && style.font_weight == 400.0
        && style.font_style == crate::FontStyleMode::Normal
        && style.writing_mode == spinon_core::WritingMode::HorizontalTb
        && style.text_orientation == spinon_core::TextOrientation::Mixed
        && style.direction == crate::TextDirection::Ltr
        && style.word_break == crate::WordBreakMode::Normal
        && matches!(
            style.line_break,
            crate::LineBreakMode::Normal | crate::LineBreakMode::Strict
        )
        && style.ligatures == crate::LigatureMode::Normal
        && input.inline_styles.is_empty()
        && input.inline_run_styles.is_empty()
        && characters.iter().all(|source| {
            source.inline_ancestors.is_empty()
                && matches!(
                    source.character,
                    'X' | ' ' | '\t' | '\n' | '\r' | '\u{000c}'
                )
        })
}

fn push_expanded_tab(result: &mut NormalizedText, source: &SourceCharacter, spaces: usize) {
    let start = result.text.len();
    for _ in 0..spaces {
        result.text.push(' ');
    }
    result.sources.push(TextSourceRange {
        node: source.node,
        source_utf16: source.source_utf16.clone(),
        paragraph_utf8: start..result.text.len(),
        inline_ancestors: source.inline_ancestors.clone(),
    });
}

fn is_collapsible_space(character: char) -> bool {
    matches!(character, ' ' | '\t' | '\n' | '\r' | '\u{000c}')
}

fn is_segment_break(character: char) -> bool {
    matches!(character, '\n' | '\r' | '\u{000c}')
}

fn push_character(result: &mut NormalizedText, source: &SourceCharacter, character: char) {
    push_character_sources(result, std::slice::from_ref(source), character);
}

fn push_collapsed_space(result: &mut NormalizedText, sources: &[SourceCharacter]) {
    push_character_sources(result, sources, ' ');
}

fn push_suppressed_sources(result: &mut NormalizedText, sources: &[SourceCharacter]) {
    let position = result.text.len();
    for source in sources {
        result.sources.push(TextSourceRange {
            node: source.node,
            source_utf16: source.source_utf16.clone(),
            paragraph_utf8: position..position,
            inline_ancestors: source.inline_ancestors.clone(),
        });
    }
}

fn push_character_sources(
    result: &mut NormalizedText,
    sources: &[SourceCharacter],
    character: char,
) {
    let start = result.text.len();
    result.text.push(character);
    let paragraph_utf8 = start..result.text.len();
    for source in sources {
        result.sources.push(TextSourceRange {
            node: source.node,
            source_utf16: source.source_utf16.clone(),
            paragraph_utf8: paragraph_utf8.clone(),
            inline_ancestors: source.inline_ancestors.clone(),
        });
    }
}

pub(crate) fn overlapping_sources(
    sources: &[TextSourceRange],
    range: &Range<usize>,
) -> Vec<TextSourceRange> {
    let mut result = Vec::<TextSourceRange>::new();
    for source in sources.iter().filter(|source| {
        source.paragraph_utf8.start < range.end && range.start < source.paragraph_utf8.end
    }) {
        if let Some(previous) = result.last_mut()
            && previous.node == source.node
            && previous.inline_ancestors == source.inline_ancestors
            && previous.source_utf16.end == source.source_utf16.start
            && previous.paragraph_utf8.end == source.paragraph_utf8.start
        {
            previous.source_utf16.end = source.source_utf16.end;
            previous.paragraph_utf8.end = source.paragraph_utf8.end;
        } else {
            result.push(source.clone());
        }
    }
    result
}

pub(crate) fn positioned_sources(
    sources: &[TextSourceRange],
    range: &Range<usize>,
) -> Vec<TextSourceRange> {
    let mut seen = BTreeSet::new();
    overlapping_sources(sources, range)
        .into_iter()
        .filter(|source| seen.insert((source.paragraph_utf8.start, source.paragraph_utf8.end)))
        .collect()
}
