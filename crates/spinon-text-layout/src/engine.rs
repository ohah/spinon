use std::collections::BTreeMap;
use std::ops::Range;

use parley::{
    FontContext, InlineBoxKind, LayoutContext,
    layout::{Alignment, AlignmentOptions, PositionedLayoutItem},
    style::{FontFamilyName, FontStyle as ParleyFontStyle},
};
use spinon_core::NodeId;

mod fonts;
mod glyphs;
mod inline_boxes;
mod intrinsic;
mod letter_spacing;
mod raster;
mod run_styles;

use glyphs::positioned_glyphs_with_sources;
use letter_spacing::visible_line_advance;

use crate::{
    FontMetricsSnapshot, FontRegistryRevision, InlineFragment, ParagraphInput, TextLayoutError,
    TextLayoutSnapshot, TextLine, TextRect, TextRun,
    normalize::{overlapping_sources, positioned_sources},
    snapshot::FontRegistryToken,
};

use intrinsic::BuiltParleyLayout;

pub struct TextLayoutEngine {
    font_context: FontContext,
    layout_context: LayoutContext,
    font_registry_revision: FontRegistryRevision,
    font_registry_token: FontRegistryToken,
}

impl Clone for TextLayoutEngine {
    fn clone(&self) -> Self {
        Self {
            font_context: self.font_context.clone(),
            layout_context: LayoutContext::new(),
            font_registry_revision: self.font_registry_revision,
            font_registry_token: self.font_registry_token.clone(),
        }
    }
}

impl TextLayoutEngine {
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
            text_index: index,
            is_start: true,
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
        let BuiltParleyLayout {
            normalized,
            inline_boundaries,
            mut layout,
        } = self.build_parley_layout(input, additional_inline_items)?;
        if normalized.text.is_empty() {
            return Ok(TextLayoutSnapshot {
                paragraph: input.node,
                font_registry_revision: self.font_registry_revision,
                font_registry_token: self.font_registry_token.clone(),
                available_width,
                direction: input.style.direction,
                line_break: input.style.line_break,
                overflow_wrap: input.style.overflow_wrap,
                text_transform: input.style.text_transform,
                ligatures: input.style.ligatures,
                width: 0.0,
                height: 0.0,
                text: normalized.text,
                sources: normalized.sources,
                lines: Vec::new(),
                runs: Vec::new(),
                inline_fragments: Vec::new(),
            });
        }
        let css_line_break_width = if input.style.white_space == crate::WhiteSpaceMode::BreakSpaces
        {
            self.intrinsic_inline_sizes(input)?
                .min_content()
                .max(available_width)
        } else {
            available_width
        };
        let line_break_width = css_line_break_width + input.style.letter_spacing;
        if !line_break_width.is_finite() {
            return Err(TextLayoutError::InvalidWidth);
        }
        let line_break_width = line_break_width.max(0.0);
        if matches!(
            input.style.white_space,
            crate::WhiteSpaceMode::PreWithTabSize(_)
        ) {
            layout.break_all_lines(None);
        } else {
            layout.break_all_lines(Some(line_break_width));
        }
        layout.align(Alignment::Start, AlignmentOptions::default());
        let layout_height = layout.height();
        if !layout_height.is_finite() || layout_height < 0.0 {
            return Err(TextLayoutError::InvalidGlyphLayout(input.node));
        }

