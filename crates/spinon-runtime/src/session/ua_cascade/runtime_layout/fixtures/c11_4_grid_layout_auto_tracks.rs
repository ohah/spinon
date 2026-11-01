use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, NodeId};
use spinon_layout::{
    FontStyleMode, GridContainerInput, LineBreakMode, ParagraphStyle, TextLayoutEngine,
    TextParagraphDirection, WhiteSpaceMode, WordBreakMode,
};
use spinon_style::{ComputedCssDimension, ComputedStyleProfile, ComputedStyleSnapshot};

use super::super::{
    RuntimeC11_4GridLayoutAutoTracksObservation, RuntimeLayoutCompleted, RuntimeLayoutFailure,
    layout_failure,
};
use super::C11_4TextInput;
use super::c11_4_image::element_identity;
use super::c11_4_nested_grid_replaced_item::{
    HTML_NAMESPACE, children, spacing_is, style_for, unsupported, zero_edges,
};

const SCENE_ID: &str = "grid-layout-auto-tracks-c114-wpt";

#[path = "c11_4_grid_layout_auto_tracks/validation.rs"]
mod validation;
const ROOT_ID: &str = "c114-grid-layout-auto-tracks-grid";
const A_ID: &str = "c114-grid-layout-auto-tracks-a";
const B_ID: &str = "c114-grid-layout-auto-tracks-b";
const B_CONTENT_ID: &str = "c114-grid-layout-auto-tracks-b-content";
const C_ID: &str = "c114-grid-layout-auto-tracks-c";
const AHEM: &[u8] = include_bytes!("../../../../../../../tests/fixtures/resources/c15/Ahem.ttf");

pub(in crate::session::ua_cascade::runtime_layout) struct Input {
    pub(in crate::session::ua_cascade::runtime_layout) node_ids: [NodeId; 5],
    pub(in crate::session::ua_cascade::runtime_layout) text_input: C11_4TextInput,
}

pub(in crate::session::ua_cascade::runtime_layout) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<Input>, RuntimeLayoutFailure> {
    let (namespace, tag, root_id) = element_identity(snapshot, root)?;
    if root_id != ROOT_ID {
        return Ok(None);
    }
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || namespace != HTML_NAMESPACE
        || tag != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
    {
        return Err(unsupported(root.id(), SCENE_ID, "profile or root identity"));
    }

    let root_children = children(snapshot, root)?;
    require_child_count(root.id(), root_children.len(), 3, "direct grid item count")?;
    let a = root_children[0];
    let b = root_children[1];
    let c = root_children[2];
    for (item, expected_id) in [(a, A_ID), (b, B_ID), (c, C_ID)] {
        if snapshot.parent(item) != Some(HostParent::Node(root)) {
            return Err(unsupported(
                item.id(),
                SCENE_ID,
                "grid item is not a direct child",
            ));
        }
        validate_element_identity(snapshot, item, expected_id)?;
    }

    let b_children = children(snapshot, b)?;
    require_child_count(b.id(), b_children.len(), 1, "B nested content count")?;
    let b_content = b_children[0];
    if snapshot.parent(b_content) != Some(HostParent::Node(b)) {
        return Err(unsupported(
            b_content.id(),
            SCENE_ID,
            "B content ownership changed",
        ));
    }
    validate_element_identity(snapshot, b_content, B_CONTENT_ID)?;

    require_single_nbsp_text(snapshot, a)?;
    require_single_nbsp_text(snapshot, b_content)?;
    require_single_nbsp_text(snapshot, c)?;
    validation::validate_root_style(style_for(styles, root)?)?;
    validation::validate_grid_item_style(style_for(styles, a)?, A_ID, 1, 1)?;
    validation::validate_grid_item_style(style_for(styles, b)?, B_ID, 1, 2)?;
    validation::validate_grid_item_style(style_for(styles, c)?, C_ID, 2, 1)?;
    validation::validate_text_block_style(style_for(styles, a)?, ComputedCssDimension::Auto, A_ID)?;
    validation::validate_text_block_style(
        style_for(styles, b_content)?,
        ComputedCssDimension::LengthPx(50.0),
        B_CONTENT_ID,
    )?;
    validation::validate_text_block_style(style_for(styles, c)?, ComputedCssDimension::Auto, C_ID)?;

    let node_ids = [root.id(), a.id(), b.id(), b_content.id(), c.id()];
    if node_ids.iter().copied().collect::<BTreeSet<_>>().len() != node_ids.len() {
        return Err(unsupported(
            root.id(),
            SCENE_ID,
            "duplicate fixture node identity",
        ));
    }
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).map_err(|error| {
        layout_failure(
            "c114_grid_auto_tracks_font_registration",
            Some(root.id().get()),
            Some(error.to_string()),
        )
    })?;
    Ok(Some(Input {
        node_ids,
        text_input: C11_4TextInput {
            paragraph_styles: BTreeMap::from([
                (a.id(), ahem_paragraph_style()),
                (b_content.id(), ahem_paragraph_style()),
                (c.id(), ahem_paragraph_style()),
            ]),
            engine,
            two_axis_nested_grid: false,
            content_box_ratio_node: None,
            aspect_ratio_minmax_node: None,
            definite_axis_ratio_node: None,
        },
    }))
}

