use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, NodeId};
use spinon_layout::TextLayoutEngine;
use spinon_render::{
    CssClip, RuntimePaintCommand, RuntimeRenderSnapshot, RuntimeRenderText, RuntimeSrgbColor,
};
use spinon_style::{ComputedCssOverflow, ComputedStyleSnapshot};
use spinon_text_layout::ParagraphStyle;

use super::{RuntimeLayoutFailure, layout_failure};

#[path = "root_html_text_paint/text.rs"]
mod text;
#[path = "root_html_text_paint/tree.rs"]
mod tree;

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
const VIEWPORT_WIDTH: f32 = 320.0;
const VIEWPORT_HEIGHT: f32 = 240.0;
const DEVICE_SCALE_FACTOR: f32 = 1.0;
const FONT_SIZE: f32 = 10.0;
const AHEM: &[u8] = include_bytes!("../../../../../../tests/fixtures/resources/c15/Ahem.ttf");

#[derive(Clone, Copy, Debug)]
struct TextOwner {
    element: NodeId,
    text: NodeId,
    color: RuntimeSrgbColor,
}

pub(super) struct Input {
    body: NodeId,
    main: NodeId,
    owners: [TextOwner; 2],
    pub(super) paragraph_styles: BTreeMap<NodeId, ParagraphStyle>,
    pub(super) engine: TextLayoutEngine,
}

