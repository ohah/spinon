use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostParent, NodeId};
use spinon_layout::{
    GridAutoFlow, GridAutoRows, GridContainerInput, GridPlacementMode, GridTrackLengthPercentage,
    GridTrackSize, LayoutGridTrackSizingAxis, LayoutGridTrackSizingPassTrace,
    LayoutGridTrackSizingStageKind, LayoutGridTrackSizingStageTrace,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize,
    ComputedCssMaxSize, ComputedElementStyle, ComputedStyleProfile, ComputedStyleSnapshot,
};

use super::super::{
    RuntimeC11_4FitContentMinHeightObservation, RuntimeLayoutCompleted, RuntimeLayoutFailure,
    RuntimeUaCascadeKey, layout_failure,
};
use super::c11_4_image::element_identity;
use super::c11_4_nested_grid_replaced_item::{
    HTML_NAMESPACE, children, spacing_is, style_for, unsupported, zero_edges,
};

const SCENE_ID: &str = "fit-content-min-height-wpt";
const ROOT_ID: &str = "c114-fit-content-min-height-grid";
const ITEM_ID: &str = "c114-fit-content-min-height-item";
const CHILD_ID: &str = "c114-fit-content-min-height-child";

pub(in crate::session::ua_cascade::runtime_layout) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<[NodeId; 3]>, RuntimeLayoutFailure> {
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
    require_child_count(root.id(), root_children.len(), 1, "direct grid item count")?;
    let item = root_children[0];
    if snapshot.parent(item) != Some(HostParent::Node(root)) {
        return Err(unsupported(
            item.id(),
            SCENE_ID,
            "grid item is not a direct child",
        ));
    }
    validate_element_identity(snapshot, item, ITEM_ID)?;
    let item_children = children(snapshot, item)?;
    require_child_count(
        item.id(),
        item_children.len(),
        1,
        "nested block child count",
    )?;
    let child = item_children[0];
    if snapshot.parent(child) != Some(HostParent::Node(item)) {
        return Err(unsupported(
            child.id(),
            SCENE_ID,
            "nested child ownership changed",
        ));
    }
    validate_element_identity(snapshot, child, CHILD_ID)?;

    let node_ids = [root.id(), item.id(), child.id()];
    let styles_for_nodes = [
        style_for(styles, root)?,
        style_for(styles, item)?,
        style_for(styles, child)?,
    ];
    validate_root_style(styles_for_nodes[0])?;
    validate_item_style(styles_for_nodes[1])?;
    validate_child_style(styles_for_nodes[2])?;
    let style_ids = styles
        .elements
        .iter()
        .map(|style| style.node_id)
        .collect::<std::collections::BTreeSet<_>>();
    if style_ids.len() != node_ids.len()
        || node_ids.iter().any(|node| !style_ids.contains(node))
        || styles.elements.len() != style_ids.len()
    {
        return Err(unsupported(
            root.id(),
            SCENE_ID,
            "computed style node set changed",
        ));
    }
    Ok(Some(node_ids))
}

pub(in crate::session::ua_cascade::runtime_layout) fn observation(
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    grid_container_inputs: &std::collections::BTreeMap<NodeId, GridContainerInput>,
    node_ids: [NodeId; 3],
) -> Result<RuntimeC11_4FitContentMinHeightObservation, RuntimeLayoutFailure> {
    validate_root_style(style_for(styles, root)?)?;
    let grid_input = grid_container_inputs.get(&root.id()).ok_or_else(|| {
        layout_failure(
            "c114_fit_content_min_height_grid_input_missing",
            Some(root.id().get()),
            None,
        )
    })?;
    validate_grid_input(root.id(), grid_input)?;
    if grid_container_inputs.len() != 1 || node_ids[0] != root.id() {
        return Err(unsupported(
            root.id(),
            SCENE_ID,
            "unexpected Grid input or root identity",
        ));
    }
    Ok(RuntimeC11_4FitContentMinHeightObservation {
        style_snapshot: styles.clone(),
        grid_root: root.id(),
        grid_container_input: grid_input.clone(),
        node_ids,
    })
}

