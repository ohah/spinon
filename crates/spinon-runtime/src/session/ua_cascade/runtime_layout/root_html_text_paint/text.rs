use std::sync::Arc;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, NodeId};
use spinon_layout::{FontStyleMode, LayoutFrame, LayoutTextSnapshot, TextSourceRange};
use spinon_render::{RuntimeGlyphBitmap, RuntimeSrgbColor};
use spinon_style::{ComputedCssColor, ComputedElementStyle};
use spinon_text_layout::{
    GlyphBitmapContent, LigatureMode, LineBreakMode, ParagraphStyle,
    TextDirection as ParagraphDirection, TextGlyphRasterSnapshot, TextLayoutSnapshot,
    WhiteSpaceMode, WordBreakMode,
};

use super::{DEVICE_SCALE_FACTOR, FONT_SIZE, RuntimeLayoutFailure, TextOwner, layout_failure};

pub(super) fn paragraph_style(
    style: &ComputedElementStyle,
) -> Result<ParagraphStyle, RuntimeLayoutFailure> {
    Ok(ParagraphStyle {
        font_family: "Ahem".to_owned(),
        font_size: style.font_size_css_px,
        font_weight: 400.0,
        font_style: FontStyleMode::Normal,
        line_height: FONT_SIZE,
        line_height_is_normal: false,
        white_space: WhiteSpaceMode::Normal,
        word_break: WordBreakMode::Normal,
        direction: ParagraphDirection::Ltr,
        line_break: LineBreakMode::Strict,
        ligatures: LigatureMode::Normal,
    })
}

pub(super) fn runtime_text_color(
    style: &ComputedElementStyle,
) -> Result<RuntimeSrgbColor, RuntimeLayoutFailure> {
    let color = style.computed_color.ok_or_else(|| {
        layout_failure(
            "missing_root_html_text_color",
            Some(style.node_id.get()),
            Some("color".to_owned()),
        )
    })?;
    let [red, green, blue, alpha] = color.gamut_mapped_srgb;
    RuntimeSrgbColor::new(red, green, blue, alpha).map_err(|error| {
        layout_failure(
            "root_html_text_color_invalid",
            Some(style.node_id.get()),
            Some(error.to_string()),
        )
    })
}

pub(super) fn color_matches(color: ComputedCssColor, expected: [f32; 4]) -> bool {
    color
        .gamut_mapped_srgb
        .into_iter()
        .zip(expected)
        .all(|(actual, expected)| (actual - expected).abs() <= f32::EPSILON)
}

pub(super) fn validate_text_layout(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    owner: TextOwner,
    text: &LayoutTextSnapshot,
    revision: spinon_layout::LayoutInputRevision,
) -> Result<(), RuntimeLayoutFailure> {
    if text.revision != revision
        || text.input.node != owner.element
        || text.input.sources.len() != 1
        || text.input.sources[0].node != owner.text
        || text.input.sources[0].text != "XX"
        || !text.input.sources[0].inline_ancestors.is_empty()
        || text.layout.paragraph != owner.element
        || text.layout.text != "XX"
        || (text.layout.available_width - 15.0).abs() > 0.05
        || (text.layout.width - 20.0).abs() > 0.05
        || (text.layout.height - 10.0).abs() > 0.05
        || text.layout.lines.len() != 1
    {
        return Err(layout_failure(
            "root_html_text_layout_source_or_geometry_mismatch",
            Some(owner.element.get()),
            None,
        ));
    }
    let text_handle = find_handle(snapshot, root, owner.text)?;
    let host_text = snapshot
        .node(text_handle)
        .and_then(|node| match node.kind() {
            HostNodeKind::Text(text) => Some(text),
            HostNodeKind::Element(_) => None,
        })
        .ok_or_else(|| {
            layout_failure(
                "root_html_text_source_missing",
                Some(owner.text.get()),
                None,
            )
        })?;
    if host_text.to_string_lossy() != "XX" {
        return Err(layout_failure(
            "root_html_text_source_mismatch",
            Some(owner.text.get()),
            None,
        ));
    }
    let expected_ranges = [0..1, 1..2];
    if text.layout.sources.len() != expected_ranges.len()
        || text
            .layout
            .sources
            .iter()
            .zip(expected_ranges.iter())
            .any(|(source, range)| {
                !source_matches(source, owner.text, range.clone(), range.clone())
            })
    {
        return Err(layout_failure(
            "root_html_text_utf16_source_mismatch",
            Some(owner.text.get()),
            None,
        ));
    }
    let glyphs = text
        .layout
        .runs
        .iter()
        .flat_map(|run| run.glyphs.iter())
        .collect::<Vec<_>>();
    if glyphs.len() != 2
        || glyphs.iter().enumerate().any(|(index, glyph)| {
            (glyph.advance - FONT_SIZE).abs() > 0.05
                || (glyph.x - index as f32 * FONT_SIZE).abs() > 0.05
                || glyph.sources.len() != 1
                || !source_matches(
                    &glyph.sources[0],
                    owner.text,
                    index..index + 1,
                    index..index + 1,
                )
        })
    {
        return Err(layout_failure(
            "root_html_text_glyph_mapping_mismatch",
            Some(owner.element.get()),
            None,
        ));
    }
    Ok(())
}

