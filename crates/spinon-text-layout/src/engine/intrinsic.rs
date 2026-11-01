use std::collections::BTreeMap;

use parley::{
    BaseDirection, IndentOptions,
    layout::PositionedLayoutItem,
    style::{
        FontFeatures, FontWeight as ParleyFontWeight, LineHeight, OverflowWrap, StyleProperty,
        TextWrapMode, WhiteSpaceCollapse, WordBreak,
    },
};
use spinon_core::{NodeId, TextOrientation, WritingMode};

use crate::{
    LigatureMode, LineBreakMode, OverflowWrapMode, ParagraphInput, TextDirection,
    TextIntrinsicInlineSizes, TextIntrinsicLogicalSizes, TextLayoutError, WhiteSpaceMode,
    WordBreakMode, normalize::normalize,
};

use super::{TextLayoutEngine, inline_boxes};

pub(super) struct BuiltParleyLayout {
    pub(super) normalized: crate::normalize::NormalizedText,
    pub(super) inline_boundaries: BTreeMap<u64, inline_boxes::InlineBoundary>,
    pub(super) layout: parley::Layout<[u8; 4]>,
}

impl TextLayoutEngine {
    /// 같은 paragraph와 font registry에서 CSS min/max-content logical size를 계산합니다.
    ///
    /// Parley의 실제 shaping 및 line-break 결과를 사용합니다. 반환값은 inline/block 논리축이며
    /// physical width/height로 변환하지 않습니다. `horizontal-tb` 입력은 일반 paragraph를
    /// 받지만, 현재 `vertical-rl/mixed` 입력은 C11.4의 고정 Ahem 10px ASCII 영숫자와
    /// 일반 공백 intrinsic profile만 지원합니다. 다른 vertical 입력은
    /// `UnsupportedVerticalIntrinsicProfile`로 거부하며,
    /// 이 profile은 일반 vertical text shaping이나 glyph painting 지원을 뜻하지 않습니다.
    /// White-space는 normal/nowrap만 허용하며, nowrap은 soft wrap 기회가 없는 ASCII 입력으로 제한합니다.
    pub fn intrinsic_logical_sizes(
        &mut self,
        input: &ParagraphInput,
    ) -> Result<TextIntrinsicLogicalSizes, TextLayoutError> {
        // Parley가 반환하는 shaped advance는 inline 축의 크기이고, line-height 합은
        // line의 block progression입니다. Writing mode에 따른 물리축 교환은 하지 않습니다.
        // vertical-rl/mixed에서는 ASCII sideways glyph가 이 advance를 inline 축에 사용합니다.
        match (input.style.writing_mode, input.style.text_orientation) {
            (WritingMode::HorizontalTb, TextOrientation::Mixed) => {}
            (WritingMode::VerticalRl, TextOrientation::Mixed)
                if supports_vertical_intrinsic_profile(input) => {}
            (WritingMode::VerticalRl, TextOrientation::Mixed) => {
                return Err(TextLayoutError::UnsupportedVerticalIntrinsicProfile(
                    input.node,
                ));
            }
        }
        let sizes = self.intrinsic_inline_sizes(input)?;
        Ok(sizes.to_logical_sizes(input.style.writing_mode, input.style.text_orientation))
    }

