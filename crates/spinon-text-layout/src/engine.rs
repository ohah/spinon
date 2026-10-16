use std::ops::Range;
use std::{collections::BTreeMap, sync::Arc};

use linebender_resource_handle::Blob;
use parley::{
    FontContext, InlineBoxKind, LayoutContext,
    fontique::{Collection, CollectionOptions, SourceCache},
    layout::{Alignment, AlignmentOptions, PositionedLayoutItem},
    style::{LineHeight, OverflowWrap, StyleProperty, TextWrapMode, WhiteSpaceCollapse, WordBreak},
};
use spinon_core::NodeId;

mod inline_boxes;

use crate::{
    FontMetricsSnapshot, FontRegistryRevision, InlineFragment, ParagraphInput, TextGlyph,
    TextLayoutError, TextLayoutSnapshot, TextLine, TextRect, TextRun, WhiteSpaceMode,
    WordBreakMode,
    normalize::{normalize, overlapping_sources, positioned_sources},
};

pub struct TextLayoutEngine {
    font_context: FontContext,
    layout_context: LayoutContext,
    font_registry_revision: FontRegistryRevision,
}

impl Clone for TextLayoutEngine {
    fn clone(&self) -> Self {
        Self {
            font_context: self.font_context.clone(),
            layout_context: LayoutContext::new(),
            font_registry_revision: self.font_registry_revision,
        }
    }
}

impl TextLayoutEngine {
    /// 시스템 폰트 검색을 끈 font registry로 시작합니다.
    pub fn new_without_system_fonts() -> Self {
        let collection = Collection::new(CollectionOptions {
            shared: false,
            system_fonts: false,
        });
        Self {
            font_context: FontContext {
                collection,
                source_cache: SourceCache::default(),
            },
            layout_context: LayoutContext::new(),
            font_registry_revision: FontRegistryRevision::default(),
        }
    }

    /// 명시적으로 제공한 글꼴 데이터만 registry에 등록합니다.
    pub fn register_font(
        &mut self,
        expected_family: &str,
        bytes: impl AsRef<[u8]>,
    ) -> Result<FontRegistryRevision, TextLayoutError> {
        let bytes = bytes.as_ref();
        let next_revision = FontRegistryRevision(
            self.font_registry_revision
                .0
                .checked_add(1)
                .ok_or(TextLayoutError::FontRevisionOverflow)?,
        );
        // 공유하지 않는 font collection을 복제해 실패 등록이 live registry에 남지 않게 합니다.
        let mut candidate = self.font_context.collection.clone();
        let registered = candidate.register_fonts(Blob::new(Arc::new(bytes.to_vec())), None);
        let expected_family_id = candidate.family_id(expected_family);
        if !registered
            .iter()
            .any(|(family_id, _)| Some(*family_id) == expected_family_id)
        {
            return Err(TextLayoutError::InvalidFontData(expected_family.to_owned()));
        }
        self.font_context.collection = candidate;
        self.font_registry_revision = next_revision;
        Ok(next_revision)
    }

    pub const fn font_registry_revision(&self) -> FontRegistryRevision {
        self.font_registry_revision
    }

    /// 실제 Parley shaping과 line breaking을 거쳐 불변 paragraph snapshot을 만듭니다.
    pub fn layout_paragraph(
        &mut self,
        input: &ParagraphInput,
        available_width: f32,
    ) -> Result<TextLayoutSnapshot, TextLayoutError> {
        self.layout_paragraph_with_inline_items(input, available_width, Vec::new())
    }

    #[cfg(test)]
    fn layout_paragraph_with_test_baseline_marker(
        &mut self,
        input: &ParagraphInput,
        available_width: f32,
        index: usize,
        marker_node: NodeId,
    ) -> Result<TextLayoutSnapshot, TextLayoutError> {
        let marker = parley::InlineBox {
            id: u64::MAX,
            kind: InlineBoxKind::InFlow,
            index,
            width: 1.0,
            height: 0.0,
            baseline: Some(0.0),
            vertical_align: parley::VerticalAlign::BASELINE,
        };
        let boundary = inline_boxes::InlineBoundary {
            owners: vec![marker_node],
            width: marker.width,
        };
        self.layout_paragraph_with_inline_items(input, available_width, vec![(marker, boundary)])
    }

