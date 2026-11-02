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
        TextTransformMode::FullSizeKana => {
            #[cfg(feature = "c11-4-text-transform-unicode-fixture-test")]
            {
                full_size_kana_replacements(text)
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

#[cfg(feature = "c11-4-text-transform-unicode-fixture-test")]
const FULL_SIZE_KANA_MAPPINGS: [(char, char); 58] = [
    ('\u{3041}', '\u{3042}'),  // U+3041 -> U+3042
    ('\u{3043}', '\u{3044}'),  // U+3043 -> U+3044
    ('\u{3045}', '\u{3046}'),  // U+3045 -> U+3046
    ('\u{3047}', '\u{3048}'),  // U+3047 -> U+3048
    ('\u{3049}', '\u{304A}'),  // U+3049 -> U+304A
    ('\u{3095}', '\u{304B}'),  // U+3095 -> U+304B
    ('\u{3096}', '\u{3051}'),  // U+3096 -> U+3051
    ('\u{1B132}', '\u{3053}'), // U+1B132 -> U+3053
    ('\u{3063}', '\u{3064}'),  // U+3063 -> U+3064
    ('\u{3083}', '\u{3084}'),  // U+3083 -> U+3084
    ('\u{3085}', '\u{3086}'),  // U+3085 -> U+3086
    ('\u{3087}', '\u{3088}'),  // U+3087 -> U+3088
    ('\u{308E}', '\u{308F}'),  // U+308E -> U+308F
    ('\u{1B150}', '\u{3090}'), // U+1B150 -> U+3090
    ('\u{1B151}', '\u{3091}'), // U+1B151 -> U+3091
    ('\u{1B152}', '\u{3092}'), // U+1B152 -> U+3092
    ('\u{30A1}', '\u{30A2}'),  // U+30A1 -> U+30A2
    ('\u{30A3}', '\u{30A4}'),  // U+30A3 -> U+30A4
    ('\u{30A5}', '\u{30A6}'),  // U+30A5 -> U+30A6
    ('\u{30A7}', '\u{30A8}'),  // U+30A7 -> U+30A8
    ('\u{30A9}', '\u{30AA}'),  // U+30A9 -> U+30AA
    ('\u{30F5}', '\u{30AB}'),  // U+30F5 -> U+30AB
    ('\u{31F0}', '\u{30AF}'),  // U+31F0 -> U+30AF
    ('\u{30F6}', '\u{30B1}'),  // U+30F6 -> U+30B1
    ('\u{1B155}', '\u{30B3}'), // U+1B155 -> U+30B3
    ('\u{31F1}', '\u{30B7}'),  // U+31F1 -> U+30B7
    ('\u{31F2}', '\u{30B9}'),  // U+31F2 -> U+30B9
    ('\u{30C3}', '\u{30C4}'),  // U+30C3 -> U+30C4
    ('\u{31F3}', '\u{30C8}'),  // U+31F3 -> U+30C8
    ('\u{31F4}', '\u{30CC}'),  // U+31F4 -> U+30CC
    ('\u{31F5}', '\u{30CF}'),  // U+31F5 -> U+30CF
    ('\u{31F6}', '\u{30D2}'),  // U+31F6 -> U+30D2
    ('\u{31F7}', '\u{30D5}'),  // U+31F7 -> U+30D5
    ('\u{31F8}', '\u{30D8}'),  // U+31F8 -> U+30D8
    ('\u{31F9}', '\u{30DB}'),  // U+31F9 -> U+30DB
    ('\u{31FA}', '\u{30E0}'),  // U+31FA -> U+30E0
    ('\u{30E3}', '\u{30E4}'),  // U+30E3 -> U+30E4
    ('\u{30E5}', '\u{30E6}'),  // U+30E5 -> U+30E6
    ('\u{30E7}', '\u{30E8}'),  // U+30E7 -> U+30E8
    ('\u{31FB}', '\u{30E9}'),  // U+31FB -> U+30E9
    ('\u{31FC}', '\u{30EA}'),  // U+31FC -> U+30EA
    ('\u{31FD}', '\u{30EB}'),  // U+31FD -> U+30EB
    ('\u{31FE}', '\u{30EC}'),  // U+31FE -> U+30EC
    ('\u{31FF}', '\u{30ED}'),  // U+31FF -> U+30ED
    ('\u{30EE}', '\u{30EF}'),  // U+30EE -> U+30EF
    ('\u{1B164}', '\u{30F0}'), // U+1B164 -> U+30F0
    ('\u{1B165}', '\u{30F1}'), // U+1B165 -> U+30F1
    ('\u{1B166}', '\u{30F2}'), // U+1B166 -> U+30F2
    ('\u{1B167}', '\u{30F3}'), // U+1B167 -> U+30F3
    ('\u{FF67}', '\u{FF71}'),  // U+FF67 -> U+FF71
    ('\u{FF68}', '\u{FF72}'),  // U+FF68 -> U+FF72
    ('\u{FF69}', '\u{FF73}'),  // U+FF69 -> U+FF73
    ('\u{FF6A}', '\u{FF74}'),  // U+FF6A -> U+FF74
    ('\u{FF6B}', '\u{FF75}'),  // U+FF6B -> U+FF75
    ('\u{FF6F}', '\u{FF82}'),  // U+FF6F -> U+FF82
    ('\u{FF6C}', '\u{FF94}'),  // U+FF6C -> U+FF94
    ('\u{FF6D}', '\u{FF95}'),  // U+FF6D -> U+FF95
    ('\u{FF6E}', '\u{FF96}'),  // U+FF6E -> U+FF96
];

#[cfg(feature = "c11-4-text-transform-unicode-fixture-test")]
fn full_size_kana_mapping(character: char) -> Option<char> {
    FULL_SIZE_KANA_MAPPINGS
        .iter()
        .find_map(|(source, destination)| (*source == character).then_some(*destination))
}

#[cfg(feature = "c11-4-text-transform-unicode-fixture-test")]
fn full_size_kana_replacements(text: &str) -> Vec<Replacement> {
    text.char_indices()
        .filter_map(|(start, character)| {
            let destination = full_size_kana_mapping(character)?;
            Some(Replacement {
                source_utf8: start..start + character.len_utf8(),
                text: destination.to_string(),
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
    use super::{
        FULL_SIZE_KANA_MAPPINGS, full_size_kana_mapping, full_size_kana_replacements,
        unicode_capitalize_replacements,
    };
    use std::collections::BTreeSet;

    #[test]
    fn unicode_full_titlecase_mapping_covers_ligature_and_supplementary_letters() {
        for (source, expected) in [("ǆ", "ǅ"), ("𐐨", "𐐀"), ("ß", "Ss")] {
            let replacements = unicode_capitalize_replacements(source);
            assert_eq!(replacements.len(), 1, "{source:?}");
            assert_eq!(replacements[0].text, expected, "{source:?}");
        }
    }

    #[test]
    fn full_size_kana_table_has_58_unique_sources_and_maps_every_entry() {
        assert_eq!(FULL_SIZE_KANA_MAPPINGS.len(), 58);
        let sources = FULL_SIZE_KANA_MAPPINGS
            .iter()
            .map(|(source, _)| *source)
            .collect::<BTreeSet<_>>();
        assert_eq!(sources.len(), FULL_SIZE_KANA_MAPPINGS.len());

        for (source, destination) in FULL_SIZE_KANA_MAPPINGS {
            assert_eq!(full_size_kana_mapping(source), Some(destination));
            let replacements = full_size_kana_replacements(&source.to_string());
            assert_eq!(replacements.len(), 1, "source {source:?}");
            assert_eq!(
                replacements[0].text,
                destination.to_string(),
                "source {source:?}"
            );
        }
        assert_eq!(full_size_kana_mapping('あ'), None);
        assert_eq!(full_size_kana_mapping('A'), None);
    }
}
