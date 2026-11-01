use std::{collections::BTreeMap, ops::Range};

use crate::{ParagraphInput, TextLayoutError, TextTransformMode};

use super::NormalizedText;

struct Replacement {
    source_utf8: Range<usize>,
    text: String,
}

struct CharacterOutput {
    source_utf8: Range<usize>,
    output_utf8: Vec<Range<usize>>,
}

pub(super) fn apply(
    input: &ParagraphInput,
    normalized: &mut NormalizedText,
) -> Result<(), TextLayoutError> {
    let replacements = replacements(input, &normalized.text);
    if replacements.is_empty() {
        return Ok(());
    }
    apply_replacements(input, normalized, replacements)
}

fn replacements(_input: &ParagraphInput, text: &str) -> Vec<Replacement> {
    match _input.style.text_transform {
        TextTransformMode::None => Vec::new(),
        TextTransformMode::Uppercase => text
            .char_indices()
            .filter_map(|(start, character)| {
                let transformed = character.to_ascii_uppercase();
                (transformed != character).then(|| Replacement {
                    source_utf8: start..start + character.len_utf8(),
                    text: transformed.to_string(),
                })
            })
            .collect(),
        TextTransformMode::Lowercase => text
            .char_indices()
            .filter_map(|(start, character)| {
                let transformed = character.to_ascii_lowercase();
                (transformed != character).then(|| Replacement {
                    source_utf8: start..start + character.len_utf8(),
                    text: transformed.to_string(),
                })
            })
            .collect(),
        TextTransformMode::Capitalize => {
            #[cfg(feature = "c11-4-text-transform-unicode-fixture-test")]
            if _input.style.locale.as_deref() == Some("en") {
                return unicode_capitalize_replacements(text);
            }
            ascii_capitalize_replacements(text)
        }
        TextTransformMode::FullWidth => {
            #[cfg(feature = "c11-4-text-transform-unicode-fixture-test")]
            {
                full_width_replacements(text)
            }
            #[cfg(not(feature = "c11-4-text-transform-unicode-fixture-test"))]
            {
                Vec::new()
            }
        }
    }
}

#[cfg(feature = "c11-4-text-transform-unicode-fixture-test")]
fn full_width_replacements(text: &str) -> Vec<Replacement> {
    text.char_indices()
        .filter_map(|(start, character)| {
            let full_width = match character {
                'A'..='Z' => char::from_u32(0xff21 + (character as u32 - 'A' as u32)),
                'a'..='z' => char::from_u32(0xff41 + (character as u32 - 'a' as u32)),
                '0'..='9' => char::from_u32(0xff10 + (character as u32 - '0' as u32)),
                ' ' => Some('\u{3000}'),
                _ => None,
            }?;
            Some(Replacement {
                source_utf8: start..start + character.len_utf8(),
                text: full_width.to_string(),
            })
        })
        .collect()
}

fn ascii_capitalize_replacements(text: &str) -> Vec<Replacement> {
    let mut replacements = Vec::new();
    let mut at_word_start = true;
    for (start, character) in text.char_indices() {
        if character == ' ' {
            at_word_start = true;
            continue;
        }
        if at_word_start {
            let transformed = character.to_ascii_uppercase();
            if transformed != character {
                replacements.push(Replacement {
                    source_utf8: start..start + character.len_utf8(),
                    text: transformed.to_string(),
                });
            }
        }
        at_word_start = false;
    }
    replacements
}

#[cfg(feature = "c11-4-text-transform-unicode-fixture-test")]
fn unicode_capitalize_replacements(text: &str) -> Vec<Replacement> {
    use icu_casemap::{TitlecaseMapper, options::TitlecaseOptions};
    use icu_locale_core::langid;
    use icu_properties::{CodePointMapData, CodePointSetData, props::GeneralCategory};
    use icu_segmenter::{WordSegmenter, options::WordBreakInvariantOptions};

    let segmenter =
        WordSegmenter::new_for_non_complex_scripts(WordBreakInvariantOptions::default());
    let categories = CodePointMapData::<GeneralCategory>::new();
    let lowercase = CodePointSetData::new::<icu_properties::props::Lowercase>();
    let mapper = TitlecaseMapper::new();
    const ENGLISH: icu_locale_core::LanguageIdentifier = langid!("en");
    let mut options = TitlecaseOptions::default();
    options.trailing_case = Some(icu_casemap::options::TrailingCase::Unchanged);

    let boundaries = segmenter.segment_str(text).collect::<Vec<_>>();
    let mut replacements = Vec::new();
    for boundary_pair in boundaries.windows(2) {
        let segment_start = boundary_pair[0];
        let segment_end = boundary_pair[1];
        let segment = &text[segment_start..segment_end];
        let Some((character_offset, character)) = segment
            .char_indices()
            .find(|(_, character)| is_typographic_letter_unit(categories.get(*character)))
        else {
            continue;
        };
        if !lowercase.contains(character) {
            continue;
        }

        let original = character.to_string();
        let transformed = mapper
            .titlecase_segment_to_string(&original, &ENGLISH, options)
            .into_owned();
        if transformed != original {
            let start = segment_start + character_offset;
            replacements.push(Replacement {
                source_utf8: start..start + character.len_utf8(),
                text: transformed,
            });
        }
    }
    replacements
}

