use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, NodeId};
use spinon_layout::TextLayoutEngine;
use spinon_render::RuntimeSrgbColor;
use spinon_style::ComputedStyleSnapshot;

use super::{Input, tree::collect_handles};
use crate::session::ua_cascade::runtime_layout::{RuntimeLayoutFailure, layout_failure};

pub(in crate::session::ua_cascade::runtime_layout) fn prepare_text_owner(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    owner: NodeId,
    engine: &TextLayoutEngine,
    color: RuntimeSrgbColor,
) -> Result<Input, RuntimeLayoutFailure> {
    if styles.profile != spinon_style::ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || !styles.viewport.device_scale_factor.is_finite()
        || styles.viewport.device_scale_factor <= 0.0
        || root.generation() != snapshot.generation()
    {
        return Err(layout_failure(
            "invalid_fixture_text_render_environment",
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
            "duplicate_fixture_text_computed_style",
            Some(root.id().get()),
            None,
        ));
    }
    if !style_by_node.contains_key(&owner) {
        return Err(layout_failure(
            "fixture_text_owner_style_missing",
            Some(owner.get()),
            None,
        ));
    }
    let node_handles = collect_handles(snapshot, root)?;
    if !node_handles.contains_key(&owner) {
        return Err(layout_failure(
            "fixture_text_owner_missing",
            Some(owner.get()),
            None,
        ));
    }

    Ok(Input {
        paragraph_styles: BTreeMap::new(),
        inline_run_styles: BTreeMap::new(),
        engine: engine.clone(),
        style_colors: BTreeMap::from([(owner, color)]),
        node_handles,
        device_scale_factor: styles.viewport.device_scale_factor,
    })
}