    fn layout_paragraph_with_inline_items(
        &mut self,
        input: &ParagraphInput,
        available_width: f32,
        additional_inline_items: Vec<(parley::InlineBox, inline_boxes::InlineBoundary)>,
    ) -> Result<TextLayoutSnapshot, TextLayoutError> {
        if !available_width.is_finite() || available_width < 0.0 {
            return Err(TextLayoutError::InvalidWidth);
        }
        let available_width = if available_width == 0.0 {
            0.0
        } else {
            available_width
        };
        input
            .style
            .validate()
            .map_err(TextLayoutError::InvalidStyle)?;
        if self
            .font_context
            .collection
            .family_id(&input.style.font_family)
            .is_none()
        {
            return Err(TextLayoutError::MissingFontFamily(
                input.style.font_family.clone(),
            ));
        }

        let normalized = normalize(input);
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
            return Ok(TextLayoutSnapshot {
                paragraph: input.node,
                font_registry_revision: self.font_registry_revision,
                available_width,
                width: 0.0,
                height: 0.0,
                text: normalized.text,
                sources: normalized.sources,
                lines: Vec::new(),
                runs: Vec::new(),
                inline_fragments: Vec::new(),
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
        builder.push_default(StyleProperty::LineHeight(LineHeight::Absolute(
            input.style.line_height,
        )));
        builder.push_default(StyleProperty::WordBreak(match input.style.word_break {
            WordBreakMode::Normal => WordBreak::Normal,
            WordBreakMode::BreakAll => WordBreak::BreakAll,
        }));
        builder.push_default(StyleProperty::OverflowWrap(OverflowWrap::Normal));
        builder.push_default(StyleProperty::TextWrapMode(match input.style.white_space {
            WhiteSpaceMode::NoWrap => TextWrapMode::NoWrap,
            WhiteSpaceMode::Normal | WhiteSpaceMode::PreLine => TextWrapMode::Wrap,
        }));
        builder.push_default(StyleProperty::WhiteSpaceCollapse(
            match input.style.white_space {
                WhiteSpaceMode::PreLine => WhiteSpaceCollapse::PreserveBreaks,
                WhiteSpaceMode::Normal | WhiteSpaceMode::NoWrap => WhiteSpaceCollapse::Collapse,
            },
        ));
        for inline_box in inline_boxes {
            builder.push_inline_box(inline_box);
        }
        let mut layout = builder.build(&normalized.text);
        layout.break_all_lines(Some(available_width));
        layout.align(Alignment::Start, AlignmentOptions::default());
        let layout_height = layout.height();
        if !layout_height.is_finite() || layout_height < 0.0 {
            return Err(TextLayoutError::InvalidGlyphLayout(input.node));
        }

        let mut lines = Vec::new();
        let mut runs = Vec::new();
        let mut fragments = BTreeMap::<(NodeId, usize), TextRect>::new();
        for (line_index, line) in layout.lines().enumerate() {
            let metrics = line.metrics();
            let text_range = line.text_range();
            let visible_line_advance = metrics.advance - metrics.hanging_advance;
            if !valid_text_range(&text_range, &normalized.text)
                || ![
                    metrics.advance,
                    metrics.hanging_advance,
                    metrics.block_min_coord,
                    metrics.line_height,
                    metrics.baseline,
                    visible_line_advance,
                ]
                .iter()
                .all(|value| value.is_finite())
                || visible_line_advance < 0.0
                || metrics.line_height < 0.0
            {
                return Err(TextLayoutError::InvalidGlyphLayout(input.node));
            }
            lines.push(TextLine {
                text_range,
                advance: visible_line_advance,
                hanging_advance: metrics.hanging_advance,
                line_top: metrics.block_min_coord,
                line_height: metrics.line_height,
                baseline: metrics.baseline,
            });

            for item in line.items() {
                match item {
                    PositionedLayoutItem::GlyphRun(glyph_run) => {
                        let run = glyph_run.run();
                        let text_range = run.text_range();
                        if !valid_text_range(&text_range, &normalized.text) {
                            return Err(TextLayoutError::InvalidGlyphLayout(input.node));
                        }
                        let run_sources = overlapping_sources(&normalized.sources, &text_range);
                        let font_metrics = run.font_metrics();
                        if ![
                            font_metrics.ascent,
                            font_metrics.descent,
                            font_metrics.leading,
                        ]
                        .iter()
                        .all(|value| value.is_finite())
                        {
                            return Err(TextLayoutError::InvalidGlyphLayout(input.node));
                        }
                        let glyphs = glyph_run
                            .positioned_glyphs()
                            .map(|glyph| {
                                if ![glyph.x, glyph.y, glyph.advance]
                                    .iter()
                                    .all(|value| value.is_finite())
                                {
                                    return Err(TextLayoutError::InvalidGlyphLayout(input.node));
                                }
                                Ok(TextGlyph {
                                    id: glyph.id,
                                    x: glyph.x,
                                    y: glyph.y,
                                    advance: glyph.advance,
                                })
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        runs.push(TextRun {
                            line_index,
                            text_range,
                            cluster_range: run.cluster_range(),
                            font_family: input.style.font_family.clone(),
                            font_size: run.font_size(),
                            font_metrics: FontMetricsSnapshot {
                                ascent: font_metrics.ascent,
                                descent: font_metrics.descent,
                                leading: font_metrics.leading,
                            },
                            sources: run_sources,
                            glyphs,
                        });

                        for cluster in run.clusters() {
                            let cluster_range = cluster.text_range();
                            if !valid_text_range(&cluster_range, &normalized.text) {
                                return Err(TextLayoutError::InvalidGlyphLayout(input.node));
                            }
                            let cluster_sources =
                                positioned_sources(&normalized.sources, &cluster_range);
                            let Some(x) = cluster.visual_offset() else {
                                return Err(TextLayoutError::InvalidGlyphLayout(input.node));
                            };
                            let cluster_advance = cluster.advance();
                            if !x.is_finite() || !cluster_advance.is_finite() {
                                return Err(TextLayoutError::InvalidGlyphLayout(input.node));
                            }
                            let width = cluster_advance
                                .max(0.0)
                                .min((visible_line_advance - x).max(0.0));
                            if width == 0.0 {
                                continue;
                            }
                            let rect = TextRect {
                                x,
                                y: metrics.block_min_coord,
                                width,
                                height: metrics.line_height,
                            };
                            for source in cluster_sources {
                                add_fragment(
                                    &mut fragments,
                                    input.node,
                                    source.node,
                                    line_index,
                                    rect,
                                )?;
                                for ancestor in source.inline_ancestors {
                                    add_fragment(
                                        &mut fragments,
                                        input.node,
                                        ancestor,
                                        line_index,
                                        rect,
                                    )?;
                                }
                            }
                        }
                    }
                    PositionedLayoutItem::InlineBox(positioned) => {
                        if positioned.kind != InlineBoxKind::InFlow
                            || ![
                                positioned.x,
                                positioned.y,
                                positioned.width,
                                positioned.height,
                            ]
                            .iter()
                            .all(|value| value.is_finite())
                            || positioned.width < 0.0
                        {
                            return Err(TextLayoutError::InvalidGlyphLayout(input.node));
                        }
                        let boundary = inline_boundaries
                            .get(&positioned.id)
                            .ok_or(TextLayoutError::UnexpectedInlineBox(input.node))?;
                        if positioned.width.to_bits() != boundary.width.to_bits() {
                            return Err(TextLayoutError::InvalidGlyphLayout(input.node));
                        }
                        let rect = TextRect {
                            x: positioned.x,
                            y: positioned.y,
                            width: positioned.width,
                            height: positioned.height,
                        };
                        for &owner in &boundary.owners {
                            add_fragment(&mut fragments, input.node, owner, line_index, rect)?;
                        }
                    }
                }
            }
        }

        let max_line_advance = lines
            .iter()
            .map(|line| line.advance)
            .fold(0.0_f32, f32::max);
        let inline_fragments = fragments
            .into_iter()
            .map(|((node, line_index), mut rect)| {
                let style = input.inline_styles.get(&node).copied().unwrap_or_default();
                let block_start = style.block_start();
                let block_end = style.block_end();
                rect.y -= block_start;
                rect.height += block_start + block_end;
                InlineFragment {
                    node,
                    line_index,
                    rect,
                }
            })
            .collect::<Vec<_>>();
        if inline_fragments
            .iter()
            .any(|fragment| !rect_is_finite(fragment.rect))
        {
            return Err(TextLayoutError::InvalidGlyphLayout(input.node));
        }
        Ok(TextLayoutSnapshot {
            paragraph: input.node,
            font_registry_revision: self.font_registry_revision,
            available_width,
            width: max_line_advance,
            height: layout_height,
            text: normalized.text,
            sources: normalized.sources,
            lines,
            runs,
            inline_fragments,
        })
    }
}

impl Default for TextLayoutEngine {
    fn default() -> Self {
        Self::new_without_system_fonts()
    }
}

fn add_fragment(
    fragments: &mut BTreeMap<(NodeId, usize), TextRect>,
    paragraph: NodeId,
    node: NodeId,
    line_index: usize,
    rect: TextRect,
) -> Result<(), TextLayoutError> {
    if !rect_is_finite(rect) {
        return Err(TextLayoutError::InvalidGlyphLayout(paragraph));
    }
    let entry = fragments.entry((node, line_index)).or_insert(rect);
    let left = entry.x.min(rect.x);
    let top = entry.y.min(rect.y);
    let right = (entry.x + entry.width).max(rect.x + rect.width);
    let bottom = (entry.y + entry.height).max(rect.y + rect.height);
    let merged = TextRect {
        x: left,
        y: top,
        width: right - left,
        height: bottom - top,
    };
    if !rect_is_finite(merged) {
        return Err(TextLayoutError::InvalidGlyphLayout(paragraph));
    }
    *entry = merged;
    Ok(())
}

fn rect_is_finite(rect: TextRect) -> bool {
    [
        rect.x,
        rect.y,
        rect.width,
        rect.height,
        rect.x + rect.width,
        rect.y + rect.height,
    ]
    .iter()
    .all(|value| value.is_finite())
        && rect.width >= 0.0
        && rect.height >= 0.0
}

fn valid_text_range(range: &Range<usize>, text: &str) -> bool {
    range.start <= range.end
        && range.end <= text.len()
        && text.is_char_boundary(range.start)
        && text.is_char_boundary(range.end)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
