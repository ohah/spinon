use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, NodeId};
use spinon_layout::{
    GlyphBitmapContent, InlineRunStyle, LayoutTextSnapshot, ParagraphStyle,
    TextGlyphRasterSnapshot, TextLayoutEngine, TextLayoutSnapshot, TextSourceRange,
};
use spinon_render::{
    RuntimeGlyphBitmap, RuntimePaintCommand, RuntimeRenderSnapshot, RuntimeRenderText,
    RuntimeSrgbColor,
};
use spinon_style::ComputedStyleSnapshot;

use super::{RuntimeLayoutFailure, layout_failure};

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "root_html_default_render/fixture.rs"]
mod fixture;
#[path = "root_html_default_render/style.rs"]
mod style;
#[path = "root_html_default_render/tree.rs"]
mod tree;

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub(super) use self::fixture::{
    prepare_text_owner as prepare_fixture_text_owner,
    prepare_text_owners as prepare_fixture_text_owners,
};

use self::style::{
    inline_run_style, paragraph_style, property, root_html_grid_text_paragraph_style,
    runtime_text_color, unsupported_style, validate_frame,
};
use self::tree::{collect_handles, validate_root, validate_visible_tree};

pub(super) struct Input {
    pub(super) paragraph_styles: BTreeMap<NodeId, ParagraphStyle>,
    pub(super) inline_run_styles: BTreeMap<NodeId, InlineRunStyle>,
    pub(super) engine: TextLayoutEngine,
    style_colors: BTreeMap<NodeId, RuntimeSrgbColor>,
    text_style_owners: BTreeMap<NodeId, NodeId>,
    node_handles: BTreeMap<NodeId, HostNodeHandle>,
    device_scale_factor: f32,
}

impl Input {
    pub(super) fn anonymous_grid_text_nodes(&self) -> std::collections::BTreeSet<NodeId> {
        self.text_style_owners.keys().copied().collect()
    }
}

#[derive(Default)]
struct OwnerText {
    frame: Option<(NodeId, spinon_layout::LayoutFrame)>,
    segments: Vec<(NodeId, LayoutTextSnapshot, f32, f32)>,
}

struct PendingText {
    origin_x_css_px: f32,
    origin_y_css_px: f32,
    glyphs: Vec<RuntimeGlyphBitmap>,
    colors: Vec<RuntimeSrgbColor>,
    fallback_color: RuntimeSrgbColor,
}

pub(super) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Input, RuntimeLayoutFailure> {
    if styles.profile != spinon_style::ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
        || !styles.viewport.device_scale_factor.is_finite()
        || styles.viewport.device_scale_factor <= 0.0
    {
        return Err(layout_failure(
            "invalid_root_html_default_render_environment",
            None,
            None,
        ));
    }
    if root.generation() != snapshot.generation() {
        return Err(layout_failure(
            "root_html_default_render_revision_mismatch",
            Some(root.id().get()),
            None,
        ));
    }

    let style_by_node = styles
        .elements
        .iter()
        .map(|style| (style.node_id, style))
        .collect::<BTreeMap<_, _>>();
    if style_by_node.len() != styles.elements.len() {
        return Err(layout_failure(
            "duplicate_root_html_computed_style",
            None,
            None,
        ));
    }
    validate_root(snapshot, root)?;
    let node_handles = collect_handles(snapshot, root)?;
    let (visible_elements, anonymous_grid_text) =
        validate_visible_tree(snapshot, root, &style_by_node)?;

    let mut paragraph_styles = BTreeMap::new();
    let mut inline_run_styles = BTreeMap::new();
    let mut style_colors = BTreeMap::new();
    let mut text_style_owners = BTreeMap::new();
    for style in styles.elements.iter() {
        if !visible_elements.contains(&style.node_id) {
            continue;
        }
        let display = property(style, "display")?;
        let color = runtime_text_color(style)?;
        style_colors.insert(style.node_id, color);
        match display {
            "block" => {
                paragraph_styles.insert(style.node_id, paragraph_style(style)?);
            }
            "inline" => {
                inline_run_styles.insert(style.node_id, inline_run_style(style)?);
            }
            "grid" => {
                if let Some(source) = anonymous_grid_text.get(&style.node_id) {
                    paragraph_styles
                        .insert(style.node_id, root_html_grid_text_paragraph_style(style)?);
                    let color = style_colors[&style.node_id];
                    if style_colors.insert(source.anchor, color).is_some()
                        || text_style_owners
                            .insert(source.anchor, style.node_id)
                            .is_some()
                    {
                        return Err(layout_failure(
                            "duplicate_root_html_default_render_text_owner",
                            Some(source.anchor.get()),
                            None,
                        ));
                    }
                }
            }
            _ => {
                return Err(unsupported_style(style.node_id, "display", display));
            }
        }
    }

    Ok(Input {
        paragraph_styles,
        inline_run_styles,
        engine: TextLayoutEngine::new_for_runtime(),
        style_colors,
        text_style_owners,
        node_handles,
        device_scale_factor: styles.viewport.device_scale_factor,
    })
}

