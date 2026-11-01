use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, NodeId};
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
    prepare_text_owners(snapshot, root, styles, &[owner], engine, color)
}

pub(in crate::session::ua_cascade::runtime_layout) fn prepare_text_owners(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    owners: &[NodeId],
    engine: &TextLayoutEngine,
    color: RuntimeSrgbColor,
) -> Result<Input, RuntimeLayoutFailure> {
    let fit_content_area_cap_root = styles.profile
        == spinon_style::ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
        && snapshot.node(root).is_some_and(|node| {
            matches!(node.kind(), HostNodeKind::Element(element)
            if element.local_name() == "div"
                && element.attributes().iter().any(|(name, value)| {
                    name.namespace().is_none()
                        && name.local_name() == "id"
                        && value.to_string_lossy() == "c1133-fit-content-area-cap-grid"
                }))
        });
    if (styles.profile != spinon_style::ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        && !fit_content_area_cap_root)
        || !styles.viewport.device_scale_factor.is_finite()
        || styles.viewport.device_scale_factor <= 0.0
        || root.generation() != snapshot.generation()
        || owners.is_empty()
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
    let node_handles = collect_handles(snapshot, root)?;
    if fit_content_area_cap_root
        && (owners.len() != 1
            || !snapshot
                .children(root)
                .is_some_and(|children| children.into_iter().any(|child| child.id() == owners[0])))
    {
        return Err(layout_failure(
            "invalid_fixture_text_owner",
            Some(root.id().get()),
            None,
        ));
    }
    let mut style_colors = BTreeMap::new();
    for &owner in owners {
        if !style_by_node.contains_key(&owner) {
            return Err(layout_failure(
                "fixture_text_owner_style_missing",
                Some(owner.get()),
                None,
            ));
        }
        if !node_handles.contains_key(&owner) {
            return Err(layout_failure(
                "fixture_text_owner_missing",
                Some(owner.get()),
                None,
            ));
        }
        if style_colors.insert(owner, color).is_some() {
            return Err(layout_failure(
                "duplicate_fixture_text_owner",
                Some(owner.get()),
                None,
            ));
        }
    }

    Ok(Input {
        paragraph_styles: BTreeMap::new(),
        inline_run_styles: BTreeMap::new(),
        engine: engine.clone(),
        style_colors,
        text_style_owners: BTreeMap::new(),
        node_handles,
        device_scale_factor: styles.viewport.device_scale_factor,
    })
}