    /// 같은 paragraph와 font registry에서 CSS min-content/max-content inline size를 계산합니다.
    ///
    /// 최종 definite-width snapshot을 만들거나 캐시에 넣지 않습니다. Parley가 같은 입력을
    /// 두 번 shape하고, 0px와 무제한 line-break 폭에서 각각 줄바꿈을 계산합니다.
    pub fn intrinsic_inline_sizes(
        &mut self,
        input: &ParagraphInput,
    ) -> Result<TextIntrinsicInlineSizes, TextLayoutError> {
        let BuiltParleyLayout {
            normalized,
            inline_boundaries,
            mut layout,
        } = self.build_parley_layout(input, Vec::new())?;
        if normalized.text.is_empty() {
            return Ok(TextIntrinsicInlineSizes::new(
                input.node,
                self.font_registry_revision(),
                self.font_registry_token.clone(),
                0.0,
                0.0,
                0.0,
                0.0,
            ));
        }
        let letter_spacing = input.style.letter_spacing;
        layout.break_all_lines(None);
        let max_content = (
            max_content_inline_advance(&layout, input.node, letter_spacing)?,
            layout.height(),
        );
        let (inline_boundaries, mut min_layout) =
            if input.style.overflow_wrap == OverflowWrapMode::BreakWord {
                // CSS Text excludes break-word emergency opportunities from min-content sizing.
                // Parley uses BreakWord for definite-width emergency wrapping, so measure this
                // one intrinsic pass with Normal while retaining the computed mode elsewhere.
                let BuiltParleyLayout {
                    normalized: min_normalized,
                    inline_boundaries,
                    layout,
                } = self.build_parley_layout_with_overflow_wrap(
                    input,
                    Vec::new(),
                    OverflowWrapMode::Normal,
                )?;
                if min_normalized.text != normalized.text {
                    return Err(TextLayoutError::InvalidGlyphLayout(input.node));
                }
                (inline_boundaries, layout)
            } else {
                (inline_boundaries, layout)
            };
        if matches!(input.style.white_space, WhiteSpaceMode::PreWithTabSize(_)) {
            min_layout.break_all_lines(None);
        } else {
            min_layout.break_all_lines(Some(0.0));
        }
        let min_content_inline = min_content_inline_advance(
            &min_layout,
            &inline_boundaries,
            input.node,
            letter_spacing,
        )?;
        let min_content_block = if input.style.white_space == WhiteSpaceMode::BreakSpaces {
            // break-spaces does not hang preserved end-of-line spaces. Reflow at the
            // computed min-content inline size so block size reflects those retained spaces.
            min_layout.break_all_lines(Some(min_content_inline));
            min_content_block_size(&min_layout, input.node)?
        } else {
            min_content_block_size(&min_layout, input.node)?
        };
        let min_content = (min_content_inline, min_content_block);
        if !min_content.0.is_finite()
            || !max_content.0.is_finite()
            || !min_content.1.is_finite()
            || !max_content.1.is_finite()
            || min_content.0 < 0.0
            || min_content.1 < 0.0
            || max_content.1 < 0.0
            || max_content.0 < min_content.0
        {
            return Err(TextLayoutError::InvalidGlyphLayout(input.node));
        }
        Ok(TextIntrinsicInlineSizes::new(
            input.node,
            self.font_registry_revision(),
            self.font_registry_token.clone(),
            min_content.0,
            max_content.0,
            min_content.1,
            max_content.1,
        ))
    }

    pub(super) fn build_parley_layout(
        &mut self,
        input: &ParagraphInput,
        additional_inline_items: Vec<(parley::InlineBox, inline_boxes::InlineBoundary)>,
    ) -> Result<BuiltParleyLayout, TextLayoutError> {
        self.build_parley_layout_with_overflow_wrap(
            input,
            additional_inline_items,
            input.style.overflow_wrap,
        )
    }

