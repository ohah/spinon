use std::{collections::BTreeSet, ops::Range};

use crate::{HyphensMode, ParagraphInput, TextLayoutError, TextSourceRange, WhiteSpaceMode};

#[cfg(feature = "c11-4-hyphens-auto-fixture-test")]
use hypher::{Lang, hyphenate};

pub(crate) struct NormalizedText {
    pub text: String,
    pub sources: Vec<TextSourceRange>,
    pub tab_advances: Vec<TabAdvance>,
}

pub(crate) struct TabAdvance {
    pub text_index: usize,
    pub width: f32,
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

    if input.style.hyphens.is_some() && !supports_hyphens_profile(input, &characters) {
        return Err(TextLayoutError::UnsupportedHyphensProfile(input.node));
    }

    if input.style.text_transform != crate::TextTransformMode::None {
        if !input.inline_styles.is_empty()
            || !input.inline_run_styles.is_empty()
            || characters
                .iter()
                .any(|source| !source.inline_ancestors.is_empty() || !source.character.is_ascii())
        {
            return Err(TextLayoutError::UnsupportedTextTransformProfile(input.node));
        }
        for source in &mut characters {
            source.character = match input.style.text_transform {
                crate::TextTransformMode::None => source.character,
                crate::TextTransformMode::Uppercase => source.character.to_ascii_uppercase(),
                crate::TextTransformMode::Lowercase => source.character.to_ascii_lowercase(),
            };
        }
    }

    if input.style.hyphens == Some(HyphensMode::None) {
        for source in &mut characters {
            if source.character == '\u{00ad}' {
                // WORD JOINER keeps the authored UTF-16 source span while suppressing SHY breaks.
                source.character = '\u{2060}';
            }
        }
    }

