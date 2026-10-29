use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, NodeId};
use spinon_layout::LayoutGridTrackSizingAxis;
use spinon_style::{ComputedElementStyle, ComputedStyleSnapshot};

use super::super::super::{
    RuntimeLayoutCompleted, RuntimeLayoutFailure, RuntimeUaCascadeKey, layout_failure,
};
use super::super::c11_4_nested_grid_replaced_item::{HTML_NAMESPACE, unsupported};
use super::Case;

#[path = "style_validation.rs"]
mod style_validation;
#[path = "trace_validation.rs"]
mod trace_validation;

const ROOT_HEIGHT: f32 = 60.0;
const ITEM_HEIGHT: f32 = 10.0;
const CHILD_WIDTH: f32 = 100.0;
const EPSILON: f32 = 1.0e-4;

pub(super) fn validate_root_style(
    style: &ComputedElementStyle,
    case: Case,
) -> Result<(), RuntimeLayoutFailure> {
    style_validation::validate_root_style(style, case)
}

pub(super) fn validate_item_style(
    style: &ComputedElementStyle,
    case: Case,
) -> Result<(), RuntimeLayoutFailure> {
    style_validation::validate_item_style(style, case)
}

pub(super) fn validate_child_style(
    style: &ComputedElementStyle,
    case: Case,
) -> Result<(), RuntimeLayoutFailure> {
    style_validation::validate_child_style(style, case)
}

pub(super) fn validate_grid_input(
    root: NodeId,
    input: &spinon_layout::GridContainerInput,
    case: Case,
) -> Result<(), RuntimeLayoutFailure> {
    trace_validation::validate_grid_input(root, input, case)
}

pub(super) fn validate_columns_pass(
    pass: &spinon_layout::LayoutGridTrackSizingPassTrace,
    item: NodeId,
    case: Case,
) -> Result<(), RuntimeLayoutFailure> {
    trace_validation::validate_columns_pass(pass, item, case)
}

pub(super) fn validate_rows_pass(
    pass: &spinon_layout::LayoutGridTrackSizingPassTrace,
) -> Result<(), RuntimeLayoutFailure> {
    trace_validation::validate_rows_pass(pass)
}

pub(super) fn validate_output(
    completed: &RuntimeLayoutCompleted,
) -> Result<(), RuntimeLayoutFailure> {
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .ok_or_else(|| failure("layout diagnostics missing"))?;
    let observation = diagnostics
        .fit_content_limit_columns
        .as_ref()
        .ok_or_else(|| failure("computed style and typed Grid input observation missing"))?;
    let case = super::case_for_scene(observation.scene_id)
        .ok_or_else(|| failure("unknown column limit boundary observation"))?;
    if observation.limit_css_px != case.limit
        || observation.expected_column_css_px != case.expected_column
        || observation.automatic_minimum != case.automatic_minimum
    {
        return Err(failure(
            "observation expectations differ from the pinned case",
        ));
    }
    if !revisions_match(
        observation.style_snapshot.style_revision,
        observation.style_snapshot.generation.get(),
        observation.style_snapshot.document_revision.get(),
        observation.style_snapshot.render_tree_revision.get(),
        diagnostics.revision,
        completed.key,
    ) {
        return Err(failure(
            "style snapshot revision does not match layout output",
        ));
    }
    if observation.style_snapshot.profile
        != spinon_style::ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || observation.node_ids[0] != observation.grid_root
        || observation
            .node_ids
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            .len()
            != observation.node_ids.len()
    {
        return Err(failure(
            "fixture identity or computed style profile changed",
        ));
    }
    let root_style = style_for_node(&observation.style_snapshot, observation.grid_root)?;
    let item_style = style_for_node(&observation.style_snapshot, observation.node_ids[1])?;
    let child_style = style_for_node(&observation.style_snapshot, observation.node_ids[2])?;
    validate_root_style(root_style, case)?;
    validate_item_style(item_style, case)?;
    validate_child_style(child_style, case)?;
    validate_grid_input(
        observation.grid_root,
        &observation.grid_container_input,
        case,
    )?;

    let relevant = completed
        .frames
        .iter()
        .filter(|frame| {
            observation
                .node_ids
                .iter()
                .any(|node| node.get() == frame.node_id)
        })
        .collect::<Vec<_>>();
    let frames = relevant
        .iter()
        .map(|frame| (frame.node_id, (frame.x, frame.y, frame.width, frame.height)))
        .collect::<BTreeMap<_, _>>();
    if relevant.len() != observation.node_ids.len() || frames.len() != observation.node_ids.len() {
        return Err(failure("missing, duplicate, or extra fixture node frames"));
    }
    for (node, expected) in observation.node_ids.iter().zip([
        (0.0, 0.0, case.root_width, ROOT_HEIGHT),
        (0.0, 0.0, case.expected_column, ITEM_HEIGHT),
        (0.0, 0.0, CHILD_WIDTH, ITEM_HEIGHT),
    ]) {
        if !frames
            .get(&node.get())
            .is_some_and(|actual| frame_matches(*actual, expected))
        {
            return Err(failure(
                "layout frame differs from the pinned CSS Grid formula",
            ));
        }
    }

    if diagnostics.traces.len() != 1
        || diagnostics.traces[0].node != observation.grid_root
        || diagnostics.traces[0].passes.len() != 2
    {
        return Err(failure(
            "expected one root trace with Columns and Rows passes",
        ));
    }
    let columns = diagnostics.traces[0]
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .ok_or_else(|| failure("column sizing pass missing"))?;
    let rows = diagnostics.traces[0]
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .ok_or_else(|| failure("row sizing pass missing"))?;
    validate_columns_pass(columns, observation.node_ids[1], case)?;
    validate_rows_pass(rows)
}