    pub(super) fn build_parley_layout_with_overflow_wrap(
        &mut self,
        input: &ParagraphInput,
        additional_inline_items: Vec<(parley::InlineBox, inline_boxes::InlineBoundary)>,
        overflow_wrap: OverflowWrapMode,
    ) -> Result<BuiltParleyLayout, TextLayoutError> {
        input
            .style
            .validate()
            .map_err(TextLayoutError::InvalidStyle)?;
        if !self.font_family_list_available(&input.style.font_family) {
            return Err(TextLayoutError::MissingFontFamily(
                input.style.font_family.clone(),
            ));
        }
        for style in input.inline_run_styles.values() {
            style.validate().map_err(TextLayoutError::InvalidStyle)?;
            if !self.font_family_list_available(&style.font_family) {
                return Err(TextLayoutError::MissingFontFamily(
                    style.font_family.clone(),
                ));
            }
        }

        let normalized = normalize(input)?;
        let (mut inline_boxes, mut inline_boundaries) =
            inline_boxes::prepare_inline_boxes(input, &normalized)?;
        for (inline_box, boundary) in additional_inline_items {
            if inline_box.index > normalized.text.len()
                || !normalized.text.is_char_boundary(inline_box.index)
                || !inline_box.width.is_finite()
                || inline_box.width < 0.0
                || inline_boundaries.contains_key(&inline_box.id)
                || boundary.width.to_bits() != inline_box.width.to_bits()
            {
                return Err(TextLayoutError::InvalidGlyphLayout(input.node));
            }
            inline_boundaries.insert(inline_box.id, boundary);
            inline_boxes.push(inline_box);
        }
        if normalized.text.is_empty() {
            return Ok(BuiltParleyLayout {
                normalized,
                inline_boundaries,
                layout: parley::Layout::new(),
            });
        }

        // Parley의 정수 metric quantization이 Blink의 정수 ascent/descent와 CSS line
        // baseline 배치에 맞습니다. Shaped glyph advance는 계속 분수 CSS px로 보존됩니다.
        let mut builder =
            self.layout_context
                .ranged_builder(&mut self.font_context, &normalized.text, 1.0, true);
        builder.push_default(StyleProperty::FontFamily(
            input.style.font_family.as_str().into(),
        ));
        builder.push_default(StyleProperty::FontSize(input.style.font_size));
        builder.push_default(StyleProperty::LetterSpacing(input.style.letter_spacing));
        builder.push_default(StyleProperty::WordSpacing(input.style.word_spacing));
        builder.push_default(StyleProperty::FontWeight(ParleyFontWeight::new(
            input.style.font_weight,
        )));
        builder.push_default(StyleProperty::FontStyle(super::parley_font_style(
            input.style.font_style,
        )));
        builder.push_default(StyleProperty::LineHeight(
            if input.style.line_height_is_normal {
                LineHeight::MetricsRelative(1.0)
            } else {
                LineHeight::Absolute(input.style.line_height)
            },
        ));
        builder.push_default(StyleProperty::WordBreak(match input.style.word_break {
            WordBreakMode::Normal => WordBreak::Normal,
            // CSS Text maps the deprecated value to normal word breaking; its emergency
            // opportunities are represented by the effective overflow-wrap value below.
            WordBreakMode::BreakWord => WordBreak::Normal,
            WordBreakMode::BreakAll => WordBreak::BreakAll,
            WordBreakMode::KeepAll => WordBreak::KeepAll,
        }));
        if let Some(locale) = input.style.locale.as_deref() {
            let locale = locale
                .parse::<parley::style::Language>()
                .map_err(|_| TextLayoutError::InvalidStyle("locale"))?;
            builder.push_default(StyleProperty::Locale(Some(locale)));
        }
        builder.push_default(StyleProperty::LineBreak(match input.style.line_break {
            LineBreakMode::Loose => parley::style::LineBreak::Loose,
            LineBreakMode::Normal => parley::style::LineBreak::Normal,
            LineBreakMode::Strict => parley::style::LineBreak::Strict,
        }));
        if input.style.ligatures == LigatureMode::None {
            builder.push_default(StyleProperty::FontFeatures(FontFeatures::from(
                "\"liga\" off, \"clig\" off, \"dlig\" off, \"hlig\" off, \"calt\" off",
            )));
        }
        let effective_overflow_wrap = if input.style.word_break == WordBreakMode::BreakWord {
            // CSS Text gives word-break:break-word the effect of overflow-wrap:anywhere,
            // regardless of the separately retained computed overflow-wrap property.
            OverflowWrapMode::Anywhere
        } else {
            overflow_wrap
        };
        builder.push_default(StyleProperty::OverflowWrap(match effective_overflow_wrap {
            OverflowWrapMode::Normal => OverflowWrap::Normal,
            OverflowWrapMode::Anywhere => OverflowWrap::Anywhere,
            OverflowWrapMode::BreakWord => OverflowWrap::BreakWord,
        }));
        builder.push_default(StyleProperty::TextWrapMode(match input.style.white_space {
            WhiteSpaceMode::NoWrap | WhiteSpaceMode::Pre | WhiteSpaceMode::PreWithTabSize(_) => {
                TextWrapMode::NoWrap
            }
            WhiteSpaceMode::Normal
            | WhiteSpaceMode::PreLine
            | WhiteSpaceMode::PreWrap
            | WhiteSpaceMode::BreakSpaces => TextWrapMode::Wrap,
        }));
        builder.push_default(StyleProperty::WhiteSpaceCollapse(
            match input.style.white_space {
                WhiteSpaceMode::PreLine => WhiteSpaceCollapse::PreserveBreaks,
                WhiteSpaceMode::Pre
                | WhiteSpaceMode::PreWithTabSize(_)
                | WhiteSpaceMode::PreWrap => WhiteSpaceCollapse::Preserve,
                WhiteSpaceMode::BreakSpaces => WhiteSpaceCollapse::BreakSpaces,
                WhiteSpaceMode::Normal | WhiteSpaceMode::NoWrap => WhiteSpaceCollapse::Collapse,
            },
        ));
        for (range, style) in super::run_styles::inline_run_style_ranges(input, &normalized) {
            builder.push(
                StyleProperty::FontFamily(style.font_family.as_str().into()),
                range.clone(),
            );
            builder.push(StyleProperty::FontSize(style.font_size), range.clone());
            builder.push(
                StyleProperty::FontWeight(ParleyFontWeight::new(style.font_weight)),
                range.clone(),
            );
            builder.push(
                StyleProperty::FontStyle(super::parley_font_style(style.font_style)),
                range.clone(),
            );
            builder.push(
                StyleProperty::LineHeight(if style.line_height_is_normal {
                    LineHeight::MetricsRelative(1.0)
                } else {
                    LineHeight::Absolute(style.line_height)
                }),
                range,
            );
        }
        builder.set_base_direction(match input.style.direction {
            TextDirection::Auto => BaseDirection::Auto,
            TextDirection::Ltr => BaseDirection::Ltr,
            TextDirection::Rtl => BaseDirection::Rtl,
        });
        for inline_box in inline_boxes {
            builder.push_inline_box(inline_box);
        }
        let mut layout = builder.build(&normalized.text);
        layout.set_text_indent(
            input.style.text_indent,
            IndentOptions {
                each_line: input.style.text_indent_flags.each_line,
                hanging: input.style.text_indent_flags.hanging,
            },
        );
        Ok(BuiltParleyLayout {
            normalized,
            inline_boundaries,
            layout,
        })
    }
}