pub(super) fn attach(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    output: &spinon_style_to_layout::StyleLayoutOutput,
    input: &Input,
    mut scene: RuntimeRenderSnapshot,
) -> Result<RuntimeRenderSnapshot, RuntimeLayoutFailure> {
    let expected_key = scene.key();
    let expected_source = spinon_layout::LayoutSourceRevision::HostDocument {
        generation: snapshot.generation(),
        document: snapshot.document_revision(),
        render_tree: snapshot.render_tree_revision(),
    };
    let viewport = output.computed_styles.viewport;
    if expected_key.generation() != snapshot.generation()
        || expected_key.document_revision() != snapshot.document_revision()
        || expected_key.render_tree_revision() != snapshot.render_tree_revision()
        || expected_key.style_revision() != output.computed_styles.style_revision
        || expected_key.environment_revision() != viewport.environment_revision
        || scene.viewport_css_px().width().to_bits() != viewport.width_css_px.to_bits()
        || scene.viewport_css_px().height().to_bits() != viewport.height_css_px.to_bits()
        || viewport.device_scale_factor.to_bits() != input.device_scale_factor.to_bits()
        || output.layout.revision.source() != expected_source
        || root.generation() != snapshot.generation()
    {
        return Err(layout_failure(
            "root_html_default_render_revision_mismatch",
            Some(root.id().get()),
            None,
        ));
    }

    let styles = output
        .computed_styles
        .elements
        .iter()
        .map(|style| (style.node_id, style))
        .collect::<BTreeMap<_, _>>();
    let mut owner_text = BTreeMap::<NodeId, OwnerText>::new();
    for (&paragraph_owner, text) in &output.layout.text_layouts {
        let render_owner = input
            .text_style_owners
            .get(&paragraph_owner)
            .copied()
            .unwrap_or(paragraph_owner);
        let frame = *output.layout.frames.get(&paragraph_owner).ok_or_else(|| {
            layout_failure(
                "root_html_default_render_owner_frame_missing",
                Some(paragraph_owner.get()),
                None,
            )
        })?;
        let owner_layouts = owner_text.entry(render_owner).or_default();
        if owner_layouts
            .frame
            .replace((paragraph_owner, frame))
            .is_some()
        {
            return Err(layout_failure(
                "duplicate_root_html_default_render_text_owner",
                Some(render_owner.get()),
                None,
            ));
        }
        owner_layouts
            .segments
            .push((paragraph_owner, text.clone(), 0.0, 0.0));
    }
    for (&owner, segments) in &output.layout.text_flow_layouts {
        let owner_segments = owner_text.entry(owner).or_default();
        if owner_segments.frame.is_none() {
            let frame = *output.layout.frames.get(&owner).ok_or_else(|| {
                layout_failure(
                    "root_html_default_render_owner_frame_missing",
                    Some(owner.get()),
                    None,
                )
            })?;
            owner_segments.frame = Some((owner, frame));
        }
        for segment in segments {
            owner_segments.segments.push((
                segment.snapshot.input.node,
                segment.snapshot.clone(),
                segment.frame.x,
                segment.frame.y,
            ));
        }
    }

    let mut pending_texts = BTreeMap::new();
    for (owner, owner_layouts) in owner_text {
        let owner_style = styles.get(&owner).copied().ok_or_else(|| {
            layout_failure(
                "root_html_default_render_owner_style_missing",
                Some(owner.get()),
                None,
            )
        })?;
        let fallback_color = input.style_colors.get(&owner).copied().ok_or_else(|| {
            layout_failure(
                "root_html_default_render_owner_color_missing",
                Some(owner.get()),
                None,
            )
        })?;
        let (frame_owner, frame) = owner_layouts.frame.ok_or_else(|| {
            layout_failure(
                "root_html_default_render_owner_frame_missing",
                Some(owner.get()),
                None,
            )
        })?;
        validate_frame(frame, frame_owner)?;
        let mut pending = PendingText {
            origin_x_css_px: frame.x,
            origin_y_css_px: frame.y,
            glyphs: Vec::new(),
            colors: Vec::new(),
            fallback_color,
        };
        for (paragraph_owner, text, offset_x, offset_y) in owner_layouts.segments {
            validate_layout_text(
                snapshot,
                &input.node_handles,
                paragraph_owner,
                &text,
                output.layout.revision,
            )?;
            let mut raster_input = text.layout.clone();
            for run in &mut raster_input.runs {
                for glyph in &mut run.glyphs {
                    glyph.x += offset_x;
                    glyph.y += offset_y;
                    if !glyph.x.is_finite() || !glyph.y.is_finite() {
                        return Err(layout_failure(
                            "root_html_default_render_glyph_position_invalid",
                            Some(owner.get()),
                            None,
                        ));
                    }
                }
            }
            let raster = input
                .engine
                .rasterize_glyphs(&raster_input, input.device_scale_factor)
                .map_err(|error| {
                    layout_failure(
                        "root_html_default_render_raster_failed",
                        Some(owner.get()),
                        Some(error.to_string()),
                    )
                })?;
            append_segment_glyphs(
                paragraph_owner,
                &raster_input,
                &raster,
                &input.style_colors,
                &mut pending,
            )?;
        }
        if !pending.glyphs.is_empty() {
            pending_texts.insert(owner, pending);
        }
        let _ = owner_style;
    }

    let mut texts = Vec::with_capacity(pending_texts.len());
    let mut text_indices = BTreeMap::new();
    let mut paint_commands = Vec::with_capacity(scene.paint_commands().len() + pending_texts.len());
    for command in scene.paint_commands() {
        paint_commands.push(*command);
        let RuntimePaintCommand::DrawBox { index } = command else {
            continue;
        };
        let render_box = scene
            .boxes()
            .get(*index)
            .ok_or_else(|| layout_failure("root_html_default_render_box_missing", None, None))?;
        let Some(pending) = pending_texts.remove(&render_box.node_id()) else {
            continue;
        };
        let text_index = texts.len();
        let text = RuntimeRenderText::new_with_glyph_colors(
            render_box.node_id(),
            expected_key,
            pending.origin_x_css_px,
            pending.origin_y_css_px,
            input.device_scale_factor,
            pending.fallback_color,
            pending.colors,
            render_box.clip_css_px(),
            pending.glyphs,
        )
        .map_err(|error| {
            layout_failure(
                "root_html_default_render_text_snapshot_failed",
                Some(render_box.node_id().get()),
                Some(error.to_string()),
            )
        })?;
        texts.push(text);
        text_indices.insert(render_box.node_id(), text_index);
        paint_commands.push(RuntimePaintCommand::DrawText { index: text_index });
    }
    if let Some((owner, _)) = pending_texts.first_key_value() {
        return Err(layout_failure(
            "root_html_default_render_owner_box_missing",
            Some(owner.get()),
            None,
        ));
    }

    scene = scene
        .with_texts_and_paint_commands(texts, paint_commands)
        .map_err(|error| {
            layout_failure(
                "root_html_default_render_scene_attach_failed",
                None,
                Some(error.to_string()),
            )
        })?;
    for &owner in text_indices.keys() {
        let render_box = scene
            .boxes()
            .iter()
            .find(|render_box| render_box.node_id() == owner)
            .ok_or_else(|| {
                layout_failure(
                    "root_html_default_render_box_missing",
                    Some(owner.get()),
                    None,
                )
            })?;
        let text = scene
            .texts()
            .iter()
            .find(|text| text.node_id() == owner)
            .ok_or_else(|| {
                layout_failure(
                    "root_html_default_render_text_missing",
                    Some(owner.get()),
                    None,
                )
            })?;
        if text.clip_css_px() != render_box.clip_css_px() {
            return Err(layout_failure(
                "root_html_default_render_owner_clip_mismatch",
                Some(owner.get()),
                None,
            ));
        }
    }
    Ok(scene)
}

