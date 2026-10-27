use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostParent, NodeId};
use spinon_layout::{
    GridAutoFlow, GridAutoRows, GridContainerInput, GridPlacementMode, GridTrackLengthPercentage,
    GridTrackSize, LayoutGridTrackSizingAxis, LayoutGridTrackSizingPassTrace,
    LayoutGridTrackSizingStageKind, LayoutGridTrackSizingStageTrace,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize,
    ComputedElementStyle, ComputedStyleProfile, ComputedStyleSnapshot,
};

use super::super::{
    RuntimeC11_4FitContentAutoNeighborObservation, RuntimeLayoutCompleted, RuntimeLayoutFailure,
    RuntimeUaCascadeKey, layout_failure,
};
use super::c11_4_image::element_identity;
use super::c11_4_nested_grid_replaced_item::{
    HTML_NAMESPACE, children, spacing_is, style_for, unsupported, zero_edges,
};

const SCENE_ID: &str = "fit-content-auto-neighbor-wpt";
const ROOT_ID: &str = "c114-fit-content-auto-neighbor-grid";
const FIT_ITEM_ID: &str = "c114-fit-content-auto-neighbor-fit-item";
const FIT_CHILD_ID: &str = "c114-fit-content-auto-neighbor-fit-child";
const AUTO_ITEM_ID: &str = "c114-fit-content-auto-neighbor-auto-item";
const AUTO_CHILD_ID: &str = "c114-fit-content-auto-neighbor-auto-child";

pub(in crate::session::ua_cascade::runtime_layout) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<[NodeId; 5]>, RuntimeLayoutFailure> {
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
    require_child_count(root.id(), root_children.len(), 2, "direct grid item count")?;
    let expected = [
        (root_children[0], FIT_ITEM_ID, FIT_CHILD_ID),
        (root_children[1], AUTO_ITEM_ID, AUTO_CHILD_ID),
    ];
    let mut node_ids = [root.id(); 5];
    for (index, (item, expected_item_id, expected_child_id)) in expected.into_iter().enumerate() {
        if snapshot.parent(item) != Some(HostParent::Node(root)) {
            return Err(unsupported(
                item.id(),
                SCENE_ID,
                "grid item is not a direct child",
            ));
        }
        validate_element_identity(snapshot, item, expected_item_id)?;
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
        validate_element_identity(snapshot, child, expected_child_id)?;
        validate_item_style(style_for(styles, item)?, expected_item_id)?;
        validate_child_style(style_for(styles, child)?, expected_child_id)?;
        node_ids[1 + index * 2] = item.id();
        node_ids[2 + index * 2] = child.id();
    }
    validate_root_style(style_for(styles, root)?)?;
    if node_ids
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>()
        .len()
        != node_ids.len()
    {
        return Err(unsupported(
            root.id(),
            SCENE_ID,
            "duplicate fixture node identity",
        ));
    }
    Ok(Some(node_ids))
}

