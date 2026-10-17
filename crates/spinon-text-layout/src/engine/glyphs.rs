use std::collections::BTreeMap;

use spinon_core::NodeId;

use crate::{TextGlyph, TextLayoutError, normalize::positioned_sources};

use super::valid_text_range;

pub(super) fn positioned_glyphs_with_sources<B: parley::style::Brush>(
    glyph_run: &parley::layout::GlyphRun<'_, B>,
    normalized: &crate::normalize::NormalizedText,
    paragraph: NodeId,
) -> Result<Vec<TextGlyph>, TextLayoutError> {
    let run = glyph_run.run();
    let clusters = run
        .clusters()
        .map(|cluster| {
            (
                cluster.text_range(),
                cluster.is_ligature_start(),
                cluster.is_ligature_continuation(),
            )
        })
        .collect::<Vec<_>>();
    let mut shaped_ranges = clusters
        .iter()
        .map(|(range, _, _)| range.clone())
        .collect::<Vec<_>>();
    let mut cluster_indices = BTreeMap::new();
    for (index, (range, _, _)) in clusters.iter().enumerate() {
        if cluster_indices.insert(range.start, index).is_some() {
            return Err(TextLayoutError::InvalidGlyphLayout(paragraph));
        }
    }

    let mut start = 0;
    while start < clusters.len() {
        if clusters[start].2 {
            // glyph가 다른 위치에 연결된 atom의 중간에서 line/run을 시작하지 않습니다.
            return Err(TextLayoutError::InvalidGlyphLayout(paragraph));
        }
        let mut end = start + 1;
        if clusters[start].1 {
            while end < clusters.len() && clusters[end].2 {
                end += 1;
            }
        }
        let mut full_range = clusters[start].0.clone();
        for cluster in &clusters[start + 1..end] {
            if full_range.end != cluster.0.start {
                return Err(TextLayoutError::InvalidGlyphLayout(paragraph));
            }
            full_range.end = cluster.0.end;
        }
        for range in &mut shaped_ranges[start..end] {
            *range = full_range.clone();
        }
        start = end;
    }

    let mut expected = Vec::new();
    for cluster in run.visual_clusters() {
        let range = cluster.text_range();
        let Some(&index) = cluster_indices.get(&range.start) else {
            return Err(TextLayoutError::InvalidGlyphLayout(paragraph));
        };
        if clusters[index].0 != range {
            return Err(TextLayoutError::InvalidGlyphLayout(paragraph));
        }
        for glyph in cluster.glyphs() {
            expected.push((glyph.id, shaped_ranges[index].clone()));
        }
    }

    let positioned = glyph_run.positioned_glyphs().collect::<Vec<_>>();
    if positioned.len() != expected.len() {
        return Err(TextLayoutError::InvalidGlyphLayout(paragraph));
    }
    positioned
        .into_iter()
        .zip(expected)
        .map(|(glyph, (expected_id, cluster_text_range))| {
            if glyph.id == 0 {
                return Err(TextLayoutError::MissingGlyphCoverage {
                    paragraph,
                    text_range: cluster_text_range,
                });
            }
            if glyph.id != expected_id
                || !valid_text_range(&cluster_text_range, &normalized.text)
                || ![glyph.x, glyph.y, glyph.advance]
                    .iter()
                    .all(|value| value.is_finite())
            {
                return Err(TextLayoutError::InvalidGlyphLayout(paragraph));
            }
            let sources = positioned_sources(&normalized.sources, &cluster_text_range);
            if sources.is_empty() {
                return Err(TextLayoutError::InvalidGlyphLayout(paragraph));
            }
            Ok(TextGlyph {
                id: glyph.id,
                x: glyph.x,
                y: glyph.y,
                advance: glyph.advance,
                cluster_text_range,
                sources,
            })
        })
        .collect()
}