fn append_segment_glyphs(
    owner: NodeId,
    layout: &TextLayoutSnapshot,
    raster: &TextGlyphRasterSnapshot,
    style_colors: &BTreeMap<NodeId, RuntimeSrgbColor>,
    pending: &mut PendingText,
) -> Result<(), RuntimeLayoutFailure> {
    if raster.paragraph != owner
        || raster.font_registry_revision != layout.font_registry_revision
        || raster.glyphs.len()
            != layout
                .runs
                .iter()
                .map(|run| run.glyphs.len())
                .sum::<usize>()
    {
        return Err(layout_failure(
            "root_html_default_render_raster_revision_mismatch",
            Some(owner.get()),
            None,
        ));
    }
    let run_colors = layout
        .runs
        .iter()
        .map(|run| run_color(owner, run.sources.as_slice(), style_colors))
        .collect::<Result<Vec<_>, _>>()?;
    for glyph in &raster.glyphs {
        if glyph.run_index >= run_colors.len() {
            return Err(layout_failure(
                "root_html_default_render_glyph_run_missing",
                Some(owner.get()),
                None,
            ));
        }
        let Some(bitmap) = glyph.bitmap.as_ref() else {
            continue;
        };
        if bitmap.content != GlyphBitmapContent::AlphaMask {
            return Err(layout_failure(
                "unsupported_root_html_color_glyph",
                Some(owner.get()),
                None,
            ));
        }
        let rendered = RuntimeGlyphBitmap::new(
            bitmap.width_px,
            bitmap.height_px,
            glyph.origin_x_px,
            glyph.baseline_y_px,
            bitmap.left_px,
            bitmap.top_px,
            std::sync::Arc::clone(&bitmap.pixels),
        )
        .map_err(|error| {
            layout_failure(
                "root_html_default_render_glyph_invalid",
                Some(owner.get()),
                Some(error.to_string()),
            )
        })?;
        pending.glyphs.push(rendered);
        pending.colors.push(run_colors[glyph.run_index]);
    }
    Ok(())
}