pub(in crate::session::ua_cascade::runtime_layout) fn observation(
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    grid_container_inputs: &std::collections::BTreeMap<NodeId, GridContainerInput>,
    node_ids: [NodeId; 5],
) -> Result<RuntimeC11_4FitContentAutoNeighborObservation, RuntimeLayoutFailure> {
    validate_root_style(style_for(styles, root)?)?;
    let grid_input = grid_container_inputs.get(&root.id()).ok_or_else(|| {
        layout_failure(
            "c114_fit_content_auto_neighbor_grid_input_missing",
            Some(root.id().get()),
            None,
        )
    })?;
    validate_grid_input(root.id(), grid_input)?;
    if grid_container_inputs.len() != 1 {
        return Err(unsupported(
            root.id(),
            SCENE_ID,
            "unexpected explicit Grid container input",
        ));
    }
    Ok(RuntimeC11_4FitContentAutoNeighborObservation {
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
        .fit_content_auto_neighbor
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
    {
        return Err(failure("computed style profile changed"));
    }
    validate_root_style(style_for_node(
        &observation.style_snapshot,
        observation.grid_root,
    )?)?;
    validate_grid_input(observation.grid_root, &observation.grid_container_input)?;

    let relevant_frame_count = completed
        .frames
        .iter()
        .filter(|frame| {
            observation
                .node_ids
                .iter()
                .any(|node| node.get() == frame.node_id)
        })
        .count();
    let frames = completed
        .frames
        .iter()
        .filter(|frame| {
            observation
                .node_ids
                .iter()
                .any(|node| node.get() == frame.node_id)
        })
        .map(|frame| (frame.node_id, (frame.x, frame.y, frame.width, frame.height)))
        .collect::<std::collections::BTreeMap<_, _>>();
    if relevant_frame_count != observation.node_ids.len()
        || frames.len() != observation.node_ids.len()
    {
        return Err(failure("missing, duplicate, or extra fixture node frames"));
    }
    for (node, expected) in observation.node_ids.iter().zip([
        (0.0, 0.0, 60.0, 300.0),
        (0.0, 0.0, 60.0, 10.0),
        (0.0, 0.0, 60.0, 10.0),
        (0.0, 10.0, 60.0, 290.0),
        (0.0, 10.0, 60.0, 10.0),
    ]) {
        let actual = frames
            .get(&node.get())
            .ok_or_else(|| failure("fixture node frame missing"))?;
        if *actual != expected {
            return Err(failure(
                "layout frame differs from pinned CSS-pixel geometry",
            ));
        }
    }

    let traces = &diagnostics.traces;
    if traces.len() != 1 || traces[0].node != observation.grid_root || traces[0].passes.len() != 2 {
        return Err(failure(
            "expected one root trace with one Columns and one Rows pass",
        ));
    }
    let columns = traces[0]
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .count();
    let rows = traces[0]
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .collect::<Vec<_>>();
    if columns != 1 || rows.len() != 1 {
        return Err(failure("expected exactly one Columns and one Rows pass"));
    }
    validate_rows_pass(rows[0])
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
    let expected_auto_rows = ComputedCssGridAutoRows::TrackList(vec![
        ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(40.0)),
        ComputedCssGridTrackSize::Auto,
    ]);
    if style.properties.get("display").map(String::as_str) != Some("grid")
        || style.properties.get("align-content").map(String::as_str) != Some("normal")
        || style.properties.get("grid-auto-flow").map(String::as_str) != Some("row")
        || style.properties.get("grid-auto-rows").map(String::as_str)
            != Some("fit-content(40px) auto")
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(60.0)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(300.0)
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
        let detail = format!(
            "computed root style differs from allowlist: display={:?}, grid-auto-rows={:?}, grid-auto-flow={:?}, align-content={:?}, dimensions={:?}, spacing={:?}, border={:?}, grid={:?}",
            style.properties.get("display"),
            style.properties.get("grid-auto-rows"),
            style.properties.get("grid-auto-flow"),
            style.properties.get("align-content"),
            style.layout_dimensions,
            style.layout_spacing,
            style.layout_border,
            style.grid_layout,
        );
        return Err(unsupported(style.node_id, SCENE_ID, &detail));
    }
    Ok(())
}

fn validate_item_style(
    style: &ComputedElementStyle,
    expected_id: &str,
) -> Result<(), RuntimeLayoutFailure> {
    if style.properties.get("display").map(String::as_str) != Some("block")
        || style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
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
        return Err(unsupported(style.node_id, SCENE_ID, expected_id));
    }
    Ok(())
}