fn source_matches(
    source: &TextSourceRange,
    node: NodeId,
    source_range: std::ops::Range<usize>,
    paragraph_range: std::ops::Range<usize>,
) -> bool {
    source.node == node
        && source.source_utf16 == source_range
        && source.paragraph_utf8 == paragraph_range
        && source.inline_ancestors.is_empty()
}

pub(super) fn runtime_glyphs(
    owner: NodeId,
    text_node: NodeId,
    text: &TextLayoutSnapshot,
    raster: &TextGlyphRasterSnapshot,
) -> Result<Vec<RuntimeGlyphBitmap>, RuntimeLayoutFailure> {
    let shaped = text
        .runs
        .iter()
        .flat_map(|run| run.glyphs.iter())
        .collect::<Vec<_>>();
    if raster.paragraph != text.paragraph
        || raster.font_registry_revision != text.font_registry_revision
        || (raster.device_scale_factor - DEVICE_SCALE_FACTOR).abs() > f32::EPSILON
        || raster.glyphs.len() != shaped.len()
        || shaped.len() != 2
    {
        return Err(layout_failure(
            "root_html_text_raster_revision_mismatch",
            Some(owner.get()),
            None,
        ));
    }
    let mut result = Vec::with_capacity(raster.glyphs.len());
    for (index, glyph) in raster.glyphs.iter().enumerate() {
        let bitmap = glyph.bitmap.as_ref().ok_or_else(|| {
            layout_failure(
                "root_html_text_glyph_bitmap_missing",
                Some(owner.get()),
                None,
            )
        })?;
        if glyph.glyph_id != shaped[index].id
            || glyph.sources.len() != 1
            || !source_matches(
                &glyph.sources[0],
                text_node,
                index..index + 1,
                index..index + 1,
            )
            || bitmap.content != GlyphBitmapContent::AlphaMask
        {
            return Err(layout_failure(
                "root_html_text_glyph_bitmap_mismatch",
                Some(owner.get()),
                None,
            ));
        }
        result.push(
            RuntimeGlyphBitmap::new(
                bitmap.width_px,
                bitmap.height_px,
                glyph.origin_x_px,
                glyph.baseline_y_px,
                bitmap.left_px,
                bitmap.top_px,
                Arc::clone(&bitmap.pixels),
            )
            .map_err(|error| {
                layout_failure(
                    "root_html_text_glyph_bitmap_invalid",
                    Some(owner.get()),
                    Some(error.to_string()),
                )
            })?,
        );
    }
    Ok(result)
}

pub(super) fn validate_frame(frame: LayoutFrame, node: NodeId) -> Result<(), RuntimeLayoutFailure> {
    if ![frame.x, frame.y, frame.width, frame.height]
        .into_iter()
        .all(f32::is_finite)
        || frame.width < 0.0
        || frame.height < 0.0
    {
        return Err(layout_failure(
            "root_html_text_frame_invalid",
            Some(node.get()),
            None,
        ));
    }
    Ok(())
}

pub(super) fn expect_frame(
    frame: LayoutFrame,
    expected: [f32; 4],
    node: NodeId,
) -> Result<(), RuntimeLayoutFailure> {
    validate_frame(frame, node)?;
    if [frame.x, frame.y, frame.width, frame.height]
        .into_iter()
        .zip(expected)
        .any(|(actual, expected)| (actual - expected).abs() > 0.05)
    {
        return Err(layout_failure(
            "root_html_text_frame_mismatch",
            Some(node.get()),
            None,
        ));
    }
    Ok(())
}

fn find_handle(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    target: NodeId,
) -> Result<HostNodeHandle, RuntimeLayoutFailure> {
    let mut pending = vec![root];
    while let Some(handle) = pending.pop() {
        if handle.id() == target {
            return Ok(handle);
        }
        if let Some(children) = snapshot.children(handle) {
            pending.extend(children);
        }
    }
    Err(layout_failure(
        "root_html_text_source_missing",
        Some(target.get()),
        None,
    ))
}
