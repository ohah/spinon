use std::{collections::BTreeSet, ops::Range};

use crate::{ParagraphInput, TextSourceRange, WhiteSpaceMode};

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

pub(crate) fn normalize(input: &ParagraphInput) -> NormalizedText {
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

    let mut result = NormalizedText {
        text: String::new(),
        sources: Vec::new(),
    };
    let mut pending_spaces = Vec::new();
    let mut line_start = true;
    let mut index = 0;
    while index < characters.len() {
        let source = &characters[index];
        if input.style.white_space == WhiteSpaceMode::PreLine && is_segment_break(source.character)
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

    result
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