pub(in crate::session::ua_cascade::runtime_layout) fn observation(
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    grid_container_inputs: &BTreeMap<NodeId, GridContainerInput>,
    node_ids: [NodeId; 5],
) -> Result<RuntimeC11_4GridLayoutAutoTracksObservation, RuntimeLayoutFailure> {
    validation::validate_root_style(style_for(styles, root)?)?;
    let grid_input = grid_container_inputs.get(&root.id()).ok_or_else(|| {
        layout_failure(
            "c114_grid_auto_tracks_grid_input_missing",
            Some(root.id().get()),
            None,
        )
    })?;
    validation::validate_grid_input(root.id(), grid_input)?;
    if grid_container_inputs.len() != 1 {
        return Err(unsupported(
            root.id(),
            SCENE_ID,
            "unexpected explicit Grid container input",
        ));
    }
    Ok(RuntimeC11_4GridLayoutAutoTracksObservation {
        style_snapshot: styles.clone(),
        grid_root: root.id(),
        grid_container_input: grid_input.clone(),
        node_ids,
    })
}

fn require_single_nbsp_text(
    snapshot: &HostDocumentSnapshot,
    owner: HostNodeHandle,
) -> Result<NodeId, RuntimeLayoutFailure> {
    let children = children(snapshot, owner)?;
    require_child_count(
        owner.id(),
        children.len(),
        1,
        "one direct NBSP text node required",
    )?;
    let text = children[0];
    let value = snapshot
        .node(text)
        .and_then(|node| match node.kind() {
            HostNodeKind::Text(text) => Some(text.to_string_lossy()),
            HostNodeKind::Element(_) => None,
        })
        .ok_or_else(|| unsupported(text.id(), SCENE_ID, "text node required"))?;
    if snapshot.parent(text) != Some(HostParent::Node(owner)) || value != "\u{00a0}" {
        return Err(unsupported(
            text.id(),
            SCENE_ID,
            "text source or ownership changed",
        ));
    }
    Ok(text.id())
}

fn validate_element_identity(
    snapshot: &HostDocumentSnapshot,
    node: HostNodeHandle,
    expected_id: &str,
) -> Result<(), RuntimeLayoutFailure> {
    let (namespace, tag, id) = element_identity(snapshot, node)?;
    if namespace != HTML_NAMESPACE || tag != "div" || id != expected_id {
        return Err(unsupported(
            node.id(),
            SCENE_ID,
            "element namespace, tag, or ID changed",
        ));
    }
    Ok(())
}

fn ahem_paragraph_style() -> ParagraphStyle {
    ParagraphStyle {
        overflow_wrap: Default::default(),
        writing_mode: spinon_core::WritingMode::HorizontalTb,
        text_orientation: spinon_core::TextOrientation::Mixed,
        font_family: "Ahem".to_owned(),
        font_size: 10.0,
        line_height: 10.0,
        line_height_is_normal: false,
        font_weight: 400.0,
        font_style: FontStyleMode::Normal,
        white_space: WhiteSpaceMode::Normal,
        word_break: WordBreakMode::Normal,
        hyphens: None,
        locale: None,
        direction: TextParagraphDirection::Ltr,
        line_break: LineBreakMode::Strict,
        ligatures: spinon_layout::LigatureMode::Normal,
        letter_spacing: 0.0,
        word_spacing: 0.0,
        text_indent: 0.0,
        text_indent_flags: spinon_layout::TextIndentFlags::default(),
        text_transform: Default::default(),
    }
}

fn require_child_count(
    node: NodeId,
    actual: usize,
    expected: usize,
    detail: &str,
) -> Result<(), RuntimeLayoutFailure> {
    if actual != expected {
        return Err(unsupported(node, SCENE_ID, detail));
    }
    Ok(())
}

pub(in crate::session::ua_cascade::runtime_layout) fn validate_output(
    completed: &RuntimeLayoutCompleted,
) -> Result<(), RuntimeLayoutFailure> {
    validation::validate_output(completed)
}