pub(in crate::session::ua_cascade::runtime_layout) fn validate_output(
    completed: &RuntimeLayoutCompleted,
) -> Result<(), RuntimeLayoutFailure> {
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .ok_or_else(|| failure("layout diagnostics missing"))?;
    let observation = diagnostics
        .fit_content_min_height
        .as_ref()
        .ok_or_else(|| failure("computed style and typed Grid input observation missing"))?;
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
    if observation.style_snapshot.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || observation.node_ids[0] != observation.grid_root
        || observation
            .node_ids
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != observation.node_ids.len()
    {
        return Err(failure(
            "fixture identity or computed style profile changed",
        ));
    }
    validate_root_style(style_for_node(
        &observation.style_snapshot,
        observation.grid_root,
    )?)?;
    validate_item_style(style_for_node(
        &observation.style_snapshot,
        observation.node_ids[1],
    )?)?;
    validate_child_style(style_for_node(
        &observation.style_snapshot,
        observation.node_ids[2],
    )?)?;
    validate_grid_input(observation.grid_root, &observation.grid_container_input)?;

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
        .collect::<std::collections::BTreeMap<_, _>>();
    if relevant.len() != observation.node_ids.len() || frames.len() != observation.node_ids.len() {
        return Err(failure("missing, duplicate, or extra fixture node frames"));
    }
    for (node, expected) in observation.node_ids.iter().zip([
        (0.0, 0.0, 60.0, 300.0),
        (0.0, 0.0, 60.0, 10.0),
        (0.0, 0.0, 60.0, 10.0),
    ]) {
        if frames.get(&node.get()) != Some(&expected) {
            return Err(failure(
                "layout frame differs from pinned CSS-pixel geometry",
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
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .count();
    let mut rows = diagnostics.traces[0]
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows);
    let row_pass = rows
        .next()
        .ok_or_else(|| failure("row sizing pass missing"))?;
    if columns != 1 || rows.next().is_some() {
        return Err(failure("expected exactly one Columns and one Rows pass"));
    }
    validate_rows_pass(row_pass)
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

fn validate_root_style(style: &ComputedElementStyle) -> Result<(), RuntimeLayoutFailure> {
    let grid = style.grid_layout.as_ref().ok_or_else(|| {
        layout_failure(
            "missing_computed_grid_layout",
            Some(style.node_id.get()),
            None,
        )
    })?;
    let expected_auto_rows =
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::FitContent(
            ComputedCssGridTrackLengthPercentage::LengthPx(40.0),
        )]);
    if style.properties.get("display").map(String::as_str) != Some("grid")
        || style.properties.get("height").map(String::as_str) != Some("auto")
        || style.properties.get("min-height").map(String::as_str) != Some("300px")
        || style.properties.get("align-content").map(String::as_str) != Some("normal")
        || style.properties.get("grid-auto-flow").map(String::as_str) != Some("row")
        || style.properties.get("grid-auto-rows").map(String::as_str) != Some("fit-content(40px)")
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(60.0)
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::LengthPx(300.0)
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_edges(style.layout_spacing.padding)
        || !zero_edges(style.layout_spacing.margin)
        || !spacing_is(style.layout_spacing.row_gap, 0.0)
        || !spacing_is(style.layout_spacing.column_gap, 0.0)
        || style.layout_border != spinon_style::ComputedLayoutBorder::default()
        || grid.columns != [ComputedCssGridTrackSize::LengthPx(60.0)]
        || !grid.rows.is_empty()
        || grid.auto_rows != expected_auto_rows
        || grid.auto_columns != ComputedCssGridAutoColumns::Auto
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || !empty_line_name_groups(&grid.columns_line_names, 2)
        || !grid.rows_line_names.is_empty()
    {
        return Err(unsupported(
            style.node_id,
            SCENE_ID,
            "computed root style changed",
        ));
    }
    Ok(())
}

fn validate_item_style(style: &ComputedElementStyle) -> Result<(), RuntimeLayoutFailure> {
    if style.properties.get("display").map(String::as_str) != Some("block")
        || style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_edges(style.layout_spacing.padding)
        || !zero_edges(style.layout_spacing.margin)
        || style.layout_border != spinon_style::ComputedLayoutBorder::default()
        || !matches!(
            style.grid_item_placement.as_ref(),
            Some(placement)
                if placement.row_start == spinon_style::ComputedCssGridPlacement::Auto
                    && placement.row_end == spinon_style::ComputedCssGridPlacement::Auto
                    && placement.column_start == spinon_style::ComputedCssGridPlacement::Auto
                    && placement.column_end == spinon_style::ComputedCssGridPlacement::Auto
        )
    {
        return Err(unsupported(
            style.node_id,
            SCENE_ID,
            "grid item style changed",
        ));
    }
    Ok(())
}

fn validate_child_style(style: &ComputedElementStyle) -> Result<(), RuntimeLayoutFailure> {
    if style.properties.get("display").map(String::as_str) != Some("block")
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(60.0)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(10.0)
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_edges(style.layout_spacing.padding)
        || !zero_edges(style.layout_spacing.margin)
        || style.layout_border != spinon_style::ComputedLayoutBorder::default()
    {
        return Err(unsupported(
            style.node_id,
            SCENE_ID,
            "nested child style changed",
        ));
    }
    Ok(())
}

fn validate_grid_input(
    root: NodeId,
    input: &GridContainerInput,
) -> Result<(), RuntimeLayoutFailure> {
    if input.columns != [GridTrackSize::LengthPx(60.0)]
        || !input.rows.is_empty()
        || input.auto_rows
            != GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
                GridTrackLengthPercentage::LengthPx(40.0),
            )])
        || input.auto_flow != GridAutoFlow::Row
        || input.placement_mode != GridPlacementMode::TrackSizing
        || input.columns_auto_repeat.is_some()
        || input.rows_auto_repeat.is_some()
        || input.template_areas.is_some()
        || !empty_line_name_groups(&input.columns_line_names, 2)
        || !input.rows_line_names.is_empty()
    {
        return Err(unsupported(
            root,
            SCENE_ID,
            "typed LayoutInput Grid container changed",
        ));
    }
    Ok(())
}