pub(super) fn validate_identity(
    snapshot: &HostDocumentSnapshot,
    node: HostNodeHandle,
    expected_id: &str,
    case: Case,
) -> Result<(), RuntimeLayoutFailure> {
    let (namespace, tag, id) = super::super::c11_4_image::element_identity(snapshot, node)?;
    if namespace != HTML_NAMESPACE || tag != "div" || id != expected_id {
        return Err(unsupported(
            node.id(),
            case.scene_id,
            "element namespace, tag, or ID changed",
        ));
    }
    Ok(())
}

pub(super) fn require_count(
    node: NodeId,
    actual: usize,
    expected: usize,
    case: Case,
    detail: &str,
) -> Result<(), RuntimeLayoutFailure> {
    if actual != expected {
        return Err(unsupported(node, case.scene_id, detail));
    }
    Ok(())
}

fn style_for_node(
    styles: &ComputedStyleSnapshot,
    node: NodeId,
) -> Result<&ComputedElementStyle, RuntimeLayoutFailure> {
    let mut matches = styles.elements.iter().filter(|style| style.node_id == node);
    let style = matches
        .next()
        .ok_or_else(|| layout_failure("missing_computed_style", Some(node.get()), None))?;
    if matches.next().is_some() {
        return Err(layout_failure(
            "duplicate_computed_style",
            Some(node.get()),
            None,
        ));
    }
    Ok(style)
}

fn frame_matches(actual: (f32, f32, f32, f32), expected: (f32, f32, f32, f32)) -> bool {
    [actual.0, actual.1, actual.2, actual.3]
        .into_iter()
        .zip([expected.0, expected.1, expected.2, expected.3])
        .all(|(actual, expected)| (actual - expected).abs() <= EPSILON)
}

fn revisions_match(
    style_revision: spinon_core::StyleRevision,
    generation: u64,
    document_revision: u64,
    render_tree_revision: u64,
    layout_revision: spinon_layout::LayoutInputRevision,
    key: RuntimeUaCascadeKey,
) -> bool {
    let source_matches = match layout_revision.source() {
        spinon_layout::LayoutSourceRevision::HostDocument {
            generation: layout_generation,
            document,
            render_tree,
        } => {
            layout_generation.get() == generation
                && document.get() == document_revision
                && render_tree.get() == render_tree_revision
                && generation == key.generation
                && document_revision == key.document_revision
                && render_tree_revision == key.render_tree_revision
        }
        spinon_layout::LayoutSourceRevision::Tree(_) => false,
    };
    source_matches
        && style_revision == layout_revision.style()
        && style_revision == key.style_revision
        && layout_revision.environment().get() == key.environment_revision
}

fn failure(detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "c114_fit_content_limit_columns_output_mismatch",
        None,
        Some(detail.to_owned()),
    )
}