fn supports_vertical_intrinsic_profile(input: &ParagraphInput) -> bool {
    let style = &input.style;
    let text = input.sources.first().map(|source| source.text.as_str());
    input.sources.len() == 1
        && input.sources[0].inline_ancestors.is_empty()
        && text.is_some_and(|text| {
            !text.is_empty()
                && text
                    .chars()
                    .all(|character| character == ' ' || character.is_ascii_alphanumeric())
        })
        && input.inline_styles.is_empty()
        && input.inline_run_styles.is_empty()
        && style.font_family == "Ahem"
        && style.font_size == 10.0
        && style.text_indent == 0.0
        && style.text_indent_flags == Default::default()
        && style.line_height == 10.0
        && !style.line_height_is_normal
        && style.font_weight == 400.0
        && style.font_style == crate::FontStyleMode::Normal
        && matches!(
            style.white_space,
            WhiteSpaceMode::Normal | WhiteSpaceMode::NoWrap
        )
        && style.word_break == WordBreakMode::Normal
        && style.overflow_wrap == OverflowWrapMode::Normal
        && style.direction == TextDirection::Ltr
        && style.line_break == LineBreakMode::Normal
        && style.ligatures == LigatureMode::Normal
}

fn min_content_inline_advance(
    layout: &parley::Layout<[u8; 4]>,
    inline_boundaries: &BTreeMap<u64, inline_boxes::InlineBoundary>,
    paragraph: NodeId,
    letter_spacing: f32,
) -> Result<f32, TextLayoutError> {
    let mut lines = Vec::with_capacity(layout.len());
    for line in layout.lines() {
        let metrics = line.metrics();
        let mut inline_width = 0.0;
        for item in line.items() {
            if let PositionedLayoutItem::InlineBox(positioned) = item {
                let boundary = inline_boundaries
                    .get(&positioned.id)
                    .ok_or(TextLayoutError::UnexpectedInlineBox(paragraph))?;
                if positioned.width.to_bits() != boundary.width.to_bits() {
                    return Err(TextLayoutError::InvalidGlyphLayout(paragraph));
                }
                inline_width += positioned.width;
            }
        }
        let advance = super::letter_spacing::visible_line_advance(metrics, letter_spacing)
            - inline_width
            + metrics.indent;
        if !advance.is_finite() {
            return Err(TextLayoutError::InvalidGlyphLayout(paragraph));
        }
        lines.push((line.text_range(), advance));
    }

    let mut line_advances = lines
        .iter()
        .map(|(_, advance)| *advance)
        .collect::<Vec<_>>();
    for boundary in inline_boundaries.values() {
        let line_index = if boundary.is_start {
            lines
                .iter()
                .position(|(range, _)| range.end > boundary.text_index)
        } else {
            lines
                .iter()
                .rposition(|(range, _)| range.start < boundary.text_index)
        }
        .ok_or_else(|| {
            TextLayoutError::InlineBoxWithoutText(
                boundary.owners.first().copied().unwrap_or(paragraph),
            )
        })?;
        line_advances[line_index] += boundary.width;
        if !line_advances[line_index].is_finite() {
            return Err(TextLayoutError::InvalidGlyphLayout(paragraph));
        }
    }
    Ok(line_advances
        .into_iter()
        .map(|advance| advance.max(0.0))
        .fold(0.0_f32, f32::max))
}

fn max_content_inline_advance(
    layout: &parley::Layout<[u8; 4]>,
    paragraph: NodeId,
    letter_spacing: f32,
) -> Result<f32, TextLayoutError> {
    let mut max_advance = 0.0_f32;
    for line in layout.lines() {
        let metrics = line.metrics();
        let advance =
            super::letter_spacing::visible_line_advance(metrics, letter_spacing) + metrics.indent;
        if !advance.is_finite() {
            return Err(TextLayoutError::InvalidGlyphLayout(paragraph));
        }
        max_advance = max_advance.max(advance.max(0.0));
    }
    Ok(max_advance)
}

fn min_content_block_size(
    layout: &parley::Layout<[u8; 4]>,
    paragraph: NodeId,
) -> Result<f32, TextLayoutError> {
    let mut block_size = 0.0;
    for line in layout.lines() {
        if !line.text_range().is_empty() {
            block_size += line.metrics().line_height;
        }
        if !block_size.is_finite() {
            return Err(TextLayoutError::InvalidGlyphLayout(paragraph));
        }
    }
    Ok(block_size)
}