fn validate_rows_pass(pass: &LayoutGridTrackSizingPassTrace) -> Result<(), RuntimeLayoutFailure> {
    let expected_stages = [
        (LayoutGridTrackSizingStageKind::Initialize, 0.0),
        (
            LayoutGridTrackSizingStageKind::IntrinsicItemBatch {
                span: 1,
                crosses_flexible_track: false,
            },
            10.0,
        ),
        (
            LayoutGridTrackSizingStageKind::IntrinsicSizingComplete,
            10.0,
        ),
        (LayoutGridTrackSizingStageKind::MaximizeTracks, 10.0),
        (LayoutGridTrackSizingStageKind::ExpandFlexibleTracks, 10.0),
        (
            LayoutGridTrackSizingStageKind::StretchAutoTracks { applied: true },
            10.0,
        ),
    ];
    if pass.stages.len() != expected_stages.len() {
        return Err(failure(
            "row sizing stage count differs from the pinned trace",
        ));
    }
    for (stage, (expected_kind, expected_base)) in pass.stages.iter().zip(expected_stages) {
        if stage.stage != expected_kind || !track_bases_match(stage, expected_base) {
            return Err(failure(
                "row sizing stage sequence or fit-content base differs from the pinned trace",
            ));
        }
    }
    Ok(())
}

fn track_bases_match(stage: &LayoutGridTrackSizingStageTrace, expected: f32) -> bool {
    let tracks = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .collect::<Vec<_>>();
    tracks.len() == 1 && tracks[0].base_size == expected
}

fn empty_line_name_groups(groups: &[Vec<String>], expected_count: usize) -> bool {
    groups.len() == expected_count && groups.iter().all(Vec::is_empty)
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
        "c114_fit_content_min_height_output_mismatch",
        None,
        Some(detail.to_owned()),
    )
}

#[cfg(test)]
mod tests {
    use super::validate_grid_input;
    use spinon_core::NodeId;
    use spinon_layout::{
        GridAutoFlow, GridAutoRows, GridContainerInput, GridPlacementMode,
        GridTrackLengthPercentage, GridTrackSize,
    };

    fn expected_grid_input() -> GridContainerInput {
        GridContainerInput {
            columns: vec![GridTrackSize::LengthPx(60.0)],
            columns_line_names: vec![vec![], vec![]],
            auto_rows: GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
                GridTrackLengthPercentage::LengthPx(40.0),
            )]),
            auto_flow: GridAutoFlow::Row,
            placement_mode: GridPlacementMode::TrackSizing,
            ..GridContainerInput::default()
        }
    }

    #[test]
    fn accepts_only_the_pinned_min_height_grid_input() {
        let root = NodeId::new(1).unwrap();
        assert!(validate_grid_input(root, &expected_grid_input()).is_ok());

        let mut changed_limit = expected_grid_input();
        changed_limit.auto_rows = GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
            GridTrackLengthPercentage::LengthPx(41.0),
        )]);
        assert!(validate_grid_input(root, &changed_limit).is_err());

        let mut explicit_row = expected_grid_input();
        explicit_row.rows.push(GridTrackSize::Auto);
        assert!(validate_grid_input(root, &explicit_row).is_err());
    }
}