fn validate_child_style(
    style: &ComputedElementStyle,
    expected_id: &str,
) -> Result<(), RuntimeLayoutFailure> {
    if style.properties.get("display").map(String::as_str) != Some("block")
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(60.0)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(10.0)
        || !zero_edges(style.layout_spacing.padding)
        || !zero_edges(style.layout_spacing.margin)
        || style.layout_border != spinon_style::ComputedLayoutBorder::default()
    {
        return Err(unsupported(style.node_id, SCENE_ID, expected_id));
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
            != GridAutoRows::TrackList(vec![
                GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(40.0)),
                GridTrackSize::Auto,
            ])
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

fn validate_rows_pass(pass: &LayoutGridTrackSizingPassTrace) -> Result<(), RuntimeLayoutFailure> {
    let stages = [
        LayoutGridTrackSizingStageKind::IntrinsicSizingComplete,
        LayoutGridTrackSizingStageKind::MaximizeTracks,
        LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
    ];
    let mut stage_indices = Vec::with_capacity(4);
    for stage in stages {
        let matching = pass
            .stages
            .iter()
            .enumerate()
            .filter(|(_, candidate)| candidate.stage == stage)
            .collect::<Vec<_>>();
        if matching.len() != 1 {
            return Err(failure("row sizing stage missing or duplicated"));
        }
        stage_indices.push(matching[0].0);
        if !track_bases_match(matching[0].1, [10.0, 10.0]) {
            return Err(failure("pre-stretch row bases differ from [10, 10] CSS px"));
        }
    }
    let stretch = pass
        .stages
        .iter()
        .enumerate()
        .filter(|(_, stage)| {
            matches!(
                stage.stage,
                LayoutGridTrackSizingStageKind::StretchAutoTracks { .. }
            )
        })
        .collect::<Vec<_>>();
    if stretch.len() != 1
        || stretch[0].0 != pass.stages.len() - 1
        || stretch[0].1.stage
            != (LayoutGridTrackSizingStageKind::StretchAutoTracks { applied: true })
        || !track_bases_match(stretch[0].1, [10.0, 290.0])
    {
        return Err(failure(
            "final row sizing stage is not the expected auto-track stretch",
        ));
    }
    stage_indices.push(stretch[0].0);
    if !stage_indices.windows(2).all(|pair| pair[0] < pair[1]) {
        return Err(failure("row sizing stages are out of order"));
    }
    Ok(())
}

fn track_bases_match(stage: &LayoutGridTrackSizingStageTrace, expected: [f32; 2]) -> bool {
    let tracks = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .collect::<Vec<_>>();
    tracks.len() == expected.len()
        && tracks
            .iter()
            .zip(expected)
            .all(|(track, expected)| track.base_size == expected)
}

fn failure(detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "c114_fit_content_auto_neighbor_output_mismatch",
        None,
        Some(detail.to_owned()),
    )
}

#[cfg(test)]
mod tests {
    use super::{
        RuntimeUaCascadeKey, require_child_count, revisions_match, validate_grid_input,
        validate_rows_pass,
    };
    use spinon_core::{EnvironmentRevision, HostDocument, NodeId, StyleRevision};
    use spinon_layout::{
        GridAutoFlow, GridAutoRows, GridContainerInput, GridPlacementMode,
        GridTrackLengthPercentage, GridTrackSize, LayoutGridTrackSizingAxis,
        LayoutGridTrackSizingPassTrace, LayoutGridTrackSizingStageKind,
        LayoutGridTrackSizingStageTrace, LayoutGridTrackSizingTrackState, LayoutInputRevision,
        LayoutSourceRevision,
    };

