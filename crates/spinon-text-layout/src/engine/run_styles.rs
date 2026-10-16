use std::{collections::BTreeSet, ops::Range};

use crate::{InlineRunStyle, ParagraphInput};

fn base_run_style(input: &ParagraphInput) -> InlineRunStyle {
    InlineRunStyle {
        font_family: input.style.font_family.clone(),
        font_size: input.style.font_size,
        line_height: input.style.line_height,
    }
}

fn style_for_source(input: &ParagraphInput, source: &crate::TextSourceRange) -> InlineRunStyle {
    let mut style = base_run_style(input);
    for owner in &source.inline_ancestors {
        if let Some(override_style) = input.inline_run_styles.get(owner) {
            style.clone_from(override_style);
        }
    }
    style
}

pub(super) fn effective_run_style(
    input: &ParagraphInput,
    normalized: &crate::normalize::NormalizedText,
    range: &Range<usize>,
) -> InlineRunStyle {
    normalized
        .sources
        .iter()
        .find(|source| {
            source.paragraph_utf8.start < range.end && range.start < source.paragraph_utf8.end
        })
        .map_or_else(
            || base_run_style(input),
            |source| style_for_source(input, source),
        )
}

pub(super) fn inline_run_style_ranges(
    input: &ParagraphInput,
    normalized: &crate::normalize::NormalizedText,
) -> Vec<(Range<usize>, InlineRunStyle)> {
    let base = base_run_style(input);
    let mut boundaries = BTreeSet::new();
    for source in &normalized.sources {
        if source.paragraph_utf8.start < source.paragraph_utf8.end {
            boundaries.insert(source.paragraph_utf8.start);
            boundaries.insert(source.paragraph_utf8.end);
        }
    }
    let boundaries = boundaries.into_iter().collect::<Vec<_>>();
    let mut ranges = Vec::<(Range<usize>, InlineRunStyle)>::new();
    for pair in boundaries.windows(2) {
        let range = pair[0]..pair[1];
        let Some(source) = normalized.sources.iter().find(|source| {
            source.paragraph_utf8.start <= range.start && range.end <= source.paragraph_utf8.end
        }) else {
            continue;
        };
        let style = style_for_source(input, source);
        if style == base {
            continue;
        }
        if let Some((previous_range, previous_style)) = ranges.last_mut()
            && previous_range.end == range.start
            && *previous_style == style
        {
            previous_range.end = range.end;
        } else {
            ranges.push((range, style));
        }
    }
    ranges
}

pub(super) fn run_box_ascent(style: &InlineRunStyle, ascent: f32, descent: f32) -> f32 {
    let ascent = ascent.round();
    let descent = descent.round();
    ascent + ((style.line_height - ascent - descent) / 2.0).floor()
}