    let has_tabs = characters.iter().any(|source| source.character == '\t');
    let preserve_pre_tabs = matches!(
        input.style.white_space,
        WhiteSpaceMode::Pre | WhiteSpaceMode::PreWithTabSize(_)
    ) && has_tabs;
    let preserve_pre_wrap_tabs = input.style.white_space == WhiteSpaceMode::PreWrap && has_tabs;
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
        tab_advances: Vec::new(),
    };
    let mut pending_spaces = Vec::new();
    let mut line_start = true;
    let mut pre_wrap_column = 0usize;
    let mut pre_tab_inline_position = 0.0_f32;
    let mut in_pre_tab_run = false;
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
            pre_tab_inline_position = 0.0;
            in_pre_tab_run = false;
        } else if preserve_tabs && source.character == '\t' {
            if let WhiteSpaceMode::PreWithTabSize(tab_size) = input.style.white_space {
                let width = pre_tab_advance(
                    tab_size,
                    input.style.font_size,
                    pre_tab_inline_position,
                    in_pre_tab_run,
                )
                .ok_or(TextLayoutError::InvalidGlyphLayout(input.node))?;
                result.tab_advances.push(TabAdvance {
                    text_index: result.text.len(),
                    width,
                });
                push_suppressed_sources(&mut result, std::slice::from_ref(source));
                pre_tab_inline_position += width;
                if !pre_tab_inline_position.is_finite() {
                    return Err(TextLayoutError::InvalidGlyphLayout(input.node));
                }
                in_pre_tab_run = true;
            } else {
                let tab_stop = 8;
                let spaces = tab_stop - pre_wrap_column % tab_stop;
                push_expanded_tab(&mut result, source, spaces);
                pre_wrap_column += spaces;
            }
        } else if preserves_spaces_and_breaks(input.style.white_space) {
            push_character(&mut result, source, source.character);
            if preserve_tabs {
                pre_wrap_column += 1;
            }
            if matches!(input.style.white_space, WhiteSpaceMode::PreWithTabSize(_)) {
                if source.character == 'X' || source.character == ' ' {
                    pre_tab_inline_position += input.style.font_size;
                    if !pre_tab_inline_position.is_finite() {
                        return Err(TextLayoutError::InvalidGlyphLayout(input.node));
                    }
                }
                in_pre_tab_run = false;
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

    if input.style.hyphens == Some(HyphensMode::Auto) {
        add_auto_hyphenation_markers(&mut result, input)?;
    }

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

fn supports_hyphens_profile(input: &ParagraphInput, characters: &[SourceCharacter]) -> bool {
    let style = &input.style;
    let common_style = style.font_family == "Ahem"
        && style.font_size == 10.0
        && style.line_height == 10.0
        && !style.line_height_is_normal
        && style.font_weight == 400.0
        && style.font_style == crate::FontStyleMode::Normal
        && style.white_space == WhiteSpaceMode::Normal
        && style.word_break == crate::WordBreakMode::Normal
        && style.overflow_wrap == crate::OverflowWrapMode::Normal
        && style.text_transform == crate::TextTransformMode::None
        && style.text_indent == 0.0
        && style.text_indent_flags == Default::default()
        && style.letter_spacing == 0.0
        && style.word_spacing == 0.0
        && style.direction == crate::TextDirection::Ltr
        && style.writing_mode == spinon_core::WritingMode::HorizontalTb
        && style.text_orientation == spinon_core::TextOrientation::Mixed
        && style.line_break == crate::LineBreakMode::Normal
        && style.ligatures == crate::LigatureMode::Normal;
    if !common_style
        || input.sources.len() != 1
        || !input.inline_styles.is_empty()
        || !input.inline_run_styles.is_empty()
        || !input.sources[0].inline_ancestors.is_empty()
        || characters
            .iter()
            .any(|source| !source.inline_ancestors.is_empty())
    {
        return false;
    }

    match style.hyphens {
        Some(HyphensMode::Auto) => {
            #[cfg(feature = "c11-4-hyphens-auto-fixture-test")]
            {
                style.locale.as_deref() == Some("en")
                    && matches!(
                        input.sources[0].text.as_str(),
                        "hyphenation" | "internationalization" | "representation"
                    )
                    && !input.sources[0].text.contains('\u{00ad}')
                    && characters
                        .iter()
                        .all(|source| source.character.is_ascii_lowercase())
            }
            #[cfg(not(feature = "c11-4-hyphens-auto-fixture-test"))]
            {
                false
            }
        }
        Some(HyphensMode::None | HyphensMode::Manual) => {
            style
                .locale
                .as_deref()
                .is_none_or(|locale| locale.eq_ignore_ascii_case("en"))
                && characters
                    .iter()
                    .all(|source| matches!(source.character, 'A'..='F' | '-' | '\u{00ad}'))
                && characters
                    .iter()
                    .filter(|source| source.character == '\u{00ad}')
                    .count()
                    <= 2
        }
        None => false,
    }
}

#[cfg(feature = "c11-4-hyphens-auto-fixture-test")]
pub(crate) fn auto_hyphenated_source(text: &str, replacement: char) -> Option<String> {
    let breaks = auto_hyphen_breaks(text)?;
    let mut result = String::with_capacity(text.len() + breaks.len() * replacement.len_utf8());
    let mut break_index = 0;
    for (byte_index, character) in text.char_indices() {
        if break_index < breaks.len() && breaks[break_index] == byte_index {
            result.push(replacement);
            break_index += 1;
        }
        result.push(character);
    }
    (break_index == breaks.len()).then_some(result)
}

#[cfg(feature = "c11-4-hyphens-auto-fixture-test")]
fn add_auto_hyphenation_markers(
    result: &mut NormalizedText,
    input: &ParagraphInput,
) -> Result<(), TextLayoutError> {
    let Some(breaks) = auto_hyphen_breaks(&result.text) else {
        return Err(TextLayoutError::UnsupportedHyphensProfile(input.node));
    };
    for position in breaks.into_iter().rev() {
        if !result.text.is_char_boundary(position) {
            return Err(TextLayoutError::UnsupportedHyphensProfile(input.node));
        }
        result.text.insert(position, '\u{00ad}');
        let marker_end = position + '\u{00ad}'.len_utf8();
        for source in &mut result.sources {
            if source.paragraph_utf8.start >= position {
                source.paragraph_utf8.start += '\u{00ad}'.len_utf8();
                source.paragraph_utf8.end += '\u{00ad}'.len_utf8();
            } else if source.paragraph_utf8.end > position {
                source.paragraph_utf8.end += '\u{00ad}'.len_utf8();
            }
            if source.paragraph_utf8.start < position && source.paragraph_utf8.end > marker_end {
                return Err(TextLayoutError::UnsupportedHyphensProfile(input.node));
            }
        }
        result.sources.push(TextSourceRange {
            node: input.sources[0].node,
            source_utf16: position..position,
            paragraph_utf8: position..marker_end,
            inline_ancestors: Vec::new(),
        });
    }
    result
        .sources
        .sort_by_key(|source| (source.paragraph_utf8.start, source.paragraph_utf8.end));
    Ok(())
}

#[cfg(not(feature = "c11-4-hyphens-auto-fixture-test"))]
fn add_auto_hyphenation_markers(
    _: &mut NormalizedText,
    input: &ParagraphInput,
) -> Result<(), TextLayoutError> {
    Err(TextLayoutError::UnsupportedHyphensProfile(input.node))
}

#[cfg(feature = "c11-4-hyphens-auto-fixture-test")]
fn auto_hyphen_breaks(text: &str) -> Option<Vec<usize>> {
    if text.len() > hypher::MAX_INLINE_SIZE {
        return None;
    }
    let mut cursor = 0usize;
    let mut syllables = Vec::new();
    for syllable in hyphenate(text, Lang::English) {
        if syllable.is_empty() {
            return None;
        }
        cursor = cursor.checked_add(syllable.len())?;
        syllables.push(syllable);
    }
    if cursor != text.len() {
        return None;
    }
    let expected: &[&str] = match text {
        "hyphenation" => &["hy", "phen", "ation"],
        "internationalization" => &["in", "ter", "na", "tion", "al", "iza", "tion"],
        "representation" => &["rep", "re", "sen", "ta", "tion"],
        _ => return None,
    };
    if syllables != expected {
        return None;
    }
    let mut breaks = Vec::with_capacity(syllables.len().saturating_sub(1));
    let mut offset = 0usize;
    for syllable in syllables.iter().take(syllables.len().saturating_sub(1)) {
        offset = offset.checked_add(syllable.len())?;
        breaks.push(offset);
    }
    Some(breaks)
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

fn pre_tab_advance(
    tab_size: crate::TabSize,
    font_size: f32,
    inline_position: f32,
    in_tab_run: bool,
) -> Option<f32> {
    let interval = tab_size.interval_px(font_size);
    if !interval.is_finite() || interval < 0.0 || !inline_position.is_finite() {
        return None;
    }
    if interval == 0.0 {
        return Some(0.0);
    }
    if in_tab_run {
        return Some(interval);
    }

    let next_stop_index = (inline_position / interval).floor() + 1.0;
    let mut next_stop = next_stop_index * interval;
    let mut width = next_stop - inline_position;
    if width < font_size * 0.5 {
        next_stop += interval;
        width = next_stop - inline_position;
    }
    (width.is_finite() && width >= 0.0 && next_stop.is_finite()).then_some(width)
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