#[cfg(feature = "c11-4-text-transform-unicode-fixture-test")]
fn is_typographic_letter_unit(category: icu_properties::props::GeneralCategory) -> bool {
    use icu_properties::props::GeneralCategory;

    matches!(
        category,
        GeneralCategory::UppercaseLetter
            | GeneralCategory::LowercaseLetter
            | GeneralCategory::TitlecaseLetter
            | GeneralCategory::ModifierLetter
            | GeneralCategory::OtherLetter
            | GeneralCategory::DecimalNumber
            | GeneralCategory::LetterNumber
            | GeneralCategory::OtherNumber
    )
}

fn apply_replacements(
    input: &ParagraphInput,
    normalized: &mut NormalizedText,
    replacements: Vec<Replacement>,
) -> Result<(), TextLayoutError> {
    let replacements = replacements
        .into_iter()
        .map(|replacement| (replacement.source_utf8.start, replacement))
        .collect::<BTreeMap<_, _>>();
    let original_text = std::mem::take(&mut normalized.text);
    let original_sources = std::mem::take(&mut normalized.sources);
    let original_tab_advances = std::mem::take(&mut normalized.tab_advances);
    let mut text = String::with_capacity(original_text.len());
    let mut boundary_offsets = vec![None; original_text.len() + 1];
    let mut character_outputs = Vec::new();

    for (source_start, character) in original_text.char_indices() {
        let source_end = source_start + character.len_utf8();
        let output_start = text.len();
        boundary_offsets[source_start] = Some(output_start);
        let output = replacements.get(&source_start).map_or_else(
            || character.to_string(),
            |replacement| replacement.text.clone(),
        );
        text.push_str(&output);
        let output_utf8 = output
            .char_indices()
            .map(|(offset, output_character)| {
                let start = output_start + offset;
                start..start + output_character.len_utf8()
            })
            .collect();
        boundary_offsets[source_end] = Some(text.len());
        character_outputs.push(CharacterOutput {
            source_utf8: source_start..source_end,
            output_utf8,
        });
    }
    boundary_offsets[original_text.len()] = Some(text.len());

    let mut sources = Vec::with_capacity(original_sources.len() + replacements.len());
    for source in original_sources {
        if source.paragraph_utf8.start > source.paragraph_utf8.end
            || source.paragraph_utf8.end > original_text.len()
        {
            return Err(TextLayoutError::InvalidGlyphLayout(input.node));
        }
        if source.paragraph_utf8.is_empty() {
            let position = boundary_offsets
                .get(source.paragraph_utf8.start)
                .and_then(|offset| *offset)
                .ok_or(TextLayoutError::InvalidGlyphLayout(input.node))?;
            let mut mapped = source;
            mapped.paragraph_utf8 = position..position;
            sources.push(mapped);
            continue;
        }

        for output in character_outputs.iter().filter(|output| {
            source.paragraph_utf8.start < output.source_utf8.end
                && output.source_utf8.start < source.paragraph_utf8.end
        }) {
            for output_range in &output.output_utf8 {
                let mut mapped = source.clone();
                mapped.paragraph_utf8 = output_range.clone();
                sources.push(mapped);
            }
        }
    }
    sources.sort_by_key(|source| (source.paragraph_utf8.start, source.paragraph_utf8.end));

    let tab_advances = original_tab_advances
        .into_iter()
        .map(|mut advance| {
            advance.text_index = boundary_offsets
                .get(advance.text_index)
                .and_then(|offset| *offset)
                .ok_or(TextLayoutError::InvalidGlyphLayout(input.node))?;
            Ok(advance)
        })
        .collect::<Result<_, TextLayoutError>>()?;

    normalized.text = text;
    normalized.sources = sources;
    normalized.tab_advances = tab_advances;
    Ok(())
}

#[cfg(all(test, feature = "c11-4-text-transform-unicode-fixture-test"))]
mod tests {
    use super::unicode_capitalize_replacements;

    #[test]
    fn unicode_full_titlecase_mapping_covers_ligature_and_supplementary_letters() {
        for (source, expected) in [("ǆ", "ǅ"), ("𐐨", "𐐀"), ("ß", "Ss")] {
            let replacements = unicode_capitalize_replacements(source);
            assert_eq!(replacements.len(), 1, "{source:?}");
            assert_eq!(replacements[0].text, expected, "{source:?}");
        }
    }
}