    fn expected_grid_input() -> GridContainerInput {
        GridContainerInput {
            columns: vec![GridTrackSize::LengthPx(60.0)],
            columns_line_names: vec![vec![], vec![]],
            auto_rows: GridAutoRows::TrackList(vec![
                GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(40.0)),
                GridTrackSize::Auto,
            ]),
            auto_flow: GridAutoFlow::Row,
            placement_mode: GridPlacementMode::TrackSizing,
            ..GridContainerInput::default()
        }
    }

    #[test]
    fn accepts_only_the_expected_typed_grid_input() {
        let root = NodeId::new(1).unwrap();
        assert!(validate_grid_input(root, &expected_grid_input()).is_ok());

        let mut changed_value = expected_grid_input();
        changed_value.auto_rows = GridAutoRows::TrackList(vec![
            GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(41.0)),
            GridTrackSize::Auto,
        ]);
        assert!(validate_grid_input(root, &changed_value).is_err());

        let mut changed_order = expected_grid_input();
        changed_order.auto_rows = GridAutoRows::TrackList(vec![
            GridTrackSize::Auto,
            GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(40.0)),
        ]);
        assert!(validate_grid_input(root, &changed_order).is_err());

        let mut extra_explicit_row = expected_grid_input();
        extra_explicit_row.rows.push(GridTrackSize::LengthPx(10.0));
        assert!(validate_grid_input(root, &extra_explicit_row).is_err());
    }

    #[test]
    fn rejects_missing_and_extra_grid_children() {
        let root = NodeId::new(1).unwrap();
        assert!(require_child_count(root, 2, 2, "direct grid item count").is_ok());
        assert!(require_child_count(root, 1, 2, "direct grid item count").is_err());
        assert!(require_child_count(root, 3, 2, "direct grid item count").is_err());
        assert!(require_child_count(root, 0, 1, "nested block child count").is_err());
        assert!(require_child_count(root, 2, 1, "nested block child count").is_err());
    }

    #[test]
    fn rejects_row_stages_after_auto_track_stretch() {
        let tracks = |base_sizes: [f32; 2]| {
            base_sizes
                .into_iter()
                .enumerate()
                .map(|(index, base_size)| LayoutGridTrackSizingTrackState {
                    internal_index: index as u16,
                    is_gutter: false,
                    is_collapsed: false,
                    base_size,
                    growth_limit: None,
                    infinitely_growable: false,
                })
                .collect()
        };
        let mut pass = LayoutGridTrackSizingPassTrace {
            sequence: 0,
            axis: LayoutGridTrackSizingAxis::Rows,
            stages: vec![
                LayoutGridTrackSizingStageTrace {
                    stage: LayoutGridTrackSizingStageKind::IntrinsicSizingComplete,
                    tracks: tracks([10.0, 10.0]),
                },
                LayoutGridTrackSizingStageTrace {
                    stage: LayoutGridTrackSizingStageKind::MaximizeTracks,
                    tracks: tracks([10.0, 10.0]),
                },
                LayoutGridTrackSizingStageTrace {
                    stage: LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
                    tracks: tracks([10.0, 10.0]),
                },
                LayoutGridTrackSizingStageTrace {
                    stage: LayoutGridTrackSizingStageKind::StretchAutoTracks { applied: true },
                    tracks: tracks([10.0, 290.0]),
                },
            ],
            contributions: Vec::new(),
            fixture_phase: None,
            fixture_auto_repeat_count: None,
            auto_repeat_count: None,
            fixture_percentage_basis: None,
        };
        assert!(validate_rows_pass(&pass).is_ok());

        pass.stages.push(LayoutGridTrackSizingStageTrace {
            stage: LayoutGridTrackSizingStageKind::FixedTrackEarlyReturn,
            tracks: tracks([10.0, 290.0]),
        });
        assert!(validate_rows_pass(&pass).is_err());
    }

    #[test]
    fn rejects_stale_layout_revision_before_publishing_scene_output() {
        let document = HostDocument::new().unwrap();
        let snapshot = document.snapshot();
        let style_revision = StyleRevision::INITIAL;
        let key = RuntimeUaCascadeKey {
            generation: snapshot.generation().get(),
            document_revision: snapshot.document_revision().get(),
            render_tree_revision: snapshot.render_tree_revision().get(),
            style_revision,
            environment_revision: EnvironmentRevision::INITIAL.get(),
        };
        let source = LayoutSourceRevision::HostDocument {
            generation: snapshot.generation(),
            document: snapshot.document_revision(),
            render_tree: snapshot.render_tree_revision(),
        };
        let current =
            LayoutInputRevision::new(source, style_revision, EnvironmentRevision::INITIAL);
        assert!(revisions_match(
            style_revision,
            key.generation,
            key.document_revision,
            key.render_tree_revision,
            current,
            key,
        ));

        let stale_style = style_revision.checked_next().unwrap();
        let stale = LayoutInputRevision::new(source, stale_style, EnvironmentRevision::INITIAL);
        assert!(!revisions_match(
            style_revision,
            key.generation,
            key.document_revision,
            key.render_tree_revision,
            stale,
            key,
        ));

        let mut stale_key = key;
        stale_key.document_revision += 1;
        assert!(!revisions_match(
            style_revision,
            key.generation,
            key.document_revision,
            key.render_tree_revision,
            current,
            stale_key,
        ));
    }
}