        let mut lines = Vec::new();
        let mut runs = Vec::new();
        let mut fragments = BTreeMap::<(NodeId, usize), TextRect>::new();
        let line_count = layout.lines().len();
        for (line_index, line) in layout.lines().enumerate() {
            let metrics = line.metrics();
            let text_range = line.text_range();
            let line_ends_with_soft_hyphen = valid_text_range(&text_range, &normalized.text)
                && normalized.text[..text_range.end].ends_with('\u{00ad}');
            let discretionary_hyphen_advance = if input.style.hyphens
                == Some(crate::HyphensMode::Manual)
                && line_index + 1 < line_count
                && line_ends_with_soft_hyphen
            {
                // Parley 0.12 retains the SHY break but reports zero advance for it. The fixed
                // C11.4 Ahem profile gives the inserted U+2010 one em of advance.
                input.style.font_size
            } else {
                0.0
            };
            let visible_line_advance = visible_line_advance(metrics, input.style.letter_spacing)
                + discretionary_hyphen_advance;
            if !valid_text_range(&text_range, &normalized.text)
                || ![
                    metrics.advance,
                    metrics.indent,
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
                indent: metrics.indent,
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
                        let run_style =
                            run_styles::effective_run_style(input, &normalized, &text_range);
                        let resolved_line_height = if run_style.line_height_is_normal {
                            font_metrics.ascent + font_metrics.descent + font_metrics.leading
                        } else {
                            run_style.line_height
                        };
                        let resolved_run_style = crate::InlineRunStyle {
                            line_height: resolved_line_height,
                            line_height_is_normal: false,
                            ..run_style.clone()
                        };
                        let synthesis = run.synthesis();
                        let run_baseline = glyph_run.baseline();
                        let run_box_ascent = run_styles::run_box_ascent(
                            &resolved_run_style,
                            font_metrics.ascent,
                            font_metrics.descent,
                        );
                        if ![run_baseline, run_box_ascent]
                            .iter()
                            .all(|value| value.is_finite())
                        {
                            return Err(TextLayoutError::InvalidGlyphLayout(input.node));
                        }
                        let glyphs =
                            positioned_glyphs_with_sources(&glyph_run, &normalized, input.node)?;
                        runs.push(TextRun {
                            line_index,
                            text_range: text_range.clone(),
                            cluster_range: run.cluster_range(),
                            is_rtl: run.is_rtl(),
                            font_family: run_style.font_family.clone(),
                            font_size: run.font_size(),
                            font_weight: run_style.font_weight,
                            font_style: run_style.font_style,
                            baseline: run_baseline,
                            line_height: resolved_line_height,
                            font_metrics: FontMetricsSnapshot {
                                ascent: font_metrics.ascent,
                                descent: font_metrics.descent,
                                leading: font_metrics.leading,
                            },
                            normalized_coords: run
                                .normalized_coords()
                                .iter()
                                .map(|coord| coord.to_bits())
                                .collect(),
                            synthesis_embolden: synthesis.embolden(),
                            synthesis_skew_degrees: synthesis.skew(),
                            sources: run_sources,
                            glyphs,
                            font_data: run.font().clone(),
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
                                y: run_baseline - run_box_ascent,
                                width,
                                height: run_style.line_height,
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
            font_registry_token: self.font_registry_token.clone(),
            available_width,
            direction: input.style.direction,
            line_break: input.style.line_break,
            overflow_wrap: input.style.overflow_wrap,
            text_transform: input.style.text_transform,
            ligatures: input.style.ligatures,
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

impl TextLayoutEngine {
    fn font_family_list_available(&mut self, source: &str) -> bool {
        let Ok(families) = FontFamilyName::parse_css_list(source).collect::<Result<Vec<_>, _>>()
        else {
            return false;
        };
        families.into_iter().any(|family| {
            let name = match family {
                FontFamilyName::Named(name) => name.into_owned(),
                FontFamilyName::Generic(_) => return true,
            };
            self.font_context.collection.family_id(&name).is_some()
        })
    }
}

fn parley_font_style(style: crate::FontStyleMode) -> ParleyFontStyle {
    match style {
        crate::FontStyleMode::Normal => ParleyFontStyle::Normal,
        crate::FontStyleMode::Italic => ParleyFontStyle::Italic,
        crate::FontStyleMode::Oblique(angle) => ParleyFontStyle::Oblique(Some(angle)),
    }
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

pub(super) fn valid_text_range(range: &Range<usize>, text: &str) -> bool {
    range.start <= range.end
        && range.end <= text.len()
        && text.is_char_boundary(range.start)
        && text.is_char_boundary(range.end)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

#[cfg(test)]
#[path = "engine/raster_tests.rs"]
mod raster_tests;