pub(super) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Input, RuntimeLayoutFailure> {
    if styles.profile != spinon_style::ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
        || (styles.viewport.width_css_px - VIEWPORT_WIDTH).abs() > f32::EPSILON
        || (styles.viewport.height_css_px - VIEWPORT_HEIGHT).abs() > f32::EPSILON
        || (styles.viewport.device_scale_factor - DEVICE_SCALE_FACTOR).abs() > f32::EPSILON
    {
        return Err(layout_failure(
            "invalid_root_html_text_paint_environment",
            None,
            None,
        ));
    }

    let style_by_node = styles
        .elements
        .iter()
        .map(|style| (style.node_id, style))
        .collect::<BTreeMap<_, _>>();
    let nodes = tree::find_fixture_nodes(snapshot, root, &style_by_node)?;
    tree::validate_visible_tree(snapshot, root, &style_by_node, &nodes)?;

    let mut paragraph_styles = BTreeMap::new();
    let mut text_owners = Vec::new();
    for (element_id, text_id, overflow) in [
        (
            nodes.visible,
            nodes.visible_text,
            ComputedCssOverflow::Visible,
        ),
        (nodes.hidden, nodes.hidden_text, ComputedCssOverflow::Hidden),
    ] {
        let style = style_by_node.get(&element_id).copied().ok_or_else(|| {
            layout_failure("missing_computed_style", Some(element_id.get()), None)
        })?;
        tree::validate_text_owner_style(style, overflow)?;
        let color = text::runtime_text_color(style)?;
        paragraph_styles.insert(element_id, text::paragraph_style(style)?);
        text_owners.push(TextOwner {
            element: element_id,
            text: text_id,
            color,
        });
    }
    tree::validate_box_style(
        style_by_node
            .get(&root.id())
            .copied()
            .ok_or_else(|| layout_failure("missing_computed_style", Some(root.id().get()), None))?,
        "html",
        320.0,
        240.0,
    )?;
    let body = nodes.body;
    let main = nodes.main;
    tree::validate_box_style(
        style_by_node
            .get(&body)
            .copied()
            .ok_or_else(|| layout_failure("missing_computed_style", Some(body.get()), None))?,
        "body",
        320.0,
        20.0,
    )?;
    tree::validate_box_style(
        style_by_node
            .get(&main)
            .copied()
            .ok_or_else(|| layout_failure("missing_computed_style", Some(main.get()), None))?,
        "main",
        320.0,
        20.0,
    )?;
    tree::validate_hidden_style(style_by_node.get(&nodes.not_rendered).copied().ok_or_else(
        || {
            layout_failure(
                "missing_computed_style",
                Some(nodes.not_rendered.get()),
                None,
            )
        },
    )?)?;

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).map_err(|error| {
        layout_failure(
            "root_html_text_font_registration",
            None,
            Some(error.to_string()),
        )
    })?;
    let owners = text_owners
        .try_into()
        .map_err(|_| layout_failure("root_html_text_owner_count", None, None))?;
    Ok(Input {
        body: nodes.body,
        main: nodes.main,
        owners,
        paragraph_styles,
        engine,
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
    if expected_key.generation() != snapshot.generation()
        || expected_key.document_revision() != snapshot.document_revision()
        || expected_key.render_tree_revision() != snapshot.render_tree_revision()
        || expected_key.style_revision() != output.computed_styles.style_revision
        || expected_key.environment_revision()
            != output.computed_styles.viewport.environment_revision
        || scene.viewport_css_px().width().to_bits() != VIEWPORT_WIDTH.to_bits()
        || scene.viewport_css_px().height().to_bits() != VIEWPORT_HEIGHT.to_bits()
        || output.layout.revision.source() != expected_source
    {
        return Err(layout_failure(
            "root_html_text_paint_revision_mismatch",
            None,
            None,
        ));
    }
    if root.generation() != snapshot.generation() {
        return Err(layout_failure(
            "root_html_text_paint_revision_mismatch",
            None,
            None,
        ));
    }

    let root_frame = *output.layout.frames.get(&root.id()).ok_or_else(|| {
        layout_failure(
            "root_html_text_root_frame_missing",
            Some(root.id().get()),
            None,
        )
    })?;
    text::expect_frame(
        root_frame,
        [0.0, 0.0, VIEWPORT_WIDTH, VIEWPORT_HEIGHT],
        root.id(),
    )?;
    for (node, expected) in [
        (input.body, [0.0, 0.0, VIEWPORT_WIDTH, 20.0]),
        (input.main, [0.0, 0.0, VIEWPORT_WIDTH, 20.0]),
    ] {
        let frame = *output.layout.frames.get(&node).ok_or_else(|| {
            layout_failure(
                "root_html_text_container_frame_missing",
                Some(node.get()),
                None,
            )
        })?;
        text::expect_frame(frame, expected, node)?;
    }

    let mut texts = Vec::with_capacity(input.owners.len());
    for owner in input.owners {
        let frame = *output.layout.frames.get(&owner.element).ok_or_else(|| {
            layout_failure(
                "root_html_text_owner_frame_missing",
                Some(owner.element.get()),
                None,
            )
        })?;
        text::validate_frame(frame, owner.element)?;
        if (frame.width - 15.0).abs() > 0.05 || (frame.height - 10.0).abs() > 0.05 {
            return Err(layout_failure(
                "root_html_text_owner_geometry_mismatch",
                Some(owner.element.get()),
                None,
            ));
        }
        let layout = output
            .layout
            .text_layouts
            .get(&owner.element)
            .ok_or_else(|| {
                layout_failure(
                    "root_html_text_layout_missing",
                    Some(owner.element.get()),
                    None,
                )
            })?;
        text::validate_text_layout(snapshot, root, owner, layout, output.layout.revision)?;
        let raster = input
            .engine
            .rasterize_glyphs(&layout.layout, DEVICE_SCALE_FACTOR)
            .map_err(|error| {
                layout_failure(
                    "root_html_text_raster_failed",
                    Some(owner.element.get()),
                    Some(error.to_string()),
                )
            })?;
        let glyphs = text::runtime_glyphs(owner.element, owner.text, &layout.layout, &raster)?;
        let render_box = scene
            .boxes()
            .iter()
            .find(|render_box| render_box.node_id() == owner.element)
            .copied()
            .ok_or_else(|| {
                layout_failure(
                    "root_html_text_render_owner_missing",
                    Some(owner.element.get()),
                    None,
                )
            })?;
        texts.push(
            RuntimeRenderText::new(
                owner.element,
                expected_key,
                frame.x,
                frame.y,
                DEVICE_SCALE_FACTOR,
                owner.color,
                CssClip::unconstrained(),
                glyphs,
            )
            .map_err(|error| {
                layout_failure(
                    "root_html_text_snapshot_failed",
                    Some(owner.element.get()),
                    Some(error.to_string()),
                )
            })?,
        );
        let commands = scene.paint_commands();
        if !commands.iter().any(|command| {
            matches!(command, RuntimePaintCommand::DrawBox { index } if scene.boxes()[*index].node_id() == render_box.node_id())
        }) {
            return Err(layout_failure(
                "root_html_text_paint_order_invalid",
                Some(owner.element.get()),
                None,
            ));
        }
    }

    let mut paint_commands = Vec::with_capacity(scene.paint_commands().len() + texts.len());
    let mut inserted_texts = BTreeSet::new();
    for command in scene.paint_commands() {
        paint_commands.push(*command);
        if let RuntimePaintCommand::DrawBox { index } = command {
            let node_id = scene.boxes()[*index].node_id();
            if let Some(text_index) = texts.iter().position(|text| text.node_id() == node_id) {
                if !inserted_texts.insert(node_id) {
                    return Err(layout_failure(
                        "root_html_text_paint_order_invalid",
                        Some(node_id.get()),
                        None,
                    ));
                }
                paint_commands.push(RuntimePaintCommand::DrawText { index: text_index });
            }
        }
    }
    if inserted_texts.len() != texts.len() {
        return Err(layout_failure(
            "root_html_text_paint_order_invalid",
            None,
            None,
        ));
    }
    scene = scene
        .with_texts_and_paint_commands(texts, paint_commands)
        .map_err(|error| {
            layout_failure(
                "root_html_text_scene_attach_failed",
                None,
                Some(error.to_string()),
            )
        })?;
    for owner in input.owners {
        let render_box = scene
            .boxes()
            .iter()
            .find(|render_box| render_box.node_id() == owner.element)
            .ok_or_else(|| {
                layout_failure(
                    "root_html_text_render_owner_missing",
                    Some(owner.element.get()),
                    None,
                )
            })?;
        let text = scene
            .texts()
            .iter()
            .find(|text| text.node_id() == owner.element)
            .ok_or_else(|| {
                layout_failure(
                    "root_html_text_snapshot_missing",
                    Some(owner.element.get()),
                    None,
                )
            })?;
        if text.clip_css_px() != render_box.clip_css_px() {
            return Err(layout_failure(
                "root_html_text_owner_clip_mismatch",
                Some(owner.element.get()),
                None,
            ));
        }
    }
    Ok(scene)
}