fn run_color(
    owner: NodeId,
    sources: &[TextSourceRange],
    style_colors: &BTreeMap<NodeId, RuntimeSrgbColor>,
) -> Result<RuntimeSrgbColor, RuntimeLayoutFailure> {
    let mut color = None;
    for source in sources {
        let style_owner = source.inline_ancestors.last().copied().unwrap_or(owner);
        let source_color = style_colors.get(&style_owner).copied().ok_or_else(|| {
            layout_failure(
                "root_html_default_render_run_color_missing",
                Some(style_owner.get()),
                None,
            )
        })?;
        if color.is_some_and(|color| color != source_color) {
            return Err(layout_failure(
                "root_html_default_render_mixed_run_color",
                Some(owner.get()),
                None,
            ));
        }
        color = Some(source_color);
    }
    color.ok_or_else(|| {
        layout_failure(
            "root_html_default_render_run_source_missing",
            Some(owner.get()),
            None,
        )
    })
}

fn validate_layout_text(
    snapshot: &HostDocumentSnapshot,
    node_handles: &BTreeMap<NodeId, HostNodeHandle>,
    owner: NodeId,
    text: &spinon_layout::LayoutTextSnapshot,
    revision: spinon_layout::LayoutInputRevision,
) -> Result<(), RuntimeLayoutFailure> {
    if text.revision != revision
        || text.input.node != owner
        || text.layout.paragraph != owner
        || text.input.sources.is_empty() && !text.layout.text.is_empty()
    {
        return Err(layout_failure(
            "root_html_default_render_text_revision_mismatch",
            Some(owner.get()),
            None,
        ));
    }
    for source in &text.input.sources {
        let handle = node_handles.get(&source.node).copied().ok_or_else(|| {
            layout_failure(
                "root_html_default_render_text_source_missing",
                Some(source.node.get()),
                None,
            )
        })?;
        let Some(node) = snapshot.node(handle) else {
            return Err(layout_failure(
                "root_html_default_render_text_source_missing",
                Some(source.node.get()),
                None,
            ));
        };
        let HostNodeKind::Text(data) = node.kind() else {
            return Err(layout_failure(
                "root_html_default_render_text_source_mismatch",
                Some(source.node.get()),
                None,
            ));
        };
        let host_text = String::from_utf16(data.code_units()).map_err(|_| {
            layout_failure(
                "root_html_default_render_text_encoding_invalid",
                Some(source.node.get()),
                None,
            )
        })?;
        if host_text != source.text {
            return Err(layout_failure(
                "root_html_default_render_text_source_mismatch",
                Some(source.node.get()),
                None,
            ));
        }
    }
    Ok(())
}
